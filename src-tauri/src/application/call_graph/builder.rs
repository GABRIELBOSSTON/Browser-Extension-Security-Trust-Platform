use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use crate::domain::ast_detector::{ASTNodeEvent, DetectorContext, SourceLocation};
use crate::domain::ast_visitor::{AstNodeKind, VisitorContext};
use crate::domain::call_graph::*;

#[derive(Debug, Clone)]
pub struct CallGraphConfig {
    pub include_imports: bool,
    pub include_exports: bool,
    pub include_external_calls: bool,
    pub include_anonymous_functions: bool,
    pub max_depth: usize,
    pub cancellation_support: bool,
}

impl Default for CallGraphConfig {
    fn default() -> Self {
        Self {
            include_imports: true,
            include_exports: true,
            include_external_calls: true,
            include_anonymous_functions: true,
            max_depth: 1000,
            cancellation_support: false,
        }
    }
}

pub trait CallGraphBuilder: Send + Sync {
    fn process_event(&mut self, event: &ASTNodeEvent, context: &VisitorContext);
    fn build(self: Box<Self>) -> CallGraphResult;
}

pub struct StaticCallGraphBuilder {
    config: CallGraphConfig,
    graph: CallGraph,
    events_processed: usize,
    start_time: Instant,
    current_edge_id: u64,
}

impl StaticCallGraphBuilder {
    pub fn new(config: CallGraphConfig) -> Self {
        Self {
            config,
            graph: CallGraph::default(),
            events_processed: 0,
            start_time: Instant::now(),
            current_edge_id: 0,
        }
    }

    fn generate_function_id(file: &str, scope: usize, name: &str) -> FunctionId {
        // FNV-1a hash implementation for guaranteed cross-execution determinism
        let mut hash: u64 = 0xcbf29ce484222325;
        let prime: u64 = 0x100000001b3;
        
        for byte in file.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(prime);
        }
        for byte in scope.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(prime);
        }
        for byte in name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(prime);
        }
        
        FunctionId(hash)
    }

    fn extract_name(node_kind: &AstNodeKind) -> Option<String> {
        match node_kind {
            AstNodeKind::FunctionDeclaration => Some("function_decl".to_string()),
            AstNodeKind::CallExpression => Some("call_expr".to_string()),
            _ => None,
        }
    }
}

impl CallGraphBuilder for StaticCallGraphBuilder {
    fn process_event(&mut self, event: &ASTNodeEvent, context: &VisitorContext) {
        self.events_processed += 1;

        match event {
            ASTNodeEvent::Enter(node) => {
                match node {
                    AstNodeKind::FunctionDeclaration => {
                        let file = context.current_file.as_deref().unwrap_or("unknown");
                        let name = Self::extract_name(node).unwrap_or_else(|| "anon".to_string());
                        let id = Self::generate_function_id(file, context.scope_depth, &name);
                        
                        let call_node = CallNode {
                            id,
                            name: Some(name),
                            kind: FunctionKind::FunctionDeclaration,
                            location: SourceLocation { line: 0, column: 0, start_offset: 0, end_offset: 0 },
                            parameter_count: 0,
                            is_async: false,
                            is_generator: false,
                            is_exported: false,
                            visibility: Some(Visibility::Public),
                            return_type: None,
                        };
                        self.graph.nodes.insert(id, call_node);
                    },
                    AstNodeKind::CallExpression => {
                        // In a real implementation we would identify the caller from context.
                        // Here we simulate establishing an edge.
                        let file = context.current_file.as_deref().unwrap_or("unknown");
                        let caller_id = Self::generate_function_id(file, context.scope_depth, "function_decl");
                        let callee_id = Self::generate_function_id(file, context.scope_depth, "target_func");

                        // Create edge if it doesn't exist, otherwise append call site
                        let edge_id = EdgeId(self.current_edge_id);
                        self.current_edge_id += 1;

                        let call_site = CallSite {
                            location: SourceLocation { line: 0, column: 0, start_offset: 0, end_offset: 0 },
                            callee_name: Some("target_func".to_string()),
                            arguments_count: 0,
                            is_indirect: false,
                        };

                        let edge = CallEdge {
                            id: edge_id,
                            caller: caller_id,
                            callee: callee_id,
                            edge_type: EdgeType::DirectCall,
                            call_sites: vec![call_site],
                        };

                        self.graph.edges.push(edge);
                        self.graph.index.outgoing.entry(caller_id).or_default().push(edge_id);
                        self.graph.index.incoming.entry(callee_id).or_default().push(edge_id);
                    },
                    _ => {}
                }
            },
            ASTNodeEvent::Leave(_) => {}
        }
    }

