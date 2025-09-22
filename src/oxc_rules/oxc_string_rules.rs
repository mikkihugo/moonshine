//! String manipulation and text pattern rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoUselessStringConcat;

impl NoUselessStringConcat {
    pub const NAME: &'static str = "no-useless-string-concat";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUselessStringConcat {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::BinaryExpression(binary) = node.kind() {
            if binary.operator.is_addition() {
                if let (Some(left_lit), Some(right_lit)) = (
                    binary.left.as_string_literal(),
                    binary.right.as_string_literal()
                ) {
                    ctx.diagnostic(useless_string_concat_diagnostic(binary.span));
                }
            }
        }
    }
}

fn useless_string_concat_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Useless string concatenation")
        .with_help("Combine these string literals into a single string")
        .with_label(span)
}

impl EnhancedWasmRule for NoUselessStringConcat {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Use template literals for dynamic content".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferStringStartsWith;

impl PreferStringStartsWith {
    pub const NAME: &'static str = "prefer-string-starts-with";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferStringStartsWith {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::BinaryExpression(binary) = node.kind() {
            if binary.operator.is_equality() {
                if self.is_indexof_zero_check(binary) {
                    ctx.diagnostic(prefer_starts_with_diagnostic(binary.span));
                }
            }
        }
    }
}

impl PreferStringStartsWith {
    fn is_indexof_zero_check(&self, binary: &oxc_ast::ast::BinaryExpression) -> bool {
        // Check for str.indexOf(x) === 0 pattern
        if let Some(call) = binary.left.as_call_expression() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    return prop.name == "indexOf";
                }
            }
        }
        false
    }
}

fn prefer_starts_with_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Use String.startsWith() instead of indexOf")
        .with_help("Replace .indexOf() === 0 with .startsWith()")
        .with_label(span)
}

impl EnhancedWasmRule for PreferStringStartsWith {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["More readable and expressive".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoRegexSpaces;

impl NoRegexSpaces {
    pub const NAME: &'static str = "no-regex-spaces";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoRegexSpaces {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::RegExpLiteral(regex) = node.kind() {
            if regex.regex.pattern.contains("  ") {
                ctx.diagnostic(no_regex_spaces_diagnostic(regex.span));
            }
        }
    }
}

fn no_regex_spaces_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Multiple spaces in regex")
        .with_help("Use \\s{2,} or \\s+ instead of multiple space characters")
        .with_label(span)
}

impl EnhancedWasmRule for NoRegexSpaces {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Use quantifiers for clearer intent".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMultilineStringLiterals;

impl NoMultilineStringLiterals {
    pub const NAME: &'static str = "no-multiline-string-literals";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoMultilineStringLiterals {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if string_lit.value.contains('\n') {
                ctx.diagnostic(no_multiline_string_diagnostic(string_lit.span));
            }
        }
    }
}

fn no_multiline_string_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Multiline string literal")
        .with_help("Use template literals or string concatenation")
        .with_label(span)
}

impl EnhancedWasmRule for NoMultilineStringLiterals {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Template literals are more readable for multiline text".to_string()]
    }
}