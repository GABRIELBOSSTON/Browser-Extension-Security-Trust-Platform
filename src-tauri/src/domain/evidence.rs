use serde::{Deserialize, Serialize};

/// Represents a single piece of evidence contributing to the final risk score.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceItem {
    pub category: String,
    pub detail: String,
    pub severity: String,
    pub base_score: i32,
}

/// The result of the Risk Correlation layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelatedRiskResult {
    pub final_score: u32,
    pub final_level: String,
    pub evidence: Vec<EvidenceItem>,
}
