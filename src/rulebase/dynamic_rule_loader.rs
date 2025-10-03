//! # Rule Loader - Load Rules from JSON
//!
//! Loads all 853 rules from the generated JSON rulebase using serde.
//! Simple and efficient loading without KV complexity.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "embedded_rulebase")]
use crate::rulebase::iter_builtin_rules;

/// A rule loader that reads rule definitions from a generated JSON file.
#[derive(Debug)]
pub struct RuleLoader {
    /// A cache of rule definitions, indexed by rule ID.
    definitions: HashMap<String, RuleDefinition>,
    /// Metadata about the loaded rulebase.
    metadata: RulebaseMetadata,
}

/// An external rule definition, typically loaded from a JSON or YAML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    /// The unique identifier for the rule.
    pub id: String,
    /// The name of the rule.
    pub name: String,
    /// A description of the rule's purpose.
    pub description: String,
    /// The category that the rule belongs to.
    pub category: String,
    /// The severity of the rule.
    pub severity: String,
    /// The implementation type of the rule.
    pub implementation: RuleImplementation,
    /// An estimate of the execution cost of the rule.
    pub cost: u32,
    /// If true, the rule supports automatic fixing.
    pub autofix: bool,
    /// If true, the rule is enhanced with AI capabilities.
    pub ai_enhanced: bool,
    /// A list of tags associated with the rule.
    pub tags: Vec<String>,
    /// A list of rule IDs that this rule depends on.
    pub dependencies: Vec<String>,
    /// An optional JSON schema for configuring the rule.
    pub config_schema: Option<serde_json::Value>,
}

/// The possible implementation types for a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleImplementation {
    /// A static analysis rule.
    StaticAnalysis {
        /// The name of the static analysis rule.
        rule_name: String,
    },
    /// An AI-powered behavioral pattern.
    AiBehavioral {
        /// The type of the behavioral pattern.
        pattern_type: String,
    },
    /// A custom rule implemented in JavaScript.
    JavaScript {
        /// The JavaScript code for the rule.
        code: String,
    },
    /// A rule that executes an external tool.
    ExternalTool {
        /// The command to execute.
        command: String,
        /// A list of arguments to pass to the command.
        args: Vec<String>,
    },
    /// A hybrid rule that combines multiple implementation approaches.
    Hybrid {
        /// A list of sub-implementations.
        implementations: Vec<RuleImplementation>,
    },
    /// A rule based on a regular expression.
    Regex {
        /// The regular expression pattern.
        pattern: String,
        /// The flags for the regular expression.
        flags: String,
    },
    /// An AST-based rule with a query.
    AstQuery {
        /// The query to execute against the AST.
        query: String,
        /// The language of the AST.
        language: String,
    },
}

/// The complete structure of the rulebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rulebase {
    /// The content of the rulebase.
    pub rulebase: RulebaseContent,
}

/// The content of the rulebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseContent {
    /// The version of the rulebase.
    pub version: String,
    /// Metadata about the rulebase.
    pub metadata: RulebaseMetadata,
    /// Global settings for the rulebase.
    pub settings: RulebaseSettings,
    /// A list of static analysis rules.
    pub static_rules: Vec<RuleDefinition>,
    /// A list of behavioral rules.
    pub behavioral_rules: Vec<RuleDefinition>,
    /// A list of hybrid rules.
    pub hybrid_rules: Vec<RuleDefinition>,
}

/// Metadata about the rulebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseMetadata {
    /// The total number of rules in the rulebase.
    pub total_rules: usize,
    /// The number of static analysis rules.
    pub static_rules: usize,
    /// The number of behavioral rules.
    pub behavioral_rules: usize,
    /// The number of hybrid rules.
    pub hybrid_rules: usize,
    /// The timestamp when the rulebase was generated.
    pub generated_at: String,
    /// The name of the generator that created the rulebase.
    pub generator: String,
}

