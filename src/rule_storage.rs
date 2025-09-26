//! # ESLint Rule Storage (WASM-safe)
//!
//! Lightweight, in-memory storage for ESLint rule configurations. The implementation
//! is designed to run inside the WASM extension without relying on external key-value
//! databases. Rules can be imported from or exported to `.eslintrc.json`, and multiple
//! named rulesets are supported.
//!
//! @category rule-storage
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// ESLint rule configuration with severity and options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub enabled: bool,
    pub severity: RuleSeverity,
    pub options: Option<serde_json::Value>,
    pub description: String,
    pub category: RuleCategory,
    pub fixable: bool,
    pub recommended: bool,
}

/// Rule severity levels (ESLint compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleSeverity {
    Off,     // 0 - Rule is disabled
    Warning, // 1 - Rule is warning
    Error,   // 2 - Rule is error
}

/// Rule categories for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    BestPractices,
    PossibleErrors,
    CodeStyle,
    TypeScript,
    React,
    Security,
    Performance,
    Accessibility,
}

/// Complete rule set configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub name: String,
    pub description: String,
    pub extends: Vec<String>, // Base configurations to extend
    pub rules: HashMap<String, RuleConfig>,
    pub env: HashMap<String, bool>,     // Environment settings
    pub globals: HashMap<String, bool>, // Global variables
    pub parser_options: ParserOptions,
}

/// Parser configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserOptions {
    pub ecma_version: u32,
    pub source_type: String, // "module" or "script"
    pub ecma_features: HashMap<String, bool>,
}

/// WASM-safe rule storage backed by an in-memory map
pub struct RuleStorage {
    current_ruleset: String,
    rulesets: HashMap<String, RuleSet>,
}

impl RuleStorage {
    /// Create new rule storage. `storage_path` is ignored for now but retained for
    /// API compatibility in case persistence is added later.
    pub fn new(_storage_path: Option<&Path>) -> Result<Self> {
        let mut storage = Self {
            current_ruleset: "default".to_string(),
            rulesets: HashMap::new(),
        };

        storage.initialize_default_rules()?;

        Ok(storage)
    }

    /// Get current rule set
    pub fn get_current_ruleset(&self) -> Result<RuleSet> {
        self.get_ruleset(&self.current_ruleset)
    }

    /// Get specific rule set by name
    pub fn get_ruleset(&self, name: &str) -> Result<RuleSet> {
        self.rulesets
            .get(name)
            .cloned()
            .ok_or_else(|| Error::Storage {
                message: format!("Rule set '{}' not found", name),
            })
    }

    /// Save rule set
    pub fn save_ruleset(&mut self, ruleset: &RuleSet) -> Result<()> {
        self.rulesets.insert(ruleset.name.clone(), ruleset.clone());
        self.current_ruleset = ruleset.name.clone();
        Ok(())
    }

    /// Set current active rule set
    pub fn set_current_ruleset(&mut self, name: &str) -> Result<()> {
        // Verify the ruleset exists
        if !self.rulesets.contains_key(name) {
            return Err(Error::Storage {
                message: format!("Rule set '{}' not found", name),
            });
        }

        self.current_ruleset = name.to_string();

        Ok(())
    }

    /// List all available rule sets
    pub fn list_rulesets(&self) -> Result<Vec<String>> {
        Ok(self.rulesets.keys().cloned().collect())
    }

    /// Update a specific rule in the current ruleset
    pub fn update_rule(&mut self, rule_name: &str, config: RuleConfig) -> Result<()> {
        let ruleset = self.get_ruleset_mut(&self.current_ruleset)?;
        ruleset.rules.insert(rule_name.to_string(), config);
        Ok(())
    }

