//! Intelligent AI Provider Router for Moon-Shine
//!
//! Smart routing system that automatically selects the best AI provider based on:
//! - Request capabilities (code fixing, DSPy optimization, analysis)
//! - Provider strengths and availability
//! - Context requirements and performance characteristics
//!
//! Supports multiple AI CLI providers:
//! - Claude CLI (excellent for code reasoning and complex analysis)
//! - Google CLI/Gemini (strong for creative tasks and broad knowledge)
//! - OpenAI Codex CLI (specialized for code generation and completion)
//! - Future providers (extensible architecture)

use crate::error::{Error, Result};
use crate::moon_pdk_interface::AiLinterConfig;
use crate::moon_pdk_interface::{execute_command, ExecCommandInput};
use crate::rulebase::RuleResult as LintIssue;
use extism_pdk::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// AI provider capabilities for intelligent routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub code_analysis: f32,      // 0.0-1.0 rating for code analysis quality
    pub code_generation: f32,    // 0.0-1.0 rating for code generation quality
    pub complex_reasoning: f32,  // 0.0-1.0 rating for complex reasoning tasks
    pub speed: f32,              // 0.0-1.0 rating for response speed
    pub context_length: u32,     // Maximum context length in tokens
    pub supports_sessions: bool, // Whether provider supports session continuity
}

/// Internal AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub name: String,
    pub command: String,
    pub model: String,
    pub api_key_env: Option<String>,
    pub requires_api_key: bool,
    pub capabilities: ProviderCapabilities,
}

impl AIProviderConfig {
    /// Create Claude provider configuration using model from config
    pub fn claude() -> Self {
        let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();
        let model = config.ai_model.unwrap_or_else(|| "sonnet".to_string());

        let uses_oauth = config.claude_uses_oauth.unwrap_or(true);
        let api_key_env = if uses_oauth { None } else { config.claude_api_key_env.clone() };

        Self {
            name: "claude".to_string(),
            command: "claude".to_string(),
            model,
            api_key_env,
            requires_api_key: !uses_oauth,
            capabilities: ProviderCapabilities {
                code_analysis: 0.95,     // Excellent at code analysis
                code_generation: 0.85,   // Very good at code generation
                complex_reasoning: 0.95, // Excellent at complex reasoning
                speed: 0.75,             // Moderate speed
                context_length: 200000,  // Large context window
                supports_sessions: true, // Supports session continuity
            },
        }
    }

    /// Create Google/Gemini provider configuration using model from config
    pub fn google() -> Self {
        let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();
        let model = config.ai_model.unwrap_or_else(|| "gemini-2.5-flash".to_string());

        // Set capabilities based on model
        let (code_analysis, code_generation, complex_reasoning, speed) = match model.as_str() {
            "gemini-2.5-pro" => (0.90, 0.85, 0.90, 0.70),   // Pro model: higher quality, slower
            "gemini-2.5-flash" => (0.80, 0.80, 0.85, 0.90), // Flash model: good quality, faster
            _ => (0.80, 0.80, 0.85, 0.85),                  // Default values
        };

        let uses_oauth = config.gemini_uses_oauth.unwrap_or(true);
        let api_key_env = if uses_oauth { None } else { config.gemini_api_key_env.clone() };

        Self {
            name: "google".to_string(),
            command: "gemini".to_string(),
            model,
            api_key_env,
            requires_api_key: !uses_oauth,
            capabilities: ProviderCapabilities {
                code_analysis,
                code_generation,
                complex_reasoning,
                speed,
                context_length: 100000,  // Large context window
                supports_sessions: true, // Supports session continuity
            },
        }
    }

    /// Create OpenAI Codex provider configuration from global config
    pub fn openai() -> Self {
        let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();

        let uses_oauth = config.codex_uses_oauth.unwrap_or(true);
        let api_key_env = if uses_oauth { None } else { config.codex_api_key_env.clone() };

        Self {
            name: "openai".to_string(),
            command: config.codex_command.unwrap_or_else(|| "codex".to_string()), // Default to system PATH, configurable via config
            model: config.codex_model.unwrap_or_else(|| "gpt-5-codex".to_string()),
            api_key_env,
            requires_api_key: !uses_oauth,
            capabilities: ProviderCapabilities {
                code_analysis: config.codex_code_analysis_rating.unwrap_or(0.88),
                code_generation: config.codex_code_generation_rating.unwrap_or(0.95),
                complex_reasoning: config.codex_complex_reasoning_rating.unwrap_or(0.85),
                speed: config.codex_speed_rating.unwrap_or(0.85),
                context_length: config.codex_context_length.unwrap_or(200000),
                supports_sessions: config.codex_supports_sessions.unwrap_or(true),
            },
        }
    }

