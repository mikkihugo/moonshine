//! Thin compatibility layer that re-exports the minimal items we still
//! need from the Extism PDK. Keeping them centralized helps ensure the
//! rest of the codebase depends on Moon-specific abstractions instead
//! of reaching for `extism_pdk` directly.

pub use extism_pdk::plugin_fn;
pub use extism_pdk::{FnResult, Json, WithReturnCode};

/// Unified error type for plugin failures when interfacing with Moon's host.
pub type PluginError = extism_pdk::Error;
