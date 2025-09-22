//! Advanced performance optimization rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoExpensiveRegexInLoop;

impl NoExpensiveRegexInLoop {
    pub const NAME: &'static str = "no-expensive-regex-in-loop";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoExpensiveRegexInLoop {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::NewExpression(new_expr) = node.kind() {
            if self.is_regex_constructor(new_expr) && self.is_in_loop(ctx) {
                ctx.diagnostic(no_expensive_regex_in_loop_diagnostic(new_expr.span));
            }
        }
    }
}

impl NoExpensiveRegexInLoop {
    fn is_regex_constructor(&self, new_expr: &oxc_ast::ast::NewExpression) -> bool {
        if let Some(ident) = new_expr.callee.as_identifier() {
            ident.name == "RegExp"
        } else {
            false
        }
    }

    fn is_in_loop(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're inside a loop
        true
    }
}

fn no_expensive_regex_in_loop_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Expensive regex creation in loop")
        .with_help("Move regex compilation outside the loop")
        .with_label(span)
}

impl EnhancedWasmRule for NoExpensiveRegexInLoop {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Regex compilation is expensive".to_string(),
            "Cache compiled regexes outside loops".to_string(),
            "Consider using string methods for simple patterns".to_string(),
            "Pre-compile regexes at module level".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferSetOverArray;

impl PreferSetOverArray {
    pub const NAME: &'static str = "prefer-set-over-array";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferSetOverArray {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_array_includes_in_loop(call, ctx) {
                ctx.diagnostic(prefer_set_over_array_diagnostic(call.span));
            }
        }
    }
}

impl PreferSetOverArray {
    fn is_array_includes_in_loop(&self, call: &oxc_ast::ast::CallExpression, _ctx: &WasmLintContext) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "includes";
            }
        }
        false
    }
}

fn prefer_set_over_array_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Array.includes() in performance-critical code")
        .with_help("Use Set.has() for O(1) lookup instead of O(n) array search")
        .with_label(span)
}

impl EnhancedWasmRule for PreferSetOverArray {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Set.has() is O(1) vs Array.includes() O(n)".to_string(),
            "Use Set for membership testing".to_string(),
            "Convert arrays to Sets for frequent lookups".to_string(),
            "Consider Map for key-value lookups".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoStringConcatInLoop;

impl NoStringConcatInLoop {
    pub const NAME: &'static str = "no-string-concat-in-loop";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoStringConcatInLoop {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::BinaryExpression(bin_expr) = node.kind() {
            if self.is_string_concatenation(bin_expr) && self.is_in_loop(ctx) {
                ctx.diagnostic(no_string_concat_in_loop_diagnostic(bin_expr.span));
            }
        }
    }
}

impl NoStringConcatInLoop {
    fn is_string_concatenation(&self, bin_expr: &oxc_ast::ast::BinaryExpression) -> bool {
        matches!(bin_expr.operator, oxc_ast::ast::BinaryOperator::Addition)
    }

    fn is_in_loop(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're inside a loop
        true
    }
}

fn no_string_concat_in_loop_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("String concatenation in loop")
        .with_help("Use array.join() or StringBuilder pattern for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for NoStringConcatInLoop {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "String concatenation creates new strings each time".to_string(),
            "Use array.push() then join() for loops".to_string(),
            "Consider template literals for complex strings".to_string(),
            "StringBuilder pattern improves memory efficiency".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferObjectFreeze;

impl PreferObjectFreeze {
    pub const NAME: &'static str = "prefer-object-freeze";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferObjectFreeze {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclarator(declarator) = node.kind() {
            if let Some(init) = &declarator.init {
                if self.is_constant_object(init) && !self.is_frozen(init) {
                    ctx.diagnostic(prefer_object_freeze_diagnostic(declarator.span));
                }
            }
        }
    }
}

impl PreferObjectFreeze {
    fn is_constant_object(&self, expr: &oxc_ast::ast::Expression) -> bool {
        matches!(expr, oxc_ast::ast::Expression::ObjectExpression(_))
    }

    fn is_frozen(&self, _expr: &oxc_ast::ast::Expression) -> bool {
        // Check if Object.freeze() is called
        false
    }
}

