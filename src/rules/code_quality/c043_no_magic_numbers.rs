//! # C043: No Magic Numbers Rule
//!
//! Detects magic numbers in code that should be replaced with named constants
//! to improve code readability, maintainability, and reduce the risk of errors.
//! Promotes self-documenting code through meaningful constant names.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col;
use oxc_ast::ast::{Program, Expression, BinaryExpression, UnaryExpression, ArrayExpression, CallExpression, Argument};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration options for the C043 rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct C043Config {
    /// Whether to ignore magic numbers in test files (default: true).
    pub ignore_in_tests: bool,
    /// A list of numbers to ignore (default: `[-1, 0, 1, 2]`).
    pub ignore_numbers: Vec<f64>,
    /// Whether to ignore magic numbers in array indexes (default: true).
    pub ignore_array_indexes: bool,
    /// Whether to ignore magic numbers in default assignments (default: true).
    pub ignore_default_assignments: bool,
    /// Whether to ignore magic numbers in return statements (default: false).
    pub ignore_return_statements: bool,
    /// Whether to enforce constants for duplicated numbers (default: true).
    pub enforce_const_for_duplicates: bool,
    /// The minimum number of occurrences to suggest constants (default: 2).
    pub duplicate_threshold: u32,
    /// The maximum allowed value for magic numbers (larger numbers are more problematic).
    pub max_allowed_value: f64,
    /// A list of function names where magic numbers are allowed (e.g., `setTimeout`, `setInterval`).
    pub allowed_function_contexts: Vec<String>,
}

/// The main entry point for the C043 rule checking.
pub fn check_no_magic_numbers(program: &Program, _semantic: &Semantic, code: &str, _config: Option<&str>) -> Vec<LintIssue> {
    let config = C043Config::default();
    let mut visitor = MagicNumberVisitor::new(program, code, &config);

    // First pass: collect all numeric literals
    visitor.visit_program(program);

    // Second pass: analyze duplicates and generate issues
    visitor.finalize_issues()
}

/// Holds information about a number usage, including its value, span, context, and a suggested name.
#[derive(Debug, Clone)]
struct NumberUsage {
    value: f64,
    span: Span,
    context: NumberContext,
    suggested_name: Option<String>,
}

/// The context in which a number is used.
#[derive(Debug, Clone)]
enum NumberContext {
    BinaryOperation,
    ArrayIndex,
    FunctionCall(String),
    ReturnStatement,
    DefaultAssignment,
    Comparison,
    ArithmeticOperation,
    Timeout,
    StatusCode,
    Generic,
}

impl NumberContext {
    /// Checks if the context should be ignored based on the configuration.
    fn should_ignore(&self, config: &C043Config) -> bool {
        match self {
            NumberContext::ArrayIndex => config.ignore_array_indexes,
            NumberContext::DefaultAssignment => config.ignore_default_assignments,
            NumberContext::ReturnStatement => config.ignore_return_statements,
            NumberContext::FunctionCall(func_name) => {
                config.allowed_function_contexts.iter().any(|allowed| func_name.contains(allowed)) ||
                // Default allowed function contexts
                matches!(func_name.as_str(), "setTimeout" | "setInterval" | "sleep" | "delay")
            }
            _ => false,
        }
    }

    /// Returns a description of the number context.
    fn description(&self) -> &'static str {
        match self {
            NumberContext::BinaryOperation => "binary operation",
            NumberContext::ArrayIndex => "array index",
            NumberContext::FunctionCall(_) => "function call",
            NumberContext::ReturnStatement => "return statement",
            NumberContext::DefaultAssignment => "default assignment",
            NumberContext::Comparison => "comparison operation",
            NumberContext::ArithmeticOperation => "arithmetic operation",
            NumberContext::Timeout => "timeout value",
            NumberContext::StatusCode => "status code",
            NumberContext::Generic => "expression",
        }
    }
}

