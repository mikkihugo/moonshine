//! # ESLint Visitor Pattern Mapping
//!
//! This module provides visitor patterns that map ESLint's rule visitor API
//! to OXC's visitor system. ESLint rules define visitors for specific AST
//! node types, and this module bridges that to our execution engine.

use super::context::{ESLintRuleContext, ESLintReport};
use oxc_ast::ast::*;
use oxc_ast_visit::{walk, Visit};
use oxc_span::GetSpan;
use std::collections::HashMap;

/// ESLint-compatible visitor trait
///
/// ESLint rules implement visitors by providing functions for specific
/// node types. This trait captures that pattern in Rust.
pub trait ESLintVisitor<'a> {
    /// Called when entering a node
    fn enter(&mut self, _node: &dyn ASTNode<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Called when exiting a node
    fn exit(&mut self, _node: &dyn ASTNode<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit call expressions (function calls)
    fn visit_call_expression(&mut self, _node: &CallExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit member expressions (property access)
    fn visit_member_expression(&mut self, _node: &MemberExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit identifier nodes
    fn visit_identifier(&mut self, _node: &Identifier<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit variable declarations
    fn visit_variable_declaration(&mut self, _node: &VariableDeclaration<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit function declarations
    fn visit_function_declaration(&mut self, _node: &Function<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit binary expressions
    fn visit_binary_expression(&mut self, _node: &BinaryExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit assignment expressions
    fn visit_assignment_expression(&mut self, _node: &AssignmentExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit if statements
    fn visit_if_statement(&mut self, _node: &IfStatement<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit return statements
    fn visit_return_statement(&mut self, _node: &ReturnStatement<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit object expressions
    fn visit_object_expression(&mut self, _node: &ObjectExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}

    /// Visit array expressions
    fn visit_array_expression(&mut self, _node: &ArrayExpression<'a>, _context: &mut ESLintRuleContext<'a>) {}
}

/// Generic trait for AST nodes to enable visitor pattern
pub trait ASTNode<'a>: GetSpan {
    fn node_type(&self) -> &'static str;
}

/// Implementation of ASTNode for common OXC AST types
impl<'a> ASTNode<'a> for CallExpression<'a> {
    fn node_type(&self) -> &'static str {
        "CallExpression"
    }
}

impl<'a> ASTNode<'a> for MemberExpression<'a> {
    fn node_type(&self) -> &'static str {
        "MemberExpression"
    }
}

impl<'a> ASTNode<'a> for Identifier<'a> {
    fn node_type(&self) -> &'static str {
        "Identifier"
    }
}

impl<'a> ASTNode<'a> for VariableDeclaration<'a> {
    fn node_type(&self) -> &'static str {
        "VariableDeclaration"
    }
}

impl<'a> ASTNode<'a> for Function<'a> {
    fn node_type(&self) -> &'static str {
        "FunctionDeclaration"
    }
}

/// OXC visitor adapter that bridges to ESLint visitor pattern
///
/// This struct implements OXC's Visit trait and delegates to an ESLint-style visitor
pub struct ESLintVisitorAdapter<'a, V: ESLintVisitor<'a>> {
    visitor: V,
    context: ESLintRuleContext<'a>,
}

impl<'a, V: ESLintVisitor<'a>> ESLintVisitorAdapter<'a, V> {
    pub fn new(visitor: V, context: ESLintRuleContext<'a>) -> Self {
        Self { visitor, context }
    }

    pub fn get_diagnostics(self) -> Vec<crate::types::LintDiagnostic> {
        self.context.get_diagnostics()
    }
}

