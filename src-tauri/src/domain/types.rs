use serde::{Deserialize, Serialize};
use super::errors::{DomainError, Result};

/// Bounded Risk Score Value Object (0.0 to 100.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RiskScore(f64);

impl RiskScore {
    pub fn new(value: f64) -> Result<Self> {
        if (0.0..=100.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidRiskScore(value))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for RiskScore {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Categorical Severity Ratings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Permission Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionType {
    HostPattern,
    ChromeApi,
    Scripting,
    Debugger,
}

/// Chromium Host Browsers Families
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserFamily {
    Chrome,
    Edge,
    Brave,
    Opera,
    Firefox,
}

/// Browser Release Channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserChannel {
    Stable,
    Beta,
    Dev,
    Canary,
    GX,
    ESR,
    Nightly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_risk_score() {
        let score = RiskScore::new(45.5).unwrap();
        assert_eq!(score.value(), 45.5);
    }

    #[test]
    fn test_invalid_risk_score() {
        assert!(RiskScore::new(-5.0).is_err());
        assert!(RiskScore::new(105.0).is_err());
    }
}
