//! Rule-related types for the modern Biome + AI analysis system

use serde::{Deserialize, Serialize};

/// Rule severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
    Hint,
    Custom(String),
}

/// Rule categories for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    Accessibility,
    Complexity,
    Correctness,
    Performance,
    Security,
    Style,
}

/// Fix availability status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixStatus {
    Autofix,
    Manual,
    None,
}

/// Modern rule metadata for Biome + AI system
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
    pub implementation: crate::rulebase::RuleImplementation,
    pub config_schema: Option<String>,
}

/// Rule registry statistics
#[derive(Debug, Clone, Default)]
pub struct RuleRegistryStats {
    pub total_rules: usize,
    pub static_rules: usize,
    pub behavioral_rules: usize,
    pub hybrid_rules: usize,
}
