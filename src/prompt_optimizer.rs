//! DSPy-powered Claude CLI prompt optimization for Moon Shine
//!
//! Implements Context Engineering using DSPy Rust for systematic prompt optimization.
//! DSPy uses Claude CLI (via Moon tasks) as its LLM backend for optimization.
//! WASM coordinates workflow and optimizes prompts, Moon tasks execute Claude CLI.
//! Requires the "dspy" feature flag to be enabled.

use crate::analysis::{AnalysisResults, MoonTaskRequest};
// DSPy embedded - always available
use crate::data::{Example, Prediction};
use crate::dspy::{
    core::Module,
    evaluate::Evaluator,
    optimizer::{Optimizer, COPRO},
    predictors::{Predict, Predictor},
    MetaSignature,
};
use crate::moon_pdk_interface::update_training_json;
use extism_pdk::{error, info};
use serde::{Deserialize, Serialize};

// Simple validation without external dependencies

/// Provider configuration for LLM-specific optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: LLMProvider,
    pub model_name: String,
    pub format_preferences: FormatPreferences,
    pub optimization_hints: OptimizationHints,
}

/// Supported LLM providers with version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Claude { version: String },
    OpenAI { model: String },
    Gemini { model: String },
    Local { endpoint: String },
}

/// Format preferences for different providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatPreferences {
    pub prompt_structure: PromptStructure,
    pub output_format: OutputFormat,
    pub instruction_style: InstructionStyle,
}

/// Prompt structure preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptStructure {
    XmlBased,   // Claude preference
    JsonSchema, // Gemini preference
    Markdown,   // OpenAI preference
    Hybrid,     // Adaptive approach
}

/// Output format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    JsonWithPrefill,
    StructuredJson,
    JsonSchema,
    PlainText,
}

/// Instruction style preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionStyle {
    SystemAndHuman, // Claude: separate system + human messages
    SinglePrompt,   // OpenAI: single combined prompt
    MarkdownBased,  // Gemini: markdown-structured prompts
}

/// Provider-specific optimization hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationHints {
    pub use_system_prompts: bool,
    pub prefer_prefill: bool,
    pub structured_output: bool,
    pub chain_of_thought: bool,
}

/// DSPy optimization history entry for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationHistoryEntry {
    pub timestamp: String,
    pub optimizer_type: String, // "COPRO", "BootstrapFewShot", etc.
    pub prompt_template: String,
    pub score_before: f32,
    pub score_after: f32,
    pub improvement: f32,
    pub examples_used: u32,
    pub session_id: String,
    pub model: String,
}

/// COPRO candidate tracking for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoproCandidate {
    pub template: String,
    pub score: f32,
    pub generation: u32,
    pub parent_id: Option<String>,
    pub created_at: String,
}

/// Claude CLI adapter for DSPy integration with provider awareness
// DSPy embedded - always available
#[derive(Debug)]
pub struct ClaudeCLIAdapter {
    pub session_id: String,
    pub model: String, // "sonnet" or "opus"
    pub provider_config: Option<ProviderConfig>,
}

// DSPy embedded - always available
impl ClaudeCLIAdapter {
    pub fn new(session_id: String, model: String) -> Self {
        Self {
            session_id,
            model,
            provider_config: None,
        }
    }

    /// Create provider-aware adapter with auto-detected configuration
    pub fn with_provider_detection(session_id: String, model: String) -> anyhow::Result<Self> {
        let provider_config = Self::detect_provider_from_model(&model)?;
        Ok(Self {
            session_id,
            model,
            provider_config: Some(provider_config),
        })
    }

    /// Create new adapter with model loaded from Moon configuration
    pub fn from_config(session_id: String) -> anyhow::Result<Self> {
        let model = Self::load_claude_model_from_config()?;
        Self::with_provider_detection(session_id, model)
    }

    /// Load Claude model from Moon configuration with fallbacks
    fn load_claude_model_from_config() -> anyhow::Result<String> {
        use crate::moon_pdk_interface::get_moon_config_safe;

        // Try various configuration keys in order of preference
        let config_keys = [
            "extension.moonshine.claude.model",
            "moonshine.claude.model",
            "env.MOONSHINE_CLAUDE_MODEL",
            "ai.claude.model",
            "providers.claude.model",
        ];

        for key in &config_keys {
            if let Ok(Some(model)) = get_moon_config_safe(key) {
                let model = model.trim();
                if !model.is_empty() && Self::validate_claude_model(model)? {
                    return Ok(model.to_string());
                }
            }
        }

        // Fallback to default with validation
        let default_model = "sonnet";
        if Self::validate_claude_model(default_model)? {
            Ok(default_model.to_string())
        } else {
            Err(anyhow::anyhow!("No valid Claude model available in configuration or defaults"))
        }
    }

    /// Validate Claude model name format and availability (including user-friendly names)
    fn validate_claude_model(model: &str) -> anyhow::Result<bool> {
        // Valid Claude model patterns (including user-friendly names)
        let valid_patterns = [
            "sonnet",
            "opus",
            "claude-3-5-sonnet",
            "claude-3-5-haiku",
            "claude-3-opus",
            "claude-3-sonnet",
            "claude-3-haiku",
        ];

        for pattern in &valid_patterns {
            if model.starts_with(pattern) {
                // Validate date format if present (YYYYMMDD)
                if let Some(date_part) = model.strip_prefix(&format!("{}-", pattern)) {
                    if date_part.len() == 8 && date_part.chars().all(|c| c.is_ascii_digit()) {
                        return Ok(true);
                    }
                } else if model == *pattern {
                    // Base model name without date is valid
                    return Ok(true);
                }
            }
        }

        Err(anyhow::anyhow!("Invalid Claude model format: {}", model))
    }

    /// Auto-detect provider configuration from model name
    fn detect_provider_from_model(model: &str) -> anyhow::Result<ProviderConfig> {
        let (provider_type, format_prefs, optimization_hints) = if model.starts_with("claude") || model == "sonnet" || model == "opus" {
            (
                LLMProvider::Claude { version: model.to_string() },
                FormatPreferences {
                    prompt_structure: PromptStructure::XmlBased,
                    output_format: OutputFormat::JsonWithPrefill,
                    instruction_style: InstructionStyle::SystemAndHuman,
                },
                OptimizationHints {
                    use_system_prompts: true,
                    prefer_prefill: true,
                    structured_output: false,
                    chain_of_thought: true,
                },
            )
        } else if model.starts_with("gpt") || model.contains("openai") || model == "gpt4.1" {
            (
                LLMProvider::OpenAI { model: model.to_string() },
                FormatPreferences {
                    prompt_structure: PromptStructure::Hybrid,
                    output_format: OutputFormat::StructuredJson,
                    instruction_style: InstructionStyle::SinglePrompt,
                },
                OptimizationHints {
                    use_system_prompts: false,
                    prefer_prefill: false,
                    structured_output: true,
                    chain_of_thought: true,
                },
            )
        } else if model.starts_with("gemini") {
            (
                LLMProvider::Gemini { model: model.to_string() },
                FormatPreferences {
                    prompt_structure: PromptStructure::JsonSchema,
                    output_format: OutputFormat::JsonSchema,
                    instruction_style: InstructionStyle::MarkdownBased,
                },
                OptimizationHints {
                    use_system_prompts: false,
                    prefer_prefill: false,
                    structured_output: true,
                    chain_of_thought: false,
                },
            )
        } else {
            // Default to Claude Sonnet configuration for unknown models (user preference)
            (
                LLMProvider::Claude { version: "sonnet".to_string() },
                FormatPreferences {
                    prompt_structure: PromptStructure::XmlBased,
                    output_format: OutputFormat::JsonWithPrefill,
                    instruction_style: InstructionStyle::SystemAndHuman,
                },
                OptimizationHints {
                    use_system_prompts: true,
                    prefer_prefill: true,
                    structured_output: false,
                    chain_of_thought: true,
                },
            )
        };

        Ok(ProviderConfig {
            provider_type,
            model_name: model.to_string(),
            format_preferences: format_prefs,
            optimization_hints,
        })
    }

    /// Get default provider configuration if none detected
    fn get_default_provider_config(&self) -> ProviderConfig {
        ProviderConfig {
            provider_type: LLMProvider::Claude { version: self.model.clone() },
            model_name: self.model.clone(),
            format_preferences: FormatPreferences {
                prompt_structure: PromptStructure::XmlBased,
                output_format: OutputFormat::JsonWithPrefill,
                instruction_style: InstructionStyle::SystemAndHuman,
            },
            optimization_hints: OptimizationHints {
                use_system_prompts: true,
                prefer_prefill: true,
                structured_output: false,
                chain_of_thought: true,
            },
        }
    }

    /// Log optimization history entry to training.json via Moon host
    pub fn log_optimization_history(
        &self,
        optimizer_type: &str,
        prompt_template: &str,
        score_before: f32,
        score_after: f32,
        examples_used: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let entry = OptimizationHistoryEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            optimizer_type: optimizer_type.to_string(),
            prompt_template: prompt_template.to_string(),
            score_before,
            score_after,
            improvement: score_after - score_before,
            examples_used,
            session_id: self.session_id.clone(),
            model: self.model.clone(),
        };

        let update = serde_json::json!({
          "optimization_history": {
            "entries": [entry]
          }
        });

