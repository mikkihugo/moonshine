//! # MoonShine RuleBase - Modern OXC + AI System
//!
//! Unified rule system combining OXC's 570+ static analysis rules
//! with AI-powered behavioral pattern detection. No legacy JSON loading.
//!
//! @category rulebase
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 4.0.0

// pub mod biome_rules; // Removed - replaced with OXC integration
pub mod execution_engine;
pub mod presets;
pub mod rule_interface;

// Modern rule implementation types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleImplementation {
    /// OXC static analysis rule
    OxcStatic { rule_name: String },
    /// AI behavioral pattern detection
    AiBehavioral { pattern_type: String },
    /// Hybrid OXC + AI analysis
    Hybrid { oxc_rule: String, ai_pattern: String },
}

// Re-exports
pub use execution_engine::{ExecutionPlan, RuleExecutionContext, RuleExecutionOutcome, RuleExecutor};
pub use presets::{available_presets, get_preset, has_preset};
pub use rule_interface::{Rule, RuleCategory, RuleContext, RuleResult, RuleSeverity};

// Legacy rule definitions - removed in favor of Biome + AI system
// pub use generated::{Implementation as GeneratedImplementation, RuleDefinition as GeneratedRuleDefinition};

/// Modern rulebase constants
pub const TOTAL_RULES: usize = 190; // Biome rules + AI patterns
pub const STATIC_RULES_COUNT: usize = 190; // Biome static rules
pub const BEHAVIORAL_RULES_COUNT: usize = 50; // AI behavioral patterns
pub const HYBRID_RULES_COUNT: usize = 20; // Hybrid rules

// Rule execution is now coordinated through the central `RuleRegistry` and
// the workflow engine. This module keeps the shared data structures and JSON
// bindings while runtime orchestration lives in `rule_registry`.
