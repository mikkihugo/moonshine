/// The chat adapter, which is responsible for formatting requests and parsing responses for chat-based language models.
pub mod chat;

pub use chat::*;

use crate::data::{Example, Prediction};
use crate::dspy::{Chat, Message, MetaSignature, LM};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

/// A trait for adapting requests and responses for different language models.
///
/// Adapters are responsible for formatting the input `Example` into a `Chat` object
/// that can be sent to a language model, and for parsing the `Message` response
/// from the language model into a `HashMap` of output values.
#[async_trait]
pub trait Adapter: Send + Sync + 'static {
    /// Formats the input `Example` into a `Chat` object.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature of the DSPy program.
    /// * `inputs` - The input example to format.
    ///
    /// # Returns
    ///
    /// A `Chat` object that can be sent to a language model.
    fn format(&self, signature: &dyn MetaSignature, inputs: Example) -> Chat;

    /// Parses the `Message` response from the language model into a `HashMap` of output values.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature of the DSPy program.
    /// * `response` - The message response from the language model.
    ///
    /// # Returns
    ///
    /// A `HashMap` where the keys are the output field names and the values are the parsed output values.
    fn parse_response(&self, signature: &dyn MetaSignature, response: Message) -> HashMap<String, Value>;

    /// Calls the language model with the formatted request and returns the parsed response.
    ///
    /// # Arguments
    ///
    /// * `lm` - The language model to call.
    /// * `signature` - The signature of the DSPy program.
    /// * `inputs` - The input example.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Prediction` on success, or an `Error` on failure.
    async fn call(&self, lm: &mut LM, signature: &dyn MetaSignature, inputs: Example) -> Result<Prediction>;
}
