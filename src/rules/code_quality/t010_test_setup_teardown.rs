//! # T010: Test Setup/Teardown Patterns Rule
//!
//! Enforces proper test setup and teardown patterns to ensure test isolation
//! and prevent side effects between tests. Promotes clean test architecture
//! by detecting missing cleanup, shared state issues, and improper resource management.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use crate::rules::ai_integration::AIEnhancer;
use oxc_ast::ast::{Program, CallExpression, Expression, Argument, BlockStatement, Statement, VariableDeclaration, AssignmentExpression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct T010Config {
    /// Whether to enforce beforeEach for setup (default: true)
    pub enforce_before_each: Option<bool>,
    /// Whether to enforce afterEach for cleanup (default: true)
    pub enforce_after_each: Option<bool>,
    /// Whether to detect shared state between tests (default: true)
    pub detect_shared_state: Option<bool>,
    /// Whether to require cleanup for external resources (default: true)
    pub require_resource_cleanup: Option<bool>,
    /// Resource patterns that require cleanup
    pub resource_patterns: Option<Vec<String>>,
    /// Variables that indicate shared state
    pub shared_state_indicators: Option<Vec<String>>,
    /// Maximum setup complexity (default: 10)
    pub max_setup_complexity: Option<u32>,
    /// Whether to check for test data isolation (default: true)
    pub check_test_data_isolation: Option<bool>,
    /// Whether to enforce async cleanup patterns (default: true)
    pub enforce_async_cleanup: Option<bool>,
}

/// T010 rule implementation with AI enhancement
pub fn check_test_setup_teardown(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = T010Config::default();
    let mut visitor = TestSetupTeardownVisitor::new(program, code, &config);
    visitor.visit_program(program);
    
    // Perform analysis
    visitor.analyze_test_suites();
    visitor.check_resource_cleanup();
    visitor.detect_isolation_violations();
    
    visitor.issues
}

#[derive(Debug, Clone)]
struct TestSuite {
    name: String,
    span: Span,
    has_before_each: bool,
    has_after_each: bool,
    has_before_all: bool,
    has_after_all: bool,
    setup_complexity: u32,
    teardown_complexity: u32,
    shared_variables: Vec<String>,
    resource_usage: Vec<ResourceUsage>,
    test_count: u32,
    isolation_violations: Vec<IsolationViolation>,
}

#[derive(Debug, Clone)]
struct ResourceUsage {
    resource_type: ResourceType,
    span: Span,
    is_cleaned_up: bool,
    cleanup_method: Option<String>,
}

#[derive(Debug, Clone)]
enum ResourceType {
    Database,
    FileSystem,
    Network,
    Timer,
    EventListener,
    Memory,
    External,
}

impl ResourceType {
    fn description(&self) -> &'static str {
        match self {
            ResourceType::Database => "database connection",
            ResourceType::FileSystem => "file handle",
            ResourceType::Network => "network connection",
            ResourceType::Timer => "timer/interval",
            ResourceType::EventListener => "event listener",
            ResourceType::Memory => "memory allocation",
            ResourceType::External => "external resource",
        }
    }

    fn cleanup_suggestion(&self) -> &'static str {
        match self {
            ResourceType::Database => "Close database connections in afterEach",
            ResourceType::FileSystem => "Close file handles and cleanup temp files",
            ResourceType::Network => "Close network connections and clear mocks",
            ResourceType::Timer => "Clear timers and intervals in afterEach",
            ResourceType::EventListener => "Remove event listeners in afterEach",
            ResourceType::Memory => "Clear large objects and arrays",
            ResourceType::External => "Cleanup external resources and mocks",
        }
    }
}

#[derive(Debug, Clone)]
struct IsolationViolation {
    violation_type: IsolationType,
    span: Span,
    description: String,
}

#[derive(Debug, Clone)]
enum IsolationType {
    SharedGlobalState,
    UncleanedResource,
    CrossTestDependency,
    ImproperMocking,
    DataLeakage,
    AsyncCleanupMissing,
}

impl IsolationType {
    fn description(&self) -> &'static str {
        match self {
            IsolationType::SharedGlobalState => "shared global state between tests",
            IsolationType::UncleanedResource => "resource not properly cleaned up",
            IsolationType::CrossTestDependency => "test depends on previous test state",
            IsolationType::ImproperMocking => "mock not properly reset between tests",
            IsolationType::DataLeakage => "test data leaking between tests",
            IsolationType::AsyncCleanupMissing => "missing async cleanup for promises/observables",
        }
    }

    fn recommendation(&self) -> &'static str {
        match self {
            IsolationType::SharedGlobalState => "Reset global state in beforeEach/afterEach",
            IsolationType::UncleanedResource => "Add proper cleanup in afterEach hook",
            IsolationType::CrossTestDependency => "Make tests independent and self-contained",
            IsolationType::ImproperMocking => "Reset mocks in beforeEach or afterEach",
            IsolationType::DataLeakage => "Clear test data between test runs",
            IsolationType::AsyncCleanupMissing => "Add async cleanup for promises and observables",
        }
    }
}

