use std::collections::HashMap;
use std::time::Instant;

use swc_common::{sync::Lrc, FileName, SourceMap, SourceMapper, Spanned};
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax};
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

impl Default for SWCAstWalker {
    fn default() -> Self {
        Self::new()
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
    cm: Lrc<SourceMap>,
}

impl<'a> SWCVisitorWrapper<'a> {
    fn check_bounds_and_visit<T: VisitWith<Self> + Spanned>(
        &mut self,
        kind: AstNodeKind,
        n: &T,
        extract_text: bool,
    ) {
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

        // Extract location and text
        if extract_text {
            let span = n.span();
            let loc = self.cm.lookup_char_pos(span.lo);
            self.context
                .metadata
                .insert("line".to_string(), loc.line.to_string());
            self.context
                .metadata
                .insert("column".to_string(), loc.col_display.to_string());

            if let Ok(snippet) = self.cm.span_to_snippet(span) {
                // Simplify expressions to their string representation
                let mut clean_snippet = snippet.trim().to_string();

                if kind == AstNodeKind::CallExpression || kind == AstNodeKind::NewExpression {
                    // Extract callee for call expressions (e.g. `chrome.tabs.query(...)` -> `chrome.tabs.query`)
                    // Basic heuristic: take everything before the first '('
                    if let Some(idx) = clean_snippet.find('(') {
                        clean_snippet = clean_snippet[..idx].trim().to_string();
                    }
                    self.context
                        .metadata
                        .insert("callee_name".to_string(), clean_snippet.clone());
                } else if kind == AstNodeKind::StringLiteral || kind == AstNodeKind::TemplateLiteral
                {
                    // Remove quotes
                    clean_snippet = clean_snippet
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`')
                        .to_string();
                    self.context.metadata.insert(
                        "simulated_string_literal".to_string(),
                        clean_snippet.clone(),
                    );
                }

                self.context
                    .metadata
                    .insert("expression_text".to_string(), clean_snippet);
            }
        }

        self.visitor.enter(kind.clone(), &mut self.context);

        if self.context.current_depth < self.config.max_depth {
            n.visit_children_with(self);
        } else {
            self.diagnostics
                .push("Max depth reached, pruning traversal.".to_string());
        }

        self.visitor.leave(kind.clone(), &mut self.context);

        // Clean up metadata
        self.context.metadata.remove("line");
        self.context.metadata.remove("column");
        self.context.metadata.remove("expression_text");
        self.context.metadata.remove("callee_name");
        self.context.metadata.remove("simulated_string_literal");

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
        self.check_bounds_and_visit(AstNodeKind::Module, n, false);
    }
    fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
        self.check_bounds_and_visit(AstNodeKind::FunctionDeclaration, n, false);
    }
    fn visit_var_decl(&mut self, n: &swc_ecma_ast::VarDecl) {
        self.check_bounds_and_visit(AstNodeKind::VariableDeclaration, n, false);
    }
    fn visit_call_expr(&mut self, n: &swc_ecma_ast::CallExpr) {
        self.check_bounds_and_visit(AstNodeKind::CallExpression, n, true);
    }
    fn visit_member_expr(&mut self, n: &swc_ecma_ast::MemberExpr) {
        self.check_bounds_and_visit(AstNodeKind::MemberExpression, n, true);
    }
    fn visit_ident(&mut self, n: &swc_ecma_ast::Ident) {
        self.check_bounds_and_visit(AstNodeKind::Identifier, n, true);
    }
    fn visit_import_decl(&mut self, n: &swc_ecma_ast::ImportDecl) {
        if self.config.visit_imports {
            self.check_bounds_and_visit(AstNodeKind::ImportDeclaration, n, true);
        }
    }
    fn visit_export_decl(&mut self, n: &swc_ecma_ast::ExportDecl) {
        if self.config.visit_exports {
            self.check_bounds_and_visit(AstNodeKind::ExportDeclaration, n, false);
        }
    }
    fn visit_class_decl(&mut self, n: &swc_ecma_ast::ClassDecl) {
        self.check_bounds_and_visit(AstNodeKind::ClassDeclaration, n, false);
    }
    fn visit_arrow_expr(&mut self, n: &swc_ecma_ast::ArrowExpr) {
        self.check_bounds_and_visit(AstNodeKind::ArrowFunction, n, false);
    }
    fn visit_new_expr(&mut self, n: &swc_ecma_ast::NewExpr) {
        self.check_bounds_and_visit(AstNodeKind::NewExpression, n, true);
    }
    fn visit_str(&mut self, n: &swc_ecma_ast::Str) {
        self.check_bounds_and_visit(AstNodeKind::StringLiteral, n, true);
    }
    fn visit_tpl(&mut self, n: &swc_ecma_ast::Tpl) {
        self.check_bounds_and_visit(AstNodeKind::TemplateLiteral, n, true);
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
        let fm = cm.new_source_file(
            Lrc::new(FileName::Custom(file_path.into())),
            source.to_string(),
        );

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
            Err(e) => {
                return Err(DomainError::IoError(format!(
                    "Parse error: {:?}",
                    e.into_kind().msg()
                )))
            }
        };

        let mut context = VisitorContext {
            current_file: Some(file_path.to_string()),
            ..VisitorContext::default()
        };

        visitor.lifecycle(TraversalLifecycleEvent::TraversalStarted, &mut context);

        let mut wrapper = SWCVisitorWrapper {
            visitor,
            context,
            config: config.clone(),
            cancelled: false,
            max_depth_reached: 0,
            diagnostics: Vec::new(),
            per_node_statistics: HashMap::new(),
            cm,
        };

        wrapper.visit_module(&module);

        let mut final_context = wrapper.context;

        if wrapper.cancelled {
            wrapper.visitor.lifecycle(
                TraversalLifecycleEvent::TraversalCancelled,
                &mut final_context,
            );
        } else {
            wrapper.visitor.lifecycle(
                TraversalLifecycleEvent::TraversalFinished,
                &mut final_context,
            );
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
    use crate::domain::ast_visitor::{
        ASTVisitor, AstNodeKind, TraversalLifecycleEvent, VisitorContext,
    };

    struct MockVisitor {
        pub enter_counts: HashMap<String, usize>,
        pub lifecycle_events: Vec<TraversalLifecycleEvent>,
        pub metadata_seen: Vec<HashMap<String, String>>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                enter_counts: HashMap::new(),
                lifecycle_events: Vec::new(),
                metadata_seen: Vec::new(),
            }
        }
    }

    impl ASTVisitor for MockVisitor {
        fn enter(&mut self, node: AstNodeKind, context: &mut VisitorContext) {
            *self.enter_counts.entry(format!("{:?}", node)).or_insert(0) += 1;
            if !context.metadata.is_empty() {
                self.metadata_seen.push(context.metadata.clone());
            }
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

        let result = walker
            .walk(source, "test.js", &config, &mut visitor)
            .unwrap();

        assert_eq!(
            *visitor
                .enter_counts
                .get("FunctionDeclaration")
                .unwrap_or(&0),
            2
        );
        assert!(!result.cancelled);
        assert!(result.visited_node_count >= 2);
    }

    #[test]
    fn test_walk_extracts_metadata() {
        let walker = SWCAstWalker::new();
        let source = "chrome.tabs.query(); eval('x'); const y = 'secret_key';";
        let config = WalkerConfig::default();
        let mut visitor = MockVisitor::new();

        walker
            .walk(source, "test.js", &config, &mut visitor)
            .unwrap();

        // Check eval extraction
        let eval_metadata = visitor
            .metadata_seen
            .iter()
            .find(|m| m.get("callee_name").map(|s| s.as_str()) == Some("eval"));
        assert!(eval_metadata.is_some());

        let string_metadata = visitor
            .metadata_seen
            .iter()
            .find(|m| m.get("simulated_string_literal").map(|s| s.as_str()) == Some("secret_key"));
        assert!(string_metadata.is_some());
    }
}
