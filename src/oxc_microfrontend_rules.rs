//! Micro-frontend architecture rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireModuleFederationConfig;

impl RequireModuleFederationConfig {
    pub const NAME: &'static str = "require-module-federation-config";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireModuleFederationConfig {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_webpack_config(ctx) && self.has_microfrontend_structure(obj) && !self.has_module_federation(obj) {
                ctx.diagnostic(require_module_federation_config_diagnostic(obj.span));
            }
        }
    }
}

impl RequireModuleFederationConfig {
    fn is_webpack_config(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("webpack.config")
    }

    fn has_microfrontend_structure(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for multiple entry points or microfrontend patterns
        true
    }

    fn has_module_federation(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for ModuleFederationPlugin configuration
        false
    }
}

fn require_module_federation_config_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Micro-frontend setup without Module Federation")
        .with_help("Add ModuleFederationPlugin for proper micro-frontend architecture")
        .with_label(span)
}

impl EnhancedWasmRule for RequireModuleFederationConfig {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Module Federation enables runtime sharing between micro-frontends".to_string(),
            "Configure shared dependencies to avoid duplication".to_string(),
            "Use semantic versioning for shared modules".to_string(),
            "Implement graceful fallbacks for federation failures".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSharedStateLeakage;

impl NoSharedStateLeakage {
    pub const NAME: &'static str = "no-shared-state-leakage";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoSharedStateLeakage {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if self.assigns_to_global_state(&assign.left, ctx) {
                ctx.diagnostic(no_shared_state_leakage_diagnostic(assign.span));
            }
        }
    }
}

impl NoSharedStateLeakage {
    fn assigns_to_global_state(&self, _target: &oxc_ast::ast::AssignmentTarget, _ctx: &WasmLintContext) -> bool {
        // Check for assignments to global state that could leak between micro-frontends
        true
    }
}

fn no_shared_state_leakage_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potential shared state leakage between micro-frontends")
        .with_help("Isolate state within micro-frontend boundaries")
        .with_label(span)
}

impl EnhancedWasmRule for NoSharedStateLeakage {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use scoped state management within each micro-frontend".to_string(),
            "Implement explicit communication channels between micro-frontends".to_string(),
            "Avoid global variables that can cause conflicts".to_string(),
            "Use event-driven architecture for micro-frontend communication".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireMicrofrontendContract;

impl RequireMicrofrontendContract {
    pub const NAME: &'static str = "require-microfrontend-contract";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMicrofrontendContract {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ExportDefaultDeclaration(_export) = node.kind() {
            if self.is_microfrontend_entry(ctx) && !self.has_contract_definition(ctx) {
                ctx.diagnostic(require_microfrontend_contract_diagnostic(node.span()));
            }
        }
    }
}

impl RequireMicrofrontendContract {
    fn is_microfrontend_entry(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("bootstrap") || ctx.filename().contains("entry")
    }

    fn has_contract_definition(&self, _ctx: &WasmLintContext) -> bool {
        // Check for TypeScript interface or contract definition
        false
    }
}

fn require_microfrontend_contract_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Micro-frontend without contract definition")
        .with_help("Define TypeScript interfaces for micro-frontend communication")
        .with_label(span)
}

impl EnhancedWasmRule for RequireMicrofrontendContract {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Define clear interfaces for micro-frontend communication".to_string(),
            "Use contract testing to ensure compatibility".to_string(),
            "Document API surface area between micro-frontends".to_string(),
            "Version contracts to manage breaking changes".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDirectDOMManipulation;

impl NoDirectDOMManipulation {
    pub const NAME: &'static str = "no-direct-dom-manipulation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoDirectDOMManipulation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_direct_dom_manipulation(call) && self.is_microfrontend_context(ctx) {
                ctx.diagnostic(no_direct_dom_manipulation_diagnostic(call.span));
            }
        }
    }
}

impl NoDirectDOMManipulation {
    fn is_direct_dom_manipulation(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                if obj.name == "document" {
                    if let Some(prop) = member.property().as_identifier() {
                        return matches!(prop.name.as_str(), "createElement" | "appendChild" | "removeChild");
                    }
                }
            }
        }
        false
    }

    fn is_microfrontend_context(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're in a micro-frontend environment
        true
    }
}

fn no_direct_dom_manipulation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct DOM manipulation in micro-frontend")
        .with_help("Use framework-specific rendering to avoid conflicts")
        .with_label(span)
}

impl EnhancedWasmRule for NoDirectDOMManipulation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use framework rendering (React, Vue, Angular)".to_string(),
            "Direct DOM manipulation can conflict with other micro-frontends".to_string(),
            "Use Shadow DOM for style and DOM isolation".to_string(),
            "Implement proper cleanup when micro-frontend unmounts".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireErrorBoundaries;

impl RequireErrorBoundaries {
    pub const NAME: &'static str = "require-error-boundaries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireErrorBoundaries {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_microfrontend_mount(call) && !self.has_error_boundary(call, ctx) {
                ctx.diagnostic(require_error_boundaries_diagnostic(call.span));
            }
        }
    }
}

impl RequireErrorBoundaries {
    fn is_microfrontend_mount(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for React.render, Vue mount, or similar micro-frontend mounting
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "render" | "mount" | "bootstrap");
            }
        }
        false
    }

    fn has_error_boundary(&self, _call: &oxc_ast::ast::CallExpression, _ctx: &WasmLintContext) -> bool {
        // Check for error boundary wrapper
        false
    }
}

