//! Unified rule registry for all OXC-compatible WASM rules

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Re-export types from oxc_compatible_rules for easy access
pub use crate::oxc_compatible_rules::{WasmRule, WasmRuleCategory, WasmFixStatus, EnhancedWasmRule};

// Import all rule modules
use crate::oxc_rules_migration::*;
use crate::oxc_performance_rules::*;
use crate::oxc_string_rules::*;
use crate::oxc_conditional_rules::*;
use crate::oxc_object_rules::*;
use crate::oxc_function_rules::*;
use crate::oxc_variable_rules::*;
use crate::oxc_import_rules::*;
use crate::oxc_error_rules::*;
use crate::oxc_typescript_rules::*;
use crate::oxc_security_rules::*;
use crate::oxc_advanced_security_rules::*;
use crate::oxc_react_rules::*;
use crate::oxc_accessibility_rules::*;
use crate::oxc_es6_rules::*;
use crate::oxc_complexity_rules::*;
use crate::oxc_nodejs_rules::*;
use crate::oxc_async_rules::*;
use crate::oxc_jsx_advanced_rules::*;
use crate::oxc_bestpractices_rules::*;
use crate::oxc_css_rules::*;
use crate::oxc_testing_rules::*;
// Import specific structs from oxc_testing_framework_rules to avoid conflicts
use crate::oxc_testing_framework_rules::{
    NoJestTimeoutInTests, RequirePlaywrightWaits, NoCypressArbitraryWaits,
    RequireVitestAsyncUtils, NoHardcodedTestData, RequireTestIsolation,
    NoTestingLibraryQueryProblems, RequireSnapshotUpdates
};
use crate::oxc_documentation_rules::*;
use crate::oxc_advanced_performance_rules::*;
use crate::oxc_vue_rules::*;
use crate::oxc_angular_rules::*;
use crate::oxc_build_tool_rules::*;
use crate::oxc_build_tool_optimization_rules::*;
use crate::oxc_database_orm_rules::*;
use crate::oxc_database_optimization_rules::*;
use crate::oxc_monorepo_workspace_rules::*;
use crate::oxc_state_management_rules::*;
use crate::oxc_graphql_rules::*;
use crate::oxc_testing_framework_integration_rules::*;
use crate::oxc_devops_deployment_rules::*;
use crate::oxc_pwa_modern_web_rules::*;
use crate::oxc_microfrontend_rules::*;
use crate::oxc_edge_serverless_rules::*;
use crate::oxc_webrtc_realtime_rules::*;
use crate::oxc_web_payments_commerce_rules::*;
use crate::oxc_enterprise_architecture_rules::*;
use crate::oxc_accessibility_i18n_rules::*;
use crate::oxc_advanced_frameworks_rules::*;
use crate::oxc_cloud_native_rules::*;
use crate::oxc_api_integration_rules::*;
use crate::oxc_data_science_ml_rules::*;
use crate::oxc_blockchain_web3_rules::*;
use crate::oxc_performance_monitoring_rules::*;
use crate::oxc_performance_profiling_rules::*;
use crate::oxc_gaming_interactive_rules::*;
use crate::oxc_iot_embedded_rules::*;
use crate::oxc_ar_vr_development_rules::*;
use crate::oxc_advanced_typescript_rules::*;
use crate::oxc_functional_programming_rules::*;
use crate::oxc_design_systems_rules::*;
use crate::oxc_enterprise_patterns_rules::*;

/// Diagnostic result from running a WASM rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

/// AI-enhanced suggestion for code improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub rule_name: String,
    pub message: String,
    pub suggestion: String,
    pub confidence: f64,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// Registry for all available WASM-compatible OXC rules
#[derive(Debug, Default)]
pub struct UnifiedRuleRegistry {
    rules: HashMap<String, Box<dyn WasmRule>>,
    enhanced_rules: HashMap<String, Box<dyn EnhancedWasmRule>>,
    enabled_rules: HashMap<String, bool>,
}

