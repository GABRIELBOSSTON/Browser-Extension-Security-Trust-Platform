use serde::{Deserialize, Serialize};
use crate::domain::risk::RiskProfile;
use crate::domain::risk_calculator::RiskAssessment;
use crate::domain::entities::{DiscoveredExtension, Manifest};
use crate::domain::capabilities::ExtensionCapabilityModel;
use crate::application::rules::models::RuleEvaluationResult;
use std::collections::HashMap;

/// Result of a single stage's execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageResult {
    pub stage_name: String,
    pub status: String,
    pub elapsed_ms: u64,
    pub warning_count: usize,
    pub error: Option<String>,
}

/// Metadata about the pipeline execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    pub pipeline_id: String,
    pub pipeline_version: String,
    pub started_at: u64,
    pub finished_at: u64,
    pub elapsed_ms: u64,
    pub engine_versions: HashMap<String, String>,
    pub os: String,
    pub browser: String,
    pub browser_profile: String,
    pub rule_set_version: String,
    pub risk_profile: RiskProfile,
}

/// The final result of evaluating a single extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub assessment: RiskAssessment,
    pub metadata: PipelineMetadata,
    pub target_info: DiscoveredExtension,
    pub stage_results: Vec<StageResult>,
}

/// The result of evaluating a batch of extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPipelineResult {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
    pub results: Vec<PipelineResult>,
    pub errors: Vec<String>,
}

/// Mutable context object passed between pipeline stages
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub target: DiscoveredExtension,
    pub manifest: Option<Manifest>,
    pub capability_model: Option<ExtensionCapabilityModel>,
    pub rule_evaluation: Option<RuleEvaluationResult>,
    pub risk_assessment: Option<RiskAssessment>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub timings: HashMap<String, u64>,
    pub debug_information: HashMap<String, String>,
}

impl AnalysisContext {
    pub fn new(target: DiscoveredExtension) -> Self {
        Self {
            target,
            manifest: None,
            capability_model: None,
            rule_evaluation: None,
            risk_assessment: None,
            warnings: Vec::new(),
            notes: Vec::new(),
            timings: HashMap::new(),
            debug_information: HashMap::new(),
        }
    }
}
