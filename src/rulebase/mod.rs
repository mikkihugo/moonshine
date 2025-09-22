//! # MoonShine RuleBase - Simple JSON Rule System
//!
//! Loads all 832 rules from generated JSON using serde.
//! Simple and efficient without KV complexity.
//!
//! @category rulebase
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 3.0.0

pub mod dynamic_rule_loader;
pub mod execution_engine;
pub mod rule_interface;

#[cfg(feature = "embedded_rulebase")]
pub mod generated;

// Re-exports
pub use dynamic_rule_loader::{RuleDefinition, RuleImplementation, RulebaseMetadata};
pub use execution_engine::{ExecutionPlan, RuleExecutor};
pub use rule_interface::{Rule, RuleCategory, RuleContext, RuleResult, RuleSeverity};

#[cfg(feature = "embedded_rulebase")]
pub use generated::{Implementation as GeneratedImplementation, RuleDefinition as GeneratedRuleDefinition};

/// Total rules in our rulebase
pub const TOTAL_RULES: usize = 832;

#[cfg(feature = "embedded_rulebase")]
pub fn iter_builtin_rules() -> impl Iterator<Item = &'static GeneratedRuleDefinition> {
    generated::all_rules()
}

// Rule execution is now coordinated through the central `RuleRegistry` and
// the workflow engine. This module keeps the shared data structures and JSON
// bindings while runtime orchestration lives in `rule_registry`.
