//! # Rule Loader - Load Rules from JSON
//!
//! Loads all 853 rules from the generated JSON rulebase using serde.
//! Simple and efficient loading without KV complexity.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "embedded_rulebase")]
use crate::rulebase::iter_builtin_rules;

/// Rule loader that reads from generated JSON
#[derive(Debug)]
pub struct RuleLoader {
    /// Rule definitions cache
    definitions: HashMap<String, RuleDefinition>,
    /// Metadata about loaded rules
    metadata: RulebaseMetadata,
}

/// External rule definition (from JSON/YAML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule category
    pub category: String,
    /// Rule severity
    pub severity: String,
    /// Rule implementation type
    pub implementation: RuleImplementation,
    /// Execution cost estimate
    pub cost: u32,
    /// Whether rule supports autofix
    pub autofix: bool,
    /// Whether rule is AI enhanced
    pub ai_enhanced: bool,
    /// Rule tags
    pub tags: Vec<String>,
    /// Rule dependencies
    pub dependencies: Vec<String>,
    /// Rule configuration schema
    pub config_schema: Option<serde_json::Value>,
}

/// Rule implementation types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleImplementation {
    /// Static analysis rule
    StaticAnalysis { rule_name: String },
    /// AI behavioral pattern
    AiBehavioral { pattern_type: String },
    /// Custom JavaScript implementation
    JavaScript { code: String },
    /// External tool execution
    ExternalTool { command: String, args: Vec<String> },
    /// Hybrid rule combining multiple approaches
    Hybrid { implementations: Vec<RuleImplementation> },
    /// RegEx-based rule
    Regex { pattern: String, flags: String },
    /// AST-based rule with query
    AstQuery { query: String, language: String },
}

/// Complete rulebase structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rulebase {
    pub rulebase: RulebaseContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseContent {
    pub version: String,
    pub metadata: RulebaseMetadata,
    pub settings: RulebaseSettings,
    pub static_rules: Vec<RuleDefinition>,
    pub behavioral_rules: Vec<RuleDefinition>,
    pub hybrid_rules: Vec<RuleDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseMetadata {
    pub total_rules: usize,
    pub static_rules: usize,
    pub behavioral_rules: usize,
    pub hybrid_rules: usize,
    pub generated_at: String,
    pub generator: String,
}

/// Global rulebase settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseSettings {
    /// Whether to enable AI enhancements
    pub ai_enabled: bool,
    /// Default execution timeout
    pub default_timeout_ms: u64,
    /// Maximum parallel rules
    pub max_parallel: usize,
    /// Cache configuration
    pub cache_enabled: bool,
}

impl RuleLoader {
    /// Create new rule loader and load rules from JSON
    pub fn new() -> Result<Self> {
        let mut loader = Self {
            definitions: HashMap::new(),
            metadata: RulebaseMetadata {
                total_rules: 0,
                static_rules: 0,
                behavioral_rules: 0,
                hybrid_rules: 0,
                generated_at: String::new(),
                generator: String::new(),
            },
        };
        loader.load_from_embedded_json()?;
        Ok(loader)
    }

    /// Load rules from embedded JSON file
    #[cfg(feature = "embedded_rulebase")]
    fn load_from_embedded_json(&mut self) -> Result<()> {
        self.load_from_compiled_rules()
    }

    #[cfg(not(feature = "embedded_rulebase"))]
    fn load_from_embedded_json(&mut self) -> Result<()> {
        let bundled_json = include_str!("../../rulebase/output/moonshine-rulebase-complete.json");
        let rulebase: Rulebase = serde_json::from_str(bundled_json).map_err(|e| Error::Configuration(format!("Invalid bundled rulebase: {}", e)))?;
        self.load_from_rulebase(rulebase.rulebase)?;
        Ok(())
    }

