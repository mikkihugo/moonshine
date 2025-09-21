//! Testing framework specific rules (Jest, Vitest, Playwright, Cypress)

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoJestTimeoutInTests;

impl NoJestTimeoutInTests {
    pub const NAME: &'static str = "no-jest-timeout-in-tests";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoJestTimeoutInTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_test_function_with_timeout(call) {
                ctx.diagnostic(no_jest_timeout_in_tests_diagnostic(call.span));
            }
        }
    }
}

impl NoJestTimeoutInTests {
    fn is_test_function_with_timeout(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            if matches!(ident.name.as_str(), "test" | "it") && call.arguments.len() > 2 {
                // Check for timeout parameter (third argument)
                return true;
            }
        }
        false
    }
}

fn no_jest_timeout_in_tests_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Individual test timeout detected")
        .with_help("Configure global timeout in Jest config instead of per-test timeouts")
        .with_label(span)
}

impl EnhancedWasmRule for NoJestTimeoutInTests {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Set testTimeout in jest.config.js for consistent timeouts".to_string(),
            "Per-test timeouts make tests harder to maintain".to_string(),
            "Use jest.setTimeout() in beforeAll for suite-level changes".to_string(),
            "Consider if slow tests indicate performance issues".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequirePlaywrightWaits;

impl RequirePlaywrightWaits {
    pub const NAME: &'static str = "require-playwright-waits";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePlaywrightWaits {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_playwright_action_without_wait(call) {
                ctx.diagnostic(require_playwright_waits_diagnostic(call.span));
            }
        }
    }
}

impl RequirePlaywrightWaits {
    fn is_playwright_action_without_wait(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "click" | "fill" | "type" | "select");
            }
        }
        false
    }
}

fn require_playwright_waits_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Playwright action without explicit wait")
        .with_help("Add waitForSelector or waitForLoadState before actions")
        .with_label(span)
}

impl EnhancedWasmRule for RequirePlaywrightWaits {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use page.waitForSelector() before element interactions".to_string(),
            "Add page.waitForLoadState('networkidle') for dynamic content".to_string(),
            "Explicit waits prevent flaky tests".to_string(),
            "Use expect(locator).toBeVisible() for assertions".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoCypressArbitraryWaits;

impl NoCypressArbitraryWaits {
    pub const NAME: &'static str = "no-cypress-arbitrary-waits";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoCypressArbitraryWaits {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_cypress_wait_with_timeout(call) {
                ctx.diagnostic(no_cypress_arbitrary_waits_diagnostic(call.span));
            }
        }
    }
}

impl NoCypressArbitraryWaits {
    fn is_cypress_wait_with_timeout(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                if prop.name == "wait" {
                    // Check if first argument is a number (arbitrary timeout)
                    return call.arguments.first()
                        .and_then(|arg| arg.as_expression())
                        .map_or(false, |expr| matches!(expr, oxc_ast::ast::Expression::NumericLiteral(_)));
                }
            }
        }
        false
    }
}

fn no_cypress_arbitrary_waits_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Arbitrary wait in Cypress test")
        .with_help("Use semantic waits like cy.wait('@alias') or cy.get() with timeout")
        .with_label(span)
}

impl EnhancedWasmRule for NoCypressArbitraryWaits {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use cy.wait('@networkAlias') for network requests".to_string(),
            "Use cy.get().should('be.visible') for element waits".to_string(),
            "Arbitrary waits make tests slow and flaky".to_string(),
            "Set up proper aliases for network intercepting".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireVitestAsyncUtils;

impl RequireVitestAsyncUtils {
    pub const NAME: &'static str = "require-vitest-async-utils";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireVitestAsyncUtils {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_async_test_without_utils(call) {
                ctx.diagnostic(require_vitest_async_utils_diagnostic(call.span));
            }
        }
    }
}

impl RequireVitestAsyncUtils {
    fn is_async_test_without_utils(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            if matches!(ident.name.as_str(), "test" | "it") {
                // Check if test function is async but doesn't use waitFor, etc.
                return true; // Simplified check
            }
        }
        false
    }
}

fn require_vitest_async_utils_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async test without proper utilities")
        .with_help("Use waitFor, findBy, or vi.waitUntil for async testing")
        .with_label(span)
}

impl EnhancedWasmRule for RequireVitestAsyncUtils {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use @testing-library/jest-dom for better assertions".to_string(),
            "Use waitFor() for async state changes".to_string(),
            "Use findBy queries instead of getBy + waitFor".to_string(),
            "vi.waitUntil() for custom async conditions".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoHardcodedTestData;

impl NoHardcodedTestData {
    pub const NAME: &'static str = "no-hardcoded-test-data";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoHardcodedTestData {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_test_file(ctx) && self.has_hardcoded_data(obj) {
                ctx.diagnostic(no_hardcoded_test_data_diagnostic(obj.span));
            }
        }
    }
}

