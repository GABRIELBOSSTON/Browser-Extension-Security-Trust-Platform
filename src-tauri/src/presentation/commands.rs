use crate::domain::entities::DiscoveredExtension;
use crate::domain::types::{BrowserChannel, BrowserFamily};
use crate::infrastructure::db::ConnectionProvider;
use crate::infrastructure::DatabaseManager;
use crate::presentation::models::{
    ExtensionSummaryResponse, ScanExtensionRequest, ScanExtensionResponse,
};
use crate::presentation::state::AppState;
use serde::Serialize;
use std::sync::Arc;
use tauri::State;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Serialize)]
pub struct DatabaseStatusResponse {
    pub connected: bool,
    pub wal_mode_enabled: bool,
    pub version: String,
}

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_database_status(
    db_state: State<'_, Arc<DatabaseManager>>,
) -> Result<DatabaseStatusResponse, String> {
    let conn = db_state
        .get_connection()
        .map_err(|e| format!("Database connection error: {}", e))?;

    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode;", [], |row| row.get(0))
        .unwrap_or_else(|_| "unknown".to_string());

    let version: String = conn
        .query_row("SELECT sqlite_version();", [], |row| row.get(0))
        .unwrap_or_else(|_| "SQLite 3".to_string());

    Ok(DatabaseStatusResponse {
        connected: true,
        wal_mode_enabled: journal_mode.to_lowercase() == "wal",
        version: format!("SQLite v{}", version),
    })
}

#[tauri::command]
pub fn get_installed_extensions(
    _state: State<'_, AppState>,
) -> Result<Vec<ExtensionSummaryResponse>, String> {
    let result = crate::application::discovery::DiscoveryService::execute_discovery()
        .map_err(|e| e.to_string())?;

    let mut summaries = Vec::new();
    for browser in result.browsers {
        for ext in browser.extensions {
            summaries.push(ExtensionSummaryResponse {
                extension_id: ext.extension_id,
                name: ext.profile_name.clone(), // Map profile to name for now
                version: ext.version,
                browser_family: format!("{:?}", browser.browser_family),
                install_path: ext.install_path,
            });
        }
    }

    Ok(summaries)
}

#[tauri::command]
pub async fn scan_extension(
    request: ScanExtensionRequest,
    state: State<'_, AppState>,
) -> Result<ScanExtensionResponse, String> {
    // Parse browser family
    let family = match request.browser_family.to_lowercase().as_str() {
        "chrome" => BrowserFamily::Chrome,
        "brave" => BrowserFamily::Brave,
        "edge" => BrowserFamily::Edge,
        "firefox" => BrowserFamily::Firefox,
        _ => BrowserFamily::Unknown,
    };

    let target = DiscoveredExtension {
        extension_id: request.extension_id,
        name: "Unknown Extension".to_string(),
        manifest: None,
        browser_family: family,
        browser_channel: BrowserChannel::Stable, // Default
        profile_name: "Default".to_string(),
        install_path: request.install_path,
        version: "unknown".to_string(),
        disabled: false,
        policy_installed: false,
    };

    let risk_profile = crate::domain::risk::RiskProfile::Default;
    let cancel_token = CancellationToken::new();

    let pipeline = state.analysis_pipeline.clone();

    // 1. Offload CPU-heavy pipeline to spawn_blocking in Application Service equivalent
    // NOTE: Application pipeline itself is internally updated to spawn_blocking.
    // Here we just call it async.
    let result = pipeline
        .analyze_single(&target, &risk_profile, cancel_token)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ScanExtensionResponse {
        pipeline_id: result.metadata.pipeline_id,
        status: "Completed".to_string(),
        risk_score: result.assessment.normalized_score.value(),
        severity: format!("{:?}", result.assessment.severity),
        elapsed_ms: result.metadata.elapsed_ms,
    })
}