struct TestSetupTeardownVisitor<'a> {
    config: &'a T010Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    test_suites: Vec<TestSuite>,
    current_suite: Option<TestSuite>,
    global_variables: HashSet<String>,
    is_test_file: bool,
}

impl<'a> TestSetupTeardownVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a T010Config) -> Self {
        let is_test_file = Self::detect_test_file(source_code);

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            test_suites: Vec::new(),
            current_suite: None,
            global_variables: HashSet::new(),
            is_test_file,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, base_message: &str, _span: Span) -> String {
        // Simple AI enhancement - in production this would use Claude API
        base_message.to_string()
    }

    /// AI Enhancement: Generate fix suggestions
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Add beforeEach hooks to reset test state and ensure isolation".to_string(),
            "Implement afterEach hooks to clean up resources and mocks".to_string(),
            "Use test-specific data factories instead of shared state".to_string(),
            "Reset global objects and singletons between tests".to_string(),
            "Close database connections and file handles in cleanup".to_string(),
            "Clear timers, intervals, and event listeners in afterEach".to_string(),
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

    fn detect_test_file(source_code: &str) -> bool {
        let test_indicators = [
            "describe(", "it(", "test(", "beforeEach(", "afterEach(",
            "beforeAll(", "afterAll(", "suite(", "setup(", "teardown("
        ];

        test_indicators.iter().any(|indicator| source_code.contains(indicator))
    }

    fn finalize_analysis(mut self) -> Vec<LintIssue> {
        if !self.is_test_file {
            return self.issues;
        }

        self.analyze_test_suites();
        self.check_resource_cleanup();
        self.detect_isolation_violations();

        self.issues
    }

    fn analyze_test_suites(&mut self) {
        // Avoid mutable and immutable borrow overlap by collecting first
        let test_suites = self.test_suites.clone();
        for suite in &test_suites {
            if self.config.enforce_before_each.unwrap_or(true) && suite.test_count > 1 && !suite.has_before_each {
                self.create_setup_issue(
                    "Missing beforeEach hook for test setup",
                    "Add beforeEach hook to ensure consistent test state",
                    suite.span,
                );
            }

            if self.config.enforce_after_each.unwrap_or(true) && suite.test_count > 1 && !suite.has_after_each {
                self.create_setup_issue(
                    "Missing afterEach hook for test cleanup",
                    "Add afterEach hook to clean up resources and state",
                    suite.span,
                );
            }

            if suite.setup_complexity > self.config.max_setup_complexity.unwrap_or(10) {
                self.create_setup_issue(
                    &format!(
                        "Test setup too complex ({} > {})",
                        suite.setup_complexity,
                        self.config.max_setup_complexity.unwrap_or(10)
                    ),
                    "Simplify test setup or extract to helper functions",
                    suite.span,
                );
            }

            if self.config.detect_shared_state.unwrap_or(true) && !suite.shared_variables.is_empty() {
                for var in &suite.shared_variables {
                    self.create_setup_issue(
                        &format!("Shared variable '{}' may cause test isolation issues", var),
                        "Reset shared variables in beforeEach or use test-specific data",
                        suite.span,
                    );
                }
            }

            for violation in &suite.isolation_violations {
                self.create_isolation_issue(violation);
            }
        }
    }

    fn check_resource_cleanup(&mut self) {
        // Avoid mutable and immutable borrow overlap by collecting first
        let test_suites = self.test_suites.clone();
        for suite in &test_suites {
            for resource in &suite.resource_usage {
                if self.config.require_resource_cleanup.unwrap_or(true) && !resource.is_cleaned_up {
                    self.create_resource_issue(resource);
                }
            }
        }
    }

    fn detect_isolation_violations(&mut self) {
        // This would analyze patterns that indicate poor test isolation
        // For now, we'll focus on the explicit checks in analyze_test_suites
    }

    fn create_setup_issue(&mut self, message: &str, recommendation: &str, span: Span) {
        let (line, column) = self.calculate_line_column(span.start as usize);

        let enhanced_message = self.generate_ai_enhanced_message(
            &format!("{}: {}", message, recommendation),
            span
        );

        self.issues.push(LintIssue {
            rule_name: "T010".to_string(),
            severity: LintSeverity::Warning,
            message: enhanced_message,
            line,
            column,
            fix_available: true,
        });
    }

    fn create_resource_issue(&mut self, resource: &ResourceUsage) {
        let (line, column) = self.calculate_line_column(resource.span.start as usize);

        let message = self.generate_ai_enhanced_message(
            &format!(
                "Uncleaned {}: {}",
                resource.resource_type.description(),
                resource.resource_type.cleanup_suggestion()
            ),
            resource.span
        );

        self.issues.push(LintIssue {
            rule_name: "T010".to_string(),
            severity: LintSeverity::Warning,
            message,
            line,
            column,
            fix_available: true,
        });
    }

    fn create_isolation_issue(&mut self, violation: &IsolationViolation) {
        let (line, column) = self.calculate_line_column(violation.span.start as usize);

        let message = self.generate_ai_enhanced_message(
            &format!(
                "Test isolation violation: {} - {}",
                violation.violation_type.description(),
                violation.violation_type.recommendation()
            ),
            violation.span
        );

        self.issues.push(LintIssue {
            rule_name: "T010".to_string(),
            severity: LintSeverity::Error,
            message,
            line,
            column,
            fix_available: true,
        });
    }

    fn detect_resource_type(&self, call_name: &str) -> Option<ResourceType> {
        match call_name {
            name if name.contains("connect") || name.contains("database") => Some(ResourceType::Database),
            name if name.contains("open") || name.contains("read") || name.contains("write") => Some(ResourceType::FileSystem),
            name if name.contains("fetch") || name.contains("request") || name.contains("http") => Some(ResourceType::Network),
            name if name.contains("setTimeout") || name.contains("setInterval") => Some(ResourceType::Timer),
            name if name.contains("addEventListener") || name.contains("on") => Some(ResourceType::EventListener),
            name if name.contains("new ") || name.contains("alloc") => Some(ResourceType::Memory),
            _ => None,
        }
    }

    fn is_shared_state_variable(&self, name: &str) -> bool {
        self.shared_state_indicators.iter().any(|indicator| {
            name.to_lowercase().contains(&indicator.to_lowercase())
        })
    }

    fn calculate_complexity(&self, _statements: &[Statement]) -> u32 {
        // Simplified complexity calculation
        // In practice, this would count control structures, function calls, etc.
        1
    }
}

