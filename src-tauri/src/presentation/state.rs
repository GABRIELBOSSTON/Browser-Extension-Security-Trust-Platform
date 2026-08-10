use crate::application::pipeline::AnalysisPipeline;
use std::sync::Arc;

pub struct AppState {
    pub analysis_pipeline: Arc<AnalysisPipeline>,
}

impl AppState {
    pub fn new(analysis_pipeline: Arc<AnalysisPipeline>) -> Self {
        Self { analysis_pipeline }
    }
}
