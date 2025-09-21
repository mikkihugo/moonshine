//! Configuration structures and schemas for Moon Shine extension

use crate::error::{Error, Result};
use crate::moon_pdk_interface::get_moon_config_safe;
use moon_pdk::get_extension_config;
use moon_pdk_api::config_struct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

config_struct!(
  #[derive(Clone, Default, Serialize)]
  pub struct MoonShineConfig {
    // Core AI configuration
    // <!-- TODO: The `MoonShineConfig` struct is quite large. Consider grouping related fields into smaller, more focused structs (e.g., `CoproConfig`, `PatternDetectionConfig`, `CodexConfig`) to improve organization and readability. -->
    #[serde(rename = "aiModel")]
    pub ai_model: Option<String>, // "sonnet", "opus", etc.

    #[serde(rename = "aiProviders")]
    pub ai_providers: Option<Vec<String>>, // List of available AI providers

    // LM Parameters (consolidated from lm_config.rs)
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>, // Model creativity (0.0-2.0, default: 0.7)

    #[serde(rename = "topP")]
    pub top_p: Option<f32>, // Nucleus sampling (0.0-1.0, default: 0.0)

    #[serde(rename = "maxTokens")]
    pub max_tokens: Option<u32>, // Maximum tokens in response (default: 8192)

    #[serde(rename = "maxCompletionTokens")]
    pub max_completion_tokens: Option<u32>, // Maximum completion tokens (default: 8192)

    #[serde(rename = "enableCoproOptimization")]
    pub enable_copro_optimization: Option<bool>, // Enable COPRO mathematical optimization

    #[serde(rename = "enablePatternDetection")]
    pub enable_pattern_detection: Option<bool>, // Enable AI code pattern learning

    // COPRO configuration
    #[serde(rename = "coproBreadth")]
    pub copro_breadth: Option<u32>, // Prompt candidates per iteration (default: 5)

    #[serde(rename = "coproDepth")]
    pub copro_depth: Option<u32>, // Optimization iterations (default: 3)

    #[serde(rename = "coproTemperature")]
    pub copro_temperature: Option<f32>, // Creativity in generation (default: 1.0)

    // Pattern detection configuration
    #[serde(rename = "patternLearningRate")]
    pub pattern_learning_rate: Option<f32>, // How quickly to learn patterns (default: 0.1)

    #[serde(rename = "patternMinFrequency")]
    pub pattern_min_frequency: Option<f32>, // Minimum frequency to consider pattern (default: 0.3)

    // Configurable pattern rules for code analysis
    #[serde(rename = "patternRules")]
    pub pattern_rules: Option<crate::pattern_config::PatternConfig>,

    // Tool integration
    #[serde(rename = "enableEslintIntegration")]
    pub enable_eslint_integration: Option<bool>,

    #[serde(rename = "enableTypescriptIntegration")]
    pub enable_typescript_integration: Option<bool>,

    #[serde(rename = "enablePrettierIntegration")]
    pub enable_prettier_integration: Option<bool>,

    // OpenAI Codex CLI integration
    #[serde(rename = "enableCodexIntegration")]
    pub enable_codex_integration: Option<bool>,

    #[serde(rename = "codexModel")]
    pub codex_model: Option<String>, // Default: "gpt-5-codex"

    #[serde(rename = "codexCommand")]
    pub codex_command: Option<String>, // Default: "/home/mhugo/.npm-global/bin/codex"

    #[serde(rename = "codexReasoningEffort")]
    pub codex_reasoning_effort: Option<String>, // Default: "low"

    #[serde(rename = "moonTaskName")]
    pub moon_task_name: Option<String>,

    #[serde(rename = "moonTaskMapping")]
    pub moon_task_mapping: Option<HashMap<String, String>>,

    #[serde(rename = "defaultLanguage")]
    pub default_language: Option<String>,

    #[serde(rename = "claudeUsesOauth")]
    pub claude_uses_oauth: Option<bool>,

    #[serde(rename = "claudeApiKeyEnv")]
    pub claude_api_key_env: Option<String>,

    #[serde(rename = "geminiUsesOauth")]
    pub gemini_uses_oauth: Option<bool>,

    #[serde(rename = "geminiApiKeyEnv")]
    pub gemini_api_key_env: Option<String>,

    #[serde(rename = "codexUsesOauth")]
    pub codex_uses_oauth: Option<bool>,

    #[serde(rename = "codexApiKeyEnv")]
    pub codex_api_key_env: Option<String>,

    // OpenAI Codex capabilities configuration
    #[serde(rename = "codexCodeAnalysisRating")]
    pub codex_code_analysis_rating: Option<f32>, // Default: 0.88

    #[serde(rename = "codexCodeGenerationRating")]
    pub codex_code_generation_rating: Option<f32>, // Default: 0.95

    #[serde(rename = "codexComplexReasoningRating")]
    pub codex_complex_reasoning_rating: Option<f32>, // Default: 0.85

    #[serde(rename = "codexSpeedRating")]
    pub codex_speed_rating: Option<f32>, // Default: 0.85

    #[serde(rename = "codexContextLength")]
    pub codex_context_length: Option<u32>, // Default: 200000

    #[serde(rename = "codexSupportsSession")]
    pub codex_supports_sessions: Option<bool>, // Default: true

    // File processing
    #[serde(rename = "maxFilesPerTask")]
    pub max_files_per_task: Option<u32>,

    #[serde(rename = "complexityThreshold")]
    pub complexity_threshold: Option<f32>,

    #[serde(rename = "includePatterns")]
    pub include_patterns: Option<Vec<String>>,

    #[serde(rename = "excludePatterns")]
    pub exclude_patterns: Option<Vec<String>>,

    // Quality configuration
    #[serde(rename = "qualityThreshold")]
    pub quality_threshold: Option<f32>,

    #[serde(rename = "enableOpusFallback")]
    pub enable_opus_fallback: Option<bool>,

    /// Custom prompt overrides from workspace.yml configuration
    #[serde(rename = "customPrompts")]
    pub custom_prompts: Option<std::collections::HashMap<String, String>>,

    // Concurrency and performance controls
    #[serde(rename = "maxConcurrentRequests")]
    pub max_concurrent_requests: Option<u32>,

    #[serde(rename = "batchSize")]
    pub batch_size: Option<u32>,

    #[serde(rename = "operationMode")]
    pub operation_mode: Option<String>,

    // Analysis configuration (from AnalysisConfig)
    #[serde(rename = "maxSuggestions")]
    pub max_suggestions: Option<u32>,

    #[serde(rename = "minConfidence")]
    pub min_confidence: Option<f32>,

    #[serde(rename = "enableAutoFix")]
    pub enable_auto_fix: Option<bool>,

    #[serde(rename = "parallelAnalysis")]
    pub parallel_analysis: Option<bool>,

    #[serde(rename = "includeMetrics")]
    pub include_metrics: Option<bool>,

    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<u64>,

    // AI code fixer configuration (from ClaudeFixerConfig)
    #[serde(rename = "enableRelationshipAnalysis")]
    pub enable_relationship_analysis: Option<bool>,

    #[serde(rename = "enableAiTsdoc")]
    pub enable_ai_tsdoc: Option<bool>,

    #[serde(rename = "tsdocCoverageTarget")]
    pub tsdoc_coverage_target: Option<f64>,

    // Optimization configuration (from OptimizationConfig)
    #[serde(rename = "optimizationEnabled")]
    pub optimization_enabled: Option<bool>,

    #[serde(rename = "maxOptimizationIterations")]
    pub max_optimization_iterations: Option<usize>,

    #[serde(rename = "confidenceThreshold")]
    pub confidence_threshold: Option<f64>,

    // Workflow configuration (from WorkflowConfig)
    #[serde(rename = "workflowEnabled")]
    pub workflow_enabled: Option<bool>,

    #[serde(rename = "workflowParallelProcessing")]
    pub workflow_parallel_processing: Option<bool>,

    #[serde(rename = "workflowTimeoutSeconds")]
    pub workflow_timeout_seconds: Option<u64>,
  }
);

