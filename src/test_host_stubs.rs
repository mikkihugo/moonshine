//! # Test Host Stubs: WASM Runtime Stubs for Native Testing
//!
//! This module provides stub implementations of WASM host functions that are
//! typically provided by the Extism runtime when running in a WASM environment.
//! These stubs enable native testing without requiring a full WASM runtime.
//!
//! ## Purpose
//! - **Native Testing**: Allows running tests with `cargo test` without WASM
//! - **Development**: Enables debugging and development outside WASM environment
//! - **CI/CD**: Supports continuous integration without WASM runtime dependencies
//!
//! ## Host Functions Stubbed
//! - Memory management: `alloc`, `length`, `load_*`, `store_*`
//! - Configuration: `config_get`, `get_log_level`
//! - Logging: `log_trace`, `log_debug`, `log_info`, `log_warn`, `log_error`
//!
//! @category testing
//! @safe team  
//! @mvp core
//! @complexity low
//! @since 1.0.0

/// Get log level (stub implementation for native testing)
/// Returns 0 (INFO level) by default
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn get_log_level() -> i32 {
  0
}

/// Get configuration value (stub implementation for native testing)
/// Returns -1 (not found) for all keys
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn config_get(_key: u64) -> i32 {
  -1
}

/// Get memory length (stub implementation for native testing)
/// Returns 0 for all pointers
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn length(_ptr: u64) -> u64 {
  0
}

/// Load 8-bit value from memory (stub implementation for native testing)
/// Returns 0 for all memory locations
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn load_u8(_ptr: u64) -> u8 {
  0
}

/// Load 64-bit value from memory (stub implementation for native testing) 
/// Returns 0 for all memory locations
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn load_u64(_ptr: u64) -> u64 {
  0
}

/// Store 8-bit value to memory (stub implementation for native testing)
/// No-op for all stores
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn store_u8(_ptr: u64, _value: u8) {}

/// Store 64-bit value to memory (stub implementation for native testing)
/// No-op for all stores
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn store_u64(_ptr: u64, _value: u64) {}

/// Allocate memory (stub implementation for native testing)
/// Returns 0 for all allocation requests
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn alloc(_size: u64) -> u64 {
  0
}

/// Log trace message (stub implementation for native testing)
/// No-op for all log calls
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_trace(_ptr: u64) {}

/// Log debug message (stub implementation for native testing)  
/// No-op for all log calls
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_debug(_ptr: u64) {}

/// Log info message (stub implementation for native testing)
/// No-op for all log calls
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_info(_ptr: u64) {}

/// Log warning message (stub implementation for native testing)
/// No-op for all log calls  
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_warn(_ptr: u64) {}

/// Log error message (stub implementation for native testing)
/// No-op for all log calls
#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_error(_ptr: u64) {}
