//! Moon-shine configuration plumbing.
//!
//! All defaults live in the `defaults` module so it's easy to audit the baseline values. The top
//! level `MoonShineConfig` struct groups settings into logical sub-structures (AI, linting,
//! adaptive rule system) that map directly to the YAML structure consumers provide in `moon.yml`.

use crate::error::{Error, Result};
use crate::moon_pdk_interface::get_moon_config_safe;
use moon_pdk::get_extension_config;
use serde::{Deserialize, Serialize};
// Removed unused imports: HashMap, PathBuf

pub use adaptive::{AdaptiveConfig, PatternTrackingConfig, RuleGenerationConfig, StarcoderConfig};
pub use ai::AiConfig;
pub use linting::LintingConfig;

/// Create a JSON schema for the Moon Shine configuration
pub fn create_config_schema() -> String {
    r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "title": "Moon Shine Configuration",
        "description": "Configuration schema for Moon Shine AI-powered code analysis",
        "properties": {
            "ai": {
                "type": "object",
                "description": "AI configuration settings",
                "properties": {
                    "model": {
                        "type": "string",
                        "description": "AI model to use for analysis",
                        "default": "claude-3-5-sonnet"
                    },
                    "enable_ai_suggestions": {
                        "type": "boolean",
                        "description": "Enable AI-powered suggestions",
                        "default": true
                    },
                    "quality_threshold": {
                        "type": "number",
                        "description": "Quality threshold for AI suggestions",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "default": 0.8
                    }
                }
            },
            "linting": {
                "type": "object",
                "description": "Linting configuration",
                "properties": {
                    "enable_oxc": {
                        "type": "boolean",
                        "description": "Enable OXC linting",
                        "default": true
                    },
                    "enable_eslint": {
                        "type": "boolean",
                        "description": "Enable ESLint",
                        "default": true
                    },
                    "enable_prettier": {
                        "type": "boolean",
                        "description": "Enable Prettier formatting",
                        "default": true
                    }
                }
            }
        }
    }"#
    .to_string()
}

/// CLI arguments structure for Moon PDK integration
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonShineArgs {
    /// Operation mode: fix, lint-only, or comprehensive
    pub mode: Option<String>,

    /// Only report issues without fixing
    pub lint_only: bool,

    /// CI-friendly reporting mode (no interactive fixes)
    pub reporting_only: bool,

    /// Force initialization of configuration files
    pub force_init: bool,

    /// Install default prompts and configuration
    pub install_prompts: bool,

    /// Files to process (supports glob patterns)
    pub files: Vec<String>,
}

// Type alias for backward compatibility
pub type AdaptiveRuleSystemConfig = AdaptiveConfig;

mod ai {
    use super::defaults;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiConfig {
        #[serde(default = "defaults::ai_model")]
        pub model: String,
        #[serde(default = "defaults::ai_temperature")]
        pub temperature: f32,
        #[serde(default = "defaults::ai_providers")]
        pub providers: Vec<String>,
        #[serde(default = "defaults::ai_max_concurrent_requests")]
        pub max_concurrent_requests: u32,
        #[serde(default = "defaults::ai_batch_size")]
        pub batch_size: u32,
        #[serde(default)]
        pub enable_copro_optimization: bool,
        #[serde(default)]
        pub enable_pattern_detection: bool,
    }

    impl Default for AiConfig {
        fn default() -> Self {
            Self {
                model: defaults::ai_model(),
                temperature: defaults::ai_temperature(),
                providers: defaults::ai_providers(),
                max_concurrent_requests: defaults::ai_max_concurrent_requests(),
                batch_size: defaults::ai_batch_size(),
                enable_copro_optimization: false,
                enable_pattern_detection: false,
            }
        }
    }
}

mod linting {
    use super::defaults;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LintingConfig {
        #[serde(default = "defaults::lint_include_patterns")]
        pub include_patterns: Vec<String>,
        #[serde(default = "defaults::lint_exclude_patterns")]
        pub exclude_patterns: Vec<String>,
        #[serde(default = "defaults::lint_max_suggestions")]
        pub max_suggestions: u32,
        #[serde(default)]
        pub enable_auto_fix: bool,
    }

