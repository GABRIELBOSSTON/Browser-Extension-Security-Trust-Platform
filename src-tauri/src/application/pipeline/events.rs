// Pipeline Events (Reserved Architecture)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    PipelineStarted {
        pipeline_id: String,
        total_targets: usize,
    },
    StageStarted {
        pipeline_id: String,
        stage_name: String,
        target_id: String,
    },
    StageCompleted {
        pipeline_id: String,
        stage_name: String,
        target_id: String,
        elapsed_ms: u64,
    },
    StageFailed {
        pipeline_id: String,
        stage_name: String,
        target_id: String,
        error: String,
    },
    PipelineCompleted {
        pipeline_id: String,
        total_success: usize,
        total_failed: usize,
        elapsed_ms: u64,
    },
}