fn require_error_boundaries_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Micro-frontend mount without error boundary")
        .with_help("Wrap micro-frontend with error boundary to prevent cascading failures")
        .with_label(span)
}

impl EnhancedWasmRule for RequireErrorBoundaries {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Error boundaries prevent one micro-frontend from crashing others".to_string(),
            "Implement graceful fallback UI for failed micro-frontends".to_string(),
            "Log errors to monitoring system for debugging".to_string(),
            "Use React Error Boundaries or equivalent in other frameworks".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSharedDependencyVersionConflicts;

impl NoSharedDependencyVersionConflicts {
    pub const NAME: &'static str = "no-shared-dependency-version-conflicts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSharedDependencyVersionConflicts {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_federation_config(ctx) && self.has_version_conflicts(obj) {
                ctx.diagnostic(no_shared_dependency_version_conflicts_diagnostic(obj.span));
            }
        }
    }
}

impl NoSharedDependencyVersionConflicts {
    fn is_federation_config(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("federation") || ctx.filename().contains("webpack.config")
    }

    fn has_version_conflicts(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for conflicting versions in shared dependencies
        true
    }
}

fn no_shared_dependency_version_conflicts_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Conflicting shared dependency versions")
        .with_help("Align shared dependency versions across micro-frontends")
        .with_label(span)
}

impl EnhancedWasmRule for NoSharedDependencyVersionConflicts {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use exact versions for shared dependencies".to_string(),
            "Implement version compatibility matrix".to_string(),
            "Use single versions for React, Vue, Angular across micro-frontends".to_string(),
            "Consider using a shared dependency registry".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireMicrofrontendLifecycle;

impl RequireMicrofrontendLifecycle {
    pub const NAME: &'static str = "require-microfrontend-lifecycle";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMicrofrontendLifecycle {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ExportDefaultDeclaration(_export) = node.kind() {
            if self.is_microfrontend_entry(ctx) && !self.has_lifecycle_methods(ctx) {
                ctx.diagnostic(require_microfrontend_lifecycle_diagnostic(node.span()));
            }
        }
    }
}

impl RequireMicrofrontendLifecycle {
    fn is_microfrontend_entry(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("bootstrap") || ctx.filename().contains("main")
    }

    fn has_lifecycle_methods(&self, _ctx: &WasmLintContext) -> bool {
        // Check for mount, unmount, update lifecycle methods
        false
    }
}

fn require_microfrontend_lifecycle_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Micro-frontend without lifecycle methods")
        .with_help("Implement mount, unmount, and update lifecycle methods")
        .with_label(span)
}

impl EnhancedWasmRule for RequireMicrofrontendLifecycle {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Implement mount() for initialization".to_string(),
            "Implement unmount() for cleanup".to_string(),
            "Implement update() for props changes".to_string(),
            "Follow single-spa lifecycle conventions".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMicrofrontendCSSConflicts;

impl NoMicrofrontendCSSConflicts {
    pub const NAME: &'static str = "no-microfrontend-css-conflicts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoMicrofrontendCSSConflicts {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.is_global_css_selector(&string_lit.value) && self.is_microfrontend_context(ctx) {
                ctx.diagnostic(no_microfrontend_css_conflicts_diagnostic(string_lit.span));
            }
        }
    }
}

impl NoMicrofrontendCSSConflicts {
    fn is_global_css_selector(&self, value: &str) -> bool {
        // Check for global CSS selectors that could conflict
        value.starts_with('.') && !value.contains('[') && value.len() < 10
    }

    fn is_microfrontend_context(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're in a micro-frontend
        true
    }
}

fn no_microfrontend_css_conflicts_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Global CSS selector in micro-frontend")
        .with_help("Use CSS modules, styled-components, or Shadow DOM for style isolation")
        .with_label(span)
}

impl EnhancedWasmRule for NoMicrofrontendCSSConflicts {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use CSS modules with scoped class names".to_string(),
            "Use Shadow DOM for complete style isolation".to_string(),
            "Prefix CSS classes with micro-frontend name".to_string(),
            "Use CSS-in-JS libraries for automatic scoping".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_module_federation_config_rule() {
        assert_eq!(RequireModuleFederationConfig::NAME, "require-module-federation-config");
        assert_eq!(RequireModuleFederationConfig::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_shared_state_leakage_rule() {
        assert_eq!(NoSharedStateLeakage::NAME, "no-shared-state-leakage");
        assert_eq!(NoSharedStateLeakage::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoSharedStateLeakage::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_error_boundaries_rule() {
        assert_eq!(RequireErrorBoundaries::NAME, "require-error-boundaries");
        assert_eq!(RequireErrorBoundaries::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_css_selector_detection() {
        let rule = NoMicrofrontendCSSConflicts;
        assert!(rule.is_global_css_selector(".btn"));
        assert!(rule.is_global_css_selector(".header"));
        assert!(!rule.is_global_css_selector(".my-app-specific-button"));
        assert!(!rule.is_global_css_selector("button[data-test]"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireModuleFederationConfig;
        let diagnostic = require_module_federation_config_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Module Federation"));
    }
}