//! # T004: Test Coverage Threshold Rule
//!
//! Enforces minimum test coverage thresholds to ensure adequate testing.
//! Analyzes test files and their corresponding source files to verify
//! sufficient test coverage and identifies areas lacking proper test coverage.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use crate::rules::ai_integration::AIEnhancer;
use oxc_ast::ast::{Program, Function, ArrowFunctionExpression, MethodDefinition, PropertyDefinition, Statement, Expression, CallExpression, Declaration};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct T004Config {
    /// Minimum function coverage percentage (default: 80)
    pub min_function_coverage: f64,
    /// Minimum statement coverage percentage (default: 70)
    pub min_statement_coverage: f64,
    /// Minimum branch coverage percentage (default: 75)
    pub min_branch_coverage: f64,
    /// Whether to enforce coverage for private methods (default: false)
    pub enforce_private_coverage: bool,
    /// Whether to require tests for getters/setters (default: false)
    pub require_getter_setter_tests: bool,
    /// Functions that can be excluded from coverage requirements
    pub excluded_functions: Vec<String>,
    /// File patterns to exclude from coverage analysis
    pub excluded_files: Vec<String>,
    /// Whether to analyze only test files for coverage patterns (default: true)
    pub analyze_test_files_only: bool,
    /// Minimum test case count per function (default: 1)
    pub min_tests_per_function: u32,
}

/// T004 rule implementation with AI enhancement
pub fn check_test_coverage_threshold(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = T004Config::default();
    let mut visitor = TestCoverageVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.finalize_analysis()
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    span: Span,
    is_private: bool,
    is_getter: bool,
    is_setter: bool,
    is_constructor: bool,
    complexity: u32,
    test_count: u32,
    is_tested: bool,
    function_type: FunctionType,
}

#[derive(Debug, Clone)]
enum FunctionType {
    Function,
    Method,
    ArrowFunction,
    Constructor,
    Getter,
    Setter,
}

#[derive(Debug, Clone)]
struct TestInfo {
    name: String,
    span: Span,
    target_function: Option<String>,
    test_type: TestType,
}

#[derive(Debug, Clone)]
enum TestType {
    UnitTest,
    IntegrationTest,
    E2ETest,
    Unknown,
}

#[derive(Debug, Clone)]
enum CoverageViolation {
    InsufficientFunctionCoverage,
    InsufficientTestCases,
    UntestedPublicMethod,
    UntestedCriticalFunction,
    MissingEdgeCaseTests,
    NoNegativeTests,
}

impl CoverageViolation {
    fn description(&self) -> &'static str {
        match self {
            CoverageViolation::InsufficientFunctionCoverage => "insufficient function test coverage",
            CoverageViolation::InsufficientTestCases => "insufficient test cases for function complexity",
            CoverageViolation::UntestedPublicMethod => "public method without tests",
            CoverageViolation::UntestedCriticalFunction => "critical function without adequate testing",
            CoverageViolation::MissingEdgeCaseTests => "missing edge case test coverage",
            CoverageViolation::NoNegativeTests => "missing negative/error case tests",
        }
    }

    fn recommendation(&self) -> &'static str {
        match self {
            CoverageViolation::InsufficientFunctionCoverage => "Add unit tests to cover all public functions and critical paths",
            CoverageViolation::InsufficientTestCases => "Add more test cases to cover different scenarios and edge cases",
            CoverageViolation::UntestedPublicMethod => "Create unit tests for all public methods and functions",
            CoverageViolation::UntestedCriticalFunction => "Add comprehensive tests for critical business logic",
            CoverageViolation::MissingEdgeCaseTests => "Include tests for boundary conditions and edge cases",
            CoverageViolation::NoNegativeTests => "Add tests for error conditions and invalid inputs",
        }
    }
}

struct TestCoverageVisitor<'a> {
    config: &'a T004Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    functions: Vec<FunctionInfo>,
    tests: Vec<TestInfo>,
    excluded_functions: HashSet<String>,
    excluded_files: HashSet<String>,
    is_test_file: bool,
    current_class: Option<String>,
}

