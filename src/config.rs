//! Configuration structures and schemas for Moon Shine extension.
//!
//! This module defines the configuration structures used by the `moon-shine` extension,
//! including command-line arguments and the main configuration loaded from `moon.yml`.

use crate::error::{Error, Result};
use crate::moon_pdk_interface::get_moon_config_safe;
use moon_pdk::get_extension_config;
use moon_pdk_api::config_struct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines the command-line arguments accepted by the `moon-shine` extension.
///
/// These arguments are parsed from the command line when the extension is invoked
/// and control its primary operation mode.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonShineArgs {
    /// Specifies the operation mode. Can be "fix", "lint-only", or "comprehensive".
    pub mode: Option<String>,
    /// If true, only reports issues without applying any fixes.
    pub lint_only: bool,
    /// If true, enables a CI-friendly reporting mode without interactive fixes.
    pub reporting_only: bool,
    /// If true, forces the initialization of configuration files.
    pub force_init: bool,
    /// If true, installs the default prompts and configuration.
    pub install_prompts: bool,
    /// A list of files to process. Supports glob patterns.
    pub files: Vec<String>,
}

config_struct!(
    /// Defines the main configuration for the `moon-shine` extension.
    ///
    /// This structure is loaded from the `toolchain.moonShine` section of `moon.yml`
    /// and provides detailed control over the extension's behavior, including AI settings,
    /// tool integrations, and workflow options.
    ///
    /// Note: This is a large configuration struct. Future refactoring may group
    /// related fields into more focused structs (e.g., `CoproConfig`, `PatternDetectionConfig`).
    #[derive(Clone, Default, Serialize)]
    pub struct MoonShineConfig {
        // Core AI configuration
        /// The primary AI model to use for code analysis (e.g., "sonnet", "opus").
        #[serde(rename = "aiModel")]
        pub ai_model: Option<String>,

        /// A list of available AI providers to route requests to.
        #[serde(rename = "aiProviders")]
        pub ai_providers: Option<Vec<String>>,

        // LM Parameters
        /// Controls the creativity of the AI model. Ranges from 0.0 to 2.0.
        #[serde(rename = "temperature")]
        pub temperature: Option<f32>,

        /// The nucleus sampling probability. Ranges from 0.0 to 1.0.
        #[serde(rename = "topP")]
        pub top_p: Option<f32>,

        /// The maximum number of tokens in a response.
        #[serde(rename = "maxTokens")]
        pub max_tokens: Option<u32>,

        /// The maximum number of tokens to generate in a completion.
        #[serde(rename = "maxCompletionTokens")]
        pub max_completion_tokens: Option<u32>,

        /// Enables Collaborative Prompt Optimization (COPRO) for systematic prompt engineering.
        #[serde(rename = "enableCoproOptimization")]
        pub enable_copro_optimization: Option<bool>,

        /// Enables AI-powered code pattern learning and detection.
        #[serde(rename = "enablePatternDetection")]
        pub enable_pattern_detection: Option<bool>,

        // COPRO configuration
        /// The number of prompt candidates to generate per optimization iteration.
        #[serde(rename = "coproBreadth")]
        pub copro_breadth: Option<u32>,

        /// The number of optimization iterations to perform.
        #[serde(rename = "coproDepth")]
        pub copro_depth: Option<u32>,

        /// The creativity temperature for generating COPRO candidates.
        #[serde(rename = "coproTemperature")]
        pub copro_temperature: Option<f32>,

        // Pattern detection configuration
        /// The learning rate for the pattern detection algorithm.
        #[serde(rename = "patternLearningRate")]
        pub pattern_learning_rate: Option<f32>,

        /// The minimum frequency for a code pattern to be considered significant.
        #[serde(rename = "patternMinFrequency")]
        pub pattern_min_frequency: Option<f32>,

        /// Configurable rules for code pattern analysis.
        #[serde(rename = "patternRules")]
        pub pattern_rules: Option<crate::pattern_config::PatternConfig>,

        // Tool integration
        /// Enables integration with ESLint for linting.
        #[serde(rename = "enableEslintIntegration")]
        pub enable_eslint_integration: Option<bool>,

        /// Enables integration with the TypeScript compiler for type checking.
        #[serde(rename = "enableTypescriptIntegration")]
        pub enable_typescript_integration: Option<bool>,

        /// Enables integration with Prettier for code formatting.
        #[serde(rename = "enablePrettierIntegration")]
        pub enable_prettier_integration: Option<bool>,

        // OpenAI Codex CLI integration
        /// Enables integration with the OpenAI Codex CLI.
        #[serde(rename = "enableCodexIntegration")]
        pub enable_codex_integration: Option<bool>,

        /// The Codex model to use (e.g., "gpt-5-codex").
        #[serde(rename = "codexModel")]
        pub codex_model: Option<String>,

        /// The command to execute the Codex CLI.
        #[serde(rename = "codexCommand")]
        pub codex_command: Option<String>,

        /// The reasoning effort level for Codex to apply.
        #[serde(rename = "codexReasoningEffort")]
        pub codex_reasoning_effort: Option<String>,

        /// Overrides the Moon task to run for every analysis invocation.
        #[serde(rename = "moonTaskName")]
        pub moon_task_name: Option<String>,

        /// A mapping of languages to Moon tasks, overriding per-language execution.
        #[serde(rename = "moonTaskMapping")]
        pub moon_task_mapping: Option<HashMap<String, String>>,

        /// The fallback language to use when language detection fails.
        #[serde(rename = "defaultLanguage")]
        pub default_language: Option<String>,

        /// If true, treats the Claude CLI as OAuth-authenticated.
        #[serde(rename = "claudeUsesOauth")]
        pub claude_uses_oauth: Option<bool>,

        /// The environment variable name for the Claude API key.
        #[serde(rename = "claudeApiKeyEnv")]
        pub claude_api_key_env: Option<String>,

        /// If true, treats the Gemini CLI as OAuth-authenticated.
        #[serde(rename = "geminiUsesOauth")]
        pub gemini_uses_oauth: Option<bool>,

        /// The environment variable name for the Gemini API key.
        #[serde(rename = "geminiApiKeyEnv")]
        pub gemini_api_key_env: Option<String>,

        /// If true, treats the Codex CLI as OAuth-authenticated.
        #[serde(rename = "codexUsesOauth")]
        pub codex_uses_oauth: Option<bool>,

        /// The environment variable name for the Codex API key.
        #[serde(rename = "codexApiKeyEnv")]
        pub codex_api_key_env: Option<String>,

        // OpenAI Codex capabilities configuration
        /// The perceived rating of Codex's code analysis capabilities.
        #[serde(rename = "codexCodeAnalysisRating")]
        pub codex_code_analysis_rating: Option<f32>,

        /// The perceived rating of Codex's code generation capabilities.
        #[serde(rename = "codexCodeGenerationRating")]
        pub codex_code_generation_rating: Option<f32>,

        /// The perceived rating of Codex's complex reasoning capabilities.
        #[serde(rename = "codexComplexReasoningRating")]
        pub codex_complex_reasoning_rating: Option<f32>,

        /// The perceived rating of Codex's speed.
        #[serde(rename = "codexSpeedRating")]
        pub codex_speed_rating: Option<f32>,

        /// The context length supported by the Codex model.
        #[serde(rename = "codexContextLength")]
        pub codex_context_length: Option<u32>,

        /// Whether the Codex integration supports session-based interactions.
        #[serde(rename = "codexSupportsSession")]
        pub codex_supports_sessions: Option<bool>,

        // File processing
        /// The maximum number of files to process in a single task.
        #[serde(rename = "maxFilesPerTask")]
        pub max_files_per_task: Option<u32>,

        /// The complexity threshold above which files receive special handling.
        #[serde(rename = "complexityThreshold")]
        pub complexity_threshold: Option<f32>,

        /// A list of glob patterns to include in the analysis.
        #[serde(rename = "includePatterns")]
        pub include_patterns: Option<Vec<String>>,

        /// A list of glob patterns to exclude from the analysis.
        #[serde(rename = "excludePatterns")]
        pub exclude_patterns: Option<Vec<String>>,

        // Quality configuration
        /// The minimum quality score for code to be considered acceptable.
        #[serde(rename = "qualityThreshold")]
        pub quality_threshold: Option<f32>,

        /// If true, falls back to the "opus" model if the primary model fails.
        #[serde(rename = "enableOpusFallback")]
        pub enable_opus_fallback: Option<bool>,

        /// Custom prompt overrides from `workspace.yml` configuration.
        #[serde(rename = "customPrompts")]
        pub custom_prompts: Option<std::collections::HashMap<String, String>>,

        // Concurrency and performance controls
        /// The maximum number of concurrent requests to the AI provider.
        #[serde(rename = "maxConcurrentRequests")]
        pub max_concurrent_requests: Option<u32>,

        /// The number of files to process in a single batch.
        #[serde(rename = "batchSize")]
        pub batch_size: Option<u32>,

        /// The default operation mode for the extension ("fix", "lint-only", "comprehensive").
        #[serde(rename = "operationMode")]
        pub operation_mode: Option<String>,

        // Analysis configuration
        /// The maximum number of suggestions to generate per file.
        #[serde(rename = "maxSuggestions")]
        pub max_suggestions: Option<u32>,

        /// The minimum confidence level for a suggestion to be considered.
        #[serde(rename = "minConfidence")]
        pub min_confidence: Option<f32>,

        /// If true, automatically applies safe fixes.
        #[serde(rename = "enableAutoFix")]
        pub enable_auto_fix: Option<bool>,

        /// If true, enables parallel analysis of files.
        #[serde(rename = "parallelAnalysis")]
        pub parallel_analysis: Option<bool>,

        /// If true, includes performance metrics in the output.
        #[serde(rename = "includeMetrics")]
        pub include_metrics: Option<bool>,

        /// The timeout in seconds for analysis operations.
        #[serde(rename = "timeoutSeconds")]
        pub timeout_seconds: Option<u64>,

        // AI code fixer configuration
        /// Enables analysis of relationships between code elements.
        #[serde(rename = "enableRelationshipAnalysis")]
        pub enable_relationship_analysis: Option<bool>,

        /// Enables AI-powered generation of TSDoc comments.
        #[serde(rename = "enableAiTsdoc")]
        pub enable_ai_tsdoc: Option<bool>,

        /// The target TSDoc coverage percentage.
        #[serde(rename = "tsdocCoverageTarget")]
        pub tsdoc_coverage_target: Option<f64>,

        // Optimization configuration
        /// Enables the optimization workflow.
        #[serde(rename = "optimizationEnabled")]
        pub optimization_enabled: Option<bool>,

        /// The maximum number of iterations for the optimization process.
        #[serde(rename = "maxOptimizationIterations")]
        pub max_optimization_iterations: Option<usize>,

        /// The confidence threshold for accepting an optimization.
        #[serde(rename = "confidenceThreshold")]
        pub confidence_threshold: Option<f64>,

        // Workflow configuration
        /// Enables the multi-phase analysis workflow.
        #[serde(rename = "workflowEnabled")]
        pub workflow_enabled: Option<bool>,

        /// Enables parallel processing within the workflow.
        #[serde(rename = "workflowParallelProcessing")]
        pub workflow_parallel_processing: Option<bool>,

        /// The timeout in seconds for each workflow step.
        #[serde(rename = "workflowTimeoutSeconds")]
        pub workflow_timeout_seconds: Option<u64>,
    }
);

