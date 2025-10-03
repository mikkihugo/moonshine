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

/// The Collaborative Prompt Optimization (COPRO) optimizer.
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
/// Real DSPy optimizers like MIPROv2, BootstrapFinetune, and COPRO provide:
/// - Multi-stage optimization (bootstrapping, proposal, search)
/// - Metric-based evaluation and selection
/// - State serialization and resumption
/// - Composable optimizer chains (BetterTogether pattern)
/// - Automatic instruction and demonstration generation
///
/// @category dspy-trait
/// @safe program
/// @mvp core
/// @complexity high
/// @since 2.0.0
pub trait Teleprompter: Send + Sync {
    /// Compiles and optimizes a DSPy program.
    async fn compile<M>(&mut self, program: M, trainset: Vec<Example>) -> Result<M>
    where
        M: Module + Clone;

    /// Gets the name of the optimizer for logging and state management.
    fn name(&self) -> &str;

    /// Serializes the optimizer's state for resumption.
    fn dump_state(&self) -> Result<serde_json::Value>;

    /// Loads the optimizer's state from a previous session.
    fn load_state(&mut self, state: serde_json::Value) -> Result<()>;
}

/// A legacy optimizer trait for backward compatibility.
#[allow(async_fn_in_trait)]
pub trait Optimizer {
    /// Compiles and optimizes a DSPy module.
    async fn compile<M>(&self, module: &mut M, trainset: Vec<Example>) -> Result<()>
    where
        M: Module + Optimizable + Evaluator;
}

/// A type alias for a metric function used to evaluate program performance.
pub type MetricFn = Box<dyn Fn(&Example, &Prediction) -> f64 + Send + Sync>;

/// Configuration for DSPy optimizers, matching real DSPy patterns.
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// The maximum number of bootstrapped demonstrations to generate.
    pub max_bootstrapped_demos: usize,
    /// The maximum number of labeled demonstrations to use.
    pub max_labeled_demos: usize,
    /// The number of candidate programs to generate during optimization.
    pub num_candidate_programs: usize,
    /// The number of threads to use for optimization.
    pub num_threads: usize,
    /// The size of the minibatch for training.
    pub minibatch_size: usize,
    /// The number of steps between full evaluations on the minibatch.
    pub minibatch_full_eval_steps: usize,
    /// The automatic optimization mode ("light", "medium", or "heavy").
    pub auto_mode: String,
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