impl<'a, V: ESLintVisitor<'a>> Visit<'a> for ESLintVisitorAdapter<'a, V> {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        self.visitor.visit_call_expression(node, &mut self.context);
        walk::walk_call_expression(self, node);
    }

    fn visit_member_expression(&mut self, node: &MemberExpression<'a>) {
        self.visitor.visit_member_expression(node, &mut self.context);
        walk::walk_member_expression(self, node);
    }

    fn visit_identifier_reference(&mut self, node: &IdentifierReference<'a>) {
        // Create an Identifier-like interface for ESLint compatibility
        let identifier = IdentifierAdapter { name: &node.name };
        self.visitor.visit_identifier(&identifier, &mut self.context);
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration<'a>) {
        self.visitor.visit_variable_declaration(node, &mut self.context);
        walk::walk_variable_declaration(self, node);
    }

    fn visit_function(&mut self, node: &Function<'a>) {
        self.visitor.visit_function_declaration(node, &mut self.context);
        walk::walk_function(self, node);
    }

    fn visit_binary_expression(&mut self, node: &BinaryExpression<'a>) {
        self.visitor.visit_binary_expression(node, &mut self.context);
        walk::walk_binary_expression(self, node);
    }

    fn visit_assignment_expression(&mut self, node: &AssignmentExpression<'a>) {
        self.visitor.visit_assignment_expression(node, &mut self.context);
        walk::walk_assignment_expression(self, node);
    }

    fn visit_if_statement(&mut self, node: &IfStatement<'a>) {
        self.visitor.visit_if_statement(node, &mut self.context);
        walk::walk_if_statement(self, node);
    }

    fn visit_return_statement(&mut self, node: &ReturnStatement<'a>) {
        self.visitor.visit_return_statement(node, &mut self.context);
        walk::walk_return_statement(self, node);
    }

    fn visit_object_expression(&mut self, node: &ObjectExpression<'a>) {
        self.visitor.visit_object_expression(node, &mut self.context);
        walk::walk_object_expression(self, node);
    }

    fn visit_array_expression(&mut self, node: &ArrayExpression<'a>) {
        self.visitor.visit_array_expression(node, &mut self.context);
        walk::walk_array_expression(self, node);
    }
}

/// Adapter to make IdentifierReference look like Identifier for ESLint compatibility
pub struct IdentifierAdapter<'a> {
    pub name: &'a str,
}

impl<'a> IdentifierAdapter<'a> {
    pub fn name(&self) -> &str {
        self.name
    }
}

impl<'a> GetSpan for IdentifierAdapter<'a> {
    fn span(&self) -> oxc_span::Span {
        // Return empty span for adapter - real implementations should track this
        oxc_span::Span::empty(0)
    }
}

impl<'a> ASTNode<'a> for IdentifierAdapter<'a> {
    fn node_type(&self) -> &'static str {
        "Identifier"
    }
}

/// Common ESLint rule implementations using the visitor pattern

/// Implementation of ESLint's no-console rule
pub struct NoConsoleRule;

impl<'a> ESLintVisitor<'a> for NoConsoleRule {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>, context: &mut ESLintRuleContext<'a>) {
        if super::context::is_console_call(node) {
            let report = ESLintReport::new(
                node.span(),
                "Unexpected console statement"
            );
            context.report(report);
        }
    }
}

/// Implementation of ESLint's no-debugger rule
pub struct NoDebuggerRule;

impl<'a> ESLintVisitor<'a> for NoDebuggerRule {
    fn visit_identifier(&mut self, node: &Identifier<'a>, context: &mut ESLintRuleContext<'a>) {
        if node.name.as_str() == "debugger" {
            let report = ESLintReport::new(
                node.span(),
                "Unexpected 'debugger' statement"
            );
            context.report(report);
        }
    }
}

/// Implementation of ESLint's no-eval rule
pub struct NoEvalRule;

impl<'a> ESLintVisitor<'a> for NoEvalRule {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>, context: &mut ESLintRuleContext<'a>) {
        if super::context::is_eval_call(node) {
            let report = ESLintReport::new(
                node.span(),
                format!("Unexpected use of '{}' is not allowed",
                    match &node.callee {
                        Expression::Identifier(ident) => ident.name.as_str(),
                        _ => "eval-like function"
                    }
                )
            );
            context.report(report);
        }
    }
}

