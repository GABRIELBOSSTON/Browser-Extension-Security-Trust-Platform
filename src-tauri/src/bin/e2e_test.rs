use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;
use antigraviiti_extension_protect::application::explanation_engine::{
    ExplanationEngine, ExplanationInput,
};
use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use antigraviiti_extension_protect::application::risk::unified_risk::UnifiedRiskService;
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
                let extension_dir = PathBuf::from(&ext.install_path);
                let manifest_content = std::fs::read_to_string(&manifest_path).ok();

                let risk_result = UnifiedRiskService::analyze_extension(
                    &extension_dir,
                    &ext.extension_id,
                    &manifest,
                    manifest_content.as_deref(),
                    vt_engine.as_ref(),
                );

                let final_score = risk_result.correlation.final_score;
                let final_level = risk_result.correlation.final_level;

                let final_reasons: Vec<String> = risk_result
                    .correlation
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
                    content_scripts: content_scripts.clone(),
                    background: background.clone(),
                    csp: csp.clone(),
                    ast_findings: risk_result.ast_findings.clone(),
                    ioc_findings: risk_result.ioc_findings,
                    vt_reports: risk_result.vt_reports,
                    trusted: risk_result.trusted,
                };

                let explanation_input = ExplanationInput {
                    extension_id: ext.extension_id.clone(),
                    extension_name: ext.name.clone(),
                    risk_score: final_score,
                    risk_level: final_level.to_string(),
                    manifest_reasons: final_reasons,
                    permissions: permissions.clone(),
                    host_permissions: host_permissions.clone(),
                    ast_findings: risk_result.ast_findings,
                    correlated_evidence: risk_result.correlation.evidence,
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
