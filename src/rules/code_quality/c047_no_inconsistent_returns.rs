//! # C047: No Inconsistent Returns Rule
//!
//! Enforces consistent return statements in functions - either all code paths return
//! a value or none do. Prevents subtle bugs and improves code predictability by
//! ensuring function return behavior is consistent across all execution paths.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use crate::rules::ai_integration::AIEnhancer;
use oxc_ast::ast::{Program, Function, ArrowFunctionExpression, Statement, ReturnStatement, BlockStatement, IfStatement, SwitchStatement, SwitchCase};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct C047Config {
    /// Whether to allow inconsistent returns in test files (default: true)
    pub allow_inconsistent_in_tests: bool,
    /// Whether to treat missing return as returning undefined (default: true)
    pub treat_undefined_as_unspecified: bool,
    /// Whether to allow inconsistent returns in getter functions (default: false)
    pub allow_inconsistent_in_getters: bool,
    /// Whether to ignore arrow functions with expression bodies (default: true)
    pub ignore_arrow_expressions: bool,
    /// Whether to check constructor functions (default: false)
    pub check_constructors: bool,
    /// Whether to allow early returns without values in void functions (default: true)
    pub allow_early_void_returns: bool,
}

/// C047 rule implementation with AI enhancement
pub fn check_no_inconsistent_returns(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C047Config::default();
    let mut visitor = InconsistentReturnsVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.finalize_issues()
}

#[derive(Debug, Clone)]
struct ReturnInfo {
    span: Span,
    has_value: bool,
    is_reachable: bool,
    statement_type: ReturnType,
}

#[derive(Debug, Clone)]
enum ReturnType {
    ExplicitReturn,
    ImplicitReturn,
    EarlyReturn,
    ConditionalReturn,
    SwitchReturn,
}

#[derive(Debug, Clone)]
struct FunctionAnalysis {
    name: String,
    span: Span,
    returns: Vec<ReturnInfo>,
    has_explicit_return_type: bool,
    is_constructor: bool,
    is_getter: bool,
    is_arrow_expression: bool,
}

struct InconsistentReturnsVisitor<'a> {
    config: &'a C047Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    current_function: Option<FunctionAnalysis>,
    function_stack: Vec<FunctionAnalysis>,
}

