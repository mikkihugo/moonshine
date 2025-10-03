//! # DSPy Language Model (LM) Integration: Direct AI Provider Access
//!
//! This module provides the `DirectAILM` struct, which serves as a direct interface
//! for DSPy to interact with various AI language models. Unlike traditional LM wrappers,
//! `DirectAILM` routes all AI requests through `moon-shine`'s unified AI provider system,
//! allowing for intelligent model selection, rate limiting, and consistent error handling.
//!
//! It maintains a history of interactions for debugging and analysis, and includes
//! utilities for converting DSPy chat messages into a format suitable for AI prompts.
//!
//! @category dspy-lm
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 1.0.0

// Removed ai_cli - using unified provider_router instead

// Import shared components from main src
pub use crate::{
    config::MoonShineConfig,
    message_types::{Chat, Message},
    provider_router::{execute_ai_prompt, AIContext},
    token_usage::LmUsage,
};

use anyhow::Result;
use bon::Builder;
use secrecy::SecretString;

/// Represents a direct AI Language Model (LM) for DSPy, routing requests through `moon-shine`'s AI provider.
///
/// `DirectAILM` acts as DSPy's primary interface to AI models, abstracting away the complexities
/// of provider selection and communication. It maintains a history of all LM interactions.
///
/// @category dspy-struct
/// @safe team
/// @mvp core
/// @complexity medium
/// @since 1.0.0
#[derive(Clone, Debug)]
pub struct DirectAILM {
    /// A unique session identifier for the LM instance.
    pub session_id: String,
    /// The `MoonShineConfig` used by this LM instance.
    pub config: MoonShineConfig,
    /// A history of all interactions with the LM.
    pub history: Vec<LMResponse>,
}

impl DirectAILM {
    /// Creates a new `DirectAILM` instance.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier for this LM.
    /// * `config` - The `MoonShineConfig` to use.
    ///
    /// # Returns
    ///
    /// A new `DirectAILM` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn new(session_id: String, config: MoonShineConfig) -> Self {
        Self {
            session_id,
            config,
            history: Vec::new(),
        }
    }

    /// Makes a call to the AI provider, processing a `Chat` and returning a `Message` and `LmUsage`.
    ///
    /// This asynchronous method converts the DSPy `Chat` messages into a single prompt string,
    /// sends it to the `moon-shine` AI provider, and then parses the response back into a `Message`
    /// and tracks token usage.
    ///
    /// # Arguments
    ///
    /// * `messages` - The `Chat` object containing the conversation history and prompt.
    /// * `signature` - A string representing the signature or task for the AI (used in prompt formatting).
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple of `(Message, LmUsage)` on success, or an `Error` on failure.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity medium
    /// @since 1.0.0
    pub async fn call(&mut self, messages: Chat, signature: &str) -> Result<(Message, LmUsage)> {
        // Convert DSPy chat to prompt for AI provider
        let prompt = self.convert_chat_to_prompt(&messages, signature);

        // Use existing execute_ai_prompt function
        let response = execute_ai_prompt(self.session_id.clone(), prompt.clone())
            .await
            .map_err(|e| anyhow::anyhow!("AI provider error: {}", e))?;

        let message = Message::Assistant {
            content: response.content.clone(),
        };

        let usage = LmUsage {
            input_tokens: (prompt.len() / 4) as u32,
            output_tokens: (response.content.len() / 4) as u32,
            total_tokens: 0,
            reasoning_tokens: None,
            provider_used: Some(response.provider_used.clone()),
            execution_time_ms: Some(response.execution_time_ms),
        };

        // Record in history
        self.history.push(LMResponse {
            chat: messages,
            output: message.clone(),
            config: self.config.clone(),
            signature: signature.to_string(),
        });

        Ok((message, usage))
    }

    /// Converts a `Chat` object into a single prompt string for the AI provider.
    ///
    /// This function concatenates system, user, and assistant messages from the chat history
    /// into a format suitable for sending to the AI model, optionally including a task signature.
    ///
    /// # Arguments
    ///
    /// * `chat` - The `Chat` object to convert.
    /// * `signature` - The task signature string.
    ///
    /// # Returns
    ///
    /// The formatted prompt string.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    fn convert_chat_to_prompt(&self, chat: &Chat, signature: &str) -> String {
        let mut prompt_parts = Vec::new();

        if !signature.is_empty() {
            prompt_parts.push(format!("Task: {}", signature));
        }

        for message in &chat.messages {
            match message {
                Message::System { content } => prompt_parts.push(format!("System: {}", content)),
                Message::User { content } => prompt_parts.push(format!("User: {}", content)),
                Message::Assistant { content } => prompt_parts.push(format!("Assistant: {}", content)),
            }
        }

        prompt_parts.join("\n\n")
    }

    /// Inspects the LM's interaction history.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of most recent interactions to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of references to `LMResponse` objects from the history.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn inspect_history(&self, n: usize) -> Vec<&LMResponse> {
        self.history.iter().rev().take(n).collect()
    }
}

