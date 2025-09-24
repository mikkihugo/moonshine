//! Compile-time rule definitions - zero runtime parsing!
//!
//! Rules are converted from JSON to native Rust const arrays at build time.
//! This provides instant access with zero parsing overhead.

use crate::rule_registry::{RuleCategory, RuleSeverity};

/// Compile-time rule definition - can be used in const contexts
#[derive(Debug, Clone)]
pub struct CompiledRuleDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: RuleCategory,
    pub severity: RuleSeverity,
    pub cost: u32,
    pub autofix: bool,
    pub implementation_type: &'static str,
    pub rule_name: &'static str,
}

// Include the auto-generated const arrays
include!(concat!(env!("OUT_DIR"), "/compiled_rules.rs"));

/// Get all rules as a single iterator - zero allocation
pub fn all_rules_iter() -> impl Iterator<Item = &'static CompiledRuleDefinition> {
    STATIC_RULES.iter().chain(BEHAVIORAL_RULES.iter()).chain(HYBRID_RULES.iter())
}

/// Get rules by category - zero allocation
pub fn rules_by_category(category: RuleCategory) -> impl Iterator<Item = &'static CompiledRuleDefinition> {
    all_rules_iter().filter(move |rule| rule.category == category)
}

/// Get rule by ID - O(n) but still very fast for 832 rules
pub fn find_rule_by_id(id: &str) -> Option<&'static CompiledRuleDefinition> {
    all_rules_iter().find(|rule| rule.id == id)
}

/// Get rules within cost threshold - for performance budgeting
pub fn rules_within_cost(max_cost: u32) -> impl Iterator<Item = &'static CompiledRuleDefinition> {
    all_rules_iter().filter(move |rule| rule.cost <= max_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_rules_loaded() {
        assert_eq!(STATIC_RULES.len(), STATIC_RULES_COUNT);
        assert_eq!(BEHAVIORAL_RULES.len(), BEHAVIORAL_RULES_COUNT);
        assert_eq!(HYBRID_RULES.len(), HYBRID_RULES_COUNT);

        let total_loaded = STATIC_RULES.len() + BEHAVIORAL_RULES.len() + HYBRID_RULES.len();
        assert_eq!(total_loaded, TOTAL_RULES);
    }

    #[test]
    fn test_rule_access() {
        // Should find at least some rules
        assert!(!STATIC_RULES.is_empty());

        // Test iterator works
        let count = all_rules_iter().count();
        assert_eq!(count, TOTAL_RULES);

        // Test category filtering
        let security_rules: Vec<_> = rules_by_category(RuleCategory::Security).collect();
        assert!(!security_rules.is_empty());
    }

    #[test]
    fn test_rule_lookup() {
        // Test finding by ID (assuming first rule exists)
        if let Some(first_rule) = STATIC_RULES.first() {
            let found = find_rule_by_id(first_rule.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().id, first_rule.id);
        }
    }

    #[test]
    fn test_cost_filtering() {
        let low_cost_rules: Vec<_> = rules_within_cost(5).collect();
        for rule in low_cost_rules {
            assert!(rule.cost <= 5);
        }
    }
}
