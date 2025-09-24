pub mod chat;

pub use chat::*;

use crate::data::{Example, Prediction};
use crate::dspy::{ConversationHistory, Message, MetaSignature, LM};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

#[async_trait]
pub trait Adapter: Send + Sync + 'static {
    fn format(&self, signature: &dyn MetaSignature, inputs: Example) -> ConversationHistory;
    fn parse_response(&self, signature: &dyn MetaSignature, response: Message) -> HashMap<String, Value>;
    async fn call(&self, lm: &mut LM, signature: &dyn MetaSignature, inputs: Example) -> Result<Prediction>;
}
