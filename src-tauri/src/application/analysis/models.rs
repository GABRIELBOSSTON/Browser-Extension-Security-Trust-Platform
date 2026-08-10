use crate::domain::capabilities::ExtensionCapabilityModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatistics {
    pub total_permissions: usize,
    pub total_hosts: usize,
    pub total_content_scripts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAnalysisResult {
    pub model: ExtensionCapabilityModel,
    pub stats: CapabilityStatistics,
    pub warnings: Vec<String>,
    pub normalization_notes: Vec<String>,
}
