use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

use crate::domain::evidence::EvidenceItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestRiskScore {
    pub score: u32,
    pub category: RiskCategory,
    pub findings: Vec<EvidenceItem>,
}