/// Implementation of ESLint's no-alert rule
pub struct NoAlertRule;

impl<'a> ESLintVisitor<'a> for NoAlertRule {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>, context: &mut ESLintRuleContext<'a>) {
        if let Expression::Identifier(ident) = &node.callee {
            if matches!(ident.name.as_str(), "alert" | "confirm" | "prompt") {
                let report = ESLintReport::new(
                    node.span(),
                    format!("Unexpected {} call", ident.name.as_str())
                );
                context.report(report);
            }
        }
    }
}

/// Implementation of ESLint's no-empty rule
pub struct NoEmptyRule;

impl<'a> ESLintVisitor<'a> for NoEmptyRule {
    fn visit_if_statement(&mut self, node: &IfStatement<'a>, context: &mut ESLintRuleContext<'a>) {
        if let Statement::BlockStatement(block) = &node.consequent {
            if block.body.is_empty() {
                let report = ESLintReport::new(
                    block.span(),
                    "Empty block statement"
                );
                context.report(report);
            }
        }

        if let Some(Statement::BlockStatement(block)) = &node.alternate {
            if block.body.is_empty() {
                let report = ESLintReport::new(
                    block.span(),
                    "Empty block statement"
                );
                context.report(report);
            }
        }
    }
}

/// Factory function to create ESLint rule visitors
pub fn create_eslint_visitor(rule_id: &str) -> Option<Box<dyn for<'a> ESLintVisitor<'a>>> {
    match rule_id {
        "no-console" => Some(Box::new(NoConsoleRule) as Box<dyn for<'a> ESLintVisitor<'a>>),
        "no-debugger" => Some(Box::new(NoDebuggerRule) as Box<dyn for<'a> ESLintVisitor<'a>>),
        "no-eval" => Some(Box::new(NoEvalRule) as Box<dyn for<'a> ESLintVisitor<'a>>),
        "no-alert" => Some(Box::new(NoAlertRule) as Box<dyn for<'a> ESLintVisitor<'a>>),
        "no-empty" => Some(Box::new(NoEmptyRule) as Box<dyn for<'a> ESLintVisitor<'a>>),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DiagnosticSeverity;
    use oxc_allocator::Allocator;
    use oxc_parser::{ParseOptions, Parser};
    use oxc_span::SourceType;

    #[test]
    fn test_no_console_rule() {
        let source = "console.log('test'); let x = 5;";
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let context = ESLintRuleContext::new(
            source,
            "test.js",
            &parser_result.program,
            None,
            "no-console".to_string(),
            DiagnosticSeverity::Warning,
            vec![],
        );

        let mut adapter = ESLintVisitorAdapter::new(NoConsoleRule, context);
        adapter.visit_program(&parser_result.program);

        let diagnostics = adapter.get_diagnostics();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, "no-console");
        assert!(diagnostics[0].message.contains("console"));
    }

    #[test]
    fn test_no_eval_rule() {
        let source = "eval('dangerous code'); let y = 10;";
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let context = ESLintRuleContext::new(
            source,
            "test.js",
            &parser_result.program,
            None,
            "no-eval".to_string(),
            DiagnosticSeverity::Error,
            vec![],
        );

        let mut adapter = ESLintVisitorAdapter::new(NoEvalRule, context);
        adapter.visit_program(&parser_result.program);

        let diagnostics = adapter.get_diagnostics();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, "no-eval");
        assert!(diagnostics[0].message.contains("eval"));
    }

    #[test]
    fn test_rule_factory() {
        assert!(create_eslint_visitor("no-console").is_some());
        assert!(create_eslint_visitor("no-debugger").is_some());
        assert!(create_eslint_visitor("no-eval").is_some());
        assert!(create_eslint_visitor("unknown-rule").is_none());
    }
}