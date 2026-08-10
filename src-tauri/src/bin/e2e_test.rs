use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;
use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use antigraviiti_extension_protect::application::risk::manifest_risk::ManifestRiskEngine;
use antigraviiti_extension_protect::application::explanation_engine::{ExplanationEngine, ExplanationInput};
use antigraviiti_extension_protect::presentation::commands::ExtensionAnalysisResponse;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let discovery_result = DiscoveryService::execute_discovery().expect("Discovery failed");
    
    // We will just use an in-memory cache or temp db for VT during this test
    let vt_db_path = std::env::temp_dir().join("e2e_vt_cache.db");
    let vt_engine = antigraviiti_extension_protect::application::virustotal::VirusTotalEngine::new(&vt_db_path).ok();

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
                    let ioc = antigraviiti_extension_protect::application::ioc::IocEngine::scan_directory(&extension_dir);
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
                    let manifest_iocs = antigraviiti_extension_protect::application::ioc::IocEngine::scan_manifest(
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
                let trusted = antigraviiti_extension_protect::domain::trust::TrustRegistry::is_trusted(&ext.extension_id);
                if trusted && final_score > 60 {
                    final_score = 60;
                }
                let final_level = match final_score {
                    0..=20 => "Safe",
                    21..=40 => "Low",
                    41..=60 => "Medium",
                    61..=80 => "High",
                    _ => "Critical",
                };

                let mut final_reasons = risk_assessment.reasons;
                ast_reasons.sort();
                ast_reasons.dedup();
                final_reasons.extend(ast_reasons.into_iter().take(5));

                let permissions: Vec<String> = manifest.permissions.items.iter().map(|p| p.permission_string.clone()).collect();
                let host_permissions: Vec<String> = manifest.host_permissions.items.iter().map(|p| p.permission_string.clone()).collect();
                let content_scripts: Vec<serde_json::Value> = manifest.content_scripts.iter().map(|cs| serde_json::to_value(cs).unwrap_or(serde_json::Value::Null)).collect();
                let background = manifest.background.as_ref().map(|bg| serde_json::to_value(bg).unwrap_or(serde_json::Value::Null));
                let csp = manifest.content_security_policy.as_ref().map(|csp| serde_json::to_value(csp).unwrap_or(serde_json::Value::Null));

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
