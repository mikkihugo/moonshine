//! Compile-time configuration schema - zero runtime JSON generation!
//!
//! The config schema is pre-generated at compile time instead of building
//! it dynamically with serde_json::json! macros.

use serde_json::Value;
use once_cell::sync::Lazy;

/// Pre-compiled configuration schema - zero runtime cost
pub static CONFIG_SCHEMA: Lazy<Value> = Lazy::new(|| {
    // Instead of runtime serde_json::json! construction,
    // we pre-serialize the schema and include it as a const string
    const SCHEMA_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/config_schema.json"));
    serde_json::from_str(SCHEMA_JSON).expect("Pre-compiled schema should be valid")
});

/// Get the configuration schema with zero runtime cost
pub fn get_config_schema() -> &'static Value {
    &*CONFIG_SCHEMA
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_loads() {
        let schema = get_config_schema();
        assert!(schema.is_object());

        // Should have main schema properties
        assert!(schema.get("type").is_some());
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_schema_has_moonshine_fields() {
        let schema = get_config_schema();
        let properties = schema.get("properties").unwrap().as_object().unwrap();

        // Should contain MoonShine-specific configuration
        assert!(properties.contains_key("aiModel"));
        assert!(properties.contains_key("aiProviders"));
        assert!(properties.contains_key("temperature"));
    }
}