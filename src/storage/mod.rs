//! Storage module for Moon Shine persistence
//!
//! Self-documenting storage components.

pub mod storage_core;

// Include the full implementation for backward compatibility
#[path = "storage_full.rs"]
pub mod storage_full;

// Re-export main functionality
pub use storage_core::*;