/// Represents a single response from the Language Model, including the chat history and configuration.
///
/// @category dspy-struct
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Clone, Debug)]
pub struct LMResponse {
    /// The `Chat` object representing the conversation history for this response.
    pub chat: Chat,
    /// The `MoonShineConfig` used during this LM interaction.
    pub config: MoonShineConfig,
    /// The `Message` output generated by the LM.
    pub output: Message,
    /// The signature or task string used for this LM interaction.
    pub signature: String,
}

/// Returns the base URL for a given AI provider.
///
/// This function maps common AI provider names to their respective API base URLs.
///
/// # Arguments
///
/// * `provider` - The name of the AI provider.
///
/// # Returns
///
/// The base URL as a `String`.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_base_url(provider: &str) -> String {
    match provider {
        "openai" => "https://api.openai.com/v1".to_string(),
        "anthropic" => "https://api.anthropic.com/v1".to_string(),
        "google" => "https://generativelanguage.googleapis.com/v1beta/openai".to_string(),
        "cohere" => "https://api.cohere.ai/compatibility/v1".to_string(),
        "groq" => "https://api.groq.com/openai/v1".to_string(),
        "openrouter" => "https://openrouter.ai/api/v1".to_string(),
        "qwen" => "https://dashscope-intl.aliyuncs.com/compatible-mode/v1".to_string(),
        "together" => "https://api.together.xyz/v1".to_string(),
        "xai" => "https://api.x.ai/v1".to_string(),
        _ => "https://openrouter.ai/api/v1".to_string(),
    }
}

/// A type alias for `DirectAILM`, representing the primary Language Model type in DSPy.
///
/// @category type-alias
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub type LM = DirectAILM;

/// A legacy type alias for `DirectAILM`, previously used for a Claude-specific LM.
///
/// This alias is maintained for backward compatibility. New code should use `DirectAILM`.
///
/// @deprecated Use `DirectAILM` instead.
/// @category type-alias
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub type ClaudeLM = DirectAILM;

/// A dummy Language Model implementation for testing and development.
///
/// This struct provides a mock LM that can be used to simulate AI responses
/// without making actual API calls. It's useful for unit testing and rapid prototyping.
///
/// @category dspy-struct
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Clone, Builder, Default)]
pub struct DummyLM {
    /// The API key for the dummy LM.
    pub api_key: SecretString,
    /// The base URL for the dummy LM's API.
    #[builder(default = "https://api.openai.com/v1".to_string())]
    pub base_url: String,
    /// The configuration for the dummy LM.
    #[builder(default = MoonShineConfig::default())]
    pub config: MoonShineConfig,
    /// A history of interactions with the dummy LM.
    #[builder(default = Vec::new())]
    pub history: Vec<LMResponse>,
}

impl DummyLM {
    /// Simulates an AI call, returning a predefined prediction.
    ///
    /// This method records the interaction in the history and returns a dummy response.
    ///
    /// # Arguments
    ///
    /// * `messages` - The `Chat` object representing the conversation.
    /// * `signature` - The task signature.
    /// * `prediction` - The predefined prediction string to return.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple of `(Message, LmUsage)`.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub async fn call(
        &mut self,
        messages: Chat,
        signature: &str,
        prediction: String,
    ) -> Result<(Message, LmUsage)> {
        self.history.push(LMResponse {
            chat: messages.clone(),
            output: Message::Assistant {
                content: prediction.clone(),
            },
            config: self.config.clone(),
            signature: signature.to_string(),
        });

        Ok((
            Message::Assistant {
                content: prediction.clone(),
            },
            LmUsage::default(),
        ))
    }

    /// Inspects the dummy LM's interaction history.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of most recent interactions to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of references to `LMResponse` objects from the history.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn inspect_history(&self, n: usize) -> Vec<&LMResponse> {
        self.history.iter().rev().take(n).collect()
    }
}
