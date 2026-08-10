use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;
use antigraviiti_extension_protect::application::manifest::service::ManifestService;
use serde_json::json;
use std::path::PathBuf;

fn main() {
    println!("Starting Manifest Analyzer...");
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
                    let analysis = json!({
                        "extension_id": ext.extension_id,
                        "name": ext.name,
                        "browser": format!("{:?}", ext.browser_family),
                        "version": ext.version,
                        "permissions": manifest.permissions,
                        "host_permissions": manifest.host_permissions,
                        "content_scripts": manifest.content_scripts,
                        "background": manifest.background,
                        "web_accessible_resources": manifest.web_accessible_resources,
                        "content_security_policy": manifest.content_security_policy,
                        "externally_connectable": manifest.externally_connectable
                    });
                    all_results.push(analysis);
                }
                Err(e) => {
                    eprintln!("Failed to parse manifest for {}: {}", ext.extension_id, e);
                }
            }
        }
    }

    println!("\nAnalysis Result:");
    println!("{}", serde_json::to_string_pretty(&all_results).unwrap());
}