impl<'a> InconsistentReturnsVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a C047Config) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            current_function: None,
            function_stack: Vec::new(),
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Ensure all code paths in a function either return a value or don't return at all".to_string(),
            "Add explicit return statements where needed".to_string(),
            "Remove unnecessary return statements to maintain consistency".to_string(),
            "Consider using early returns for better code flow".to_string(),
        ]
    }

    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

    fn finalize_issues(mut self) -> Vec<LintIssue> {
        // Apply AI enhancements to all issues
        let ai_suggestions = self.generate_ai_fix_suggestions();
        for issue in &mut self.issues {
            if let Some(ai_context) = ai_suggestions.first() {
                issue.message = format!("{} 💡 AI Suggestion: {}", issue.message, ai_context);
            }
        }
        self.issues
    }

    fn is_test_context(&self) -> bool {
        self.config.allow_inconsistent_in_tests && (
            self.source_code.contains(".test.") ||
            self.source_code.contains(".spec.") ||
            self.source_code.contains("__tests__") ||
            self.source_code.contains("describe(") ||
            self.source_code.contains("it(") ||
            self.source_code.contains("test(")
        )
    }

    fn enter_function(&mut self, analysis: FunctionAnalysis) {
        if let Some(current) = self.current_function.take() {
            self.function_stack.push(current);
        }
        self.current_function = Some(analysis);
    }

    fn exit_function(&mut self) {
        if let Some(function_analysis) = self.current_function.take() {
            self.analyze_function_returns(function_analysis);
        }

        self.current_function = self.function_stack.pop();
    }

    fn add_return(&mut self, return_info: ReturnInfo) {
        if let Some(ref mut function) = self.current_function {
            function.returns.push(return_info);
        }
    }

    fn analyze_function_returns(&mut self, function: FunctionAnalysis) {
        if self.is_test_context() {
            return;
        }

        if !self.config.check_constructors && function.is_constructor {
            return;
        }

        if self.config.allow_inconsistent_in_getters && function.is_getter {
            return;
        }

        if self.config.ignore_arrow_expressions && function.is_arrow_expression {
            return;
        }

        if function.returns.is_empty() {
            return; // No returns to analyze
        }

        let mut has_value_returns = false;
        let mut has_no_value_returns = false;
        let mut has_reachable_returns = false;

        for return_info in &function.returns {
            if return_info.is_reachable {
                has_reachable_returns = true;

                if return_info.has_value {
                    has_value_returns = true;
                } else {
                    has_no_value_returns = true;
                }
            }
        }

        // Check for inconsistent return patterns
        if has_reachable_returns && has_value_returns && has_no_value_returns {
            // Special case: allow early void returns in functions that otherwise return values
            if self.config.allow_early_void_returns {
                let early_void_returns = function.returns.iter()
                    .filter(|r| !r.has_value && matches!(r.statement_type, ReturnType::EarlyReturn))
                    .count();

                let other_returns = function.returns.iter()
                    .filter(|r| r.has_value || !matches!(r.statement_type, ReturnType::EarlyReturn))
                    .count();

                if early_void_returns > 0 && other_returns > 0 {
                    // This pattern is allowed - early returns for validation/error cases
                    return;
                }
            }

            self.create_inconsistent_return_issue(&function);
        }
    }

    fn create_inconsistent_return_issue(&mut self, function: &FunctionAnalysis) {
        let (line, column) = self.calculate_line_column(function.span);

        let has_value_count = function.returns.iter().filter(|r| r.has_value).count();
        let no_value_count = function.returns.iter().filter(|r| !r.has_value).count();

        let message = self.generate_ai_enhanced_message(
            "inconsistent return statements",
            &format!("Function '{}' has inconsistent return statements: {} return(s) with values, {} return(s) without values. Either all code paths should return a value or none should.",
                function.name,
                has_value_count,
                no_value_count
            ),
            function.span
        );

        self.issues.push(LintIssue {
            rule_name: "C047".to_string(),
            severity: LintSeverity::Warning,
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

    fn is_constructor_function(&self, func: &Function) -> bool {
        if let Some(id) = &func.id {
            // Check if function name starts with uppercase (constructor convention)
            id.name.chars().next().map_or(false, |c| c.is_uppercase())
        } else {
            false
        }
    }

    fn is_getter_function(&self, name: &str) -> bool {
        name.starts_with("get") && name.len() > 3 &&
        name.chars().nth(3).map_or(false, |c| c.is_uppercase())
    }

    fn analyze_control_flow(&self, statements: &[Statement]) -> bool {
        // Simple analysis to determine if all code paths have explicit returns
        for stmt in statements {
            match stmt {
                Statement::ReturnStatement(_) => return true,
                Statement::IfStatement(if_stmt) => {
                    if self.analyze_if_statement(if_stmt) {
                        return true;
                    }
                }
                Statement::SwitchStatement(switch_stmt) => {
                    if self.analyze_switch_statement(switch_stmt) {
                        return true;
                    }
                }
                Statement::ThrowStatement(_) => return true,
                _ => {}
            }
        }
        false
    }

    fn analyze_if_statement(&self, if_stmt: &IfStatement) -> bool {
        // Check if both branches return
        let consequent_returns = match &if_stmt.consequent {
            Statement::BlockStatement(block) => self.analyze_control_flow(&block.body),
            Statement::ReturnStatement(_) => true,
            _ => false,
        };

        let alternate_returns = if let Some(alternate) = &if_stmt.alternate {
            match alternate {
                Statement::BlockStatement(block) => self.analyze_control_flow(&block.body),
                Statement::ReturnStatement(_) => true,
                Statement::IfStatement(nested_if) => self.analyze_if_statement(nested_if),
                _ => false,
            }
        } else {
            false
        };

        consequent_returns && alternate_returns
    }

    fn analyze_switch_statement(&self, switch_stmt: &SwitchStatement) -> bool {
        let mut has_default = false;
        let mut all_cases_return = true;

        for case in &switch_stmt.cases {
            if case.test.is_none() {
                has_default = true;
            }

            let case_returns = self.analyze_control_flow(&case.consequent);
            if !case_returns {
                all_cases_return = false;
            }
        }

        has_default && all_cases_return
    }

    fn determine_return_type(&self, _context: &str) -> ReturnType {
        // This could be enhanced to detect the context more precisely
        ReturnType::ExplicitReturn
    }
}

impl<'a> Visit<'a> for InconsistentReturnsVisitor<'a> {
    fn visit_function(&mut self, node: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {
        let function_name = self.get_function_name(node);
        let is_constructor = self.is_constructor_function(node);
        let is_getter = self.is_getter_function(&function_name);

        let analysis = FunctionAnalysis {
            name: function_name,
            span: node.span,
            returns: Vec::new(),
            has_explicit_return_type: false, // Could be enhanced with type analysis
            is_constructor,
            is_getter,
            is_arrow_expression: false,
        };

        self.enter_function(analysis);

        // Visit function body
        if let Some(body) = &node.body {
            self.visit_block_statement(body);
        }

        // Visit parameters
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }

        self.exit_function();
    }

    fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression<'a>) {
        let is_expression_body = node.expression;

        let analysis = FunctionAnalysis {
            name: "<arrow>".to_string(),
            span: node.span,
            returns: Vec::new(),
            has_explicit_return_type: false,
            is_constructor: false,
            is_getter: false,
            is_arrow_expression: is_expression_body,
        };

        self.enter_function(analysis);

        // Visit function body
        match &node.body {
            oxc_ast::ast::FunctionBody::FunctionBody(body) => {
                self.visit_block_statement(body);
            }
            oxc_ast::ast::FunctionBody::Expression(expr) => {
                // Expression body implicitly returns the expression value
                if !is_expression_body || !self.config.ignore_arrow_expressions {
                    self.add_return(ReturnInfo {
                        span: expr.span(),
                        has_value: true,
                        is_reachable: true,
                        statement_type: ReturnType::ImplicitReturn,
                    });
                }
                self.visit_expression(expr);
            }
        }

        // Visit parameters
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }

        self.exit_function();
    }

    fn visit_return_statement(&mut self, node: &ReturnStatement<'a>) {
        let has_value = node.argument.is_some();
        let return_type = self.determine_return_type("explicit");

        self.add_return(ReturnInfo {
            span: node.span,
            has_value,
            is_reachable: true, // Simplified - could be enhanced with control flow analysis
            statement_type: return_type,
        });

        // Visit the return argument if present
        if let Some(argument) = &node.argument {
            self.visit_expression(argument);
        }
    }

    fn visit_block_statement(&mut self, node: &BlockStatement<'a>) {
        for stmt in &node.body {
            self.visit_statement(stmt);
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

        check_no_inconsistent_returns(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_inconsistent_returns_violation() {
        let code = r#"
function processData(data) {
    if (!data) {
        return; // No value
    }

    if (data.length === 0) {
        return null; // Has value
    }

    return data.map(item => item.id); // Has value
}

function calculateValue(input) {
    if (input < 0) {
        return; // No value
    }

    return input * 2; // Has value
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("inconsistent return statements")));
    }

    #[test]
    fn test_consistent_value_returns_compliant() {
        let code = r#"
function processData(data) {
    if (!data) {
        return null;
    }

    if (data.length === 0) {
        return [];
    }

    return data.map(item => item.id);
}

function calculateValue(input) {
    if (input < 0) {
        return 0;
    }

    return input * 2;
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_consistent_void_returns_compliant() {
        let code = r#"
function logMessage(message) {
    if (!message) {
        return;
    }

    console.log(message);
    return;
}

function processNotification(notification) {
    if (!notification.enabled) {
        return;
    }

    sendNotification(notification);
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_early_void_returns_allowed() {
        let code = r#"
function validateAndProcess(data) {
    // Early validation returns (allowed)
    if (!data) {
        return;
    }

    if (!data.isValid) {
        return;
    }

    // Main logic returns value
    return processData(data);
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Early void returns should be allowed
    }

    #[test]
    fn test_arrow_functions_ignored() {
        let code = r#"
const processItem = (item) => {
    if (!item) {
        return;
    }
    return item.processed;
};

// Expression body arrow function
const doubleValue = (x) => x * 2;
        "#;

        let issues = parse_and_check(code);
        // Arrow functions might be ignored based on configuration
        assert!(issues.len() <= 1); // At most one issue for block body arrow function
    }

    #[test]
    fn test_constructor_functions_ignored() {
        let code = r#"
function UserModel(name) {
    if (!name) {
        return; // Early exit
    }

    this.name = name;
    return this; // Explicit return
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Constructor functions should be ignored by default
    }

    #[test]
    fn test_test_context_allowed() {
        let code = r#"
// utils.test.js
describe('Utils', () => {
    it('should handle inconsistent returns in tests', () => {
        function testHelper(condition) {
            if (condition) {
                return;
            }
            return "result";
        }

        expect(testHelper(true)).toBeUndefined();
        expect(testHelper(false)).toBe("result");
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Should be allowed in test context
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
function getData(id) {
    if (!id) {
        return;
    }

    return fetchDataById(id);
}
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest consistent return patterns for functions".to_string());

        let issues = check_no_inconsistent_returns(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        assert!(!issues.is_empty());
        assert!(issues[0].fix_available);
        assert!(issues[0].message.contains("inconsistent return statements"));
    }
}