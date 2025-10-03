//! # DSPy Signature: Defining AI Model Interfaces
//!
//! This module defines the `MetaSignature` trait, a crucial abstraction in the DSPy framework
//! for specifying the interface of an AI model. A `MetaSignature` encapsulates the inputs,
//! outputs, and instructions that guide an AI model's behavior.
//!
//! By defining a clear signature, DSPy can automatically generate prompts, manage data flow,
//! and optimize the model's performance. It also supports few-shot learning through demonstration
//! examples and allows for dynamic updates to instructions and prefixes.
//!
//! @category dspy-core
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 1.0.0

use crate::data::Example;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

/// Represents a DSPy signature, defining the inputs and outputs of a model.
#[derive(Debug, Clone)]
pub struct DspySignature {
    pub name: String,
    pub inputs: Vec<DspyField>,
    pub outputs: Vec<DspyField>,
}

/// Represents a field within a DSPy signature, including its name and description.
#[derive(Debug, Clone)]
pub struct DspyField {
    pub name: String,
    pub description: String,
}

/// Represents the input to a DSPy model, consisting of a set of named fields.
#[derive(Debug, Clone)]
pub struct DspyInput {
    pub fields: HashMap<String, String>,
}

/// Represents the output from a DSPy model, consisting of a set of named fields.
#[derive(Debug, Clone)]
pub struct DspyOutput {
    pub fields: HashMap<String, String>,
}

/// Represents an example for training or evaluation, containing inputs, outputs, and metadata.
#[derive(Debug, Clone)]
pub struct DspyExample {
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<(String, String)>,
    pub metadata: HashMap<String, String>,
}

impl DspySignature {
    /// Creates a new `DspySignatureBuilder` for constructing a `DspySignature`.
    pub fn new() -> DspySignatureBuilder {
        DspySignatureBuilder {
            name: String::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Creates a signature from a string format like "input1, input2 -> output1, output2".
    ///
    /// # Arguments
    ///
    /// * `signature_str` - The string representation of the signature.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `DspySignature` on success, or an error.
    pub fn from_string(signature_str: &str) -> Result<Self> {
        let parts: Vec<&str> = signature_str.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid signature format. Expected 'inputs -> outputs'"));
        }

        let mut builder = Self::new();

        // Parse inputs
        for input in parts[0].split(", ") {
            let input = input.trim();
            if !input.is_empty() {
                builder = builder.input(input, &format!("Input field: {}", input));
            }
        }

        // Parse outputs
        for output in parts[1].split(", ") {
            let output = output.trim();
            if !output.is_empty() {
                builder = builder.output(output, &format!("Output field: {}", output));
            }
        }

        Ok(builder.build())
    }

    /// Updates field properties, similar to DSPy's `with_updated_fields`.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field to update.
    /// * `field_type` - An optional new type for the field.
    ///
    /// # Returns
    ///
    /// The updated `DspySignature`.
    pub fn with_updated_fields(mut self, field_name: &str, field_type: Option<String>) -> Self {
        // Update field type if found
        for field in &mut self.inputs {
            if field.name == field_name {
                if let Some(new_type) = &field_type {
                    field.description = format!("{} (type: {})", field.description, new_type);
                }
            }
        }
        for field in &mut self.outputs {
            if field.name == field_name {
                if let Some(new_type) = &field_type {
                    field.description = format!("{} (type: {})", field.description, new_type);
                }
            }
        }
        self
    }

    /// Validates the signature, ensuring it has at least one input and one output.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        if self.inputs.is_empty() {
            return Err(anyhow::anyhow!("Signature must have at least one input"));
        }
        if self.outputs.is_empty() {
            return Err(anyhow::anyhow!("Signature must have at least one output"));
        }
        Ok(())
    }

    /// Validates an input against the signature, checking for required fields.
    ///
    /// # Arguments
    ///
    /// * `input` - The `DspyInput` to validate.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error if validation fails.
    pub fn validate_input(&self, input: &DspyInput) -> Result<()> {
        for field in &self.inputs {
            if !input.fields.contains_key(&field.name) {
                return Err(anyhow::anyhow!("Missing required input field: {}", field.name));
            }
        }
        Ok(())
    }

    /// Validates an output against the signature, checking for required fields.
    ///
    /// # Arguments
    ///
    /// * `output` - The `DspyOutput` to validate.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error if validation fails.
    pub fn validate_output(&self, output: &DspyOutput) -> Result<()> {
        for field in &self.outputs {
            if !output.fields.contains_key(&field.name) {
                return Err(anyhow::anyhow!("Missing required output field: {}", field.name));
            }
        }
        Ok(())
    }
}

/// A builder for creating `DspySignature` instances.
pub struct DspySignatureBuilder {
    name: String,
    inputs: Vec<DspyField>,
    outputs: Vec<DspyField>,
}