    /// Provider name for logging and error handling
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Request requirements for intelligent provider selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRequirements {
    pub needs_code_analysis: bool,     // Requires strong code analysis capabilities
    pub needs_code_generation: bool,   // Requires strong code generation capabilities
    pub needs_complex_reasoning: bool, // Requires complex reasoning capabilities
    pub needs_speed: bool,             // Prioritize response speed
    pub context_size: u32,             // Estimated context size in tokens
    pub needs_sessions: bool,          // Requires session continuity
}

/// Execution context for AI requests (now focuses on task type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIContext {
    /// Code fixing and optimization - needs strong code analysis
    CodeFix { language: String, content: String },
    /// DSPy optimization and training - needs complex reasoning and sessions
    DSPyOptimization { signature: String, messages: Vec<String> },
    /// Code generation tasks - needs strong code generation
    CodeGeneration { language: String, specification: String },
    /// Code analysis tasks - needs strong analysis capabilities
    CodeAnalysis { language: String, content: String },
    /// General prompt execution - balanced requirements
    General,
}

/// AI execution request with intelligent routing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub session_id: String,
    pub file_path: Option<String>,
    pub context: AIContext,
    pub preferred_providers: Vec<String>,
}

/// AI execution response with standardized format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub provider_used: String,
    pub content: String,
    pub session_id: String,
    pub success: bool,
    pub execution_time_ms: u64,
    pub error_message: Option<String>,
    pub routing_reason: String, // Why this provider was selected
}

/// Intelligent AI provider router
#[derive(Debug)]
pub struct AIRouter {
    providers: Vec<AIProviderConfig>,
}

impl Default for AIRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl AIRouter {
    /// Create new AI router with available providers (Claude, Gemini, GPT5-Codex)
    pub fn new() -> Self {
        Self {
            providers: vec![AIProviderConfig::claude(), AIProviderConfig::google(), AIProviderConfig::openai()],
        }
    }

    /// Intelligently select the best provider for a request
    pub fn select_provider(&self, request: &AIRequest) -> Result<(&AIProviderConfig, String)> {
        let mut ranked = self.rank_providers(request);
        if let Some((score, provider, reason)) = ranked.pop() {
            info!("AI Provider Selected: {} (Score: {:.2}) - {}", provider.name, score, reason);
            Ok((provider, format!("Score: {:.2} - {}", score, reason)))
        } else {
            let dummy_config = AIProviderConfig::claude();
            Err(Error::ai_execution(&dummy_config, "No AI providers available - check API key configuration"))
        }
    }

    fn rank_providers(&self, request: &AIRequest) -> Vec<(f32, &AIProviderConfig, String)> {
        let requirements = self.infer_requirements(request);
        let mut scored_providers: Vec<(f32, &AIProviderConfig, String)> = self
            .providers
            .iter()
            .filter_map(|provider| {
                if provider.requires_api_key {
                    let env_key = match provider.api_key_env.as_ref() {
                        Some(key) if !key.is_empty() => key,
                        _ => return None,
                    };

                    if std::env::var(env_key).is_err() {
                        return None;
                    }
                }

                let (mut score, mut reason) = self.score_provider(provider, &requirements);

                if let Some((position, _)) = request
                    .preferred_providers
                    .iter()
                    .enumerate()
                    .find(|(_, preferred)| **preferred == provider.name)
                {
                    let preference_boost = 1.0 + (request.preferred_providers.len() - position) as f32 * 0.1;
                    score += preference_boost;
                    reason = format!("preferred order {} | {}", position + 1, reason);
                }

                Some((score, provider, reason))
            })
            .collect();

        scored_providers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        scored_providers
    }

    /// Infer request requirements from context
    fn infer_requirements(&self, request: &AIRequest) -> RequestRequirements {
        match &request.context {
            AIContext::CodeFix { content, .. } => RequestRequirements {
                needs_code_analysis: true,
                needs_code_generation: true,
                needs_complex_reasoning: true,
                needs_speed: false,                       // Quality over speed for fixing
                context_size: (content.len() / 4) as u32, // Rough token estimate
                needs_sessions: false,                    // One-shot fixes
            },
            AIContext::DSPyOptimization { .. } => RequestRequirements {
                needs_code_analysis: false,
                needs_code_generation: false,
                needs_complex_reasoning: true,
                needs_speed: false,   // Quality over speed for optimization
                context_size: 50000,  // DSPy typically has large context
                needs_sessions: true, // Multi-turn optimization
            },
            AIContext::CodeGeneration { specification, .. } => RequestRequirements {
                needs_code_analysis: false,
                needs_code_generation: true,
                needs_complex_reasoning: false,
                needs_speed: true, // Fast code generation preferred
                context_size: (specification.len() / 4) as u32,
                needs_sessions: false,
            },
            AIContext::CodeAnalysis { content, .. } => RequestRequirements {
                needs_code_analysis: true,
                needs_code_generation: false,
                needs_complex_reasoning: true,
                needs_speed: false, // Thorough analysis preferred
                context_size: (content.len() / 4) as u32,
                needs_sessions: false,
            },
            AIContext::General => RequestRequirements {
                needs_code_analysis: false,
                needs_code_generation: false,
                needs_complex_reasoning: false,
                needs_speed: true,   // General requests prefer speed
                context_size: 10000, // Modest context for general requests
                needs_sessions: false,
            },
        }
    }