impl UnifiedRuleRegistry {
    /// Create new rule registry with all available rules
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_all_rules();
        registry
    }

    /// Register all available OXC-compatible rules
    fn register_all_rules(&mut self) {
        // Minimal working set of rules for initial build
        // TODO: Systematically add verified rules from each module

        // OXC Complexity rules (verified to exist)
        self.register_rule(MaxComplexity {});

        // Testing framework rules (verified to exist)
        self.register_rule(NoJestTimeoutInTests {});
        self.register_rule(RequirePlaywrightWaits {});

        // End of minimal rule set for initial working build
        // TODO: Add more verified rules systematically after build succeeds
    }

    /// Register a single rule with the registry
    fn register_rule<T>(&mut self, rule: T)
    where
        T: WasmRule + EnhancedWasmRule + Clone + 'static
    {
        let name = rule.name().to_string();

        // Clone for enhanced rules registry
        let enhanced_rule = rule.clone();

        // Store both standard and enhanced versions
        self.rules.insert(name.clone(), Box::new(rule) as Box<dyn WasmRule>);
        self.enhanced_rules.insert(name.clone(), Box::new(enhanced_rule) as Box<dyn EnhancedWasmRule>);

        // Enable by default
        self.enabled_rules.insert(name, true);
    }

    /// Get all registered rule names
    pub fn get_all_rule_names(&self) -> Vec<&String> {
        self.rules.keys().collect()
    }

    /// Get rules by category
    pub fn get_rules_by_category(&self, category: WasmRuleCategory) -> Vec<&Box<dyn WasmRule>> {
        self.rules.values()
            .filter(|rule| rule.category() == category)
            .collect()
    }

    /// Get enabled rules only
    pub fn get_enabled_rules(&self) -> Vec<&Box<dyn WasmRule>> {
        self.rules.iter()
            .filter(|(name, _)| self.enabled_rules.get(*name).unwrap_or(&false))
            .map(|(_, rule)| rule)
            .collect()
    }

    /// Enable/disable a specific rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) {
        if self.rules.contains_key(rule_name) {
            self.enabled_rules.insert(rule_name.to_string(), enabled);
        }
    }

    /// Check if a rule is enabled
    pub fn is_rule_enabled(&self, rule_name: &str) -> bool {
        self.enabled_rules.get(rule_name).unwrap_or(&false)
    }

    /// Get rule by name
    pub fn get_rule(&self, rule_name: &str) -> Option<&Box<dyn WasmRule>> {
        self.rules.get(rule_name)
    }

    /// Get enhanced rule by name
    pub fn get_enhanced_rule(&self, rule_name: &str) -> Option<&Box<dyn EnhancedWasmRule>> {
        self.enhanced_rules.get(rule_name)
    }

    /// Get rule statistics
    pub fn get_statistics(&self) -> RuleRegistryStats {
        let total_rules = self.rules.len();
        let enabled_rules = self.enabled_rules.values().filter(|&&enabled| enabled).count();

        let mut category_counts = HashMap::new();
        for rule in self.rules.values() {
            *category_counts.entry(rule.category()).or_insert(0) += 1;
        }

        let mut fix_status_counts = HashMap::new();
        for rule in self.rules.values() {
            *fix_status_counts.entry(rule.fix_status()).or_insert(0) += 1;
        }

        RuleRegistryStats {
            total_rules,
            enabled_rules,
            category_counts,
            fix_status_counts,
        }
    }

    /// Configure rules from settings
    pub fn configure_from_settings(&mut self, settings: &RuleSettings) {
        // Enable/disable rules by category
        for (category, enabled) in &settings.categories {
            self.toggle_category(*category, *enabled);
        }

        // Enable/disable individual rules
        for (rule_name, enabled) in &settings.individual_rules {
            self.set_rule_enabled(rule_name, *enabled);
        }
    }

    /// Toggle entire category of rules
    pub fn toggle_category(&mut self, category: WasmRuleCategory, enabled: bool) {
        let rule_names: Vec<String> = self.rules.values()
            .filter(|rule| rule.category() == category)
            .map(|rule| rule.name().to_string())
            .collect();

        for rule_name in rule_names {
            self.set_rule_enabled(&rule_name, enabled);
        }
    }
}

/// Statistics about the rule registry
#[derive(Debug)]
pub struct RuleRegistryStats {
    pub total_rules: usize,
    pub enabled_rules: usize,
    pub category_counts: HashMap<WasmRuleCategory, usize>,
    pub fix_status_counts: HashMap<WasmFixStatus, usize>,
}

/// Configuration settings for rules
#[derive(Debug, Default)]
pub struct RuleSettings {
    pub categories: HashMap<WasmRuleCategory, bool>,
    pub individual_rules: HashMap<String, bool>,
}

impl RuleSettings {
    /// Create new settings with all categories enabled
    pub fn all_enabled() -> Self {
        let mut categories = HashMap::new();
        categories.insert(WasmRuleCategory::Correctness, true);
        categories.insert(WasmRuleCategory::Suspicious, true);
        categories.insert(WasmRuleCategory::Pedantic, true);
        categories.insert(WasmRuleCategory::Perf, true);
        categories.insert(WasmRuleCategory::Restriction, true);
        categories.insert(WasmRuleCategory::Style, true);
        categories.insert(WasmRuleCategory::Nursery, true);

        Self {
            categories,
            individual_rules: HashMap::new(),
        }
    }

    /// Create strict settings (only correctness and suspicious)
    pub fn strict() -> Self {
        let mut categories = HashMap::new();
        categories.insert(WasmRuleCategory::Correctness, true);
        categories.insert(WasmRuleCategory::Suspicious, true);
        categories.insert(WasmRuleCategory::Pedantic, false);
        categories.insert(WasmRuleCategory::Perf, false);
        categories.insert(WasmRuleCategory::Restriction, false);
        categories.insert(WasmRuleCategory::Style, false);
        categories.insert(WasmRuleCategory::Nursery, false);

        Self {
            categories,
            individual_rules: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = UnifiedRuleRegistry::new();
        assert!(registry.get_all_rule_names().len() > 0);
    }

    #[test]
    fn test_rule_categories() {
        let registry = UnifiedRuleRegistry::new();
        let correctness_rules = registry.get_rules_by_category(WasmRuleCategory::Correctness);
        assert!(correctness_rules.len() > 0);
    }

    #[test]
    fn test_rule_enabling() {
        let mut registry = UnifiedRuleRegistry::new();
        registry.set_rule_enabled("no-unused-vars", false);
        assert!(!registry.is_rule_enabled("no-unused-vars"));
    }

    #[test]
    fn test_statistics() {
        let registry = UnifiedRuleRegistry::new();
        let stats = registry.get_statistics();
        assert!(stats.total_rules > 0);
        assert!(stats.enabled_rules > 0);
    }

    #[test]
    fn test_settings_configuration() {
        let mut registry = UnifiedRuleRegistry::new();
        let settings = RuleSettings::strict();
        registry.configure_from_settings(&settings);

        let stats = registry.get_statistics();
        assert!(stats.enabled_rules > 0);
    }
}