impl MoonShineConfig {
    /// Returns the appropriate AI model name based on the provider type.
    pub fn get_model_for_provider(&self, provider: &str) -> String {
        match provider.to_lowercase().as_str() {
            "claude" => "sonnet",
            "gemini" => "gemini-2.5-flash",
            "openai" | "codex" => "gpt5-codex-medium",
            _ => "sonnet", // Default fallback
        }
        .to_string()
    }

    /// Returns the configured AI model name, or a default if not specified.
    pub fn get_model(&self) -> String {
        self.ai_model
            .clone()
            .unwrap_or_else(|| "sonnet".to_string())
    }

    /// Updates the AI model to the default for a specific provider.
    pub fn set_model_for_provider(&mut self, provider: &str) {
        self.ai_model = Some(self.get_model_for_provider(provider));
    }

    /// Resolves the Moon task name for a given language.
    ///
    /// The task name is determined in the following order of precedence:
    /// 1. `moon_task_name` field if explicitly set.
    /// 2. `moon_task_mapping` for the given language.
    /// 3. A default based on the language.
    pub fn resolve_task_name(&self, language: &str) -> String {
        if let Some(explicit) = self.moon_task_name.as_ref() {
            return explicit.clone();
        }

        if let Some(mapping) = self.moon_task_mapping.as_ref() {
            if let Some(mapped) = mapping.get(language) {
                return mapped.clone();
            }
        }

        match language {
            "typescript" | "javascript" => "moon-shine:typescript".to_string(),
            "rust" => "moon-shine:rust".to_string(),
            "python" => "moon-shine:python".to_string(),
            "go" => "moon-shine:go".to_string(),
            "java" => "moon-shine:java".to_string(),
            "cpp" | "c" => "moon-shine:cpp".to_string(),
            _ => "moon-shine:lint".to_string(),
        }
    }