    #[cfg(feature = "embedded_rulebase")]
    fn load_from_compiled_rules(&mut self) -> Result<()> {
        println!("📦 Loading rules from compiled definitions...");

        let mut static_rules = 0usize;
        let mut behavioral_rules = 0usize;
        let mut hybrid_rules = 0usize;

        for def in iter_builtin_rules() {
            let implementation = match def.implementation.kind.as_str() {
                "StaticAnalysis" => {
                    let name = def.implementation.rule_name.clone().unwrap_or_default();
                    RuleImplementation::StaticAnalysis { rule_name: name }
                }
                "AiBehavioral" => {
                    let pattern = def.implementation.rule_name.clone().unwrap_or_default();
                    RuleImplementation::AiBehavioral { pattern_type: pattern }
                }
                "JavaScript" => RuleImplementation::JavaScript {
                    code: def.implementation.code.clone().unwrap_or_default(),
                },
                "ExternalTool" => RuleImplementation::ExternalTool {
                    command: def.implementation.command.clone().unwrap_or_default(),
                    args: def.implementation.args.clone().unwrap_or_default(),
                },
                "Hybrid" => RuleImplementation::Hybrid { implementations: Vec::new() },
                "Regex" => RuleImplementation::Regex {
                    pattern: def.implementation.rule_name.clone().unwrap_or_default(),
                    flags: def.implementation.code.clone().unwrap_or_default(),
                },
                "AstQuery" => RuleImplementation::AstQuery {
                    query: def.implementation.rule_name.clone().unwrap_or_default(),
                    language: def.implementation.code.clone().unwrap_or_else(|| "typescript".to_string()),
                },
                other => {
                    return Err(Error::Config {
                        message: format!("Unsupported rule implementation: {}", other),
                        field: Some("implementation.kind".to_string()),
                        value: Some(other.to_string()),
                    })
                }
            };

            let rule = RuleDefinition {
                id: def.id.clone(),
                name: def.name.clone(),
                description: def.description.clone(),
                category: def.category.clone(),
                severity: def.severity.clone(),
                implementation,
                cost: def.cost,
                autofix: def.autofix,
                ai_enhanced: def.ai_enhanced,
                tags: def.tags.clone(),
                dependencies: def.dependencies.clone(),
                config_schema: def.config_schema.clone(),
            };

            match rule.category.as_str() {
                "Behavioral" => behavioral_rules += 1,
                "Hybrid" => hybrid_rules += 1,
                _ => static_rules += 1,
            }

            self.definitions.insert(rule.id.clone(), rule);
        }

        self.metadata = RulebaseMetadata {
            total_rules: self.definitions.len(),
            static_rules,
            behavioral_rules,
            hybrid_rules,
            generated_at: "compiled".to_string(),
            generator: "embedded-json".to_string(),
        };

        println!("✅ Loaded {} compiled rules", self.definitions.len());
        Ok(())
    }

    /// Load rules from rulebase content
    fn load_from_rulebase(&mut self, content: RulebaseContent) -> Result<()> {
        println!("📦 Loading rules from rulebase...");

        // Store metadata
        self.metadata = content.metadata.clone();

        // Load all rules into definitions
        for rule_def in content.static_rules {
            self.definitions.insert(rule_def.id.clone(), rule_def);
        }

        for rule_def in content.behavioral_rules {
            self.definitions.insert(rule_def.id.clone(), rule_def);
        }

        for rule_def in content.hybrid_rules {
            self.definitions.insert(rule_def.id.clone(), rule_def);
        }

        println!("✅ Loaded {} total rules", self.definitions.len());
        Ok(())
    }

    /// Get rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&RuleDefinition> {
        self.definitions.get(rule_id)
    }

    /// Check if rule exists
    pub fn has_rule(&self, rule_id: &str) -> bool {
        self.definitions.contains_key(rule_id)
    }

    /// Get all rule definitions
    pub fn get_all_rules(&self) -> &HashMap<String, RuleDefinition> {
        &self.definitions
    }

    /// Get metadata
    pub fn get_metadata(&self) -> &RulebaseMetadata {
        &self.metadata
    }

    /// Filter rules by category
    pub fn filter_rules_by_category(&self, category: &str) -> Vec<&RuleDefinition> {
        self.definitions.values().filter(|rule| rule.category == category).collect()
    }

    /// Filter rules by tags
    pub fn filter_rules_by_tags(&self, tags: &[String]) -> Vec<&RuleDefinition> {
        self.definitions
            .values()
            .filter(|rule| tags.iter().any(|tag| rule.tags.contains(tag)))
            .collect()
    }

    /// Filter rules by AI enhancement status
    pub fn filter_ai_enhanced_rules(&self, ai_enhanced: bool) -> Vec<&RuleDefinition> {
        self.definitions.values().filter(|rule| rule.ai_enhanced == ai_enhanced).collect()
    }

    /// Get count of rules by category
    pub fn count_rules_by_category(&self, category: &str) -> usize {
        self.definitions.values().filter(|rule| rule.category == category).count()
    }

    /// Check if rule exists by ID
    pub fn has_rule(&self, rule_id: &str) -> bool {
        self.definitions.contains_key(rule_id)
    }

    /// Get all available rule categories
    pub fn get_all_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.definitions.values().map(|rule| rule.category.clone()).collect();
        categories.sort();
        categories.dedup();
        categories
    }
}

impl Default for RulebaseSettings {
    fn default() -> Self {
        Self {
            ai_enabled: true,
            default_timeout_ms: 10000,
            max_parallel: get_optimal_parallelism(),
            cache_enabled: true,
        }
    }
}

/// Get optimal parallelism for Moon extension environment
/// Falls back gracefully if num_cpus is not available
fn get_optimal_parallelism() -> usize {
    // In WASM environment, we use a conservative default
    // since we can't easily detect CPU cores
    #[cfg(target_arch = "wasm32")]
    {
        // Conservative default for WASM: 4 parallel tasks
        4
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Try to get actual CPU count, fall back to reasonable default
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8) // Fallback to 8 if detection fails
    }
}
