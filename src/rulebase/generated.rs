//! Auto-generated rule definitions compiled into the binary.
//!
//! These statics are only available when the `embedded_rulebase` feature is
//! enabled. They allow the workflow to access rule metadata without
//! touching the filesystem at runtime.

#![cfg(feature = "embedded_rulebase")]

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RuleDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
    pub implementation: Implementation,
    pub cost: u32,
    #[serde(default)]
    pub autofix: bool,
    #[serde(default)]
    pub ai_enhanced: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Implementation {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub rule_name: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RulebaseWrapper {
    rulebase: RulebaseContent,
}

#[derive(Debug, Deserialize)]
struct RulebaseContent {
    static_rules: Vec<RuleDefinition>,
    behavioral_rules: Vec<RuleDefinition>,
    hybrid_rules: Vec<RuleDefinition>,
}

static RULEBASE: Lazy<RulebaseContent> = Lazy::new(|| {
    const JSON: &str = include_str!("../../rulebase/output/moonshine-rulebase-complete.json");
    serde_json::from_str::<RulebaseWrapper>(JSON).expect("Invalid rulebase JSON").rulebase
});

pub static STATIC_RULES: Lazy<&'static [RuleDefinition]> = Lazy::new(|| RULEBASE.static_rules.as_slice());

pub static BEHAVIORAL_RULES: Lazy<&'static [RuleDefinition]> = Lazy::new(|| RULEBASE.behavioral_rules.as_slice());

pub static HYBRID_RULES: Lazy<&'static [RuleDefinition]> = Lazy::new(|| RULEBASE.hybrid_rules.as_slice());

pub fn all_rules() -> impl Iterator<Item = &'static RuleDefinition> {
    STATIC_RULES.iter().chain(BEHAVIORAL_RULES.iter()).chain(HYBRID_RULES.iter())
}
