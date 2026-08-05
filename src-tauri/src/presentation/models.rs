use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionSummaryResponse {
    pub extension_id: String,
    pub name: String,
    pub version: String,
    pub browser_family: String,
    pub install_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanExtensionRequest {
    pub extension_id: String,
    pub browser_family: String,
    pub install_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanExtensionResponse {
    pub pipeline_id: String,
    pub status: String,
    pub risk_score: f64,
    pub severity: String,
    pub elapsed_ms: u64,
}
