//! # C018: No Generic Throw Rule
//!
//! Enforces specific error messages when throwing errors to improve debugging and error handling.
//! Prevents generic error messages that don't provide helpful context for developers.
//! Supports extensive configuration for different error handling patterns and contexts.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp enhanced
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use oxc_ast::ast::{Program, ThrowStatement, Expression, NewExpression, Argument};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use regex::Regex;

/// Configuration options for C018 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C018Config {
    /// Whether to allow generic throws in test files (default: true)
    #[serde(default = "default_allow_generic_in_tests")]
    pub allow_generic_in_tests: bool,
    /// Whether to allow rethrowing caught errors (default: true)
    #[serde(default = "default_allow_rethrow")]
    pub allow_rethrow: bool,
    /// Whether to allow throwing variables/identifiers (default: false)
    #[serde(default)]
    pub allow_throw_variable: bool,
    /// Regex patterns that error messages must match
    #[serde(default)]
    pub required_message_patterns: Vec<String>,
    /// Minimum length for error messages (default: 10)
    #[serde(default = "default_minimum_message_length")]
    pub minimum_message_length: u32,
    /// List of allowed generic messages
    #[serde(default)]
    pub allowed_generic_messages: Vec<String>,
    /// Custom error class names that are allowed
    #[serde(default)]
    pub custom_error_classes: Vec<String>,
    /// Enable strict mode with additional checks (default: false)
    #[serde(default)]
    pub strict_mode: bool,
}

fn default_allow_generic_in_tests() -> bool {
    true
}

fn default_allow_rethrow() -> bool {
    true
}

fn default_minimum_message_length() -> u32 {
    10
}

impl Default for C018Config {
    fn default() -> Self {
        Self {
            allow_generic_in_tests: true,
            allow_rethrow: true,
            allow_throw_variable: false,
            required_message_patterns: vec![],
            minimum_message_length: 10,
            allowed_generic_messages: vec![
                "An error occurred".to_string(),
                "Something went wrong".to_string(),
                "Unexpected error".to_string(),
                "Error".to_string(),
                "Failed".to_string(),
            ],
            custom_error_classes: vec![
                "Error".to_string(),
                "TypeError".to_string(),
                "ReferenceError".to_string(),
                "SyntaxError".to_string(),
                "RangeError".to_string(),
                "URIError".to_string(),
                "EvalError".to_string(),
            ],
            strict_mode: false,
        }
    }
}

/// Main entry point for C018 rule checking
pub fn check_no_generic_throw(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C018Config::default();
    let mut visitor = C018Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting generic throw violations
struct C018Visitor<'a> {
    config: &'a C018Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    allowed_generic_messages: HashSet<String>,
    custom_error_classes: HashSet<String>,
    required_patterns: Vec<Regex>,
}

