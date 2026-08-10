use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use super::ast_visitor::AstNodeKind;

#[derive(Debug, Clone, Default)]
pub struct DetectorContext {
    pub current_file: Option<String>,
    pub current_function: Option<String>,
    pub scope_depth: usize,
    pub node_stack: Vec<AstNodeKind>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionFinding {
    pub finding_id: String,
    pub node_kind: AstNodeKind,
    pub location: SourceLocation,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DetectorResult {
    pub detector_id: String,
    pub findings: Vec<DetectionFinding>,
    pub statistics: HashMap<String, usize>,
    pub warnings: Vec<String>,

    // Telemetry and Execution Metadata
    pub elapsed_ms: u64,
    pub visited_nodes: usize,
    pub skipped_nodes: usize,
    pub cancelled: bool,
}

pub trait AstDetector: Send + Sync {
    fn detector_id(&self) -> &str;
    fn detector_name(&self) -> &str;

    // Zero-allocation node dispatch filtering
    fn supported_nodes(&self) -> &'static [AstNodeKind];

    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext);
    fn leave(&mut self, node: AstNodeKind, context: &mut DetectorContext);
    fn finish(&mut self, context: &mut DetectorContext) -> DetectorResult;
}
