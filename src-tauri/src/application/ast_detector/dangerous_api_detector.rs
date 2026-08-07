use std::time::Instant;
use std::collections::HashSet;

use crate::domain::ast_detector::{AstDetector, DetectorContext, DetectorResult, AstNodeKind, SourceLocation};
use crate::domain::dangerous_api::*;
use crate::domain::call_graph::FunctionId;

pub struct DangerousApiDetector {
    calls: Vec<DangerousApiCall>,
    start_time: Instant,
}

impl Default for DangerousApiDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DangerousApiDetector {
    pub fn new() -> Self {
        Self {
            calls: Vec::new(),
            start_time: Instant::now(),
        }
    }

    fn resolve_api(raw_name: &str) -> (DangerousApi, DangerousApiCategory, DangerousApiId) {
        match raw_name {
            "eval" => (DangerousApi::Eval, DangerousApiCategory::DynamicExecution, DangerousApiId(100)),
            "Function" => (DangerousApi::FunctionConstructor, DangerousApiCategory::DynamicExecution, DangerousApiId(101)),
            "setTimeout" => (DangerousApi::SetTimeout, DangerousApiCategory::TimerExecution, DangerousApiId(200)),
            "setInterval" => (DangerousApi::SetInterval, DangerousApiCategory::TimerExecution, DangerousApiId(201)),
            "import" => (DangerousApi::DynamicImport, DangerousApiCategory::ModuleExecution, DangerousApiId(300)),
            "importScripts" => (DangerousApi::ImportScripts, DangerousApiCategory::ScriptLoading, DangerousApiId(400)),
            _ => {
                // FNV-1a hash for deterministic Unknown ID
                let mut hash: u32 = 0x811c9dc5;
                for byte in raw_name.bytes() {
                    hash ^= byte as u32;
                    hash = hash.wrapping_mul(0x01000193);
                }
                (DangerousApi::Unknown, DangerousApiCategory::Unknown, DangerousApiId(hash))
            }
        }
    }

    fn truncate_preview(preview: &str) -> String {
        if preview.len() > 128 {
            let mut s = preview.chars().take(125).collect::<String>();
            s.push_str("...");
            s
        } else {
            preview.to_string()
        }
    }
}

impl AstDetector for DangerousApiDetector {
    fn detector_id(&self) -> &str { "DET-DANGEROUS-API-001" }
    fn detector_name(&self) -> &str { "Dangerous JavaScript API Foundation" }
    
    fn supported_nodes(&self) -> &'static [AstNodeKind] {
        &[
            AstNodeKind::CallExpression, 
            AstNodeKind::NewExpression, 
            AstNodeKind::ImportExpression
        ]
    }
    
    fn enter(&mut self, node: AstNodeKind, context: &mut DetectorContext) {
        if let Some(raw_name) = context.metadata.get("simulated_dangerous_call") {
            let (api, category, api_id) = Self::resolve_api(raw_name);
            
            // In a real walker, we'd grab the expression text from the AST node
            let expression_preview = Self::truncate_preview(context.metadata.get("simulated_expression").unwrap_or(raw_name));
            
            self.calls.push(DangerousApiCall {
                api_id,
                api,
                category,
                expression_preview,
                function_id: FunctionId(0),
                source_location: SourceLocation { line: 1, column: 1, start_offset: 0, end_offset: 10 },
                call_depth: context.scope_depth,
                argument_count: 1,
                is_await: node == AstNodeKind::ImportExpression, // simplification for simulation
                is_indirect: false,
            });
        }
    }

    fn leave(&mut self, _node: AstNodeKind, _context: &mut DetectorContext) {}
    
    fn finish(&mut self, _context: &mut DetectorContext) -> DetectorResult {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        let total_calls = self.calls.len();
        
        let mut unique_apis = HashSet::new();
        let mut unknown_calls = 0;
        let mut eval_calls = 0;
        let mut function_constructor_calls = 0;
        let mut dynamic_import_calls = 0;
        let mut timer_string_calls = 0;
        
        for call in &self.calls {
            unique_apis.insert(call.api_id);
            match call.api {
                DangerousApi::Unknown => unknown_calls += 1,
                DangerousApi::Eval => eval_calls += 1,
                DangerousApi::FunctionConstructor => function_constructor_calls += 1,
                DangerousApi::DynamicImport => dynamic_import_calls += 1,
                DangerousApi::SetTimeout | DangerousApi::SetInterval => timer_string_calls += 1,
                _ => {}
            }
        }

        let _statistics = DangerousApiStatistics {
            total_calls,
            unique_calls: unique_apis.len(),
            unknown_calls,
            eval_calls,
            function_constructor_calls,
            dynamic_import_calls,
            timer_string_calls,
        };

        let _inventory = DangerousApiInventory {
            calls: std::mem::take(&mut self.calls),
            unique_apis_used: unique_apis,
        };

        DetectorResult {
            detector_id: self.detector_id().to_string(),
            findings: Vec::new(),
            statistics: std::collections::HashMap::new(),
            warnings: Vec::new(),
            elapsed_ms,
            visited_nodes: total_calls,
            skipped_nodes: 0,
            cancelled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dangerous_api_extraction_and_statistics() {
        let mut detector = DangerousApiDetector::new();
        let mut context = DetectorContext::default();
        
        // 1. Eval
        context.metadata.insert("simulated_dangerous_call".to_string(), "eval".to_string());
        detector.enter(AstNodeKind::CallExpression, &mut context);
        
        // 2. Function Constructor
        context.metadata.insert("simulated_dangerous_call".to_string(), "Function".to_string());
        detector.enter(AstNodeKind::NewExpression, &mut context);

        // 3. SetTimeout
        context.metadata.insert("simulated_dangerous_call".to_string(), "setTimeout".to_string());
        detector.enter(AstNodeKind::CallExpression, &mut context);

        let result = detector.finish(&mut context);
        assert_eq!(result.visited_nodes, 3);
    }
    
    #[test]
    fn test_expression_preview_truncation() {
        let preview = DangerousApiDetector::truncate_preview(&"A".repeat(200));
        assert_eq!(preview.len(), 128); // 125 chars + "..."
        assert!(preview.ends_with("..."));
    }
    
    #[test]
    fn test_dangerous_statistics_consistency() {
        let mut detector = DangerousApiDetector::new();
        let mut context = DetectorContext::default();
        
        context.metadata.insert("simulated_dangerous_call".to_string(), "eval".to_string());
        detector.enter(AstNodeKind::CallExpression, &mut context);
        
        context.metadata.insert("simulated_dangerous_call".to_string(), "import".to_string());
        detector.enter(AstNodeKind::ImportExpression, &mut context);
        
        let result = detector.finish(&mut context);
        assert_eq!(result.visited_nodes, 2);
    }
}