    /// Returns the optional default language for when detection fails.
    pub fn default_language(&self) -> Option<&str> {
        self.default_language.as_deref()
    }
}

/// Creates the JSON schema for the `moon-shine` extension configuration.
///
/// This schema is used by Moon to validate the configuration in `moon.yml`
/// and provide autocompletion in supported editors.
pub fn create_config_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "aiModel": {
                "type": "string",
                "enum": ["sonnet", "opus", "gemini-2.5-pro", "gemini-2.5-flash", "gpt5-codex"],
                "default": "sonnet",
                "description": "AI model for code analysis and fixes"
            },
            "enableCoproOptimization": {
                "type": "boolean",
                "default": true,
                "description": "Enable COPRO prompt optimization"
            },
            "maxConcurrentRequests": {
                "type": "integer",
                "default": 3,
                "minimum": 1,
                "maximum": 10,
                "description": "Maximum concurrent Claude API requests"
            },
            "batchSize": {
                "type": "integer",
                "default": 5,
                "minimum": 1,
                "maximum": 20,
                "description": "Number of files to process per batch"
            },
            "operationMode": {
                "type": "string",
                "enum": ["fix", "lint-only", "comprehensive"],
                "default": "fix",
                "description": "Default operation mode for the extension"
            },
            "moonTaskName": {
                "type": "string",
                "description": "Override the Moon task to run for every analysis invocation"
            },
            "moonTaskMapping": {
                "type": "object",
                "description": "Language to Moon task mapping (overrides per-language execution)",
                "additionalProperties": { "type": "string" }
            },
            "defaultLanguage": {
                "type": "string",
                "description": "Fallback language when detection cannot infer a value from the file"
            },
            "claudeUsesOauth": {
                "type": "boolean",
                "default": true,
                "description": "Treat Claude CLI as OAuth-authenticated (no API key lookup)"
            },
            "claudeApiKeyEnv": {
                "type": "string",
                "description": "Environment variable name holding a Claude API key when OAuth is disabled"
            },
            "geminiUsesOauth": {
                "type": "boolean",
                "default": true,
                "description": "Treat Gemini CLI as OAuth-authenticated (no API key lookup)"
            },
            "geminiApiKeyEnv": {
                "type": "string",
                "description": "Environment variable name holding a Gemini API key when OAuth is disabled"
            },
            "codexUsesOauth": {
                "type": "boolean",
                "default": true,
                "description": "Treat Codex CLI as OAuth-authenticated (no API key lookup)"
            },
            "codexApiKeyEnv": {
                "type": "string",
                "description": "Environment variable name holding a Codex API key when OAuth is disabled"
            },
            "includePatterns": {
                "type": "array",
                "items": { "type": "string" },
                "default": ["**/*.{ts,tsx,js,jsx}"],
                "description": "File patterns to include in analysis"
            },
            "excludePatterns": {
                "type": "array",
                "items": { "type": "string" },
                "default": ["node_modules/**", "dist/**", "build/**"],
                "description": "File patterns to exclude from analysis"
            },
            "qualityThreshold": {
                "type": "number",
                "default": 0.8,
                "minimum": 0.0,
                "maximum": 1.0,
                "description": "Minimum quality score for code acceptance"
            },
            "prompts": {
                "type": "object",
                "description": "Custom prompt overrides for specific rule types",
                "properties": {
                    "no_unused_vars": { "type": "string" },
                    "missing_types": { "type": "string" },
                    "no_console": { "type": "string" },
                    "missing_jsdoc": { "type": "string" },
                    "async_best_practices": { "type": "string" },
                    "pass_1_compilation_critical": { "type": "string" },
                    "pass_2_type_safety_implementation": { "type": "string" },
                    "pass_3_code_quality_google_style": { "type": "string" }
                }
            }
        }
    })
}

