use crate::domain::errors::{DomainError, Result};
use crate::infrastructure::manifest::models::RawManifest;

pub struct ManifestValidator;

impl ManifestValidator {
    pub fn validate(raw: &RawManifest) -> Result<()> {
        let version = raw.manifest_version
            .ok_or_else(|| DomainError::MissingRequiredField("manifest_version".into()))?;

        if version != 2 && version != 3 {
            return Err(DomainError::UnsupportedManifestVersion(version));
        }

        if raw.name.is_none() {
            return Err(DomainError::MissingRequiredField("name".into()));
        }

        if raw.version.is_none() {
            return Err(DomainError::MissingRequiredField("version".into()));
        }

        Ok(())
    }
}
