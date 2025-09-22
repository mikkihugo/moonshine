//! Testing framework rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireTestDescription;

impl RequireTestDescription {
    pub const NAME: &'static str = "require-test-description";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTestDescription {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_test_function(call) {
                if let Some(desc) = self.get_test_description(call) {
                    if desc.len() < 10 {
                        ctx.diagnostic(require_test_description_diagnostic(call.span));
                    }
                }
            }
        }
    }
}

impl RequireTestDescription {
    fn is_test_function(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            matches!(ident.name.as_str(), "test" | "it" | "describe")
        } else {
            false
        }
    }

    fn get_test_description(&self, call: &oxc_ast::ast::CallExpression) -> Option<String> {
        call.arguments.first()?.as_expression()?
            .as_string_literal()
            .map(|lit| lit.value.to_string())
    }
}

fn require_test_description_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Test description too short")
        .with_help("Provide descriptive test names that explain the expected behavior")
        .with_label(span)
}

impl EnhancedWasmRule for RequireTestDescription {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Given-When-Then format for test descriptions".to_string(),
            "Describe the expected behavior, not implementation".to_string(),
            "Include edge cases in test descriptions".to_string(),
            "Good tests serve as living documentation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoFocusedTests;

impl NoFocusedTests {
    pub const NAME: &'static str = "no-focused-tests";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoFocusedTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_focused_test(call) {
                ctx.diagnostic(no_focused_tests_diagnostic(call.span));
            }
        }
    }
}

impl NoFocusedTests {
    fn is_focused_test(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "only";
            }
        }

        if let Some(ident) = call.callee.as_identifier() {
            matches!(ident.name.as_str(), "fdescribe" | "fit")
        } else {
            false
        }
    }
}

fn no_focused_tests_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Focused test detected")
        .with_help("Remove .only() or focused test functions before committing")
        .with_label(span)
}

impl EnhancedWasmRule for NoFocusedTests {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Focused tests prevent other tests from running".to_string(),
            "Use pre-commit hooks to catch focused tests".to_string(),
            "Consider test filtering instead of .only()".to_string(),
            "Focused tests can cause CI failures".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSkippedTests;

impl NoSkippedTests {
    pub const NAME: &'static str = "no-skipped-tests";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSkippedTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_skipped_test(call) {
                ctx.diagnostic(no_skipped_tests_diagnostic(call.span));
            }
        }
    }
}

impl NoSkippedTests {
    fn is_skipped_test(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "skip";
            }
        }

        if let Some(ident) = call.callee.as_identifier() {
            matches!(ident.name.as_str(), "xdescribe" | "xit")
        } else {
            false
        }
    }
}

fn no_skipped_tests_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Skipped test detected")
        .with_help("Fix or remove skipped tests to maintain test coverage")
        .with_label(span)
}

impl EnhancedWasmRule for NoSkippedTests {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Skipped tests reduce test coverage".to_string(),
            "Add TODO comments explaining why test is skipped".to_string(),
            "Create issues for skipped tests".to_string(),
            "Skipped tests often become forgotten technical debt".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireTestAssertions;

impl RequireTestAssertions {
    pub const NAME: &'static str = "require-test-assertions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTestAssertions {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_test_function(call) && !self.has_assertions(call) {
                ctx.diagnostic(require_test_assertions_diagnostic(call.span));
            }
        }
    }
}

impl RequireTestAssertions {
    fn is_test_function(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            matches!(ident.name.as_str(), "test" | "it")
        } else {
            false
        }
    }

    fn has_assertions(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if test function body contains assertions
        // Simplified implementation
        false
    }
}

fn require_test_assertions_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Test without assertions")
        .with_help("Add expect() assertions to verify test behavior")
        .with_label(span)
}

impl EnhancedWasmRule for RequireTestAssertions {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Tests without assertions don't verify behavior".to_string(),
            "Use expect().toBe() for equality checks".to_string(),
            "Add multiple assertions for complex behavior".to_string(),
            "Consider using custom matchers for clarity".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLargeTestFiles;

impl NoLargeTestFiles {
    pub const NAME: &'static str = "no-large-test-files";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoLargeTestFiles {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(program) = node.kind() {
            if self.is_test_file(&ctx.filename()) && program.body.len() > 50 {
                ctx.diagnostic(no_large_test_files_diagnostic(program.span));
            }
        }
    }
}

impl NoLargeTestFiles {
    fn is_test_file(&self, filename: &str) -> bool {
        filename.contains(".test.") || filename.contains(".spec.")
    }
}

fn no_large_test_files_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Large test file detected")
        .with_help("Split large test files into smaller, focused test suites")
        .with_label(span)
}

impl EnhancedWasmRule for NoLargeTestFiles {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Group related tests into separate files".to_string(),
            "Use describe blocks to organize tests".to_string(),
            "Consider test utilities for common setup".to_string(),
            "Large test files are harder to maintain".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferTestDataBuilders;

impl PreferTestDataBuilders {
    pub const NAME: &'static str = "prefer-test-data-builders";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferTestDataBuilders {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_test_data_object(obj) && obj.properties.len() > 5 {
                ctx.diagnostic(prefer_test_data_builders_diagnostic(obj.span));
            }
        }
    }
}

impl PreferTestDataBuilders {
    fn is_test_data_object(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check if this looks like test data
        true
    }
}

fn prefer_test_data_builders_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Complex test data object")
        .with_help("Use test data builders for complex test objects")
        .with_label(span)
}

impl EnhancedWasmRule for PreferTestDataBuilders {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Test data builders improve test readability".to_string(),
            "Use factory functions for test data creation".to_string(),
            "Consider libraries like factory-bot".to_string(),
            "Builders make tests more maintainable".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireMockCleanup;

impl RequireMockCleanup {
    pub const NAME: &'static str = "require-mock-cleanup";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMockCleanup {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_mock_call(call) && !self.has_cleanup_in_scope(ctx) {
                ctx.diagnostic(require_mock_cleanup_diagnostic(call.span));
            }
        }
    }
}

impl RequireMockCleanup {
    fn is_mock_call(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "mock" | "mockImplementation" | "mockReturnValue");
            }
        }
        false
    }

    fn has_cleanup_in_scope(&self, _ctx: &WasmLintContext) -> bool {
        // Check for afterEach or similar cleanup
        false
    }
}

fn require_mock_cleanup_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Mock without cleanup")
        .with_help("Add afterEach() to restore mocks between tests")
        .with_label(span)
}

impl EnhancedWasmRule for RequireMockCleanup {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use jest.restoreAllMocks() in afterEach".to_string(),
            "Mocks can leak between tests".to_string(),
            "Consider jest.resetAllMocks() for stateful mocks".to_string(),
            "Clean tests prevent flaky test behavior".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_test_description_rule() {
        assert_eq!(RequireTestDescription::NAME, "require-test-description");
        assert_eq!(RequireTestDescription::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_no_focused_tests_rule() {
        assert_eq!(NoFocusedTests::NAME, "no-focused-tests");
        assert_eq!(NoFocusedTests::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoFocusedTests::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_test_file_detection() {
        let rule = NoLargeTestFiles;
        assert!(rule.is_test_file("component.test.ts"));
        assert!(rule.is_test_file("utils.spec.js"));
        assert!(!rule.is_test_file("component.ts"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireTestAssertions;
        let diagnostic = require_test_assertions_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("assertions"));
    }
}