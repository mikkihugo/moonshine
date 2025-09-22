//! Runtime registry that exposes the compiled Moon Shine rulebase.
//!
//! The rule definitions are generated into `rulebase/output/moonshine-rulebase-complete.json`
//! and embedded in the binary through the `embedded_rulebase` feature. This module provides
//! a lightweight interface for the workflow engine to query rule metadata, toggle rule
//! activation, and obtain simple statistics without needing the legacy hard-coded stacks.

use crate::error::Result;
use crate::rulebase::dynamic_rule_loader::RuleLoader;
use crate::rulebase::{RuleDefinition as RulebaseDefinition, RuleImplementation, RulebaseMetadata};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Primary entry point for accessing rule metadata inside the WASM runtime.
#[derive(Debug)]
pub struct RuleRegistry {
    loader: RuleLoader,
    enabled_rules: HashMap<String, bool>,
    category_counts: BTreeMap<RuleCategory, usize>,
    ai_enhanced_count: usize,
    autofix_capable_count: usize,
}

impl RuleRegistry {
    /// Build a registry backed by the embedded JSON rulebase.
    pub fn new() -> Result<Self> {
        let loader = RuleLoader::new()?;
        Self::from_loader(loader)
    }

    /// Rebuild the registry using a pre-loaded rule loader (useful for tests).
    pub fn from_loader(loader: RuleLoader) -> Result<Self> {
        let mut registry = Self {
            loader,
            enabled_rules: HashMap::new(),
            category_counts: BTreeMap::new(),
            ai_enhanced_count: 0,
            autofix_capable_count: 0,
        };
        registry.rebuild_caches();
        Ok(registry)
    }

    /// Reload rule definitions from the embedded JSON bundle.
    pub fn refresh(&mut self) -> Result<()> {
        self.loader = RuleLoader::new()?;
        self.enabled_rules.clear();
        self.rebuild_caches();
        Ok(())
    }

    /// Legacy helper retained for compatibility with previous code paths.
    pub fn load_from_json_rulebase(&mut self) -> Result<()> {
        self.refresh()
    }

    /// Total number of rules currently available.
    pub fn total_rules(&self) -> usize {
        self.loader.get_metadata().total_rules
    }

    /// Return all rule identifiers in deterministic order.
    pub fn get_all_rule_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.loader.get_all_rules().keys().cloned().collect();
        names.sort();
        names
    }

    /// Fetch a single rule by identifier.
    pub fn get_rule(&self, rule_id: &str) -> Option<RuleMetadata> {
        self.loader.get_rule(rule_id).map(RuleMetadata::from)
    }

    /// Iterate over every rule definition.
    pub fn iter_rules(&self) -> impl Iterator<Item = RuleMetadata> + '_ {
        self.loader.get_all_rules().values().map(RuleMetadata::from)
    }

    /// Materialise all rule metadata into a vector.
    pub fn all_rules(&self) -> Vec<RuleMetadata> {
        self.iter_rules().collect()
    }

    /// Return rules that belong to the requested category.
    pub fn get_rules_by_category(&self, category: &RuleCategory) -> Vec<RuleMetadata> {
        self.loader
            .get_all_rules()
            .values()
            .filter(|rule| category.matches(&rule.category))
            .map(RuleMetadata::from)
            .collect()
    }

    /// Return only the rules that are currently enabled according to the registry configuration.
    pub fn get_enabled_rules(&self) -> Vec<RuleMetadata> {
        self.loader
            .get_all_rules()
            .values()
            .filter(|rule| self.is_rule_enabled(&rule.id))
            .map(RuleMetadata::from)
            .collect()
    }

    /// Enable or disable a specific rule by identifier.
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) {
        if self.loader.has_rule(rule_id) {
            self.enabled_rules.insert(rule_id.to_owned(), enabled);
        }
    }

    /// Whether the rule is currently enabled (defaults to enabled).
    pub fn is_rule_enabled(&self, rule_id: &str) -> bool {
        self.enabled_rules.get(rule_id).copied().unwrap_or(true)
    }

    /// Apply configuration from user-provided settings.
    pub fn configure_from_settings(&mut self, settings: &RuleSettings) {
        for (category, enabled) in &settings.categories {
            self.toggle_category(category, *enabled);
        }

        for (rule_id, enabled) in &settings.individual_rules {
            self.set_rule_enabled(rule_id, *enabled);
        }
    }

    /// Bulk toggle of rules belonging to a specific category.
    pub fn toggle_category(&mut self, category: &RuleCategory, enabled: bool) {
        for rule in self.loader.get_all_rules().values() {
            if category.matches(&rule.category) {
                self.enabled_rules.insert(rule.id.clone(), enabled);
            }
        }
    }

    /// Retrieve summary statistics about the registry contents.
    pub fn get_statistics(&self) -> RuleRegistryStats {
        let metadata = self.loader.get_metadata().clone();
        RuleRegistryStats {
            total_rules: metadata.total_rules,
            static_rules: metadata.static_rules,
            behavioral_rules: metadata.behavioral_rules,
            hybrid_rules: metadata.hybrid_rules,
            ai_enhanced_rules: self.ai_enhanced_count,
            autofix_capable_rules: self.autofix_capable_count,
            category_counts: self.category_counts.clone(),
        }
    }

    /// Underlying rulebase metadata (version, generation timestamp, …).
    pub fn rulebase_metadata(&self) -> &RulebaseMetadata {
        self.loader.get_metadata()
    }

    /// Cached category counts for quick diagnostics.
    pub fn category_counts(&self) -> &BTreeMap<RuleCategory, usize> {
        &self.category_counts
    }

    fn rebuild_caches(&mut self) {
        let mut category_counts: BTreeMap<RuleCategory, usize> = BTreeMap::new();
        let mut ai_enhanced = 0usize;
        let mut autofix_capable = 0usize;

        for rule in self.loader.get_all_rules().values() {
            let category = RuleCategory::from(rule.category.as_str());
            *category_counts.entry(category).or_insert(0) += 1;

            if rule.ai_enhanced {
                ai_enhanced += 1;
            }

            if rule.autofix {
                autofix_capable += 1;
            }
        }

        self.category_counts = category_counts;
        self.ai_enhanced_count = ai_enhanced;
        self.autofix_capable_count = autofix_capable;
    }
}

