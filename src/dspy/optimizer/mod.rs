//! # DSPy Optimizer: Core Optimization Components
//!
//! This module defines the core components for optimizing DSPy modules. It re-exports
//! specific optimizers, such as `copro`, and defines the fundamental `Optimizer` trait.
//!
//! The `Optimizer` trait provides a standardized interface for compiling and improving
//! DSPy modules based on a given training set. This is where the self-improving aspect
//! of DSPy comes into play, as optimizers iteratively refine module behavior to enhance
//! performance and quality.
//!
//! @category dspy-optimizer
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 1.0.0

pub mod copro;

pub use copro::*;

use crate::data::{Example, Prediction};
use crate::dspy::{
  core::{Module, Optimizable},
  evaluate::Evaluator,
};
use anyhow::Result;

/// A trait for modern DSPy optimizers, known as "teleprompters".
///
/// Teleprompters are responsible for compiling and optimizing DSPy programs.
/// They support multi-stage optimization, metric-based evaluation, and state management.
pub trait Teleprompter: Send + Sync {
  /// Compiles and optimizes a DSPy program.
  ///
  /// # Arguments
  ///
  /// * `program` - The DSPy module to be optimized.
  /// * `trainset` - A vector of `Example`s for training.
  ///
  /// # Returns
  ///
  /// A `Result` containing the optimized module on success, or an error.
  async fn compile<M>(
    &mut self,
    program: M,
    trainset: Vec<Example>,
  ) -> Result<M>
  where
    M: Module + Clone;

  /// Returns the name of the optimizer for logging and state management.
  fn name(&self) -> &str;

  /// Serializes the optimizer's state for resumption.
  fn dump_state(&self) -> Result<serde_json::Value>;

  /// Loads the optimizer's state from a previous session.
  fn load_state(&mut self, state: serde_json::Value) -> Result<()>;
}

/// A legacy trait for DSPy optimizers, maintained for backward compatibility.
#[allow(async_fn_in_trait)]
pub trait Optimizer {
  /// Compiles and optimizes a DSPy module.
  ///
  /// # Arguments
  ///
  /// * `module` - The DSPy module to be optimized.
  /// * `trainset` - A vector of `Example`s for training.
  ///
  /// # Returns
  ///
  /// A `Result` indicating success or an error.
  async fn compile<M>(
    &self,
    module: &mut M,
    trainset: Vec<Example>,
  ) -> Result<()>
  where
    M: Module + Optimizable + Evaluator;
}

/// A type alias for a metric function used to evaluate program performance.
pub type MetricFn = Box<dyn Fn(&Example, &Prediction) -> f64 + Send + Sync>;

/// Configuration settings for the optimization process.
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
  pub max_bootstrapped_demos: usize,
  pub max_labeled_demos: usize,
  pub num_candidate_programs: usize,
  pub num_threads: usize,
  pub minibatch_size: usize,
  pub minibatch_full_eval_steps: usize,
  pub auto_mode: String, // "light", "medium", "heavy"
}

impl Default for OptimizationConfig {
  fn default() -> Self {
    Self {
      max_bootstrapped_demos: 4,
      max_labeled_demos: 4,
      num_candidate_programs: 10,
      num_threads: 4,
      minibatch_size: 40,
      minibatch_full_eval_steps: 4,
      auto_mode: "medium".to_string(),
    }
  }
}
