use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::ast_detector::SourceLocation;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionId(pub u64);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionKind {
    FunctionDeclaration,
    FunctionExpression,
    ArrowFunction,
    ClassMethod,
    Constructor,
    Anonymous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    DirectCall,
    MethodCall,
    ConstructorCall,
    ImportCall,
    DynamicCall,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub id: FunctionId,
    pub name: Option<String>,
    pub kind: FunctionKind,
    pub location: SourceLocation,
    pub parameter_count: usize,
    pub is_async: bool,
    pub is_generator: bool,
    pub is_exported: bool,
    pub visibility: Option<Visibility>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSite {
    pub location: SourceLocation,
    pub callee_name: Option<String>,
    pub arguments_count: usize,
    pub is_indirect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    pub id: EdgeId,
    pub caller: FunctionId,
    pub callee: FunctionId,
    pub edge_type: EdgeType,
    pub call_sites: Vec<CallSite>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphIndex {
    pub incoming: HashMap<FunctionId, Vec<EdgeId>>,
    pub outgoing: HashMap<FunctionId, Vec<EdgeId>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallGraph {
    pub nodes: HashMap<FunctionId, CallNode>,
    pub edges: Vec<CallEdge>,
    pub index: GraphIndex,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallGraphMetadata {
    pub graph_version: u32,
    pub builder_version: String,

    pub node_count: usize,
    pub edge_count: usize,

    pub disconnected_nodes: usize,
    pub isolated_components: usize,
    pub recursive_functions: usize,
    pub entry_points: usize,
    pub orphan_functions: usize,

    pub max_call_depth: usize,
    pub max_fan_in: usize,
    pub max_fan_out: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphStatistics {
    pub distinct_call_sites: usize,
    pub average_fan_out: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionStatistics {
    pub elapsed_ms: u64,
    pub events_processed: usize,
    pub memory_allocated_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphResult {
    pub graph: CallGraph,
    pub metadata: CallGraphMetadata,
    pub graph_statistics: GraphStatistics,
    pub execution_statistics: ExecutionStatistics,
    pub diagnostics: Vec<String>,
    pub warnings: Vec<String>,
}