/// Lightweight metadata describing a single rule from the rulebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: RuleCategory,
    pub severity: RuleSeverity,
    pub fix_status: FixStatus,
    pub ai_enhanced: bool,
    pub cost: u32,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub implementation: RuleImplementation,
    pub config_schema: Option<serde_json::Value>,
}

impl RuleMetadata {
    /// Helper for checking whether the rule supports automatic fixes.
    pub fn is_autofix_capable(&self) -> bool {
        matches!(self.fix_status, FixStatus::Autofix)
    }
}

impl From<&RulebaseDefinition> for RuleMetadata {
    fn from(value: &RulebaseDefinition) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            category: RuleCategory::from(value.category.as_str()),
            severity: RuleSeverity::from(value.severity.as_str()),
            fix_status: FixStatus::from(value.autofix),
            ai_enhanced: value.ai_enhanced,
            cost: value.cost,
            tags: value.tags.clone(),
            dependencies: value.dependencies.clone(),
            implementation: value.implementation.clone(),
            config_schema: value.config_schema.clone(),
        }
    }
}

/// Canonical rule categories exposed by the registry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    Security,
    Performance,
    Correctness,
    Style,
    Maintainability,
    Testing,
    Documentation,
    Accessibility,
    Complexity,
    Observability,
    Reliability,
    Unknown(String),
}

impl RuleCategory {
    pub fn as_str(&self) -> &str {
        match self {
            RuleCategory::Security => "Security",
            RuleCategory::Performance => "Performance",
            RuleCategory::Correctness => "Correctness",
            RuleCategory::Style => "Style",
            RuleCategory::Maintainability => "Maintainability",
            RuleCategory::Testing => "Testing",
            RuleCategory::Documentation => "Documentation",
            RuleCategory::Accessibility => "Accessibility",
            RuleCategory::Complexity => "Complexity",
            RuleCategory::Observability => "Observability",
            RuleCategory::Reliability => "Reliability",
            RuleCategory::Unknown(value) => value,
        }
    }

    pub fn matches(&self, raw_category: &str) -> bool {
        match self {
            RuleCategory::Unknown(expected) => expected.eq_ignore_ascii_case(raw_category),
            other => other.as_str().eq_ignore_ascii_case(raw_category),
        }
    }

