use std::path::Path;
use std::sync::Arc;
use crate::domain::extraction::{SandboxContext, SandboxManager, ArchiveExtractorRegistry, SandboxValidator};
use crate::domain::errors::{DomainError, Result};
use crate::infrastructure::extraction::sandbox::SandboxHandle;

pub struct ExtractionService {
    sandbox_manager: Arc<dyn SandboxManager>,
    registry: Arc<dyn ArchiveExtractorRegistry>,
    validators: Vec<Arc<dyn SandboxValidator>>,
}

impl ExtractionService {
    pub fn new(
        sandbox_manager: Arc<dyn SandboxManager>,
        registry: Arc<dyn ArchiveExtractorRegistry>,
        validators: Vec<Arc<dyn SandboxValidator>>,
    ) -> Self {
        Self {
            sandbox_manager,
            registry,
            validators,
        }
    }

    /// Unpacks an archive into a sandbox and returns the SandboxHandle.
    /// The SandboxHandle uses RAII to clean up the sandbox when dropped.
    pub fn unpack_for_analysis(&self, source: &Path) -> Result<SandboxHandle> {
        let sandbox_context = self.sandbox_manager.create_sandbox()?;
        
        // Wrap it in the RAII handle immediately so it cleans up on error returns
        let handle = SandboxHandle::new(sandbox_context.clone(), self.sandbox_manager.clone());
        
        let extractor = self.registry.get_extractor(source)?;

        extractor.extract(source, &sandbox_context)?;
        
        let mut all_errors = Vec::new();
        for validator in &self.validators {
            let result = validator.validate(&sandbox_context)?;
            if !result.is_valid {
                all_errors.extend(result.errors);
            }
        }
        
        if !all_errors.is_empty() {
            // handle will be dropped here, automatically triggering cleanup
            return Err(DomainError::ValidationFailed(all_errors));
        }

        Ok(handle)
    }
}
