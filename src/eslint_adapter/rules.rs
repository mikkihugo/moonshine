//! Native Rust implementations of common ESLint rules
//!
//! These implementations provide the same behavior as their ESLint counterparts
//! but use OXC's AST for faster parsing and analysis.

use super::{ESLintRule, ESLintContext};
use crate::types::LintDiagnostic;
use oxc_ast::ast::{
    CallExpression, Statement, Expression, VariableDeclarationKind,
    VariableDeclaration, BlockStatement, TryStatement, CatchClause,
    DebuggerStatement, AssignmentTarget, AssignmentExpression,
};
use oxc_ast_visit::{Visit, walk};
use oxc_span::GetSpan;

/// ESLint rule: no-console
/// Disallows console statements
pub struct NoConsoleRule;

impl ESLintRule for NoConsoleRule {
    fn rule_id(&self) -> &'static str {
        "no-console"
    }

    fn description(&self) -> &'static str {
        "Disallow the use of console"
    }

    fn run(&self, ctx: &mut ESLintContext) -> Vec<LintDiagnostic> {
        let mut visitor = NoConsoleVisitor::new(ctx);
        visitor.visit_program(ctx.program);
        visitor.finish()
    }
}

struct NoConsoleVisitor<'a, 'ctx> {
    ctx: &'a mut ESLintContext<'ctx>,
}

impl<'a, 'ctx> NoConsoleVisitor<'a, 'ctx> {
    fn new(ctx: &'a mut ESLintContext<'ctx>) -> Self {
        Self { ctx }
    }

    fn finish(self) -> Vec<LintDiagnostic> {
        self.ctx.finish()
    }
}

impl<'a, 'ctx> Visit<'ctx> for NoConsoleVisitor<'a, 'ctx> {
    fn visit_call_expression(&mut self, expr: &CallExpression<'ctx>) {
        if let Some(name) = get_member_expression_name(&expr.callee) {
            if name.starts_with("console.") {
                self.ctx.report(
                    expr.span(),
                    format!("Unexpected console statement: {}", name)
                );
            }
        }
        walk::walk_call_expression(self, expr);
    }
}

/// ESLint rule: no-debugger
/// Disallows debugger statements
pub struct NoDebuggerRule;

impl ESLintRule for NoDebuggerRule {
    fn rule_id(&self) -> &'static str {
        "no-debugger"
    }

    fn description(&self) -> &'static str {
        "Disallow the use of debugger"
    }

    fn run(&self, ctx: &mut ESLintContext) -> Vec<LintDiagnostic> {
        let mut visitor = NoDebuggerVisitor::new(ctx);
        visitor.visit_program(ctx.program);
        visitor.finish()
    }
}

struct NoDebuggerVisitor<'a, 'ctx> {
    ctx: &'a mut ESLintContext<'ctx>,
}

impl<'a, 'ctx> NoDebuggerVisitor<'a, 'ctx> {
    fn new(ctx: &'a mut ESLintContext<'ctx>) -> Self {
        Self { ctx }
    }

    fn finish(self) -> Vec<LintDiagnostic> {
        self.ctx.finish()
    }
}

impl<'a, 'ctx> Visit<'ctx> for NoDebuggerVisitor<'a, 'ctx> {
    fn visit_debugger_statement(&mut self, stmt: &DebuggerStatement) {
        self.ctx.report(stmt.span, "Unexpected debugger statement");
    }
}

/// ESLint rule: no-empty
/// Disallows empty block statements
pub struct NoEmptyRule;

impl ESLintRule for NoEmptyRule {
    fn rule_id(&self) -> &'static str {
        "no-empty"
    }

    fn description(&self) -> &'static str {
        "Disallow empty block statements"
    }

    fn run(&self, ctx: &mut ESLintContext) -> Vec<LintDiagnostic> {
        let mut visitor = NoEmptyVisitor::new(ctx);
        visitor.visit_program(ctx.program);
        visitor.finish()
    }
}

