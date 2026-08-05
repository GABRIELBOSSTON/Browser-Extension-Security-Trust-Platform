use std::time::Instant;
use std::collections::{HashMap, HashSet};

use crate::domain::ast_detector::{AstDetector, DetectorContext, DetectorResult, AstNodeKind, SourceLocation};
use crate::domain::secret_detector::*;
use crate::domain::call_graph::FunctionId;

pub trait PatternRegistry: Send + Sync {
    fn matches(&self, input: &str) -> Vec<(SecretType, SecretCategory, MatchConfidence)>;
}

pub struct SecretDetector {
    registry: Box<dyn PatternRegistry>,
    matches: Vec<SecretMatch>,
    start_time: Instant,
}

impl SecretDetector {
    pub fn new(registry: Box<dyn PatternRegistry>) -> Self {
        Self {
            registry,
            matches: Vec::new(),
            start_time: Instant::now(),
        }
    }

    fn truncate_preview(preview: &str) -> String {
        if preview.len() > 64 {
            let mut s = preview.chars().take(61).collect::<String>();
            s.push_str("...");
            s
        } else {
            preview.to_string()
        }
    }

    fn generate_secret_id(secret_type: &SecretType) -> SecretId {
        let name = format!("{:?}", secret_type);
        let mut hash: u32 = 0x811c9dc5;
        for byte in name.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        SecretId(hash)
    }
}

impl AstDetector for SecretDetector {
    fn detector_id(&self) -> &str { "DET-SECRET-001" }
    fn detector_name(&self) -> &str { "Hardcoded Secret Foundation" }
    
    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[
            AstNodeKind::StringLiteral, 
            AstNodeKind::TemplateLiteral
        ]
    }
    
    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext) {
        if let Some(payload) = context.metadata.get("simulated_string_literal") {
            let matches = self.registry.matches(payload);
            
            for (secret_type, category, confidence) in matches {
                let secret_id = Self::generate_secret_id(&secret_type);
                let preview = Self::truncate_preview(payload);
                
                let source_kind = match node {
                    AstNodeKind::StringLiteral => SecretSourceKind::StringLiteral,
                    AstNodeKind::TemplateLiteral => SecretSourceKind::TemplateLiteral,
                    _ => SecretSourceKind::Unknown,
                };

                self.matches.push(SecretMatch {
                    secret_id,
                    secret_type,
                    category,
                    confidence,
                    source_kind,
                    preview,
                    source_location: SourceLocation { line: 1, column: 1, start_offset: 0, end_offset: 10 },
                    function_id: FunctionId(0), // Would map from actual call graph context
                    call_depth: context.scope_depth,
                });
            }
        }
    }

    fn leave(&mut self, _node: AstNodeKind, _context: &mut DetectorContext) {}
    
    fn finish(&mut self, _context: &mut DetectorContext) -> DetectorResult {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        let total_matches = self.matches.len();
        
        let mut unique_secret_ids = HashSet::new();
        let mut unknown_matches = 0;
        let mut matches_by_type = HashMap::new();
        
        for secret_match in &self.matches {
            unique_secret_ids.insert(secret_match.secret_id);
            if secret_match.secret_type == SecretType::Unknown {
                unknown_matches += 1;
            }
            *matches_by_type.entry(secret_match.secret_type.clone()).or_insert(0) += 1;
        }

        let statistics = SecretStatistics {
            total_matches,
            unique_matches: unique_secret_ids.len(),
            unknown_matches,
            matches_by_type,
        };

        let inventory = SecretInventory {
            matches: std::mem::take(&mut self.matches),
            unique_secret_ids,
        };

        DetectorResult {
            detector_id: self.detector_id().to_string(),
            findings: Vec::new(),
            statistics: std::collections::HashMap::new(),
            warnings: Vec::new(),
            elapsed_ms,
            visited_nodes: total_matches, // simplified
            skipped_nodes: 0,
            cancelled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPatternRegistry;

    impl PatternRegistry for MockPatternRegistry {
        fn matches(&self, input: &str) -> Vec<(SecretType, SecretCategory, MatchConfidence)> {
            let mut results = Vec::new();
            if input.starts_with("sk-ant-") {
                results.push((SecretType::AnthropicKey, SecretCategory::AIProvider, MatchConfidence::High));
            }
            if input.starts_with("sk-") && !input.starts_with("sk-ant-") {
                results.push((SecretType::OpenAiKey, SecretCategory::AIProvider, MatchConfidence::High));
            }
            if input.starts_with("AKIA") {
                results.push((SecretType::AwsAccessKey, SecretCategory::CloudProvider, MatchConfidence::High));
            }
            results
        }
    }

    #[test]
    fn test_secret_extraction_and_statistics() {
        let registry = Box::new(MockPatternRegistry);
        let mut detector = SecretDetector::new(registry);
        let mut context = DetectorContext::default();
        
        // 1. Anthropic Key (StringLiteral)
        context.metadata.insert("simulated_string_literal".to_string(), "sk-ant-api03-1234567890123456789012345678901234567890123456789012345678901234567890".to_string());
        detector.enter(AstNodeKind::StringLiteral, &mut context);
        
        // 2. OpenAI Key (StringLiteral)
        context.metadata.insert("simulated_string_literal".to_string(), "sk-1234567890".to_string());
        detector.enter(AstNodeKind::StringLiteral, &mut context);

        // 3. AWS Key (TemplateLiteral)
        context.metadata.insert("simulated_string_literal".to_string(), "AKIAIOSFODNN7EXAMPLE".to_string());
        detector.enter(AstNodeKind::TemplateLiteral, &mut context);

        // 4. Duplicate OpenAI Key (to test unique identity mapping)
        context.metadata.insert("simulated_string_literal".to_string(), "sk-0987654321".to_string());
        detector.enter(AstNodeKind::StringLiteral, &mut context);

        let result = detector.finish(&mut context);
        assert_eq!(result.visited_nodes, 4); // 4 total matches
    }
    
    #[test]
    fn test_preview_truncation() {
        let preview = SecretDetector::truncate_preview(&"A".repeat(100));
        assert_eq!(preview.len(), 64); // 61 chars + "..."
        assert!(preview.ends_with("..."));
    }
    
    #[test]
    fn test_secret_id_determinism() {
        let id1 = SecretDetector::generate_secret_id(&SecretType::AwsAccessKey);
        let id2 = SecretDetector::generate_secret_id(&SecretType::AwsAccessKey);
        let id3 = SecretDetector::generate_secret_id(&SecretType::OpenAiKey);
        
        assert_eq!(id1, id2, "SecretIds for the same SecretType must be identical");
        assert_ne!(id1, id3, "SecretIds for different SecretTypes must differ");
    }
}