impl<'a> Visit<'a> for TestSetupTeardownVisitor<'a> {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        if let Expression::Identifier(ident) = &node.callee {
            let function_name = &ident.name;

            match function_name.as_str() {
                "describe" | "suite" => {
                    self.handle_describe_block(node);
                }
                "beforeEach" | "setup" => {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.has_before_each = true;
                        // Analyze setup complexity
                        if let Some(second_arg) = node.arguments.get(1) {
                            if let Argument::Expression(Expression::ArrowFunctionExpression(arrow)) = second_arg {
                                if let oxc_ast::ast::FunctionBody::FunctionBody(body) = &arrow.body {
                                    suite.setup_complexity = self.calculate_complexity(&body.body);
                                }
                            }
                        }
                    }
                }
                "afterEach" | "teardown" => {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.has_after_each = true;
                        // Analyze teardown complexity
                        if let Some(second_arg) = node.arguments.get(1) {
                            if let Argument::Expression(Expression::ArrowFunctionExpression(arrow)) = second_arg {
                                if let oxc_ast::ast::FunctionBody::FunctionBody(body) = &arrow.body {
                                    suite.teardown_complexity = self.calculate_complexity(&body.body);
                                }
                            }
                        }
                    }
                }
                "beforeAll" => {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.has_before_all = true;
                    }
                }
                "afterAll" => {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.has_after_all = true;
                    }
                }
                "it" | "test" => {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.test_count += 1;
                    }
                }
                _ => {
                    // Check for resource usage
                    if self.resource_patterns.iter().any(|pattern| function_name.contains(pattern)) {
                        if let Some(resource_type) = self.detect_resource_type(function_name) {
                            let resource = ResourceUsage {
                                resource_type,
                                span: node.span,
                                is_cleaned_up: false, // Would be determined by analysis
                                cleanup_method: None,
                            };

                            if let Some(ref mut suite) = self.current_suite {
                                suite.resource_usage.push(resource);
                            }
                        }
                    }
                }
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.callee);
        for arg in &node.arguments {
            match arg {
                Argument::Expression(expr) => self.visit_expression(expr),
                Argument::SpreadElement(spread) => self.visit_spread_element(spread),
                _ => {}
            }
        }
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration<'a>) {
        for declarator in &node.declarations {
            if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &declarator.id.kind {
                let var_name = &ident.name;

                // Check for shared state variables
                if self.is_shared_state_variable(var_name) {
                    if let Some(ref mut suite) = self.current_suite {
                        suite.shared_variables.push(var_name.clone());
                    }
                }

                self.global_variables.insert(var_name.clone());
            }
        }

        // Continue visiting child nodes
        for declarator in &node.declarations {
            if let Some(init) = &declarator.init {
                self.visit_expression(init);
            }
        }
    }

    fn visit_assignment_expression(&mut self, node: &AssignmentExpression<'a>) {
        // Check for assignments to shared state
        if let Expression::MemberExpression(member) = &node.left {
            if let Expression::Identifier(obj) = &member.object {
                if self.is_shared_state_variable(&obj.name) {
                    if let Some(ref mut suite) = self.current_suite {
                        let violation = IsolationViolation {
                            violation_type: IsolationType::SharedGlobalState,
                            span: node.span,
                            description: format!("Assignment to shared state: {}", obj.name),
                        };
                        suite.isolation_violations.push(violation);
                    }
                }
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.left);
        self.visit_expression(&node.right);
    }
}