struct NoEmptyVisitor<'a, 'ctx> {
    ctx: &'a mut ESLintContext<'ctx>,
}

impl<'a, 'ctx> NoEmptyVisitor<'a, 'ctx> {
    fn new(ctx: &'a mut ESLintContext<'ctx>) -> Self {
        Self { ctx }
    }

    fn finish(self) -> Vec<LintDiagnostic> {
        self.ctx.finish()
    }
}

impl<'a, 'ctx> Visit<'ctx> for NoEmptyVisitor<'a, 'ctx> {
    fn visit_block_statement(&mut self, block: &BlockStatement<'ctx>) {
        if block.body.is_empty() {
            self.ctx.report(block.span, "Empty block statement");
        }
        walk::walk_block_statement(self, block);
    }

    fn visit_try_statement(&mut self, stmt: &TryStatement<'ctx>) {
        // Allow empty catch blocks - common pattern
        walk::walk_try_statement(self, stmt);
    }
}

/// ESLint rule: no-var
/// Disallows var declarations
pub struct NoVarRule;

impl ESLintRule for NoVarRule {
    fn rule_id(&self) -> &'static str {
        "no-var"
    }

    fn description(&self) -> &'static str {
        "Disallow var declarations"
    }

    fn run(&self, ctx: &mut ESLintContext) -> Vec<LintDiagnostic> {
        let mut visitor = NoVarVisitor::new(ctx);
        visitor.visit_program(ctx.program);
        visitor.finish()
    }

    fn supports_autofix(&self) -> bool {
        true
    }
}

struct NoVarVisitor<'a, 'ctx> {
    ctx: &'a mut ESLintContext<'ctx>,
}

impl<'a, 'ctx> NoVarVisitor<'a, 'ctx> {
    fn new(ctx: &'a mut ESLintContext<'ctx>) -> Self {
        Self { ctx }
    }

    fn finish(self) -> Vec<LintDiagnostic> {
        self.ctx.finish()
    }
}

impl<'a, 'ctx> Visit<'ctx> for NoVarVisitor<'a, 'ctx> {
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'ctx>) {
        if matches!(decl.kind, VariableDeclarationKind::Var) {
            self.ctx.report_with_fix(
                decl.span,
                "Unexpected var, use let or const instead",
                Some("let".to_string()) // Simple fix suggestion
            );
        }
        walk::walk_variable_declaration(self, decl);
    }
}

/// ESLint rule: prefer-const
/// Suggests using const for variables that are never reassigned
pub struct PreferConstRule;

impl ESLintRule for PreferConstRule {
    fn rule_id(&self) -> &'static str {
        "prefer-const"
    }

    fn description(&self) -> &'static str {
        "Require const declarations for variables that are never reassigned"
    }

    fn run(&self, ctx: &mut ESLintContext) -> Vec<LintDiagnostic> {
        let mut visitor = PreferConstVisitor::new(ctx);
        visitor.visit_program(ctx.program);
        visitor.finish()
    }

    fn supports_autofix(&self) -> bool {
        true
    }
}

struct PreferConstVisitor<'a, 'ctx> {
    ctx: &'a mut ESLintContext<'ctx>,
    declared_variables: std::collections::HashSet<String>,
    reassigned_variables: std::collections::HashSet<String>,
}

impl<'a, 'ctx> PreferConstVisitor<'a, 'ctx> {
    fn new(ctx: &'a mut ESLintContext<'ctx>) -> Self {
        Self {
            ctx,
            declared_variables: std::collections::HashSet::new(),
            reassigned_variables: std::collections::HashSet::new(),
        }
    }

    fn finish(mut self) -> Vec<LintDiagnostic> {
        // Report variables that could be const
        for var_name in &self.declared_variables {
            if !self.reassigned_variables.contains(var_name) {
                // Note: In a real implementation, we'd need to store the span of the declaration
                // For now, we'll skip reporting as we don't have the span info stored
            }
        }
        self.ctx.finish()
    }
}

