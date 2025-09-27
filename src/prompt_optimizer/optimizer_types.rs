//! Prompt optimization types and configurations
//!
//! Self-documenting types for DSPy-powered prompt optimization.

use serde::{Deserialize, Serialize};

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
    TaskFirst,
    ContextFirst,
    ExampleFirst,
    Conversational,
}

/// Output format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Structured,
    Natural,
    Code,
}

/// Instruction style preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionStyle {
    Direct,
    Conversational,
    Technical,
    Creative,
}

/// Optimization hints for prompt improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationHints {
    pub focus_areas: Vec<String>,
    pub avoid_patterns: Vec<String>,
    pub preferred_examples: usize,
    pub max_context_length: usize,
}

/// DSPy optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSPyConfig {
    pub breadth: usize,
    pub depth: usize,
    pub temperature: f32,
    pub max_iterations: usize,
    pub evaluation_metrics: Vec<String>,
}

impl Default for DSPyConfig {
    fn default() -> Self {
        Self {
            breadth: 5,
            depth: 3,
            temperature: 1.0,
            max_iterations: 10,
            evaluation_metrics: vec!["accuracy".to_string(), "coherence".to_string()],
        }
    }
}

impl Default for OptimizationHints {
    fn default() -> Self {
        Self {
            focus_areas: vec!["clarity".to_string(), "specificity".to_string()],
            avoid_patterns: vec!["ambiguity".to_string(), "verbosity".to_string()],
            preferred_examples: 3,
            max_context_length: 4000,
        }
    }
}

impl Default for FormatPreferences {
    fn default() -> Self {
        Self {
            prompt_structure: PromptStructure::TaskFirst,
            output_format: OutputFormat::Structured,
            instruction_style: InstructionStyle::Technical,
        }
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_type: LLMProvider::Claude { version: "3.5".to_string() },
            model_name: "claude-3-5-sonnet".to_string(),
            format_preferences: FormatPreferences::default(),
            optimization_hints: OptimizationHints::default(),
        }
    }
}