    impl Default for LintingConfig {
        fn default() -> Self {
            Self {
                include_patterns: defaults::lint_include_patterns(),
                exclude_patterns: defaults::lint_exclude_patterns(),
                max_suggestions: defaults::lint_max_suggestions(),
                enable_auto_fix: false,
            }
        }
    }
}

mod adaptive {
    use super::defaults;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AdaptiveConfig {
        #[serde(default = "defaults::adaptive_enabled")]
        pub enabled: bool,
        #[serde(default)]
        pub pattern_tracking: PatternTrackingConfig,
        #[serde(default)]
        pub rule_generation: RuleGenerationConfig,
        #[serde(default)]
        pub starcoder: StarcoderConfig,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PatternTrackingConfig {
        #[serde(default = "defaults::pattern_min_frequency")]
        pub min_pattern_frequency: u32,
        #[serde(default = "defaults::pattern_max_age_days")]
        pub pattern_max_age_days: u32,
        #[serde(default = "defaults::pattern_similarity_threshold")]
        pub clustering_similarity_threshold: f32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleGenerationConfig {
        #[serde(default = "defaults::rule_min_cluster_size")]
        pub min_cluster_size_for_rules: usize,
        #[serde(default = "defaults::rule_max_per_cluster")]
        pub max_rules_per_cluster: usize,
        #[serde(default = "defaults::rule_quality_threshold")]
        pub quality_threshold: f64,
        #[serde(default = "defaults::rule_auto_activate")]
        pub enable_auto_rule_activation: bool,
        #[serde(default = "defaults::rule_max_training_examples")]
        pub max_training_examples: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StarcoderConfig {
        #[serde(default = "defaults::starcoder_enabled")]
        pub enabled: bool,
        #[serde(default = "defaults::starcoder_training_threshold")]
        pub training_threshold: usize,
        #[serde(default = "defaults::starcoder_train_on_good")]
        pub train_on_good_code: bool,
        #[serde(default = "defaults::starcoder_train_on_bad")]
        pub train_on_bad_patterns: bool,
        #[serde(default = "defaults::starcoder_model_path")]
        pub model_path: Option<String>,
    }

    impl Default for AdaptiveConfig {
        fn default() -> Self {
            Self {
                enabled: defaults::adaptive_enabled(),
                pattern_tracking: PatternTrackingConfig::default(),
                rule_generation: RuleGenerationConfig::default(),
                starcoder: StarcoderConfig::default(),
            }
        }
    }

    impl Default for PatternTrackingConfig {
        fn default() -> Self {
            Self {
                min_pattern_frequency: defaults::pattern_min_frequency(),
                pattern_max_age_days: defaults::pattern_max_age_days(),
                clustering_similarity_threshold: defaults::pattern_similarity_threshold(),
            }
        }
    }

    impl Default for RuleGenerationConfig {
        fn default() -> Self {
            Self {
                min_cluster_size_for_rules: defaults::rule_min_cluster_size(),
                max_rules_per_cluster: defaults::rule_max_per_cluster(),
                quality_threshold: defaults::rule_quality_threshold(),
                enable_auto_rule_activation: defaults::rule_auto_activate(),
                max_training_examples: defaults::rule_max_training_examples(),
            }
        }
    }

    impl Default for StarcoderConfig {
        fn default() -> Self {
            Self {
                enabled: defaults::starcoder_enabled(),
                training_threshold: defaults::starcoder_training_threshold(),
                train_on_good_code: defaults::starcoder_train_on_good(),
                train_on_bad_patterns: defaults::starcoder_train_on_bad(),
                model_path: defaults::starcoder_model_path(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonShineConfig {
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub linting: LintingConfig,
    #[serde(default)]
    pub adaptive: AdaptiveConfig,
    #[serde(default)]
    pub enable_relationship_analysis: Option<bool>,
    #[serde(default)]
    pub ai_model: Option<String>,
    #[serde(default)]
    pub enable_ai_tsdoc: Option<bool>,
    #[serde(default)]
    pub tsdoc_coverage_target: Option<f64>,
    #[serde(default)]
    pub operation_mode: Option<String>,
    #[serde(default)]
    pub custom_prompts: Option<std::collections::HashMap<String, String>>,
}

impl Default for MoonShineConfig {
    fn default() -> Self {
        MoonShineConfig {
            ai: AiConfig::default(),
            linting: LintingConfig::default(),
            adaptive: AdaptiveConfig::default(),
            enable_relationship_analysis: Some(false),
            ai_model: Some("sonnet".to_string()),
            enable_ai_tsdoc: Some(true),
            tsdoc_coverage_target: Some(90.0),
            operation_mode: Some("fix".to_string()),
            custom_prompts: None,
        }
    }
}

impl MoonShineConfig {
    /// Load configuration from Moon’s extension config, applying defaults and validation.
    pub fn from_moon_workspace() -> Result<Self> {
        let mut config = get_extension_config::<Self>().unwrap_or_default();
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&mut self) -> Result<()> {
        if !(0.0..=2.0).contains(&self.ai.temperature) {
            return Err(Error::config("ai.temperature must be between 0.0 and 2.0"));
        }

        if self.adaptive.rule_generation.quality_threshold < 0.0 || self.adaptive.rule_generation.quality_threshold > 1.0 {
            return Err(Error::config("adaptive.ruleGeneration.qualityThreshold must be between 0.0 and 1.0"));
        }

        Ok(())
    }

    /// Directory where Moon stores moon-shine data (prompts, sessions, cache).
    pub fn moonshine_directory() -> String {
        get_moon_config_safe("moonshine_directory")
            .or_else(|_| get_moon_config_safe("extension.moonshine.data_directory"))
            .unwrap_or(None)
            .unwrap_or_else(|| ".moon/moonshine".to_string())
    }

    /// Get debug session retention hours
    pub fn debug_session_retention_hours(&self) -> u32 {
        // Default to 12 hours for debug sessions
        12
    }

    /// Get cleanup threshold for sessions older than specified hours
    pub fn cleanup_sessions_older_than_hours(&self) -> u32 {
        // Default to 48 hours cleanup threshold
        48
    }

    pub fn keep_debug_sessions(&self) -> bool {
        self.ai.providers.iter().any(|p| p == "debug")
    }
}

pub mod defaults {
    use std::path::PathBuf;

    // AI defaults
    pub fn ai_model() -> String {
        "sonnet".into()
    }
    pub fn ai_temperature() -> f32 {
        0.7
    }
    pub fn ai_providers() -> Vec<String> {
        vec!["claude".into(), "codex".into()]
    }
    pub fn ai_max_concurrent_requests() -> u32 {
        3
    }
    pub fn ai_batch_size() -> u32 {
        5
    }

    // Linting defaults
    pub fn lint_include_patterns() -> Vec<String> {
        vec!["**/*.{ts,tsx,js,jsx}".into()]
    }
    pub fn lint_exclude_patterns() -> Vec<String> {
        vec!["node_modules/**".into(), "dist/**".into(), "build/**".into()]
    }
    pub fn lint_max_suggestions() -> u32 {
        50
    }

    // Adaptive defaults
    pub fn adaptive_enabled() -> bool {
        true
    }
    pub fn pattern_min_frequency() -> u32 {
        10
    }
    pub fn pattern_max_age_days() -> u32 {
        30
    }
    pub fn pattern_similarity_threshold() -> f32 {
        0.7
    }
    pub fn rule_min_cluster_size() -> usize {
        5
    }
    pub fn rule_max_per_cluster() -> usize {
        3
    }
    pub fn rule_quality_threshold() -> f64 {
        0.85
    }
    pub fn rule_auto_activate() -> bool {
        false
    }
    pub fn rule_max_training_examples() -> usize {
        1000
    }
    pub fn starcoder_enabled() -> bool {
        true
    }
    pub fn starcoder_training_threshold() -> usize {
        10
    }
    pub fn starcoder_train_on_good() -> bool {
        true
    }
    pub fn starcoder_train_on_bad() -> bool {
        true
    }
    pub fn starcoder_model_path() -> Option<String> {
        Some(PathBuf::from("models/starcoder-1b").to_string_lossy().to_string())
    }
}
