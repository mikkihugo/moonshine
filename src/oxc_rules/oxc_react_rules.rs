//! React-specific rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct ReactHooksRules;

impl ReactHooksRules {
    pub const NAME: &'static str = "react-hooks-rules-of-hooks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for ReactHooksRules {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(ident) = call.callee.as_identifier() {
                if self.is_hook_call(&ident.name) {
                    if self.is_in_conditional_or_loop(ctx) {
                        ctx.diagnostic(hooks_rules_diagnostic(&ident.name, call.span));
                    }
                }
            }
        }
    }
}

impl ReactHooksRules {
    fn is_hook_call(&self, name: &str) -> bool {
        name.starts_with("use") && name.len() > 3 &&
        name.chars().nth(3).map_or(false, |c| c.is_uppercase())
    }

    fn is_in_conditional_or_loop(&self, _ctx: &WasmLintContext) -> bool {
        // Simplified check - would need to analyze call stack
        false
    }
}

fn hooks_rules_diagnostic(hook_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Hook called conditionally")
        .with_help(format!("Hook '{}' must be called at the top level", hook_name))
        .with_label(span)
}

impl EnhancedWasmRule for ReactHooksRules {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Move hooks to the top level of your component".to_string(),
            "Don't call hooks inside loops, conditions, or nested functions".to_string(),
            "Use early returns after all hook calls".to_string(),
            "Hooks must be called in the same order every time".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnusedState;

impl NoUnusedState {
    pub const NAME: &'static str = "react-no-unused-state";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUnusedState {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(ident) = call.callee.as_identifier() {
                if ident.name == "useState" {
                    if let Some(var_pattern) = self.get_state_variable_pattern(call) {
                        if !self.is_state_variable_used(&var_pattern, ctx) {
                            ctx.diagnostic(unused_state_diagnostic(&var_pattern, call.span));
                        }
                    }
                }
            }
        }
    }
}

impl NoUnusedState {
    fn get_state_variable_pattern(&self, _call: &oxc_ast::ast::CallExpression) -> Option<String> {
        // Extract state variable name from destructuring pattern
        // Simplified implementation
        Some("stateVar".to_string())
    }

    fn is_state_variable_used(&self, _var_name: &str, _ctx: &WasmLintContext) -> bool {
        // Check if state variable is referenced in component
        false
    }
}

fn unused_state_diagnostic(var_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused state variable")
        .with_help(format!("State variable '{}' is declared but never used", var_name))
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedState {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove unused state variables to improve performance".to_string(),
            "Consider if this state should be derived from props".to_string(),
            "Unused state can cause unnecessary re-renders".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireKey;

impl RequireKey {
    pub const NAME: &'static str = "react-require-key";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireKey {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if self.is_in_array_map(ctx) && !self.has_key_prop(jsx) {
                ctx.diagnostic(require_key_diagnostic(jsx.span));
            }
        }
    }
}

impl RequireKey {
    fn is_in_array_map(&self, _ctx: &WasmLintContext) -> bool {
        // Check if JSX element is inside array.map()
        // Simplified implementation
        true
    }

    fn has_key_prop(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return name.name == "key";
                }
            }
            false
        })
    }
}

fn require_key_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing key prop")
        .with_help("Add a unique 'key' prop to JSX elements in arrays")
        .with_label(span)
}

impl EnhancedWasmRule for RequireKey {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use unique, stable identifiers as keys".to_string(),
            "Avoid using array indices as keys when order can change".to_string(),
            "Keys help React identify which items have changed".to_string(),
            "Missing keys can cause rendering performance issues".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoStringRefs;

impl NoStringRefs {
    pub const NAME: &'static str = "react-no-string-refs";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoStringRefs {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttribute(attr) = node.kind() {
            if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                if name.name == "ref" {
                    if let Some(value) = &attr.value {
                        if self.is_string_ref(value) {
                            ctx.diagnostic(string_refs_diagnostic(attr.span));
                        }
                    }
                }
            }
        }
    }
}

impl NoStringRefs {
    fn is_string_ref(&self, value: &oxc_ast::ast::JSXAttributeValue) -> bool {
        match value {
            oxc_ast::ast::JSXAttributeValue::StringLiteral(_) => true,
            oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) => {
                matches!(container.expression, oxc_ast::ast::JSXExpression::StringLiteral(_))
            }
            _ => false,
        }
    }
}

fn string_refs_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("String refs are deprecated")
        .with_help("Use React.createRef() or useRef() hook instead")
        .with_label(span)
}

impl EnhancedWasmRule for NoStringRefs {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use React.createRef() in class components".to_string(),
            "Use useRef() hook in function components".to_string(),
            "String refs will be removed in future React versions".to_string(),
            "Ref callbacks provide more flexibility".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDirectMutationState;

impl NoDirectMutationState {
    pub const NAME: &'static str = "react-no-direct-mutation-state";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDirectMutationState {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if let Some(member) = assign.left.as_member_expression() {
                if self.is_state_mutation(member) {
                    ctx.diagnostic(direct_state_mutation_diagnostic(assign.span));
                }
            }
        }
    }
}

impl NoDirectMutationState {
    fn is_state_mutation(&self, member: &oxc_ast::ast::MemberExpression) -> bool {
        // Check if this is this.state.something = value
        if let Some(obj_member) = member.object().as_member_expression() {
            if let Some(this_obj) = obj_member.object().as_this_expression() {
                if let Some(prop) = obj_member.property().as_identifier() {
                    return prop.name == "state";
                }
            }
        }
        false
    }
}

fn direct_state_mutation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct state mutation")
        .with_help("Use setState() or state setter function instead")
        .with_label(span)
}

impl EnhancedWasmRule for NoDirectMutationState {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use this.setState() in class components".to_string(),
            "Use state setter function from useState hook".to_string(),
            "Direct mutations don't trigger re-renders".to_string(),
            "State should be treated as immutable".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react_hooks_rules() {
        assert_eq!(ReactHooksRules::NAME, "react-hooks-rules-of-hooks");
        assert_eq!(ReactHooksRules::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_hook_detection() {
        let rule = ReactHooksRules;
        assert!(rule.is_hook_call("useState"));
        assert!(rule.is_hook_call("useEffect"));
        assert!(rule.is_hook_call("useCustomHook"));
        assert!(!rule.is_hook_call("use"));
        assert!(!rule.is_hook_call("user"));
        assert!(!rule.is_hook_call("regular_function"));
    }

    #[test]
    fn test_no_unused_state_rule() {
        assert_eq!(NoUnusedState::NAME, "react-no-unused-state");
        assert_eq!(NoUnusedState::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoUnusedState::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_key_rule() {
        assert_eq!(RequireKey::NAME, "react-require-key");
        assert_eq!(RequireKey::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(RequireKey::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_string_refs_rule() {
        assert_eq!(NoStringRefs::NAME, "react-no-string-refs");
        assert_eq!(NoStringRefs::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoStringRefs::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_direct_mutation_state_rule() {
        assert_eq!(NoDirectMutationState::NAME, "react-no-direct-mutation-state");
        assert_eq!(NoDirectMutationState::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoDirectMutationState::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = ReactHooksRules;
        let diagnostic = hooks_rules_diagnostic("useState", Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("top level"));
    }
}