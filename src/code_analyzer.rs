//! Code analyzer module - streamlined entry point
//!
//! Self-documenting code analyzer with focused functionality.

// Re-export core analyzer functionality
pub use crate::code_analyzer_core::*;

// Core analyzer module
#[path = "code_analyzer/mod.rs"]
mod code_analyzer_core;