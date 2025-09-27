//! Prompt optimization module with DSPy integration
//!
//! Self-documenting prompt optimization components.

pub mod dspy_optimizer;
pub mod optimizer_types;

// Include the full implementation for backward compatibility
#[path = "optimizer_full.rs"]
pub mod optimizer_full;

// Re-export main types and functionality
pub use dspy_optimizer::*;
pub use optimizer_types::*;