#[derive(Debug, Serialize)]
pub struct ExtensionAnalysisResponse {
    pub id: String,
    pub browser: String,
    pub name: String,
    pub version: String,
    pub risk_score: u32,
    pub risk_level: String,
    pub reasons: Vec<String>,
    pub permissions: Vec<String>,
    pub host_permissions: Vec<String>,
    pub content_scripts: Vec<serde_json::Value>,
    pub background: Option<serde_json::Value>,
    pub csp: Option<serde_json::Value>,
    pub ast_findings: Vec<crate::application::ast_detector::scanner::ASTFinding>,
    pub ioc_findings: Vec<crate::application::ioc::models::IOCFinding>,
    pub vt_reports: Vec<crate::application::virustotal::models::VirusTotalReport>,
    pub trusted: bool,
}

#[tauri::command]
pub async fn scan_extensions(
    app_handle: tauri::AppHandle,
) -> Result<Vec<ExtensionAnalysisResponse>, String> {
    use crate::application::discovery::service::DiscoveryService;
    use crate::application::manifest::service::ManifestService;
    use crate::application::risk::manifest_risk::ManifestRiskEngine;
    use std::path::PathBuf;
    use tauri::Manager;

    let discovery_result =
        DiscoveryService::execute_discovery().map_err(|e| format!("Discovery failed: {}", e))?;

    let mut all_results = Vec::new();

    // Initialize VT Engine
    let vt_db_path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("vt_cache.db");
    let vt_engine = crate::application::virustotal::VirusTotalEngine::new(&vt_db_path).ok();

    for browser in discovery_result.browsers {
        for ext in browser.extensions {
            let manifest_path = PathBuf::from(&ext.install_path).join("manifest.json");

            if !manifest_path.exists() {
                continue;
            }

            if let Ok(manifest) = ManifestService::load_manifest(&manifest_path) {
                let risk_assessment = ManifestRiskEngine::analyze(&manifest);

                let extension_dir = PathBuf::from(&ext.install_path);
                let (ast_findings, mut ioc_findings, vt_reports) = if extension_dir.exists() {
                    let ast = crate::application::ast_detector::scanner::AstScannerService::scan_directory(
                        &extension_dir,
                    );
                    let ioc = crate::application::ioc::IocEngine::scan_directory(&extension_dir);
                    let vts = if let Some(vt) = &vt_engine {
                        vt.scan_extension(&extension_dir)
                    } else {
                        Vec::new()
                    };
                    (ast, ioc, vts)
                } else {
                    (Vec::new(), Vec::new(), Vec::new())
                };

                if let Ok(manifest_content) = std::fs::read_to_string(&manifest_path) {
                    let manifest_iocs = crate::application::ioc::IocEngine::scan_manifest(
                        &manifest_content,
                        "manifest.json",
                    );
                    ioc_findings.extend(manifest_iocs);
                }

                let mut evidence_items = Vec::new();

                for reason in &risk_assessment.reasons {
                    let lower = reason.to_lowercase();
                    let sev = if lower.contains("password stealer")
                        || lower.contains("nativemessaging")
                        || lower.contains("debugger")
                        || lower.contains("<all_urls>")
                        || lower.contains("*://*/*")
                    {
                        ("Critical", 80)
                    } else if lower.contains("proxy")
                        || lower.contains("cookies")
                        || lower.contains("management")
                        || lower.contains("webrequest")
                    {
                        ("High", 40)
                    } else if lower.contains("history")
                        || lower.contains("tabs")
                        || lower.contains("unsafe-inline")
                        || lower.contains("unsafe-eval")
                    {
                        ("Medium", 20)
                    } else {
                        ("Low", 5)
                    };
                    evidence_items.push(crate::domain::evidence::EvidenceItem {
                        category: "Manifest".to_string(),
                        detail: reason.clone(),
                        severity: sev.0.to_string(),
                        base_score: sev.1,
                    });
                }

                for finding in &ast_findings {
                    let base_score = match finding.severity.as_str() {
                        "Critical" => 80,
                        "High" => 40,
                        "Medium" => 20,
                        "Low" => 5,
                        _ => 0,
                    };
                    evidence_items.push(crate::domain::evidence::EvidenceItem {
                        category: "Code Analysis".to_string(),
                        detail: finding.reason.clone(),
                        severity: finding.severity.clone(),
                        base_score,
                    });
                }

                for finding in &ioc_findings {
                    let sev_str = format!("{:?}", finding.severity);
                    let base_score = match sev_str.as_str() {
                        "Critical" => 80,
                        "High" => 40,
                        "Medium" => 20,
                        "Low" => 5,
                        _ => 0,
                    };
                    evidence_items.push(crate::domain::evidence::EvidenceItem {
                        category: "IOC".to_string(),
                        detail: finding.title.clone(),
                        severity: sev_str,
                        base_score,
                    });
                }

                for vt in &vt_reports {
                    if vt.malicious > 0 {
                        evidence_items.push(crate::domain::evidence::EvidenceItem {
                            category: "VirusTotal".to_string(),
                            detail: "Flagged as Malicious".to_string(),
                            severity: "Critical".to_string(),
                            base_score: 80,
                        });
                    } else if vt.suspicious > 0 {
                        evidence_items.push(crate::domain::evidence::EvidenceItem {
                            category: "VirusTotal".to_string(),
                            detail: "Flagged as Suspicious".to_string(),
                            severity: "High".to_string(),
                            base_score: 40,
                        });
                    }
                }

                let trusted = crate::domain::trust::TrustRegistry::is_trusted(&ext.extension_id);
                if trusted {
                    evidence_items.push(crate::domain::evidence::EvidenceItem {
                        category: "Trust".to_string(),
                        detail: "Trusted Publisher".to_string(),
                        severity: "Good".to_string(),
                        base_score: -30,
                    });
                }

                let correlation =
                    crate::application::risk::correlator::RiskCorrelator::correlate(evidence_items);

                let final_score = correlation.final_score;
                let final_level = correlation.final_level;

                let final_reasons: Vec<String> = correlation
                    .evidence
                    .iter()
                    .take(5)
                    .map(|e| format!("[{}] {}", e.severity, e.detail))
                    .collect();

                let permissions = manifest
                    .permissions
                    .items
                    .iter()
                    .map(|p| p.permission_string.clone())
                    .collect();
                let host_permissions = manifest
                    .host_permissions
                    .items
                    .iter()
                    .map(|p| p.permission_string.clone())
                    .collect();
                let content_scripts = manifest
                    .content_scripts
                    .iter()
                    .map(|cs| serde_json::to_value(cs).unwrap_or(serde_json::Value::Null))
                    .collect();
                let background = manifest
                    .background
                    .as_ref()
                    .map(|bg| serde_json::to_value(bg).unwrap_or(serde_json::Value::Null));
                let csp = manifest
                    .content_security_policy
                    .as_ref()
                    .map(|csp| serde_json::to_value(csp).unwrap_or(serde_json::Value::Null));

                all_results.push(ExtensionAnalysisResponse {
                    id: ext.extension_id,
                    browser: format!("{:?}", ext.browser_family),
                    name: ext.name,
                    version: ext.version,
                    risk_score: final_score,
                    risk_level: final_level.to_string(),
                    reasons: final_reasons,
                    permissions,
                    host_permissions,
                    content_scripts,
                    background,
                    csp,
                    ast_findings,
                    ioc_findings,
                    vt_reports,
                    trusted,
                });
            }
        }
    }

    Ok(all_results)
}

