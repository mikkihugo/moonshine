//! # C029: Catch Block Logging Requirements
//!
//! This rule ensures that catch blocks include appropriate logging for error handling
//! and debugging purposes. It helps maintain proper error tracking and debugging capabilities.
//!
//! @category code-quality
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 1.0.0

use oxc_ast::ast::{Program, Statement, Expression, CallExpression, MemberExpression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;

/// Stub implementation for C029 catch logging rule
pub struct CatchLoggingRule;

impl CatchLoggingRule {
    /// Create a new catch logging rule instance
    pub fn new() -> Self {
        Self
    }

    /// Check catch block logging using OXC AST visitor pattern
    /// Production implementation using proper AST traversal instead of regex patterns
    pub fn check_catch_logging(&self, program: &Program, semantic: &Semantic, code: &str) -> Vec<String> {
        let mut visitor = CatchLoggingVisitor::new(program, code);
        visitor.visit_program(program);
        visitor.violations
    }

}

/// OXC AST visitor for catch block logging analysis
struct CatchLoggingVisitor<'a> {
    program: &'a Program<'a>,
    code: &'a str,
    violations: Vec<String>,
}

impl<'a> CatchLoggingVisitor<'a> {
    fn new(program: &'a Program<'a>, code: &'a str) -> Self {
        Self {
            program,
            code,
            violations: Vec::new(),
        }
    }

    fn span_to_line(&self, span: Span) -> usize {
        let source_text = self.code;
        let offset = span.start as usize;

        let mut line = 1;
        for (i, ch) in source_text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
            }
        }
        line
    }

    fn check_statement_for_logging(&self, stmt: &Statement) -> bool {
        match stmt {
            // Check for throw statements
            Statement::ThrowStatement(_) => true,

            Statement::ExpressionStatement(expr_stmt) => {
                match &expr_stmt.expression {
                    Expression::CallExpression(call) => {
                        self.is_logging_call(call)
                    }
                    _ => false,
                }
            },

            Statement::ReturnStatement(return_stmt) => {
                // Check for return rejectWithValue or similar error handling
                if let Some(Expression::CallExpression(call)) = &return_stmt.argument {
                    self.is_error_handling_call(call)
                } else {
                    false
                }
            },

            _ => false,
        }
    }

    fn is_logging_call(&self, call: &CallExpression) -> bool {
        match &call.callee {
            Expression::MemberExpression(member) => {
                self.is_console_error(member) || self.is_logger_call(member)
            }
            _ => false,
        }
    }

    fn is_console_error(&self, member: &MemberExpression) -> bool {
        if let (Expression::Identifier(obj), Some(prop)) = (&member.object, &member.property) {
            if let Expression::Identifier(prop_ident) = prop {
                return obj.name == "console" &&
                       (prop_ident.name == "error" || prop_ident.name == "warn");
            }
        }
        false
    }

    fn is_logger_call(&self, member: &MemberExpression) -> bool {
        if let Some(prop) = &member.property {
            if let Expression::Identifier(prop_ident) = prop {
                return prop_ident.name == "error" || prop_ident.name == "warn";
            }
        }
        false
    }

    fn is_error_handling_call(&self, call: &CallExpression) -> bool {
        if let Expression::Identifier(callee) = &call.callee {
            matches!(callee.name.as_str(), "rejectWithValue" | "handleError" | "logError")
        } else {
            false
        }
    }
}

impl<'a> Visit<'a> for CatchLoggingVisitor<'a> {
    fn visit_catch_clause(&mut self, catch_clause: &oxc_ast::ast::CatchClause<'a>) {
        let body = &catch_clause.body.body;
        let line = self.span_to_line(catch_clause.span);

        // Check for empty catch blocks
        if body.is_empty() {
            self.violations.push(format!(
                "Line {}: Empty catch block - error is silently ignored. Add logging or re-throw.",
                line
            ));
            return;
        }

        // Check if any statement in the catch block logs or throws
        let has_log_or_throw = body.iter().any(|stmt| self.check_statement_for_logging(stmt));

        if !has_log_or_throw {
            self.violations.push(format!(
                "Line {}: Catch block lacks error logging. Consider adding console.error, logger.error, or re-throwing.",
                line
            ));
        }

        // Continue visiting
        self.visit_block_statement(&catch_clause.body);
    }
}
}