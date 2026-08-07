use std::time::Instant;
use std::collections::HashMap;

use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, EsSyntax};
use swc_ecma_visit::{Visit, VisitWith};

use crate::application::ast_walker::{ASTWalker, WalkerConfig};
use crate::domain::ast_visitor::{
    ASTVisitor, AstNodeKind, TraversalLifecycleEvent, TraversalResult, VisitorContext,
};
use crate::domain::errors::{DomainError, Result};

pub struct SWCAstWalker;

impl SWCAstWalker {
    pub fn new() -> Self {
        Self
    }
}

struct SWCVisitorWrapper<'a> {
    visitor: &'a mut dyn ASTVisitor,
    context: VisitorContext,
    config: WalkerConfig,
    cancelled: bool,
    max_depth_reached: usize,
    diagnostics: Vec<String>,
    per_node_statistics: HashMap<String, usize>,
}

impl<'a> SWCVisitorWrapper<'a> {
    fn check_bounds_and_visit<T: VisitWith<Self>>(&mut self, kind: AstNodeKind, n: &T) {
        if self.cancelled {
            return;
        }

        self.context.current_depth += 1;
        self.context.visited_count += 1;
        
        if self.context.current_depth > self.max_depth_reached {
            self.max_depth_reached = self.context.current_depth;
        }

        let kind_str = format!("{:?}", kind);
        *self.per_node_statistics.entry(kind_str).or_insert(0) += 1;

        self.context.parent_stack.push(kind.clone());
        self.visitor.enter(kind.clone(), &mut self.context);

        if self.context.current_depth < self.config.max_depth {
            n.visit_children_with(self);
        } else {
            self.diagnostics.push("Max depth reached, pruning traversal.".to_string());
        }

        self.visitor.leave(kind.clone(), &mut self.context);
        self.context.parent_stack.pop();
        self.context.current_depth -= 1;

        // Simplified cancellation check hook
        if self.config.cancellation_support && self.context.visited_count > 100000 {
            self.cancelled = true;
        }
    }
}

impl<'a> Visit for SWCVisitorWrapper<'a> {
    fn visit_module(&mut self, n: &swc_ecma_ast::Module) {
        self.check_bounds_and_visit(AstNodeKind::Module, n);
    }
    fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
        self.check_bounds_and_visit(AstNodeKind::FunctionDeclaration, n);
    }
    fn visit_var_decl(&mut self, n: &swc_ecma_ast::VarDecl) {
        self.check_bounds_and_visit(AstNodeKind::VariableDeclaration, n);
    }
    fn visit_call_expr(&mut self, n: &swc_ecma_ast::CallExpr) {
        self.check_bounds_and_visit(AstNodeKind::CallExpression, n);
    }
    fn visit_member_expr(&mut self, n: &swc_ecma_ast::MemberExpr) {
        self.check_bounds_and_visit(AstNodeKind::MemberExpression, n);
    }
    fn visit_ident(&mut self, n: &swc_ecma_ast::Ident) {
        self.check_bounds_and_visit(AstNodeKind::Identifier, n);
    }
    fn visit_import_decl(&mut self, n: &swc_ecma_ast::ImportDecl) {
        if self.config.visit_imports {
            self.check_bounds_and_visit(AstNodeKind::ImportDeclaration, n);
        }
    }
    fn visit_export_decl(&mut self, n: &swc_ecma_ast::ExportDecl) {
        if self.config.visit_exports {
            self.check_bounds_and_visit(AstNodeKind::ExportDeclaration, n);
        }
    }
    fn visit_class_decl(&mut self, n: &swc_ecma_ast::ClassDecl) {
        self.check_bounds_and_visit(AstNodeKind::ClassDeclaration, n);
    }
    fn visit_arrow_expr(&mut self, n: &swc_ecma_ast::ArrowExpr) {
        self.check_bounds_and_visit(AstNodeKind::ArrowFunction, n);
    }
    fn visit_new_expr(&mut self, n: &swc_ecma_ast::NewExpr) {
        self.check_bounds_and_visit(AstNodeKind::NewExpression, n);
    }
}

