use std::path::Path;
use crate::domain::errors::{DomainError, Result};
use crate::domain::ast_visitor::{ASTVisitor, TraversalResult};
use super::walker::{ASTWalker, WalkerConfig};
use super::factory::{WalkerFactory, WalkerBackend};

pub struct ASTWalkerService {
    walker: Box<dyn ASTWalker>,
}

impl ASTWalkerService {
    pub fn new(backend: WalkerBackend) -> Self {
        Self {
            walker: WalkerFactory::create_walker(backend),
        }
    }

    pub fn walk_file(
        &self,
        file_path: &Path,
        config: &WalkerConfig,
        visitor: &mut dyn ASTVisitor,
    ) -> Result<TraversalResult> {
        let source = std::fs::read_to_string(file_path)
            .map_err(|e| DomainError::IoError(e.to_string()))?;
        self.walker.walk(&source, file_path.to_string_lossy().as_ref(), config, visitor)
    }

    pub fn walk_source(
        &self,
        source: &str,
        file_name: &str,
        config: &WalkerConfig,
        visitor: &mut dyn ASTVisitor,
    ) -> Result<TraversalResult> {
        self.walker.walk(source, file_name, config, visitor)
    }
}