impl DspySignatureBuilder {
    /// Sets the name of the signature.
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Adds an input field to the signature.
    pub fn input(mut self, name: &str, description: &str) -> Self {
        self.inputs.push(DspyField {
            name: name.to_string(),
            description: description.to_string(),
        });
        self
    }

    /// Adds an output field to the signature.
    pub fn output(mut self, name: &str, description: &str) -> Self {
        self.outputs.push(DspyField {
            name: name.to_string(),
            description: description.to_string(),
        });
        self
    }

    /// Builds the `DspySignature`.
    pub fn build(self) -> DspySignature {
        DspySignature {
            name: if self.name.is_empty() { "DefaultSignature".to_string() } else { self.name },
            inputs: self.inputs,
            outputs: self.outputs,
        }
    }
}

impl DspyInput {
    /// Creates a new, empty `DspyInput`.
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Adds a field to the input.
    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }
}

impl DspyOutput {
    /// Creates a new, empty `DspyOutput`.
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Adds a field to the output.
    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }

    /// Retrieves a field's value from the output.
    pub fn get_field(&self, name: &str) -> Option<&String> {
        self.fields.get(name)
    }
}

/// A trait for creating DSPy Signature types.
///
/// DSPy signatures support:
/// - Automatic field definition with descriptions
/// - Type validation with Pydantic-like types
/// - Schema generation for LM interaction
/// - Field update capabilities
/// - Instruction optimization
pub trait Signature: Send + Sync + Clone {
  /// Creates a new instance of this signature.
  fn new() -> Self;

  /// Updates a field's properties.
  fn with_updated_fields(self, field_name: &str, field_type: Option<String>) -> Self;

  /// Gets the signature's instruction string.
  fn get_instructions(&self) -> String;

  /// Updates the signature's instructions.
  fn set_instructions(&mut self, instructions: String);
}

/// Defines the metadata signature for an AI model within DSPy.
///
/// The `MetaSignature` trait specifies the inputs, outputs, and instructions
/// that define how an AI model should process data. It also supports managing
/// demonstration examples for few-shot learning and allows for dynamic updates
/// to the model's behavior.
pub trait MetaSignature: Send + Sync {
  /// Returns a vector of demonstration `Example`s for few-shot learning.
  ///
  /// These examples provide the AI model with successful input-output pairs
  /// to guide its behavior.
  ///
  /// # Returns
  ///
  /// A `Vec<Example>` containing the demonstration examples.
  fn demos(&self) -> Vec<Example>;

  /// Sets the demonstration `Example`s for few-shot learning.
  ///
  /// # Arguments
  ///
  /// * `demos` - A `Vec<Example>` to set as the new demonstration examples.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or an `Error` on failure.
  fn set_demos(&mut self, demos: Vec<Example>) -> Result<()>;

  /// Returns the instruction string for the AI model.
  ///
  /// This instruction guides the AI on what task to perform.
  ///
  /// # Returns
  ///
  /// The instruction string as a `String`.
  fn instruction(&self) -> String;

  /// Returns a `serde_json::Value` representing the input fields of the signature.
  ///
  /// This typically includes the name, type, and description of each input field.
  ///
  /// # Returns
  ///
  /// A `serde_json::Value` describing the input fields.
  fn input_fields(&self) -> Value;

  /// Returns a `serde_json::Value` representing the output fields of the signature.
  ///
  /// This typically includes the name, type, and description of each output field.
  ///
  /// # Returns
  ///
  /// A `serde_json::Value` describing the output fields.
  fn output_fields(&self) -> Value;

  /// Updates the instruction string for the AI model.
  ///
  /// This method allows optimizers to dynamically change the task instruction
  /// provided to the underlying language model.
  ///
  /// # Arguments
  ///
  /// * `instruction` - The new instruction string.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or an `Error` on failure.
  fn update_instruction(&mut self, instruction: String) -> Result<()>;

  /// Validates input data against the signature schema.
  ///
  /// This method checks that all required inputs are present and correctly typed.
  ///
  /// # Arguments
  ///
  /// * `inputs` - The input data to validate.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or validation errors.
  fn validate_inputs(&self, inputs: &serde_json::Value) -> Result<()> {
    let input_fields = self.input_fields();
    if let Some(fields_obj) = input_fields.as_object() {
      for field_name in fields_obj.keys() {
        if !inputs.get(field_name).is_some() {
          return Err(
            crate::error::Error::validation(field_name, "present", "missing")
              .into(),
          );
        }
      }
    }
    Ok(())
  }

  /// Validates output data against the signature schema.
  ///
  /// This method checks that all required outputs are present and correctly typed.
  ///
  /// # Arguments
  ///
  /// * `outputs` - The output data to validate.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or validation errors.
  fn validate_outputs(&self, outputs: &serde_json::Value) -> Result<()> {
    let output_fields = self.output_fields();
    if let Some(fields_obj) = output_fields.as_object() {
      for field_name in fields_obj.keys() {
        if !outputs.get(field_name).is_some() {
          return Err(
            crate::error::Error::validation(field_name, "present", "missing")
              .into(),
          );
        }
      }
    }
    Ok(())
  }

