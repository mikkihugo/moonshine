//! # Message Types: Core Communication Structures for DSPy
//!
//! This module defines the fundamental data structures used for representing conversational
//! turns and chat histories within the DSPy framework. These types are crucial for building
//! and interacting with AI models, enabling structured communication between the DSPy core
//! and various language model backends.
//!
//! Key components include:
//! - `Message`: An enum representing a single conversational message with different roles (system, user, assistant).
//! - `Chat`: A struct encapsulating a sequence of `Message`s, forming a complete conversation history.
//!
//! @category communication
//! @safe team
//! @mvp core
//! @complexity low
//! @since 1.0.0

use anyhow::Result;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents a single message in a conversation, categorized by its role.
///
/// This enum is used to distinguish between instructions from the system, input from the user,
/// and responses from the AI assistant.
///
/// @category data-model
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    /// A system message, typically providing instructions or context to the AI.
    System { content: String },
    /// A user message, representing input or queries from the user.
    User { content: String },
    /// An assistant message, representing responses or generated content from the AI.
    Assistant { content: String },
}

impl Message {
    /// Creates a new `Message` based on the specified role and content.
    ///
    /// If an invalid role is provided, it defaults to a `User` message with an informative error.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message ("system", "user", "assistant").
    /// * `content` - The textual content of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn new(role: &str, content: &str) -> Self {
        match role {
            "system" => Message::system(content),
            "user" => Message::user(content),
            "assistant" => Message::assistant(content),
            _ => Message::user(format!("Invalid role '{}', defaulting to user: {}", role, content)),
        }
    }

    /// Creates a new `User` message.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the user message.
    ///
    /// # Returns
    ///
    /// A new `Message::User` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn user(content: impl Into<String>) -> Self {
        Message::User { content: content.into() }
    }

    /// Creates a new `Assistant` message.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the assistant message.
    ///
    /// # Returns
    ///
    /// A new `Message::Assistant` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn assistant(content: impl Into<String>) -> Self {
        Message::Assistant { content: content.into() }
    }

    /// Creates a new `System` message.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the system message.
    ///
    /// # Returns
    ///
    /// A new `Message::System` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn system(content: impl Into<String>) -> Self {
        Message::System { content: content.into() }
    }

    /// Returns the textual content of the message.
    ///
    /// # Returns
    ///
    /// The message content as a `String`.
    ///
    /// @category getter
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn content(&self) -> String {
        match self {
            Message::System { content } => content.clone(),
            Message::User { content } => content.clone(),
            Message::Assistant { content } => content.clone(),
        }
    }

    /// Converts the `Message` into a `serde_json::Value` representation.
    ///
    /// This is typically used for serialization when communicating with external systems or APIs.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the message.
    ///
    /// @category serialization
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn to_json(&self) -> Value {
        match self {
            Message::System { content } => {
                json!({ "role": "system", "content": content })
            }
            Message::User { content } => {
                json!({ "role": "user", "content": content })
            }
            Message::Assistant { content } => {
                json!({ "role": "assistant", "content": content })
            }
        }
    }
}

/// Represents a conversation history, composed of a sequence of `Message`s.
///
/// The `Chat` struct provides methods for managing the conversation flow,
/// including adding, removing, and converting messages to and from JSON formats.
///
/// @category data-model
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
    /// The ordered list of messages in the conversation.
    pub messages: Vec<Message>,
}

impl Chat {
    /// Creates a new `Chat` instance with an initial list of messages.
    ///
    /// # Arguments
    ///
    /// * `messages` - A `Vec<Message>` to initialize the chat history.
    ///
    /// # Returns
    ///
    /// A new `Chat` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    /// Returns the number of messages in the chat history.
    ///
    /// # Returns
    ///
    /// The length of the message vector as a `usize`.
    ///
    /// @category getter
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Checks if the chat history is empty.
    ///
    /// # Returns
    ///
    /// `true` if there are no messages, `false` otherwise.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Adds a new message to the end of the chat history.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message ("system", "user", "assistant").
    /// * `content` - The textual content of the message.
    ///
    /// @category mutator
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn push(&mut self, role: &str, content: &str) {
        self.messages.push(Message::new(role, content));
    }

    /// Appends all messages from another `Chat` instance to the current chat history.
    ///
    /// # Arguments
    ///
    /// * `chat` - A reference to the `Chat` instance whose messages are to be appended.
    ///
    /// @category mutator
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn push_all(&mut self, chat: &Chat) {
        self.messages.extend(chat.messages.clone());
    }

