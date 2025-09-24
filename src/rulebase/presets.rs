//! Compile-time rule presets - zero runtime parsing!
//!
//! Rule presets are converted from JSON to native Rust static data at build time.
//! This provides instant access with zero parsing overhead.

use serde_json::Value;
use std::collections::HashMap;

// Include the auto-generated preset constants
include!(concat!(env!("OUT_DIR"), "/compiled_presets.rs"));

/// Get all available preset names
pub fn available_presets() -> Vec<&'static str> {
    vec![
        "performance-optimized",
        "development-friendly",
        "typescript-strict",
        "enterprise-strict",
        "security-critical",
    ]
}

/// Get preset configuration by name - zero allocation
pub fn get_preset(name: &str) -> Option<&'static HashMap<String, Value>> {
    match name {
        "performance-optimized" => Some(&*PERFORMANCE_OPTIMIZED_PRESET),
        "development-friendly" => Some(&*DEVELOPMENT_FRIENDLY_PRESET),
        "typescript-strict" => Some(&*TYPESCRIPT_STRICT_PRESET),
        "enterprise-strict" => Some(&*ENTERPRISE_STRICT_PRESET),
        "security-critical" => Some(&*SECURITY_CRITICAL_PRESET),
        _ => None,
    }
}

/// Check if a preset exists
pub fn has_preset(name: &str) -> bool {
    get_preset(name).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presets_load() {
        let presets = available_presets();
        assert!(!presets.is_empty());

        for preset_name in presets {
            assert!(has_preset(preset_name));
            let preset = get_preset(preset_name).unwrap();
            assert!(!preset.is_empty());
        }
    }

    #[test]
    fn test_preset_access() {
        // Test accessing a specific preset
        if let Some(preset) = get_preset("performance-optimized") {
            assert!(!preset.is_empty());
        }
    }

    #[test]
    fn test_invalid_preset() {
        assert!(get_preset("nonexistent").is_none());
        assert!(!has_preset("nonexistent"));
    }
}
