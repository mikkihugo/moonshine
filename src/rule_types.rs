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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum RuleCategory {
    Accessibility,
    Complexity,
    Correctness,
    Documentation,
    Maintainability,
    Observability,
    Performance,
    Reliability,
    Security,
    Style,
    Testing,
    Unknown(String),
}

impl RuleCategory {
    /// Convert category to string representation
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

    /// Check if category matches a string (case-insensitive)
    pub fn matches(&self, raw_category: &str) -> bool {
        match self {
            RuleCategory::Unknown(expected) => expected.eq_ignore_ascii_case(raw_category),
            _ => self.as_str().eq_ignore_ascii_case(raw_category),
        }
    }

    /// Get common categories used in default configurations
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
            RuleCategory::Observability,
            RuleCategory::Reliability,
        ]
    }
}

impl From<&str> for RuleCategory {
    fn from(raw_category: &str) -> Self {
        match raw_category.to_lowercase().as_str() {
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
