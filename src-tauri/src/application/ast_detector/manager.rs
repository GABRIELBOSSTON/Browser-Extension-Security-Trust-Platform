use crate::domain::ast_detector::{AstDetector, DetectorContext, DetectorResult};
use crate::domain::ast_visitor::{AstNodeKind, ASTVisitor, TraversalLifecycleEvent, VisitorContext};

pub struct DetectorRegistry {
    detectors: Vec<Box<dyn AstDetector>>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }

    pub fn add_detector(&mut self, detector: Box<dyn AstDetector>) {
        self.detectors.push(detector);
    }

    pub fn get_detectors_mut(&mut self) -> &mut [Box<dyn AstDetector>] {
        &mut self.detectors
    }
}

pub struct DetectorManager {
    registry: DetectorRegistry,
    context: DetectorContext,
    aggregated_results: Vec<DetectorResult>,
}

impl DetectorManager {
    pub fn new(registry: DetectorRegistry) -> Self {
        Self {
            registry,
            context: DetectorContext::default(),
            aggregated_results: Vec::new(),
        }
    }

    pub fn take_results(self) -> Vec<DetectorResult> {
        self.aggregated_results
    }

    fn sync_context(&mut self, walker_ctx: &VisitorContext) {
        self.context.current_file = walker_ctx.current_file.clone();
        self.context.current_function = walker_ctx.current_function.clone();
        self.context.scope_depth = walker_ctx.scope_depth;
        self.context.node_stack = walker_ctx.parent_stack.clone();
    }
}

impl ASTVisitor for DetectorManager {
    fn enter(&mut self, node: AstNodeKind, walker_ctx: &mut VisitorContext) {
        self.sync_context(walker_ctx);
        for det in self.registry.get_detectors_mut() {
            if det.supported_nodes().contains(&node) {
                det.enter(node.clone(), &mut self.context);
            }
        }
    }

    fn leave(&mut self, node: AstNodeKind, walker_ctx: &mut VisitorContext) {
        self.sync_context(walker_ctx);
        for det in self.registry.get_detectors_mut() {
            if det.supported_nodes().contains(&node) {
                det.leave(node.clone(), &mut self.context);
            }
        }
    }

    fn lifecycle(&mut self, event: TraversalLifecycleEvent, walker_ctx: &mut VisitorContext) {
        self.sync_context(walker_ctx);
        
        if event == TraversalLifecycleEvent::TraversalFinished || event == TraversalLifecycleEvent::TraversalCancelled {
            for det in self.registry.get_detectors_mut() {
                let mut result = det.finish(&mut self.context);
                if event == TraversalLifecycleEvent::TraversalCancelled {
                    result.cancelled = true;
                }
                self.aggregated_results.push(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast_detector::{DetectionFinding, SourceLocation};

    struct MockDetectorA {
        visited: usize,
    }

    impl AstDetector for MockDetectorA {
        fn detector_id(&self) -> &str { "MOCK-001" }
        fn detector_name(&self) -> &str { "Mock Detector A" }
        fn supported_nodes(&self) -> &'static [AstNodeKind] {
            &[AstNodeKind::CallExpression]
        }
        fn enter(&mut self, node: AstNodeKind, _context: &mut DetectorContext) {
            if node == AstNodeKind::CallExpression {
                self.visited += 1;
            }
        }
        fn leave(&mut self, _node: AstNodeKind, _context: &mut DetectorContext) {}
        fn finish(&mut self, _context: &mut DetectorContext) -> DetectorResult {
            let mut findings = Vec::new();
            if self.visited > 0 {
                findings.push(DetectionFinding {
                    finding_id: "F-001".to_string(),
                    node_kind: AstNodeKind::CallExpression,
                    location: SourceLocation { line: 1, column: 1, start_offset: 0, end_offset: 10 },
                    message: "Found call expression".to_string(),
                    metadata: std::collections::HashMap::new(),
                });
            }
            DetectorResult {
                detector_id: self.detector_id().to_string(),
                findings,
                statistics: std::collections::HashMap::new(),
                warnings: Vec::new(),
                elapsed_ms: 5,
                visited_nodes: self.visited,
                skipped_nodes: 0,
                cancelled: false,
            }
        }
    }

    struct MockDetectorB {
        visited: usize,
    }

    impl AstDetector for MockDetectorB {
        fn detector_id(&self) -> &str { "MOCK-002" }
        fn detector_name(&self) -> &str { "Mock Detector B" }
        fn supported_nodes(&self) -> &'static [AstNodeKind] {
            &[AstNodeKind::FunctionDeclaration]
        }
        fn enter(&mut self, node: AstNodeKind, _context: &mut DetectorContext) {
            if node == AstNodeKind::FunctionDeclaration {
                self.visited += 1;
            }
        }
        fn leave(&mut self, _node: AstNodeKind, _context: &mut DetectorContext) {}
        fn finish(&mut self, _context: &mut DetectorContext) -> DetectorResult {
            DetectorResult {
                detector_id: self.detector_id().to_string(),
                findings: Vec::new(),
                statistics: std::collections::HashMap::new(),
                warnings: Vec::new(),
                elapsed_ms: 2,
                visited_nodes: self.visited,
                skipped_nodes: 0,
                cancelled: false,
            }
        }
    }

    #[test]
    fn test_detector_dispatch_and_aggregation() {
        let mut registry = DetectorRegistry::new();
        registry.add_detector(Box::new(MockDetectorA { visited: 0 }));
        registry.add_detector(Box::new(MockDetectorB { visited: 0 }));

        let mut manager = DetectorManager::new(registry);
        let mut walker_ctx = VisitorContext::default();

        // Dispatch an event supported only by Detector A
        manager.enter(AstNodeKind::CallExpression, &mut walker_ctx);
        // Dispatch an event supported by neither (won't increase counts)
        manager.enter(AstNodeKind::VariableDeclaration, &mut walker_ctx);

        manager.lifecycle(TraversalLifecycleEvent::TraversalFinished, &mut walker_ctx);

        let results = manager.take_results();
        assert_eq!(results.len(), 2);
        
        let result_a = results.iter().find(|r| r.detector_id == "MOCK-001").unwrap();
        assert_eq!(result_a.visited_nodes, 1);
        assert_eq!(result_a.findings.len(), 1);

        let result_b = results.iter().find(|r| r.detector_id == "MOCK-002").unwrap();
        assert_eq!(result_b.visited_nodes, 0);
        assert_eq!(result_b.findings.len(), 0);
    }
}