impl<'a> C018Visitor<'a> {
    fn new(config: &'a C018Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let allowed_generic_messages: HashSet<String> = config.allowed_generic_messages
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        let custom_error_classes: HashSet<String> = config.custom_error_classes
            .iter()
            .cloned()
            .collect();

        let required_patterns: Vec<Regex> = config.required_message_patterns
            .iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            allowed_generic_messages,
            custom_error_classes,
            required_patterns,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, issue_type: &str, details: &str) -> String {
        match issue_type {
            "generic_message" => format!("Generic error message '{}' detected. Use specific, descriptive error messages that help developers understand what went wrong and how to fix it.", details),
            "too_short" => format!("Error message '{}' is too short ({} characters). Error messages should be at least {} characters long and provide meaningful context.", details, details.len(), self.config.minimum_message_length),
            "missing_message" => "Throwing an error without a message. Always provide a descriptive error message to help with debugging.".to_string(),
            "pattern_mismatch" => format!("Error message '{}' doesn't match required patterns. Ensure error messages follow the project's error message conventions.", details),
            _ => format!("Generic throw detected: {}. Use specific error messages with context.", details),
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, issue_type: &str) -> Vec<String> {
        match issue_type {
            "generic_message" => vec![
                "Replace with a specific error message describing what failed".to_string(),
                "Include relevant context like variable values or operation details".to_string(),
                "Use error codes or categories for different error types".to_string(),
            ],
            "too_short" => vec![
                "Add more context about what operation failed".to_string(),
                "Include relevant data values in the error message".to_string(),
                "Explain what the user should do to resolve the error".to_string(),
            ],
            "missing_message" => vec![
                "Add a descriptive error message as the first parameter".to_string(),
                "Use new Error('descriptive message') instead of throwing a value".to_string(),
            ],
            "pattern_mismatch" => vec![
                "Follow the project's error message format conventions".to_string(),
                "Include error codes or prefixes as required".to_string(),
            ],
            _ => vec!["Provide a specific, descriptive error message".to_string()],
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

    /// Check if a throw statement violates generic throw rules
    fn check_throw_statement(&mut self, node: &ThrowStatement) {
        // Allow rethrowing caught errors if configured
        if self.config.allow_rethrow {
            // This would need more complex analysis to detect rethrows
            // For now, we'll check all throws
        }

        if let Some(argument) = &node.argument {
            // Check for throwing variables/identifiers
            if let Expression::Identifier(_) = argument {
                if !self.config.allow_throw_variable {
                    self.issues.push(self.create_throw_issue("missing_message", "throwing variable", node.span));
                }
                return;
            }

            // Check for throwing bare strings
            if let Expression::StringLiteral(literal) = argument {
                if self.config.strict_mode {
                    self.issues.push(self.create_throw_issue("missing_message", "throwing bare string", node.span));
                    return;
                }

                self.validate_message(&literal.value, node.span);
                return;
            }

            // Check for new Error() constructions
            if let Expression::NewExpression(_) = argument {
                if let Some(error_class) = self.get_error_class_name(argument) {
                    if self.config.strict_mode && error_class == "Error" {
                        self.issues.push(self.create_throw_issue("generic_message", "using generic Error class", node.span));
                        return;
                    }

                    // Check if it's a custom error class
                    if !self.custom_error_classes.contains(&error_class) {
                        // Get the message from the constructor arguments
                        if let Some(message) = self.get_error_message(argument) {
                            self.validate_message(&message, node.span);
                        } else {
                            self.issues.push(self.create_throw_issue("missing_message", "no message provided", node.span));
                        }
                    }
                }
                return;
            }

            // Other types of throws (numbers, objects, etc.)
            self.issues.push(self.create_throw_issue("missing_message", "throwing non-error value", node.span));
        } else {
            self.issues.push(self.create_throw_issue("missing_message", "empty throw", node.span));
        }
    }

    fn get_error_class_name(&self, expr: &Expression) -> Option<String> {
        if let Expression::NewExpression(new_expr) = expr {
            if let Expression::Identifier(ident) = &new_expr.callee {
                return Some(ident.name.to_string());
            }
        }
        None
    }

    fn get_error_message(&self, expr: &Expression) -> Option<String> {
        if let Expression::NewExpression(new_expr) = expr {
            if let Some(first_arg) = new_expr.arguments.first() {
                match first_arg {
                    Argument::Expression(Expression::StringLiteral(literal)) => {
                        Some(literal.value.to_string())
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn validate_message(&mut self, message: &str, span: Span) {
        if message.trim().is_empty() {
            self.issues.push(self.create_throw_issue("missing_message", "empty message", span));
            return;
        }

        if self.allowed_generic_messages.contains(&message.to_lowercase()) {
            return;
        }

        if self.is_generic_error_message(message) {
            self.issues.push(self.create_throw_issue("generic_message", message, span));
            return;
        }

        if (message.trim().len() as u32) < self.config.minimum_message_length {
            self.issues.push(self.create_throw_issue("too_short", message, span));
            return;
        }

        if !self.matches_required_patterns(message) {
            self.issues.push(self.create_throw_issue("pattern_mismatch", message, span));
        }
    }

    fn is_generic_error_message(&self, message: &str) -> bool {
        let binding = message.to_lowercase();
        let normalized = binding.trim();
        let generic_messages = [
            "error", "something went wrong", "something failed",
            "operation failed", "invalid", "invalid input", "bad input", "error occurred",
            "an error occurred", "failed", "failure", "exception", "unexpected error",
            "internal error", "system error", "unknown error"
        ];

        generic_messages.iter().any(|&generic| normalized.contains(generic))
    }

    fn matches_required_patterns(&self, message: &str) -> bool {
        if self.required_patterns.is_empty() {
            return true;
        }
        self.required_patterns.iter().any(|pattern| pattern.is_match(message))
    }
}

impl<'a> Visit<'a> for C018Visitor<'a> {
    fn visit_throw_statement(&mut self, node: &ThrowStatement<'a>) {
        self.check_throw_statement(node);

        // Continue visiting the thrown expression
        if let Some(argument) = &node.argument {
            self.visit_expression(argument);
        }
    }
}

impl<'a> C018Visitor<'a> {
    fn new(program: &'a Program<'a>, code: &'a str, config: &'a C018Config) -> Self {
        let generic_messages = [
            "error", "Error", "ERROR", "something went wrong", "something failed",
            "operation failed", "invalid", "invalid input", "bad input", "error occurred",
            "an error occurred", "failed", "failure", "exception", "unexpected error",
            "internal error", "system error", "unknown error"
        ].iter().map(|s| s.to_string()).collect();

        let mut allowed_generic_messages = HashSet::new();
        if !config.allowed_generic_messages.is_empty() {
            allowed_generic_messages.extend(config.allowed_generic_messages.clone());
        }

        let mut custom_error_classes = HashSet::from([
            "ValidationError".to_string(), "BusinessError".to_string(),
            "NetworkError".to_string(), "AuthenticationError".to_string(),
            "AuthorizationError".to_string()
        ]);
        if !config.custom_error_classes.is_empty() {
            custom_error_classes.extend(config.custom_error_classes.clone());
        }

        let required_message_patterns = if !config.required_message_patterns.is_empty() {
            config.required_message_patterns.iter()
                .filter_map(|pattern| Regex::new(pattern).ok())
                .collect()
        } else {
            Vec::new()
        };

        Self {
            program,
            code,
            issues: Vec::new(),
            allow_generic_in_tests: config.allow_generic_in_tests,
            allow_rethrow: config.allow_rethrow,
            allow_throw_variable: config.allow_throw_variable,
            required_message_patterns,
            minimum_message_length: config.minimum_message_length,
            allowed_generic_messages,
            custom_error_classes,
            strict_mode: config.strict_mode,
            generic_messages,
        }
    }

    fn is_test_context(&self) -> bool {
        // Simple heuristic: check if filename contains test patterns
        // In production, this could use semantic analysis to detect test files
        self.allow_generic_in_tests && (
            self.code.contains(".test.") ||
            self.code.contains(".spec.") ||
            self.code.contains("__tests__")
        )
    }

    fn get_error_message(&self, argument: &Expression) -> Option<String> {
        match argument {
            Expression::StringLiteral(literal) => Some(literal.value.to_string()),
            Expression::NewExpression(new_expr) => {
                if let Some(first_arg) = new_expr.arguments.first() {
                    if let Argument::StringLiteral(literal) = first_arg {
                        return Some(literal.value.to_string());
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn get_error_class_name(&self, argument: &Expression) -> Option<String> {
        if let Expression::NewExpression(new_expr) = argument {
            if let Expression::Identifier(ident) = &new_expr.callee {
                return Some(ident.name.to_string());
            }
        }
        None
    }

    fn is_generic_error_message(&self, message: &str) -> bool {
        let normalized = message.to_lowercase().trim();
        self.generic_messages.contains(normalized) || self.generic_messages.contains(message.trim())
    }

    fn is_message_too_short(&self, message: &str) -> bool {
        (message.trim().len() as u32) < self.minimum_message_length
    }

    fn matches_required_patterns(&self, message: &str) -> bool {
        if self.required_message_patterns.is_empty() {
            return true;
        }
        self.required_message_patterns.iter().any(|pattern| pattern.is_match(message))
    }

    fn is_allowed_generic_message(&self, message: &str) -> bool {
        self.allowed_generic_messages.contains(message.trim())
    }

    fn validate_message(&mut self, node: &ThrowStatement, message: &str, span: oxc_span::Span) {
        if message.trim().is_empty() {
            let (line, column) = span_to_line_col_legacy(self.program, span);
            self.issues.push(LintIssue {
                rule_name: "C018".to_string(),
                severity: LintSeverity::Error,
                message: "Error message cannot be empty. Provide a specific error message.".to_string(),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
            return;
        }

        if self.is_allowed_generic_message(message) {
            return;
        }

        if self.is_generic_error_message(message) {
            let (line, column) = span_to_line_col_legacy(self.program, span);
            self.issues.push(LintIssue {
                rule_name: "C018".to_string(),
                severity: LintSeverity::Warning,
                message: format!("Generic error message '{}' should be more specific.", message.trim()),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
            return;
        }

        if self.is_message_too_short(message) {
            let (line, column) = span_to_line_col_legacy(self.program, span);
            self.issues.push(LintIssue {
                rule_name: "C018".to_string(),
                severity: LintSeverity::Warning,
                message: format!("Error message too short (minimum {} characters).", self.minimum_message_length),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
            return;
        }

        if !self.matches_required_patterns(message) {
            let (line, column) = span_to_line_col_legacy(self.program, span);
            self.issues.push(LintIssue {
                rule_name: "C018".to_string(),
                severity: LintSeverity::Warning,
                message: "Error message doesn't match required patterns.".to_string(),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
        }
    }

    fn check_throw_statement(&mut self, node: &ThrowStatement) {
        if self.is_test_context() {
            return;
        }

        // Check for throw without argument
        if node.argument.is_none() {
            let (line, column) = span_to_line_col_legacy(self.program, node.span);
            self.issues.push(LintIssue {
                rule_name: "C018".to_string(),
                severity: LintSeverity::Error,
                message: "Throw statement must include a specific error message.".to_string(),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
            return;
        }

        let argument = node.argument.as_ref().unwrap();

        // Check for throwing variables/identifiers
        if let Expression::Identifier(_) = argument {
            if !self.allow_throw_variable {
                let (line, column) = span_to_line_col_legacy(self.program, node.span);
                self.issues.push(LintIssue {
                    rule_name: "C018".to_string(),
                    severity: LintSeverity::Warning,
                    message: "Do not throw generic errors. Provide a specific error message.".to_string(),
                    line,
                    column,
                    fix_available: false, // AI can enhance this
                });
            }
            return;
        }

        // Check for throwing bare strings
        if let Expression::StringLiteral(literal) = argument {
            if self.strict_mode {
                let (line, column) = span_to_line_col_legacy(self.program, node.span);
                self.issues.push(LintIssue {
                    rule_name: "C018".to_string(),
                    severity: LintSeverity::Warning,
                    message: "Throwing bare string is not recommended, use Error object with message.".to_string(),
                    line,
                    column,
                    fix_available: false, // AI can enhance this
                });
                return;
            }

            self.validate_message(node, &literal.value, node.span);
            return;
        }

        // Check for new Error() constructions
        if let Expression::NewExpression(_) = argument {
            if let Some(error_class) = self.get_error_class_name(argument) {
                if self.strict_mode && error_class == "Error" {
                    let (line, column) = span_to_line_col_legacy(self.program, node.span);
                    self.issues.push(LintIssue {
                        rule_name: "C018".to_string(),
                        severity: LintSeverity::Warning,
                        message: "Use specific error class instead of generic Error.".to_string(),
                        line,
                        column,
                        fix_available: false, // AI can enhance this
                    });
                }
            }

            if let Some(message) = self.get_error_message(argument) {
                self.validate_message(node, &message, node.span);
            } else {
                let (line, column) = span_to_line_col_legacy(self.program, node.span);
                self.issues.push(LintIssue {
                    rule_name: "C018".to_string(),
                    severity: LintSeverity::Error,
                    message: "Throw statement must include a specific error message.".to_string(),
                    line,
                    column,
                    fix_available: false, // AI can enhance this
                });
            }
            return;
        }

        // Generic throw statement
        let (line, column) = span_to_line_col_legacy(self.program, node.span);
        self.issues.push(LintIssue {
            rule_name: "C018".to_string(),
            severity: LintSeverity::Warning,
            message: "Do not throw generic errors. Provide a specific error message.".to_string(),
            line,
            column,
            fix_available: false, // AI can enhance this
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        check_no_generic_throw(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_generic_error_messages_violation() {
        let code = r#"
function validateInput(input) {
    if (!input) {
        throw new Error("error"); // Generic message
    }
    if (input.length < 5) {
        throw new Error("invalid"); // Generic message
    }
    if (!input.includes("@")) {
        throw new Error("failed"); // Generic message
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Generic error message")));
    }

    #[test]
    fn test_short_error_messages_violation() {
        let code = r#"
function processData(data) {
    if (!data) {
        throw new Error("bad"); // Too short
    }
    if (data.length === 0) {
        throw new Error("empty"); // Too short
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("too short")));
    }

    #[test]
    fn test_empty_error_message_violation() {
        let code = r#"
function handleError() {
    throw new Error(""); // Empty message
    throw new Error(); // No message
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("empty") || issue.message.contains("must include")));
    }

    #[test]
    fn test_specific_error_messages_compliant() {
        let code = r#"
function validateUserInput(input) {
    if (!input) {
        throw new Error("User input cannot be null or undefined");
    }
    if (input.length < 5) {
        throw new Error("User input must be at least 5 characters long");
    }
    if (!input.includes("@")) {
        throw new Error("User input must contain a valid email address with @ symbol");
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_custom_error_classes_compliant() {
        let code = r#"
function authenticateUser(credentials) {
    if (!credentials.username) {
        throw new ValidationError("Username is required for authentication");
    }
    if (!credentials.password) {
        throw new AuthenticationError("Password is required for user authentication");
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_throw_variable_violation() {
        let code = r#"
function handleError() {
    const error = new Error("Something went wrong");
    throw error; // Throwing variable
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("generic")));
    }
}