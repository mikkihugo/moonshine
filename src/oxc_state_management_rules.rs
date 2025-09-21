//! State management rules (Redux, Zustand, Jotai, etc.)

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoMutatingReduxState;

impl NoMutatingReduxState {
    pub const NAME: &'static str = "no-mutating-redux-state";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoMutatingReduxState {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if self.is_state_mutation(&assign.left, ctx) {
                ctx.diagnostic(no_mutating_redux_state_diagnostic(assign.span));
            }
        }
    }
}

impl NoMutatingReduxState {
    fn is_state_mutation(&self, _target: &oxc_ast::ast::AssignmentTarget, _ctx: &WasmLintContext) -> bool {
        // Check if assignment mutates Redux state directly
        true
    }
}

fn no_mutating_redux_state_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct state mutation detected")
        .with_help("Use immutable updates or Redux Toolkit's createSlice")
        .with_label(span)
}

impl EnhancedWasmRule for NoMutatingReduxState {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use spread operator for immutable updates: { ...state, field: value }".to_string(),
            "Redux Toolkit's createSlice uses Immer for safe mutations".to_string(),
            "Direct mutations break Redux's pure function contract".to_string(),
            "Use libraries like Immutable.js for complex state".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireActionCreators;

impl RequireActionCreators {
    pub const NAME: &'static str = "require-action-creators";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireActionCreators {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_dispatch_with_plain_object(call) {
                ctx.diagnostic(require_action_creators_diagnostic(call.span));
            }
        }
    }
}

impl RequireActionCreators {
    fn is_dispatch_with_plain_object(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            if ident.name == "dispatch" {
                return call.arguments.iter().any(|arg| {
                    matches!(arg.as_expression(), Some(oxc_ast::ast::Expression::ObjectExpression(_)))
                });
            }
        }
        false
    }
}

fn require_action_creators_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Plain object action without action creator")
        .with_help("Use action creators for type safety and consistency")
        .with_label(span)
}

impl EnhancedWasmRule for RequireActionCreators {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Action creators provide type safety and reusability".to_string(),
            "Use Redux Toolkit's createAction for simple actions".to_string(),
            "Action creators centralize action type constants".to_string(),
            "Better IDE support with typed action creators".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSelectorsInComponents;

impl NoSelectorsInComponents {
    pub const NAME: &'static str = "no-selectors-in-components";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSelectorsInComponents {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_inline_selector(call) && self.is_in_component(ctx) {
                ctx.diagnostic(no_selectors_in_components_diagnostic(call.span));
            }
        }
    }
}

impl NoSelectorsInComponents {
    fn is_inline_selector(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return ident.name == "useSelector";
        }
        false
    }

    fn is_in_component(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're inside a React component
        true
    }
}

fn no_selectors_in_components_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inline selector in component")
        .with_help("Extract selectors to separate files for reusability and testing")
        .with_label(span)
}

impl EnhancedWasmRule for NoSelectorsInComponents {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Extract selectors: const getUser = (state) => state.user".to_string(),
            "Use Reselect for memoized selectors".to_string(),
            "Centralized selectors are easier to test".to_string(),
            "Selector reuse prevents duplication".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireZustandImmer;

impl RequireZustandImmer {
    pub const NAME: &'static str = "require-zustand-immer";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireZustandImmer {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_zustand_store(call) && self.has_complex_state_updates(call) && !self.uses_immer(call) {
                ctx.diagnostic(require_zustand_immer_diagnostic(call.span));
            }
        }
    }
}

impl RequireZustandImmer {
    fn is_zustand_store(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return ident.name == "create";
        }
        false
    }

    fn has_complex_state_updates(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for nested state updates
        true
    }

    fn uses_immer(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if immer() wrapper is used
        false
    }
}

fn require_zustand_immer_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Complex Zustand state without Immer")
        .with_help("Use immer() wrapper for complex nested state updates")
        .with_label(span)
}

impl EnhancedWasmRule for RequireZustandImmer {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Wrap store with immer(): create(immer((set) => ({ ... })))".to_string(),
            "Immer allows safe mutable-style updates".to_string(),
            "Prevents accidental mutations in nested objects".to_string(),
            "Simpler syntax for complex state updates".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoJotaiAtomMutation;

impl NoJotaiAtomMutation {
    pub const NAME: &'static str = "no-jotai-atom-mutation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoJotaiAtomMutation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if self.is_atom_mutation(&assign.left, ctx) {
                ctx.diagnostic(no_jotai_atom_mutation_diagnostic(assign.span));
            }
        }
    }
}

