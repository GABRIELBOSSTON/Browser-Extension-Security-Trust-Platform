use serde::Serialize;
use crate::domain::types::{BrowserFamily, BrowserChannel};

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredExtension {
    pub extension_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_name: String,
    pub install_path: String,
    pub version: String,
    pub disabled: bool,
    pub policy_installed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrowserDiscoveryResult {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub extensions: Vec<DiscoveredExtension>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryResult {
    pub browsers: Vec<BrowserDiscoveryResult>,
}