    /// Score a provider based on request requirements
    fn score_provider(&self, provider: &AIProviderConfig, requirements: &RequestRequirements) -> (f32, String) {
        let mut score = 0.0f32;
        let mut reasons = Vec::new();

        // Score based on capability requirements
        if requirements.needs_code_analysis {
            score += provider.capabilities.code_analysis * 0.3;
            reasons.push(format!("code analysis: {:.2}", provider.capabilities.code_analysis));
        }

        if requirements.needs_code_generation {
            score += provider.capabilities.code_generation * 0.3;
            reasons.push(format!("code generation: {:.2}", provider.capabilities.code_generation));
        }

        if requirements.needs_complex_reasoning {
            score += provider.capabilities.complex_reasoning * 0.3;
            reasons.push(format!("reasoning: {:.2}", provider.capabilities.complex_reasoning));
        }

        if requirements.needs_speed {
            score += provider.capabilities.speed * 0.2;
            reasons.push(format!("speed: {:.2}", provider.capabilities.speed));
        }

        // Context length requirements
        if requirements.context_size > provider.capabilities.context_length {
            score *= 0.5; // Heavy penalty for insufficient context
            reasons.push("insufficient context length".to_string());
        }

        // Session requirements
        if requirements.needs_sessions && !provider.capabilities.supports_sessions {
            score *= 0.7; // Penalty for lack of session support
            reasons.push("limited session support".to_string());
        }

        // Base capability score if no specific requirements
        if !requirements.needs_code_analysis && !requirements.needs_code_generation && !requirements.needs_complex_reasoning {
            score = (provider.capabilities.code_analysis + provider.capabilities.code_generation + provider.capabilities.complex_reasoning) / 3.0;
            reasons.push("balanced capabilities".to_string());
        }

        let reason = format!("{} - {}", provider.name, reasons.join(", "));
        (score, reason)
    }

    /// Execute AI request with intelligent provider selection and rate limiting
    pub async fn execute(&self, request: AIRequest) -> Result<AIResponse> {
        // Apply rate limiting before execution
        let default_config = AiLinterConfig::default();
        apply_rate_limiting(&default_config)?;
        let ranked = self.rank_providers(&request);
        if ranked.is_empty() {
            let dummy_config = AIProviderConfig::claude();
            return Err(Error::ai_execution(&dummy_config, "No AI providers available - check API key configuration"));
        }

        let mut errors = Vec::new();

        for (score, provider, reason) in ranked.into_iter().rev() {
            match self.execute_with_provider(provider, &request, score, &reason) {
                Ok(response) => return Ok(response),
                Err(error) => {
                    warn!("AI Provider {} failed: {} - attempting fallback", provider.name, error);
                    errors.push(error);
                    continue;
                }
            }
        }

        Err(Error::Multiple { errors, successful_count: 0 })
    }

    fn execute_with_provider(&self, provider: &AIProviderConfig, request: &AIRequest, score: f32, reason: &str) -> Result<AIResponse> {
        let start_time = std::time::Instant::now();

        let args = self.build_provider_args(request, provider)?;
        let working_dir = self.get_working_directory(request);
        let env = self.build_environment(provider)?;

        let command_input = ExecCommandInput {
            command: provider.command.clone(),
            args,
            env,
            working_dir,
        };

        let output = execute_command(command_input).map_err(|e| Error::ai_execution(provider, format!("AI CLI execution failed: {}", e)))?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        if output.exit_code != 0 {
            info!(
                "AI Provider {} failed with exit code {} in {}ms (score {:.2})",
                provider.name, output.exit_code, execution_time, score
            );

            return Err(Error::ai_execution(
                provider,
                format!("AI CLI failed with exit code {}: {}", output.exit_code, output.stderr),
            ));
        }

        info!(
            "AI Provider {} executed successfully in {}ms (score {:.2})",
            provider.name, execution_time, score
        );

        Ok(AIResponse {
            provider_used: provider.name.clone(),
            content: output.stdout,
            session_id: request.session_id.clone(),
            success: true,
            execution_time_ms: execution_time,
            error_message: None,
            routing_reason: format!("Score: {:.2} - {}", score, reason),
        })
    }

