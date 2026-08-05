use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectorKind {
    ChromeApi,
    DangerousApi,
    Secret,
    Any,
}

pub trait DetectorInventory: Send + Sync {
    fn detector_kind(&self) -> DetectorKind;
    fn get_match_count(&self, expected_id: u32) -> usize;
    fn exists(&self, expected_id: u32) -> bool;
    // Basic abstraction for Contains / StartsWith over string-based features might require more complex traits
    // But for the structural foundation we stick to the required trait interface layout.
}

pub trait RuleRepository: Send + Sync {
    fn fetch_active_rules(&self) -> Vec<RuleDefinition>;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Equals,
    NotEquals,
    Exists,
    NotExists,
    CountGreaterThan(usize),
    CountLessThan(usize),
    Contains(String),
    StartsWith(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub target_detector: DetectorKind,
    pub expected_id: u32,
    pub condition_type: ConditionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub rule_id: RuleId,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub version: String,
    pub tags: Vec<String>,
    pub conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceReference {
    pub reference_id: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralFinding {
    pub finding_id: FindingId,
    pub rule_id: RuleId,
    pub target_detector: DetectorKind,
    pub evidence: Vec<EvidenceReference>,
    pub evidence_count: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionStatistics {
    pub rules_evaluated: usize,
    pub conditions_checked: usize,
    pub finding_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMatcherResult {
    pub matched_rules: usize,
    pub findings: Vec<StructuralFinding>,
    pub statistics: ExecutionStatistics,
    pub elapsed_ms: u64,
}