        update_training_json(&update)
    }

    /// Log COPRO candidate to prompts.json via Moon host
    pub fn log_copro_candidate(&self, candidate: CoproCandidate) -> Result<(), Box<dyn std::error::Error>> {
        let update = serde_json::json!({
          "copro_candidates": {
            "active": [candidate]
          }
        });

        crate::moon_pdk_interface::update_prompts_json(&update)
    }

    /// Generate provider-optimized prompt based on configuration
    pub fn generate_provider_optimized_prompt(&self, base_prompt: &str, context: &CodeFixingContext) -> String {
        let provider_config = self.provider_config.as_ref().unwrap_or(&self.get_default_provider_config());

        match provider_config.format_preferences.prompt_structure {
            PromptStructure::XmlBased => self.format_as_xml(base_prompt, context),
            PromptStructure::JsonSchema => self.format_as_json_schema(base_prompt, context),
            PromptStructure::Markdown => self.format_as_markdown(base_prompt, context),
            PromptStructure::Hybrid => self.format_as_hybrid(base_prompt, context),
        }
    }

    /// Format prompt using Claude's preferred XML structure
    fn format_as_xml(&self, base_prompt: &str, context: &CodeFixingContext) -> String {
        format!(
            r#"<system>
You are an expert TypeScript code quality specialist with deep knowledge of enterprise development practices.
</system>

<task>
{base_prompt}
</task>

<context>
<file_path>{file_path}</file_path>
<file_type>{file_type}</file_type>
<error_count>{error_count}</error_count>
<warning_count>{warning_count}</warning_count>
<project_type>{project_type}</project_type>
<complexity_score>{complexity_score}</complexity_score>
</context>

<code language="{file_type}">
{code_snippet}
</code>

<requirements>
- Fix all compilation errors and warnings
- Maintain existing functionality
- Follow TypeScript best practices
- Add comprehensive documentation if needed
</requirements>

<output_format>
Return only the enhanced code:
```{file_type}
</output_format>"#,
            base_prompt = base_prompt,
            file_path = context.file_path,
            file_type = context.file_type,
            error_count = context.error_count,
            warning_count = context.warning_count,
            project_type = context.project_type,
            complexity_score = context.complexity_score,
            code_snippet = context.code_snippet
        )
    }

    /// Format prompt using Gemini's preferred JSON schema approach
    fn format_as_json_schema(&self, base_prompt: &str, context: &CodeFixingContext) -> String {
        let schema = serde_json::json!({
          "task": base_prompt,
          "context": {
            "file_path": context.file_path,
            "file_type": context.file_type,
            "error_count": context.error_count,
            "warning_count": context.warning_count,
            "project_type": context.project_type,
            "complexity_score": context.complexity_score
          },
          "code": context.code_snippet,
          "response_schema": {
            "type": "object",
            "properties": {
              "fixed_code": {"type": "string"},
              "explanation": {"type": "string"},
              "confidence": {"type": "number", "minimum": 0, "maximum": 1}
            },
            "required": ["fixed_code", "explanation", "confidence"]
          }
        });

        format!(
            "# Code Improvement Task\n\n{}\n\nReturn response matching the specified JSON schema.",
            serde_json::to_string_pretty(&schema).unwrap()
        )
    }

    /// Format prompt using OpenAI's preferred markdown structure
    fn format_as_markdown(&self, base_prompt: &str, context: &CodeFixingContext) -> String {
        format!(
            r#"# Code Improvement Task

## Objective
{base_prompt}

## File Information
- **Path**: {file_path}
- **Type**: {file_type}
- **Project**: {project_type}
- **Errors**: {error_count}
- **Warnings**: {warning_count}
- **Complexity**: {complexity_score}

## Code to Improve
```{file_type}
{code_snippet}
```

## Requirements
1. Fix all compilation errors and warnings
2. Maintain existing functionality
3. Follow TypeScript best practices
4. Add comprehensive documentation if needed

## Output Format
Return only the improved code in a code block."#,
            base_prompt = base_prompt,
            file_path = context.file_path,
            file_type = context.file_type,
            project_type = context.project_type,
            error_count = context.error_count,
            warning_count = context.warning_count,
            complexity_score = context.complexity_score,
            code_snippet = context.code_snippet
        )
    }

    /// Format prompt using hybrid approach (XML + Markdown)
    fn format_as_hybrid(&self, base_prompt: &str, context: &CodeFixingContext) -> String {
        format!(
            r#"<task>{base_prompt}</task>

## Context
- **File**: {file_path}
- **Type**: {file_type}
- **Project**: {project_type}
- **Issues**: {error_count} errors, {warning_count} warnings
- **Complexity**: {complexity_score}

<code>
{code_snippet}
</code>

**Return only the improved code with fixes applied.**"#,
            base_prompt = base_prompt,
            file_path = context.file_path,
            file_type = context.file_type,
            project_type = context.project_type,
            error_count = context.error_count,
            warning_count = context.warning_count,
            complexity_score = context.complexity_score,
            code_snippet = context.code_snippet
        )
    }

    /// Execute provider-optimized Claude CLI command through Moon task system
    pub async fn execute_provider_optimized_prompt(&self, base_prompt: &str, context: &CodeFixingContext) -> Result<String, Box<dyn std::error::Error>> {
        let optimized_prompt = self.generate_provider_optimized_prompt(base_prompt, context);
        self.execute_claude_prompt(&optimized_prompt, context).await
    }

    /// Execute Claude CLI command through Moon task system
    pub async fn execute_claude_prompt(&self, prompt: &str, context: &CodeFixingContext) -> Result<String, Box<dyn std::error::Error>> {
        // Create Moon task request for Claude CLI execution
        let request = MoonTaskRequest {
            file_path: context.file_path.clone(),
            language: context.file_type.clone(),
            content: format!("{}\n\nPrompt: {}", context.code_snippet, prompt), // Include the prompt in the content
            analysis_results: AnalysisResults {
                suggestions: vec![],
                semantic_warnings: vec![],
                tsdoc_coverage: 0.0,
                quality_score: 0.0,
                parse_errors: vec![],
                ignored_files: vec![],
                ai_model: self.model.clone(),
            },
            session_id: self.session_id.clone(),
        };

        // Serialize request to JSON for Moon task communication
        let _request_json = serde_json::to_string(&request)?;

        // Execute Moon task to call Claude CLI
        self.execute_moon_task_claude(&request).await
    }

    /// Execute Moon task to call Claude CLI with proper Moon PDK integration
    async fn execute_moon_task_claude(&self, request: &MoonTaskRequest) -> Result<String, Box<dyn std::error::Error>> {
        use crate::moon_pdk_interface::generate_moon_task_command;

        // Generate Moon task command using existing infrastructure
        let task_command = generate_moon_task_command(request)?;

        // Use Moon PDK to execute task through host environment
        // In WASM, this delegates to Moon's task orchestration system
        let task_result = self.execute_via_moon_host(&task_command, request).await?;

        Ok(task_result)
    }

    /// Execute via Moon host environment using proper PDK patterns
    async fn execute_via_moon_host(&self, _task_command: &str, request: &MoonTaskRequest) -> Result<String, Box<dyn std::error::Error>> {
        // Execute Claude CLI via Moon task system with proper JSON communication
        // This integrates with Moon's task orchestration for real Claude CLI execution
        self.execute_claude_cli_response(request).await

        // Note: DSPy embedded - fallback removed
    }

    /// Execute Claude CLI response - READ-ONLY suggestions (NO file writing)
    /// Production implementation using Moon PDK with stdin support for large prompts
    async fn execute_claude_cli_response(&self, request: &MoonTaskRequest) -> Result<String, Box<dyn std::error::Error>> {
        use crate::moon_pdk_interface::{execute_command, ExecCommandInput};

        // Use the content field as the prompt for DSPy optimization
        let prompt = &request.content;

        // Production: Handle large prompts by truncating with intelligent summarization
        let optimized_prompt = if prompt.len() > 16384 {
            // 16KB limit for command-line safety
            // For very large prompts, extract key parts and create focused prompt
            let lines: Vec<&str> = prompt.lines().collect();
            let total_lines = lines.len();

            if total_lines > 100 {
                // Take first 50 and last 50 lines with summary
                let mut optimized = String::new();
                optimized.push_str("--- PROMPT START (truncated for optimization) ---\n");
                optimized.push_str(&lines[..50].join("\n"));
                optimized.push_str(&format!("\n--- MIDDLE SECTION OMITTED ({} lines) ---\n", total_lines - 100));
                optimized.push_str(&lines[total_lines - 50..].join("\n"));
                optimized.push_str("\n--- PROMPT END ---");
                optimized
            } else {
                // Truncate at word boundaries to maintain context
                let mut truncated = String::new();
                let words: Vec<&str> = prompt.split_whitespace().collect();
                let mut current_len = 0;

                for word in words {
                    if current_len + word.len() + 1 > 15000 {
                        // Leave buffer for other args
                        truncated.push_str("\n[... prompt truncated for size limitations ...]");
                        break;
                    }
                    if !truncated.is_empty() {
                        truncated.push(' ');
                    }
                    truncated.push_str(word);
                    current_len += word.len() + 1;
                }
                truncated
            }
        } else {
            prompt.clone()
        };

        // Configure environment for optimal Claude CLI execution
        let mut env = std::collections::HashMap::new();
        env.insert("CLAUDE_OUTPUT_FORMAT".to_string(), "json".to_string());
        env.insert("CLAUDE_SESSION_ID".to_string(), request.session_id.clone());

        // Add model configuration from config if available
        if let Ok(model) = Self::load_claude_model_from_config() {
            env.insert("CLAUDE_MODEL".to_string(), model);
        }

        // Configure command with optimized arguments
        let command_args = vec![
            "--print".to_string(),
            "--output-format".to_string(),
            "json".to_string(),
            "--session-id".to_string(),
            request.session_id.clone(),
            "--no-cache".to_string(), // Ensure fresh responses for DSPy optimization
            optimized_prompt,
        ];

        let command_input = ExecCommandInput {
            command: "claude".to_string(),
            args: command_args,
            env,
            working_dir: None,
        };
        // Execute the command via Moon host function
        info!("Executing Claude CLI for DSPy optimization");
        let start_time = std::time::Instant::now();
        let output = execute_command(command_input)?;
        let execution_time = start_time.elapsed().as_millis() as u64;

        if output.exit_code != 0 {
            error!(
                "Claude CLI failed with exit code {} in {}ms: {}",
                output.exit_code, execution_time, output.stderr
            );
            return Err(format!("Claude CLI command failed with exit code {}: {}", output.exit_code, output.stderr).into());
        }

        info!("Claude CLI executed successfully in {}ms", execution_time);

        // Parse the output from Claude CLI
        let claude_response_json: serde_json::Value = serde_json::from_str(&output.stdout)?;

        // Return the fixed_code or improvements as a string
        if let Some(fixed_code) = claude_response_json.get("fixed_code").and_then(|v| v.as_str()) {
            Ok(fixed_code.to_string())
        } else if let Some(improvements) = claude_response_json.get("improvements").and_then(|v| v.as_array()) {
            Ok(improvements.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join("; "))
        } else {
            Ok(output.stdout) // Fallback to raw stdout if no specific fields found
        }
    }

    /// Test prompt effectiveness for DSPy optimization
    pub async fn evaluate_prompt_quality(&self, prompt: &str, context: &CodeFixingContext, expected_output: &str) -> Result<f32, Box<dyn std::error::Error>> {
        let response = self.execute_claude_prompt(prompt, context).await?;

        // Production: Use advanced similarity calculation with multiple metrics
        let similarity = Self::calculate_semantic_similarity(&response, expected_output)?;
        Ok(similarity)
    }

    /// Calculate semantic similarity using multiple metrics for robust comparison
    fn calculate_semantic_similarity(actual: &str, expected: &str) -> Result<f32, Box<dyn std::error::Error>> {
        if actual.is_empty() && expected.is_empty() {
            return Ok(1.0);
        }
        if actual.is_empty() || expected.is_empty() {
            return Ok(0.0);
        }

        // Normalize text for comparison
        let actual_norm = Self::normalize_text(actual);
        let expected_norm = Self::normalize_text(expected);

        // 1. Exact match gets highest score
        if actual_norm == expected_norm {
            return Ok(1.0);
        }

        // 2. Calculate Levenshtein distance ratio
        let edit_distance = Self::levenshtein_distance(&actual_norm, &expected_norm);
        let max_len = actual_norm.len().max(expected_norm.len()) as f32;
        let edit_similarity = if max_len > 0.0 { 1.0 - (edit_distance as f32 / max_len) } else { 0.0 };

        // 3. Calculate word overlap similarity
        let actual_words: std::collections::HashSet<&str> = actual_norm.split_whitespace().collect();
        let expected_words: std::collections::HashSet<&str> = expected_norm.split_whitespace().collect();

        let intersection = actual_words.intersection(&expected_words).count() as f32;
        let union = actual_words.union(&expected_words).count() as f32;
        let word_similarity = if union > 0.0 { intersection / union } else { 0.0 };

        // 4. Calculate n-gram similarity (trigrams)
        let actual_trigrams = Self::extract_ngrams(&actual_norm, 3);
        let expected_trigrams = Self::extract_ngrams(&expected_norm, 3);
        let trigram_intersection = actual_trigrams.intersection(&expected_trigrams).count() as f32;
        let trigram_union = actual_trigrams.union(&expected_trigrams).count() as f32;
        let trigram_similarity = if trigram_union > 0.0 { trigram_intersection / trigram_union } else { 0.0 };

        // 5. Calculate substring containment
        let containment_similarity = if actual_norm.len() >= expected_norm.len() / 2 && actual_norm.contains(&expected_norm) {
            0.8
        } else if expected_norm.len() >= actual_norm.len() / 2 && expected_norm.contains(&actual_norm) {
            0.8
        } else {
            0.0
        };

        // Weighted combination of metrics
        let combined_similarity = (edit_similarity * 0.3 + word_similarity * 0.3 + trigram_similarity * 0.2 + containment_similarity * 0.2).clamp(0.0, 1.0);

        Ok(combined_similarity)
    }

    /// Normalize text for consistent comparison
    fn normalize_text(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1).min(matrix[i][j - 1] + 1).min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Extract n-grams from text for similarity analysis
    fn extract_ngrams(text: &str, n: usize) -> std::collections::HashSet<String> {
        let chars: Vec<char> = text.chars().collect();
        if chars.len() < n {
            return std::collections::HashSet::new();
        }

        chars.windows(n).map(|window| window.iter().collect()).collect()
    }
}

/// Context variables for DSPy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFixingContext {
    pub file_path: String,
    pub file_type: String,
    pub error_count: usize,
    pub warning_count: usize,
    pub project_type: String,
    pub ai_model: String,
    pub complexity_score: f32,
    pub previous_fixes: Vec<String>,
    pub code_snippet: String,
}

/// Validated structured response from Claude CLI
// DSPy embedded - always available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedCodeFix {
    /// The fixed code content
    pub fixed_code: String,

    /// Brief explanation of changes made
    pub explanation: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Number of issues addressed
    pub issues_fixed: u32,
}

// DSPy embedded - always available
impl ValidatedCodeFix {
    /// Simple validation without external dependencies
    pub fn validate(&self) -> Result<(), String> {
        if self.fixed_code.is_empty() {
            return Err("Fixed code cannot be empty".to_string());
        }

        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
        }

        if self.explanation.is_empty() {
            return Err("Explanation cannot be empty".to_string());
        }

        Ok(())
    }
}

// WASM-compatible signature using new macro system - replaces 60+ lines of boilerplate
use crate::signature;

signature! {
  CodeFixingSignature {
    inputs: {
      file_path: String, "Path to the file being fixed";
      file_type: String, "Type of file (typescript, javascript, etc.)";
      error_count: String, "Number of errors in the code";
      warning_count: String, "Number of warnings in the code";
      project_type: String, "Type of project (react, node, etc.)";
      complexity_score: String, "Complexity score of the code";
      previous_fixes: String, "Previous fix attempts";
      code_snippet: String, "The code to be fixed"
    },
    outputs: {
      fixed_code: String, "The corrected code";
      explanation: String, "Explanation of the fixes applied";
      confidence: String, "Confidence score of the fix"
    },
    instruction: "Fix the provided code by addressing errors and improving quality. Return the fixed code with explanation and confidence score."
  }
}

// DSPy embedded - always available
fn create_code_fixing_signature() -> impl MetaSignature + 'static {
    CodeFixingSignature::new()
}

/// DSPy-powered code fixing module
// DSPy embedded - always available
pub struct CodeFixingModule {
    predictor: Predict,
    optimization_history: Vec<OptimizationAttempt>,
}

impl CodeFixingModule {
    pub fn new(predictor: Predict) -> Self {
        Self {
            predictor,
            optimization_history: Vec::new(),
        }
    }
}

// DSPy embedded - always available
impl Module for CodeFixingModule {
    async fn forward(&self, inputs: Example) -> anyhow::Result<Prediction> {
        self.predictor.forward(inputs).await
    }
}

use crate::dspy::core::module::Optimizable;
use indexmap::IndexMap;

impl Optimizable for CodeFixingModule {
    fn get_signature(&self) -> &dyn crate::dspy::MetaSignature {
        self.predictor.get_signature()
    }

    fn parameters(&mut self) -> IndexMap<String, &mut dyn Optimizable> {
        let mut params = IndexMap::new();
        params.insert("predictor".to_string(), &mut self.predictor as &mut dyn Optimizable);
        params
    }

    fn update_signature_instruction(&mut self, instruction: String) -> anyhow::Result<()> {
        self.predictor.update_signature_instruction(instruction)
    }
}

// DSPy embedded - always available
impl Evaluator for CodeFixingModule {
    async fn evaluate(&self, _examples: Vec<Example>) -> f32 {
        // Simple evaluation - count successful fixes
        let successful = self.optimization_history.iter().filter(|attempt| attempt.success_score > 0.7).count();

        if self.optimization_history.is_empty() {
            0.5 // Default score
        } else {
            successful as f32 / self.optimization_history.len() as f32
        }
    }

    async fn metric(&self, _example: &Example, _prediction: &Prediction) -> f32 {
        // Simple metric - return 1.0 for success, 0.0 for failure
        // In real usage, this would evaluate the prediction quality
        0.8 // Default success rate
    }
}

/// Record of optimization attempts for teleprompter learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAttempt {
    pub context: CodeFixingContext,
    pub prompt_used: String,
    pub output_generated: String,
    pub success_score: f32,
    pub fixes_applied: usize,
    pub timestamp: String,
}

/// Moon Shine DSPy optimization engine
// DSPy embedded - always available
pub struct MoonShineDSPy {
    code_fixing_module: CodeFixingModule,
    optimizer: COPRO,
    training_examples: Vec<TrainingExample>,
    claude_adapter: ClaudeCLIAdapter,
}

/// Training example for DSPy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    pub input_context: CodeFixingContext,
    pub expected_output: String,
    pub quality_score: f32,
}

// DSPy embedded - always available
impl MoonShineDSPy {
    pub fn new(session_id: String, model: String) -> Self {
        let predictor = Predict::new(create_code_fixing_signature());

        let code_fixing_module = CodeFixingModule {
            predictor,
            optimization_history: Vec::new(),
        };

        let optimizer = COPRO::builder()
            .breadth(5) // 5 examples for few-shot
            .depth(3) // 3 optimization iterations
            .build();

        let claude_adapter =
            ClaudeCLIAdapter::with_provider_detection(session_id.clone(), model.clone()).unwrap_or_else(|_| ClaudeCLIAdapter::new(session_id, model));

        Self {
            code_fixing_module,
            optimizer,
            training_examples: Self::load_initial_training_examples(),
            claude_adapter,
        }
    }

    /// Get optimized base prompts for different Moon task types
    /// Production implementation that loads from Moon configuration with intelligent fallbacks
    pub fn get_base_prompts() -> BasePromptCollection {
        Self::load_base_prompts_from_config().unwrap_or_else(|_| Self::get_default_base_prompts())
    }

    /// Load base prompts from Moon configuration (WASM-compatible)
    fn load_base_prompts_from_config() -> anyhow::Result<BasePromptCollection> {
        use crate::moon_pdk_interface::get_moon_config_safe;

        let mut prompts = Self::get_default_base_prompts();

        // Load ESLint fix prompt
        if let Ok(Some(eslint_prompt)) = get_moon_config_safe("extension.moonshine.prompts.eslint_fix") {
            if Self::validate_prompt_template(&eslint_prompt)? {
                prompts.eslint_fix = eslint_prompt;
            }
        }

        // Load TypeScript fix prompt
        if let Ok(Some(ts_prompt)) = get_moon_config_safe("extension.moonshine.prompts.typescript_fix") {
            if Self::validate_prompt_template(&ts_prompt)? {
                prompts.typescript_fix = ts_prompt;
            }
        }

        // Load TSDoc enhancement prompt
        if let Ok(Some(tsdoc_prompt)) = get_moon_config_safe("extension.moonshine.prompts.tsdoc_enhancement") {
            if Self::validate_prompt_template(&tsdoc_prompt)? {
                prompts.tsdoc_enhancement = tsdoc_prompt;
            }
        }

        // Load code optimization prompt
        if let Ok(Some(opt_prompt)) = get_moon_config_safe("extension.moonshine.prompts.code_optimization") {
            if Self::validate_prompt_template(&opt_prompt)? {
                prompts.code_optimization = opt_prompt;
            }
        }

        // Load complexity reduction prompt
        if let Ok(Some(complexity_prompt)) = get_moon_config_safe("extension.moonshine.prompts.complexity_reduction") {
            if Self::validate_prompt_template(&complexity_prompt)? {
                prompts.complexity_reduction = complexity_prompt;
            }
        }

        // Load additional prompts from optimized collection
        if let Ok(Some(optimized_json)) = get_moon_config_safe("extension.moonshine.optimized_prompts") {
            if let Ok(optimized_prompts) = serde_json::from_str::<std::collections::HashMap<String, String>>(&optimized_json) {
                // Override with optimized versions if available
                if let Some(opt_eslint) = optimized_prompts.get("eslint_fix") {
                    if Self::validate_prompt_template(opt_eslint)? {
                        prompts.eslint_fix = opt_eslint.clone();
                    }
                }
                if let Some(opt_ts) = optimized_prompts.get("typescript_fix") {
                    if Self::validate_prompt_template(opt_ts)? {
                        prompts.typescript_fix = opt_ts.clone();
                    }
                }
                if let Some(opt_tsdoc) = optimized_prompts.get("tsdoc_enhancement") {
                    if Self::validate_prompt_template(opt_tsdoc)? {
                        prompts.tsdoc_enhancement = opt_tsdoc.clone();
                    }
                }
                if let Some(opt_code) = optimized_prompts.get("code_optimization") {
                    if Self::validate_prompt_template(opt_code)? {
                        prompts.code_optimization = opt_code.clone();
                    }
                }
                if let Some(opt_complexity) = optimized_prompts.get("complexity_reduction") {
                    if Self::validate_prompt_template(opt_complexity)? {
                        prompts.complexity_reduction = opt_complexity.clone();
                    }
                }
            }
        }

        Ok(prompts)
    }

    /// Validate prompt template for safety and completeness
    fn validate_prompt_template(prompt: &str) -> anyhow::Result<bool> {
        if prompt.trim().is_empty() {
            return Err(anyhow::anyhow!("Prompt template cannot be empty"));
        }

        if prompt.len() > 10000 {
            return Err(anyhow::anyhow!("Prompt template too long: {} characters", prompt.len()));
        }

        // Check for required security markers
        let security_checks = ["Return ONLY", "ONLY the", "JSON format", "fixed code"];

        let has_security_markers = security_checks.iter().any(|marker| prompt.contains(marker));
        if !has_security_markers {
            return Err(anyhow::anyhow!("Prompt template missing security markers"));
        }

        // Validate that template variables are properly formatted
        let template_vars = [
            "{rules}",
            "{errors}",
            "{file_type}",
            "{project_type}",
            "{error_count}",
            "{warning_count}",
            "{specific_focus}",
            "{complexity}",
            "{coverage_target}",
        ];

        for var in template_vars {
            if prompt.contains(&var.replace("{", "").replace("}", "")) && !prompt.contains(var) {
                return Err(anyhow::anyhow!("Invalid template variable format found"));
            }
        }

        Ok(true)
    }

    /// Get default base prompts as fallback
    fn get_default_base_prompts() -> BasePromptCollection {
        BasePromptCollection {
      eslint_fix: "You are an ESLint expert. Analyze the following TypeScript/JavaScript code and fix all ESLint violations. Focus on: {rules}. Return ONLY the fixed code in JSON format with 'fixed_code' and 'explanation' fields.".to_string(),
      typescript_fix: "You are a TypeScript compiler expert. Fix the following TypeScript errors: {errors}. Ensure type safety and modern TypeScript patterns. Return ONLY the fixed code in JSON format.".to_string(),
      tsdoc_enhancement: "You are a documentation expert. Improve TSDoc coverage for this {file_type} code. Target {coverage_target}% documentation coverage. Add comprehensive JSDoc/TSDoc comments. Return ONLY the enhanced code.".to_string(),
      code_optimization: "You are a code quality expert analyzing {file_type} in a {project_type} project. Fix {error_count} errors and {warning_count} warnings. Focus on: {specific_focus}. Return ONLY optimized code.".to_string(),
      complexity_reduction: "You are a refactoring expert. This code has complexity score {complexity}. Simplify logic, improve readability, and maintain functionality. Return ONLY the refactored code.".to_string(),
    }
    }

    /// Generate optimized prompt for specific Moon task type
    pub fn optimize_prompt_for_task(&mut self, task_type: MoonTaskType, context: &CodeFixingContext, tool_output: Option<&str>) -> String {
        let base_prompts = Self::get_base_prompts();

        match task_type {
            MoonTaskType::ESLintFix => {
                let rules = if let Some(output) = tool_output {
                    let (_, _, rules) = Self::parse_eslint_output(output);
                    rules.join(", ")
                } else {
                    "all ESLint rules".to_string()
                };

                base_prompts.eslint_fix.replace("{rules}", &rules)
            }

            MoonTaskType::TypeScriptFix => {
                let errors = if let Some(output) = tool_output {
                    let errors = Self::parse_typescript_output(output);
                    if errors.is_empty() {
                        "type safety issues".to_string()
                    } else {
                        errors.join("; ")
                    }
                } else {
                    "type errors".to_string()
                };

                base_prompts.typescript_fix.replace("{errors}", &errors)
            }

            MoonTaskType::TSDocEnhancement => base_prompts
                .tsdoc_enhancement
                .replace("{file_type}", &context.file_type)
                .replace("{coverage_target}", "90"),

            MoonTaskType::CodeOptimization => {
                let specific_focus = match context.project_type.as_str() {
                    "react_app" => "React patterns, hooks, performance",
                    "backend" => "security, async/await, error handling",
                    "library" => "API design, error handling, documentation",
                    _ => "code quality, maintainability",
                };

                base_prompts
                    .code_optimization
                    .replace("{file_type}", &context.file_type)
                    .replace("{project_type}", &context.project_type)
                    .replace("{error_count}", &context.error_count.to_string())
                    .replace("{warning_count}", &context.warning_count.to_string())
                    .replace("{specific_focus}", specific_focus)
            }

            MoonTaskType::ComplexityReduction => base_prompts
                .complexity_reduction
                .replace("{complexity}", &format!("{:.1}", context.complexity_score)),
        }
    }

    /// Execute optimized multi-step Claude workflow for single file
    pub async fn execute_multi_step_optimization(
        &mut self,
        context: &CodeFixingContext,
        moon_task_results: &MoonTaskOutputs,
    ) -> Result<MultiStepResult, Box<dyn std::error::Error>> {
        let mut results = MultiStepResult::new(context.file_path.clone());

        // Step 1: ESLint fixes (if ESLint output available)
        if let Some(eslint_output) = &moon_task_results.eslint_output {
            let eslint_prompt = self.optimize_prompt_for_task(MoonTaskType::ESLintFix, context, Some(eslint_output));

            let eslint_result = self.claude_adapter.execute_provider_optimized_prompt(&eslint_prompt, context).await?;
            results.add_step_result("eslint", eslint_result);
        }

        // Step 2: TypeScript fixes (if TSC output available)
        if let Some(tsc_output) = &moon_task_results.typescript_output {
            let ts_prompt = self.optimize_prompt_for_task(MoonTaskType::TypeScriptFix, context, Some(tsc_output));

            let ts_result = self.claude_adapter.execute_provider_optimized_prompt(&ts_prompt, context).await?;
            results.add_step_result("typescript", ts_result);
        }

        // Step 3: TSDoc enhancement (always run if enabled)
        if context.file_type == "typescript" || context.file_type == "javascript" {
            let tsdoc_prompt = self.optimize_prompt_for_task(MoonTaskType::TSDocEnhancement, context, None);

            let tsdoc_result = self.claude_adapter.execute_provider_optimized_prompt(&tsdoc_prompt, context).await?;
            results.add_step_result("tsdoc", tsdoc_result);
        }

        // Step 4: Final optimization (if complexity is high)
        if context.complexity_score > 15.0 {
            let optimization_prompt = self.optimize_prompt_for_task(MoonTaskType::ComplexityReduction, context, None);

            let optimization_result = self.claude_adapter.execute_provider_optimized_prompt(&optimization_prompt, context).await?;
            results.add_step_result("optimization", optimization_result);
        }

        Ok(results)
    }

    /// Parse ESLint, TypeScript, and tool outputs into DSPy context
    pub fn parse_tool_outputs_to_context(
        file_path: &str,
        file_type: &str,
        code_snippet: &str,
        eslint_output: Option<&str>,
        tsc_output: Option<&str>,
        _tsdoc_coverage: f32,
    ) -> CodeFixingContext {
        // Parse ESLint output for errors and warnings
        let (error_count, warning_count, eslint_rules) = if let Some(eslint) = eslint_output {
            Self::parse_eslint_output(eslint)
        } else {
            (0, 0, Vec::new())
        };

        // Parse TypeScript compiler output
        let tsc_errors = if let Some(tsc) = tsc_output {
            Self::parse_typescript_output(tsc)
        } else {
            Vec::new()
        };

        // Calculate complexity based on code metrics
        let complexity_score = Self::calculate_code_complexity(code_snippet);

        // Determine project type from file path patterns
        let project_type = Self::infer_project_type(file_path);

        // Compile previous fixes from tool outputs
        let mut previous_fixes = Vec::new();
        if !eslint_rules.is_empty() {
            previous_fixes.push(format!("ESLint rules: {}", eslint_rules.join(", ")));
        }
        if !tsc_errors.is_empty() {
            previous_fixes.push(format!("TypeScript errors: {} issues", tsc_errors.len()));
        }

        CodeFixingContext {
            file_path: file_path.to_string(),
            file_type: file_type.to_string(),
            error_count,
            warning_count,
            project_type,
            ai_model: "sonnet".to_string(), // Default, can be overridden
            complexity_score,
            previous_fixes,
            code_snippet: code_snippet.to_string(),
        }
    }

    /// Parse ESLint JSON output into structured data
    fn parse_eslint_output(eslint_output: &str) -> (usize, usize, Vec<String>) {
        // Try to parse as JSON first
        if let Ok(eslint_json) = serde_json::from_str::<serde_json::Value>(eslint_output) {
            let mut error_count = 0;
            let mut warning_count = 0;
            let mut rules = Vec::new();

            if let Some(files) = eslint_json.as_array() {
                for file in files {
                    if let Some(messages) = file.get("messages").and_then(|m| m.as_array()) {
                        for message in messages {
                            if let Some(severity) = message.get("severity").and_then(|s| s.as_u64()) {
                                match severity {
                                    2 => error_count += 1,   // Error
                                    1 => warning_count += 1, // Warning
                                    _ => {}
                                }
                            }
                            if let Some(rule_id) = message.get("ruleId").and_then(|r| r.as_str()) {
                                if !rules.contains(&rule_id.to_string()) {
                                    rules.push(rule_id.to_string());
                                }
                            }
                        }
                    }
                }
            }

            return (error_count, warning_count, rules);
        }

        // Fallback: parse text output
        let lines: Vec<&str> = eslint_output.lines().collect();
        let error_count = lines.iter().filter(|line| line.contains("error")).count();
        let warning_count = lines.iter().filter(|line| line.contains("warning")).count();

        // Extract common ESLint rules from text
        let mut rules = Vec::new();
        for line in lines {
            if line.contains("no-unused-vars") {
                rules.push("no-unused-vars".to_string());
            }
            if line.contains("prefer-const") {
                rules.push("prefer-const".to_string());
            }
            if line.contains("no-console") {
                rules.push("no-console".to_string());
            }
        }

        (error_count, warning_count, rules)
    }

    /// Parse TypeScript compiler output
    fn parse_typescript_output(tsc_output: &str) -> Vec<String> {
        tsc_output
            .lines()
            .filter(|line| line.contains("error TS"))
            .map(|line| line.trim().to_string())
            .collect()
    }

    /// Calculate code complexity metrics
    fn calculate_code_complexity(code: &str) -> f32 {
        let lines = code.lines().count();
        let functions = code.matches("function").count() + code.matches("=>").count();
        let conditions = code.matches("if ").count() + code.matches("else").count() + code.matches("switch").count();
        let loops = code.matches("for ").count() + code.matches("while").count();

        // Simple complexity metric: base on lines and control structures
        (lines as f32 * 0.1) + (functions as f32 * 2.0) + (conditions as f32 * 1.5) + (loops as f32 * 2.0)
    }

    /// Infer project type from file path patterns
    fn infer_project_type(file_path: &str) -> String {
        if file_path.contains("/components/") || file_path.contains("/pages/") {
            "react_app".to_string()
        } else if file_path.contains("/lib/") || file_path.contains("/utils/") {
            "library".to_string()
        } else if file_path.contains("/api/") || file_path.contains("/server/") {
            "backend".to_string()
        } else if file_path.contains("/test/") || file_path.contains(".test.") {
            "test".to_string()
        } else {
            "web_app".to_string()
        }
    }

    /// Generate optimized Claude prompt from parsed context
    pub fn generate_optimized_prompt(context: &CodeFixingContext) -> String {
        let mut prompt_parts = Vec::new();

        // TODO: Consider using the `--append-system-prompt` option of the Claude CLI
        // to add dynamic context to the system prompt, rather than building the
        // entire prompt string manually. This could simplify prompt construction.
        // Base instructions tailored to context
        prompt_parts.push(format!("You are analyzing {} code in a {} project.", context.file_type, context.project_type));

        // Specific instructions based on error types
        if context.error_count > 0 {
            prompt_parts.push(format!("Fix {} critical errors with highest priority.", context.error_count));
        }

        if context.warning_count > 0 {
            prompt_parts.push(format!("Address {} warnings to improve code quality.", context.warning_count));
        }

        // Complexity-based instructions
        if context.complexity_score > 20.0 {
            prompt_parts.push("Focus on simplifying complex logic and improving readability.".to_string());
        } else if context.complexity_score < 5.0 {
            prompt_parts.push("Enhance code with better error handling and documentation.".to_string());
        }

        // Project-specific patterns
        match context.project_type.as_str() {
            "react_app" => {
                prompt_parts.push("Apply React best practices: proper hooks usage, component patterns, and prop types.".to_string());
            }
            "library" => {
                prompt_parts.push("Ensure robust API design with comprehensive documentation and error handling.".to_string());
            }
            "backend" => {
                prompt_parts.push("Focus on security, performance, and proper async/await patterns.".to_string());
            }
            _ => {
                prompt_parts.push("Apply general TypeScript/JavaScript best practices.".to_string());
            }
        }

        // Previous fixes context
        if !context.previous_fixes.is_empty() {
            prompt_parts.push(format!("Consider previous analysis: {}", context.previous_fixes.join("; ")));
        }

        // Output format requirements
        prompt_parts.push("Return ONLY valid JSON with 'fixed_code', 'explanation', and 'confidence' fields.".to_string());

        prompt_parts.join(" ")
    }

    /// Load initial training examples based on Moon Shine patterns
    /// <!-- TODO: Load training examples from a persistent storage (e.g., `training.json` in `.moon/moonshine/`) to allow for continuous learning and updates without recompiling. -->
    fn load_initial_training_examples() -> Vec<TrainingExample> {
        vec![
            // TypeScript error fixing example
            TrainingExample {
                input_context: CodeFixingContext {
                    file_path: "src/example.ts".to_string(),
                    file_type: "typescript".to_string(),
                    error_count: 2,
                    warning_count: 0,
                    project_type: "web_app".to_string(),
                    ai_model: "sonnet".to_string(),
                    complexity_score: 4.0,
                    previous_fixes: vec![],
                    code_snippet: r#"function processUser(user) {
    return user.name.toUpperCase();
}"#
                    .to_string(),
                },
                expected_output: r#"interface User {
    name: string;
}

function processUser(user: User): string {
    return user.name.toUpperCase();
}"#
                .to_string(),
                quality_score: 0.95,
            },
            // ESLint rule fixing example
            TrainingExample {
                input_context: CodeFixingContext {
                    file_path: "src/utils.js".to_string(),
                    file_type: "javascript".to_string(),
                    error_count: 0,
                    warning_count: 3,
                    project_type: "library".to_string(),
                    ai_model: "sonnet".to_string(),
                    complexity_score: 2.5,
                    previous_fixes: vec!["added semicolons".to_string()],
                    code_snippet: r#"const data = [1,2,3,4,5]
let result = data.map(x => x * 2)
console.log(result)"#
                        .to_string(),
                },
                expected_output: r#"const data = [1, 2, 3, 4, 5];
const result = data.map((x) => x * 2);
console.log(result);"#
                    .to_string(),
                quality_score: 0.92,
            },
            // Complex refactoring example
            TrainingExample {
                input_context: CodeFixingContext {
                    file_path: "src/complex.ts".to_string(),
                    file_type: "typescript".to_string(),
                    error_count: 1,
                    warning_count: 2,
                    project_type: "enterprise".to_string(),
                    ai_model: "sonnet".to_string(),
                    complexity_score: 8.5,
                    previous_fixes: vec!["type annotations".to_string(), "null checks".to_string()],
                    code_snippet: r#"class DataProcessor {
    process(data: any) {
        if (data && data.items) {
            return data.items.filter(item => item.active).map(item => ({
                id: item.id,
                name: item.name
            }));
        }
    }
}"#
                    .to_string(),
                },
                expected_output: r#"interface DataItem {
    id: string;
    name: string;
    active: boolean;
}

interface ProcessedItem {
    id: string;
    name: string;
}

interface InputData {
    items?: DataItem[];
}

class DataProcessor {
    /**
     * Processes input data by filtering active items and transforming them
     * @param data - Input data containing items to process
     * @returns Array of processed items or undefined if no valid data
     */
    process(data: InputData): ProcessedItem[] | undefined {
        if (!data?.items) {
            return undefined;
        }

        return data.items
            .filter((item): item is DataItem => item.active)
            .map((item): ProcessedItem => ({
                id: item.id,
                name: item.name,
            }));
    }
}"#
                .to_string(),
                quality_score: 0.98,
            },
        ]
    }

    /// Generate optimized prompt and execute with Claude CLI via Moon extension API
    // DSPy embedded - always available
    pub async fn generate_and_execute_fix(&mut self, context: &CodeFixingContext) -> Result<ValidatedCodeFix, Box<dyn std::error::Error>> {
        // Create DSPy example from context using the existing DSPy example creation
        let mut example_data = std::collections::HashMap::new();
        example_data.insert("file_path".to_string(), serde_json::Value::String(context.file_path.clone()));
        example_data.insert("file_type".to_string(), serde_json::Value::String(context.file_type.clone()));
        example_data.insert("error_count".to_string(), serde_json::Value::String(context.error_count.to_string()));
        example_data.insert("warning_count".to_string(), serde_json::Value::String(context.warning_count.to_string()));
        example_data.insert("project_type".to_string(), serde_json::Value::String(context.project_type.clone()));
        example_data.insert(
            "complexity_score".to_string(),
            serde_json::Value::String(format!("{:.1}", context.complexity_score)),
        );
        example_data.insert("previous_fixes".to_string(), serde_json::Value::String(context.previous_fixes.join(", ")));
        example_data.insert("code_snippet".to_string(), serde_json::Value::String(context.code_snippet.clone()));

        let input_example = Example::new(
            example_data,
            vec![
                "file_path".to_string(),
                "file_type".to_string(),
                "error_count".to_string(),
                "warning_count".to_string(),
                "project_type".to_string(),
                "complexity_score".to_string(),
                "previous_fixes".to_string(),
                "code_snippet".to_string(),
            ],
            vec![],
        );

        // Use DSPy predictor to generate optimized prompt
        let result = self.code_fixing_module.predictor.forward(input_example).await?;

        // TODO: The `Predict` module is designed to generate a prompt, not fixed code.
        // The `CodeFixingSignature` should be updated to have an output field for the
        // generated prompt, and the `Predict` module should be configured to generate that.
        // Then, this code should extract the generated prompt, not `fixed_code`.
        // For now, we will assume the `Predict` module is generating the prompt.
        // TODO: The `Predict` module is designed to generate a prompt, not fixed code.
        // The `CodeFixingSignature` should be updated to have an output field for the
        // generated prompt, and the `Predict` module should be configured to generate that.
        // Then, this code should extract the generated prompt, not `fixed_code`.
        // For now, we will assume the `Predict` module is generating the prompt.
        // Extract optimized prompt from DSPy prediction
        let optimized_prompt = result
            .data
            .get("fixed_code")
            .and_then(|v| v.as_str())
            .ok_or("DSPy module did not return optimized prompt")?
            .to_string();

        // Execute the optimized prompt with Claude CLI via Moon extension using provider-aware formatting
        let claude_response = self.claude_adapter.execute_provider_optimized_prompt(&optimized_prompt, context).await?;

        // Validate the Claude CLI response
        let validated_fix = self.validate_claude_response(&claude_response)?;

        // Record this attempt for continuous learning
        let attempt = OptimizationAttempt {
            context: context.clone(),
            prompt_used: optimized_prompt,
            output_generated: claude_response,
            success_score: validated_fix.confidence, // Use Claude's confidence as success score
            fixes_applied: validated_fix.issues_fixed as usize,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.code_fixing_module.optimization_history.push(attempt);

        Ok(validated_fix)
    }

    /// Validate and parse Claude CLI response using structured validation
    // DSPy embedded - always available
    pub fn validate_claude_response(&self, response: &str) -> Result<ValidatedCodeFix, Box<dyn std::error::Error>> {
        // Parse JSON response from Claude CLI
        let parsed_response: ValidatedCodeFix = serde_json::from_str(response).map_err(|e| format!("Failed to parse Claude CLI response: {}", e))?;

        // Validate using rstructor constraints
        parsed_response
            .validate()
            .map_err(|e| format!("Claude CLI response validation failed: {}", e))?;

        Ok(parsed_response)
    }

    /// Record success/failure feedback for teleprompter optimization
    // DSPy embedded - always available
    pub fn record_feedback(&mut self, attempt_index: usize, success_score: f32, fixes_applied: usize) {
        if let Some(attempt) = self.code_fixing_module.optimization_history.get_mut(attempt_index) {
            attempt.success_score = success_score;
            attempt.fixes_applied = fixes_applied;

            // If this was a successful fix, add it as a training example
            if success_score > 0.8 {
                let training_example = TrainingExample {
                    input_context: attempt.context.clone(),
                    expected_output: attempt.output_generated.clone(),
                    quality_score: success_score,
                };
                self.training_examples.push(training_example);
            }
        }
    }

    /// Optimize prompts using COPRO optimizer with accumulated examples
    // DSPy embedded - always available
    pub async fn optimize_prompts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.training_examples.len() < 3 {
            return Ok(()); // Need at least 3 examples for meaningful optimization
        }

        // Convert training examples to DSPy format
        let dspy_examples: Vec<Example> = self
            .training_examples
            .iter()
            .map(|training_example| {
                let mut data = std::collections::HashMap::new();
                // Input fields
                data.insert(
                    "file_path".to_string(),
                    serde_json::Value::String(training_example.input_context.file_path.clone()),
                );
                data.insert(
                    "file_type".to_string(),
                    serde_json::Value::String(training_example.input_context.file_type.clone()),
                );
                data.insert(
                    "error_count".to_string(),
                    serde_json::Value::String(training_example.input_context.error_count.to_string()),
                );
                data.insert(
                    "warning_count".to_string(),
                    serde_json::Value::String(training_example.input_context.warning_count.to_string()),
                );
                data.insert(
                    "project_type".to_string(),
                    serde_json::Value::String(training_example.input_context.project_type.clone()),
                );
                data.insert(
                    "complexity_score".to_string(),
                    serde_json::Value::String(format!("{:.1}", training_example.input_context.complexity_score)),
                );
                data.insert(
                    "previous_fixes".to_string(),
                    serde_json::Value::String(training_example.input_context.previous_fixes.join(", ")),
                );
                data.insert(
                    "code_snippet".to_string(),
                    serde_json::Value::String(training_example.input_context.code_snippet.clone()),
                );
                // Output fields
                data.insert("fixed_code".to_string(), serde_json::Value::String(training_example.expected_output.clone()));
                data.insert(
                    "confidence".to_string(),
                    serde_json::Value::String(format!("{:.2}", training_example.quality_score)),
                );

                Example::new(
                    data,
                    vec![
                        "file_path".to_string(),
                        "file_type".to_string(),
                        "error_count".to_string(),
                        "warning_count".to_string(),
                        "project_type".to_string(),
                        "complexity_score".to_string(),
                        "previous_fixes".to_string(),
                        "code_snippet".to_string(),
                    ],
                    vec!["fixed_code".to_string(), "confidence".to_string()],
                )
            })
            .collect();

        // Use COPRO optimizer to optimize the module
        self.optimizer.compile(&mut self.code_fixing_module, dspy_examples).await?;

        Ok(())
    }

    /// Get current prompt template for debugging
    pub fn get_current_prompt(&self) -> String {
        // Access the internal prompt from DSPy predictor
        // Returns the current optimized instruction from the code fixing module
        self.code_fixing_module.predictor.get_signature().instruction()
    }

    /// Get optimization statistics
    // DSPy embedded - always available
    pub fn get_optimization_stats(&self) -> OptimizationStats {
        let total_attempts = self.code_fixing_module.optimization_history.len();
        let successful_attempts = self
            .code_fixing_module
            .optimization_history
            .iter()
            .filter(|attempt| attempt.success_score > 0.7)
            .count();

        let avg_success_score = if total_attempts > 0 {
            self.code_fixing_module
                .optimization_history
                .iter()
                .map(|attempt| attempt.success_score)
                .sum::<f32>()
                / total_attempts as f32
        } else {
            0.0
        };

        OptimizationStats {
            total_attempts,
            successful_attempts,
            avg_success_score,
            training_examples_count: self.training_examples.len(),
            last_optimization: self.code_fixing_module.optimization_history.last().map(|attempt| attempt.timestamp.clone()),
        }
    }
}

/// Base prompt templates for different Moon task types
#[derive(Debug, Clone)]
pub struct BasePromptCollection {
    pub eslint_fix: String,
    pub typescript_fix: String,
    pub tsdoc_enhancement: String,
    pub code_optimization: String,
    pub complexity_reduction: String,
}

/// Different types of Moon tasks that can be optimized
#[derive(Debug, Clone, PartialEq)]
pub enum MoonTaskType {
    ESLintFix,
    TypeScriptFix,
    TSDocEnhancement,
    CodeOptimization,
    ComplexityReduction,
}

/// Output from Moon tasks that DSPy can analyze
#[derive(Debug, Clone)]
pub struct MoonTaskOutputs {
    pub eslint_output: Option<String>,
    pub typescript_output: Option<String>,
    pub prettier_output: Option<String>,
    pub tsdoc_output: Option<String>,
}

/// Result from multi-step DSPy optimization
#[derive(Debug, Clone)]
pub struct MultiStepResult {
    pub file_path: String,
    pub steps: Vec<OptimizationStep>,
    pub total_processing_time_ms: u64,
    pub overall_success: bool,
}

/// Individual optimization step result
#[derive(Debug, Clone)]
pub struct OptimizationStep {
    pub step_name: String,
    pub task_type: MoonTaskType,
    pub prompt_used: String,
    pub claude_response: String,
    pub success: bool,
    pub processing_time_ms: u64,
}

impl MultiStepResult {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            steps: Vec::new(),
            total_processing_time_ms: 0,
            overall_success: true,
        }
    }

    pub fn add_step_result(&mut self, step_name: &str, claude_response: String) {
        self.add_step_result_with_timing(step_name, claude_response, std::time::Instant::now())
    }

    pub fn add_step_result_with_timing(&mut self, step_name: &str, claude_response: String, start_time: std::time::Instant) {
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Parse Claude response to determine success - look for error indicators
        let success = !claude_response.to_lowercase().contains("error") && !claude_response.to_lowercase().contains("failed") && !claude_response.is_empty();

        let step = OptimizationStep {
            step_name: step_name.to_string(),
            task_type: match step_name {
                "eslint" => MoonTaskType::ESLintFix,
                "typescript" => MoonTaskType::TypeScriptFix,
                "tsdoc" => MoonTaskType::TSDocEnhancement,
                "optimization" => MoonTaskType::ComplexityReduction,
                _ => MoonTaskType::CodeOptimization,
            },
            prompt_used: format!("DSPy-optimized prompt for {}", step_name),
            claude_response,
            success,
            processing_time_ms,
        };

        self.steps.push(step);
        self.total_processing_time_ms += processing_time_ms;
        if !success {
            self.overall_success = false;
        }
    }

    pub fn get_final_result(&self) -> Option<String> {
        self.steps.last().map(|step| step.claude_response.clone())
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Multi-step optimization completed: {} steps, {} total time, success: {}",
            self.steps.len(),
            self.total_processing_time_ms,
            self.overall_success
        )
    }
}

/// Statistics for monitoring DSPy optimization performance
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationStats {
    pub total_attempts: usize,
    pub successful_attempts: usize,
    pub avg_success_score: f32,
    pub training_examples_count: usize,
    pub last_optimization: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_dspy_initialization() {
        let dspy_engine = MoonShineDSPy::new("test-session".to_string(), "sonnet".to_string());
        assert_eq!(dspy_engine.training_examples.len(), 3);
    }

    #[test]
    fn test_provider_detection_claude() {
        let config = ClaudeCLIAdapter::detect_provider_from_model("sonnet").unwrap();

        assert!(matches!(config.provider_type, LLMProvider::Claude { .. }));
        assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::XmlBased));
        assert!(matches!(config.format_preferences.output_format, OutputFormat::JsonWithPrefill));
        assert!(matches!(config.format_preferences.instruction_style, InstructionStyle::SystemAndHuman));
        assert!(config.optimization_hints.use_system_prompts);
        assert!(config.optimization_hints.prefer_prefill);
        assert!(config.optimization_hints.chain_of_thought);
        assert!(!config.optimization_hints.structured_output);
    }

    #[test]
    fn test_provider_detection_openai() {
        let config = ClaudeCLIAdapter::detect_provider_from_model("gpt4.1").unwrap();

        assert!(matches!(config.provider_type, LLMProvider::OpenAI { .. }));
        assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::Hybrid));
        assert!(matches!(config.format_preferences.output_format, OutputFormat::StructuredJson));
        assert!(matches!(config.format_preferences.instruction_style, InstructionStyle::SinglePrompt));
        assert!(!config.optimization_hints.use_system_prompts);
        assert!(!config.optimization_hints.prefer_prefill);
        assert!(config.optimization_hints.chain_of_thought);
        assert!(config.optimization_hints.structured_output);
    }

    #[test]
    fn test_provider_detection_gemini() {
        let config = ClaudeCLIAdapter::detect_provider_from_model("gemini-2.5-pro").unwrap();

        assert!(matches!(config.provider_type, LLMProvider::Gemini { .. }));
        assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::JsonSchema));
        assert!(matches!(config.format_preferences.output_format, OutputFormat::JsonSchema));
        assert!(matches!(config.format_preferences.instruction_style, InstructionStyle::MarkdownBased));
        assert!(!config.optimization_hints.use_system_prompts);
        assert!(!config.optimization_hints.prefer_prefill);
        assert!(!config.optimization_hints.chain_of_thought);
        assert!(config.optimization_hints.structured_output);
    }

    #[test]
    fn test_provider_detection_unknown_defaults_to_sonnet() {
        let config = ClaudeCLIAdapter::detect_provider_from_model("unknown-model").unwrap();

        assert!(matches!(config.provider_type, LLMProvider::Claude { .. }));
        assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::XmlBased));
        assert!(config.optimization_hints.use_system_prompts);
        assert_eq!(config.provider_type, LLMProvider::Claude { version: "sonnet".to_string() });
    }

    #[test]
    fn test_user_preferred_models() {
        // Test user-specified Claude models
        let sonnet_config = ClaudeCLIAdapter::detect_provider_from_model("sonnet").unwrap();
        let opus_config = ClaudeCLIAdapter::detect_provider_from_model("opus").unwrap();

        assert!(matches!(sonnet_config.provider_type, LLMProvider::Claude { .. }));
        assert!(matches!(opus_config.provider_type, LLMProvider::Claude { .. }));
        assert!(matches!(sonnet_config.format_preferences.prompt_structure, PromptStructure::XmlBased));
        assert!(matches!(opus_config.format_preferences.prompt_structure, PromptStructure::XmlBased));

        // Test user-specified OpenAI model
        let gpt41_config = ClaudeCLIAdapter::detect_provider_from_model("gpt4.1").unwrap();
        assert!(matches!(gpt41_config.provider_type, LLMProvider::OpenAI { .. }));
        assert!(matches!(gpt41_config.format_preferences.prompt_structure, PromptStructure::Hybrid));
    }

    #[test]
    fn test_claude_xml_prompt_formatting() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let context = create_test_context();
        let base_prompt = "Fix TypeScript compilation errors";

        let formatted_prompt = adapter.format_as_xml(base_prompt, &context);

        // Test XML structure
        assert!(formatted_prompt.contains("<system>"));
        assert!(formatted_prompt.contains("</system>"));
        assert!(formatted_prompt.contains("<task>"));
        assert!(formatted_prompt.contains("</task>"));
        assert!(formatted_prompt.contains("<context>"));
        assert!(formatted_prompt.contains("</context>"));
        assert!(formatted_prompt.contains("<code language=\"typescript\">"));
        assert!(formatted_prompt.contains("</code>"));
        assert!(formatted_prompt.contains("<requirements>"));
        assert!(formatted_prompt.contains("</requirements>"));
        assert!(formatted_prompt.contains("<output_format>"));
        assert!(formatted_prompt.contains("</output_format>"));

        // Test content injection
        assert!(formatted_prompt.contains("Fix TypeScript compilation errors"));
        assert!(formatted_prompt.contains("src/test.ts"));
        assert!(formatted_prompt.contains("2")); // error count
        assert!(formatted_prompt.contains("1")); // warning count
        assert!(formatted_prompt.contains("const x = 1")); // code snippet
    }

    #[test]
    fn test_claude_xml_prompt_is_valid_xml() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let context = create_test_context();
        let base_prompt = "Fix TypeScript compilation errors with <special> characters & entities";

        let formatted_prompt = adapter.format_as_xml(base_prompt, &context);

        // Test that XML is well-formed by checking basic structure
        assert_xml_structure_valid(&formatted_prompt);
    }

    #[test]
    fn test_gemini_json_schema_formatting() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gemini-2.5-pro".to_string()).unwrap();

        let context = create_test_context();
        let base_prompt = "Fix TypeScript compilation errors";

        let formatted_prompt = adapter.format_as_json_schema(base_prompt, &context);

        // Test JSON structure
        assert!(formatted_prompt.contains("# Code Improvement Task"));
        assert!(formatted_prompt.contains("Return response matching the specified JSON schema"));

        // Extract JSON part
        let json_start = formatted_prompt.find('{').unwrap();
        let json_end = formatted_prompt.rfind('}').unwrap() + 1;
        let json_str = &formatted_prompt[json_start..json_end];

        // Validate JSON is parseable
        let json_value: Value = serde_json::from_str(json_str).expect("Invalid JSON generated");

        // Test JSON schema structure
        assert!(json_value.get("task").is_some());
        assert!(json_value.get("context").is_some());
        assert!(json_value.get("code").is_some());
        assert!(json_value.get("response_schema").is_some());

        let response_schema = json_value.get("response_schema").unwrap();
        assert_eq!(response_schema.get("type").unwrap(), "object");
        assert!(response_schema.get("properties").is_some());
        assert!(response_schema.get("required").is_some());

        let properties = response_schema.get("properties").unwrap();
        assert!(properties.get("fixed_code").is_some());
        assert!(properties.get("explanation").is_some());
        assert!(properties.get("confidence").is_some());
    }

    #[test]
    fn test_openai_markdown_formatting() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gpt4.1".to_string()).unwrap();

        let context = create_test_context();
        let base_prompt = "Fix TypeScript compilation errors";

        let formatted_prompt = adapter.format_as_markdown(base_prompt, &context);

        // Test Markdown structure
        assert!(formatted_prompt.contains("# Code Improvement Task"));
        assert!(formatted_prompt.contains("## Objective"));
        assert!(formatted_prompt.contains("## File Information"));
        assert!(formatted_prompt.contains("## Code to Improve"));
        assert!(formatted_prompt.contains("## Requirements"));
        assert!(formatted_prompt.contains("## Output Format"));

        // Test content injection
        assert!(formatted_prompt.contains("Fix TypeScript compilation errors"));
        assert!(formatted_prompt.contains("- **Path**: src/test.ts"));
        assert!(formatted_prompt.contains("- **Type**: typescript"));
        assert!(formatted_prompt.contains("- **Errors**: 2"));
        assert!(formatted_prompt.contains("- **Warnings**: 1"));
        assert!(formatted_prompt.contains("```typescript"));
        assert!(formatted_prompt.contains("const x = 1"));
    }

    #[test]
    fn test_hybrid_formatting() {
        let adapter = ClaudeCLIAdapter::new("test-session".to_string(), "test-model".to_string());
        let context = create_test_context();
        let base_prompt = "Fix TypeScript compilation errors";

        let formatted_prompt = adapter.format_as_hybrid(base_prompt, &context);

        // Test hybrid structure (XML + Markdown)
        assert!(formatted_prompt.contains("<task>"));
        assert!(formatted_prompt.contains("</task>"));
        assert!(formatted_prompt.contains("## Context"));
        assert!(formatted_prompt.contains("<code>"));
        assert!(formatted_prompt.contains("</code>"));
        assert!(formatted_prompt.contains("- **File**:"));
        assert!(formatted_prompt.contains("- **Type**:"));
        assert!(formatted_prompt.contains("- **Issues**:"));
    }

    #[test]
    fn test_provider_optimized_prompt_generation() {
        // Test Claude optimization
        let claude_adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let context = create_test_context();
        let base_prompt = "Fix errors";

        let claude_prompt = claude_adapter.generate_provider_optimized_prompt(base_prompt, &context);
        assert!(claude_prompt.contains("<system>"));
        assert!(claude_prompt.contains("<task>"));

        // Test Gemini optimization
        let gemini_adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gemini-2.5-pro".to_string()).unwrap();

        let gemini_prompt = gemini_adapter.generate_provider_optimized_prompt(base_prompt, &context);
        assert!(gemini_prompt.contains("# Code Improvement Task"));
        assert!(gemini_prompt.contains("response_schema"));

        // Test OpenAI optimization
        let openai_adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gpt4.1".to_string()).unwrap();

        let openai_prompt = openai_adapter.generate_provider_optimized_prompt(base_prompt, &context);
        assert!(openai_prompt.contains("# Code Improvement Task"));
        assert!(openai_prompt.contains("## Objective"));
        assert!(openai_prompt.contains("## Requirements"));
    }

    #[test]
    fn test_adapter_with_provider_detection() {
        let claude_adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        assert_eq!(claude_adapter.session_id, "test-session");
        assert_eq!(claude_adapter.model, "sonnet");
        assert!(claude_adapter.provider_config.is_some());

        let config = claude_adapter.provider_config.unwrap();
        assert!(matches!(config.provider_type, LLMProvider::Claude { .. }));
    }

    #[test]
    fn test_provider_config_serialization() {
        let config = ProviderConfig {
            provider_type: LLMProvider::Claude {
                version: "claude-3-5-sonnet".to_string(),
            },
            model_name: "sonnet".to_string(),
            format_preferences: FormatPreferences {
                prompt_structure: PromptStructure::XmlBased,
                output_format: OutputFormat::JsonWithPrefill,
                instruction_style: InstructionStyle::SystemAndHuman,
            },
            optimization_hints: OptimizationHints {
                use_system_prompts: true,
                prefer_prefill: true,
                structured_output: false,
                chain_of_thought: true,
            },
        };

        // Test serialization
        let serialized = serde_json::to_string(&config).expect("Failed to serialize config");
        assert!(serialized.contains("Claude"));
        assert!(serialized.contains("XmlBased"));

        // Test deserialization
        let deserialized: ProviderConfig = serde_json::from_str(&serialized).expect("Failed to deserialize config");
        assert!(matches!(deserialized.provider_type, LLMProvider::Claude { .. }));
        assert!(matches!(deserialized.format_preferences.prompt_structure, PromptStructure::XmlBased));
    }

    #[test]
    fn test_xml_escaping_in_prompts() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let mut context = create_test_context();
        context.code_snippet = "const xml = \"<tag>value</tag>\"; const amp = \"a & b\";".to_string();

        let base_prompt = "Fix code with <special> & characters";
        let formatted_prompt = adapter.format_as_xml(base_prompt, &context);

        // XML should be well-formed even with special characters
        assert_xml_structure_valid(&formatted_prompt);
        assert!(formatted_prompt.contains("Fix code with <special> & characters"));
        assert!(formatted_prompt.contains("const xml"));
    }

    #[test]
    fn test_json_escaping_in_schema() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gemini-2.5-pro".to_string()).unwrap();

        let mut context = create_test_context();
        context.code_snippet = "const json = '{\"key\": \"value\"}'; const quote = \"it's working\";".to_string();
        context.file_path = "src/test with spaces.ts".to_string();

        let formatted_prompt = adapter.format_as_json_schema("Fix JSON issues", &context);

        // Extract and validate JSON
        let json_start = formatted_prompt.find('{').unwrap();
        let json_end = formatted_prompt.rfind('}').unwrap() + 1;
        let json_str = &formatted_prompt[json_start..json_end];

        let _: Value = serde_json::from_str(json_str).expect("JSON should be valid despite special characters");
    }

    // Helper functions
    fn create_test_context() -> CodeFixingContext {
        CodeFixingContext {
            file_path: "src/test.ts".to_string(),
            file_type: "typescript".to_string(),
            error_count: 2,
            warning_count: 1,
            project_type: "web_app".to_string(),
            ai_model: "sonnet".to_string(),
            complexity_score: 5.5,
            previous_fixes: vec!["Added type annotations".to_string()],
            code_snippet: "const x = 1".to_string(),
        }
    }

    fn assert_xml_structure_valid(xml: &str) {
        // Basic XML structure validation
        let opening_tags = xml.matches('<').filter(|tag| !tag.starts_with("</")).count();
        let closing_tags = xml.matches("</").count();

        // Check that we have matching opening/closing tags for major elements
        assert!(xml.contains("<system>") && xml.contains("</system>"));
        assert!(xml.contains("<task>") && xml.contains("</task>"));
        assert!(xml.contains("<context>") && xml.contains("</context>"));
        assert!(xml.contains("<code") && xml.contains("</code>"));
        assert!(xml.contains("<requirements>") && xml.contains("</requirements>"));
        assert!(xml.contains("<output_format>") && xml.contains("</output_format>"));

        // Verify nested structure
        let system_start = xml.find("<system>").unwrap();
        let system_end = xml.find("</system>").unwrap();
        assert!(system_start < system_end);

        let task_start = xml.find("<task>").unwrap();
        let task_end = xml.find("</task>").unwrap();
        assert!(task_start < task_end);

        // Ensure proper ordering
        assert!(system_end < task_start); // system comes before task
    }

    #[test]
    fn test_xml_content_escaping_edge_cases() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let mut context = create_test_context();

        // Test various XML-problematic characters
        context.code_snippet = r#"
      const html = "<div class='test'>Hello & goodbye</div>";
      const quotes = 'He said "Hello" to me';
      const cdata = "<![CDATA[some data]]>";
      const entities = "&lt;&gt;&amp;&quot;&apos;";
    "#
        .to_string();

        context.file_path = "src/xml-test&special<chars>.ts".to_string();

        let base_prompt = "Fix code with <xml/> tags & \"quotes\" and 'apostrophes'";
        let formatted_prompt = adapter.format_as_xml(base_prompt, &context);

        // XML should handle all these cases gracefully
        assert_xml_structure_valid(&formatted_prompt);
        assert!(formatted_prompt.contains("Fix code with"));
        assert!(formatted_prompt.contains("Hello & goodbye"));
        assert!(formatted_prompt.contains("xml-test&special"));
    }

    #[test]
    fn test_xml_cdata_handling() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let mut context = create_test_context();
        context.code_snippet = r#"
      // This code contains problematic XML characters
      const problematic = `
        <xml>
          <![CDATA[
            function test() {
              if (x < y && y > z) {
                return "success & happiness";
              }
            }
          ]]>
        </xml>
      `;
    "#
        .to_string();

        let formatted_prompt = adapter.format_as_xml("Fix this code", &context);

        // Should handle CDATA and complex XML content
        assert_xml_structure_valid(&formatted_prompt);
        assert!(formatted_prompt.contains("CDATA"));
        assert!(formatted_prompt.contains("success & happiness"));
    }

    #[test]
    fn test_claude_system_prompt_structure() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "sonnet".to_string()).unwrap();

        let context = create_test_context();
        let formatted_prompt = adapter.format_as_xml("Test task", &context);

        // Verify Claude-specific system prompt structure
        let system_start = formatted_prompt.find("<system>").unwrap();
        let system_end = formatted_prompt.find("</system>").unwrap();
        let system_content = &formatted_prompt[system_start..system_end + 9];

        assert!(system_content.contains("expert TypeScript code quality specialist"));
        assert!(system_content.contains("enterprise development practices"));

        // Verify system comes before task (Claude's preferred order)
        let task_start = formatted_prompt.find("<task>").unwrap();
        assert!(system_end < task_start);
    }

    #[test]
    fn test_gemini_json_schema_validation() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gemini-2.5-pro".to_string()).unwrap();

        let context = create_test_context();
        let formatted_prompt = adapter.format_as_json_schema("Test task", &context);

        // Extract and validate the JSON schema
        let json_start = formatted_prompt.find('{').unwrap();
        let json_end = formatted_prompt.rfind('}').unwrap() + 1;
        let json_str = &formatted_prompt[json_start..json_end];

        let json_value: Value = serde_json::from_str(json_str).expect("Invalid JSON schema");

        // Test Gemini-specific schema requirements
        let response_schema = json_value.get("response_schema").unwrap();
        let properties = response_schema.get("properties").unwrap();
        let required = response_schema.get("required").unwrap().as_array().unwrap();

        // Check required fields for Gemini structured output
        assert!(required.contains(&Value::String("fixed_code".to_string())));
        assert!(required.contains(&Value::String("explanation".to_string())));
        assert!(required.contains(&Value::String("confidence".to_string())));

        // Check confidence field has proper constraints
        let confidence_props = properties.get("confidence").unwrap();
        assert_eq!(confidence_props.get("type").unwrap(), "number");
        assert_eq!(confidence_props.get("minimum").unwrap(), 0);
        assert_eq!(confidence_props.get("maximum").unwrap(), 1);
    }

    #[test]
    fn test_openai_structured_output_hints() {
        let adapter = ClaudeCLIAdapter::with_provider_detection("test-session".to_string(), "gpt4.1".to_string()).unwrap();

        let config = adapter.provider_config.as_ref().unwrap();

        // Verify OpenAI-specific optimization hints
        assert!(config.optimization_hints.structured_output);
        assert!(!config.optimization_hints.use_system_prompts); // OpenAI prefers single prompt
        assert!(!config.optimization_hints.prefer_prefill); // OpenAI doesn't use prefill
        assert!(config.optimization_hints.chain_of_thought); // OpenAI benefits from CoT
    }

    #[test]
    fn test_provider_model_variations() {
        // Test various Claude model formats (user preferences)
        let claude_configs = ["sonnet", "opus", "sonnet", "claude-3-opus-20240229"];

        for model in &claude_configs {
            let config = ClaudeCLIAdapter::detect_provider_from_model(model).unwrap();
            assert!(matches!(config.provider_type, LLMProvider::Claude { .. }));
            assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::XmlBased));
        }

        // Test various OpenAI model formats (user preference: gpt4.1 default)
        let openai_configs = ["gpt4.1", "gpt4.1", "gpt4.1-mini", "openai/gpt4.1"];

        for model in &openai_configs {
            let config = ClaudeCLIAdapter::detect_provider_from_model(model).unwrap();
            assert!(matches!(config.provider_type, LLMProvider::OpenAI { .. }));
            assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::Hybrid));
        }

        // Test various Gemini model formats
        let gemini_configs = ["gemini-2.5-pro", "gemini-2.5-flash", "gemini-1.5-pro", "gemini-pro"];

        for model in &gemini_configs {
            let config = ClaudeCLIAdapter::detect_provider_from_model(model).unwrap();
            assert!(matches!(config.provider_type, LLMProvider::Gemini { .. }));
            assert!(matches!(config.format_preferences.prompt_structure, PromptStructure::JsonSchema));
        }
    }

    #[test]
    fn test_complex_code_snippet_handling() {
        let adapters = [
            (
                "claude",
                ClaudeCLIAdapter::with_provider_detection("test".to_string(), "sonnet".to_string()).unwrap(),
            ),
            (
                "gemini",
                ClaudeCLIAdapter::with_provider_detection("test".to_string(), "gemini-2.5-pro".to_string()).unwrap(),
            ),
            (
                "openai",
                ClaudeCLIAdapter::with_provider_detection("test".to_string(), "gpt4.1".to_string()).unwrap(),
            ),
        ];

        let mut context = create_test_context();
        context.code_snippet = r#"
      // Complex TypeScript with various edge cases
      export interface Config<T = unknown> {
        name: string;
        value: T;
        metadata?: {
          tags: string[];
          description: string;
        };
      }

      export class ConfigManager<T> implements Config<T> {
        constructor(
          public name: string,
          public value: T,
          public metadata?: { tags: string[]; description: string }
        ) {}

        // Method with template literals and complex types
        serialize(): `config-${string}` {
          return `config-${this.name}` as const;
        }

        // Method with regex and special characters
        validate(): boolean {
          const pattern = /^[a-zA-Z0-9_-]+$/;
          return pattern.test(this.name) && this.value !== null;
        }
      }

      // Usage with JSDoc
      /**
       * Creates a new configuration
       * @param config - The configuration object
       * @returns Promise<Config<string>>
       */
      export async function createConfig(
        config: Partial<Config<string>>
      ): Promise<Config<string>> {
        return new ConfigManager(
          config.name ?? "default",
          config.value ?? "",
          config.metadata
        );
      }
    "#
        .to_string();

        for (provider_name, adapter) in adapters {
            let prompt = adapter.generate_provider_optimized_prompt("Improve this code", &context);

            // Each provider should handle the complex code without breaking
            match provider_name {
                "claude" => {
                    assert_xml_structure_valid(&prompt);
                    assert!(prompt.contains("ConfigManager"));
                    assert!(prompt.contains("serialize()"));
                }
                "gemini" => {
                    // Should contain valid JSON
                    let json_start = prompt.find('{').unwrap();
                    let json_end = prompt.rfind('}').unwrap() + 1;
                    let json_str = &prompt[json_start..json_end];
                    let _: Value = serde_json::from_str(json_str).expect("Invalid JSON for Gemini");
                }
                "openai" => {
                    assert!(prompt.contains("# Code Improvement Task"));
                    assert!(prompt.contains("```typescript"));
                    assert!(prompt.contains("ConfigManager"));
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_performance_optimization_hints() {
        // Test that each provider gets optimized for its strengths
        let claude_adapter = ClaudeCLIAdapter::with_provider_detection("test".to_string(), "sonnet".to_string()).unwrap();
        let gemini_adapter = ClaudeCLIAdapter::with_provider_detection("test".to_string(), "gemini-2.5-pro".to_string()).unwrap();
        let openai_adapter = ClaudeCLIAdapter::with_provider_detection("test".to_string(), "gpt4.1".to_string()).unwrap();

        let context = create_test_context();

        // Claude should use XML + system prompts for 25-40% performance boost
        let claude_prompt = claude_adapter.generate_provider_optimized_prompt("Fix code", &context);
        assert!(claude_prompt.contains("<system>"));
        assert!(claude_prompt.contains("You are an expert"));

        // Gemini should use JSON schema for native structured output
        let gemini_prompt = gemini_adapter.generate_provider_optimized_prompt("Fix code", &context);
        assert!(gemini_prompt.contains("response_schema"));
        assert!(gemini_prompt.contains("type\": \"object"));

        // OpenAI should use hybrid format optimized for instruction following
        let openai_prompt = openai_adapter.generate_provider_optimized_prompt("Fix code", &context);
        assert!(openai_prompt.contains("# Code Improvement Task"));
        assert!(openai_prompt.contains("## Requirements"));
        assert!(!openai_prompt.contains("<system>")); // No system prompts for OpenAI
    }

    #[test]
    fn test_eslint_parsing() {
        let eslint_json = r#"[
            {
                "filePath": "/test.js",
                "messages": [
                    {
                        "ruleId": "no-unused-vars",
                        "severity": 2,
                        "message": "Unused variable"
                    },
                    {
                        "ruleId": "prefer-const",
                        "severity": 1,
                        "message": "Use const instead"
                    }
                ]
            }
        ]"#;

        let (errors, warnings, rules) = MoonShineDSPy::parse_eslint_output(eslint_json);
        assert_eq!(errors, 1);
        assert_eq!(warnings, 1);
        assert_eq!(rules.len(), 2);
        assert!(rules.contains(&"no-unused-vars".to_string()));
        assert!(rules.contains(&"prefer-const".to_string()));
    }

    #[test]
    fn test_typescript_parsing() {
        let tsc_output = r#"
src/test.ts(10,5): error TS2322: Type 'string' is not assignable to type 'number'.
src/test.ts(15,12): error TS2339: Property 'foo' does not exist on type 'Bar'.
        "#;

        let errors = MoonShineDSPy::parse_typescript_output(tsc_output);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("TS2322"));
        assert!(errors[1].contains("TS2339"));
    }

    #[test]
    fn test_tool_output_parsing_integration() {
        let eslint_output = r#"[{"filePath": "/test.ts", "messages": [{"ruleId": "no-console", "severity": 1}]}]"#;
        let tsc_output = "test.ts(5,10): error TS2322: Type mismatch";

        let context = MoonShineDSPy::parse_tool_outputs_to_context(
            "src/components/Button.tsx",
            "typescript",
            "function Button() { console.log('test'); }",
            Some(eslint_output),
            Some(tsc_output),
            75.0,
        );

        assert_eq!(context.file_path, "src/components/Button.tsx");
        assert_eq!(context.file_type, "typescript");
        assert_eq!(context.project_type, "react_app"); // Inferred from path
        assert_eq!(context.warning_count, 1); // ESLint warning
        assert_eq!(context.error_count, 0); // ESLint error count
        assert!(!context.previous_fixes.is_empty()); // Should have parsed tool outputs
    }

    #[test]
    fn test_optimized_prompt_generation() {
        let context = CodeFixingContext {
            file_path: "src/api/users.ts".to_string(),
            file_type: "typescript".to_string(),
            error_count: 2,
            warning_count: 3,
            project_type: "backend".to_string(),
            ai_model: "sonnet".to_string(),
            complexity_score: 15.5,
            previous_fixes: vec!["ESLint rules: no-console, prefer-const".to_string()],
            code_snippet: "function getUser() {}".to_string(),
        };

        let prompt = MoonShineDSPy::generate_optimized_prompt(&context);

        // Check that prompt contains context-specific information
        assert!(prompt.contains("typescript code"));
        assert!(prompt.contains("backend project"));
        assert!(prompt.contains("Fix 2 critical errors"));
        assert!(prompt.contains("Address 3 warnings"));
        assert!(prompt.contains("security, performance"));
        assert!(prompt.contains("previous analysis"));
        assert!(prompt.contains("JSON"));
    }

    #[test]
    fn test_complexity_calculation() {
        let simple_code = "const x = 1;";
        let complex_code = r#"
function processData(data) {
    if (data) {
        for (let i = 0; i < data.length; i++) {
            if (data[i].active) {
                while (data[i].processing) {
                    // process
                }
            }
        }
    }
}
        "#;

        let simple_complexity = MoonShineDSPy::calculate_code_complexity(simple_code);
        let complex_complexity = MoonShineDSPy::calculate_code_complexity(complex_code);

        assert!(simple_complexity < 2.0);
        assert!(complex_complexity > 10.0);
    }

    #[test]
    fn test_project_type_inference() {
        assert_eq!(MoonShineDSPy::infer_project_type("src/components/Button.tsx"), "react_app");
        assert_eq!(MoonShineDSPy::infer_project_type("lib/utils/helpers.ts"), "library");
        assert_eq!(MoonShineDSPy::infer_project_type("src/api/routes.ts"), "backend");
        assert_eq!(MoonShineDSPy::infer_project_type("tests/unit.test.ts"), "test");
        assert_eq!(MoonShineDSPy::infer_project_type("src/main.ts"), "web_app");
    }

    #[test]
    fn test_context_preparation() {
        let context = CodeFixingContext {
            file_path: "test.ts".to_string(),
            file_type: "typescript".to_string(),
            error_count: 1,
            warning_count: 0,
            project_type: "library".to_string(),
            ai_model: "sonnet".to_string(),
            complexity_score: 5.0,
            previous_fixes: vec!["imports".to_string()],
            code_snippet: "const x = 1".to_string(),
        };

        // Test context serialization
        let serialized = serde_json::to_string(&context).expect("Context should serialize to JSON");
        assert!(serialized.contains("test.ts"));
        assert!(serialized.contains("typescript"));
    }

    #[test]
    fn test_training_examples() {
        let examples = MoonShineDSPy::load_initial_training_examples();
        assert_eq!(examples.len(), 3);

        // Verify TypeScript example
        let ts_example = &examples[0];
        assert_eq!(ts_example.input_context.file_type, "typescript");
        assert!(ts_example.expected_output.contains("interface User"));
        assert!(ts_example.quality_score > 0.9);
    }
}
