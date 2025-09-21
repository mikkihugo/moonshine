//! Angular specific rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct AngularNoDirectDOMAccess;

impl AngularNoDirectDOMAccess {
    pub const NAME: &'static str = "angular-no-direct-dom-access";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularNoDirectDOMAccess {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_direct_dom_access(call) {
                ctx.diagnostic(angular_no_direct_dom_access_diagnostic(call.span));
            }
        }
    }
}

impl AngularNoDirectDOMAccess {
    fn is_direct_dom_access(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                return obj.name == "document" || obj.name == "window";
            }
        }
        false
    }
}

fn angular_no_direct_dom_access_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct DOM access in Angular component")
        .with_help("Use Renderer2 or ElementRef for DOM manipulation")
        .with_label(span)
}

impl EnhancedWasmRule for AngularNoDirectDOMAccess {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Renderer2 for safe DOM manipulation".to_string(),
            "Direct DOM access breaks SSR compatibility".to_string(),
            "Angular's Renderer2 provides platform abstraction".to_string(),
            "Use ViewChild/ElementRef for element references".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularRequireOnDestroyInterface;

impl AngularRequireOnDestroyInterface {
    pub const NAME: &'static str = "angular-require-ondestroy-interface";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for AngularRequireOnDestroyInterface {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Class(class) = node.kind() {
            if self.has_ng_on_destroy_method(class) && !self.implements_on_destroy_interface(class) {
                ctx.diagnostic(angular_require_ondestroy_interface_diagnostic(class.span));
            }
        }
    }
}

impl AngularRequireOnDestroyInterface {
    fn has_ng_on_destroy_method(&self, class: &oxc_ast::ast::Class) -> bool {
        if let Some(body) = &class.body {
            return body.body.iter().any(|member| {
                if let oxc_ast::ast::ClassElement::MethodDefinition(method) = member {
                    if let Some(key) = method.key.as_identifier() {
                        return key.name == "ngOnDestroy";
                    }
                }
                false
            });
        }
        false
    }

    fn implements_on_destroy_interface(&self, _class: &oxc_ast::ast::Class) -> bool {
        // Check if class implements OnDestroy interface
        false
    }
}

fn angular_require_ondestroy_interface_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("NgOnDestroy method without OnDestroy interface")
        .with_help("Implement OnDestroy interface when using ngOnDestroy")
        .with_label(span)
}

impl EnhancedWasmRule for AngularRequireOnDestroyInterface {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Interfaces provide compile-time type checking".to_string(),
            "OnDestroy interface ensures correct method signature".to_string(),
            "Use interfaces for all lifecycle hooks".to_string(),
            "Better IDE support with explicit interfaces".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularNoUnsubscribedObservables;

impl AngularNoUnsubscribedObservables {
    pub const NAME: &'static str = "angular-no-unsubscribed-observables";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularNoUnsubscribedObservables {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_observable_subscription(call) && !self.has_unsubscribe_logic(ctx) {
                ctx.diagnostic(angular_no_unsubscribed_observables_diagnostic(call.span));
            }
        }
    }
}

impl AngularNoUnsubscribedObservables {
    fn is_observable_subscription(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "subscribe";
            }
        }
        false
    }

    fn has_unsubscribe_logic(&self, _ctx: &WasmLintContext) -> bool {
        // Check for unsubscribe logic in ngOnDestroy
        false
    }
}

fn angular_no_unsubscribed_observables_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Observable subscription without unsubscribe")
        .with_help("Add unsubscribe logic in ngOnDestroy to prevent memory leaks")
        .with_label(span)
}

impl EnhancedWasmRule for AngularNoUnsubscribedObservables {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use takeUntil pattern with destroy$ subject".to_string(),
            "Store subscriptions and unsubscribe in ngOnDestroy".to_string(),
            "Consider using async pipe in templates".to_string(),
            "Memory leaks cause performance degradation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularPreferOnPush;

impl AngularPreferOnPush {
    pub const NAME: &'static str = "angular-prefer-onpush";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularPreferOnPush {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Decorator(decorator) = node.kind() {
            if self.is_component_decorator(decorator) && !self.uses_on_push_strategy(decorator) {
                ctx.diagnostic(angular_prefer_onpush_diagnostic(decorator.span));
            }
        }
    }
}

impl AngularPreferOnPush {
    fn is_component_decorator(&self, decorator: &oxc_ast::ast::Decorator) -> bool {
        if let Some(call) = decorator.expression.as_call_expression() {
            if let Some(ident) = call.callee.as_identifier() {
                return ident.name == "Component";
            }
        }
        false
    }

    fn uses_on_push_strategy(&self, _decorator: &oxc_ast::ast::Decorator) -> bool {
        // Check if changeDetection: ChangeDetectionStrategy.OnPush is set
        false
    }
}

