//! Analysis engine module
//!
//! Self-documenting analysis engine components.

pub mod engine_core;

// Include the full implementation for backward compatibility
#[path = "engine_full.rs"]
pub mod engine_full;

// Re-export main functionality
pub use engine_core::*;
