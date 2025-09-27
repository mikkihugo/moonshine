//! # OXC Formatter Integration (Beta)
//!
//! High-performance JavaScript/TypeScript formatting using OXC.
//! Note: OXC formatter is not yet published as a separate crate.
//! This module provides a placeholder for future formatter integration.

// Note: oxc_formatter crate is not yet published on crates.io
// Once available, uncomment the following:
// use oxc_formatter::{format, FormatterOptions};
use oxc_allocator::Allocator;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// OXC formatter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxcFormatterConfig {
    pub enabled: bool,
    pub indent_width: Option<u8>,
    pub use_tabs: Option<bool>,
    pub line_width: Option<u8>,
    pub quote_style: Option<String>,    // "single", "double", "preserve"
    pub trailing_comma: Option<String>, // "none", "es5", "all"
    pub semicolons: Option<bool>,
    pub bracket_spacing: Option<bool>,
    pub arrow_parentheses: Option<String>, // "always", "avoid"
}

impl Default for OxcFormatterConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default since it's beta
            indent_width: Some(2),
            use_tabs: Some(false),
            line_width: Some(80),
            quote_style: Some("single".to_string()),
            trailing_comma: Some("es5".to_string()),
            semicolons: Some(true),
            bracket_spacing: Some(true),
            arrow_parentheses: Some("avoid".to_string()),
        }
    }
}

/// OXC formatter result
#[derive(Debug)]
pub struct OxcFormatterResult {
    pub formatted_code: String,
    pub source_unchanged: bool,
    pub formatting_time_ms: u64,
}

/// OXC formatter implementation
pub struct OxcFormatter {
    config: OxcFormatterConfig,
    allocator: Allocator,
}

impl OxcFormatter {
    /// Create new OXC formatter
    pub fn new(config: OxcFormatterConfig) -> Self {
        Self {
            config,
            allocator: Allocator::default(),
        }
    }

    /// Format JavaScript/TypeScript code
    pub fn format_code(&self, source_code: &str, file_path: &str) -> Result<OxcFormatterResult, Box<dyn std::error::Error>> {
        if !self.config.enabled {
            return Ok(OxcFormatterResult {
                formatted_code: source_code.to_string(),
                source_unchanged: true,
                formatting_time_ms: 0,
            });
        }

        let start_time = std::time::Instant::now();

        // Detect source type from file extension
        let source_type = self.detect_source_type(file_path);

        // Log formatting attempt for debugging
        log::debug!("Formatting {} ({:?})", file_path, source_type);

        // OXC formatter is not yet available as a separate crate
        // Return original code unchanged for now
        let formatting_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(OxcFormatterResult {
            formatted_code: source_code.to_string(),
            source_unchanged: true,
            formatting_time_ms,
        })
    }

    /// Check if formatter is available and enabled
    pub fn is_enabled(&self) -> bool {
        // Formatter is not yet available
        false
    }

    /// Get formatter configuration
    pub fn config(&self) -> &OxcFormatterConfig {
        &self.config
    }

    /// Update formatter configuration
    pub fn update_config(&mut self, config: OxcFormatterConfig) {
        self.config = config;
    }

    /// Detect source type from file path
    fn detect_source_type(&self, file_path: &str) -> SourceType {
        let path = Path::new(file_path);

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("ts") => SourceType::ts(),
            Some("tsx") => SourceType::tsx(),
            Some("jsx") => SourceType::jsx(),
            Some("mjs") => SourceType::mjs(),
            Some("cjs") => SourceType::cjs(),
            _ => SourceType::default(),
        }
    }
}

impl Default for OxcFormatter {
    fn default() -> Self {
        Self::new(OxcFormatterConfig::default())
    }
}

// #[cfg(test)]
// mod tests {
    use super::*;

    #[test]
    fn test_formatter_disabled_by_default() {
        let formatter = OxcFormatter::default();
        assert!(!formatter.is_enabled());
    }

    #[test]
    fn test_formatter_config() {
        let mut config = OxcFormatterConfig::default();
        config.enabled = true;
        config.indent_width = Some(4);

        let formatter = OxcFormatter::new(config);
        assert_eq!(formatter.config().indent_width, Some(4));
    }

    #[test]
    fn test_format_disabled() {
        let formatter = OxcFormatter::default();
        let source_code = "const x=1;";

        let result = formatter.format_code(source_code, "test.js").unwrap();
        assert!(result.source_unchanged);
        assert_eq!(result.formatted_code, source_code);
        assert_eq!(result.formatting_time_ms, 0);
    }

    #[test]
    fn test_format_not_yet_available() {
        let mut config = OxcFormatterConfig::default();
        config.enabled = true;

        let formatter = OxcFormatter::new(config);
        let source_code = "const x=1;";

        // Since formatter is not yet available, returns unchanged
        let result = formatter.format_code(source_code, "test.js").unwrap();
        assert!(result.source_unchanged);
        assert_eq!(result.formatted_code, source_code);
    }

    #[test]
    fn test_source_type_detection() {
        let formatter = OxcFormatter::default();

        // Test source type detection (simplified test)
        let ts_type = formatter.detect_source_type("test.ts");
        let js_type = formatter.detect_source_type("test.js");
        assert!(ts_type != js_type); // Different file types should be detected differently
        // Additional test for JavaScript detection
        let js_default = formatter.detect_source_type("test.js");
        assert!(js_default == js_type); // Should be consistent
    }
// }
