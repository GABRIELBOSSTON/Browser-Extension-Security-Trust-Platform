use crate::application::discovery::models::{
    BrowserDiscoveryResult, DiscoveredExtension, DiscoveryResult,
};
use crate::infrastructure::scanner::DiscoveryEngine;

pub struct DiscoveryService;

impl DiscoveryService {
    pub fn execute_discovery() -> Result<DiscoveryResult, String> {
        let engine = DiscoveryEngine::new();
        let raw_results = engine.scan_all();

        let browsers = raw_results
            .into_iter()
            .map(|raw_browser| {
                let extensions = raw_browser
                    .extensions
                    .into_iter()
                    .map(|raw_ext| DiscoveredExtension {
                        extension_id: raw_ext.extension_id,
                        name: raw_ext.name,
                        browser_family: raw_browser.browser_family,
                        browser_channel: raw_browser.browser_channel,
                        profile_name: raw_ext.profile_name,
                        install_path: raw_ext.install_path,
                        version: raw_ext.version,
                        manifest_version: raw_ext.manifest_version,
                        disabled: raw_ext.disabled,
                        policy_installed: false, // Default placeholder
                    })
                    .collect();

                BrowserDiscoveryResult {
                    browser_family: raw_browser.browser_family,
                    browser_channel: raw_browser.browser_channel,
                    extensions,
                    error: raw_browser.error,
                }
            })
            .collect();

        Ok(DiscoveryResult { browsers })
    }
}
