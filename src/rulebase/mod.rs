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
pub mod generated; // Auto-generated rule definitions
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
    /// Custom command-based rule
    Command { command: String, args: Vec<String> },
    /// Code-based rule implementation
    Code { code: String },
}

impl RuleImplementation {
    /// Create RuleImplementation from generated rule definition
    pub fn from_rule_definition(rule_def: &crate::rulebase::generated::RuleDefinition) -> Self {
        match rule_def.implementation.kind.as_str() {
            "oxc_static" => RuleImplementation::OxcStatic {
                rule_name: rule_def.implementation.rule_name.clone().unwrap_or_default(),
            },
            "ai_behavioral" => RuleImplementation::AiBehavioral {
                pattern_type: rule_def.implementation.rule_name.clone().unwrap_or_default(),
            },
            "hybrid" => RuleImplementation::Hybrid {
                oxc_rule: rule_def.implementation.rule_name.clone().unwrap_or_default(),
                ai_pattern: "default".to_string(),
            },
            "command" => RuleImplementation::Command {
                command: rule_def.implementation.command.clone().unwrap_or_default(),
                args: rule_def.implementation.args.clone().unwrap_or_default(),
            },
            "code" => RuleImplementation::Code {
                code: rule_def.implementation.code.clone().unwrap_or_default(),
            },
            _ => RuleImplementation::OxcStatic {
                rule_name: rule_def.implementation.rule_name.clone().unwrap_or_default(),
            },
        }
    }
}

// Re-exports
pub use execution_engine::{ExecutionPlan, RuleExecutionContext, RuleExecutionOutcome, RuleExecutor};
pub use presets::{available_presets, get_preset, has_preset};
pub use rule_interface::{Rule, RuleCategory, RuleContext, RuleResult, RuleSeverity};

// Legacy rule definitions - removed in favor of Biome + AI system
// pub use generated::{Implementation as GeneratedImplementation, RuleDefinition as GeneratedRuleDefinition};

/// Modern rulebase constants - updated to match generated rulebase
pub const TOTAL_RULES: usize = 832; // Total rules in generated rulebase
pub const STATIC_RULES_COUNT: usize = 582; // Static analysis rules
pub const BEHAVIORAL_RULES_COUNT: usize = 200; // AI behavioral patterns
pub const HYBRID_RULES_COUNT: usize = 50; // Hybrid rules

// Rule execution is now coordinated through the central `RuleRegistry` and
// the workflow engine. This module keeps the shared data structures and JSON
// bindings while runtime orchestration lives in `rule_registry`.
