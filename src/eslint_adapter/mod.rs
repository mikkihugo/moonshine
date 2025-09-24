//! # ESLint Compatibility Layer
//!
//! This module provides ESLint rule compatibility for the Moon Shine WASM extension.
//! It bridges ESLint's rule API with our OXC-based execution engine, allowing us to
//! run ESLint-compatible rules in pure Rust without external processes.
//!
//! ## Architecture
//! - **Fast Path**: ESLint rules compiled to Rust at build time
//! - **Fallback Path**: External ESLint execution via Moon tasks
//! - **Context Mapping**: ESLint's context API mapped to our diagnostic system
//! - **AST Compatibility**: ESTree nodes mapped to OXC AST structures
//!
//! ## Performance Benefits
//! - Zero runtime parsing for compiled rules
//! - Single-pass AST traversal
//! - WASM-compatible execution
//! - Leverages existing OXC parser infrastructure

pub mod context;
pub mod visitor;
pub mod utils;
pub mod rule_mapping;
pub mod execution;

use crate::rule_registry::{RuleMetadata, RuleSeverity};
use crate::types::LintDiagnostic;
use oxc_ast::ast::Program;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ESLint rule execution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ESLintRuleStrategy {
    /// Rule compiled to native Rust (fast path)
    Native,
    /// Rule executed via external ESLint (fallback)
    External,
    /// Rule not available
    Unavailable,
}

/// ESLint rule compatibility information
#[derive(Debug, Clone)]
pub struct ESLintRuleInfo {
    pub eslint_id: String,
    pub description: String,
    pub category: String,
    pub severity: RuleSeverity,
    pub strategy: ESLintRuleStrategy,
    pub autofix: bool,
}

/// ESLint adapter for running ESLint-compatible rules in Rust
pub struct ESLintAdapter {
    rule_mappings: HashMap<String, ESLintRuleInfo>,
}

impl ESLintAdapter {
    pub fn new() -> Self {
        Self {
            rule_mappings: Self::initialize_rule_mappings(),
        }
    }

    /// Initialize mappings between ESLint rule names and our internal system
    fn initialize_rule_mappings() -> HashMap<String, ESLintRuleInfo> {
        let mut mappings = HashMap::new();

        // Common ESLint rules that we'll compile to Rust
        let common_rules = vec![
            ("no-console", "Security", RuleSeverity::Warning),
            ("no-debugger", "Security", RuleSeverity::Error),
            ("no-alert", "Security", RuleSeverity::Warning),
            ("no-eval", "Security", RuleSeverity::Error),
            ("no-implied-eval", "Security", RuleSeverity::Error),
            ("no-new-func", "Security", RuleSeverity::Error),
            ("no-script-url", "Security", RuleSeverity::Error),
            ("no-unused-vars", "Correctness", RuleSeverity::Warning),
            ("no-undef", "Correctness", RuleSeverity::Error),
            ("no-redeclare", "Correctness", RuleSeverity::Error),
            ("no-dupe-keys", "Correctness", RuleSeverity::Error),
            ("no-duplicate-case", "Correctness", RuleSeverity::Error),
            ("no-empty", "Style", RuleSeverity::Warning),
            ("no-empty-function", "Style", RuleSeverity::Warning),
            ("no-unreachable", "Correctness", RuleSeverity::Error),
            ("no-constant-condition", "Correctness", RuleSeverity::Warning),
            ("curly", "Style", RuleSeverity::Warning),
            ("eqeqeq", "Correctness", RuleSeverity::Warning),
            ("prefer-const", "Style", RuleSeverity::Warning),
            ("no-var", "Style", RuleSeverity::Warning),
        ];

        for (rule_id, category, severity) in common_rules {
            mappings.insert(rule_id.to_string(), ESLintRuleInfo {
                eslint_id: rule_id.to_string(),
                description: format!("ESLint rule: {}", rule_id),
                category: category.to_string(),
                severity,
                strategy: ESLintRuleStrategy::Native, // Will be implemented as native Rust
                autofix: false, // TODO: Implement autofix for specific rules
            });
        }

        mappings
    }

    /// Check if an ESLint rule is supported
    pub fn is_rule_supported(&self, rule_id: &str) -> bool {
        self.rule_mappings.contains_key(rule_id)
    }

    /// Get ESLint rule information
    pub fn get_rule_info(&self, rule_id: &str) -> Option<&ESLintRuleInfo> {
        self.rule_mappings.get(rule_id)
    }

    /// Get all supported ESLint rules
    pub fn get_supported_rules(&self) -> Vec<&ESLintRuleInfo> {
        self.rule_mappings.values().collect()
    }

