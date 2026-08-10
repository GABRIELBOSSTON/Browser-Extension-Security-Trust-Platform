use crate::domain::errors::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SandboxId(pub Uuid);

/// Pure data object, entirely ignorant of filesystem lifecycles
#[derive(Debug, Clone)]
pub struct SandboxContext {
    pub id: SandboxId,
    pub root_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub trait SandboxManager: Send + Sync {
    /// Creates a unique, ephemeral sandbox directory
    fn create_sandbox(&self) -> Result<SandboxContext>;

    /// Cleans up the sandbox directory explicitly
    fn destroy_sandbox(&self, context: &SandboxContext) -> Result<()>;
}

pub trait ArchiveExtractor: Send + Sync {
    /// Extracts the archive securely into `destination_sandbox`
    fn extract(&self, source_archive: &Path, destination_sandbox: &SandboxContext)
        -> Result<usize>;

    /// Identifies if this extractor supports the given file signature by inspecting magic bytes
    fn supports(&self, header_bytes: &[u8]) -> bool;
}

pub trait ArchiveExtractorRegistry: Send + Sync {
    fn get_extractor(&self, source_archive: &Path) -> Result<Arc<dyn ArchiveExtractor>>;
}

pub trait SandboxValidator: Send + Sync {
    /// Validates the contents of the sandbox post-extraction
    fn validate(&self, context: &SandboxContext) -> Result<ValidationResult>;
}
