//! Compile-time pattern configurations - zero runtime parsing!
//!
//! Pattern configurations are converted from JSON to native Rust const data at build time.
//! This provides instant access with zero parsing overhead.

use crate::linter::SuggestionSeverity;

/// Compile-time pattern rule definition - can be used in const contexts
#[derive(Debug, Clone)]
pub struct PatternRule {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub severity: SuggestionSeverity,
    pub category: &'static str,
    pub enabled: bool,
    pub fix_template: Option<&'static str>,
    pub languages: &'static [&'static str],
    pub impact_score: u64,
    pub pattern: &'static str,
}

/// Compile-time pattern configuration
#[derive(Debug, Clone)]
pub struct PatternConfig {
    pub security_patterns: &'static [PatternRule],
    pub performance_patterns: &'static [PatternRule],
    pub typescript_patterns: &'static [PatternRule],
    pub documentation_patterns: &'static [PatternRule],
    pub custom_patterns: &'static [PatternRule],
}

// Include the auto-generated pattern configuration
include!(concat!(env!("OUT_DIR"), "/compiled_patterns.rs"));

/// Get default pattern configuration with zero runtime cost
pub fn get_default_pattern_config() -> &'static PatternConfig {
    &DEFAULT_PATTERN_CONFIG
}

/// Get all patterns as a single iterator - zero allocation
pub fn all_patterns_iter() -> impl Iterator<Item = &'static PatternRule> {
    DEFAULT_PATTERN_CONFIG
        .security_patterns
        .iter()
        .chain(DEFAULT_PATTERN_CONFIG.performance_patterns.iter())
        .chain(DEFAULT_PATTERN_CONFIG.typescript_patterns.iter())
        .chain(DEFAULT_PATTERN_CONFIG.documentation_patterns.iter())
        .chain(DEFAULT_PATTERN_CONFIG.custom_patterns.iter())
}

/// Get patterns by category - zero allocation
pub fn patterns_by_category(category: &'static str) -> impl Iterator<Item = &'static PatternRule> {
    all_patterns_iter().filter(move |pattern| pattern.category == category)
}

/// Get enabled patterns only - zero allocation
pub fn enabled_patterns_iter() -> impl Iterator<Item = &'static PatternRule> {
    all_patterns_iter().filter(|pattern| pattern.enabled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_patterns_loaded() {
        let config = get_default_pattern_config();

        assert!(!config.security_patterns.is_empty());
        assert!(!config.performance_patterns.is_empty());
        assert!(!config.typescript_patterns.is_empty());
        assert!(!config.documentation_patterns.is_empty());
    }

    #[test]
    fn test_pattern_iteration() {
        let pattern_count = all_patterns_iter().count();
        assert!(pattern_count > 0);

        let enabled_count = enabled_patterns_iter().count();
        assert!(enabled_count > 0);
    }

    #[test]
    fn test_pattern_categories() {
        let security_patterns: Vec<_> = patterns_by_category("security").collect();
        assert!(!security_patterns.is_empty());

        let performance_patterns: Vec<_> = patterns_by_category("performance").collect();
        assert!(!performance_patterns.is_empty());
    }
}