impl MoonShineConfig {
    /// Loads the `moon-shine` configuration from the Moon workspace.
    ///
    /// This function uses the Moon PDK to safely access the extension's configuration,
    /// applies any necessary overrides, validates the settings, and loads custom prompts.
    pub fn from_moon_workspace() -> Result<Self> {
        let mut config = get_extension_config::<Self>().unwrap_or_default();

        config.apply_overrides()?;
        config.validate_and_fix()?;

        if let Ok(Some(prompts)) = Self::load_custom_prompts() {
            config.custom_prompts = Some(prompts);
        }

        Ok(config)
    }

    /// Returns the configured pattern rules, or a default set if not specified.
    pub fn get_pattern_rules(&self) -> crate::pattern_config::PatternConfig {
        self.pattern_rules.clone().unwrap_or_default()
    }

    /// Returns the version of the `moon-shine` extension from `Cargo.toml`.
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Returns the path to the `moonshine` data directory within the `.moon` directory.
    ///
    /// This function safely determines the directory path in a WASM-compatible way
    /// by checking Moon configuration and environment variables via the PDK.
    pub fn moonshine_directory() -> String {
        if let Ok(Some(moon_dir)) = get_moon_config_safe("moonshine_directory") {
            return moon_dir;
        }

        if let Ok(Some(ext_dir)) = get_moon_config_safe("extension.moonshine.data_directory") {
            return ext_dir;
        }

        if let Ok(Some(env_dir)) = get_moon_config_safe("env.MOONSHINE_DIR") {
            return env_dir;
        }

        ".moon/moonshine".to_string()
    }

