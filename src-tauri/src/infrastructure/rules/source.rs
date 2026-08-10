use crate::domain::errors::{DomainError, Result};
use crate::domain::rules::RuleSet;
use std::path::Path;

pub trait RuleSource {
    fn fetch(&self) -> Result<RuleSet>;
}

pub struct JsonRuleSource {
    pub file_path: String,
}

impl RuleSource for JsonRuleSource {
    fn fetch(&self) -> Result<RuleSet> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Err(DomainError::IoError(format!(
                "Rule file not found: {}",
                self.file_path
            )));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| DomainError::IoError(format!("Failed to read rule file: {}", e)))?;

        let rule_set: RuleSet = serde_json::from_str(&content).map_err(|e| {
            DomainError::SerializationError(format!("Failed to parse JSON rules: {}", e))
        })?;

        Ok(rule_set)
    }
}

pub struct EmbeddedRuleSource {
    pub raw_json: &'static str,
}

impl RuleSource for EmbeddedRuleSource {
    fn fetch(&self) -> Result<RuleSet> {
        let rule_set: RuleSet = serde_json::from_str(self.raw_json).map_err(|e| {
            DomainError::SerializationError(format!("Failed to parse embedded JSON rules: {}", e))
        })?;

        Ok(rule_set)
    }
}
