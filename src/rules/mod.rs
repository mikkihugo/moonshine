//! # MoonShine Rules - Modular Rule Engine Architecture
//!
//! This module provides a clean, modular architecture for MoonShine's rule system:
//! - **engine**: Core rule execution engine
//! - **code_quality**: C-series code quality rules
//! - **security**: S-series security rules
//! - **ai_integration**: Claude AI enhancement
//!
//! ## Architecture Benefits
//! - **Modular**: Each rule in its own file
//! - **Maintainable**: Clear separation of concerns
//! - **Testable**: Individual rule testing
//! - **Scalable**: Easy to add new rules
//!
//! @category moonshine-rules
//! @safe program
//! @mvp enhanced
//! @complexity medium
//! @since 2.1.0

pub mod engine;
pub mod code_quality;
pub mod security;
pub mod ai_integration;
pub mod utils;

#[cfg(test)]
mod integration_test;

// Re-exports for convenience
pub use engine::{MoonShineRuleEngine, MoonShineRule, MoonShineRuleCategory, RuleImplementation};
pub use ai_integration::AIEnhancer;