    /// Build provider-specific command arguments
    fn build_provider_args(&self, request: &AIRequest, provider: &AIProviderConfig) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match provider.name.as_str() {
            "claude" => {
                // Claude CLI arguments
                if !request.session_id.is_empty() {
                    args.extend_from_slice(&["--session-id".to_string(), request.session_id.clone()]);
                }

                args.extend_from_slice(&["--model".to_string(), provider.model.clone(), "--no-stream".to_string()]);

                // Add context-specific arguments
                match &request.context {
                    AIContext::CodeFix { .. } => {
                        args.push("code".to_string());
                        if let Some(file_path) = &request.file_path {
                            args.extend_from_slice(&["--file".to_string(), file_path.clone()]);
                        }
                    }
                    AIContext::DSPyOptimization { .. } => {
                        // DSPy-specific Claude arguments
                        args.extend_from_slice(&["--output-format".to_string(), "json".to_string()]);
                    }
                    _ => {
                        // General prompt execution
                    }
                }

                // Add prompt
                args.extend_from_slice(&["--prompt".to_string(), request.prompt.clone()]);
            }

            "google" => {
                // Gemini CLI arguments based on official documentation
                // Use short flags and positional prompts (modern Gemini CLI style)

                // Add model specification
                args.extend_from_slice(&["-m".to_string(), provider.model.clone()]);

                // Add context-specific arguments
                match &request.context {
                    AIContext::CodeFix { .. } | AIContext::CodeAnalysis { .. } => {
                        if let Some(file_path) = &request.file_path {
                            // Use @file syntax for file context injection
                            let context_prompt = format!("@{} {}", file_path, request.prompt);
                            args.push(context_prompt);
                            return Ok(args); // Early return to avoid duplicate prompt
                        }
                    }
                    AIContext::DSPyOptimization { .. } => {
                        // Use sandbox mode for DSPy to ensure isolation
                        args.extend_from_slice(&["-s".to_string()]);
                    }
                    AIContext::CodeGeneration { .. } => {
                        // Use non-interactive mode for code generation
                        args.extend_from_slice(&["-p".to_string()]);
                    }
                    _ => {
                        // General usage - no special flags needed
                    }
                }

                // Add the prompt as positional argument (modern Gemini CLI style)
                args.push(request.prompt.clone());
            }

            "openai" => {
                // OpenAI Codex CLI v0.39.0 - Always read-only like Claude Code

                // Use exec command for non-interactive mode
                args.insert(0, "exec".to_string());

                // Add JSON output for structured parsing
                args.extend_from_slice(&["--json".to_string()]);

                // Always use read-only sandbox (like Claude Code)
                args.extend_from_slice(&["--sandbox".to_string(), "read-only".to_string()]);

                // Add model specification if not default
                if provider.model != "gpt-5-codex" {
                    args.extend_from_slice(&["--model".to_string(), provider.model.clone()]);
                }

                // Add reasoning effort from config
                let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();
                let reasoning_effort = config.codex_reasoning_effort.unwrap_or_else(|| "low".to_string());
                args.extend_from_slice(&["--config".to_string(), format!("model_reasoning_effort=\"{}\"", reasoning_effort)]);

                // Set working directory if file path is provided
                if let Some(file_path) = &request.file_path {
                    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
                        args.extend_from_slice(&["--cd".to_string(), parent_dir.to_string_lossy().to_string()]);
                    }
                }

                // Add the prompt as positional argument (Codex CLI style)
                args.push(request.prompt.clone());
            }

            _ => {
                return Err(Error::ai_execution(provider, format!("Unknown provider: {}", provider.name)));
            }
        }

        Ok(args)
    }

    /// Get standardized working directory for AI execution
    fn get_working_directory(&self, request: &AIRequest) -> Option<String> {
        // Standardized working directory pattern for all providers
        match &request.context {
            AIContext::DSPyOptimization { .. } => Some(format!("/tmp/moon-shine-dspy/{}", request.session_id)),
            AIContext::CodeFix { .. } => Some(format!("/tmp/moon-shine-fixes/{}", request.session_id)),
            AIContext::CodeGeneration { .. } => Some(format!("/tmp/moon-shine-generation/{}", request.session_id)),
            AIContext::CodeAnalysis { .. } => Some(format!("/tmp/moon-shine-analysis/{}", request.session_id)),
            AIContext::General => Some(format!("/tmp/moon-shine-general/{}", request.session_id)),
        }
    }

    /// Build environment variables for AI provider
    fn build_environment(&self, provider: &AIProviderConfig) -> Result<HashMap<String, String>> {
        let mut env = HashMap::new();

        // Add API key from environment
        if let Some(api_key_env) = provider.api_key_env.as_ref() {
            if let Ok(api_key) = std::env::var(api_key_env) {
                env.insert(api_key_env.clone(), api_key);
            } else if provider.requires_api_key {
                return Err(Error::ai_execution(
                    provider,
                    format!("Required API key environment variable '{}' not set", api_key_env),
                ));
            }
        } else if provider.requires_api_key {
            return Err(Error::ai_execution(
                provider,
                "API key environment variable must be configured in moon-shine settings",
            ));
        }

        // Add provider-specific environment variables
        match provider.name.as_str() {
            "claude" => {
                // Claude-specific environment setup
            }
            "google" => {
                // Google-specific environment setup
                if let Ok(creds) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
                    env.insert("GOOGLE_APPLICATION_CREDENTIALS".to_string(), creds);
                }
            }
            "openai" => {
                // Production: OpenAI authentication handling consistent with other providers
                if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                    env.insert("OPENAI_API_KEY".to_string(), api_key);
                    env.insert("OPENAI_BASE_URL".to_string(), "https://api.openai.com/v1".to_string());
                } else {
                    eprintln!("Warning: OpenAI API key not found in environment");
                }
            }
            _ => {
                // Unknown provider - no special environment setup
            }
        }

        Ok(env)
    }
}

