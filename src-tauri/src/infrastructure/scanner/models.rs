use crate::domain::types::{BrowserChannel, BrowserFamily};

#[derive(Debug, Clone)]
pub struct RawDiscoveredExtension {
    pub extension_id: String,
    pub profile_name: String,
    pub version: String,
    pub install_path: String,
    pub name: String,
    pub manifest_version: i32,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct BrowserScanResult {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub extensions: Vec<RawDiscoveredExtension>,
    pub error: Option<String>,
}
