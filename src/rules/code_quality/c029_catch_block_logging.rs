//! Custom MoonShine rule for: C029 – Every catch block must log the error cause
//! Rule ID: moonshine/c029
//! Purpose: Prevent silent error handling which hides bugs and makes debugging difficult
//!
//! Converted from JavaScript ESLint rule
//! @category code-quality-rules
//! @complexity medium

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Statement, Expression, CallExpression, MemberExpression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};

/// Configuration options for C029 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C029Config {
    /// Whether to allow empty catch blocks in test files (default: true)
    pub allow_empty_in_tests: bool,
    /// Additional logging function names to recognize (default: ["log", "error", "warn"])
    pub allowed_logging_functions: Vec<String>,
}

impl Default for C029Config {
    fn default() -> Self {
        Self {
            allow_empty_in_tests: true,
            allowed_logging_functions: vec![
                "log".to_string(),
                "error".to_string(),
                "warn".to_string(),
                "console".to_string(),
                "dispatch".to_string(),
            ],
        }
    }
}

/// Main entry point for C029 rule checking
pub fn check_catch_block_logging(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C029Config::default();
    let mut visitor = CatchBlockVisitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting catch block logging violations
struct CatchBlockVisitor<'a> {
    config: &'a C029Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
}

impl<'a> CatchBlockVisitor<'a> {
    fn new(config: &'a C029Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, has_empty_block: bool) -> String {
        if has_empty_block {
            "Empty catch block detected - this silently swallows errors and makes debugging impossible. Add error logging or rethrow the error.".to_string()
        } else {
            "Catch block does not log the error - this can hide bugs during debugging. Consider logging the error or rethrowing with context.".to_string()
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, has_empty_block: bool) -> Vec<String> {
        if has_empty_block {
            vec![
                "console.error('Error:', error);".to_string(),
                "throw new Error(`Operation failed: ${error.message}`);".to_string(),
                "logger.error('Unexpected error occurred', { error: error.message, stack: error.stack });".to_string(),
            ]
        } else {
            vec![
                "console.error('Error caught:', error);".to_string(),
                "logger.error('Operation failed', error);".to_string(),
                "Sentry.captureException(error);".to_string(),
            ]
        }
    }

    /// Calculate line and column from byte offset
    fn calculate_line_column(&self, offset: usize) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source_code.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    fn check_statement_for_logging(&self, stmt: &Statement) -> bool {
        match stmt {
            // Check for throw statements
            Statement::ThrowStatement(_) => true,

            Statement::ExpressionStatement(expr_stmt) => {
                match &expr_stmt.expression {
                    Expression::CallExpression(call) => {
                        self.is_logging_call(call) || self.is_test_assertion(call) || self.is_dispatch_call(call)
                    }
                    _ => false,
                }
            },

            Statement::VariableDeclaration(var_decl) => {
                // Check for Redux thunk error handling patterns
                var_decl.declarations.iter().any(|decl| {
                    if let Some(Expression::CallExpression(call)) = &decl.init {
                        self.is_handle_axios_error(call)
                    } else {
                        false
                    }
                })
            },

            Statement::ReturnStatement(return_stmt) => {
                // Check for return rejectWithValue
                if let Some(Expression::CallExpression(call)) = &return_stmt.argument {
                    self.is_reject_with_value(call)
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
                self.is_console_log(member) || self.is_custom_logger(member)
            }
            _ => false,
        }
    }

    fn is_console_log(&self, member: &MemberExpression) -> bool {
        // Check for console.log, console.error, console.warn
        if let (Expression::Identifier(obj), Some(prop)) = (&member.object, &member.property) {
            if let Expression::Identifier(prop_ident) = prop {
                return obj.name == "console" &&
                       (prop_ident.name == "log" || prop_ident.name == "error" || prop_ident.name == "warn");
            }
        }
        false
    }

    fn is_custom_logger(&self, member: &MemberExpression) -> bool {
        // Check for custom logger calls (logger.error, log.error, etc.)
        if let Some(prop) = &member.property {
            if let Expression::Identifier(prop_ident) = prop {
                return prop_ident.name == "error" ||
                       prop_ident.name == "warn" ||
                       prop_ident.name == "log";
            }
        }
        false
    }

    fn is_test_assertion(&self, call: &CallExpression) -> bool {
        // Check for test assertions (Jest patterns)
        if let Expression::Identifier(callee) = &call.callee {
            callee.name == "expect"
        } else {
            false
        }
    }

    fn is_handle_axios_error(&self, call: &CallExpression) -> bool {
        if let Expression::Identifier(callee) = &call.callee {
            callee.name == "handleAxiosError"
        } else {
            false
        }
    }

    fn is_reject_with_value(&self, call: &CallExpression) -> bool {
        if let Expression::Identifier(callee) = &call.callee {
            callee.name == "rejectWithValue"
        } else {
            false
        }
    }

    /// Create lint issue for catch block without proper error handling with AI enhancement
    fn create_catch_logging_issue(&self, span: oxc_span::Span, is_empty: bool) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware suggestions
        let ai_enhanced_message = self.generate_ai_enhanced_message(is_empty);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(is_empty);

        LintIssue {
            rule_name: "moonshine/c029".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }
}

impl<'a> Visit<'a> for CatchBlockVisitor<'a> {
    fn visit_catch_clause(&mut self, catch_clause: &oxc_ast::ast::CatchClause<'a>) {
        let body = &catch_clause.body.body;

        // Check for empty catch blocks
        if body.is_empty() {
            let issue = self.create_catch_logging_issue(catch_clause.span, true);
            self.issues.push(issue);
            return;
        }

        // Check if any statement in the catch block logs or throws
        let has_log_or_throw = body.iter().any(|stmt| self.check_statement_for_logging(stmt));

        if !has_log_or_throw {
            let issue = self.create_catch_logging_issue(catch_clause.span, false);
            self.issues.push(issue);
        }

        // Continue visiting
        self.visit_block_statement(&catch_clause.body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    /// Helper function to parse code and run the rule
    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path("test.ts").unwrap();
        let parser = Parser::new(&allocator, code, source_type);
        let parse_result = parser.parse();

        if !parse_result.errors.is_empty() {
            panic!("Parse errors: {:?}", parse_result.errors);
        }

        let semantic = SemanticBuilder::new(code, source_type)
            .build(&parse_result.program)
            .semantic;

        check_catch_block_logging(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_catch_block_logging_empty_violation() {
        let code = r#"
            try {
                doSomething();
            } catch (e) {
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c029");
        assert!(issues[0].message.contains("Empty catch block"));
        assert_eq!(issues[0].fix_available, true);
    }

    #[test]
    fn test_catch_block_logging_console_compliant() {
        let code = r#"
            try {
                doSomething();
            } catch (e) {
                console.error(e);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_block_logging_custom_logger_compliant() {
        let code = r#"
            try {
                riskyOperation();
            } catch (error) {
                logger.error("Operation failed:", error);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_block_logging_rethrow_compliant() {
        let code = r#"
            try {
                parseData();
            } catch (e) {
                throw new Error(`Parse failed: ${e.message}`);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_block_logging_reject_with_value_compliant() {
        let code = r#"
            try {
                await apiCall();
            } catch (error) {
                return rejectWithValue(error.message);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_block_logging_no_logging_violation() {
        let code = r#"
            try {
                processData();
            } catch (error) {
                // Just return without logging
                return null;
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c029");
        assert!(issues[0].message.contains("does not log the error"));
        assert_eq!(issues[0].fix_available, true);
    }

    #[test]
    fn test_catch_block_logging_multiple_blocks_mixed() {
        let code = r#"
            try {
                operation1();
            } catch (e) {
                console.error("Op1 failed:", e);
            }

            try {
                operation2();
            } catch (e) {
                // Silent failure
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "moonshine/c029");
        assert!(issues[0].message.contains("Empty catch block"));
    }

    #[test]
    fn test_catch_block_logging_nested_try_catch() {
        let code = r#"
            function processData() {
                try {
                    try {
                        innerOperation();
                    } catch (innerError) {
                        console.warn("Inner operation failed:", innerError);
                    }
                } catch (outerError) {
                    // Outer catch without logging
                }
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "moonshine/c029");
        assert!(issues[0].message.contains("Empty catch block"));
    }

    #[test]
    fn test_catch_block_logging_test_assertion_compliant() {
        let code = r#"
            try {
                riskyTestOperation();
            } catch (error) {
                expect(error).toBeInstanceOf(CustomError);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_block_logging_handle_axios_error_compliant() {
        let code = r#"
            try {
                await apiRequest();
            } catch (error) {
                const handledError = handleAxiosError(error);
            }
        "#;
        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }
}