    pub fn common_categories() -> Vec<RuleCategory> {
        vec![
            RuleCategory::Security,
            RuleCategory::Performance,
            RuleCategory::Correctness,
            RuleCategory::Style,
            RuleCategory::Maintainability,
            RuleCategory::Testing,
            RuleCategory::Documentation,
            RuleCategory::Accessibility,
            RuleCategory::Complexity,
        ]
    }
}

impl From<&str> for RuleCategory {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "security" => RuleCategory::Security,
            "performance" => RuleCategory::Performance,
            "correctness" => RuleCategory::Correctness,
            "style" => RuleCategory::Style,
            "maintainability" => RuleCategory::Maintainability,
            "testing" => RuleCategory::Testing,
            "documentation" => RuleCategory::Documentation,
            "accessibility" => RuleCategory::Accessibility,
            "complexity" => RuleCategory::Complexity,
            "observability" => RuleCategory::Observability,
            "reliability" => RuleCategory::Reliability,
            other => RuleCategory::Unknown(other.to_string()),
        }
    }
}

/// Severity attached to a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
    Hint,
    Custom(String),
}

impl RuleSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            RuleSeverity::Error => "Error",
            RuleSeverity::Warning => "Warning",
            RuleSeverity::Info => "Info",
            RuleSeverity::Hint => "Hint",
            RuleSeverity::Custom(value) => value,
        }
    }
}

impl From<&str> for RuleSeverity {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "error" => RuleSeverity::Error,
            "warning" => RuleSeverity::Warning,
            "warn" => RuleSeverity::Warning,
            "info" => RuleSeverity::Info,
            "hint" => RuleSeverity::Hint,
            other => RuleSeverity::Custom(other.to_string()),
        }
    }
}

/// Whether a rule provides an automatic fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixStatus {
    Autofix,
    Manual,
}

impl From<bool> for FixStatus {
    fn from(value: bool) -> Self {
        if value {
            FixStatus::Autofix
        } else {
            FixStatus::Manual
        }
    }
}

/// Aggregate statistics collected from the rule registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleRegistryStats {
    pub total_rules: usize,
    pub static_rules: usize,
    pub behavioral_rules: usize,
    pub hybrid_rules: usize,
    pub ai_enhanced_rules: usize,
    pub autofix_capable_rules: usize,
    pub category_counts: BTreeMap<RuleCategory, usize>,
}

/// User-facing configuration for enabling/disabling rules.
#[derive(Debug, Clone, Default)]
pub struct RuleSettings {
    pub categories: HashMap<RuleCategory, bool>,
    pub individual_rules: HashMap<String, bool>,
}

impl RuleSettings {
    pub fn all_enabled() -> Self {
        let mut settings = Self::default();
        for category in RuleCategory::common_categories() {
            settings.categories.insert(category, true);
        }
        settings
    }

    pub fn strict() -> Self {
        let mut settings = Self::default();
        settings.categories.insert(RuleCategory::Correctness, true);
        settings.categories.insert(RuleCategory::Security, true);

        for category in RuleCategory::common_categories() {
            if category == RuleCategory::Correctness || category == RuleCategory::Security {
                continue;
            }
            settings.categories.insert(category, false);
        }

        settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_rulebase() {
        let registry = RuleRegistry::new().expect("registry loads");
        let stats = registry.get_statistics();

        assert!(stats.total_rules >= 800, "expected at least 800 rules, got {}", stats.total_rules);
        assert_eq!(stats.total_rules, registry.all_rules().len());
        assert!(stats.ai_enhanced_rules <= stats.total_rules);
        assert!(stats.autofix_capable_rules <= stats.total_rules);
    }

    #[test]
    fn filters_rules_by_category() {
        let registry = RuleRegistry::new().expect("registry loads");
        let security_rules = registry.get_rules_by_category(&RuleCategory::Security);

        assert!(!security_rules.is_empty(), "security rules should not be empty");
        assert!(security_rules.iter().all(|rule| rule.category == RuleCategory::Security));
    }

    #[test]
    fn toggling_category_updates_enabled_state() {
        let mut registry = RuleRegistry::new().expect("registry loads");
        let security_rules = registry.get_rules_by_category(&RuleCategory::Security);
        assert!(!security_rules.is_empty());

        registry.toggle_category(&RuleCategory::Security, false);
        assert!(security_rules.iter().all(|rule| !registry.is_rule_enabled(&rule.id)));

        registry.toggle_category(&RuleCategory::Security, true);
        assert!(security_rules.iter().all(|rule| registry.is_rule_enabled(&rule.id)));
    }
}
