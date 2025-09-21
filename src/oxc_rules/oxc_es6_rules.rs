//! ES6+ Modern JavaScript rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct PreferTemplateStrings;

impl PreferTemplateStrings {
    pub const NAME: &'static str = "prefer-template";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferTemplateStrings {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::BinaryExpression(binary) = node.kind() {
            if binary.operator.is_addition() && self.involves_string_concatenation(binary) {
                ctx.diagnostic(prefer_template_diagnostic(binary.span));
            }
        }
    }
}

impl PreferTemplateStrings {
    fn involves_string_concatenation(&self, binary: &oxc_ast::ast::BinaryExpression) -> bool {
        self.is_string_like(&binary.left) || self.is_string_like(&binary.right)
    }

    fn is_string_like(&self, expr: &oxc_ast::ast::Expression) -> bool {
        matches!(expr,
            oxc_ast::ast::Expression::StringLiteral(_) |
            oxc_ast::ast::Expression::TemplateLiteral(_)
        )
    }
}

fn prefer_template_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer template literals over string concatenation")
        .with_help("Use template literals: `Hello ${name}` instead of 'Hello ' + name")
        .with_label(span)
}

impl EnhancedWasmRule for PreferTemplateStrings {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Template literals are more readable".to_string(),
            "Support multiline strings without escaping".to_string(),
            "Better performance than string concatenation".to_string(),
            "Use ${expression} for variable interpolation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferDestructuringAssignment;

impl PreferDestructuringAssignment {
    pub const NAME: &'static str = "prefer-destructuring-assignment";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferDestructuringAssignment {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclarator(declarator) = node.kind() {
            if let Some(init) = &declarator.init {
                if self.can_use_destructuring(init) {
                    ctx.diagnostic(prefer_destructuring_assignment_diagnostic(declarator.span));
                }
            }
        }
    }
}

impl PreferDestructuringAssignment {
    fn can_use_destructuring(&self, expr: &oxc_ast::ast::Expression) -> bool {
        // Check for property access patterns that could use destructuring
        if let Some(member) = expr.as_member_expression() {
            return member.property().as_identifier().is_some();
        }
        false
    }
}

fn prefer_destructuring_assignment_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Consider using destructuring assignment")
        .with_help("Use destructuring: const { prop } = obj instead of const prop = obj.prop")
        .with_label(span)
}

impl EnhancedWasmRule for PreferDestructuringAssignment {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Destructuring is more concise and expressive".to_string(),
            "Works with arrays: const [first, second] = arr".to_string(),
            "Supports default values: const { prop = 'default' } = obj".to_string(),
            "Can rename variables: const { prop: newName } = obj".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferSpreadOperator;

impl PreferSpreadOperator {
    pub const NAME: &'static str = "prefer-spread";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferSpreadOperator {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_apply_call(call) {
                ctx.diagnostic(prefer_spread_diagnostic(call.span));
            }
        }
    }
}

impl PreferSpreadOperator {
    fn is_apply_call(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "apply";
            }
        }
        false
    }
}

fn prefer_spread_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer spread operator over apply")
        .with_help("Use spread operator: func(...args) instead of func.apply(null, args)")
        .with_label(span)
}

impl EnhancedWasmRule for PreferSpreadOperator {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Spread operator is more readable".to_string(),
            "Works with any iterable, not just arrays".to_string(),
            "No need to specify 'this' context".to_string(),
            "Better performance in modern engines".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoVarKeyword;

impl NoVarKeyword {
    pub const NAME: &'static str = "no-var-keyword";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoVarKeyword {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclaration(var_decl) = node.kind() {
            if var_decl.kind.is_var() {
                ctx.diagnostic(no_var_keyword_diagnostic(var_decl.span));
            }
        }
    }
}

fn no_var_keyword_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Use let or const instead of var")
        .with_help("var has function scope, let and const have block scope")
        .with_label(span)
}

impl EnhancedWasmRule for NoVarKeyword {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use const for values that don't change".to_string(),
            "Use let for values that will be reassigned".to_string(),
            "Block scope prevents common hoisting bugs".to_string(),
            "Temporal dead zone catches usage before declaration".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferRestParams;

impl PreferRestParams {
    pub const NAME: &'static str = "prefer-rest-params";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferRestParams {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::IdentifierReference(ident) = node.kind() {
            if ident.name == "arguments" {
                ctx.diagnostic(prefer_rest_params_diagnostic(ident.span));
            }
        }
    }
}

fn prefer_rest_params_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Use rest parameters instead of arguments object")
        .with_help("Use ...args parameter instead of arguments object")
        .with_label(span)
}

impl EnhancedWasmRule for PreferRestParams {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Rest parameters are a real array".to_string(),
            "Works with arrow functions".to_string(),
            "More explicit function signature".to_string(),
            "Better IDE support and type checking".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUselessComputedKeys;

impl NoUselessComputedKeys {
    pub const NAME: &'static str = "no-useless-computed-key";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUselessComputedKeys {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            for prop in &obj.properties {
                if let oxc_ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop {
                    if self.has_useless_computed_key(&prop.key) {
                        ctx.diagnostic(useless_computed_key_diagnostic(prop.span));
                    }
                }
            }
        }
    }
}

impl NoUselessComputedKeys {
    fn has_useless_computed_key(&self, key: &oxc_ast::ast::PropertyKey) -> bool {
        match key {
            oxc_ast::ast::PropertyKey::StringLiteral(_) |
            oxc_ast::ast::PropertyKey::NumericLiteral(_) => true,
            oxc_ast::ast::PropertyKey::Identifier(_) => false,
            _ => false,
        }
    }
}

fn useless_computed_key_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unnecessary computed property key")
        .with_help("Remove brackets for literal property keys")
        .with_label(span)
}

impl EnhancedWasmRule for NoUselessComputedKeys {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use computed keys only when necessary".to_string(),
            "Literal keys don't need brackets".to_string(),
            "Computed keys are for dynamic property names".to_string(),
            "Cleaner syntax improves readability".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefer_template_strings_rule() {
        assert_eq!(PreferTemplateStrings::NAME, "prefer-template");
        assert_eq!(PreferTemplateStrings::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferTemplateStrings::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_prefer_destructuring_assignment_rule() {
        assert_eq!(PreferDestructuringAssignment::NAME, "prefer-destructuring-assignment");
        assert_eq!(PreferDestructuringAssignment::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_prefer_spread_operator_rule() {
        assert_eq!(PreferSpreadOperator::NAME, "prefer-spread");
        assert_eq!(PreferSpreadOperator::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferSpreadOperator::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_var_keyword_rule() {
        assert_eq!(NoVarKeyword::NAME, "no-var-keyword");
        assert_eq!(NoVarKeyword::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoVarKeyword::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_prefer_rest_params_rule() {
        assert_eq!(PreferRestParams::NAME, "prefer-rest-params");
        assert_eq!(PreferRestParams::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferRestParams::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_useless_computed_keys_rule() {
        assert_eq!(NoUselessComputedKeys::NAME, "no-useless-computed-key");
        assert_eq!(NoUselessComputedKeys::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoUselessComputedKeys::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = PreferTemplateStrings;
        let diagnostic = prefer_template_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("readable"));
    }
}