//! Compile-time provider capabilities - zero runtime parsing!
//!
//! Provider capabilities are converted from JSON to native Rust const data at build time.
//! This provides instant access with zero parsing overhead.

use super::ProviderCapabilities;

// Include the auto-generated provider capabilities
include!(concat!(env!("OUT_DIR"), "/compiled_providers.rs"));

/// Get compiled provider capabilities by name with zero runtime cost
pub fn get_compiled_provider_capabilities(name: &str) -> Option<ProviderCapabilities> {
    get_default_provider_capabilities(name)
}

/// Get all available provider names
pub fn available_compiled_provider_names() -> Vec<&'static str> {
    available_provider_names()
}

/// Check if a provider has compiled capabilities
pub fn has_compiled_provider(name: &str) -> bool {
    get_compiled_provider_capabilities(name).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_providers_loaded() {
        assert!(!DEFAULT_PROVIDER_CAPABILITIES.is_empty());
    }

    #[test]
    fn test_provider_access() {
        let available_names = available_compiled_provider_names();
        assert!(!available_names.is_empty());

        // Test accessing specific providers
        for name in available_names {
            assert!(has_compiled_provider(name));
            let capabilities = get_compiled_provider_capabilities(name);
            assert!(capabilities.is_some());

            let caps = capabilities.unwrap();
            assert!(caps.code_analysis >= 0.0 && caps.code_analysis <= 1.0);
            assert!(caps.code_generation >= 0.0 && caps.code_generation <= 1.0);
            assert!(caps.complex_reasoning >= 0.0 && caps.complex_reasoning <= 1.0);
            assert!(caps.speed >= 0.0 && caps.speed <= 1.0);
            assert!(caps.context_length > 0);
        }
    }

    #[test]
    fn test_basic_providers_exist() {
        // Test that basic providers are compiled
        assert!(has_compiled_provider("claude"));
        assert!(has_compiled_provider("google"));
        assert!(has_compiled_provider("openai"));
    }
}
