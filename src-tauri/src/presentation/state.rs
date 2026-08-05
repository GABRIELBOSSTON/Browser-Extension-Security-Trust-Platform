use std::sync::Arc;
use crate::application::pipeline::AnalysisPipeline;

pub struct AppState {
    pub analysis_pipeline: Arc<AnalysisPipeline>,
}

impl AppState {
    pub fn new(analysis_pipeline: Arc<AnalysisPipeline>) -> Self {
        Self { analysis_pipeline }
    }
}
