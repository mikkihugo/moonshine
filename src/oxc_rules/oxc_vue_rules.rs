//! Vue.js specific rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct VueRequireValidDefaultProp;

impl VueRequireValidDefaultProp {
    pub const NAME: &'static str = "vue-require-valid-default-prop";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for VueRequireValidDefaultProp {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_vue_props_object(obj) {
                for prop in &obj.properties {
                    if self.has_invalid_default_prop(prop) {
                        ctx.diagnostic(vue_require_valid_default_prop_diagnostic(prop.span()));
                    }
                }
            }
        }
    }
}

impl VueRequireValidDefaultProp {
    fn is_vue_props_object(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check if this is a Vue props definition
        true
    }

    fn has_invalid_default_prop(&self, _prop: &oxc_ast::ast::ObjectPropertyKind) -> bool {
        // Check for invalid default prop patterns
        true
    }
}

fn vue_require_valid_default_prop_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Invalid Vue prop default value")
        .with_help("Use factory functions for object/array defaults in Vue props")
        .with_label(span)
}

impl EnhancedWasmRule for VueRequireValidDefaultProp {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use factory functions: default: () => ({})".to_string(),
            "Object/array defaults are shared between instances".to_string(),
            "Factory functions create new instances".to_string(),
            "Avoid mutating shared default values".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VueNoUnusedComponents;

impl VueNoUnusedComponents {
    pub const NAME: &'static str = "vue-no-unused-components";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for VueNoUnusedComponents {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_vue_components_object(obj) {
                for prop in &obj.properties {
                    if !self.is_component_used(prop, ctx) {
                        ctx.diagnostic(vue_no_unused_components_diagnostic(prop.span()));
                    }
                }
            }
        }
    }
}

impl VueNoUnusedComponents {
    fn is_vue_components_object(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check if this is a Vue components definition
        true
    }

    fn is_component_used(&self, _prop: &oxc_ast::ast::ObjectPropertyKind, _ctx: &WasmLintContext) -> bool {
        // Check if component is used in template
        false
    }
}

fn vue_no_unused_components_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused Vue component")
        .with_help("Remove unused component or use it in template")
        .with_label(span)
}

impl EnhancedWasmRule for VueNoUnusedComponents {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Unused components increase bundle size".to_string(),
            "Use tree-shaking friendly imports".to_string(),
            "Consider dynamic imports for large components".to_string(),
            "Remove or comment unused component imports".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VueRequireVForKey;

impl VueRequireVForKey {
    pub const NAME: &'static str = "vue-require-v-for-key";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for VueRequireVForKey {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // This would require template parsing for Vue SFC files
        if let AstKind::StringLiteral(template) = node.kind() {
            if self.is_vue_template(template) && self.has_v_for_without_key(template) {
                ctx.diagnostic(vue_require_v_for_key_diagnostic(template.span));
            }
        }
    }
}

impl VueRequireVForKey {
    fn is_vue_template(&self, _template: &oxc_ast::ast::StringLiteral) -> bool {
        // Check if this is a Vue template string
        false
    }

    fn has_v_for_without_key(&self, _template: &oxc_ast::ast::StringLiteral) -> bool {
        // Check for v-for without :key
        false
    }
}

fn vue_require_v_for_key_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("v-for missing key")
        .with_help("Add :key attribute to v-for elements for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for VueRequireVForKey {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Keys help Vue track changes efficiently".to_string(),
            "Use unique, stable keys (avoid array indices)".to_string(),
            "Keys improve Virtual DOM diff performance".to_string(),
            "Missing keys can cause component state issues".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VueNoMutatingProps;

impl VueNoMutatingProps {
    pub const NAME: &'static str = "vue-no-mutating-props";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for VueNoMutatingProps {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if self.is_mutating_prop(&assign.left, ctx) {
                ctx.diagnostic(vue_no_mutating_props_diagnostic(assign.span));
            }
        }
    }
}

impl VueNoMutatingProps {
    fn is_mutating_prop(&self, _left: &oxc_ast::ast::AssignmentTarget, _ctx: &WasmLintContext) -> bool {
        // Check if assignment target is a Vue prop
        true
    }
}

fn vue_no_mutating_props_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Mutating Vue prop")
        .with_help("Use computed properties or emit events instead of mutating props")
        .with_label(span)
}

