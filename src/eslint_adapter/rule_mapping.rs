//! # ESLint Rule Mapping
//!
//! This module provides mappings between ESLint rules and our internal rule system.
//! It handles rule ID translation, configuration mapping, and determines whether
//! a rule should use the native Rust implementation or fall back to external ESLint.

use super::{ESLintRuleInfo, ESLintRuleStrategy};
use crate::rule_registry::{RuleCategory, RuleMetadata, RuleSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for ESLint rule mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintRuleMappingConfig {
    /// Rules that have native Rust implementations
    pub native_rules: HashMap<String, NativeRuleMapping>,
    /// Rules that should use external ESLint
    pub external_rules: HashMap<String, ExternalRuleMapping>,
    /// Rules that are disabled/unavailable
    pub disabled_rules: HashMap<String, String>, // rule_id -> reason
}

/// Mapping information for native Rust rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeRuleMapping {
    pub internal_id: String,
    pub category: String,
    pub default_severity: String,
    pub description: String,
    pub supports_autofix: bool,
    pub performance_cost: u32,
}

/// Mapping information for external ESLint rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalRuleMapping {
    pub eslint_plugin: Option<String>,
    pub category: String,
    pub default_severity: String,
    pub description: String,
    pub reason_for_external: String,
}

/// ESLint rule mapper handles translation between ESLint and internal rules
pub struct ESLintRuleMapper {
    config: ESLintRuleMappingConfig,
}

impl ESLintRuleMapper {
    pub fn new() -> Self {
        Self {
            config: Self::create_default_config(),
        }
    }

    pub fn with_config(config: ESLintRuleMappingConfig) -> Self {
        Self { config }
    }

    /// Create default mapping configuration
    fn create_default_config() -> ESLintRuleMappingConfig {
        let mut native_rules = HashMap::new();
        let mut external_rules = HashMap::new();
        let disabled_rules = HashMap::new();

        // Core ESLint rules with native implementations
        let native_rule_specs = vec![
            ("no-console", "Security", "warn", "Disallow console statements", false, 1),
            ("no-debugger", "Security", "error", "Disallow debugger statements", false, 1),
            ("no-alert", "Security", "warn", "Disallow alert/confirm/prompt", false, 1),
            ("no-eval", "Security", "error", "Disallow eval() usage", false, 1),
            ("no-implied-eval", "Security", "error", "Disallow implied eval", false, 1),
            ("no-new-func", "Security", "error", "Disallow Function constructor", false, 1),
            ("no-script-url", "Security", "error", "Disallow script URLs", false, 1),
            ("no-empty", "Style", "warn", "Disallow empty block statements", false, 1),
            ("no-empty-function", "Style", "warn", "Disallow empty functions", false, 1),
            ("no-unreachable", "Correctness", "error", "Disallow unreachable code", false, 1),
            ("no-constant-condition", "Correctness", "warn", "Disallow constant conditions", false, 1),
            ("no-dupe-keys", "Correctness", "error", "Disallow duplicate object keys", false, 1),
            ("no-duplicate-case", "Correctness", "error", "Disallow duplicate case labels", false, 1),
            ("curly", "Style", "warn", "Enforce consistent brace style", true, 1),
            ("eqeqeq", "Correctness", "warn", "Require === and !==", true, 1),
        ];

        for (rule_id, category, severity, description, autofix, cost) in native_rule_specs {
            native_rules.insert(
                rule_id.to_string(),
                NativeRuleMapping {
                    internal_id: format!("eslint_{}", rule_id.replace('-', "_")),
                    category: category.to_string(),
                    default_severity: severity.to_string(),
                    description: description.to_string(),
                    supports_autofix: autofix,
                    performance_cost: cost,
                },
            );
        }

        // Complex ESLint rules that need external execution
        let external_rule_specs = vec![
            ("no-unused-vars", "Correctness", "warn", "Requires complex scope analysis"),
            ("no-undef", "Correctness", "error", "Requires full semantic analysis"),
            ("no-redeclare", "Correctness", "error", "Requires scope tracking"),
            ("prefer-const", "Style", "warn", "Requires data flow analysis"),
            ("no-var", "Style", "warn", "Complex binding analysis"),
            ("complexity", "Complexity", "warn", "Requires complex cyclomatic calculation"),
            ("max-depth", "Complexity", "warn", "Requires nesting analysis"),
            ("max-params", "Complexity", "warn", "Function signature analysis"),
        ];

        for (rule_id, category, severity, reason) in external_rule_specs {
            external_rules.insert(
                rule_id.to_string(),
                ExternalRuleMapping {
                    eslint_plugin: None,
                    category: category.to_string(),
                    default_severity: severity.to_string(),
                    description: format!("ESLint rule: {}", rule_id),
                    reason_for_external: reason.to_string(),
                },
            );
        }

        ESLintRuleMappingConfig {
            native_rules,
            external_rules,
            disabled_rules,
        }
    }