    /// Validates the current configuration values.
    ///
    /// Returns an error if any configuration setting is invalid.
    pub fn validate(&self) -> Result<()> {
        if let Some(ref model) = self.ai_model {
            match model.as_str() {
                "sonnet" | "opus" | "gemini-2.5-pro" | "gemini-2.5-flash" | "gpt5-codex" => {}
                _ => return Err(Error::config(format!("Invalid AI model: {}", model))),
            }
        }

        if let Some(breadth) = self.copro_breadth {
            if !(1..=20).contains(&breadth) {
                return Err(Error::config(format!(
                    "copro_breadth must be between 1 and 20, got {}",
                    breadth
                )));
            }
        }

        if let Some(depth) = self.copro_depth {
            if !(1..=10).contains(&depth) {
                return Err(Error::config(format!(
                    "copro_depth must be between 1 and 10, got {}",
                    depth
                )));
            }
        }

        if let Some(temp) = self.copro_temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(Error::config(format!(
                    "copro_temperature must be between 0.0 and 2.0, got {}",
                    temp
                )));
            }
        }

        if let Some(rate) = self.pattern_learning_rate {
            if !(0.0..=1.0).contains(&rate) {
                return Err(Error::config(format!(
                    "pattern_learning_rate must be between 0.0 and 1.0, got {}",
                    rate
                )));
            }
        }

        if let Some(freq) = self.pattern_min_frequency {
            if !(0.0..=1.0).contains(&freq) {
                return Err(Error::config(format!(
                    "pattern_min_frequency must be between 0.0 and 1.0, got {}",
                    freq
                )));
            }
        }

        if let Some(files) = self.max_files_per_task {
            if !(1..=100).contains(&files) {
                return Err(Error::config(format!(
                    "max_files_per_task must be between 1 and 100, got {}",
                    files
                )));
            }
        }

        if let Some(complexity) = self.complexity_threshold {
            if !(1.0..=100.0).contains(&complexity) {
                return Err(Error::config(format!(
                    "complexity_threshold must be between 1.0 and 100.0, got {}",
                    complexity
                )));
            }
        }

        if let Some(quality) = self.quality_threshold {
            if !(0.0..=1.0).contains(&quality) {
                return Err(Error::config(format!(
                    "quality_threshold must be between 0.0 and 1.0, got {}",
                    quality
                )));
            }
        }

        if let Some(concurrent) = self.max_concurrent_requests {
            if !(1..=10).contains(&concurrent) {
                return Err(Error::config(format!(
                    "max_concurrent_requests must be between 1 and 10, got {}",
                    concurrent
                )));
            }
        }

        if let Some(batch) = self.batch_size {
            if !(1..=50).contains(&batch) {
                return Err(Error::config(format!(
                    "batch_size must be between 1 and 50, got {}",
                    batch
                )));
            }
        }

        if let Some(ref mode) = self.operation_mode {
            match mode.as_str() {
                "fix" | "lint-only" | "comprehensive" => {}
                _ => return Err(Error::config(format!("Invalid operation mode: {}", mode))),
            }
        }

        Ok(())
    }

    /// Parses a string into a boolean with flexible matching.
    pub fn parse_bool(s: &str) -> Result<bool> {
        match s.to_lowercase().trim() {
            "true" | "1" | "yes" | "on" | "enabled" => Ok(true),
            "false" | "0" | "no" | "off" | "disabled" => Ok(false),
            _ => Err(Error::config(format!(
                "Invalid boolean value: '{}'. Expected: true/false, yes/no, on/off, enabled/disabled, 1/0",
                s
            ))),
        }
    }

    /// Parses a string into a `u32`.
    pub fn parse_u32(s: &str) -> Result<u32> {
        s.trim()
            .parse::<u32>()
            .map_err(|_| Error::config(format!("Invalid unsigned integer: '{}'", s)))
    }

    /// Parses a string into an `f32`.
    pub fn parse_f32(s: &str) -> Result<f32> {
        s.trim()
            .parse::<f32>()
            .map_err(|_| Error::config(format!("Invalid float value: '{}'", s)))
    }

    /// Validates configuration values and corrects any out-of-range settings to their defaults.
    pub fn validate_and_fix(&mut self) -> Result<()> {
        if let Some(temp) = self.temperature {
            if !(0.0..=2.0).contains(&temp) {
                self.temperature = Some(0.7);
            }
        }

        if let Some(top_p) = self.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                self.top_p = Some(0.9);
            }
        }

        if let Some(tokens) = self.max_tokens {
            if !(100..=32000).contains(&tokens) {
                self.max_tokens = Some(8192);
            }
        }

        if let Some(breadth) = self.copro_breadth {
            if !(1..=50).contains(&breadth) {
                self.copro_breadth = Some(10);
            }
        }

        if let Some(depth) = self.copro_depth {
            if !(1..=20).contains(&depth) {
                self.copro_depth = Some(5);
            }
        }

        Ok(())
    }

    /// Applies configuration overrides in a WASM-compatible manner.
    ///
    /// In a WASM environment, all configuration must come through the Moon PDK.
    /// This function ensures that the loaded configuration is validated and fixed.
    pub fn apply_overrides(&mut self) -> Result<()> {
        self.validate_and_fix()?;
        Ok(())
    }

    /// Parses a delimited string into a vector of strings.
    ///
    /// Supports multiple delimiters: comma, semicolon, pipe, and newline.
    pub fn parse_string_list(s: &str) -> Vec<String> {
        if s.trim().is_empty() {
            return Vec::new();
        }

        let delimiters = [',', ';', '|', '\n'];
        let mut result = Vec::new();

        for delimiter in delimiters {
            if s.contains(delimiter) {
                result = s
                    .split(delimiter)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                break;
            }
        }

        if result.is_empty() {
            result.push(s.trim().to_string());
        }

        result
    }

    /// Applies environment variable overrides through the Moon PDK for WASM compatibility.
    pub fn apply_env_overrides(&mut self) -> Result<()> {
        if let Ok(Some(model)) = get_moon_config_safe("env.MOONSHINE_AI_MODEL") {
            self.ai_model = Some(model);
        }

        if let Ok(Some(temp_str)) = get_moon_config_safe("env.MOONSHINE_TEMPERATURE") {
            if let Ok(temp) = temp_str.parse::<f32>() {
                if (0.0..=2.0).contains(&temp) {
                    self.temperature = Some(temp);
                }
            }
        }

        if let Ok(Some(top_p_str)) = get_moon_config_safe("env.MOONSHINE_TOP_P") {
            if let Ok(top_p) = top_p_str.parse::<f32>() {
                if (0.0..=1.0).contains(&top_p) {
                    self.top_p = Some(top_p);
                }
            }
        }

        if let Ok(Some(tokens_str)) = get_moon_config_safe("env.MOONSHINE_MAX_TOKENS") {
            if let Ok(tokens) = tokens_str.parse::<u32>() {
                if tokens > 0 && tokens <= 200000 {
                    self.max_tokens = Some(tokens);
                }
            }
        }

        if let Ok(Some(copro_str)) = get_moon_config_safe("env.MOONSHINE_ENABLE_COPRO") {
            self.enable_copro_optimization = Some(copro_str.to_lowercase() == "true");
        }

        if let Ok(Some(pattern_str)) =
            get_moon_config_safe("env.MOONSHINE_ENABLE_PATTERN_DETECTION")
        {
            self.enable_pattern_detection = Some(pattern_str.to_lowercase() == "true");
        }

        if let Ok(Some(auto_fix_str)) = get_moon_config_safe("env.MOONSHINE_ENABLE_AUTO_FIX") {
            self.enable_auto_fix = Some(auto_fix_str.to_lowercase() == "true");
        }

        if let Ok(Some(eslint_str)) = get_moon_config_safe("env.MOONSHINE_ENABLE_ESLINT") {
            self.enable_eslint_integration = Some(eslint_str.to_lowercase() == "true");
        }

        if let Ok(Some(ts_str)) = get_moon_config_safe("env.MOONSHINE_ENABLE_TYPESCRIPT") {
            self.enable_typescript_integration = Some(ts_str.to_lowercase() == "true");
        }

        Ok(())
    }

    /// Loads custom prompts from various sources within the Moon workspace.
    ///
    /// This function aggregates prompts from default templates, Moon extension configuration,
    /// DSPy optimized prompts, project-specific configuration, and language-specific settings.
    fn load_custom_prompts() -> Result<Option<HashMap<String, String>>> {
        let mut prompts = HashMap::new();

        prompts.insert(
            "compilation_critical".to_string(),
            "Focus on compilation errors, syntax issues, and critical TypeScript problems that prevent code execution.".to_string(),
        );
        prompts.insert(
            "type_safety".to_string(),
            "Analyze type safety, missing type annotations, and potential runtime type errors.".to_string(),
        );
        prompts.insert(
            "code_quality".to_string(),
            "Review code quality, best practices, performance optimizations, and maintainability improvements.".to_string(),
        );
        prompts.insert(
            "performance".to_string(),
            "Optimize performance bottlenecks, reduce bundle size, and improve runtime efficiency.".to_string(),
        );
        prompts.insert(
            "accessibility".to_string(),
            "Ensure accessibility compliance, ARIA attributes, and inclusive design patterns.".to_string(),
        );
        prompts.insert(
            "security".to_string(),
            "Identify security vulnerabilities, sanitize inputs, and implement secure coding practices.".to_string(),
        );

        if let Ok(Some(custom_prompts_json)) =
            get_moon_config_safe("extension.moonshine.custom_prompts")
        {
            if let Ok(custom_prompts) =
                serde_json::from_str::<HashMap<String, String>>(&custom_prompts_json)
            {
                for (key, value) in custom_prompts {
                    if !key.trim().is_empty() && !value.trim().is_empty() && value.len() <= 5000 {
                        prompts.insert(key, value);
                    }
                }
            }
        }

        if let Ok(Some(optimized_prompts_json)) =
            get_moon_config_safe("extension.moonshine.optimized_prompts")
        {
            if let Ok(optimized_prompts) =
                serde_json::from_str::<HashMap<String, String>>(&optimized_prompts_json)
            {
                for (key, value) in optimized_prompts {
                    if !key.trim().is_empty() && !value.trim().is_empty() && value.len() <= 10000 {
                        prompts.insert(format!("optimized_{}", key), value);
                    }
                }
            }
        }

        if let Ok(Some(project_prompts_json)) = get_moon_config_safe("project.moonshine.prompts") {
            if let Ok(project_prompts) =
                serde_json::from_str::<HashMap<String, String>>(&project_prompts_json)
            {
                for (key, value) in project_prompts {
                    if !key.trim().is_empty() && !value.trim().is_empty() && value.len() <= 15000 {
                        prompts.insert(format!("project_{}", key), value);
                    }
                }
            }
        }

        let languages = ["typescript", "javascript", "rust", "python", "go", "java"];
        for lang in &languages {
            let config_key = format!("extension.moonshine.prompts.{}", lang);
            if let Ok(Some(lang_prompt)) = get_moon_config_safe(&config_key) {
                if !lang_prompt.trim().is_empty() && lang_prompt.len() <= 8000 {
                    prompts.insert(format!("{}_specific", lang), lang_prompt);
                }
            }
        }

        if prompts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(prompts))
        }
    }

    /// Saves optimized prompts back to the workspace via Moon tasks.
    ///
    /// This function validates the prompts, serializes them to JSON, and communicates
    /// with a Moon task to handle the actual file writing, ensuring WASM compatibility.
    pub fn save_optimized_prompts(prompts: &HashMap<String, String>) -> Result<()> {
        if prompts.is_empty() {
            return Err(Error::config(
                "Cannot save empty prompts collection".to_string(),
            ));
        }

        for (key, value) in prompts {
            if key.trim().is_empty() {
                return Err(Error::config("Prompt key cannot be empty".to_string()));
            }
            if value.trim().is_empty() {
                return Err(Error::config(format!(
                    "Prompt value for '{}' cannot be empty",
                    key
                )));
            }
            if value.len() > 50000 {
                return Err(Error::config(format!(
                    "Prompt '{}' exceeds maximum length of 50KB",
                    key
                )));
            }
            if value
                .chars()
                .any(|c| c.is_control() && c != '\n' && c != '\t' && c != '\r')
            {
                return Err(Error::config(format!(
                    "Prompt '{}' contains invalid control characters",
                    key
                )));
            }
        }

        let json = serde_json::to_string_pretty(prompts)
            .map_err(|e| Error::config(format!("Failed to serialize prompts: {}", e)))?;

        let save_metadata = serde_json::json!({
            "action": "save_optimized_prompts",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "prompt_count": prompts.len(),
            "total_size_bytes": json.len(),
            "data": prompts
        });

        let json_value: serde_json::Value = serde_json::from_str(&json)
            .map_err(|e| Error::config(format!("Failed to parse JSON for Moon PDK: {}", e)))?;

        if let Err(e) = crate::moon_pdk_interface::update_prompts_json(&json_value) {
            eprintln!(
                "Warning: Failed to save optimized prompts via Moon PDK: {}",
                e
            );
            if let Err(e2) =
                crate::moon_pdk_interface::request_storage_update("prompts", &save_metadata)
            {
                return Err(Error::config(format!(
                    "Failed to save prompts: Primary save failed ({}), Fallback failed ({})",
                    e, e2
                )));
            }
        }

        let success_indicator = serde_json::json!({
            "saved_at": chrono::Utc::now().to_rfc3339(),
            "prompt_keys": prompts.keys().collect::<Vec<_>>(),
            "status": "ready_for_persistence"
        });

        let _ = crate::moon_pdk_interface::request_storage_update(
            "prompt_save_status",
            &success_indicator,
        );

        Ok(())
    }
}

