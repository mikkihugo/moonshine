//! Engine module - streamlined entry point
//!
//! Self-documenting analysis engine with focused functionality.

// Re-export core engine functionality
pub use crate::engine_core::*;

// Core engine module
#[path = "engine/mod.rs"]
mod engine_core;