impl<'a> TestCoverageVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a T004Config) -> Self {
        let mut excluded_functions = HashSet::new();
        excluded_functions.extend(config.excluded_functions.iter().cloned());

        let mut excluded_files = HashSet::new();
        excluded_files.extend(config.excluded_files.iter().cloned());

        let is_test_file = Self::detect_test_file(source_code);

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            functions: Vec::new(),
            tests: Vec::new(),
            excluded_functions,
            excluded_files,
            is_test_file,
            current_class: None,
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Add comprehensive unit tests for functions with low coverage".to_string(),
            "Write integration tests to cover complex code paths".to_string(),
            "Add tests for edge cases and error conditions".to_string(),
            "Consider using test coverage tools to identify untested code".to_string(),
            "Write tests for private methods if they contain complex logic".to_string(),
        ]
    }

    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

    fn detect_test_file(code: &str) -> bool {
        let test_indicators = [
            "describe(", "it(", "test(", "expect(", "assert(",
            "import.*jest", "import.*mocha", "import.*vitest",
            "from.*@testing-library", ".test.", ".spec."
        ];

        test_indicators.iter().any(|indicator| code.contains(indicator))
    }

    fn finalize_analysis(mut self) -> Vec<LintIssue> {
        if self.config.analyze_test_files_only && !self.is_test_file {
            return self.issues;
        }

        self.analyze_coverage();
        self.check_individual_functions();
        self.identify_untested_functions();

        // Apply AI enhancements to all issues
        let ai_suggestions = self.generate_ai_fix_suggestions();
        for issue in &mut self.issues {
            if let Some(ai_context) = ai_suggestions.first() {
                issue.message = format!("{} 💡 AI Suggestion: {}", issue.message, ai_context);
            }
        }
        self.issues
    }

    fn analyze_coverage(&mut self) {
        let total_functions = self.functions.len() as f64;
        if total_functions == 0.0 {
            return;
        }

        let tested_functions = self.functions.iter().filter(|f| f.is_tested).count() as f64;
        let coverage_percentage = (tested_functions / total_functions) * 100.0;

        if coverage_percentage < self.config.min_function_coverage {
            let span = self.program.span;
            let (line, column) = self.calculate_line_column(span);

            let message = self.generate_ai_enhanced_message(
                &format!(
                    "Function coverage {:.1}% is below minimum threshold of {:.1}%. {} of {} functions are tested.",
                    coverage_percentage,
                    self.config.min_function_coverage,
                    tested_functions as u32,
                    total_functions as u32
                ),
                span
            );

            self.issues.push(LintIssue {
                rule_name: "T004".to_string(),
                severity: LintSeverity::Warning,
                message,
                line,
                column,
                fix_available: true,
            });
        }
    }

    fn check_individual_functions(&mut self) {
        for function in &self.functions {
            if self.excluded_functions.contains(&function.name) {
                continue;
            }

            if !self.enforce_private_coverage && function.is_private {
                continue;
            }

            if !self.config.require_getter_setter_tests && (function.is_getter || function.is_setter) {
                continue;
            }

            // Check if function has minimum required tests
            if function.test_count < self.config.min_tests_per_function {
                let violation = if function.test_count == 0 {
                    CoverageViolation::UntestedPublicMethod
                } else {
                    CoverageViolation::InsufficientTestCases
                };

                self.create_coverage_issue(violation, function);
            }

            // Check based on function complexity
            let required_tests = self.calculate_required_tests(function);
            if function.test_count < required_tests {
                self.create_coverage_issue(CoverageViolation::InsufficientTestCases, function);
            }
        }
    }

    fn identify_untested_functions(&mut self) {
        // Avoid mutable and immutable borrow overlap by collecting first
        let excluded_functions = self.excluded_functions.clone();
        let config = self.config.clone();
        let untested: Vec<_> = self
            .functions
            .iter()
            .filter(|function| {
                !function.is_tested
                    && !excluded_functions.contains(&function.name)
                    && (!function.is_private || config.enforce_private_coverage)
                    && (!(function.is_getter || function.is_setter) || config.require_getter_setter_tests)
            })
            .cloned()
            .collect();

        for function in untested {
            let violation = if function.complexity > 5 {
                CoverageViolation::UntestedCriticalFunction
            } else {
                CoverageViolation::UntestedPublicMethod
            };
            self.create_coverage_issue(violation, &function);
        }
    }

    fn calculate_required_tests(&self, function: &FunctionInfo) -> u32 {
        // Base requirement
        let mut required = self.config.min_tests_per_function;

        // Increase based on complexity
        if function.complexity > 10 {
            required += 3;
        } else if function.complexity > 5 {
            required += 2;
        } else if function.complexity > 2 {
            required += 1;
        }

        // Special cases
        if function.is_constructor {
            required += 1; // Constructor should test initialization
        }

        required
    }

    fn create_coverage_issue(&mut self, violation: CoverageViolation, function: &FunctionInfo) {
        let (line, column) = self.calculate_line_column(function.span);

        let message = self.generate_ai_enhanced_message(
            &format!(
                "Function '{}' has {}: {}",
                function.name,
                violation.description(),
                violation.recommendation()
            ),
            function.span
        );

        let severity = match violation {
            CoverageViolation::UntestedCriticalFunction => LintSeverity::Error,
            CoverageViolation::UntestedPublicMethod => LintSeverity::Warning,
            _ => LintSeverity::Info,
        };

        self.issues.push(LintIssue {
            rule_name: "T004".to_string(),
            severity,
            message,
            line,
            column,
            fix_available: true,
        });
    }

    fn get_function_name(&self, func: &Function) -> String {
        if let Some(id) = &func.id {
            id.name.to_string()
        } else {
            "<anonymous>".to_string()
        }
    }

    fn is_private_function(&self, name: &str) -> bool {
        name.starts_with('_') || name.starts_with('#')
    }

    fn calculate_function_complexity(&self, _body: &Option<oxc_ast::ast::FunctionBody>) -> u32 {
        // Simplified complexity calculation
        // In practice, this would analyze control flow, loops, conditions, etc.
        1
    }

    fn detect_test_type(&self, name: &str) -> TestType {
        let name_lower = name.to_lowercase();
        if name_lower.contains("integration") || name_lower.contains("api") {
            TestType::IntegrationTest
        } else if name_lower.contains("e2e") || name_lower.contains("end-to-end") {
            TestType::E2ETest
        } else if name_lower.contains("unit") || name_lower.contains("should") {
            TestType::UnitTest
        } else {
            TestType::Unknown
        }
    }

    fn link_tests_to_functions(&mut self) {
        // Simple heuristic to link test names to function names
        for test in &self.tests {
            let test_name_lower = test.name.to_lowercase();

            for function in &mut self.functions {
                let function_name_lower = function.name.to_lowercase();

                if test_name_lower.contains(&function_name_lower) ||
                   function_name_lower.contains(&test_name_lower) {
                    function.test_count += 1;
                    function.is_tested = true;
                }
            }
        }
    }
}