impl ASTWalker for SWCAstWalker {
    fn walk(
        &self,
        source: &str,
        file_path: &str,
        config: &WalkerConfig,
        visitor: &mut dyn ASTVisitor,
    ) -> Result<TraversalResult> {
        let start = Instant::now();
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(Lrc::new(FileName::Custom(file_path.into())), source.to_string());

        // Basic parser just for the walker string
        let lexer = Lexer::new(
            Syntax::Es(EsSyntax {
                jsx: true,
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = match parser.parse_module() {
            Ok(m) => m,
            Err(e) => return Err(DomainError::IoError(format!("Parse error: {:?}", e.into_kind().msg()))),
        };

        let mut context = VisitorContext::default();
        context.current_file = Some(file_path.to_string());
        
        visitor.lifecycle(TraversalLifecycleEvent::TraversalStarted, &mut context);

        let mut wrapper = SWCVisitorWrapper {
            visitor,
            context,
            config: config.clone(),
            cancelled: false,
            max_depth_reached: 0,
            diagnostics: Vec::new(),
            per_node_statistics: HashMap::new(),
        };

        wrapper.visit_module(&module);

        let mut final_context = wrapper.context;
        
        if wrapper.cancelled {
            wrapper.visitor.lifecycle(TraversalLifecycleEvent::TraversalCancelled, &mut final_context);
        } else {
            wrapper.visitor.lifecycle(TraversalLifecycleEvent::TraversalFinished, &mut final_context);
        }

        let duration = start.elapsed();

        Ok(TraversalResult {
            visited_node_count: final_context.visited_count,
            traversal_duration_ms: duration.as_millis() as u64,
            diagnostics: wrapper.diagnostics,
            warnings: Vec::new(),
            per_node_statistics: wrapper.per_node_statistics,
            max_depth: wrapper.max_depth_reached,
            cancelled: wrapper.cancelled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast_visitor::{AstNodeKind, ASTVisitor, TraversalLifecycleEvent, VisitorContext};
    
    struct MockVisitor {
        pub enter_counts: HashMap<String, usize>,
        pub lifecycle_events: Vec<TraversalLifecycleEvent>,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            Self {
                enter_counts: HashMap::new(),
                lifecycle_events: Vec::new(),
            }
        }
    }
    
    impl ASTVisitor for MockVisitor {
        fn enter(&mut self, node: AstNodeKind, _context: &mut VisitorContext) {
            *self.enter_counts.entry(format!("{:?}", node)).or_insert(0) += 1;
        }
        
        fn leave(&mut self, _node: AstNodeKind, _context: &mut VisitorContext) {}
        
        fn lifecycle(&mut self, event: TraversalLifecycleEvent, _context: &mut VisitorContext) {
            self.lifecycle_events.push(event);
        }
    }

    #[test]
    fn test_walk_nested_functions() {
        let walker = SWCAstWalker::new();
        let source = "function a() { function b() { return 1; } }";
        let config = WalkerConfig::default();
        let mut visitor = MockVisitor::new();
        
        let result = walker.walk(source, "test.js", &config, &mut visitor).unwrap();
        
        assert_eq!(*visitor.enter_counts.get("FunctionDeclaration").unwrap_or(&0), 2);
        assert!(!result.cancelled);
        assert!(result.visited_node_count >= 2);
        assert_eq!(visitor.lifecycle_events[0], TraversalLifecycleEvent::TraversalStarted);
        assert_eq!(visitor.lifecycle_events[1], TraversalLifecycleEvent::TraversalFinished);
    }

    #[test]
    fn test_walk_imports_exports() {
        let walker = SWCAstWalker::new();
        let source = "import { x } from 'y'; export const z = 1;";
        let config = WalkerConfig::default();
        let mut visitor = MockVisitor::new();
        
        let _ = walker.walk(source, "test.js", &config, &mut visitor).unwrap();
        
        assert_eq!(*visitor.enter_counts.get("ImportDeclaration").unwrap_or(&0), 1);
        assert_eq!(*visitor.enter_counts.get("ExportDeclaration").unwrap_or(&0), 1);
    }
}