/// Request payload for the explain_extension command.
#[derive(Debug, serde::Deserialize)]
pub struct ExplainExtensionRequest {
    pub extension_id: String,
    pub extension_name: String,
    pub risk_score: u32,
    pub risk_level: String,
    pub reasons: Vec<String>,
    pub permissions: Vec<String>,
    pub host_permissions: Vec<String>,
    pub ast_findings: Vec<crate::application::ast_detector::scanner::ASTFinding>,
    pub ioc_findings: Vec<crate::application::ioc::models::IOCFinding>,
    pub vt_reports: Vec<crate::application::virustotal::models::VirusTotalReport>,
    pub trusted: bool,
}

#[tauri::command]
pub fn explain_extension(
    request: ExplainExtensionRequest,
) -> Result<crate::domain::explanation::SecurityExplanation, String> {
    use crate::application::explanation_engine::{ExplanationEngine, ExplanationInput};

    let mut evidence_items = Vec::new();

    for reason in &request.reasons {
        let mut sev = "Low";
        let mut base_score = 5;
        if reason.starts_with("[Critical]") {
            sev = "Critical";
            base_score = 80;
        } else if reason.starts_with("[High]") {
            sev = "High";
            base_score = 40;
        } else if reason.starts_with("[Medium]") {
            sev = "Medium";
            base_score = 20;
        } else if reason.starts_with("[Safe]") {
            sev = "Safe";
            base_score = 0;
        }

        let detail = if let Some(idx) = reason.find("] ") {
            reason[idx + 2..].to_string()
        } else {
            reason.clone()
        };

        evidence_items.push(crate::domain::evidence::EvidenceItem {
            category: "Manifest".to_string(), // fallback
            detail: detail.clone(),
            severity: sev.to_string(),
            base_score,
        });
    }

    for finding in &request.ast_findings {
        let base_score = match finding.severity.as_str() {
            "Critical" => 80,
            "High" => 40,
            "Medium" => 20,
            "Low" => 5,
            _ => 0,
        };
        evidence_items.push(crate::domain::evidence::EvidenceItem {
            category: "Code Analysis".to_string(),
            detail: finding.reason.clone(),
            severity: finding.severity.clone(),
            base_score,
        });
    }

    for finding in &request.ioc_findings {
        let sev_str = format!("{:?}", finding.severity);
        let base_score = match sev_str.as_str() {
            "Critical" => 80,
            "High" => 40,
            "Medium" => 20,
            "Low" => 5,
            _ => 0,
        };
        evidence_items.push(crate::domain::evidence::EvidenceItem {
            category: "IOC".to_string(),
            detail: finding.title.clone(),
            severity: sev_str,
            base_score,
        });
    }

    for vt in &request.vt_reports {
        if vt.malicious > 0 {
            evidence_items.push(crate::domain::evidence::EvidenceItem {
                category: "VirusTotal".to_string(),
                detail: "Flagged as Malicious".to_string(),
                severity: "Critical".to_string(),
                base_score: 80,
            });
        } else if vt.suspicious > 0 {
            evidence_items.push(crate::domain::evidence::EvidenceItem {
                category: "VirusTotal".to_string(),
                detail: "Flagged as Suspicious".to_string(),
                severity: "High".to_string(),
                base_score: 40,
            });
        }
    }

    if request.trusted {
        evidence_items.push(crate::domain::evidence::EvidenceItem {
            category: "Trust".to_string(),
            detail: "Trusted Publisher".to_string(),
            severity: "Good".to_string(),
            base_score: -30,
        });
    }

    let correlation =
        crate::application::risk::correlator::RiskCorrelator::correlate(evidence_items);

    let input = ExplanationInput {
        extension_id: request.extension_id,
        extension_name: request.extension_name,
        risk_score: request.risk_score,
        risk_level: request.risk_level,
        manifest_reasons: request.reasons, // Pass raw strings down so regex matches in build_impact still work
        permissions: request.permissions,
        host_permissions: request.host_permissions,
        ast_findings: request.ast_findings,
        correlated_evidence: correlation.evidence,
    };

    Ok(ExplanationEngine::explain(&input))
}