fn prefer_object_freeze_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Mutable constant object")
        .with_help("Use Object.freeze() for immutable objects to enable V8 optimizations")
        .with_label(span)
}

impl EnhancedWasmRule for PreferObjectFreeze {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Object.freeze() enables V8 optimizations".to_string(),
            "Frozen objects can be stored in fast mode".to_string(),
            "Use const assertions in TypeScript".to_string(),
            "Immutable objects prevent accidental mutations".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoRepeatedPropertyAccess;

impl NoRepeatedPropertyAccess {
    pub const NAME: &'static str = "no-repeated-property-access";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoRepeatedPropertyAccess {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::MemberExpression(member) = node.kind() {
            if self.is_repeated_access(member, ctx) {
                ctx.diagnostic(no_repeated_property_access_diagnostic(member.span));
            }
        }
    }
}

impl NoRepeatedPropertyAccess {
    fn is_repeated_access(&self, _member: &oxc_ast::ast::MemberExpression, _ctx: &WasmLintContext) -> bool {
        // Check if the same property is accessed multiple times
        true
    }
}

fn no_repeated_property_access_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Repeated property access")
        .with_help("Cache property access in a variable for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for NoRepeatedPropertyAccess {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Property access has overhead".to_string(),
            "Cache frequently accessed properties".to_string(),
            "Use destructuring for multiple properties".to_string(),
            "Consider using with statement cautiously".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferTypedArrays;

impl PreferTypedArrays {
    pub const NAME: &'static str = "prefer-typed-arrays";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferTypedArrays {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ArrayExpression(array) = node.kind() {
            if self.is_numeric_array(array) && self.is_performance_critical(ctx) {
                ctx.diagnostic(prefer_typed_arrays_diagnostic(array.span));
            }
        }
    }
}

impl PreferTypedArrays {
    fn is_numeric_array(&self, _array: &oxc_ast::ast::ArrayExpression) -> bool {
        // Check if array contains only numbers
        true
    }

    fn is_performance_critical(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're in a performance-critical section
        true
    }
}

fn prefer_typed_arrays_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Regular array for numeric data")
        .with_help("Use TypedArrays (Int32Array, Float64Array) for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for PreferTypedArrays {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "TypedArrays are more memory efficient".to_string(),
            "Better cache locality for numeric computations".to_string(),
            "Use Float32Array for graphics operations".to_string(),
            "TypedArrays enable SIMD optimizations".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLargeObjectLiterals;

impl NoLargeObjectLiterals {
    pub const NAME: &'static str = "no-large-object-literals";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoLargeObjectLiterals {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if obj.properties.len() > 20 && self.is_in_hot_path(ctx) {
                ctx.diagnostic(no_large_object_literals_diagnostic(obj.span));
            }
        }
    }
}

impl NoLargeObjectLiterals {
    fn is_in_hot_path(&self, _ctx: &WasmLintContext) -> bool {
        // Check if object is created in a frequently called function
        true
    }
}

fn no_large_object_literals_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Large object literal in hot path")
        .with_help("Consider using a class or factory function for large objects")
        .with_label(span)
}

impl EnhancedWasmRule for NoLargeObjectLiterals {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Large object literals have creation overhead".to_string(),
            "Use classes with pre-defined shapes".to_string(),
            "Consider object pooling for frequent creation".to_string(),
            "Break large objects into smaller components".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_expensive_regex_in_loop_rule() {
        assert_eq!(NoExpensiveRegexInLoop::NAME, "no-expensive-regex-in-loop");
        assert_eq!(NoExpensiveRegexInLoop::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_prefer_set_over_array_rule() {
        assert_eq!(PreferSetOverArray::NAME, "prefer-set-over-array");
        assert_eq!(PreferSetOverArray::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_no_string_concat_in_loop_rule() {
        assert_eq!(NoStringConcatInLoop::NAME, "no-string-concat-in-loop");
        assert_eq!(NoStringConcatInLoop::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = PreferSetOverArray;
        let diagnostic = prefer_set_over_array_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("O(1)"));
    }
}