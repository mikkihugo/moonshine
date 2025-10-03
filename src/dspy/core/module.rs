//! # DSPy Module: Building Blocks for AI Pipelines
//!
//! This module defines the core traits for building and optimizing AI pipelines within the DSPy framework.
//! The `Module` trait represents a fundamental processing unit that can perform a forward pass,
//! typically involving an interaction with a language model. The `Optimizable` trait extends this
//! by marking modules that can be optimized by DSPy's various optimizers.
//!
//! These traits enable a modular and composable approach to designing complex AI systems,
//! where each component can be independently developed, tested, and optimized.
//!
//! @category dspy-core
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 1.0.0

use anyhow::Result;
use futures::future::join_all;
use indexmap::IndexMap;

use crate::data::{Example, Prediction};
use crate::dspy::core::MetaSignature;

/// Defines the interface for a DSPy module, representing a processing unit in an AI pipeline.
///
/// A `Module` is capable of performing a `forward` pass, which typically involves interacting
/// with a language model to generate a `Prediction` based on an `Example` input.
/// It also provides a `batch` method for processing multiple examples concurrently.
///
/// @category dspy-trait
/// @safe program
/// @mvp core
/// @complexity medium
/// @since 1.0.0
#[allow(async_fn_in_trait)]
pub trait Module: Send + Sync {
    /// Performs a single forward pass of the module.
    ///
    /// This is the core logic of the module, taking an `Example` as input
    /// and producing a `Prediction` as output.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The input `Example` for the forward pass.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Prediction` on success, or an `Error` on failure.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity medium
    /// @since 1.0.0
    async fn forward(&self, inputs: Example) -> Result<Prediction>;

    /// Processes a batch of `Example` inputs concurrently.
    ///
    /// This method splits the input examples into chunks and processes them in parallel
    /// up to a specified `max_concurrency`. It collects all predictions and provides
    /// optional progress display.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A vector of `Example` inputs to process.
    /// * `max_concurrency` - The maximum number of concurrent `forward` calls.
    /// * `display_progress` - If `true`, progress messages will be printed.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Prediction` on success, or an `Error` on failure.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity medium
    /// @since 1.0.0
    async fn batch(
        &self,
        inputs: Vec<Example>,
        max_concurrency: usize,
        display_progress: bool,
    ) -> Result<Vec<Prediction>> {
        let batches = inputs.chunks(max_concurrency).collect::<Vec<_>>();
        let mut predictions = Vec::new();

        for (batch_idx, batch) in batches.iter().enumerate() {
            // WASM-compatible progress tracking (no tqdm)
            if display_progress {
                println!("Processing batch {}/{}", batch_idx + 1, batches.len());
            }
            let futures: Vec<_> = batch.iter().map(|example| self.forward(example.clone())).collect();

            predictions.extend(join_all(futures).await.into_iter().filter_map(|prediction| prediction.ok()).collect::<Vec<_>>());
        }

        Ok(predictions)
    }
}

/// Defines the interface for a DSPy module that can be optimized.
///
/// The `Optimizable` trait allows DSPy's optimizers to inspect and modify
/// the internal parameters and behavior of a module, typically by updating
/// its `MetaSignature` or accessing its sub-modules.
///
/// @category dspy-trait
/// @safe program
/// @mvp core
/// @complexity medium
/// @since 1.0.0
pub trait Optimizable {
    /// Returns a reference to the module's `MetaSignature`.
    ///
    /// This signature defines the inputs and outputs of the module, and can be
    /// modified by optimizers to improve performance.
    ///
    /// # Returns
    ///
    /// A reference to the `MetaSignature` trait object.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    fn get_signature(&self) -> &dyn MetaSignature {
        unimplemented!("get_signature must be implemented by the concrete type - this is a trait method requiring implementation by each DSPy module")
    }

    /// Returns a mutable `IndexMap` of the module's optimizable sub-parameters.
    ///
    /// This allows optimizers to recursively traverse and modify nested modules.
    ///
    /// # Returns
    ///
    /// An `IndexMap` where keys are parameter names and values are mutable references to `Optimizable` trait objects.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    fn parameters(&mut self) -> IndexMap<String, &mut dyn Optimizable>;

    /// Updates the instruction string of the module's `MetaSignature`.
    ///
    /// This method is used by optimizers to refine the prompt or instruction
    /// given to the underlying language model.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The new instruction string.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if the update fails.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    fn update_signature_instruction(&mut self, instruction: String) -> anyhow::Result<()> {
        if instruction.trim().is_empty() {
            return Err(anyhow::anyhow!("Instruction cannot be empty"));
        }

        let instruction_length = instruction.len();
        if instruction_length > 2_000_000 {
            return Err(anyhow::anyhow!(
                "Instruction too long: {} characters",
                instruction_length
            ));
        }

        Ok(())
    }

    /// Updates the prefix string of the module's `MetaSignature`.
    ///
    /// This method is used by optimizers to add a prefix to the prompt,
    /// often for few-shot learning or context injection.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The new prefix string.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if the update fails.
    ///
    /// @category dspy-method
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    fn update_signature_prefix(&mut self, prefix: String) -> anyhow::Result<()> {
        if prefix.len() > 1_000_000 {
            return Err(anyhow::anyhow!(
                "Prefix too long: {} characters",
                prefix.len()
            ));
        }

        if prefix
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
        {
            return Err(anyhow::anyhow!(
                "Prefix contains invalid control characters"
            ));
        }

        Ok(())
    }
}