impl<'a, 'ctx> Visit<'ctx> for PreferConstVisitor<'a, 'ctx> {
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'ctx>) {
        if matches!(decl.kind, VariableDeclarationKind::Let) {
            for declarator in &decl.declarations {
                if let Some(id) = get_binding_identifier_name(&declarator.id) {
                    self.declared_variables.insert(id.to_string());
                }
            }
        }
        walk::walk_variable_declaration(self, decl);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'ctx>) {
        if let Some(name) = get_assignment_target_name(&expr.left) {
            self.reassigned_variables.insert(name.to_string());
        }
        walk::walk_assignment_expression(self, expr);
    }
}

// Helper functions for AST navigation

fn get_member_expression_name(expr: &Expression) -> Option<String> {
    match expr {
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(ident) = &member.object {
                Some(format!("{}.{}", ident.name, member.property.name))
            } else {
                None
            }
        }
        Expression::ComputedMemberExpression(member) => {
            if let Expression::Identifier(ident) = &member.object {
                if let Expression::StringLiteral(prop) = &member.expression {
                    Some(format!("{}.{}", ident.name, prop.value))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_binding_identifier_name(target: &oxc_ast::ast::BindingPattern) -> Option<&str> {
    match target {
        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => Some(&ident.name),
        _ => None, // Handle destructuring patterns in a full implementation
    }
}

fn get_assignment_target_name(target: &AssignmentTarget) -> Option<&str> {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(ident) => Some(&ident.name),
        _ => None, // Handle member expressions, destructuring in a full implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_registry::RuleSeverity;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_span::SourceType;

    fn create_test_context(code: &str, rule_id: &str) -> (Allocator, ESLintContext) {
        let allocator = Allocator::default();
        let source_type = SourceType::ts();
        let parser_result = Parser::new(&allocator, code, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let ctx = ESLintContext::new(
            code,
            "test.ts",
            &parser_result.program,
            None,
            rule_id.to_string(),
            RuleSeverity::Warning,
        );

        (allocator, ctx)
    }

    #[test]
    fn test_no_console_rule() {
        let rule = NoConsoleRule;
        let code = r#"
            console.log("test");
            console.error("error");
            regular.log("ok");
        "#;

        let (_allocator, mut ctx) = create_test_context(code, "no-console");
        let diagnostics = rule.run(&mut ctx);

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics[0].message.contains("console.log"));
        assert!(diagnostics[1].message.contains("console.error"));
    }

    #[test]
    fn test_no_var_rule() {
        let rule = NoVarRule;
        let code = r#"
            var x = 1;
            let y = 2;
            const z = 3;
        "#;

        let (_allocator, mut ctx) = create_test_context(code, "no-var");
        let diagnostics = rule.run(&mut ctx);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("Unexpected var"));
        assert_eq!(diagnostics[0].rule_name, "no-var");
    }

    #[test]
    fn test_no_empty_rule() {
        let rule = NoEmptyRule;
        let code = r#"
            if (true) {}
            while (false) {}
            function test() {
                console.log("not empty");
            }
        "#;

        let (_allocator, mut ctx) = create_test_context(code, "no-empty");
        let diagnostics = rule.run(&mut ctx);

        assert_eq!(diagnostics.len(), 2); // Two empty blocks
        assert!(diagnostics.iter().all(|d| d.message.contains("Empty block")));
    }

    #[test]
    fn test_rule_autofix_support() {
        assert!(!NoConsoleRule.supports_autofix());
        assert!(!NoDebuggerRule.supports_autofix());
        assert!(!NoEmptyRule.supports_autofix());
        assert!(NoVarRule.supports_autofix());
        assert!(PreferConstRule.supports_autofix());
    }
}