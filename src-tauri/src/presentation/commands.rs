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

                let mut extra_score = 0;
                let mut ast_reasons = Vec::new();
                for finding in &ast_findings {
                    match finding.severity.as_str() {
                        "Critical" => {
                            if extra_score < 20 {
                                extra_score = 20;
                            }
                            ast_reasons.push(format!("AST Critical: {}", finding.reason));
                        }
                        "High" if extra_score < 10 => {
                            extra_score = 10;
                        }
                        "Medium" if extra_score < 5 => {
                            extra_score = 5;
                        }
                        _ => {}
                    }
                }

                let mut final_score = (risk_assessment.score + extra_score).min(100);
                let trusted = crate::domain::trust::TrustRegistry::is_trusted(&ext.extension_id);
                
                // If it's a trusted extension, cap the risk score so it never shows as Critical
                if trusted && final_score > 60 {
                    final_score = 60; // Cap at Medium/High boundary
                }

                let final_level = match final_score {
                    0..=20 => "Safe",
                    21..=40 => "Low",
                    41..=60 => "Medium",
                    61..=80 => "High",
                    _ => "Critical",
                };

                let mut final_reasons = risk_assessment.reasons;
                // Deduplicate and take top AST reasons so UI doesn't blow up
                ast_reasons.sort();
                ast_reasons.dedup();
                final_reasons.extend(ast_reasons.into_iter().take(5));

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
}

#[tauri::command]
pub fn explain_extension(
    request: ExplainExtensionRequest,
) -> Result<crate::domain::explanation::SecurityExplanation, String> {
    use crate::application::explanation_engine::{ExplanationEngine, ExplanationInput};

    let input = ExplanationInput {
        extension_id: request.extension_id,
        extension_name: request.extension_name,
        risk_score: request.risk_score,
        risk_level: request.risk_level,
        manifest_reasons: request.reasons,
        permissions: request.permissions,
        host_permissions: request.host_permissions,
        ast_findings: request.ast_findings,
    };

    Ok(ExplanationEngine::explain(&input))
}
