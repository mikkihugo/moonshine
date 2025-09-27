//! Code analyzer module with AST-based analysis using OXC
//!
//! Self-documenting modules for comprehensive TypeScript/JavaScript code analysis.

pub mod analysis_types;
pub mod analyzer_engine;
pub mod complexity_metrics;
pub mod config_types;
pub mod security_analysis;

// Re-export main types and structs for convenience
pub use analysis_types::*;
pub use analyzer_engine::*;
pub use complexity_metrics::*;
pub use config_types::*;
pub use security_analysis::*;