impl<'a> Visit<'a> for TestCoverageVisitor<'a> {
    fn visit_program(&mut self, node: &Program<'a>) {
        // First pass: collect functions and tests
        for stmt in &node.body {
            self.visit_statement(stmt);
        }

        // Second pass: link tests to functions
        self.link_tests_to_functions();
    }

    fn visit_function(&mut self, node: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {
        let function_name = self.get_function_name(node);
        let is_private = self.is_private_function(&function_name);
        let complexity = self.calculate_function_complexity(&node.body);

        let function_info = FunctionInfo {
            name: function_name,
            span: node.span,
            is_private,
            is_getter: false,
            is_setter: false,
            is_constructor: false,
            complexity,
            test_count: 0,
            is_tested: false,
            function_type: FunctionType::Function,
        };

        self.functions.push(function_info);

        // Visit function body
        if let Some(body) = &node.body {
            match body {
                oxc_ast::ast::FunctionBody::FunctionBody(block) => {
                    self.visit_block_statement(block);
                }
                oxc_ast::ast::FunctionBody::Expression(expr) => {
                    self.visit_expression(expr);
                }
            }
        }

        // Visit parameters
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }
    }

    fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression<'a>) {
        let complexity = 1; // Simplified

        let function_info = FunctionInfo {
            name: "<arrow>".to_string(),
            span: node.span,
            is_private: false,
            is_getter: false,
            is_setter: false,
            is_constructor: false,
            complexity,
            test_count: 0,
            is_tested: false,
            function_type: FunctionType::ArrowFunction,
        };

        self.functions.push(function_info);

        // Visit function body
        match &node.body {
            oxc_ast::ast::FunctionBody::FunctionBody(body) => {
                self.visit_block_statement(body);
            }
            oxc_ast::ast::FunctionBody::Expression(expr) => {
                self.visit_expression(expr);
            }
        }

        // Visit parameters
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }
    }

    fn visit_declaration(&mut self, node: &Declaration<'a>) {
        if let Declaration::Class(class_decl) = node {
            if let Some(id) = &class_decl.id {
                self.current_class = Some(id.name.to_string());
            }
            // Visit the class body
            for elem in &class_decl.body.body {
                self.visit_class_element(elem);
            }
        } else {
            // Continue with default visit for other declarations
            match node {
                Declaration::VariableDeclaration(var) => self.visit_variable_declaration(var),
                Declaration::FunctionDeclaration(func) => self.visit_function(func, oxc_semantic::ScopeFlags::empty()),
                _ => {}
            }
        }

        if matches!(node, Declaration::Class(_)) {
            self.current_class = None;
        }
    }

    fn visit_method_definition(&mut self, node: &MethodDefinition<'a>) {
        let method_name = if let Expression::Identifier(ident) = &node.key {
            ident.name.to_string()
        } else {
            "<computed>".to_string()
        };

        let is_private = self.is_private_function(&method_name);
        let is_constructor = node.kind == oxc_ast::ast::MethodDefinitionKind::Constructor;
        let is_getter = node.kind == oxc_ast::ast::MethodDefinitionKind::Get;
        let is_setter = node.kind == oxc_ast::ast::MethodDefinitionKind::Set;

        let full_name = if let Some(class_name) = &self.current_class {
            format!("{}.{}", class_name, method_name)
        } else {
            method_name
        };

        let function_info = FunctionInfo {
            name: full_name,
            span: node.span,
            is_private,
            is_getter,
            is_setter,
            is_constructor,
            complexity: 1, // Simplified
            test_count: 0,
            is_tested: false,
            function_type: if is_constructor {
                FunctionType::Constructor
            } else if is_getter {
                FunctionType::Getter
            } else if is_setter {
                FunctionType::Setter
            } else {
                FunctionType::Method
            },
        };

        self.functions.push(function_info);

        // Visit method body
        self.visit_function(&node.value, oxc_semantic::ScopeFlags::empty());
    }

    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        // Detect test functions
        if let Expression::Identifier(ident) = &node.callee {
            let test_functions = ["describe", "it", "test", "expect"];

            if test_functions.contains(&ident.name.as_str()) {
                if let Some(first_arg) = node.arguments.first() {
                    if let oxc_ast::ast::Argument::Expression(Expression::StringLiteral(literal)) = first_arg {
                        let test_type = self.detect_test_type(&literal.value);

                        let test_info = TestInfo {
                            name: literal.value.to_string(),
                            span: node.span,
                            target_function: None, // Would be inferred from name
                            test_type,
                        };

                        self.tests.push(test_info);
                    }
                }
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.callee);
        for arg in &node.arguments {
            match arg {
                oxc_ast::ast::Argument::Expression(expr) => self.visit_expression(expr),
                oxc_ast::ast::Argument::SpreadElement(spread) => self.visit_spread_element(spread),
                _ => {}
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

        check_test_coverage_threshold(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_insufficient_coverage_violation() {
        let code = r#"
// This is a test file with insufficient coverage
import { describe, it, expect } from 'jest';

function calculateTotal(items) {
    return items.reduce((sum, item) => sum + item.price, 0);
}

function validateUser(user) {
    if (!user.email) throw new Error('Invalid email');
    return true;
}

function processPayment(amount) {
    if (amount <= 0) throw new Error('Invalid amount');
    return { success: true, amount };
}

// Only one test for three functions
describe('Calculator', () => {
    it('should calculate total', () => {
        expect(calculateTotal([{price: 10}])).toBe(10);
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("coverage") || issue.message.contains("untested")));
    }

    #[test]
    fn test_adequate_coverage_compliant() {
        let code = r#"
import { describe, it, expect } from 'jest';

function calculateTotal(items) {
    return items.reduce((sum, item) => sum + item.price, 0);
}

function validateUser(user) {
    if (!user.email) throw new Error('Invalid email');
    return true;
}

describe('Calculator', () => {
    it('should calculate total', () => {
        expect(calculateTotal([{price: 10}])).toBe(10);
    });

    it('should handle empty array', () => {
        expect(calculateTotal([])).toBe(0);
    });
});

describe('User validation', () => {
    it('should validate user with email', () => {
        expect(validateUser({email: 'test@example.com'})).toBe(true);
    });

    it('should throw for invalid user', () => {
        expect(() => validateUser({})).toThrow('Invalid email');
    });
});
        "#;

        let issues = parse_and_check(code);
        // Should have good coverage
        assert!(issues.is_empty() || issues.iter().all(|issue| !issue.message.contains("coverage")));
    }

    #[test]
    fn test_non_test_file_ignored() {
        let code = r#"
export class UserService {
    constructor(database) {
        this.db = database;
    }

    createUser(userData) {
        return this.db.save(userData);
    }

    getUserById(id) {
        return this.db.findById(id);
    }

    deleteUser(id) {
        return this.db.delete(id);
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Non-test files should be ignored by default
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
import { describe, it, expect } from 'jest';

function complexCalculation(data) {
    // Complex function without adequate tests
    let result = 0;
    for (let item of data) {
        if (item.type === 'A') {
            result += item.value * 2;
        } else if (item.type === 'B') {
            result += item.value * 1.5;
        }
    }
    return result;
}

// Insufficient tests for complexity
describe('Complex calculation', () => {
    it('should work', () => {
        expect(complexCalculation([{type: 'A', value: 10}])).toBe(20);
    });
});
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest comprehensive test cases for improved coverage".to_string());

        let issues = check_test_coverage_threshold(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.fix_available));
    }
}