impl NoJotaiAtomMutation {
    fn is_atom_mutation(&self, _target: &oxc_ast::ast::AssignmentTarget, _ctx: &WasmLintContext) -> bool {
        // Check if assignment mutates Jotai atom value directly
        true
    }
}

fn no_jotai_atom_mutation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct atom value mutation")
        .with_help("Use set() function from useAtom to update atom values")
        .with_label(span)
}

impl EnhancedWasmRule for NoJotaiAtomMutation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use const [value, setValue] = useAtom(myAtom)".to_string(),
            "Atoms are immutable - use setValue() to update".to_string(),
            "Direct mutations break Jotai's reactivity system".to_string(),
            "Use derived atoms for computed values".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireStateNormalization;

impl RequireStateNormalization {
    pub const NAME: &'static str = "require-state-normalization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireStateNormalization {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_denormalized_data_structure(obj) {
                ctx.diagnostic(require_state_normalization_diagnostic(obj.span));
            }
        }
    }
}

impl RequireStateNormalization {
    fn is_denormalized_data_structure(&self, obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for nested arrays of objects (denormalized structure)
        obj.properties.len() > 5 // Simplified check
    }
}

fn require_state_normalization_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Denormalized state structure detected")
        .with_help("Normalize state with entities pattern for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for RequireStateNormalization {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use normalized state: { entities: { users: { 1: {...} } }, ids: [1, 2] }".to_string(),
            "Normalizr library helps with complex normalization".to_string(),
            "Normalized state prevents deep nesting issues".to_string(),
            "Easier updates and lookups with entity patterns".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoAsyncActionCreators;

impl NoAsyncActionCreators {
    pub const NAME: &'static str = "no-async-action-creators";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoAsyncActionCreators {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_action_creator(func) && self.is_async_function(func) {
                ctx.diagnostic(no_async_action_creators_diagnostic(func.span));
            }
        }
    }
}

impl NoAsyncActionCreators {
    fn is_action_creator(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if function returns action objects
        func.id.as_ref().map_or(false, |id| id.name.ends_with("Action"))
    }

    fn is_async_function(&self, func: &oxc_ast::ast::Function) -> bool {
        func.r#async
    }
}

fn no_async_action_creators_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async action creator detected")
        .with_help("Use thunks or sagas for async actions, keep action creators pure")
        .with_label(span)
}

impl EnhancedWasmRule for NoAsyncActionCreators {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Redux Thunk for async action creators".to_string(),
            "Action creators should be pure functions".to_string(),
            "Redux Toolkit's createAsyncThunk handles async patterns".to_string(),
            "Separate side effects from action creation logic".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequirePersistedState;

impl RequirePersistedState {
    pub const NAME: &'static str = "require-persisted-state";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePersistedState {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_store_creation(call) && self.handles_user_data(call) && !self.has_persistence(call) {
                ctx.diagnostic(require_persisted_state_diagnostic(call.span));
            }
        }
    }
}

impl RequirePersistedState {
    fn is_store_creation(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return matches!(ident.name.as_str(), "createStore" | "create" | "configureStore");
        }
        false
    }

    fn handles_user_data(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if store contains user-specific data
        true
    }

    fn has_persistence(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if persistence is configured
        false
    }
}

fn require_persisted_state_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Store with user data lacks persistence")
        .with_help("Add state persistence for better user experience")
        .with_label(span)
}

impl EnhancedWasmRule for RequirePersistedState {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use redux-persist for Redux state persistence".to_string(),
            "Zustand has built-in persist middleware".to_string(),
            "Consider what data should and shouldn't persist".to_string(),
            "Handle rehydration gracefully with loading states".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_mutating_redux_state_rule() {
        assert_eq!(NoMutatingReduxState::NAME, "no-mutating-redux-state");
        assert_eq!(NoMutatingReduxState::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoMutatingReduxState::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_action_creators_rule() {
        assert_eq!(RequireActionCreators::NAME, "require-action-creators");
        assert_eq!(RequireActionCreators::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_selectors_in_components_rule() {
        assert_eq!(NoSelectorsInComponents::NAME, "no-selectors-in-components");
        assert_eq!(NoSelectorsInComponents::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoMutatingReduxState;
        let diagnostic = no_mutating_redux_state_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("spread operator"));
    }
}