/// An AST visitor for detecting magic number violations.
struct MagicNumberVisitor<'a> {
    config: &'a C043Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    number_usages: Vec<NumberUsage>,
    current_function_name: Option<String>,
    in_array_index: bool,
    in_default_assignment: bool,
    in_return_statement: bool,
}

impl<'a> MagicNumberVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a C043Config) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            number_usages: Vec::new(),
            current_function_name: None,
            in_array_index: false,
            in_default_assignment: false,
            in_return_statement: false,
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Replace magic numbers with named constants".to_string(),
            "Use descriptive constant names that explain the value's purpose".to_string(),
            "Consider extracting constants to a separate constants file".to_string(),
            "Use enums for related constant values".to_string(),
        ]
    }

    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

    fn get_ignored_numbers(&self) -> HashSet<String> {
        let mut ignore_numbers = HashSet::new();

        // Default ignored numbers
        let default_ignored = vec![-1.0, 0.0, 1.0, 2.0];
        for num in default_ignored {
            ignore_numbers.insert(format!("{:.1}", num));
            ignore_numbers.insert(num.to_string());
        }

        // Add custom ignored numbers
        for num in &self.config.ignore_numbers {
            ignore_numbers.insert(format!("{:.1}", num));
            ignore_numbers.insert(num.to_string());
        }

        ignore_numbers
    }

    fn is_test_context(&self) -> bool {
        self.config.ignore_in_tests && (
            self.source_code.contains(".test.") ||
            self.source_code.contains(".spec.") ||
            self.source_code.contains("__tests__") ||
            self.source_code.contains("describe(") ||
            self.source_code.contains("it(") ||
            self.source_code.contains("test(")
        )
    }

    fn should_ignore_number(&self, value: f64) -> bool {
        let value_str = value.to_string();
        let value_formatted = format!("{:.1}", value);

        let ignored_numbers = self.get_ignored_numbers();
        ignored_numbers.contains(&value_str) || ignored_numbers.contains(&value_formatted)
    }

    fn suggest_constant_name(&self, value: f64, context: &NumberContext) -> Option<String> {
        match context {
            NumberContext::Timeout if value >= 1000.0 => {
                Some(format!("TIMEOUT_{}MS", value as u32))
            }
            NumberContext::StatusCode if value >= 100.0 && value < 600.0 => {
                match value as u32 {
                    200 => Some("HTTP_OK".to_string()),
                    201 => Some("HTTP_CREATED".to_string()),
                    400 => Some("HTTP_BAD_REQUEST".to_string()),
                    401 => Some("HTTP_UNAUTHORIZED".to_string()),
                    403 => Some("HTTP_FORBIDDEN".to_string()),
                    404 => Some("HTTP_NOT_FOUND".to_string()),
                    500 => Some("HTTP_INTERNAL_ERROR".to_string()),
                    _ => Some(format!("HTTP_STATUS_{}", value as u32)),
                }
            }
            NumberContext::Comparison if value > 0.0 && value < 100.0 => {
                Some(format!("MAX_{}", value.to_string().replace(".", "_")))
            }
            _ => {
                if value >= 100.0 {
                    Some(format!("CONSTANT_{}", value.to_string().replace(".", "_")))
                } else {
                    None
                }
            }
        }
    }

    fn determine_context(&self, parent_is_binary: bool, parent_is_call: bool) -> NumberContext {
        if self.in_array_index {
            NumberContext::ArrayIndex
        } else if self.in_default_assignment {
            NumberContext::DefaultAssignment
        } else if self.in_return_statement {
            NumberContext::ReturnStatement
        } else if let Some(func_name) = &self.current_function_name {
            NumberContext::FunctionCall(func_name.clone())
        } else if parent_is_binary {
            NumberContext::Comparison
        } else {
            NumberContext::Generic
        }
    }

    fn check_numeric_literal(&mut self, value: f64, span: Span, parent_is_binary: bool, parent_is_call: bool) {
        if self.is_test_context() || self.should_ignore_number(value) {
            return;
        }

        let context = self.determine_context(parent_is_binary, parent_is_call);

        if context.should_ignore(self.config) {
            return;
        }

        let suggested_name = self.suggest_constant_name(value, &context);

        self.number_usages.push(NumberUsage {
            value,
            span,
            context,
            suggested_name,
        });
    }

    fn finalize_issues(mut self) -> Vec<LintIssue> {
        // Count duplicate values
        let mut value_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        for usage in &self.number_usages {
            let key = usage.value.to_string();
            *value_counts.entry(key).or_insert(0) += 1;
        }

        // Generate issues
        for usage in &self.number_usages {
            let value_key = usage.value.to_string();
            let occurrence_count = value_counts.get(&value_key).unwrap_or(&1);

            let should_report = if self.config.enforce_const_for_duplicates {
                *occurrence_count >= self.config.duplicate_threshold
            } else {
                usage.value.abs() > self.config.max_allowed_value ||
                (usage.value.abs() > 10.0 && usage.value.fract() == 0.0) // Integer values > 10
            };

            if should_report {
                let (line, column) = self.calculate_line_column(usage.span);

                let base_message = if let Some(suggested_name) = &usage.suggested_name {
                    format!(
                        "Magic number {} in {}. Consider using a named constant: const {} = {};",
                        usage.value,
                        usage.context.description(),
                        suggested_name,
                        usage.value
                    )
                } else {
                    format!(
                        "Magic number {} in {}{}. Consider using a named constant for better code readability.",
                        usage.value,
                        usage.context.description(),
                        if *occurrence_count > 1 {
                            format!(" (appears {} times)", occurrence_count)
                        } else {
                            String::new()
                        }
                    )
                };

                self.issues.push(LintIssue {
                    rule_name: "C043".to_string(),
                    severity: if usage.value.abs() > 100.0 || *occurrence_count >= 3 {
                        LintSeverity::Warning
                    } else {
                        LintSeverity::Info
                    },
                    message: self.generate_ai_enhanced_message(&base_message, usage.span),
                    line,
                    column,
                    fix_available: true, // AI can suggest appropriate constant names
                });
            }
        }

        self.issues
    }
}