    /// Removes and returns the last message from the chat history.
    ///
    /// # Returns
    ///
    /// An `Option` containing the last `Message` if the chat is not empty, otherwise `None`.
    ///
    /// @category mutator
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    /// Converts a `serde_json::Value` (expected to be a JSON array of messages) into a `Chat` instance.
    ///
    /// This function is used for deserializing chat histories from JSON formats.
    ///
    /// # Arguments
    ///
    /// * `json_dump` - The `serde_json::Value` representing the JSON array of messages.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Chat` instance on success, or an `Error` on failure.
    ///
    /// @category deserialization
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn from_json(&self, json_dump: Value) -> Result<Self> {
        let messages = json_dump.as_array().ok_or_else(|| anyhow::anyhow!("JSON dump should be an array"))?;
        let messages = messages
            .iter()
            .filter_map(|message| {
                let role = message["role"].as_str()?;
                let content = message["content"].as_str()?;
                Some(Message::new(role, content))
            })
            .collect();
        Ok(Self { messages })
    }

    /// Converts the `Chat` instance into a `serde_json::Value` representation.
    ///
    /// This is typically used for serialization when storing or transmitting chat histories.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the chat history as a JSON array.
    ///
    /// @category serialization
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn to_json(&self) -> Value {
        let messages = self.messages.iter().map(|message| message.to_json()).collect::<Vec<Value>>();
        json!(messages)
    }
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_message_creation() {
        // Test system message
        let system_msg = Message::System {
            content: "You are an expert TypeScript analyzer.".to_string(),
        };

        match system_msg {
            Message::System { content } => {
                assert_eq!(content, "You are an expert TypeScript analyzer.");
            }
            _ => {
                panic!("Expected System message but got different message type");
            }
        }

        // Test user message
        let user_msg = Message::User {
            content: "Analyze this function: function test() {}".to_string(),
        };

        match user_msg {
            Message::User { content } => {
                assert!(content.contains("function test"));
            }
            _ => {
                panic!("Expected User message but got different message type");
            }
        }

        // Test assistant message
        let assistant_msg = Message::Assistant {
            content: "This function is well-formed but could benefit from type annotations.".to_string(),
        };

        match assistant_msg {
            Message::Assistant { content } => {
                assert!(content.contains("type annotations"));
            }
            _ => {
                panic!("Expected Assistant message but got different message type");
            }
        }
    }

    #[test]
    fn test_message_constructors() {
        let system_msg = Message::system("System prompt");
        assert_eq!(system_msg.content(), "System prompt");

        let user_msg = Message::user("User input");
        assert_eq!(user_msg.content(), "User input");

        let assistant_msg = Message::assistant("Assistant response");
        assert_eq!(assistant_msg.content(), "Assistant response");

        // Test new() with valid roles
        let system_new = Message::new("system", "System test");
        assert!(matches!(system_new, Message::System { .. }));

        let user_new = Message::new("user", "User test");
        assert!(matches!(user_new, Message::User { .. }));

        let assistant_new = Message::new("assistant", "Assistant test");
        assert!(matches!(assistant_new, Message::Assistant { .. }));

        // Test new() with invalid role
        let invalid_new = Message::new("invalid", "Test content");
        assert!(matches!(invalid_new, Message::User { .. }));
        assert!(invalid_new.content().contains("Invalid role"));
    }

    #[test]
    fn test_message_json_conversion() {
        let system_msg = Message::system("System prompt for AI");
        let json = system_msg.to_json();
        assert_eq!(json["role"], "system");
        assert_eq!(json["content"], "System prompt for AI");

        let user_msg = Message::user("User input for analysis");
        let json = user_msg.to_json();
        assert_eq!(json["role"], "user");
        assert_eq!(json["content"], "User input for analysis");

        let assistant_msg = Message::assistant("AI response with suggestions");
        let json = assistant_msg.to_json();
        assert_eq!(json["role"], "assistant");
        assert_eq!(json["content"], "AI response with suggestions");
    }

    #[test]
    fn test_chat_creation_and_basic_operations() {
        let chat = Chat::new(vec![]);
        assert!(chat.is_empty());
        assert_eq!(chat.len(), 0);

        // Test with initial messages
        let initial_messages = vec![Message::system("You are a helpful assistant"), Message::user("Hello")];
        let chat_with_messages = Chat::new(initial_messages);
        assert_eq!(chat_with_messages.len(), 2);
        assert!(!chat_with_messages.is_empty());
    }

    #[test]
    fn test_chat_push_operations() {
        let mut chat = Chat::new(vec![]);

        // Test push with different roles
        chat.push("system", "You are a helpful TypeScript assistant.");
        assert_eq!(chat.len(), 1);

        chat.push("user", "Fix this code: let x;");
        assert_eq!(chat.len(), 2);

        chat.push("assistant", "Consider: let x: unknown;");
        assert_eq!(chat.len(), 3);

        // Verify message content
        assert!(chat.messages[0].content().contains("TypeScript assistant"));
        assert!(chat.messages[1].content().contains("let x;"));
        assert!(chat.messages[2].content().contains("unknown"));
    }