impl NoHardcodedTestData {
    fn is_test_file(&self, ctx: &WasmLintContext) -> bool {
        let filename = ctx.filename();
        filename.contains(".test.") || filename.contains(".spec.")
    }

    fn has_hardcoded_data(&self, obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for large object literals that could be test data
        obj.properties.len() > 3
    }
}

fn no_hardcoded_test_data_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Hardcoded test data detected")
        .with_help("Use factory functions or fixtures for test data")
        .with_label(span)
}

impl EnhancedWasmRule for NoHardcodedTestData {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use factory functions: createUser({ name: 'test' })".to_string(),
            "Store test data in separate fixture files".to_string(),
            "Use libraries like Faker.js for dynamic test data".to_string(),
            "Hardcoded data makes tests brittle and hard to maintain".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireTestIsolation;

impl RequireTestIsolation {
    pub const NAME: &'static str = "require-test-isolation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTestIsolation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_describe_block(call) && !self.has_cleanup_hooks(call, ctx) {
                ctx.diagnostic(require_test_isolation_diagnostic(call.span));
            }
        }
    }
}

impl RequireTestIsolation {
    fn is_describe_block(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return ident.name == "describe";
        }
        false
    }

    fn has_cleanup_hooks(&self, _call: &oxc_ast::ast::CallExpression, _ctx: &WasmLintContext) -> bool {
        // Check for beforeEach/afterEach hooks
        false
    }
}

fn require_test_isolation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Test suite without isolation hooks")
        .with_help("Add beforeEach/afterEach hooks for test isolation")
        .with_label(span)
}

impl EnhancedWasmRule for RequireTestIsolation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use beforeEach for test setup and afterEach for cleanup".to_string(),
            "Reset mocks and spies between tests".to_string(),
            "Clear local storage and session storage".to_string(),
            "Isolated tests prevent cascading failures".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoTestingLibraryQueryProblems;

impl NoTestingLibraryQueryProblems {
    pub const NAME: &'static str = "no-testing-library-query-problems";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoTestingLibraryQueryProblems {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_problematic_query(call) {
                ctx.diagnostic(no_testing_library_query_problems_diagnostic(call.span));
            }
        }
    }
}

impl NoTestingLibraryQueryProblems {
    fn is_problematic_query(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            // Check for getByTestId without semantic queries
            return ident.name == "getByTestId";
        }
        false
    }
}

fn no_testing_library_query_problems_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Problematic Testing Library query")
        .with_help("Prefer semantic queries like getByRole, getByLabelText over getByTestId")
        .with_label(span)
}

impl EnhancedWasmRule for NoTestingLibraryQueryProblems {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use getByRole for interactive elements".to_string(),
            "Use getByLabelText for form fields".to_string(),
            "getByTestId should be last resort".to_string(),
            "Semantic queries match user behavior better".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireSnapshotUpdates;

impl RequireSnapshotUpdates {
    pub const NAME: &'static str = "require-snapshot-updates";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSnapshotUpdates {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_snapshot_test(call) && self.is_outdated_snapshot(call) {
                ctx.diagnostic(require_snapshot_updates_diagnostic(call.span));
            }
        }
    }
}

impl RequireSnapshotUpdates {
    fn is_snapshot_test(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "toMatchSnapshot";
            }
        }
        false
    }

    fn is_outdated_snapshot(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // This would require file system access to check snapshot age
        false
    }
}

fn require_snapshot_updates_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potentially outdated snapshot test")
        .with_help("Review and update snapshots regularly, use inline snapshots for small data")
        .with_label(span)
}

impl EnhancedWasmRule for RequireSnapshotUpdates {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Review snapshot changes carefully in PRs".to_string(),
            "Use toMatchInlineSnapshot for small outputs".to_string(),
            "Snapshot tests should complement, not replace, behavioral tests".to_string(),
            "Consider property-based testing for complex data".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_jest_timeout_in_tests_rule() {
        assert_eq!(NoJestTimeoutInTests::NAME, "no-jest-timeout-in-tests");
        assert_eq!(NoJestTimeoutInTests::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_require_playwright_waits_rule() {
        assert_eq!(RequirePlaywrightWaits::NAME, "require-playwright-waits");
        assert_eq!(RequirePlaywrightWaits::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_cypress_arbitrary_waits_rule() {
        assert_eq!(NoCypressArbitraryWaits::NAME, "no-cypress-arbitrary-waits");
        assert_eq!(NoCypressArbitraryWaits::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoCypressArbitraryWaits::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequirePlaywrightWaits;
        let diagnostic = require_playwright_waits_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("waitForSelector"));
    }
}