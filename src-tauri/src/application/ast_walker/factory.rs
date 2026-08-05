use super::walker::ASTWalker;
use crate::infrastructure::ast::swc_walker::SWCAstWalker;

pub enum WalkerBackend {
    RecursiveWalker,
    FastWalker,
    StreamingWalker,
    MetricsWalker,
}

pub struct WalkerFactory;

impl WalkerFactory {
    pub fn create_walker(backend: WalkerBackend) -> Box<dyn ASTWalker> {
        match backend {
            WalkerBackend::RecursiveWalker => Box::new(SWCAstWalker::new()),
            _ => unimplemented!("Only RecursiveWalker (SWC) is currently implemented."),
        }
    }
}
