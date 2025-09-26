//! Runtime registry that exposes the compiled Moon Shine rulebase.
//!
//! The rule definitions are generated into `rulebase/output/moonshine-rulebase-complete.json`
//! and embedded in the binary through the `embedded_rulebase` feature. This module provides
//! a lightweight interface for the workflow engine to query rule metadata, toggle rule
//! activation, and obtain simple statistics without needing the legacy hard-coded stacks.

use crate::error::Result;
// Legacy imports removed - using modern Biome + AI system
use crate::rule_types::{RuleCategory, RuleMetadata, RuleSeverity};
use crate::rulebase::RuleImplementation;
use crate::smart_rule_strategy::{get_core_rules, CoreStaticRule};
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
        self.loader.get_rule(rule_id).cloned()
    }

    /// Iterate over every rule definition.
    pub fn iter_rules(&self) -> impl Iterator<Item = RuleMetadata> + '_ {
        self.loader.get_all_rules().values().cloned()
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
            .filter(|rule| category.matches(&rule.category.as_str()))
            .cloned()
            .collect()
    }

    /// Return only the rules that are currently enabled according to the registry configuration.
    pub fn get_enabled_rules(&self) -> Vec<RuleMetadata> {
        self.loader
            .get_all_rules()
            .values()
            .filter(|rule| self.is_rule_enabled(&rule.id))
            .cloned()
            .collect()
    }

    /// Check if a rule is enabled (defaults to true for all rules).
    pub fn is_rule_enabled(&self, rule_id: &str) -> bool {
        self.enabled_rules.get(rule_id).copied().unwrap_or(true)
    }

    /// Enable or disable a specific rule.
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) {
        self.enabled_rules.insert(rule_id.to_string(), enabled);
    }

    /// Bulk enable/disable rules by category.
    pub fn toggle_category(&mut self, category: &RuleCategory, enabled: bool) {
        for rule in self.loader.get_all_rules().values() {
            if category.matches(&rule.category.as_str()) {
                self.enabled_rules.insert(rule.id.clone(), enabled);
            }
        }
    }

    /// Get count of rules by category.
    pub fn get_category_count(&self, category: &RuleCategory) -> usize {
        self.category_counts.get(category).copied().unwrap_or(0)
    }

    /// Get count of AI-enhanced rules.
    pub fn get_ai_enhanced_count(&self) -> usize {
        self.ai_enhanced_count
    }

    /// Get count of rules capable of automatic fixes.
    pub fn get_autofix_capable_count(&self) -> usize {
        self.autofix_capable_count
    }

    /// Get category counts map.
    pub fn category_counts(&self) -> &BTreeMap<RuleCategory, usize> {
        &self.category_counts
    }

    /// Rebuild internal caches for efficient lookups.
    fn rebuild_caches(&mut self) {
        let mut category_counts: BTreeMap<RuleCategory, usize> = BTreeMap::new();
        self.ai_enhanced_count = 0;
        self.autofix_capable_count = 0;

        for rule in self.loader.get_all_rules().values() {
            let category = RuleCategory::from(rule.category.as_str());
            *category_counts.entry(category).or_insert(0) += 1;

            if rule.ai_enhanced {
                self.ai_enhanced_count += 1;
            }

            if rule.fix_status == crate::rule_types::FixStatus::Autofix {
                self.autofix_capable_count += 1;
            }
        }

        self.category_counts = category_counts;
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

    /// Get basic rulebase statistics
    pub fn get_stats(&self) -> RuleRegistryStats {
        RuleRegistryStats {
            total_rules: crate::rulebase::TOTAL_RULES,
            static_rules: crate::rulebase::STATIC_RULES_COUNT,
            behavioral_rules: crate::rulebase::BEHAVIORAL_RULES_COUNT,
            hybrid_rules: crate::rulebase::HYBRID_RULES_COUNT,
            ai_enhanced_rules: self.ai_enhanced_count,
            autofix_capable_rules: self.autofix_capable_count,
            category_counts: self.category_counts.clone(),
        }
    }

    /// Configure registry from settings
    pub fn configure_from_settings(&mut self, settings: &RuleSettings) {
        for (category, enabled) in &settings.categories {
            self.toggle_category(category, *enabled);
        }

        for (rule_id, enabled) in &settings.individual_rules {
            self.set_rule_enabled(rule_id, *enabled);
        }
    }
}

/// Rule loader for embedded rulebase
#[derive(Debug)]
pub struct RuleLoader {
    rules: HashMap<String, RuleMetadata>,
    total_rules: usize,
}