    /// Map ESLint rule to internal rule info
    pub fn map_eslint_rule(&self, eslint_rule_id: &str) -> Option<ESLintRuleInfo> {
        // Check native rules first
        if let Some(native_mapping) = self.config.native_rules.get(eslint_rule_id) {
            return Some(ESLintRuleInfo {
                eslint_id: eslint_rule_id.to_string(),
                description: native_mapping.description.clone(),
                category: native_mapping.category.clone(),
                severity: self.parse_severity(&native_mapping.default_severity),
                strategy: ESLintRuleStrategy::Native,
                autofix: native_mapping.supports_autofix,
            });
        }

        // Check external rules
        if let Some(external_mapping) = self.config.external_rules.get(eslint_rule_id) {
            return Some(ESLintRuleInfo {
                eslint_id: eslint_rule_id.to_string(),
                description: external_mapping.description.clone(),
                category: external_mapping.category.clone(),
                severity: self.parse_severity(&external_mapping.default_severity),
                strategy: ESLintRuleStrategy::External,
                autofix: false, // External rules don't support autofix in our system yet
            });
        }

        // Check disabled rules
        if self.config.disabled_rules.contains_key(eslint_rule_id) {
            return Some(ESLintRuleInfo {
                eslint_id: eslint_rule_id.to_string(),
                description: format!("Disabled rule: {}", eslint_rule_id),
                category: "Disabled".to_string(),
                severity: RuleSeverity::Info,
                strategy: ESLintRuleStrategy::Unavailable,
                autofix: false,
            });
        }

        None
    }

    /// Convert ESLint rule config to internal rule metadata
    pub fn convert_to_internal_rule(&self, eslint_rule_id: &str, severity: RuleSeverity) -> Option<RuleMetadata> {
        if let Some(rule_info) = self.map_eslint_rule(eslint_rule_id) {
            let native_mapping = self.config.native_rules.get(eslint_rule_id);

            Some(RuleMetadata {
                id: format!("eslint:{}", eslint_rule_id),
                name: eslint_rule_id.to_string(),
                description: rule_info.description,
                category: RuleCategory::from(rule_info.category.as_str()),
                severity,
                fix_status: if rule_info.autofix {
                    crate::rule_registry::FixStatus::Autofix
                } else {
                    crate::rule_registry::FixStatus::Manual
                },
                ai_enhanced: false,
                cost: native_mapping.map(|m| m.performance_cost).unwrap_or(1),
                tags: vec!["eslint".to_string(), rule_info.category.to_lowercase()],
                dependencies: vec![],
                implementation: match rule_info.strategy {
                    ESLintRuleStrategy::Native => crate::rulebase::RuleImplementation::Static,
                    ESLintRuleStrategy::External => crate::rulebase::RuleImplementation::External,
                    ESLintRuleStrategy::Unavailable => crate::rulebase::RuleImplementation::Static,
                },
                config_schema: None,
            })
        } else {
            None
        }
    }

    /// Get all native ESLint rules
    pub fn get_native_rules(&self) -> Vec<String> {
        self.config.native_rules.keys().cloned().collect()
    }

    /// Get all external ESLint rules
    pub fn get_external_rules(&self) -> Vec<String> {
        self.config.external_rules.keys().cloned().collect()
    }

    /// Check if a rule has native implementation
    pub fn has_native_implementation(&self, rule_id: &str) -> bool {
        self.config.native_rules.contains_key(rule_id)
    }

    /// Check if a rule needs external execution
    pub fn needs_external_execution(&self, rule_id: &str) -> bool {
        self.config.external_rules.contains_key(rule_id)
    }

    /// Get performance cost for a rule
    pub fn get_rule_cost(&self, rule_id: &str) -> u32 {
        if let Some(native) = self.config.native_rules.get(rule_id) {
            native.performance_cost
        } else if self.config.external_rules.contains_key(rule_id) {
            5 // External rules are more expensive
        } else {
            1 // Default cost
        }
    }

    /// Parse severity string to RuleSeverity enum
    fn parse_severity(&self, severity_str: &str) -> RuleSeverity {
        match severity_str.to_lowercase().as_str() {
            "error" => RuleSeverity::Error,
            "warn" | "warning" => RuleSeverity::Warning,
            "info" => RuleSeverity::Info,
            "hint" => RuleSeverity::Hint,
            _ => RuleSeverity::Warning,
        }
    }

