use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;
use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use antigraviiti_extension_protect::application::risk::manifest_risk::ManifestRiskEngine;
use serde_json::json;
use std::path::PathBuf;

fn main() {
    println!("Starting Manifest Risk Analyzer...");
    let discovery_result = match DiscoveryService::execute_discovery() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Discovery failed: {}", e);
            return;
        }
    };

    let mut all_results = Vec::new();

    for browser in discovery_result.browsers {
        for ext in browser.extensions {
            let manifest_path = PathBuf::from(&ext.install_path).join("manifest.json");

            if !manifest_path.exists() {
                continue;
            }

            match ManifestService::load_manifest(&manifest_path) {
                Ok(manifest) => {
                    let risk_assessment = ManifestRiskEngine::analyze(&manifest);
                    let analysis = json!({
                        "extension_id": ext.extension_id,
                        "name": ext.name,
                        "browser": format!("{:?}", ext.browser_family),
                        "version": ext.version,
                        "risk_score": risk_assessment.score,
                        "risk_category": format!("{:?}", risk_assessment.category),
                        "reasons": risk_assessment.reasons,
                    });
                    all_results.push(analysis);
                }
                Err(e) => {
                    eprintln!("Failed to parse manifest for {}: {}", ext.extension_id, e);
                }
            }
        }
    }

    println!("\nRisk Analysis Result:");
    println!("{}", serde_json::to_string_pretty(&all_results).unwrap());
}