impl RuleLoader {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "embedded_rulebase")]
        {
            use crate::rulebase::generated::all_rules;

            let mut rules = HashMap::new();

            // Load all rules from the embedded rulebase
            for rule_def in all_rules() {
                let rule_metadata = RuleMetadata {
                    id: rule_def.id.clone(),
                    name: rule_def.name.clone(),
                    description: rule_def.description.clone(),
                    category: RuleCategory::from(rule_def.category.as_str()),
                    severity: RuleSeverity::from(rule_def.severity.as_str()),
                    fix_status: if rule_def.autofix {
                        crate::rule_types::FixStatus::Autofix
                    } else {
                        crate::rule_types::FixStatus::Manual
                    },
                    ai_enhanced: rule_def.ai_enhanced,
                    cost: rule_def.cost,
                    tags: rule_def.tags.clone(),
                    dependencies: rule_def.dependencies.clone(),
                    implementation: RuleImplementation::from_rule_definition(rule_def),
                    config_schema: rule_def.config_schema.clone(),
                };

                rules.insert(rule_def.id.clone(), rule_metadata);
            }

            Ok(Self {
                rules,
                total_rules: rules.len(),
            })
        }

        #[cfg(not(feature = "embedded_rulebase"))]
        {
            // Fallback to placeholder rules when embedded_rulebase is disabled
            let mut rules = HashMap::new();

            rules.insert(
                "oxc:noUndeclaredVariables".to_string(),
                RuleMetadata {
                    id: "oxc:noUndeclaredVariables".to_string(),
                    name: "noUndeclaredVariables".to_string(),
                    description: "Disallow undeclared variables".to_string(),
                    category: RuleCategory::Correctness,
                    severity: RuleSeverity::Error,
                    fix_status: crate::rule_types::FixStatus::Autofix,
                    ai_enhanced: false,
                    cost: 1,
                    tags: vec!["oxc".to_string(), "correctness".to_string()],
                    dependencies: vec![],
                    implementation: RuleImplementation::OxcStatic {
                        rule_name: "noUndeclaredVariables".to_string(),
                    },
                    config_schema: None,
                },
            );

            Ok(Self { rules, total_rules: 1 })
        }
    }

    pub fn get_metadata(&self) -> RulebaseMetadata {
        RulebaseMetadata {
            total_rules: self.total_rules,
            static_rules: self.total_rules,
            behavioral_rules: 0,
            hybrid_rules: 0,
        }
    }

    pub fn get_all_rules(&self) -> &HashMap<String, RuleMetadata> {
        &self.rules
    }

    pub fn get_rule(&self, rule_id: &str) -> Option<&RuleMetadata> {
        self.rules.get(rule_id)
    }
}

/// Rulebase metadata structure
#[derive(Debug, Clone)]
pub struct RulebaseMetadata {
    pub total_rules: usize,
    pub static_rules: usize,
    pub behavioral_rules: usize,
    pub hybrid_rules: usize,
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

/// Configuration settings for rule registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSettings {
    pub categories: HashMap<RuleCategory, bool>,
    pub individual_rules: HashMap<String, bool>,
}

impl Default for RuleSettings {
    fn default() -> Self {
        let mut settings = Self {
            categories: HashMap::new(),
            individual_rules: HashMap::new(),
        };

        for category in RuleCategory::common_categories() {
            if category == RuleCategory::Correctness || category == RuleCategory::Security {
                continue;
            }
            settings.categories.insert(category, false);
        }

        // Enable security and correctness by default
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

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback to empty registry if loading fails
            Self {
                loader: RuleLoader::new().unwrap_or_else(|_| RuleLoader {
                    rules: HashMap::new(),
                    total_rules: 0,
                }),
                enabled_rules: HashMap::new(),
                category_counts: BTreeMap::new(),
                ai_enhanced_count: 0,
                autofix_capable_count: 0,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_rulebase() {
        let registry = RuleRegistry::new().expect("registry loads");
        let stats = registry.get_statistics();

        assert!(stats.total_rules >= 2, "expected at least 2 rules, got {}", stats.total_rules);
        assert_eq!(stats.total_rules, registry.all_rules().len());
        assert!(stats.ai_enhanced_rules <= stats.total_rules);
        assert!(stats.autofix_capable_rules <= stats.total_rules);
    }

    #[test]
    fn filters_rules_by_category() {
        let registry = RuleRegistry::new().expect("registry loads");
        let correctness_rules = registry.get_rules_by_category(&RuleCategory::Correctness);

        assert!(!correctness_rules.is_empty(), "correctness rules should not be empty");
        assert!(correctness_rules.iter().all(|rule| rule.category == RuleCategory::Correctness));
    }

    #[test]
    fn toggling_category_updates_enabled_state() {
        let mut registry = RuleRegistry::new().expect("registry loads");
        let correctness_rules = registry.get_rules_by_category(&RuleCategory::Correctness);
        assert!(!correctness_rules.is_empty());

        registry.toggle_category(&RuleCategory::Correctness, false);
        assert!(correctness_rules.iter().all(|rule| !registry.is_rule_enabled(&rule.id)));

        registry.toggle_category(&RuleCategory::Correctness, true);
        assert!(correctness_rules.iter().all(|rule| registry.is_rule_enabled(&rule.id)));
    }
}
