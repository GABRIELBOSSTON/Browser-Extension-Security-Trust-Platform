use crate::domain::extraction::{SandboxContext, SandboxValidator, ValidationResult};
use crate::domain::errors::Result;

pub struct ManifestExistsValidator;

impl SandboxValidator for ManifestExistsValidator {
    fn validate(&self, context: &SandboxContext) -> Result<ValidationResult> {
        let manifest_path = context.root_path.join("manifest.json");
        
        if manifest_path.exists() && manifest_path.is_file() {
            Ok(ValidationResult {
                is_valid: true,
                warnings: vec![],
                errors: vec![],
            })
        } else {
            Ok(ValidationResult {
                is_valid: false,
                warnings: vec![],
                errors: vec!["manifest.json is missing from the archive root".to_string()],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use crate::domain::extraction::SandboxId;
    use uuid::Uuid;

    #[test]
    fn test_manifest_exists_validator() {
        let root_path = std::env::temp_dir().join(format!("test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root_path).unwrap();
        
        let context = SandboxContext {
            id: SandboxId(Uuid::new_v4()),
            root_path: root_path.clone(),
        };

        let validator = ManifestExistsValidator;
        
        // Should fail initially
        let res1 = validator.validate(&context).unwrap();
        assert!(!res1.is_valid);
        assert_eq!(res1.errors.len(), 1);
        
        // Should pass after creation
        let _ = File::create(root_path.join("manifest.json")).unwrap();
        let res2 = validator.validate(&context).unwrap();
        assert!(res2.is_valid);
        assert_eq!(res2.errors.len(), 0);
        
        std::fs::remove_dir_all(&root_path).unwrap();
    }
}
