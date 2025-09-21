//! Variable declaration and usage rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoVarDeclarations;

impl NoVarDeclarations {
    pub const NAME: &'static str = "no-var";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoVarDeclarations {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclaration(var_decl) = node.kind() {
            if var_decl.kind.is_var() {
                ctx.diagnostic(no_var_diagnostic(var_decl.span));
            }
        }
    }
}

fn no_var_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unexpected var declaration")
        .with_help("Use 'let' or 'const' instead of 'var'")
        .with_label(span)
}

impl EnhancedWasmRule for NoVarDeclarations {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use 'const' for values that never change".to_string(),
            "Use 'let' for values that will be reassigned".to_string(),
            "'var' has function scope, which can lead to bugs".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUndeclaredVariables;

impl NoUndeclaredVariables {
    pub const NAME: &'static str = "no-undef";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoUndeclaredVariables {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::IdentifierReference(ident) = node.kind() {
            if !self.is_declared(&ident.name, ctx) {
                ctx.diagnostic(undeclared_variable_diagnostic(&ident.name, ident.span));
            }
        }
    }
}

impl NoUndeclaredVariables {
    fn is_declared(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if variable is declared in current scope
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name
        })
    }
}

fn undeclared_variable_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Undeclared variable")
        .with_help(format!("'{}' is not defined", name))
        .with_label(span)
}

impl EnhancedWasmRule for NoUndeclaredVariables {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check for typos in variable names".to_string(),
            "Add import statement if using external library".to_string(),
            "Declare variable before use".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferConst;

impl PreferConst {
    pub const NAME: &'static str = "prefer-const";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferConst {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclaration(var_decl) = node.kind() {
            if var_decl.kind.is_let() {
                for declarator in &var_decl.declarations {
                    if let Some(id) = declarator.id.as_binding_identifier() {
                        if !self.is_reassigned(&id.name, ctx) {
                            ctx.diagnostic(prefer_const_diagnostic(&id.name, declarator.span));
                        }
                    }
                }
            }
        }
    }
}

impl PreferConst {
    fn is_reassigned(&self, name: &str, _ctx: &WasmLintContext) -> bool {
        // Check if variable is reassigned after declaration
        // Simplified implementation - would need full control flow analysis
        false
    }
}

fn prefer_const_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer const declaration")
        .with_help(format!("'{}' is never reassigned, use 'const' instead", name))
        .with_label(span)
}

impl EnhancedWasmRule for PreferConst {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "const declarations prevent accidental reassignment".to_string(),
            "Use const for immutable values to improve code clarity".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoGlobalAssignment;

impl NoGlobalAssignment {
    pub const NAME: &'static str = "no-global-assign";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoGlobalAssignment {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if let Some(ident) = assign.left.as_identifier() {
                if self.is_global_object(&ident.name) {
                    ctx.diagnostic(global_assignment_diagnostic(&ident.name, assign.span));
                }
            }
        }
    }
}

impl NoGlobalAssignment {
    fn is_global_object(&self, name: &str) -> bool {
        matches!(name,
            "Object" | "Array" | "String" | "Number" | "Boolean" |
            "Function" | "Date" | "RegExp" | "Error" | "JSON" |
            "Math" | "parseInt" | "parseFloat" | "isNaN" | "isFinite" |
            "console" | "window" | "document" | "global" | "process"
        )
    }
}

fn global_assignment_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Assignment to global object")
        .with_help(format!("'{}' is a global object, avoid reassigning it", name))
        .with_label(span)
}

impl EnhancedWasmRule for NoGlobalAssignment {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Assigning to globals can break other code".to_string(),
            "Use a different variable name".to_string(),
            "Consider using a namespace or module pattern".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDeleteVar;

impl NoDeleteVar {
    pub const NAME: &'static str = "no-delete-var";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDeleteVar {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::UnaryExpression(unary) = node.kind() {
            if unary.operator.is_delete() {
                if let Some(_ident) = unary.argument.as_identifier_reference() {
                    ctx.diagnostic(delete_var_diagnostic(unary.span));
                }
            }
        }
    }
}

fn delete_var_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Delete operator on variable")
        .with_help("Variables cannot be deleted, only object properties")
        .with_label(span)
}

impl EnhancedWasmRule for NoDeleteVar {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "delete only works on object properties".to_string(),
            "Set variable to null or undefined instead".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oxc_compatible_rules::LintSeverity;

    #[test]
    fn test_no_var_rule() {
        assert_eq!(NoVarDeclarations::NAME, "no-var");
        assert_eq!(NoVarDeclarations::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoVarDeclarations::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_prefer_const_rule() {
        assert_eq!(PreferConst::NAME, "prefer-const");
        assert_eq!(PreferConst::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferConst::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_undef_rule() {
        assert_eq!(NoUndeclaredVariables::NAME, "no-undef");
        assert_eq!(NoUndeclaredVariables::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_global_object_detection() {
        let rule = NoGlobalAssignment;
        assert!(rule.is_global_object("Object"));
        assert!(rule.is_global_object("Array"));
        assert!(rule.is_global_object("console"));
        assert!(!rule.is_global_object("myVariable"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoVarDeclarations;
        let diagnostic = no_var_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("const"));
    }
}