impl<'a> Visit<'a> for MagicNumberVisitor<'a> {
    fn visit_expression(&mut self, node: &Expression<'a>) {
        match node {
            Expression::NumericLiteral(literal) => {
                self.check_numeric_literal(literal.value, literal.span, false, false);
            }
            Expression::BinaryExpression(binary) => {
                self.visit_binary_expression(binary);
            }
            Expression::UnaryExpression(unary) => {
                self.visit_unary_expression(unary);
            }
            Expression::CallExpression(call) => {
                // Track function name for context
                let old_function_name = self.current_function_name.clone();

                if let Expression::Identifier(ident) = &call.callee {
                    self.current_function_name = Some(ident.name.to_string());
                } else if let Expression::MemberExpression(member) = &call.callee {
                    if let Some(property) = &member.property {
                        if let Expression::Identifier(prop_ident) = property {
                            self.current_function_name = Some(prop_ident.name.to_string());
                        }
                    }
                }

                self.visit_call_expression(call);
                self.current_function_name = old_function_name;
            }
            Expression::ArrayExpression(array) => {
                self.visit_array_expression(array);
            }
            _ => {
                // Default visiting for other expressions
                match node {
                    Expression::ArrowFunctionExpression(arrow) => self.visit_arrow_function_expression(arrow),
                    Expression::AssignmentExpression(assign) => self.visit_assignment_expression(assign),
                    Expression::ConditionalExpression(cond) => self.visit_conditional_expression(cond),
                    Expression::LogicalExpression(logical) => self.visit_logical_expression(logical),
                    Expression::MemberExpression(member) => self.visit_member_expression(member),
                    Expression::ObjectExpression(obj) => self.visit_object_expression(obj),
                    Expression::UpdateExpression(update) => self.visit_update_expression(update),
                    _ => {}
                }
            }
        }
    }

    fn visit_binary_expression(&mut self, node: &BinaryExpression<'a>) {
        // Check left side
        if let Expression::NumericLiteral(literal) = &node.left {
            self.check_numeric_literal(literal.value, literal.span, true, false);
        } else {
            self.visit_expression(&node.left);
        }

        // Check right side
        if let Expression::NumericLiteral(literal) = &node.right {
            self.check_numeric_literal(literal.value, literal.span, true, false);
        } else {
            self.visit_expression(&node.right);
        }
    }

    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        // Visit callee
        self.visit_expression(&node.callee);

        // Visit arguments with function call context
        for arg in &node.arguments {
            match arg {
                Argument::Expression(Expression::NumericLiteral(literal)) => {
                    self.check_numeric_literal(literal.value, literal.span, false, true);
                }
                Argument::Expression(expr) => {
                    self.visit_expression(expr);
                }
                Argument::SpreadElement(spread) => {
                    self.visit_spread_element(spread);
                }
                _ => {}
            }
        }
    }

    fn visit_array_expression(&mut self, node: &ArrayExpression<'a>) {
        for (index, element) in node.elements.iter().enumerate() {
            if let Some(expr) = element {
                // Track if we're in an array index context
                let old_in_array_index = self.in_array_index;
                self.in_array_index = index < 5; // Only consider first few indices as potential indexes

                self.visit_expression(expr);

                self.in_array_index = old_in_array_index;
            }
        }
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

        check_no_magic_numbers(&parse_result.program, &semantic_result.semantic, code, None)
    }

    #[test]
    fn test_magic_numbers_violation() {
        let code = r#"
function calculateTimeout() {
    return 30000; // Magic number - should be named constant
}

function processData(items) {
    if (items.length > 100) { // Magic number
        console.log("Large dataset");
    }

    return items.slice(0, 50); // Magic number
}

const config = {
    maxRetries: 5,
    timeoutMs: 60000,
    batchSize: 250
};
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("30000")));
        assert!(issues.iter().any(|issue| issue.message.contains("100")));
    }

    #[test]
    fn test_allowed_numbers_compliant() {
        let code = r#"
function processArray(items) {
    if (items.length === 0) return null;
    if (items.length === 1) return items[0];
    if (items.length === 2) return items;

    return items.slice(-1)[0];
}

const multiplier = -1;
const defaultValue = 0;
const increment = 1;
const doubleValue = 2;
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // These numbers should be ignored
    }

    #[test]
    fn test_named_constants_compliant() {
        let code = r#"
const TIMEOUT_MS = 30000;
const MAX_ITEMS = 100;
const BATCH_SIZE = 50;
const HTTP_OK = 200;
const MAX_RETRIES = 5;

function calculateTimeout() {
    return TIMEOUT_MS;
}

function processData(items) {
    if (items.length > MAX_ITEMS) {
        console.log("Large dataset");
    }

    return items.slice(0, BATCH_SIZE);
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_function_context_allowed() {
        let code = r#"
function scheduleTask() {
    setTimeout(() => {
        console.log("Task executed");
    }, 5000); // Allowed in setTimeout

    setInterval(() => {
        checkStatus();
    }, 30000); // Allowed in setInterval
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Should be allowed in timeout functions
    }

    #[test]
    fn test_test_context_ignored() {
        let code = r#"
// math.test.js
describe('Math operations', () => {
    it('should handle large numbers', () => {
        expect(calculate(999999)).toBe(1000000);
        expect(process(12345)).toEqual([1, 2, 3, 4, 5]);
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Should be ignored in test context
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
function timeout() {
    return 30000;
}

function checkStatus() {
    if (response.status === 200) {
        return true;
    }
    return false;
}
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = check_no_magic_numbers(&parse_result.program, &semantic_result.semantic, code);

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.fix_available));
        assert!(issues.iter().any(|issue| issue.message.contains("TIMEOUT") || issue.message.contains("HTTP_OK")));
    }
}