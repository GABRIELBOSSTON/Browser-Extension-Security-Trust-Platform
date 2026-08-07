use serde::{Deserialize, Serialize};
use crate::domain::types::{RiskScore, Severity};
use crate::domain::rules::AggregationPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRuleConfig {
    pub rule_id: String,
    pub base_weight: f64,
    pub policy: AggregationPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBreakdown {
    pub rule_id: String,
    pub findings_count: usize,
    pub applied_policy: AggregationPolicy,
    pub cumulative_weight: f64,
    pub associated_finding_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreExplanation {
    pub top_contributors: Vec<RiskBreakdown>, // strictly bounded to Top 3
    pub breakdown_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub assessment_id: String, // uuid::Uuid::new_v4().to_string()
    pub raw_score: f64,
    pub normalized_score: RiskScore,
    pub severity: Severity,
    pub breakdown: Vec<RiskBreakdown>,
    pub explanation: ScoreExplanation,
    pub diagnostics: Vec<String>, // Captures warnings (e.g. unknown RuleIds)
}
