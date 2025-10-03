//! Provides adapters for interfacing with language models.
//!
//! Adapters are responsible for formatting requests to and parsing responses from
//! language models, according to the structure defined by a `Signature`.

pub mod chat;

pub use chat::*;

use crate::data::{Example, Prediction};
use crate::dspy::{Chat, Message, MetaSignature, LM};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

/// A trait for adapting signatures to language model interactions.
///
/// Adapters handle the conversion of structured `Example` data into a format
/// suitable for a language model (like a chat history) and parse the model's
/// response back into a structured format.
#[async_trait]
pub trait Adapter: Send + Sync + 'static {
    /// Formats an `Example` into a `Chat` history according to the given `Signature`.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature defining the structure of the inputs and outputs.
    /// * `inputs` - The example data to be formatted.
    ///
    /// # Returns
    ///
    /// A `Chat` object representing the formatted request.
    fn format(&self, signature: &dyn MetaSignature, inputs: Example) -> Chat;

    /// Parses a language model's `Message` response into a structured `HashMap`.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature that guided the request.
    /// * `response` - The message from the language model.
    ///
    /// # Returns
    ///
    /// A `HashMap` where keys are field names from the signature and values are the parsed data.
    fn parse_response(
        &self,
        signature: &dyn MetaSignature,
        response: Message,
    ) -> HashMap<String, Value>;

    /// Executes a call to the language model.
    ///
    /// This method orchestrates the formatting of the request, sending it to the
    /// language model, and parsing the response.
    ///
    /// # Arguments
    ///
    /// * `lm` - The language model to be called.
    /// * `signature` - The signature for the interaction.
    /// * `inputs` - The input data for the call.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Prediction` on success, or an error.
    async fn call(
        &self,
        lm: &mut LM,
        signature: &dyn MetaSignature,
        inputs: Example,
    ) -> Result<Prediction>;
}