// Default implementation generated by config_struct! macro

impl MoonShineConfig {
  /// Get the appropriate model name based on provider type
  pub fn get_model_for_provider(&self, provider: &str) -> String {
    match provider.to_lowercase().as_str() {
      "claude" => "sonnet",
      "gemini" => "gemini-2.5-flash",
      "openai" | "codex" => "gpt5-codex-medium",
      _ => "sonnet", // Default fallback
    }
    .to_string()
  }

  /// Get the actual model name, using provider defaults if not specified
  pub fn get_model(&self) -> String {
    self
      .ai_model
      .clone()
      .unwrap_or_else(|| "sonnet".to_string())
  }

  /// Update model for a specific provider
  pub fn set_model_for_provider(&mut self, provider: &str) {
    self.ai_model = Some(self.get_model_for_provider(provider));
  }

  /// Resolve the Moon task name for a detected language.
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

  /// Optional default language when detection fails.
  pub fn default_language(&self) -> Option<&str> {
    self.default_language.as_deref()
  }
}

/// Create configuration schema for Moon workspace.yml integration
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
  /// Load configuration from Moon workspace with robust error handling
  pub fn from_moon_workspace() -> Result<Self> {
    // Use the proper Moon PDK approach to get extension configuration
    // PDK handles both configuration values and environment variables
    let mut config = get_extension_config::<Self>().unwrap_or_default();

    // Apply WASM-compatible overrides and validation
    config.apply_overrides()?;
    config.validate_and_fix()?;

    // Load custom prompts (WASM-compatible defaults)
    if let Ok(Some(prompts)) = Self::load_custom_prompts() {
      config.custom_prompts = Some(prompts);
    }

    Ok(config)
  }

  /// Get configured pattern rules or defaults
  pub fn get_pattern_rules(&self) -> crate::pattern_config::PatternConfig {
    self.pattern_rules.clone().unwrap_or_default()
  }

  /// Get the extension version from Cargo.toml at compile time
  pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
  }

  /// Get moonshine directory path - WASM-compatible with Moon PDK integration
  pub fn moonshine_directory() -> String {
    // Production implementation: Get directory through Moon PDK configuration
    // Priority: Moon config > Host env vars > Default fallback

    // 1. Try Moon workspace configuration first
    if let Ok(Some(moon_dir)) = get_moon_config_safe("moonshine_directory") {
      return moon_dir;
    }

    // 2. Try Moon extension configuration
    if let Ok(Some(ext_dir)) =
      get_moon_config_safe("extension.moonshine.data_directory")
    {
      return ext_dir;
    }

    // 3. Try host environment variable through Moon PDK
    if let Ok(Some(env_dir)) = get_moon_config_safe("env.MOONSHINE_DIR") {
      return env_dir;
    }

    // 4. Production fallback: Standard Moon extension directory
    ".moon/moonshine".to_string()
  }

  /// Validate configuration values and return errors for invalid settings
  pub fn validate(&self) -> Result<()> {
    // Validate AI model
    if let Some(ref model) = self.ai_model {
      match model.as_str() {
        "sonnet" | "opus" | "gemini-2.5-pro" | "gemini-2.5-flash"
        | "gpt5-codex" => {}
        _ => return Err(Error::config(format!("Invalid AI model: {}", model))),
      }
    }

    // Validate numeric ranges
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

    // Validate operation mode
    if let Some(ref mode) = self.operation_mode {
      match mode.as_str() {
        "fix" | "lint-only" | "comprehensive" => {}
        _ => {
          return Err(Error::config(format!(
            "Invalid operation mode: {}",
            mode
          )))
        }
      }
    }

    Ok(())
  }

  // Enhanced helper parsing methods for production configuration validation
  pub fn parse_bool(s: &str) -> Result<bool> {
    match s.to_lowercase().trim() {
      "true" | "1" | "yes" | "on" | "enabled" => Ok(true),
      "false" | "0" | "no" | "off" | "disabled" => Ok(false),
      _ => Err(Error::config(format!("Invalid boolean value: '{}'. Expected: true/false, yes/no, on/off, enabled/disabled, 1/0", s))),
    }
  }

  pub fn parse_u32(s: &str) -> Result<u32> {
    s.trim()
      .parse::<u32>()
      .map_err(|_| Error::config(format!("Invalid unsigned integer: '{}'", s)))
  }

  pub fn parse_f32(s: &str) -> Result<f32> {
    s.trim()
      .parse::<f32>()
      .map_err(|_| Error::config(format!("Invalid float value: '{}'", s)))
  }

  /// Validate configuration values and fix any out-of-range settings
  pub fn validate_and_fix(&mut self) -> Result<()> {
    // Validate temperature range
    if let Some(temp) = self.temperature {
      if !(0.0..=2.0).contains(&temp) {
        self.temperature = Some(0.7); // Reset to default
      }
    }

    // Validate top_p range
    if let Some(top_p) = self.top_p {
      if !(0.0..=1.0).contains(&top_p) {
        self.top_p = Some(0.9); // Reset to default
      }
    }

    // Validate max_tokens
    if let Some(tokens) = self.max_tokens {
      if !(100..=32000).contains(&tokens) {
        self.max_tokens = Some(8192); // Reset to default
      }
    }

    // Validate COPRO parameters
    if let Some(breadth) = self.copro_breadth {
      if !(1..=50).contains(&breadth) {
        self.copro_breadth = Some(10); // Reset to default
      }
    }

    if let Some(depth) = self.copro_depth {
      if !(1..=20).contains(&depth) {
        self.copro_depth = Some(5); // Reset to default
      }
    }

    Ok(())
  }

  /// Apply configuration overrides (WASM-compatible)
  pub fn apply_overrides(&mut self) -> Result<()> {
    // In WASM, we can only get configuration through Moon PDK
    // Environment variables and file system are not available
    // All configuration must come through the Moon extension config

    // Validate and fix any invalid values
    self.validate_and_fix()?;

    Ok(())
  }

  pub fn parse_string_list(s: &str) -> Vec<String> {
    if s.trim().is_empty() {
      return Vec::new();
    }

    // Support multiple delimiters: comma, semicolon, pipe, newline
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

    // If no delimiters found, treat as single item
    if result.is_empty() {
      result.push(s.trim().to_string());
    }

    result
  }

  /// Apply environment variable overrides through Moon PDK for WASM compatibility
  pub fn apply_env_overrides(&mut self) -> Result<()> {
    // Production implementation: Get environment variables through Moon PDK
    // WASM-compatible environment variable access through Moon configuration system

    // AI Model configuration
    if let Ok(Some(model)) = get_moon_config_safe("env.MOONSHINE_AI_MODEL") {
      self.ai_model = Some(model);
    }

    // Temperature configuration
    if let Ok(Some(temp_str)) =
      get_moon_config_safe("env.MOONSHINE_TEMPERATURE")
    {
      if let Ok(temp) = temp_str.parse::<f32>() {
        if (0.0..=2.0).contains(&temp) {
          self.temperature = Some(temp);
        }
      }
    }

    // Top-P configuration
    if let Ok(Some(top_p_str)) = get_moon_config_safe("env.MOONSHINE_TOP_P") {
      if let Ok(top_p) = top_p_str.parse::<f32>() {
        if (0.0..=1.0).contains(&top_p) {
          self.top_p = Some(top_p);
        }
      }
    }

    // Max tokens configuration
    if let Ok(Some(tokens_str)) =
      get_moon_config_safe("env.MOONSHINE_MAX_TOKENS")
    {
      if let Ok(tokens) = tokens_str.parse::<u32>() {
        if tokens > 0 && tokens <= 200000 {
          self.max_tokens = Some(tokens);
        }
      }
    }

    // Enable/disable features via environment
    if let Ok(Some(copro_str)) =
      get_moon_config_safe("env.MOONSHINE_ENABLE_COPRO")
    {
      self.enable_copro_optimization = Some(copro_str.to_lowercase() == "true");
    }

    if let Ok(Some(pattern_str)) =
      get_moon_config_safe("env.MOONSHINE_ENABLE_PATTERN_DETECTION")
    {
      self.enable_pattern_detection =
        Some(pattern_str.to_lowercase() == "true");
    }

    if let Ok(Some(auto_fix_str)) =
      get_moon_config_safe("env.MOONSHINE_ENABLE_AUTO_FIX")
    {
      self.enable_auto_fix = Some(auto_fix_str.to_lowercase() == "true");
    }

    if let Ok(Some(eslint_str)) =
      get_moon_config_safe("env.MOONSHINE_ENABLE_ESLINT")
    {
      self.enable_eslint_integration =
        Some(eslint_str.to_lowercase() == "true");
    }

    if let Ok(Some(ts_str)) =
      get_moon_config_safe("env.MOONSHINE_ENABLE_TYPESCRIPT")
    {
      self.enable_typescript_integration =
        Some(ts_str.to_lowercase() == "true");
    }

    // Note: parallel_threads configuration handled through Moon's task system

    Ok(())
  }

  fn load_custom_prompts() -> Result<Option<HashMap<String, String>>> {
    let mut prompts = HashMap::new();

    // Load default prompt templates first
    prompts.insert("compilation_critical".to_string(),
      "Focus on compilation errors, syntax issues, and critical TypeScript problems that prevent code execution.".to_string());
    prompts.insert("type_safety".to_string(),
      "Analyze type safety, missing type annotations, and potential runtime type errors.".to_string());
    prompts.insert("code_quality".to_string(),
      "Review code quality, best practices, performance optimizations, and maintainability improvements.".to_string());
    prompts.insert("performance".to_string(),
      "Optimize performance bottlenecks, reduce bundle size, and improve runtime efficiency.".to_string());
    prompts.insert("accessibility".to_string(),
      "Ensure accessibility compliance, ARIA attributes, and inclusive design patterns.".to_string());
    prompts.insert("security".to_string(),
      "Identify security vulnerabilities, sanitize inputs, and implement secure coding practices.".to_string());

    // Load custom prompts from Moon extension configuration (WASM-compatible)
    if let Ok(Some(custom_prompts_json)) =
      get_moon_config_safe("extension.moonshine.custom_prompts")
    {
      match serde_json::from_str::<HashMap<String, String>>(
        &custom_prompts_json,
      ) {
        Ok(custom_prompts) => {
          // Validate custom prompts before merging
          for (key, value) in custom_prompts {
            if !key.trim().is_empty()
              && !value.trim().is_empty()
              && value.len() <= 5000
            {
              prompts.insert(key, value);
            }
          }
        }
        Err(e) => {
          // Log parsing error but continue with defaults
          eprintln!(
            "Warning: Failed to parse custom prompts from Moon config: {}",
            e
          );
        }
      }
    }

    // Load prompts from Moon task communication for DSPy optimized prompts
    if let Ok(Some(optimized_prompts_json)) =
      get_moon_config_safe("extension.moonshine.optimized_prompts")
    {
      match serde_json::from_str::<HashMap<String, String>>(
        &optimized_prompts_json,
      ) {
        Ok(optimized_prompts) => {
          // Optimized prompts override defaults and customs
          for (key, value) in optimized_prompts {
            if !key.trim().is_empty()
              && !value.trim().is_empty()
              && value.len() <= 10000
            {
              prompts.insert(format!("optimized_{}", key), value);
            }
          }
        }
        Err(e) => {
          eprintln!(
            "Warning: Failed to parse optimized prompts from Moon config: {}",
            e
          );
        }
      }
    }

    // Load project-specific prompts from Moon project configuration
    if let Ok(Some(project_prompts_json)) =
      get_moon_config_safe("project.moonshine.prompts")
    {
      match serde_json::from_str::<HashMap<String, String>>(
        &project_prompts_json,
      ) {
        Ok(project_prompts) => {
          // Project prompts have highest priority
          for (key, value) in project_prompts {
            if !key.trim().is_empty()
              && !value.trim().is_empty()
              && value.len() <= 15000
            {
              prompts.insert(format!("project_{}", key), value);
            }
          }
        }
        Err(e) => {
          eprintln!(
            "Warning: Failed to parse project prompts from Moon config: {}",
            e
          );
        }
      }
    }

    // Load language-specific prompts
    let languages =
      ["typescript", "javascript", "rust", "python", "go", "java"];
    for lang in &languages {
      let config_key = format!("extension.moonshine.prompts.{}", lang);
      if let Ok(Some(lang_prompt)) = get_moon_config_safe(&config_key) {
        if !lang_prompt.trim().is_empty() && lang_prompt.len() <= 8000 {
          prompts.insert(format!("{}_specific", lang), lang_prompt);
        }
      }
    }

    if prompts.is_empty() {
      return Ok(None);
    }

    Ok(Some(prompts))
  }

  /// Save optimized prompts back to workspace (WASM-compatible via Moon tasks)
  pub fn save_optimized_prompts(
    prompts: &HashMap<String, String>,
  ) -> Result<()> {
    // Validate prompts before saving
    if prompts.is_empty() {
      return Err(Error::config(
        "Cannot save empty prompts collection".to_string(),
      ));
    }

    // Validate individual prompts
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
      // Validate prompt contains reasonable text (no binary data)
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

    // Serialize prompts to JSON for Moon task communication
    let json = serde_json::to_string_pretty(prompts).map_err(|e| {
      Error::config(format!("Failed to serialize prompts: {}", e))
    })?;

    // Create metadata for Moon task communication
    let save_metadata = serde_json::json!({
      "action": "save_optimized_prompts",
      "timestamp": chrono::Utc::now().to_rfc3339(),
      "prompt_count": prompts.len(),
      "total_size_bytes": json.len(),
      "data": prompts
    });

    let _metadata_json =
      serde_json::to_string(&save_metadata).map_err(|e| {
        Error::config(format!("Failed to serialize save metadata: {}", e))
      })?;

    // WASM-compatible: Communicate with Moon task through PDK
    // Moon task will handle actual file writing to .moon/moonshine-prompts.json
    // WASM-compatible: Use file writing via Moon PDK instead of config setting
    use crate::moon_pdk_interface::update_prompts_json;

    // Convert strings to serde_json::Value for Moon PDK functions
    let json_value: serde_json::Value =
      serde_json::from_str(&json).map_err(|e| {
        Error::config(format!("Failed to parse JSON for Moon PDK: {}", e))
      })?;

    if let Err(e) = update_prompts_json(&json_value) {
      eprintln!(
        "Warning: Failed to save optimized prompts via Moon PDK: {}",
        e
      );

      // Fallback: Try to use Moon request storage
      if let Err(e2) = crate::moon_pdk_interface::request_storage_update(
        "prompts",
        &save_metadata,
      ) {
        return Err(Error::config(format!(
          "Failed to save prompts: Primary save failed ({}), Fallback failed ({})",
          e, e2
        )));
      }
    }

    // Set success indicator for Moon task to process
    let success_indicator = serde_json::json!({
      "saved_at": chrono::Utc::now().to_rfc3339(),
      "prompt_keys": prompts.keys().collect::<Vec<_>>(),
      "status": "ready_for_persistence"
    });

    // Use request storage to signal completion status
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

    // Test invalid copro_breadth
    config.copro_breadth = Some(0);
    assert!(config.validate().is_err());
    config.copro_breadth = Some(25);
    assert!(config.validate().is_err());
    config.copro_breadth = Some(5); // Valid
    assert!(config.validate().is_ok());

    // Test invalid temperature
    config.copro_temperature = Some(-1.0);
    assert!(config.validate().is_err());
    config.copro_temperature = Some(3.0);
    assert!(config.validate().is_err());
    config.copro_temperature = Some(1.0); // Valid
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
    // Version should be from Cargo.toml
    assert!(version.chars().next().unwrap().is_ascii_digit());
  }

  #[test]
  fn test_moonshine_directory() {
    // In WASM environment, always returns the default directory
    // Environment variables are not accessible in WASM
    assert_eq!(MoonShineConfig::moonshine_directory(), ".moon/moonshine");
  }
}
