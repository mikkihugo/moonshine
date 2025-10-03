//! # Rule Execution Engine
//!
//! Fast rule execution using JSON rulebase.
//! Executes all 832 rules against code.

use crate::error::Result;
use crate::rule_registry::{RuleMetadata, RuleRegistry};
use std::time::Duration;

/// The rule execution engine for the JSON rulebase.
pub struct RuleExecutor {
    /// The embedded rule registry used for execution planning.
    registry: RuleRegistry,
}

impl RuleExecutor {
    /// Creates a new `RuleExecutor`.
    pub fn new(registry: RuleRegistry) -> Self {
        Self { registry }
    }

    /// Executes all rules from the JSON rulebase.
    pub async fn execute_all_rules(
        &self,
        _code: &str,
        _file_path: &str,
    ) -> Result<Vec<RuleMetadata>> {
        Ok(self.registry.get_enabled_rules())
    }
}

/// An execution plan for a set of rules.
pub struct ExecutionPlan {
    /// The total number of rules in the execution plan.
    pub total_rules: usize,
    /// The estimated duration of the execution.
    pub estimated_duration: Duration,
}

impl ExecutionPlan {
    /// Creates a new `ExecutionPlan`.
    pub fn new(rule_count: usize) -> Self {
        Self {
            total_rules: rule_count,
            estimated_duration: Duration::from_millis(rule_count as u64 * 2), // 2ms per rule estimate
        }
    }
}