fn angular_prefer_onpush_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Component without OnPush change detection")
        .with_help("Use OnPush strategy for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for AngularPreferOnPush {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "OnPush reduces unnecessary change detection cycles".to_string(),
            "Use immutable data patterns with OnPush".to_string(),
            "Manually trigger change detection when needed".to_string(),
            "OnPush improves application performance significantly".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularNoFunctionsInTemplates;

impl AngularNoFunctionsInTemplates {
    pub const NAME: &'static str = "angular-no-functions-in-templates";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularNoFunctionsInTemplates {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // This would require template parsing
        if let AstKind::StringLiteral(template) = node.kind() {
            if self.is_angular_template(template) && self.has_function_calls(template) {
                ctx.diagnostic(angular_no_functions_in_templates_diagnostic(template.span));
            }
        }
    }
}

impl AngularNoFunctionsInTemplates {
    fn is_angular_template(&self, _template: &oxc_ast::ast::StringLiteral) -> bool {
        // Check if this is an Angular template
        false
    }

    fn has_function_calls(&self, template: &oxc_ast::ast::StringLiteral) -> bool {
        // Simple check for function call patterns
        template.value.contains("()")
    }
}

fn angular_no_functions_in_templates_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Function call in Angular template")
        .with_help("Use computed properties or memoization instead of function calls")
        .with_label(span)
}

impl EnhancedWasmRule for AngularNoFunctionsInTemplates {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Function calls in templates run on every change detection".to_string(),
            "Use getters or computed properties for expensive operations".to_string(),
            "Consider memoization with @Memo decorator".to_string(),
            "Pure pipes are better for transformation logic".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularRequireTrackByFunction;

impl AngularRequireTrackByFunction {
    pub const NAME: &'static str = "angular-require-trackby-function";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularRequireTrackByFunction {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // This would require template parsing
        if let AstKind::StringLiteral(template) = node.kind() {
            if self.has_ngfor_without_trackby(template) {
                ctx.diagnostic(angular_require_trackby_function_diagnostic(template.span));
            }
        }
    }
}

impl AngularRequireTrackByFunction {
    fn has_ngfor_without_trackby(&self, template: &oxc_ast::ast::StringLiteral) -> bool {
        // Simple check for *ngFor without trackBy
        template.value.contains("*ngFor") && !template.value.contains("trackBy")
    }
}

fn angular_require_trackby_function_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("*ngFor without trackBy function")
        .with_help("Add trackBy function for better list performance")
        .with_label(span)
}

impl EnhancedWasmRule for AngularRequireTrackByFunction {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "trackBy functions prevent unnecessary DOM updates".to_string(),
            "Use unique identifiers for tracking (id, key)".to_string(),
            "TrackBy improves performance with large lists".to_string(),
            "Avoid using array index for tracking".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct AngularNoOutputNativeEvents;

impl AngularNoOutputNativeEvents {
    pub const NAME: &'static str = "angular-no-output-native-events";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AngularNoOutputNativeEvents {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Decorator(decorator) = node.kind() {
            if self.is_output_with_native_event(decorator) {
                ctx.diagnostic(angular_no_output_native_events_diagnostic(decorator.span));
            }
        }
    }
}

impl AngularNoOutputNativeEvents {
    fn is_output_with_native_event(&self, _decorator: &oxc_ast::ast::Decorator) -> bool {
        // Check for @Output() events that shadow native events
        false
    }
}

fn angular_no_output_native_events_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Output event shadows native event")
        .with_help("Use semantic event names that don't conflict with native events")
        .with_label(span)
}

impl EnhancedWasmRule for AngularNoOutputNativeEvents {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Avoid event names like 'click', 'focus', 'blur'".to_string(),
            "Use semantic names: 'itemSelected', 'formSubmitted'".to_string(),
            "Native event shadowing causes confusion".to_string(),
            "Follow Angular naming conventions for events".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angular_no_direct_dom_access_rule() {
        assert_eq!(AngularNoDirectDOMAccess::NAME, "angular-no-direct-dom-access");
        assert_eq!(AngularNoDirectDOMAccess::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_angular_require_ondestroy_interface_rule() {
        assert_eq!(AngularRequireOnDestroyInterface::NAME, "angular-require-ondestroy-interface");
        assert_eq!(AngularRequireOnDestroyInterface::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(AngularRequireOnDestroyInterface::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_angular_prefer_onpush_rule() {
        assert_eq!(AngularPreferOnPush::NAME, "angular-prefer-onpush");
        assert_eq!(AngularPreferOnPush::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_template_function_detection() {
        let rule = AngularNoFunctionsInTemplates;
        let template_with_function = oxc_ast::ast::StringLiteral {
            span: Span::default(),
            value: "{{ getValue() }}".into(),
        };
        assert!(rule.has_function_calls(&template_with_function));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = AngularNoUnsubscribedObservables;
        let diagnostic = angular_no_unsubscribed_observables_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("takeUntil"));
    }
}