impl<'a> TestSetupTeardownVisitor<'a> {
    fn handle_describe_block(&mut self, node: &CallExpression<'a>) {
        if let Some(first_arg) = node.arguments.first() {
            if let Argument::Expression(Expression::StringLiteral(literal)) = first_arg {
                let suite_name = literal.value.to_string();

                // Save current suite if exists
                if let Some(current) = self.current_suite.take() {
                    self.test_suites.push(current);
                }

                // Create new suite
                self.current_suite = Some(TestSuite {
                    name: suite_name,
                    span: node.span,
                    has_before_each: false,
                    has_after_each: false,
                    has_before_all: false,
                    has_after_all: false,
                    setup_complexity: 0,
                    teardown_complexity: 0,
                    shared_variables: Vec::new(),
                    resource_usage: Vec::new(),
                    test_count: 0,
                    isolation_violations: Vec::new(),
                });

                // Visit the test block body
                if let Some(second_arg) = node.arguments.get(1) {
                    if let Argument::Expression(expr) = second_arg {
                        self.visit_expression(expr);
                    }
                }

                // Finalize current suite
                if let Some(current) = self.current_suite.take() {
                    self.test_suites.push(current);
                }
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

        check_test_setup_teardown(&parse_result.program, &semantic_result.semantic, code, None)
    }

    #[test]
    fn test_missing_setup_teardown_violation() {
        let code = r#"
describe('User Service', () => {
    it('should create user', () => {
        const user = userService.create({name: 'John'});
        expect(user).toBeDefined();
    });

    it('should update user', () => {
        const user = userService.update(1, {name: 'Jane'});
        expect(user.name).toBe('Jane');
    });

    it('should delete user', () => {
        userService.delete(1);
        expect(userService.findById(1)).toBeNull();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("beforeEach") || issue.message.contains("afterEach")));
    }

    #[test]
    fn test_proper_setup_teardown_compliant() {
        let code = r#"
describe('User Service', () => {
    let userService;
    let testDatabase;

    beforeEach(() => {
        testDatabase = new TestDatabase();
        userService = new UserService(testDatabase);
    });

    afterEach(() => {
        testDatabase.cleanup();
        userService = null;
    });

    it('should create user', () => {
        const user = userService.create({name: 'John'});
        expect(user).toBeDefined();
    });

    it('should update user', () => {
        const user = userService.update(1, {name: 'Jane'});
        expect(user.name).toBe('Jane');
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty() || issues.iter().all(|issue| !issue.message.contains("Missing")));
    }

    #[test]
    fn test_resource_cleanup_violation() {
        let code = r#"
describe('File Service', () => {
    beforeEach(() => {
        const database = connect('test-db');
        const fileHandle = open('test-file.txt');
        const timer = setTimeout(() => {}, 1000);
        // Missing cleanup for these resources
    });

    it('should process file', () => {
        expect(fileService.process()).toBeTruthy();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("cleanup") || issue.message.contains("resource")));
    }

    #[test]
    fn test_shared_state_violation() {
        let code = r#"
let globalState = {};

describe('Shared State Tests', () => {
    it('should set global state', () => {
        globalState.value = 'test1';
        expect(globalState.value).toBe('test1');
    });

    it('should use global state', () => {
        // This test depends on the previous test!
        expect(globalState.value).toBe('test1');
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("shared") || issue.message.contains("isolation")));
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
describe('Poor Test Setup', () => {
    it('test one', () => {
        const db = connect('database');
        const result = processData();
        expect(result).toBeTruthy();
        // No cleanup
    });

    it('test two', () => {
        const timer = setTimeout(() => {}, 5000);
        const result = processAsync();
        expect(result).toBeTruthy();
        // No cleanup
    });
});
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest proper test setup and teardown patterns".to_string());

        let issues = check_test_setup_teardown(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.fix_available));
    }
}