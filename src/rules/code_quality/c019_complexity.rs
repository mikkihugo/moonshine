//! # C019: Function Complexity Limits
//!
//! This rule enforces maximum complexity limits for functions to maintain code readability
//! and maintainability. It uses cyclomatic complexity analysis to detect overly complex functions.
//!
//! @category code-quality
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 1.0.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Function, ArrowFunctionExpression, Statement, IfStatement, ForStatement, ForInStatement, ForOfStatement, WhileStatement, DoWhileStatement, SwitchStatement, SwitchCase, BinaryExpression, LogicalExpression, ConditionalExpression, Expression, FunctionBody};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};

/// Configuration options for C019 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C019Config {
    /// Maximum allowed cyclomatic complexity (default: 10)
    #[serde(default = "default_max_complexity")]
    pub max_complexity: u32,
    /// Whether to ignore arrow functions (default: false)
    #[serde(default)]
    pub ignore_arrow_functions: bool,
    /// Whether to ignore simple getters/setters (default: true)
    #[serde(default = "default_ignore_simple_getters")]
    pub ignore_simple_getters: bool,
}

fn default_max_complexity() -> u32 {
    10
}

fn default_ignore_simple_getters() -> bool {
    true
}

impl Default for C019Config {
    fn default() -> Self {
        Self {
            max_complexity: 10,
            ignore_arrow_functions: false,
            ignore_simple_getters: true,
        }
    }
}

/// Main entry point for C019 rule checking
pub fn check_complexity(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C019Config::default();
    let mut visitor = C019Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting function complexity violations
struct C019Visitor<'a> {
    config: &'a C019Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    current_complexity: u32,
}

impl<'a> C019Visitor<'a> {
    fn new(config: &'a C019Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            current_complexity: 0,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, complexity: u32, function_name: &str) -> String {
        format!("Function '{}' has a cyclomatic complexity of {} which exceeds the maximum allowed complexity of {}. High complexity makes code harder to understand, test, and maintain. Consider breaking this function into smaller, more focused functions.", function_name, complexity, self.config.max_complexity)
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Extract complex conditional logic into separate functions".to_string(),
            "Use early returns to reduce nesting and complexity".to_string(),
            "Split the function into smaller, single-responsibility functions".to_string(),
            "Consider using polymorphism or strategy pattern for complex conditionals".to_string(),
            "Add comprehensive unit tests to ensure correctness when refactoring".to_string(),
        ]
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

    /// Create lint issue for complexity violation with AI enhancement
    fn create_complexity_issue(&self, complexity: u32, function_name: &str, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(complexity, function_name);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions();

        LintIssue {
            rule_name: "moonshine/c019".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Calculate cyclomatic complexity for a function
    fn calculate_function_complexity(&self, statements: &[Statement]) -> u32 {
        let mut complexity = 1; // Base complexity

        for stmt in statements {
            complexity += self.calculate_statement_complexity(stmt);
        }

        complexity
    }

    /// Calculate complexity contribution of a single statement
    fn calculate_statement_complexity(&self, stmt: &Statement) -> u32 {
        match stmt {
            Statement::IfStatement(_) => 1, // +1 for if
            Statement::ForStatement(_) => 1, // +1 for for
            Statement::ForInStatement(_) => 1, // +1 for for-in
            Statement::ForOfStatement(_) => 1, // +1 for for-of
            Statement::WhileStatement(_) => 1, // +1 for while
            Statement::DoWhileStatement(_) => 1, // +1 for do-while
            Statement::SwitchStatement(switch) => switch.cases.len() as u32, // +1 for each case
            Statement::ExpressionStatement(expr_stmt) => {
                self.calculate_expression_complexity(&expr_stmt.expression)
            },
            Statement::ReturnStatement(ret_stmt) => {
                if let Some(expr) = &ret_stmt.argument {
                    self.calculate_expression_complexity(expr)
                } else {
                    0
                }
            },
            _ => 0,
        }
    }

    /// Calculate complexity contribution of expressions
    fn calculate_expression_complexity(&self, expr: &Expression) -> u32 {
        match expr {
            Expression::BinaryExpression(bin_expr) => {
                match bin_expr.operator {
                    oxc_ast::ast::BinaryOperator::LogicalAnd |
                    oxc_ast::ast::BinaryOperator::LogicalOr => 1, // +1 for && and ||
                    _ => 0,
                }
            },
            Expression::LogicalExpression(_) => 1, // +1 for logical expressions
            Expression::ConditionalExpression(_) => 1, // +1 for ternary operator
            Expression::CallExpression(call_expr) => {
                // Check for recursive calls or complex call chains
                0 // For now, don't add complexity for calls
            },
            _ => 0,
        }
    }

    /// Get function name for reporting
    fn get_function_name(&self, func: &Function) -> String {
        // Try to get function name from context - this is simplified
        "anonymous function".to_string()
    }

    /// Check if function should be ignored
    fn should_ignore_function(&self, func: &Function) -> bool {
        if self.config.ignore_simple_getters {
            // Simple getters/setters have very low complexity
            if let Some(body) = &func.body {
                if body.statements.len() <= 2 {
                    return true;
                }
            }
        }
        false
    }
}

impl<'a> Visit<'a> for C019Visitor<'a> {
    fn visit_function(&mut self, func: &Function<'a>) {
        if self.should_ignore_function(func) {
            // Continue visiting function body without checking complexity
            oxc_ast_visit::walk::walk_function(self, func);
            return;
        }

        if let Some(body) = &func.body {
            let complexity = self.calculate_function_complexity(&body.statements);

            if complexity > self.config.max_complexity {
                let function_name = self.get_function_name(func);
                self.issues.push(self.create_complexity_issue(complexity, &function_name, func.span));
            }
        }

        // Continue visiting function body
        oxc_ast_visit::walk::walk_function(self, func);
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        if self.config.ignore_arrow_functions {
            return;
        }

        // For arrow functions, we check the body if it's a block
        let complexity = if arrow.expression {
            1 // Simple expression body
        } else if let Some(body) = &arrow.body {
            self.calculate_function_complexity(&body.statements)
        } else {
            1 // Fallback
        };

        if complexity > self.config.max_complexity {
            self.issues.push(self.create_complexity_issue(complexity, "arrow function", arrow.span));
        }

        // Continue visiting arrow function body
        oxc_ast_visit::walk::walk_arrow_function_expression(self, arrow);
    }
}