/// Rate limiting state for AI provider requests
struct RateLimiterState {
    last_request_time: Option<Instant>,
    request_count: u32,
    minute_start: Instant, // Tracks start of rate limit window
}

impl RateLimiterState {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_request_time: None,
            request_count: 0,
            minute_start: now,
        }
    }
}

/// Global rate limiter for AI provider requests
static RATE_LIMITER: OnceLock<Mutex<RateLimiterState>> = OnceLock::new();

fn rate_limiter_state() -> &'static Mutex<RateLimiterState> {
    RATE_LIMITER.get_or_init(|| Mutex::new(RateLimiterState::new()))
}

/// Apply rate limiting and concurrency controls for AI operations
pub fn apply_rate_limiting(config: &AiLinterConfig) -> Result<()> {
    let limiter = rate_limiter_state();
    let min_delay = Duration::from_millis(config.retry_delay_ms as u64);

    loop {
        let mut state = limiter.lock().expect("Rate limiter mutex poisoned");
        let now = Instant::now();

        // Reset rate limit window if a minute has passed
        if now.duration_since(state.minute_start) >= Duration::from_secs(60) {
            state.minute_start = now;
            state.request_count = 0;
        }

        // Check if we've exceeded the rate limit
        if state.request_count >= config.rate_limit_per_minute {
            return Err(Error::data_access("Rate limit exceeded: too many requests per minute".to_string()));
        }

        // Check if we need to wait for minimum delay between requests
        if let Some(last_request) = state.last_request_time {
            let elapsed = now.duration_since(last_request);
            if elapsed < min_delay {
                let sleep_duration = min_delay - elapsed;
                drop(state); // Release mutex before sleeping
                std::thread::sleep(sleep_duration);
                continue; // Retry after sleeping
            }
        }

        // Update state and allow request
        state.last_request_time = Some(now);
        state.request_count += 1;
        return Ok(());
    }
}

/// Global AI router instance - this is the main interface for AI operations
static AI_ROUTER: OnceLock<AIRouter> = OnceLock::new();

/// Get the global AI router instance
pub fn get_ai_router() -> &'static AIRouter {
    AI_ROUTER.get_or_init(AIRouter::new)
}

/// Convenience functions for common AI operations
/// Execute code fixing with intelligent AI provider selection
pub async fn fix_code_with_ai(session_id: String, file_path: String, content: String, language: String, prompt: String) -> Result<AIResponse> {
    let router = get_ai_router();

    let request = AIRequest {
        prompt,
        session_id,
        file_path: Some(file_path),
        context: AIContext::CodeFix { language, content },
        preferred_providers: Vec::new(),
    };

    router.execute(request).await
}

/// Execute DSPy optimization with intelligent AI provider selection
pub async fn optimize_with_dspy(session_id: String, signature: String, messages: Vec<String>, prompt: String) -> Result<AIResponse> {
    let router = get_ai_router();

    let request = AIRequest {
        prompt,
        session_id,
        file_path: None,
        context: AIContext::DSPyOptimization { signature, messages },
        preferred_providers: Vec::new(),
    };

    router.execute(request).await
}

/// Execute general AI prompt with intelligent provider selection
pub async fn execute_ai_prompt(session_id: String, prompt: String) -> Result<AIResponse> {
    let router = get_ai_router();

    let request = AIRequest {
        prompt,
        session_id,
        file_path: None,
        context: AIContext::General,
        preferred_providers: Vec::new(),
    };

    router.execute(request).await
}

/// Execute code analysis with intelligent AI provider selection
pub async fn analyze_code_with_ai(session_id: String, content: String, language: String, prompt: String) -> Result<AIResponse> {
    let router = get_ai_router();

    let request = AIRequest {
        prompt,
        session_id,
        file_path: None,
        context: AIContext::CodeAnalysis { language, content },
        preferred_providers: Vec::new(),
    };

    router.execute(request).await
}

/// Execute code generation with intelligent AI provider selection
pub async fn generate_code_with_ai(session_id: String, specification: String, language: String, prompt: String) -> Result<AIResponse> {
    let router = get_ai_router();

    let request = AIRequest {
        prompt,
        session_id,
        file_path: None,
        context: AIContext::CodeGeneration { language, specification },
        preferred_providers: Vec::new(),
    };

    router.execute(request).await
}