    /// Get rule mapping statistics
    pub fn get_mapping_stats(&self) -> ESLintMappingStats {
        ESLintMappingStats {
            total_mapped_rules: self.config.native_rules.len() + self.config.external_rules.len(),
            native_rules: self.config.native_rules.len(),
            external_rules: self.config.external_rules.len(),
            disabled_rules: self.config.disabled_rules.len(),
            rules_with_autofix: self.config.native_rules.values().filter(|r| r.supports_autofix).count(),
        }
    }
}

impl Default for ESLintRuleMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about ESLint rule mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintMappingStats {
    pub total_mapped_rules: usize,
    pub native_rules: usize,
    pub external_rules: usize,
    pub disabled_rules: usize,
    pub rules_with_autofix: usize,
}

/// Popular ESLint rule presets
pub struct ESLintPresets;

impl ESLintPresets {
    /// Get recommended ESLint rules (equivalent to eslint:recommended)
    pub fn recommended() -> Vec<&'static str> {
        vec![
            "no-console",
            "no-debugger",
            "no-alert",
            "no-eval",
            "no-empty",
            "no-unreachable",
            "no-constant-condition",
            "no-dupe-keys",
            "no-duplicate-case",
            "curly",
            "eqeqeq",
        ]
    }

    /// Get strict ESLint rules for production code
    pub fn strict() -> Vec<&'static str> {
        let mut rules = Self::recommended();
        rules.extend(vec!["no-implied-eval", "no-new-func", "no-script-url", "no-empty-function"]);
        rules
    }

    /// Get rules that require external execution
    pub fn external_only() -> Vec<&'static str> {
        vec![
            "no-unused-vars",
            "no-undef",
            "no-redeclare",
            "prefer-const",
            "no-var",
            "complexity",
            "max-depth",
            "max-params",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_mapper_initialization() {
        let mapper = ESLintRuleMapper::new();
        let stats = mapper.get_mapping_stats();

        assert!(stats.total_mapped_rules > 0);
        assert!(stats.native_rules > 0);
        assert!(stats.external_rules > 0);
    }

    #[test]
    fn test_native_rule_mapping() {
        let mapper = ESLintRuleMapper::new();

        let rule_info = mapper.map_eslint_rule("no-console");
        assert!(rule_info.is_some());

        let rule = rule_info.unwrap();
        assert_eq!(rule.eslint_id, "no-console");
        assert_eq!(rule.strategy, ESLintRuleStrategy::Native);
    }

    #[test]
    fn test_external_rule_mapping() {
        let mapper = ESLintRuleMapper::new();

        let rule_info = mapper.map_eslint_rule("no-unused-vars");
        assert!(rule_info.is_some());

        let rule = rule_info.unwrap();
        assert_eq!(rule.eslint_id, "no-unused-vars");
        assert_eq!(rule.strategy, ESLintRuleStrategy::External);
    }

    #[test]
    fn test_unknown_rule_mapping() {
        let mapper = ESLintRuleMapper::new();

        let rule_info = mapper.map_eslint_rule("unknown-rule");
        assert!(rule_info.is_none());
    }

    #[test]
    fn test_rule_conversion() {
        let mapper = ESLintRuleMapper::new();

        let internal_rule = mapper.convert_to_internal_rule("no-console", RuleSeverity::Error);
        assert!(internal_rule.is_some());

        let rule = internal_rule.unwrap();
        assert_eq!(rule.name, "no-console");
        assert_eq!(rule.severity, RuleSeverity::Error);
        assert!(rule.id.starts_with("eslint:"));
    }

    #[test]
    fn test_rule_cost_calculation() {
        let mapper = ESLintRuleMapper::new();

        let native_cost = mapper.get_rule_cost("no-console");
        let external_cost = mapper.get_rule_cost("no-unused-vars");
        let unknown_cost = mapper.get_rule_cost("unknown-rule");

        assert_eq!(native_cost, 1);
        assert_eq!(external_cost, 5);
        assert_eq!(unknown_cost, 1);
    }

    #[test]
    fn test_eslint_presets() {
        let recommended = ESLintPresets::recommended();
        let strict = ESLintPresets::strict();
        let external = ESLintPresets::external_only();

        assert!(!recommended.is_empty());
        assert!(strict.len() > recommended.len());
        assert!(!external.is_empty());

        assert!(recommended.contains(&"no-console"));
        assert!(strict.contains(&"no-console"));
        assert!(external.contains(&"no-unused-vars"));
    }
}
