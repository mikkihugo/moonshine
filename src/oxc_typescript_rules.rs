//! TypeScript-specific rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoExplicitAny;

impl NoExplicitAny {
    pub const NAME: &'static str = "no-explicit-any";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoExplicitAny {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TSAnyKeyword(_any) = node.kind() {
            ctx.diagnostic(no_explicit_any_diagnostic(_any.span));
        }
    }
}

fn no_explicit_any_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Explicit 'any' type")
        .with_help("Use more specific types instead of 'any'")
        .with_label(span)
}

impl EnhancedWasmRule for NoExplicitAny {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use 'unknown' for truly unknown types".to_string(),
            "Define proper interfaces for object types".to_string(),
            "Use union types for multiple possibilities".to_string(),
            "Consider using generic types for flexibility".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferInterfaceOverType;

impl PreferInterfaceOverType {
    pub const NAME: &'static str = "prefer-interface";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferInterfaceOverType {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TSTypeAliasDeclaration(type_alias) = node.kind() {
            if self.should_be_interface(&type_alias.type_annotation) {
                ctx.diagnostic(prefer_interface_diagnostic(&type_alias.id.name, type_alias.span));
            }
        }
    }
}

impl PreferInterfaceOverType {
    fn should_be_interface(&self, type_annotation: &oxc_ast::ast::TSType) -> bool {
        // Check if type alias defines an object that could be an interface
        matches!(type_annotation, oxc_ast::ast::TSType::TSTypeLiteral(_))
    }
}

fn prefer_interface_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer interface over type alias")
        .with_help(format!("Convert type alias '{}' to an interface", name))
        .with_label(span)
}

impl EnhancedWasmRule for PreferInterfaceOverType {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Interfaces can be extended and merged".to_string(),
            "Interfaces provide better IDE support".to_string(),
            "Use type aliases for unions and primitives".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnusedVariables;

impl NoUnusedVariables {
    pub const NAME: &'static str = "no-unused-vars-ts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUnusedVariables {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::TSInterfaceDeclaration(interface) => {
                if !self.is_interface_used(&interface.id.name, ctx) {
                    ctx.diagnostic(unused_interface_diagnostic(&interface.id.name, interface.span));
                }
            }
            AstKind::TSTypeAliasDeclaration(type_alias) => {
                if !self.is_type_used(&type_alias.id.name, ctx) {
                    ctx.diagnostic(unused_type_diagnostic(&type_alias.id.name, type_alias.span));
                }
            }
            _ => {}
        }
    }
}

impl NoUnusedVariables {
    fn is_interface_used(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if interface is referenced anywhere
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name &&
            symbol.flags().contains(oxc_semantic::SymbolFlags::Type)
        })
    }

    fn is_type_used(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if type alias is referenced anywhere
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name &&
            symbol.flags().contains(oxc_semantic::SymbolFlags::TypeAlias)
        })
    }
}

fn unused_interface_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused interface")
        .with_help(format!("Interface '{}' is defined but never used", name))
        .with_label(span)
}

fn unused_type_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused type alias")
        .with_help(format!("Type alias '{}' is defined but never used", name))
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedVariables {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove unused type definitions to reduce bundle size".to_string(),
            "Consider if this type should be exported".to_string(),
            "Check if type name has a typo".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireReturnType;

impl RequireReturnType {
    pub const NAME: &'static str = "explicit-function-return-type";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireReturnType {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if func.return_type.is_none() {
                    ctx.diagnostic(missing_return_type_diagnostic("function", func.span));
                }
            }
            AstKind::ArrowFunction(func) => {
                if func.return_type.is_none() {
                    ctx.diagnostic(missing_return_type_diagnostic("arrow function", func.span));
                }
            }
            _ => {}
        }
    }
}

fn missing_return_type_diagnostic(func_type: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing return type annotation")
        .with_help(format!("Add explicit return type to {}", func_type))
        .with_label(span)
}

impl EnhancedWasmRule for RequireReturnType {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Explicit return types improve code documentation".to_string(),
            "Return types catch type errors early".to_string(),
            "Use 'void' for functions that don't return a value".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoNonNullAssertion;

impl NoNonNullAssertion {
    pub const NAME: &'static str = "no-non-null-assertion";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoNonNullAssertion {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TSNonNullExpression(_expr) = node.kind() {
            ctx.diagnostic(non_null_assertion_diagnostic(_expr.span));
        }
    }
}

fn non_null_assertion_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Non-null assertion operator")
        .with_help("Use explicit null checking instead of '!' operator")
        .with_label(span)
}

impl EnhancedWasmRule for NoNonNullAssertion {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use optional chaining: obj?.prop".to_string(),
            "Add explicit null checks: if (obj !== null)".to_string(),
            "Non-null assertions can cause runtime errors".to_string(),
            "Consider using nullish coalescing: obj ?? defaultValue".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_explicit_any_rule() {
        assert_eq!(NoExplicitAny::NAME, "no-explicit-any");
        assert_eq!(NoExplicitAny::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_prefer_interface_rule() {
        assert_eq!(PreferInterfaceOverType::NAME, "prefer-interface");
        assert_eq!(PreferInterfaceOverType::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferInterfaceOverType::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_unused_vars_ts_rule() {
        assert_eq!(NoUnusedVariables::NAME, "no-unused-vars-ts");
        assert_eq!(NoUnusedVariables::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_explicit_return_type_rule() {
        assert_eq!(RequireReturnType::NAME, "explicit-function-return-type");
        assert_eq!(RequireReturnType::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_non_null_assertion_rule() {
        assert_eq!(NoNonNullAssertion::NAME, "no-non-null-assertion");
        assert_eq!(NoNonNullAssertion::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_type_literal_detection() {
        let rule = PreferInterfaceOverType;
        // Would test with actual AST nodes in real implementation
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoExplicitAny;
        let diagnostic = no_explicit_any_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("unknown"));
    }
}