// LMConfig removed - use MoonShineConfig directly for all configuration

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validation() {
        let config = MoonShineConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_model() {
        let mut config = MoonShineConfig::default();
        config.ai_model = Some("invalid-model".to_string());
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_ranges() {
        let mut config = MoonShineConfig::default();

        config.copro_breadth = Some(0);
        assert!(config.validate().is_err());
        config.copro_breadth = Some(25);
        assert!(config.validate().is_err());
        config.copro_breadth = Some(5);
        assert!(config.validate().is_ok());

        config.copro_temperature = Some(-1.0);
        assert!(config.validate().is_err());
        config.copro_temperature = Some(3.0);
        assert!(config.validate().is_err());
        config.copro_temperature = Some(1.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(MoonShineConfig::parse_bool("true").unwrap(), true);
        assert_eq!(MoonShineConfig::parse_bool("false").unwrap(), false);
        assert_eq!(MoonShineConfig::parse_bool("1").unwrap(), true);
        assert_eq!(MoonShineConfig::parse_bool("0").unwrap(), false);
        assert_eq!(MoonShineConfig::parse_bool("yes").unwrap(), true);
        assert_eq!(MoonShineConfig::parse_bool("no").unwrap(), false);
        assert!(MoonShineConfig::parse_bool("invalid").is_err());
    }

    #[test]
    fn test_parse_string_list() {
        let result = MoonShineConfig::parse_string_list("a,b,c");
        assert_eq!(result, vec!["a", "b", "c"]);

        let result = MoonShineConfig::parse_string_list("a, b , c ");
        assert_eq!(result, vec!["a", "b", "c"]);

        let result = MoonShineConfig::parse_string_list("");
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_version() {
        let version = MoonShineConfig::version();
        assert!(!version.is_empty());
        assert!(version.chars().next().unwrap().is_ascii_digit());
    }

    #[test]
    fn test_moonshine_directory() {
        assert_eq!(MoonShineConfig::moonshine_directory(), ".moon/moonshine");
    }
}
