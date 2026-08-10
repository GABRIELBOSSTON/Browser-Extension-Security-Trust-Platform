use crate::domain::types::{RiskScore, Severity};
use serde::{Deserialize, Serialize};

/// Pre-defined risk profiles mapping scores to severities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskProfile {
    Default,
    Strict,
    Relaxed,
}

impl RiskProfile {
    pub fn classify(&self, score: RiskScore) -> Severity {
        let val = score.value();
        match self {
            RiskProfile::Default => {
                if val <= 20.0 {
                    Severity::Low
                } else if val <= 50.0 {
                    Severity::Medium
                } else if val <= 80.0 {
                    Severity::High
                } else {
                    Severity::Critical
                }
            }
            RiskProfile::Strict => {
                if val <= 10.0 {
                    Severity::Low
                } else if val <= 30.0 {
                    Severity::Medium
                } else if val <= 60.0 {
                    Severity::High
                } else {
                    Severity::Critical
                }
            }
            RiskProfile::Relaxed => {
                if val <= 30.0 {
                    Severity::Low
                } else if val <= 60.0 {
                    Severity::Medium
                } else if val <= 90.0 {
                    Severity::High
                } else {
                    Severity::Critical
                }
            }
        }
    }
}
