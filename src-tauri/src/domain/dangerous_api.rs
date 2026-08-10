use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::domain::ast_detector::SourceLocation;
use crate::domain::call_graph::FunctionId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DangerousApiId(pub u32);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DangerousApiCategory {
    DynamicExecution,
    TimerExecution,
    ModuleExecution,
    ScriptLoading,
    Unknown,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DangerousApi {
    Eval,
    FunctionConstructor,
    SetTimeout,
    SetInterval,
    DynamicImport,
    ImportScripts,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerousApiCall {
    pub api_id: DangerousApiId,
    pub api: DangerousApi,
    pub category: DangerousApiCategory,
    pub expression_preview: String,
    pub function_id: FunctionId,
    pub source_location: SourceLocation,
    pub call_depth: usize,
    pub argument_count: usize,
    pub is_await: bool,
    pub is_indirect: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DangerousApiInventory {
    pub calls: Vec<DangerousApiCall>,
    pub unique_apis_used: HashSet<DangerousApiId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DangerousApiStatistics {
    pub total_calls: usize,
    pub unique_calls: usize,
    pub unknown_calls: usize,
    pub eval_calls: usize,
    pub function_constructor_calls: usize,
    pub dynamic_import_calls: usize,
    pub timer_string_calls: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerousApiResult {
    pub detector_id: String,
    pub detector_version: String,
    pub inventory: DangerousApiInventory,
    pub statistics: DangerousApiStatistics,
    pub elapsed_ms: u64,
    pub diagnostics: Vec<String>,
}
