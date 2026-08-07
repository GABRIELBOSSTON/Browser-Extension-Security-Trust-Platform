use serde::{Deserialize, Serialize};
use crate::domain::types::Severity;
use crate::domain::capabilities::{CapabilityId, MatchPattern};

/// Rule Categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleCategory {
    Permission,
    Network,
    Behavioral,
    Code,
}

/// Aggregation Policy for duplicate findings of this rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationPolicy {
    Once,
    Sum,
    Max,
    Decay,
}

/// Rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_id: String,
    pub version: String,
    pub category: RuleCategory,
    pub severity: Severity,
    pub description: String,
    pub matcher_id: String,
    pub weight: f64,
    pub aggregation_policy: AggregationPolicy,
}

/// RuleSet aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub version: String,
    pub checksum: String,
    pub source: String,
    pub rules: Vec<Rule>,
}

impl Default for RuleSet {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            checksum: String::new(),
            source: "local".to_string(),
            rules: Vec::new(),
        }
    }
}

/// Evidence value object detailing what triggered the rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Evidence {
    Capability(CapabilityId),
    MatchPattern(MatchPattern),
    Multiple(Vec<Evidence>),
    MissingCapability(String),
}

/// Finding resulting from a rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub finding_id: String,
    pub rule_id: String,
    pub severity: Severity,
    pub description: String,
    pub evidence: Evidence,
}
