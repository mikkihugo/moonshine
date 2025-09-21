//! Testing framework integration rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireDescriptiveTestNames;

impl RequireDescriptiveTestNames {
    pub const NAME: &'static str = "require-descriptive-test-names";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDescriptiveTestNames {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(callee) = call.callee.as_identifier() {
                if matches!(callee.name.as_str(), "it" | "test" | "describe") {
                    if let Some(first_arg) = call.arguments.first() {
                        if let Some(string_lit) = first_arg.as_expression().and_then(|e| e.as_string_literal()) {
                            let test_name = &string_lit.value;
                            if test_name == "test" || test_name == "works" || test_name == "should work" {
                                ctx.diagnostic(require_descriptive_test_names_diagnostic(call.span));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn require_descriptive_test_names_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Test names should be descriptive")
        .with_help("Use descriptive test names that explain the expected behavior")
        .with_label(span)
}

impl EnhancedWasmRule for RequireDescriptiveTestNames {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use 'should' to describe expected behavior".to_string(),
            "Include context about what is being tested".to_string(),
            "Avoid vague names like 'test', 'works', or 'it works'".to_string(),
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
            if let Some(callee) = call.callee.as_identifier() {
                if matches!(callee.name.as_str(), "fit" | "fdescribe" | "xdescribe" | "xit") {
                    ctx.diagnostic(no_focused_tests_diagnostic(call.span));
                }
            }
        }
    }
}

fn no_focused_tests_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Focused tests detected")
        .with_help("Remove focused tests before committing")
        .with_label(span)
}

impl EnhancedWasmRule for NoFocusedTests {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove .only() or focused test modifiers".to_string(),
            "Focused tests prevent other tests from running".to_string(),
            "Use IDE features to run specific tests during development".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_descriptive_test_names_rule() {
        assert_eq!(RequireDescriptiveTestNames::NAME, "require-descriptive-test-names");
        assert_eq!(RequireDescriptiveTestNames::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_focused_tests_rule() {
        assert_eq!(NoFocusedTests::NAME, "no-focused-tests");
        assert_eq!(NoFocusedTests::CATEGORY, WasmRuleCategory::Correctness);
    }
}