    #[test]
    fn test_chat_push_all() {
        let mut chat1 = Chat::new(vec![Message::system("System prompt"), Message::user("User message")]);

        let chat2 = Chat::new(vec![Message::assistant("Assistant response"), Message::user("Another user message")]);

        assert_eq!(chat1.len(), 2);
        chat1.push_all(&chat2);
        assert_eq!(chat1.len(), 4);

        // Verify all messages are present
        assert!(chat1.messages[0].content().contains("System prompt"));
        assert!(chat1.messages[1].content().contains("User message"));
        assert!(chat1.messages[2].content().contains("Assistant response"));
        assert!(chat1.messages[3].content().contains("Another user message"));
    }

    #[test]
    fn test_chat_pop() {
        let mut chat = Chat::new(vec![
            Message::system("System prompt"),
            Message::user("User message"),
            Message::assistant("Assistant response"),
        ]);

        assert_eq!(chat.len(), 3);

        // Pop last message
        let popped = chat.pop();
        assert!(popped.is_some());
        assert!(popped.unwrap().content().contains("Assistant response"));
        assert_eq!(chat.len(), 2);

        // Pop another message
        let popped = chat.pop();
        assert!(popped.is_some());
        assert!(popped.unwrap().content().contains("User message"));
        assert_eq!(chat.len(), 1);

        // Pop last message
        let popped = chat.pop();
        assert!(popped.is_some());
        assert!(popped.unwrap().content().contains("System prompt"));
        assert_eq!(chat.len(), 0);
        assert!(chat.is_empty());

        // Pop from empty chat
        let popped = chat.pop();
        assert!(popped.is_none());
    }

    #[test]
    fn test_chat_to_json() {
        let chat = Chat::new(vec![
            Message::system("System prompt"),
            Message::user("User input"),
            Message::assistant("Assistant response"),
        ]);

        let json = chat.to_json();
        let array = json.as_array().unwrap();
        assert_eq!(array.len(), 3);

        assert_eq!(array[0]["role"], "system");
        assert_eq!(array[0]["content"], "System prompt");

        assert_eq!(array[1]["role"], "user");
        assert_eq!(array[1]["content"], "User input");

        assert_eq!(array[2]["role"], "assistant");
        assert_eq!(array[2]["content"], "Assistant response");
    }

    #[test]
    fn test_chat_from_json() {
        let json_data = json!([
          {"role": "system", "content": "System prompt"},
          {"role": "user", "content": "User input"},
          {"role": "assistant", "content": "Assistant response"},
          {"role": "invalid", "content": "Invalid role message"}
        ]);

        let chat = Chat::new(vec![]);
        let result = chat.from_json(json_data);
        assert!(result.is_ok());

        let parsed_chat = result.unwrap();
        assert_eq!(parsed_chat.len(), 4);

        // Verify message types
        assert!(matches!(parsed_chat.messages[0], Message::System { .. }));
        assert!(matches!(parsed_chat.messages[1], Message::User { .. }));
        assert!(matches!(parsed_chat.messages[2], Message::Assistant { .. }));
        assert!(matches!(parsed_chat.messages[3], Message::User { .. })); // Invalid role becomes User

        // Verify content
        assert_eq!(parsed_chat.messages[0].content(), "System prompt");
        assert_eq!(parsed_chat.messages[1].content(), "User input");
        assert_eq!(parsed_chat.messages[2].content(), "Assistant response");
        assert!(parsed_chat.messages[3].content().contains("Invalid role"));
    }

    #[test]
    fn test_chat_json_roundtrip() {
        let original_chat = Chat::new(vec![
            Message::system("System prompt"),
            Message::user("User input"),
            Message::assistant("Assistant response"),
        ]);

        // Convert to JSON and back
        let json = original_chat.to_json();
        let parsed_chat = original_chat.from_json(json).unwrap();

        // Verify roundtrip integrity
        assert_eq!(original_chat.len(), parsed_chat.len());
        for (original, parsed) in original_chat.messages.iter().zip(parsed_chat.messages.iter()) {
            assert_eq!(original.content(), parsed.content());
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test empty content
        let empty_msg = Message::user("");
        assert_eq!(empty_msg.content(), "");

        // Test very long content
        let long_content = "a".repeat(10000);
        let long_msg = Message::assistant(&long_content);
        assert_eq!(long_msg.content().len(), 10000);

        // Test special characters
        let special_msg = Message::system("Special chars: \n\t\r\"'\\");
        assert!(special_msg.content().contains("Special chars"));

        // Test JSON with empty content
        let empty_json = json!([
          {"role": "user", "content": ""}
        ]);
        let chat = Chat::new(vec![]);
        let result = chat.from_json(empty_json).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.messages[0].content(), "");
    }
}
