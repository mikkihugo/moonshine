//! AI-powered linter module with comprehensive code analysis
//!
//! Self-documenting linter components for TypeScript/JavaScript analysis.

pub mod ai_linter_core;
pub mod pattern_matcher;
pub mod pattern_types;

// Include the full implementation for backward compatibility
#[path = "ai_linter_full.rs"]
pub mod ai_linter_full;

// Re-export main types and functionality
pub use ai_linter_core::*;
pub use pattern_matcher::*;
pub use pattern_types::*;