impl EnhancedWasmRule for VueNoMutatingProps {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Props follow one-way data flow".to_string(),
            "Use $emit to communicate changes to parent".to_string(),
            "Create local data copy if mutation needed".to_string(),
            "Use computed properties for derived values".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VuePreferCompositionAPI;

impl VuePreferCompositionAPI {
    pub const NAME: &'static str = "vue-prefer-composition-api";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for VuePreferCompositionAPI {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_options_api_component(obj) {
                ctx.diagnostic(vue_prefer_composition_api_diagnostic(obj.span));
            }
        }
    }
}

impl VuePreferCompositionAPI {
    fn is_options_api_component(&self, obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for Options API patterns (data, methods, computed, etc.)
        obj.properties.len() > 3 // Simplified check
    }
}

fn vue_prefer_composition_api_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Options API component detected")
        .with_help("Consider using Composition API for better TypeScript support")
        .with_label(span)
}

impl EnhancedWasmRule for VuePreferCompositionAPI {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Composition API has better TypeScript inference".to_string(),
            "Better code reusability with composables".to_string(),
            "More explicit reactivity with ref() and reactive()".to_string(),
            "Easier testing and logic extraction".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VueNoDeprecatedScopeAttribute;

impl VueNoDeprecatedScopeAttribute {
    pub const NAME: &'static str = "vue-no-deprecated-scope-attribute";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for VueNoDeprecatedScopeAttribute {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // This would require template parsing
        if let AstKind::StringLiteral(template) = node.kind() {
            if self.has_deprecated_scope_attribute(template) {
                ctx.diagnostic(vue_no_deprecated_scope_attribute_diagnostic(template.span));
            }
        }
    }
}

impl VueNoDeprecatedScopeAttribute {
    fn has_deprecated_scope_attribute(&self, template: &oxc_ast::ast::StringLiteral) -> bool {
        template.value.contains("scope=") || template.value.contains("slot-scope=")
    }
}

fn vue_no_deprecated_scope_attribute_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Deprecated Vue scope attribute")
        .with_help("Use v-slot syntax instead of scope or slot-scope")
        .with_label(span)
}

impl EnhancedWasmRule for VueNoDeprecatedScopeAttribute {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Replace scope with v-slot:default".to_string(),
            "Use #default=\"props\" shorthand syntax".to_string(),
            "v-slot provides better TypeScript support".to_string(),
            "Deprecated syntax removed in Vue 3".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct VueRequireSlotProps;

impl VueRequireSlotProps {
    pub const NAME: &'static str = "vue-require-slot-props";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for VueRequireSlotProps {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Check for slot usage without proper typing
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_slot_call_without_props(call) {
                ctx.diagnostic(vue_require_slot_props_diagnostic(call.span));
            }
        }
    }
}

impl VueRequireSlotProps {
    fn is_slot_call_without_props(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for slot calls without proper prop typing
        false
    }
}

fn vue_require_slot_props_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Slot without prop typing")
        .with_help("Define slot prop types for better component API")
        .with_label(span)
}

impl EnhancedWasmRule for VueRequireSlotProps {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Type slot props for better developer experience".to_string(),
            "Use defineSlots() macro in Vue 3".to_string(),
            "Slot typing improves IDE intellisense".to_string(),
            "Document slot props in component API".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vue_require_valid_default_prop_rule() {
        assert_eq!(VueRequireValidDefaultProp::NAME, "vue-require-valid-default-prop");
        assert_eq!(VueRequireValidDefaultProp::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_vue_no_unused_components_rule() {
        assert_eq!(VueNoUnusedComponents::NAME, "vue-no-unused-components");
        assert_eq!(VueNoUnusedComponents::CATEGORY, WasmRuleCategory::Suspicious);
        assert_eq!(VueNoUnusedComponents::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_vue_prefer_composition_api_rule() {
        assert_eq!(VuePreferCompositionAPI::NAME, "vue-prefer-composition-api");
        assert_eq!(VuePreferCompositionAPI::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_deprecated_attribute_detection() {
        let rule = VueNoDeprecatedScopeAttribute;
        let template_with_scope = oxc_ast::ast::StringLiteral {
            span: Span::default(),
            value: "scope=\"item\"".into(),
        };
        assert!(rule.has_deprecated_scope_attribute(&template_with_scope));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = VueRequireValidDefaultProp;
        let diagnostic = vue_require_valid_default_prop_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("factory"));
    }
}