/// Global settings for the rulebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulebaseSettings {
    /// If true, AI enhancements are enabled.
    pub ai_enabled: bool,
    /// The default timeout for rule execution in milliseconds.
    pub default_timeout_ms: u64,
    /// The maximum number of rules to execute in parallel.
    pub max_parallel: usize,
    /// If true, caching is enabled for rule execution.
    pub cache_enabled: bool,
}

impl RuleLoader {
    /// Creates a new `RuleLoader` and loads the rules from the embedded JSON file.
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

    /// Loads rules from the embedded JSON file.
    #[cfg(feature = "embedded_rulebase")]
    fn load_from_embedded_json(&mut self) -> Result<()> {
        self.load_from_compiled_rules()
    }

    #[cfg(not(feature = "embedded_rulebase"))]
    fn load_from_embedded_json(&mut self) -> Result<()> {
        let bundled_json = include_str!("../../rulebase/output/moonshine-rulebase-complete.json");
        let rulebase: Rulebase = serde_json::from_str(bundled_json)
            .map_err(|e| Error::Configuration(format!("Invalid bundled rulebase: {}", e)))?;
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
                    RuleImplementation::AiBehavioral {
                        pattern_type: pattern,
                    }
                }
                "JavaScript" => RuleImplementation::JavaScript {
                    code: def.implementation.code.clone().unwrap_or_default(),
                },
                "ExternalTool" => RuleImplementation::ExternalTool {
                    command: def.implementation.command.clone().unwrap_or_default(),
                    args: def.implementation.args.clone().unwrap_or_default(),
                },
                "Hybrid" => RuleImplementation::Hybrid {
                    implementations: Vec::new(),
                },
                "Regex" => RuleImplementation::Regex {
                    pattern: def.implementation.rule_name.clone().unwrap_or_default(),
                    flags: def.implementation.code.clone().unwrap_or_default(),
                },
                "AstQuery" => RuleImplementation::AstQuery {
                    query: def.implementation.rule_name.clone().unwrap_or_default(),
                    language: def
                        .implementation
                        .code
                        .clone()
                        .unwrap_or_else(|| "typescript".to_string()),
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

    /// Loads rules from the given `RulebaseContent`.
    fn load_from_rulebase(&mut self, content: RulebaseContent) -> Result<()> {
        println!("📦 Loading rules from rulebase...");

        self.metadata = content.metadata.clone();

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

    /// Gets a rule by its ID.
    pub fn get_rule(&self, rule_id: &str) -> Option<&RuleDefinition> {
        self.definitions.get(rule_id)
    }

    /// Checks if a rule exists by its ID.
    pub fn has_rule(&self, rule_id: &str) -> bool {
        self.definitions.contains_key(rule_id)
    }

    /// Gets all rule definitions.
    pub fn get_all_rules(&self) -> &HashMap<String, RuleDefinition> {
        &self.definitions
    }

    /// Gets the metadata for the rulebase.
    pub fn get_metadata(&self) -> &RulebaseMetadata {
        &self.metadata
    }

    /// Filters rules by category.
    pub fn filter_rules_by_category(&self, category: &str) -> Vec<&RuleDefinition> {
        self.definitions
            .values()
            .filter(|rule| rule.category == category)
            .collect()
    }

    /// Filters rules by tags.
    pub fn filter_rules_by_tags(&self, tags: &[String]) -> Vec<&RuleDefinition> {
        self.definitions
            .values()
            .filter(|rule| tags.iter().any(|tag| rule.tags.contains(tag)))
            .collect()
    }

    /// Filters rules by their AI enhancement status.
    pub fn filter_ai_enhanced_rules(&self, ai_enhanced: bool) -> Vec<&RuleDefinition> {
        self.definitions
            .values()
            .filter(|rule| rule.ai_enhanced == ai_enhanced)
            .collect()
    }

    /// Counts the number of rules in a given category.
    pub fn count_rules_by_category(&self, category: &str) -> usize {
        self.definitions
            .values()
            .filter(|rule| rule.category == category)
            .count()
    }

    /// Gets all available rule categories.
    pub fn get_all_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> =
            self.definitions.values().map(|rule| rule.category.clone()).collect();
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