/// Process files in batches with integrated rate limiting for AI operations
/// This function coordinates batch processing with the AI provider system
pub fn batch_process_files(
    files: &[String],
    config: &AiLinterConfig,
    processor: impl Fn(&[String]) -> std::result::Result<Vec<LintIssue>, Box<dyn std::error::Error>>,
) -> std::result::Result<Vec<LintIssue>, Box<dyn std::error::Error>> {
    let mut all_suggestions = Vec::new();
    let batch_size = config.batch_size as usize;

    for batch in files.chunks(batch_size) {
        // Apply rate limiting before each batch (now integrated in provider router)
        if let Err(e) = apply_rate_limiting(config) {
            return Err(format!("Rate limiting failed: {}", e).into());
        }

        // Process the batch
        let mut batch_suggestions = processor(batch)?;
        all_suggestions.append(&mut batch_suggestions);

        info!("Processed batch of {} files, total suggestions: {}", batch.len(), all_suggestions.len());
    }

    Ok(all_suggestions)
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
    use super::*;

    #[test]
    fn test_provider_capabilities_creation() {
        let capabilities = ProviderCapabilities {
            code_analysis: 0.95,
            code_generation: 0.85,
            complex_reasoning: 0.90,
            speed: 0.75,
            context_length: 200000,
            supports_sessions: true,
        };

        assert!((capabilities.code_analysis - 0.95).abs() < f32::EPSILON);
        assert!((capabilities.code_generation - 0.85).abs() < f32::EPSILON);
        assert!((capabilities.complex_reasoning - 0.90).abs() < f32::EPSILON);
        assert!((capabilities.speed - 0.75).abs() < f32::EPSILON);
        assert_eq!(capabilities.context_length, 200000);
        assert!(capabilities.supports_sessions);
    }

    #[test]
    fn test_ai_provider_config_claude() {
        let claude_config = AIProviderConfig::claude();

        assert_eq!(claude_config.name, "claude");
        assert_eq!(claude_config.command, "claude");
        assert!(claude_config.api_key_env.is_none());
        assert!(!claude_config.requires_api_key);
        assert!(claude_config.model == "sonnet" || claude_config.model == "opus");

        // Test Claude capabilities
        let capabilities = &claude_config.capabilities;
        assert!(capabilities.code_analysis >= 0.9);
        assert!(capabilities.complex_reasoning >= 0.9);
        assert!(capabilities.context_length >= 100000);
        assert!(capabilities.supports_sessions);
    }

    #[test]
    fn test_ai_provider_config_google() {
        let google_config = AIProviderConfig::google();

        assert_eq!(google_config.name, "google");
        assert_eq!(google_config.command, "gemini");
        assert!(google_config.api_key_env.is_none());
        assert!(!google_config.requires_api_key);
        assert!(google_config.model.contains("gemini"));

        // Test Google capabilities
        let capabilities = &google_config.capabilities;
        assert!(capabilities.code_analysis >= 0.8);
        assert!(capabilities.code_generation >= 0.8);
        assert!(capabilities.context_length >= 50000);
        assert!(capabilities.supports_sessions);
    }

    #[test]
    fn test_ai_provider_config_openai() {
        let openai_config = AIProviderConfig::openai();

        assert_eq!(openai_config.name, "openai");
        assert!(openai_config.command.contains("codex"));
        assert!(openai_config.api_key_env.is_none());
        assert!(!openai_config.requires_api_key);
        assert!(openai_config.model.contains("codex"));

        // Test OpenAI Codex capabilities
        let capabilities = &openai_config.capabilities;
        assert!(capabilities.code_generation >= 0.9);
        assert!(capabilities.code_analysis >= 0.8);
        assert!(capabilities.context_length >= 100000);
        assert!(capabilities.supports_sessions);
    }

    #[test]
    fn test_ai_context_variants() {
        let code_fix = AIContext::CodeFix {
            language: "typescript".to_string(),
            content: "const x = 1;".to_string(),
        };
        match code_fix {
            AIContext::CodeFix { language, content } => {
                assert_eq!(language, "typescript");
                assert_eq!(content, "const x = 1;");
            }
            _ => {
                panic!("Expected CodeFix context but got different AIContext variant")
            }
        }

        let dspy_optimization = AIContext::DSPyOptimization {
            signature: "input -> output".to_string(),
            messages: vec!["message1".to_string(), "message2".to_string()],
        };
        match dspy_optimization {
            AIContext::DSPyOptimization { signature, messages } => {
                assert_eq!(signature, "input -> output");
                assert_eq!(messages.len(), 2);
            }
            _ => panic!("Expected DSPyOptimization context but got different AIContext variant"),
        }

        let code_analysis = AIContext::CodeAnalysis {
            language: "python".to_string(),
            content: "def test(): pass".to_string(),
        };
        match code_analysis {
            AIContext::CodeAnalysis { language, content } => {
                assert_eq!(language, "python");
                assert!(content.contains("def test"));
            }
            _ => panic!("Expected CodeAnalysis context but got different AIContext variant"),
        }

        let code_generation = AIContext::CodeGeneration {
            language: "rust".to_string(),
            specification: "Create a function that sorts numbers".to_string(),
        };
        match code_generation {
            AIContext::CodeGeneration { language, specification } => {
                assert_eq!(language, "rust");
                assert!(specification.contains("sorts numbers"));
            }
            _ => panic!("Expected CodeGeneration context but got different AIContext variant"),
        }

        let general = AIContext::General;
        assert!(matches!(general, AIContext::General));
    }

    #[test]
    fn test_ai_request_creation() {
        let request = AIRequest {
            prompt: "Fix this TypeScript code".to_string(),
            session_id: "session-123".to_string(),
            file_path: Some("src/test.ts".to_string()),
            context: AIContext::CodeFix {
                language: "typescript".to_string(),
                content: "const x = 1;".to_string(),
            },
            preferred_providers: vec!["claude".to_string(), "google".to_string()],
        };

        assert_eq!(request.prompt, "Fix this TypeScript code");
        assert_eq!(request.session_id, "session-123");
        assert_eq!(request.file_path, Some("src/test.ts".to_string()));
        assert_eq!(request.preferred_providers.len(), 2);
        assert!(matches!(request.context, AIContext::CodeFix { .. }));
    }

    #[test]
    fn test_ai_response_creation() {
        let response = AIResponse {
            content: "Fixed code: const x: number = 1;".to_string(),
            session_id: "session-123".to_string(),
            provider_used: "claude".to_string(),
            execution_time_ms: 1500,
            success: true,
            error_message: None,
            routing_reason: "Best for code analysis".to_string(),
        };

        assert!(response.content.contains("Fixed code"));
        assert_eq!(response.session_id, "session-123");
        assert_eq!(response.provider_used, "claude");
        assert_eq!(response.execution_time_ms, 1500);
        assert!(response.success);
        assert!(response.error_message.is_none());
    }

    #[test]
    fn test_ai_router_creation() {
        let router = AIRouter::new();

        assert_eq!(router.providers.len(), 3); // Claude, Google, OpenAI
        assert!(router.providers.iter().any(|p| p.name == "claude"));
        assert!(router.providers.iter().any(|p| p.name == "google"));
        assert!(router.providers.iter().any(|p| p.name == "openai"));
    }

    #[test]
    fn test_provider_selection_by_context() {
        let router = AIRouter::new();

        // Test code fix context (should prefer Claude for complex reasoning)
        let code_fix_request = AIRequest {
            prompt: "Fix this bug".to_string(),
            session_id: "session-1".to_string(),
            file_path: None,
            context: AIContext::CodeFix {
                language: "typescript".to_string(),
                content: "const x = 1;".to_string(),
            },
            preferred_providers: vec![],
        };

        let selected = router.select_provider(&code_fix_request);
        // Should select based on capabilities - Claude is good for code analysis

        // Test code generation context (should prefer OpenAI Codex)
        let code_gen_request = AIRequest {
            prompt: "Generate a function".to_string(),
            session_id: "session-2".to_string(),
            file_path: None,
            context: AIContext::CodeGeneration {
                language: "rust".to_string(),
                specification: "Sort function".to_string(),
            },
            preferred_providers: vec![],
        };

        let selected_gen = router.select_provider(&code_gen_request);
        // Should work with provider selection logic
        assert!(selected.is_ok());
        assert!(selected_gen.is_ok());
    }

    #[test]
    fn test_provider_selection_with_preferences() {
        let router = AIRouter::new();

        let request_with_preference = AIRequest {
            prompt: "Analyze this code".to_string(),
            session_id: "session-pref".to_string(),
            file_path: None,
            context: AIContext::CodeAnalysis {
                language: "python".to_string(),
                content: "def test(): pass".to_string(),
            },
            preferred_providers: vec!["google".to_string()],
        };

        let selected = router.select_provider(&request_with_preference);
        assert!(selected.is_ok());
        // Should respect preferred provider if available
    }

    #[test]
    fn test_rate_limiter_state() {
        let state = RateLimiterState::new();

        assert!(state.last_request_time.is_none());
        assert_eq!(state.request_count, 0);
        // minute_start should be recent
        assert!(state.minute_start.elapsed().as_secs() < 1);
    }

    #[test]
    fn test_apply_rate_limiting_success() {
        let config = AiLinterConfig {
            enable_claude_ai: true,
            enable_semantic_checks: true,
            claude_model: "sonnet".to_string(),
            max_processing_time: 600, // 10 minutes for big code files
            quality_threshold: 0.8,
            max_concurrent_requests: 3,
            batch_size: 5,
            rate_limit_per_minute: 60, // High limit for testing
            max_tokens_per_request: 4000,
            retry_attempts: 3,
            retry_delay_ms: 10, // Very short delay for testing
        };

        // Should succeed with reasonable limits
        let result = apply_rate_limiting(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_rate_limiting_rate_limit_exceeded() {
        let config = AiLinterConfig {
            enable_claude_ai: true,
            enable_semantic_checks: true,
            claude_model: "sonnet".to_string(),
            max_processing_time: 600, // 10 minutes for big code files
            quality_threshold: 0.8,
            max_concurrent_requests: 3,
            batch_size: 5,
            rate_limit_per_minute: 0, // Zero limit should cause failure
            max_tokens_per_request: 4000,
            retry_attempts: 3,
            retry_delay_ms: 1000,
        };

        // Should fail with zero rate limit
        let result = apply_rate_limiting(&config);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Rate limit exceeded"));
        }
    }

    #[test]
    fn test_batch_process_files() {
        let files = vec![
            "file1.ts".to_string(),
            "file2.ts".to_string(),
            "file3.ts".to_string(),
            "file4.ts".to_string(),
            "file5.ts".to_string(),
        ];

        let config = AiLinterConfig {
            enable_claude_ai: true,
            enable_semantic_checks: true,
            claude_model: "sonnet".to_string(),
            max_processing_time: 600, // 10 minutes for big code files
            quality_threshold: 0.8,
            max_concurrent_requests: 3,
            batch_size: 2, // Small batch size for testing
            rate_limit_per_minute: 60,
            max_tokens_per_request: 4000,
            retry_attempts: 3,
            retry_delay_ms: 10,
        };

        // Mock processor that returns one suggestion per file
        let processor = |batch: &[String]| -> std::result::Result<Vec<LintIssue>, Box<dyn std::error::Error>> {
            let suggestions: Vec<LintIssue> = batch
                .iter()
                .map(|filename| LintIssue {
                    rule: "test-rule".to_string(),
                    message: format!("Issue in {}", filename),
                    line: 1,
                    column: 1,
                    severity: "warning".to_string(),
                    fix_available: true,
                    ai_confidence: 0.9,
                    pattern_frequency: Some(0.5),
                })
                .collect();
            Ok(suggestions)
        };

        let result = batch_process_files(&files, &config, processor);
        assert!(result.is_ok());

        let suggestions = result.unwrap();
        assert_eq!(suggestions.len(), 5); // One suggestion per file
        assert!(suggestions[0].message.contains("file1.ts"));
        assert!(suggestions[4].message.contains("file5.ts"));
    }

    #[test]
    fn test_batch_process_files_processor_error() {
        let files = vec!["file1.ts".to_string()];
        let config = AiLinterConfig::default();

        // Mock processor that always fails
        let processor = |_batch: &[String]| -> std::result::Result<Vec<LintIssue>, Box<dyn std::error::Error>> { Err("Processor failed".into()) };

        let result = batch_process_files(&files, &config, processor);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Processor failed"));
    }

    #[test]
    fn test_serialization_and_deserialization() {
        let capabilities = ProviderCapabilities {
            code_analysis: 0.95,
            code_generation: 0.85,
            complex_reasoning: 0.90,
            speed: 0.75,
            context_length: 200000,
            supports_sessions: true,
        };

        // Test JSON serialization
        let json = serde_json::to_string(&capabilities).unwrap();
        assert!(json.contains("0.95"));
        assert!(json.contains("200000"));

        // Test JSON deserialization
        let deserialized: ProviderCapabilities = serde_json::from_str(&json).unwrap();
        assert!((deserialized.code_analysis - capabilities.code_analysis).abs() < f32::EPSILON);
        assert_eq!(deserialized.context_length, capabilities.context_length);
        assert_eq!(deserialized.supports_sessions, capabilities.supports_sessions);
    }

    #[test]
    fn test_global_ai_router() {
        let router1 = get_ai_router();
        let router2 = get_ai_router();

        // Should return the same instance (singleton pattern)
        assert_eq!(router1.providers.len(), router2.providers.len());
        assert_eq!(router1.providers.len(), 3); // Claude, Google, OpenAI
    }
}

/// Production: Get API key for specified provider with fallback hierarchy
fn get_provider_api_key(provider: &str, ai_context: &AIContext) -> Option<String> {
    // 1. Check provider-specific environment variables
    let env_var = match provider {
        "anthropic" => "ANTHROPIC_API_KEY",
        "openai" => "OPENAI_API_KEY",
        "google" => "GOOGLE_API_KEY",
        "cohere" => "COHERE_API_KEY",
        "groq" => "GROQ_API_KEY",
        "together" => "TOGETHER_API_KEY",
        "openrouter" => "OPENROUTER_API_KEY",
        "xai" => "XAI_API_KEY",
        _ => return None,
    };

    if let Ok(api_key) = std::env::var(env_var) {
        if !api_key.trim().is_empty() {
            return Some(api_key);
        }
    }

    // 2. Check Moon configuration
    if let Ok(config) = crate::config::MoonShineConfig::from_moon_workspace() {
        if let Some(_providers) = config.ai_providers {
            // ai_providers is Vec<String>, not a map with api_key configs
            // TODO: Update config structure to support provider-specific settings
            // For now, skip this configuration source
        }
    }

    // 3. Check generic fallback environment variables
    if let Ok(generic_key) = std::env::var("AI_API_KEY") {
        if !generic_key.trim().is_empty() {
            return Some(generic_key);
        }
    }

    None
}

// Include comprehensive test suite
// Tests are defined inline above with #[cfg(test)]