    /// Enable/disable a rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) -> Result<()> {
        let ruleset = self.get_ruleset_mut(&self.current_ruleset)?;
        let rule = ruleset.rules.get_mut(rule_name).ok_or_else(|| Error::Storage {
            message: format!("Rule '{}' not found", rule_name),
        })?;
        rule.enabled = enabled;
        Ok(())
    }

    /// Set rule severity
    pub fn set_rule_severity(&mut self, rule_name: &str, severity: RuleSeverity) -> Result<()> {
        let ruleset = self.get_ruleset_mut(&self.current_ruleset)?;
        let rule = ruleset.rules.get_mut(rule_name).ok_or_else(|| Error::Storage {
            message: format!("Rule '{}' not found", rule_name),
        })?;
        rule.enabled = !matches!(severity, RuleSeverity::Off);
        rule.severity = severity;
        Ok(())
    }

    /// Import ESLint configuration from JSON (.eslintrc.json compatible)
    pub fn import_eslintrc(&mut self, json_config: &str, ruleset_name: &str) -> Result<()> {
        let eslint_config: serde_json::Value = serde_json::from_str(json_config).map_err(|e| Error::Storage {
            message: format!("Invalid ESLint JSON: {}", e),
        })?;

        let mut ruleset = RuleSet {
            name: ruleset_name.to_string(),
            description: format!("Imported from .eslintrc.json"),
            extends: Vec::new(),
            rules: HashMap::new(),
            env: HashMap::new(),
            globals: HashMap::new(),
            parser_options: ParserOptions::default(),
        };

        // Parse extends
        if let Some(extends) = eslint_config.get("extends") {
            if let Some(extends_str) = extends.as_str() {
                ruleset.extends.push(extends_str.to_string());
            } else if let Some(extends_array) = extends.as_array() {
                for item in extends_array {
                    if let Some(extend_str) = item.as_str() {
                        ruleset.extends.push(extend_str.to_string());
                    }
                }
            }
        }

        // Parse rules
        if let Some(rules) = eslint_config.get("rules").and_then(|r| r.as_object()) {
            for (rule_name, rule_value) in rules {
                let rule_config = self.parse_eslint_rule_value(rule_value)?;
                ruleset.rules.insert(rule_name.clone(), rule_config);
            }
        }

        // Parse env
        if let Some(env) = eslint_config.get("env").and_then(|e| e.as_object()) {
            for (env_name, env_value) in env {
                if let Some(enabled) = env_value.as_bool() {
                    ruleset.env.insert(env_name.clone(), enabled);
                }
            }
        }

        // Parse globals
        if let Some(globals) = eslint_config.get("globals").and_then(|g| g.as_object()) {
            for (global_name, global_value) in globals {
                let enabled = match global_value {
                    serde_json::Value::Bool(b) => *b,
                    serde_json::Value::String(s) => s == "readonly" || s == "writable",
                    _ => false,
                };
                ruleset.globals.insert(global_name.clone(), enabled);
            }
        }

        self.save_ruleset(&ruleset)
    }

    /// Export current ruleset as ESLint-compatible JSON
    pub fn export_eslintrc(&self) -> Result<String> {
        let ruleset = self.get_current_ruleset()?;

        let mut eslint_config = serde_json::Map::new();

        // Add extends
        if !ruleset.extends.is_empty() {
            eslint_config.insert(
                "extends".to_string(),
                serde_json::Value::Array(ruleset.extends.iter().map(|s| serde_json::Value::String(s.clone())).collect()),
            );
        }

        // Add rules
        let mut rules_obj = serde_json::Map::new();
        for (rule_name, rule_config) in &ruleset.rules {
            let rule_value = if rule_config.enabled {
                match rule_config.severity {
                    RuleSeverity::Off => serde_json::Value::Number(0.into()),
                    RuleSeverity::Warning => serde_json::Value::Number(1.into()),
                    RuleSeverity::Error => serde_json::Value::Number(2.into()),
                }
            } else {
                serde_json::Value::Number(0.into())
            };
            rules_obj.insert(rule_name.clone(), rule_value);
        }
        eslint_config.insert("rules".to_string(), serde_json::Value::Object(rules_obj));

        // Add env
        if !ruleset.env.is_empty() {
            let env_obj: serde_json::Map<String, serde_json::Value> = ruleset.env.iter().map(|(k, v)| (k.clone(), serde_json::Value::Bool(*v))).collect();
            eslint_config.insert("env".to_string(), serde_json::Value::Object(env_obj));
        }

        // Add globals
        if !ruleset.globals.is_empty() {
            let globals_obj: serde_json::Map<String, serde_json::Value> =
                ruleset.globals.iter().map(|(k, v)| (k.clone(), serde_json::Value::Bool(*v))).collect();
            eslint_config.insert("globals".to_string(), serde_json::Value::Object(globals_obj));
        }

        serde_json::to_string_pretty(&eslint_config).map_err(|e| Error::Storage {
            message: format!("Failed to serialize ESLint config: {}", e),
        })
    }

    /// Create a snapshot for persistence (placeholder)
    pub fn create_snapshot(&self) -> Result<()> {
        Ok(())
    }

    /// Initialize default ESLint rules
    fn initialize_default_rules(&mut self) -> Result<()> {
        if self.rulesets.contains_key("default") {
            return Ok(());
        }

        let mut default_ruleset = RuleSet {
            name: "default".to_string(),
            description: "Default ESLint rules for moon-shine".to_string(),
            extends: vec!["eslint:recommended".to_string()],
            rules: HashMap::new(),
            env: HashMap::new(),
            globals: HashMap::new(),
            parser_options: ParserOptions::default(),
        };

        // Add core rules
        self.add_core_rules(&mut default_ruleset);
        self.add_typescript_rules(&mut default_ruleset);
        self.add_security_rules(&mut default_ruleset);

        // Add common environments
        default_ruleset.env.insert("es2024".to_string(), true);
        default_ruleset.env.insert("node".to_string(), true);
        default_ruleset.env.insert("browser".to_string(), true);

        self.rulesets.insert(default_ruleset.name.clone(), default_ruleset);
        self.current_ruleset = "default".to_string();

        Ok(())
    }

    fn get_ruleset_mut(&mut self, name: &str) -> Result<&mut RuleSet> {
        self.rulesets.get_mut(name).ok_or_else(|| Error::Storage {
            message: format!("Rule set '{}' not found", name),
        })
    }

    fn add_core_rules(&self, ruleset: &mut RuleSet) {
        let core_rules = [
            (
                "no-unused-vars",
                "Variables that are declared and not used anywhere",
                RuleCategory::BestPractices,
                true,
            ),
            ("no-console", "Use of console.log statements", RuleCategory::BestPractices, true),
            ("no-debugger", "Use of debugger statements", RuleCategory::PossibleErrors, true),
            ("prefer-const", "Variables that could be constants", RuleCategory::CodeStyle, true),
            ("eqeqeq", "Require === and !== instead of == and !=", RuleCategory::BestPractices, true),
            ("no-eval", "Disallow eval() usage", RuleCategory::Security, false),
            ("no-var", "Require let or const instead of var", RuleCategory::CodeStyle, true),
            (
                "prefer-arrow-functions",
                "Prefer arrow functions for simple functions",
                RuleCategory::CodeStyle,
                true,
            ),
            (
                "prefer-template-literals",
                "Prefer template literals over string concatenation",
                RuleCategory::CodeStyle,
                true,
            ),
        ];

        for (rule_name, description, category, fixable) in core_rules {
            ruleset.rules.insert(
                rule_name.to_string(),
                RuleConfig {
                    enabled: true,
                    severity: RuleSeverity::Warning,
                    options: None,
                    description: description.to_string(),
                    category,
                    fixable,
                    recommended: true,
                },
            );
        }
    }

    fn add_typescript_rules(&self, ruleset: &mut RuleSet) {
        let ts_rules = [
            ("no-any", "Disallow usage of the any type", RuleCategory::TypeScript, true),
            (
                "explicit-function-return-type",
                "Require explicit return types on functions",
                RuleCategory::TypeScript,
                false,
            ),
            (
                "prefer-readonly",
                "Prefer readonly for properties that are never reassigned",
                RuleCategory::TypeScript,
                true,
            ),
        ];

        for (rule_name, description, category, fixable) in ts_rules {
            ruleset.rules.insert(
                rule_name.to_string(),
                RuleConfig {
                    enabled: true,
                    severity: RuleSeverity::Warning,
                    options: None,
                    description: description.to_string(),
                    category,
                    fixable,
                    recommended: false,
                },
            );
        }
    }

    fn add_security_rules(&self, ruleset: &mut RuleSet) {
        let security_rules = [
            ("no-eval", "Disallow eval() usage", RuleCategory::Security, false),
            ("no-implied-eval", "Disallow implied eval()", RuleCategory::Security, false),
            ("no-new-func", "Disallow Function constructor", RuleCategory::Security, false),
        ];

        for (rule_name, description, category, fixable) in security_rules {
            ruleset.rules.insert(
                rule_name.to_string(),
                RuleConfig {
                    enabled: true,
                    severity: RuleSeverity::Error,
                    options: None,
                    description: description.to_string(),
                    category,
                    fixable,
                    recommended: true,
                },
            );
        }
    }

    fn parse_eslint_rule_value(&self, value: &serde_json::Value) -> Result<RuleConfig> {
        let (severity, options) = match value {
            serde_json::Value::Number(n) => {
                let severity = match n.as_u64() {
                    Some(0) => RuleSeverity::Off,
                    Some(1) => RuleSeverity::Warning,
                    Some(2) => RuleSeverity::Error,
                    _ => RuleSeverity::Warning,
                };
                (severity, None)
            }
            serde_json::Value::String(s) => {
                let severity = match s.as_str() {
                    "off" => RuleSeverity::Off,
                    "warn" => RuleSeverity::Warning,
                    "error" => RuleSeverity::Error,
                    _ => RuleSeverity::Warning,
                };
                (severity, None)
            }
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    (RuleSeverity::Off, None)
                } else {
                    let severity = match &arr[0] {
                        serde_json::Value::Number(n) => match n.as_u64() {
                            Some(0) => RuleSeverity::Off,
                            Some(1) => RuleSeverity::Warning,
                            Some(2) => RuleSeverity::Error,
                            _ => RuleSeverity::Warning,
                        },
                        serde_json::Value::String(s) => match s.as_str() {
                            "off" => RuleSeverity::Off,
                            "warn" => RuleSeverity::Warning,
                            "error" => RuleSeverity::Error,
                            _ => RuleSeverity::Warning,
                        },
                        _ => RuleSeverity::Warning,
                    };
                    let options = if arr.len() > 1 { Some(arr[1].clone()) } else { None };
                    (severity, options)
                }
            }
            _ => (RuleSeverity::Warning, None),
        };

        Ok(RuleConfig {
            enabled: !matches!(severity, RuleSeverity::Off),
            severity,
            options,
            description: "Imported rule".to_string(),
            category: RuleCategory::BestPractices,
            fixable: false,
            recommended: false,
        })
    }
}

impl Default for ParserOptions {
    fn default() -> Self {
        let mut ecma_features = HashMap::new();
        ecma_features.insert("jsx".to_string(), true);
        ecma_features.insert("globalReturn".to_string(), false);

        Self {
            ecma_version: 2024,
            source_type: "module".to_string(),
            ecma_features,
        }
    }
}