    /// Convert ESLint rule configuration to our internal format
    pub fn convert_eslint_config(&self, eslint_config: &ESLintConfig) -> Vec<RuleMetadata> {
        let mut rules = Vec::new();

        for (rule_id, rule_level) in &eslint_config.rules {
            if let Some(rule_info) = self.get_rule_info(rule_id) {
                let severity = match rule_level {
                    ESLintRuleLevel::Off => continue, // Skip disabled rules
                    ESLintRuleLevel::Warn => RuleSeverity::Warning,
                    ESLintRuleLevel::Error => RuleSeverity::Error,
                };

                rules.push(RuleMetadata {
                    id: format!("eslint:{}", rule_id),
                    name: rule_info.eslint_id.clone(),
                    description: rule_info.description.clone(),
                    category: crate::rule_registry::RuleCategory::from(rule_info.category.as_str()),
                    severity,
                    fix_status: if rule_info.autofix {
                        crate::rule_registry::FixStatus::Autofix
                    } else {
                        crate::rule_registry::FixStatus::Manual
                    },
                    ai_enhanced: false,
                    cost: 1, // ESLint rules are generally low cost
                    tags: vec!["eslint".to_string()],
                    dependencies: vec![],
                    implementation: crate::rulebase::RuleImplementation::Static,
                    config_schema: None,
                });
            }
        }

        rules
    }

    /// Execute ESLint-compatible rules using our execution engine
    pub fn execute_rules<'a>(
        &self,
        rules: &[ESLintRuleInfo],
        program: &Program<'a>,
        semantic: &Semantic<'a>,
        source: &str,
    ) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for rule in rules {
            match rule.strategy {
                ESLintRuleStrategy::Native => {
                    // Execute native Rust implementation
                    let rule_diagnostics = self.execute_native_rule(rule, program, semantic, source);
                    diagnostics.extend(rule_diagnostics);
                }
                ESLintRuleStrategy::External => {
                    // TODO: Fallback to external ESLint execution
                    log::debug!("External ESLint execution not yet implemented for rule: {}", rule.eslint_id);
                }
                ESLintRuleStrategy::Unavailable => {
                    log::warn!("Rule {} is not available", rule.eslint_id);
                }
            }
        }

        diagnostics
    }

    /// Execute a native Rust implementation of an ESLint rule
    fn execute_native_rule<'a>(
        &self,
        rule: &ESLintRuleInfo,
        program: &Program<'a>,
        semantic: &Semantic<'a>,
        source: &str,
    ) -> Vec<LintDiagnostic> {
        // This will be expanded with actual rule implementations
        // For now, return empty to avoid compilation errors
        let _ = (rule, program, semantic, source);
        Vec::new()
    }
}

impl Default for ESLintAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// ESLint configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintConfig {
    pub rules: HashMap<String, ESLintRuleLevel>,
    pub env: Option<HashMap<String, bool>>,
    pub extends: Option<Vec<String>>,
    pub parser_options: Option<ESLintParserOptions>,
}

/// ESLint rule level (off/warn/error)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ESLintRuleLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

/// ESLint parser options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintParserOptions {
    pub ecma_version: Option<u32>,
    pub source_type: Option<String>,
    pub ecma_features: Option<HashMap<String, bool>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eslint_adapter_initialization() {
        let adapter = ESLintAdapter::new();
        assert!(!adapter.rule_mappings.is_empty());
        assert!(adapter.is_rule_supported("no-console"));
        assert!(adapter.is_rule_supported("no-debugger"));
        assert!(!adapter.is_rule_supported("non-existent-rule"));
    }

    #[test]
    fn test_eslint_config_conversion() {
        let adapter = ESLintAdapter::new();
        let mut rules = HashMap::new();
        rules.insert("no-console".to_string(), ESLintRuleLevel::Warn);
        rules.insert("no-debugger".to_string(), ESLintRuleLevel::Error);
        rules.insert("no-alert".to_string(), ESLintRuleLevel::Off);

        let config = ESLintConfig {
            rules,
            env: None,
            extends: None,
            parser_options: None,
        };

        let converted_rules = adapter.convert_eslint_config(&config);

        // Should have 2 rules (no-alert is disabled)
        assert_eq!(converted_rules.len(), 2);

        // Check that rules are properly converted
        let console_rule = converted_rules.iter().find(|r| r.name == "no-console");
        assert!(console_rule.is_some());
        assert_eq!(console_rule.unwrap().severity, RuleSeverity::Warning);
    }

    #[test]
    fn test_supported_rules_list() {
        let adapter = ESLintAdapter::new();
        let supported_rules = adapter.get_supported_rules();

        assert!(!supported_rules.is_empty());
        assert!(supported_rules.iter().any(|r| r.eslint_id == "no-console"));
        assert!(supported_rules.iter().any(|r| r.eslint_id == "no-debugger"));
    }
}