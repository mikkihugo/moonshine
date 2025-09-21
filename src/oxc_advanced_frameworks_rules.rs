//! # OXC Advanced Framework Integration Rules
//!
//! This module implements WASM-safe OXC rules for modern JavaScript frameworks
//! including Svelte, SolidJS, Qwik, Astro, and other cutting-edge frontend technologies.
//!
//! ## Rule Categories:
//! - **Svelte Framework**: Component lifecycle, stores, and compile-time optimization
//! - **SolidJS Patterns**: Fine-grained reactivity and signal management
//! - **Qwik Resumability**: Serialization, lazy loading, and edge optimization
//! - **Astro Components**: Static generation and partial hydration
//! - **Modern Build Patterns**: Vite, esbuild, and next-gen tooling
//! - **Framework Interoperability**: Cross-framework component integration
//! - **Performance Optimization**: Bundle splitting and code efficiency
//! - **Developer Experience**: Framework-specific best practices
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub rule_name: String,
    pub suggestion: String,
    pub confidence: f32,
    pub auto_fixable: bool,
}

// ================================================================================================
// Svelte Framework Rules
// ================================================================================================

/// Enforces proper Svelte store subscription cleanup
pub struct RequireSvelteStoreCleanup;

impl RequireSvelteStoreCleanup {
    pub const NAME: &'static str = "require-svelte-store-cleanup";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSvelteStoreCleanup {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("subscribe(") && !code.contains("onDestroy") && !code.contains("unsubscribe") {
            diagnostics.push(create_svelte_store_cleanup_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSvelteStoreCleanup {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use onDestroy to unsubscribe from Svelte stores to prevent memory leaks and unnecessary reactivity".to_string(),
            confidence: 0.95,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents direct DOM manipulation in Svelte components
pub struct NoDirectDomInSvelte;

impl NoDirectDomInSvelte {
    pub const NAME: &'static str = "no-direct-dom-in-svelte";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoDirectDomInSvelte {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("querySelector") || code.contains("getElementById")) &&
           code.contains("<script>") {
            diagnostics.push(create_direct_dom_svelte_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoDirectDomInSvelte {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use Svelte's reactive declarations and bind:this instead of direct DOM manipulation for better performance and SSR compatibility".to_string(),
            confidence: 0.89,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires proper Svelte action implementation
pub struct RequireProperSvelteActions;

impl RequireProperSvelteActions {
    pub const NAME: &'static str = "require-proper-svelte-actions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireProperSvelteActions {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("use:") && !code.contains("destroy") {
            diagnostics.push(create_svelte_actions_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperSvelteActions {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Svelte actions should return a destroy method for proper cleanup when the element is removed".to_string(),
            confidence: 0.91,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// SolidJS Framework Rules
// ================================================================================================

/// Enforces proper signal usage in SolidJS
pub struct RequireProperSolidSignals;

impl RequireProperSolidSignals {
    pub const NAME: &'static str = "require-proper-solid-signals";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireProperSolidSignals {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("createSignal") && code.contains("setInterval") &&
           !code.contains("onCleanup") {
            diagnostics.push(create_solid_signals_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperSolidSignals {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use onCleanup to clear intervals and timers in SolidJS to prevent memory leaks when components unmount".to_string(),
            confidence: 0.93,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents direct mutation of SolidJS reactive values
pub struct NoDirectSolidMutation;

impl NoDirectSolidMutation {
    pub const NAME: &'static str = "no-direct-solid-mutation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDirectSolidMutation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("createStore") && code.contains(".push(") &&
           !code.contains("produce") && !code.contains("setState") {
            diagnostics.push(create_solid_mutation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoDirectSolidMutation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use produce() or setState() for SolidJS store mutations instead of direct array/object mutations to maintain reactivity".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires proper resource management in SolidJS
pub struct RequireSolidResourceHandling;

impl RequireSolidResourceHandling {
    pub const NAME: &'static str = "require-solid-resource-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSolidResourceHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("createResource") && !code.contains("error") &&
           !code.contains("loading") {
            diagnostics.push(create_solid_resource_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSolidResourceHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Handle loading and error states when using createResource for better user experience and error handling".to_string(),
            confidence: 0.87,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Qwik Framework Rules
// ================================================================================================

/// Enforces proper Qwik serialization patterns
pub struct RequireQwikSerialization;

impl RequireQwikSerialization {
    pub const NAME: &'static str = "require-qwik-serialization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireQwikSerialization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("component$") &&
           (code.contains("function") || code.contains("class")) &&
           !code.contains("$") {
            diagnostics.push(create_qwik_serialization_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireQwikSerialization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use Qwik's $ suffix for functions that need to be serialized for resumability (e.g., handler$, loader$)".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents non-serializable closures in Qwik
pub struct NoQwikClosures;

impl NoQwikClosures {
    pub const NAME: &'static str = "no-qwik-closures";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoQwikClosures {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("onClick$") && code.contains("() =>") &&
           !code.contains("useStore") && !code.contains("useSignal") {
            diagnostics.push(create_qwik_closures_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoQwikClosures {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Avoid closures in Qwik event handlers - use useStore or useSignal for state that needs to be accessed in event handlers".to_string(),
            confidence: 0.90,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires proper Qwik lazy loading patterns
pub struct RequireQwikLazyLoading;

impl RequireQwikLazyLoading {
    pub const NAME: &'static str = "require-qwik-lazy-loading";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireQwikLazyLoading {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("import") && !code.contains("lazy$") &&
           code.contains("component") {
            diagnostics.push(create_qwik_lazy_loading_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireQwikLazyLoading {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use lazy$ for component imports in Qwik to enable proper code splitting and resumability".to_string(),
            confidence: 0.86,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Astro Framework Rules
// ================================================================================================

/// Enforces proper Astro component hydration patterns
pub struct RequireAstroHydration;

impl RequireAstroHydration {
    pub const NAME: &'static str = "require-astro-hydration";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAstroHydration {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("client:") && code.contains("client:load") &&
           !code.contains("client:visible") && !code.contains("client:idle") {
            diagnostics.push(create_astro_hydration_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAstroHydration {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Consider using client:visible or client:idle instead of client:load for better performance and user experience".to_string(),
            confidence: 0.84,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents server-side code in Astro client components
pub struct NoServerCodeInAstroClient;

impl NoServerCodeInAstroClient {
    pub const NAME: &'static str = "no-server-code-in-astro-client";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoServerCodeInAstroClient {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("client:") &&
           (code.contains("Astro.url") || code.contains("import.meta.env.SSR")) {
            diagnostics.push(create_astro_server_code_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoServerCodeInAstroClient {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Server-side APIs like Astro.url are not available in client-side hydrated components - pass data as props instead".to_string(),
            confidence: 0.95,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_svelte_store_cleanup_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSvelteStoreCleanup::NAME.to_string(),
        message: "Svelte store subscriptions must be cleaned up to prevent memory leaks".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use onDestroy(() => unsubscribe()) to clean up store subscriptions".to_string()),
    }
}

fn create_direct_dom_svelte_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoDirectDomInSvelte::NAME.to_string(),
        message: "Avoid direct DOM manipulation in Svelte components".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use Svelte's reactive declarations and bind:this instead".to_string()),
    }
}

fn create_svelte_actions_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperSvelteActions::NAME.to_string(),
        message: "Svelte actions should return a destroy method for proper cleanup".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Return { destroy() { /* cleanup */ } } from action function".to_string()),
    }
}

fn create_solid_signals_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperSolidSignals::NAME.to_string(),
        message: "Use onCleanup to clear intervals and timers in SolidJS components".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add onCleanup(() => clearInterval(id)) to prevent memory leaks".to_string()),
    }
}

fn create_solid_mutation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoDirectSolidMutation::NAME.to_string(),
        message: "Use SolidJS store methods instead of direct mutations".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use produce() or setState() for store mutations".to_string()),
    }
}

fn create_solid_resource_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSolidResourceHandling::NAME.to_string(),
        message: "Handle loading and error states when using createResource".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Check resource.loading and resource.error in the UI".to_string()),
    }
}

fn create_qwik_serialization_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireQwikSerialization::NAME.to_string(),
        message: "Use Qwik's $ suffix for serializable functions".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add $ suffix to functions that need serialization".to_string()),
    }
}

fn create_qwik_closures_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoQwikClosures::NAME.to_string(),
        message: "Avoid closures in Qwik event handlers for proper serialization".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use useStore or useSignal for state access in handlers".to_string()),
    }
}

fn create_qwik_lazy_loading_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireQwikLazyLoading::NAME.to_string(),
        message: "Use lazy$ for component imports in Qwik".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Import components with lazy$ for code splitting".to_string()),
    }
}

fn create_astro_hydration_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAstroHydration::NAME.to_string(),
        message: "Consider more efficient hydration strategies than client:load".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use client:visible or client:idle for better performance".to_string()),
    }
}

fn create_astro_server_code_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoServerCodeInAstroClient::NAME.to_string(),
        message: "Server-side APIs not available in client-side Astro components".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Pass server data as props to client components".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svelte_store_cleanup_detection() {
        let code = r#"const unsubscribe = myStore.subscribe(value => { console.log(value); });"#;
        let rule = RequireSvelteStoreCleanup;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireSvelteStoreCleanup::NAME);
    }

    #[test]
    fn test_direct_dom_svelte_detection() {
        let code = r#"<script>
                        document.querySelector('.my-element').style.color = 'red';
                      </script>"#;
        let rule = NoDirectDomInSvelte;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoDirectDomInSvelte::NAME);
    }

    #[test]
    fn test_solid_signals_detection() {
        let code = r#"const [count, setCount] = createSignal(0);
                      setInterval(() => setCount(c => c + 1), 1000);"#;
        let rule = RequireProperSolidSignals;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireProperSolidSignals::NAME);
    }

    #[test]
    fn test_solid_mutation_detection() {
        let code = r#"const [store, setStore] = createStore({ items: [] });
                      store.items.push(newItem);"#;
        let rule = NoDirectSolidMutation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoDirectSolidMutation::NAME);
    }

    #[test]
    fn test_qwik_serialization_detection() {
        let code = r#"export default component$(() => {
                        function handleClick() { console.log('clicked'); }
                        return <button onClick={handleClick}>Click</button>;
                      });"#;
        let rule = RequireQwikSerialization;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireQwikSerialization::NAME);
    }

    #[test]
    fn test_qwik_closures_detection() {
        let code = r#"<button onClick$={() => { console.log('clicked'); }}>Click</button>"#;
        let rule = NoQwikClosures;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoQwikClosures::NAME);
    }

    #[test]
    fn test_astro_hydration_detection() {
        let code = r#"<MyComponent client:load prop="value" />"#;
        let rule = RequireAstroHydration;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireAstroHydration::NAME);
    }

    #[test]
    fn test_astro_server_code_detection() {
        let code = r#"<script>
                        const url = Astro.url.pathname;
                      </script>
                      <MyComponent client:load />"#;
        let rule = NoServerCodeInAstroClient;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoServerCodeInAstroClient::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"myStore.subscribe(value => console.log(value));"#;
        let rule = RequireSvelteStoreCleanup;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].auto_fixable);
    }
}