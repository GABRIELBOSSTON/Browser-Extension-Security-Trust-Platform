use crate::domain::rules::Finding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub execution_time_ms: u64,
    pub rules_evaluated: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEvaluationResult {
    pub findings: Vec<Finding>,
    pub execution_stats: ExecutionStats,
    pub rule_version: String,
}
