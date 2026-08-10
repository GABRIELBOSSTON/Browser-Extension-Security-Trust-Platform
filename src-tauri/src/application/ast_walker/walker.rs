use crate::domain::ast_visitor::{ASTVisitor, TraversalResult};
use crate::domain::errors::Result;

#[derive(Debug, Clone)]
pub struct WalkerConfig {
    pub visit_comments: bool,
    pub visit_imports: bool,
    pub visit_exports: bool,
    pub max_depth: usize,
    pub cancellation_support: bool,
}

impl Default for WalkerConfig {
    fn default() -> Self {
        Self {
            visit_comments: false,
            visit_imports: true,
            visit_exports: true,
            max_depth: 1000,
            cancellation_support: false,
        }
    }
}

pub trait ASTWalker: Send + Sync {
    fn walk(
        &self,
        source: &str,
        file_path: &str,
        config: &WalkerConfig,
        visitor: &mut dyn ASTVisitor,
    ) -> Result<TraversalResult>;
}
