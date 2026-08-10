use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;
use antigraviiti_extension_protect::application::explanation_engine::{
    ExplanationEngine, ExplanationInput,
};
use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use antigraviiti_extension_protect::application::risk::manifest_risk::ManifestRiskEngine;
use antigraviiti_extension_protect::presentation::commands::ExtensionAnalysisResponse;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let discovery_result = DiscoveryService::execute_discovery().expect("Discovery failed");

    // We will just use an in-memory cache or temp db for VT during this test
    let vt_db_path = std::env::temp_dir().join("e2e_vt_cache.db");
    let vt_engine =
        antigraviiti_extension_protect::application::virustotal::VirusTotalEngine::new(&vt_db_path)
            .ok();

    let mut results = Vec::new();

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
                    let ast = antigraviiti_extension_protect::application::ast_detector::scanner::AstScannerService::scan_directory(&extension_dir);
                    let ioc =
                        antigraviiti_extension_protect::application::ioc::IocEngine::scan_directory(
                            &extension_dir,
                        );
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
                    let manifest_iocs =
                        antigraviiti_extension_protect::application::ioc::IocEngine::scan_manifest(
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
                    evidence_items.push(
                        antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                            category: "Manifest".to_string(),
                            detail: reason.clone(),
                            severity: sev.0.to_string(),
                            base_score: sev.1,
                        },
                    );
                }

                for finding in &ast_findings {
                    let base_score = match finding.severity.as_str() {
                        "Critical" => 80,
                        "High" => 40,
                        "Medium" => 20,
                        "Low" => 5,
                        _ => 0,
                    };
                    evidence_items.push(
                        antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                            category: "Code Analysis".to_string(),
                            detail: finding.reason.clone(),
                            severity: finding.severity.clone(),
                            base_score,
                        },
                    );
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
                    evidence_items.push(
                        antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                            category: "IOC".to_string(),
                            detail: finding.title.clone(),
                            severity: sev_str,
                            base_score,
                        },
                    );
                }

                for vt in &vt_reports {
                    if vt.malicious > 0 {
                        evidence_items.push(
                            antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                                category: "VirusTotal".to_string(),
                                detail: "Flagged as Malicious".to_string(),
                                severity: "Critical".to_string(),
                                base_score: 80,
                            },
                        );
                    } else if vt.suspicious > 0 {
                        evidence_items.push(
                            antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                                category: "VirusTotal".to_string(),
                                detail: "Flagged as Suspicious".to_string(),
                                severity: "High".to_string(),
                                base_score: 40,
                            },
                        );
                    }
                }

                let trusted =
                    antigraviiti_extension_protect::domain::trust::TrustRegistry::is_trusted(
                        &ext.extension_id,
                    );
                if trusted {
                    evidence_items.push(
                        antigraviiti_extension_protect::domain::evidence::EvidenceItem {
                            category: "Trust".to_string(),
                            detail: "Trusted Publisher".to_string(),
                            severity: "Good".to_string(),
                            base_score: -30,
                        },
                    );
                }

                let correlation = antigraviiti_extension_protect::application::risk::correlator::RiskCorrelator::correlate(evidence_items);

                let final_score = correlation.final_score;
                let final_level = correlation.final_level;

                let final_reasons: Vec<String> = correlation
                    .evidence
                    .iter()
                    .take(5)
                    .map(|e| format!("[{}] {}", e.severity, e.detail))
                    .collect();

                let permissions: Vec<String> = manifest
                    .permissions
                    .items
                    .iter()
                    .map(|p| p.permission_string.clone())
                    .collect();
                let host_permissions: Vec<String> = manifest
                    .host_permissions
                    .items
                    .iter()
                    .map(|p| p.permission_string.clone())
                    .collect();
                let content_scripts: Vec<serde_json::Value> = manifest
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

                let response = ExtensionAnalysisResponse {
                    id: ext.extension_id.clone(),
                    browser: format!("{:?}", ext.browser_family),
                    name: ext.name.clone(),
                    version: ext.version,
                    risk_score: final_score,
                    risk_level: final_level.to_string(),
                    reasons: final_reasons.clone(),
                    permissions: permissions.clone(),
                    host_permissions: host_permissions.clone(),
                    content_scripts,
                    background,
                    csp,
                    ast_findings: ast_findings.clone(),
                    ioc_findings,
                    vt_reports,
                    trusted,
                };

                let explanation_input = ExplanationInput {
                    extension_id: ext.extension_id,
                    extension_name: ext.name,
                    risk_score: final_score,
                    risk_level: final_level.to_string(),
                    manifest_reasons: final_reasons,
                    permissions,
                    host_permissions,
                    ast_findings,
                    correlated_evidence: correlation.evidence,
                };

                let explanation = ExplanationEngine::explain(&explanation_input);

                results.push(serde_json::json!({
                    "analysis": response,
                    "explanation": explanation
                }));
            }
        }
    }

    let json = serde_json::to_string_pretty(&results).unwrap();
    println!("{}", json);
}
