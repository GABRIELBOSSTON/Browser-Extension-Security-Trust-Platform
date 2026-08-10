use crate::domain::errors::{DomainError, Result};
use crate::domain::extraction::{SandboxContext, SandboxId, SandboxManager};
use std::env;
use std::fs;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

pub struct TempSandboxManager;

impl SandboxManager for TempSandboxManager {
    fn create_sandbox(&self) -> Result<SandboxContext> {
        let id = SandboxId(Uuid::new_v4());
        let mut path = env::temp_dir();
        path.push(format!("aep-sandbox-{}", id.0));

        fs::create_dir_all(&path).map_err(|e| {
            DomainError::IoError(format!("Failed to create sandbox directory: {}", e))
        })?;

        info!("Created ephemeral sandbox at {:?}", path);

        Ok(SandboxContext {
            id,
            root_path: path,
        })
    }

    fn destroy_sandbox(&self, context: &SandboxContext) -> Result<()> {
        if context.root_path.exists() {
            fs::remove_dir_all(&context.root_path)
                .map_err(|e| DomainError::IoError(format!("Failed to cleanup sandbox: {}", e)))?;
            info!("Destroyed sandbox {:?}", context.root_path);
        }
        Ok(())
    }
}

/// Infrastructure-owned wrapper that implements RAII Drop for guaranteed cleanup
pub struct SandboxHandle {
    pub context: SandboxContext,
    manager: Arc<dyn SandboxManager>,
}

impl SandboxHandle {
    pub fn new(context: SandboxContext, manager: Arc<dyn SandboxManager>) -> Self {
        Self { context, manager }
    }
}

impl Drop for SandboxHandle {
    fn drop(&mut self) {
        if let Err(e) = self.manager.destroy_sandbox(&self.context) {
            warn!(
                "RAII Drop failed to clean up sandbox {:?}: {}",
                self.context.root_path, e
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_sandbox_lifecycle() {
        let manager = Arc::new(TempSandboxManager);
        let context = manager.create_sandbox().unwrap();
        let path = context.root_path.clone();

        assert!(path.exists());

        {
            let _handle = SandboxHandle::new(context, manager);
            // Handle holds it, still exists
            assert!(path.exists());
        }

        // Handle dropped, should be deleted
        assert!(!path.exists());
    }
}
