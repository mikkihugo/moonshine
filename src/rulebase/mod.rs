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

/// A dynamic loader for rules defined in JSON.
pub mod dynamic_rule_loader;
/// The execution engine for running rules against code.
pub mod execution_engine;
/// The interface for defining and interacting with rules.
pub mod rule_interface;

/// The generated module containing the built-in rules.
#[cfg(feature = "embedded_rulebase")]
pub mod generated;

// Re-exports
pub use dynamic_rule_loader::{RuleDefinition, RuleImplementation, RulebaseMetadata};
pub use execution_engine::{ExecutionPlan, RuleExecutor};
pub use rule_interface::{Rule, RuleCategory, RuleContext, RuleResult, RuleSeverity};

#[cfg(feature = "embedded_rulebase")]
pub use generated::{
    Implementation as GeneratedImplementation, RuleDefinition as GeneratedRuleDefinition,
};

/// The total number of rules in the rulebase.
pub const TOTAL_RULES: usize = 832;

/// Returns an iterator over the built-in rules.
///
/// This function is only available when the `embedded_rulebase` feature is enabled.
#[cfg(feature = "embedded_rulebase")]
pub fn iter_builtin_rules() -> impl Iterator<Item = &'static GeneratedRuleDefinition> {
    generated::all_rules()
}

// Rule execution is now coordinated through the central `RuleRegistry` and
// the workflow engine. This module keeps the shared data structures and JSON
// bindings while runtime orchestration lives in `rule_registry`.
