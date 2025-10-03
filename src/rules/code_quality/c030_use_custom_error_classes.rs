//! Custom MoonShine rule for: C030 – Use Custom Error Classes
//! Rule ID: moonshine/c030
//! Purpose: Enforce use of custom error classes instead of generic Error objects
//!
//! Converted from JavaScript ESLint rule
//! @category code-quality-rules
//! @complexity medium

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Expression, NewExpression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration options for the C030 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C030Config {
    /// A list of allowed built-in error classes.
    #[serde(default)]
    pub allowed_builtin_errors: Vec<String>,
    /// A list of custom error class patterns (regex).
    #[serde(default)]
    pub custom_error_patterns: Vec<String>,
}

impl Default for C030Config {
    fn default() -> Self {
        Self {
            allowed_builtin_errors: vec![
                "TypeError".to_string(),
                "ReferenceError".to_string(),
                "RangeError".to_string(),
                "SyntaxError".to_string(),
                "URIError".to_string(),
                "EvalError".to_string(),
            ],
            custom_error_patterns: vec![
                r".*Error$".to_string(), // Ends with Error
            ],
        }
    }
}

/// The main entry point for the C030 rule checking.
pub fn check_use_custom_error_classes(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C030Config::default();
    let mut visitor = C030Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// An AST visitor for detecting generic error class usage.
struct C030Visitor<'a> {
    config: &'a C030Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    allowed_builtin_errors: HashSet<String>,
}

impl<'a> C030Visitor<'a> {
    /// Creates a new `C030Visitor`.
    fn new(config: &'a C030Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let allowed_builtin_errors: HashSet<String> = config.allowed_builtin_errors
            .iter()
            .cloned()
            .collect();

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            allowed_builtin_errors,
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, error_class: &str) -> String {
        format!("Using generic error class '{}'. Create custom error classes that extend Error to provide better error categorization and handling. Custom errors improve debugging and allow for more specific error handling.", error_class)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self, error_class: &str) -> Vec<String> {
        vec![
            format!("Create a custom {} class that extends Error", error_class),
            "Use specific error types like ValidationError, NetworkError, etc.".to_string(),
            "Define error classes in a separate errors.js file".to_string(),
            "Include error codes and additional context in custom errors".to_string(),
        ]
    }

    /// Calculates the line and column from a byte offset.
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

    /// Creates a lint issue for a custom error class violation with AI enhancement.
    fn create_error_class_issue(&self, error_class: &str, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(error_class);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(error_class);

        LintIssue {
            rule_name: "moonshine/c030".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Checks if an expression is a generic `Error`.
    fn is_generic_error(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(ident) => {
                ident.name == "Error" && !self.allowed_builtin_errors.contains("Error")
            },
            _ => false,
        }
    }

    /// Returns the name of the error class from an expression.
    fn get_error_class_name(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        }
    }
}

impl<'a> Visit<'a> for C030Visitor<'a> {
    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        if self.is_generic_error(&expr.callee) {
            if let Some(error_class) = self.get_error_class_name(&expr.callee) {
                self.issues.push(self.create_error_class_issue(&error_class, expr.span));
            }
        }

        // Continue visiting
        self.visit_expression(&expr.callee);
        for arg in &expr.arguments {
            self.visit_argument(arg);
        }
    }

    fn visit_throw_statement(&mut self, stmt: &oxc_ast::ast::ThrowStatement<'a>) {
        if let Some(Expression::NewExpression(new_expr)) = &stmt.argument {
            if self.is_generic_error(&new_expr.callee) {
                if let Some(error_class) = self.get_error_class_name(&new_expr.callee) {
                    self.issues.push(self.create_error_class_issue(&error_class, stmt.span));
                }
            }
        }

        // Continue visiting
        if let Some(argument) = &stmt.argument {
            self.visit_expression(argument);
        }
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

        check_use_custom_error_classes(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_use_custom_error_classes_violation() {
        let code = r#"throw new Error("Something went wrong");"#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c030");
        assert!(issues[0].message.contains("custom error classes"));
        assert_eq!(issues[0].severity, LintSeverity::Warning);
    }

    #[test]
    fn test_use_custom_error_classes_compliant() {
        let code = r#"throw new ValidationError("Invalid input");"#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_multiple_error_violations() {
        let code = r#"
            function test() {
                throw new Error("First error");
                if (condition) {
                    throw new Error("Second error");
                }
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].rule_name, "moonshine/c030");
        assert_eq!(issues[1].rule_name, "moonshine/c030");
        assert!(issues[0].message.contains("custom error classes"));
        assert!(issues[1].message.contains("custom error classes"));
    }

    #[test]
    fn test_nested_error_handling() {
        let code = r#"
            try {
                try {
                    throw new Error("Nested error");
                } catch (e) {
                    throw new Error("Outer error");
                }
            } catch (e) {
                console.log(e);
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 2);
        assert!(issues[0].message.contains("Error"));
        assert!(issues[1].message.contains("Error"));
    }

    #[test]
    fn test_custom_error_classes_no_violation() {
        let code = r#"
            class CustomError extends Error {
                constructor(message) {
                    super(message);
                    this.name = 'CustomError';
                }
            }

            throw new CustomError("This is fine");
            throw new ValidationError("Also fine");
            throw new NetworkError("Network issue");
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_variable_assignment_error() {
        let code = r#"
            const err = new Error("Assignment error");
            const customErr = new CustomError("Custom assignment");
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "moonshine/c030");
        assert!(issues[0].message.contains("Error"));
    }
}