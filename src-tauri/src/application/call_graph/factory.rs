use super::builder::{BuilderBackend, CallGraphBuilder, CallGraphConfig, StaticCallGraphBuilder};

pub struct CallGraphFactory;

impl CallGraphFactory {
    pub fn create_builder(backend: BuilderBackend, config: CallGraphConfig) -> Box<dyn CallGraphBuilder> {
        match backend {
            BuilderBackend::StaticBuilder => Box::new(StaticCallGraphBuilder::new(config)),
            _ => unimplemented!("Only StaticBuilder is currently implemented."),
        }
    }
}