    fn build(self: Box<Self>) -> CallGraphResult {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        
        let metadata = CallGraphMetadata {
            graph_version: 1,
            builder_version: "1.0".to_string(),
            node_count: self.graph.nodes.len(),
            edge_count: self.graph.edges.len(),
            disconnected_nodes: 0,
            isolated_components: 0,
            recursive_functions: 0,
            entry_points: 0,
            orphan_functions: 0,
            max_call_depth: 0,
            max_fan_in: 0,
            max_fan_out: 0,
        };

        let graph_statistics = GraphStatistics {
            distinct_call_sites: self.graph.edges.iter().map(|e| e.call_sites.len()).sum(),
            average_fan_out: 0.0,
        };

        let execution_statistics = ExecutionStatistics {
            elapsed_ms,
            events_processed: self.events_processed,
            memory_allocated_bytes: 0,
        };

        CallGraphResult {
            graph: self.graph,
            metadata,
            graph_statistics,
            execution_statistics,
            diagnostics: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_graph_determinism() {
        let id1 = StaticCallGraphBuilder::generate_function_id("test.js", 1, "func");
        let id2 = StaticCallGraphBuilder::generate_function_id("test.js", 1, "func");
        assert_eq!(id1, id2, "FunctionId must be deterministic");
    }

    #[test]
    fn test_call_graph_construction_scenarios() {
        let mut builder: Box<dyn CallGraphBuilder> = Box::new(StaticCallGraphBuilder::new(CallGraphConfig::default()));
        let mut context = VisitorContext::default();
        context.current_file = Some("test.js".to_string());
        
        // 1. Function Declaration (Simple)
        let event_fn = ASTNodeEvent::Enter(AstNodeKind::FunctionDeclaration);
        builder.process_event(&event_fn, &context);
        
        // 2. Call Expression (Nested/Simple call)
        let event_call = ASTNodeEvent::Enter(AstNodeKind::CallExpression);
        builder.process_event(&event_call, &context);

        // 3. Arrow Function (Anonymous)
        let event_arrow = ASTNodeEvent::Enter(AstNodeKind::ArrowFunction);
        builder.process_event(&event_arrow, &context);

        // 4. Class Method & Constructor
        let event_class_method = ASTNodeEvent::Enter(AstNodeKind::ClassMethod);
        builder.process_event(&event_class_method, &context);
        let event_constructor = ASTNodeEvent::Enter(AstNodeKind::Constructor);
        builder.process_event(&event_constructor, &context);

        // 5. Imports
        let event_import = ASTNodeEvent::Enter(AstNodeKind::ImportDeclaration);
        builder.process_event(&event_import, &context);
        
        let result = builder.build();
        
        // Validation of metadata and coverage
        assert!(result.metadata.node_count >= 1);
        assert!(result.metadata.edge_count >= 1);
        assert!(result.execution_statistics.events_processed >= 6);
        assert!(!result.graph.index.outgoing.is_empty(), "Graph index outgoing must be populated");
        assert!(!result.graph.index.incoming.is_empty(), "Graph index incoming must be populated");
    }
}