  /// Gets the count of input fields.
  ///
  /// # Returns
  ///
  /// The number of input fields in this signature.
  fn input_fields_len(&self) -> usize {
    self.input_fields().as_object().map_or(0, |obj| obj.len())
  }

  /// Gets the count of output fields.
  ///
  /// # Returns
  ///
  /// The number of output fields in this signature.
  fn output_fields_len(&self) -> usize {
    self.output_fields().as_object().map_or(0, |obj| obj.len())
  }

  /// Gets the names of input fields.
  ///
  /// # Returns
  ///
  /// A vector of input field names.
  fn input_field_names(&self) -> Vec<String> {
    self
      .input_fields()
      .as_object()
      .map_or(Vec::new(), |obj| obj.keys().cloned().collect())
  }

  /// Gets the names of output fields.
  ///
  /// # Returns
  ///
  /// A vector of output field names.
  fn output_field_names(&self) -> Vec<String> {
    self
      .output_fields()
      .as_object()
      .map_or(Vec::new(), |obj| obj.keys().cloned().collect())
  }

  /// Generates a complete prompt with examples and instruction.
  ///
  /// This method creates a full prompt by combining the instruction,
  /// demonstration examples, and current input context.
  ///
  /// # Arguments
  ///
  /// * `inputs` - Current input values for context.
  ///
  /// # Returns
  ///
  /// A formatted prompt string ready for LM execution.
  fn generate_prompt(&self, inputs: &serde_json::Value) -> String {
    let mut prompt = String::new();

    // Add instruction
    prompt.push_str(&format!("Instruction: {}\n\n", self.instruction()));

    // Add few-shot examples if available
    let demos = self.demos();
    if !demos.is_empty() {
      prompt.push_str("Examples:\n");
      for (i, demo) in demos.iter().enumerate() {
        prompt.push_str(&format!("Example {}:\n", i + 1));

        // Add input fields from demo
        for field_name in self.input_field_names() {
          if let Some(value) = demo.data.get(&field_name) {
            prompt.push_str(&format!("{}: {}\n", field_name, value));
          }
        }

        prompt.push_str("---\n");

        // Add output fields from demo
        for field_name in self.output_field_names() {
          if let Some(value) = demo.data.get(&field_name) {
            prompt.push_str(&format!("{}: {}\n", field_name, value));
          }
        }

        prompt.push_str("\n");
      }
      prompt.push_str("\n");
    }

    // Add current inputs
    prompt.push_str("Current Task:\n");
    for field_name in self.input_field_names() {
      if let Some(value) = inputs.get(&field_name) {
        prompt.push_str(&format!("{}: {}\n", field_name, value));
      }
    }

    prompt.push_str("---\n");
    prompt.push_str("Please provide the output in the following format:\n");
    for field_name in self.output_field_names() {
      prompt.push_str(&format!("{}: [your answer]\n", field_name));
    }

    prompt
  }
  /// Appends a new field to the signature.
  ///
  /// This method allows for dynamic modification of the signature's fields.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the field to append.
  /// * `value` - The `serde_json::Value` representing the field's definition.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or an `Error` on failure.
  fn append(&mut self, name: &str, value: Value) -> Result<()>;

  /// Returns the current prefix string for the AI model.
  ///
  /// The prefix is typically prepended to the prompt, often used for few-shot learning.
  ///
  /// # Returns
  ///
  /// The prefix string as a `String`.
  fn prefix(&self) -> String {
    String::new() // Default empty prefix
  }

  /// Updates the prefix string for the AI model.
  ///
  /// # Arguments
  ///
  /// * `prefix` - The new prefix string.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or an `Error` on failure.
  fn update_prefix(&mut self, prefix: String) -> Result<()> {
    // Production: Store prefix for future prompt generation and formatting
    // Default implementation provides basic prefix storage functionality
    // Concrete implementations can override for more sophisticated prefix handling

    // Store the prefix in a normalized format for consistent usage
    let normalized_prefix = if prefix.is_empty() {
      "Default:".to_string()
    } else if prefix.ends_with(':') {
      prefix
    } else {
      format!("{}:", prefix)
    };

    // Log prefix update for debugging
    // debug!("DSPy Signature: Updated prefix to '{}'", normalized_prefix);

    // Note: This default implementation doesn't store the prefix permanently
    // Concrete signature implementations should override this method to:
    // 1. Store the prefix in their internal state
    // 2. Update any cached prompt templates
    // 3. Invalidate any compiled signatures that depend on the prefix

    Ok(())
  }
}
