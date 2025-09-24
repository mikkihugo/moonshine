//! # Custom Assertions for Moon-Shine Testing
//!
//! Domain-specific assertions for readable and maintainable tests.
//! Provides fluent assertion API for moon-shine functionality.
//!
//! @category testing
//! @safe program
//! @complexity medium
//! @since 2.0.0

#[macro_export]
macro_rules! assert_moonshine {
    // Usage: assert_moonshine!(results, has_suggestions: 1, has_errors, fast_processing: 1000)
    ($results:expr, $($key:ident $( : $val:expr )?),* $(,)?) => {{
        $(
            match stringify!($key) {
                "has_suggestions" => {
                    $results.assert_suggestion_count($($val)?).unwrap();
                },
                "has_errors" => {
                    $results.assert_has_error_severity().unwrap();
                },
                "has_warnings" => {
                    $results.assert_has_warning_severity().unwrap();
                },
                "has_info" => {
                    $results.assert_has_info_severity().unwrap();
                },
                "fast_processing" => {
                    $results.assert_processing_time_within($($val)?).unwrap();
                },
                "no_suggestions" => {
                    $results.assert_no_suggestions().unwrap();
                },
                "min_confidence" => {
                    $results.assert_min_confidence($($val)?).unwrap();
                },
                _ => panic!("Unknown assertion key: {}", stringify!($key)),
            }
        )*
    }};
}
use std::time::Duration;

use crate::analysis::{AnalysisResults, MoonShineResponse};
use crate::config::MoonShineConfig;
use crate::error::{Error, Result};
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};

/// Custom assertion trait for MoonShine analysis results
pub trait MoonShineAssertions {
    /// Assert specific number of suggestions
    fn assert_suggestion_count(&self, expected: usize) -> Result<()>;

    /// Assert has suggestions with error severity
    fn assert_has_error_severity(&self) -> Result<()>;

    /// Assert has suggestions with warning severity
    fn assert_has_warning_severity(&self) -> Result<()>;

    /// Assert has suggestions with info severity
    fn assert_has_info_severity(&self) -> Result<()>;

    /// Assert specific category exists
    fn assert_has_category(&self, category: SuggestionCategory) -> Result<()>;

    /// Assert suggestion contains message pattern
    fn assert_contains_message(&self, pattern: &str) -> Result<()>;

    /// Assert file was processed
    fn assert_file_processed(&self, file_path: &str) -> Result<()>;

    /// Assert processing time within range
    fn assert_processing_time_within(&self, max_ms: u64) -> Result<()>;

    /// Assert no suggestions (clean code)
    fn assert_no_suggestions(&self) -> Result<()>;

    /// Assert minimum confidence score
    fn assert_min_confidence(&self, min_score: f64) -> Result<()>;
}

impl MoonShineAssertions for AnalysisResults {
    fn assert_suggestion_count(&self, expected: usize) -> Result<()> {
        if self.suggestions.len() == expected {
            Ok(())
        } else {
            Err(Error::Config {
                message: format!("Expected {} suggestions, got {}", expected, self.suggestions.len()),
                field: None,
                value: None,
            })
        }
    }

    fn assert_has_error_severity(&self) -> Result<()> {
        let has_error = self.suggestions.iter().any(|s| matches!(s.severity, LintSeverity::Error));

        if has_error {
            Ok(())
        } else {
            Err(Error::Config {
                message: "Expected at least one error severity issue".to_string(),
                field: None,
                value: None,
            })
        }
    }

    fn assert_has_warning_severity(&self) -> Result<()> {
        let has_warning = self.suggestions.iter().any(|s| matches!(s.severity, LintSeverity::Warning));

        if has_warning {
            Ok(())
        } else {
            Err(Error::Config {
                message: "Expected at least one warning severity issue".to_string(),
                field: None,
                value: None,
            })
        }
    }

    fn assert_has_info_severity(&self) -> Result<()> {
        let has_info = self.suggestions.iter().any(|s| matches!(s.severity, LintSeverity::Info));

        if has_info {
            Ok(())
        } else {
            Err(Error::Config {
                message: "Expected at least one info severity issue".to_string(),
                field: None,
                value: None,
            })
        }
    }

    fn assert_contains_message(&self, pattern: &str) -> Result<()> {
        let has_pattern = self.suggestions.iter().any(|s| s.message.contains(pattern));

        if has_pattern {
            Ok(())
        } else {
            Err(Error::Config {
                message: format!("Expected issue message containing '{}'", pattern),
                field: None,
                value: None,
            })
        }
    }

    fn assert_no_suggestions(&self) -> Result<()> {
        if self.suggestions.is_empty() {
            Ok(())
        } else {
            Err(Error::Config {
                message: format!("Expected no issues, got {}", self.suggestions.len()),
                field: None,
                value: None,
            })
        }
    }
}

/// Custom assertion trait for configuration
pub trait ConfigAssertions {
    /// Assert configuration is valid
    fn assert_valid(&self) -> Result<()>;

    /// Assert cache is enabled
    fn assert_cache_enabled(&self) -> Result<()>;

    /// Assert cache is disabled
    fn assert_cache_disabled(&self) -> Result<()>;

    /// Assert has AI model configured
    fn assert_has_ai_model(&self) -> Result<()>;

    /// Assert max files within range
    fn assert_max_files_within(&self, min: usize, max: usize) -> Result<()>;

    /// Assert has include patterns
    fn assert_has_include_patterns(&self) -> Result<()>;

    /// Assert has exclude patterns
    fn assert_has_exclude_patterns(&self) -> Result<()>;
}

impl ConfigAssertions for MoonShineConfig {
    fn assert_valid(&self) -> Result<()> {
        if self.ai_model.is_empty() {
            return Err(Error::Configuration {
                message: "AI model cannot be empty".to_string(),
            });
        }

        // max_files field removed

        Ok(())
    }

    // cache_enabled field and assertion removed

    // cache_enabled field and assertion removed

    fn assert_has_ai_model(&self) -> Result<()> {
        if self.ai_model.as_ref().map_or(false, |s| !s.is_empty()) {
            Ok(())
        } else {
            Err(Error::Configuration {
                message: "Expected AI model to be configured".to_string(),
            })
        }
    }

    // max_files field and assertion removed

    fn assert_has_include_patterns(&self) -> Result<()> {
        if self.include_patterns.as_ref().map_or(false, |v| !v.is_empty()) {
            Ok(())
        } else {
            Err(Error::Config {
                message: "Expected include patterns to be configured".to_string(),
                field: None,
            })
        }
    }

    fn assert_has_exclude_patterns(&self) -> Result<()> {
        if self.exclude_patterns.as_ref().map_or(false, |v| !v.is_empty()) {
            Ok(())
        } else {
            Err(Error::Config {
                message: "Expected exclude patterns to be configured".to_string(),
                field: None,
            })
        }
    }
}

/// Performance assertion trait
pub trait PerformanceAssertions {
    /// Assert execution time within limit
    fn assert_execution_time_within(&self, max_duration: Duration) -> Result<()>;

    /// Assert memory usage within limit
    fn assert_memory_usage_within(&self, max_mb: u64) -> Result<()>;

    /// Assert throughput meets minimum
    fn assert_throughput_at_least(&self, min_ops_per_sec: f64) -> Result<()>;
}

/// Performance metrics for assertions
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub execution_time: Duration,
    pub memory_usage_mb: u64,
    pub operations_completed: usize,
}

impl PerformanceAssertions for PerformanceMetrics {
    fn assert_execution_time_within(&self, max_duration: Duration) -> Result<()> {
        if self.execution_time <= max_duration {
            Ok(())
        } else {
            Err(Error::Config {
                message: format!("Execution time {:?} exceeds maximum {:?}", self.execution_time, max_duration),
                field: None,
            })
        }
    }

    fn assert_memory_usage_within(&self, max_mb: u64) -> Result<()> {
        if self.memory_usage_mb <= max_mb {
            Ok(())
        } else {
            Err(Error::Configuration {
                message: format!("Memory usage {}MB exceeds maximum {}MB", self.memory_usage_mb, max_mb),
            })
        }
    }

    fn assert_throughput_at_least(&self, min_ops_per_sec: f64) -> Result<()> {
        let actual_throughput = if self.execution_time.as_secs_f64() > 0.0 {
            self.operations_completed as f64 / self.execution_time.as_secs_f64()
        } else {
            0.0
        };

        if actual_throughput >= min_ops_per_sec {
            Ok(())
        } else {
            Err(Error::Configuration {
                message: format!("Throughput {:.2} ops/sec below minimum {:.2} ops/sec", actual_throughput, min_ops_per_sec),
            })
        }
    }
}

/// Fluent assertion builder
pub struct AssertThat<T> {
    value: T,
}

impl<T> AssertThat<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

/// Create fluent assertion
pub fn assert_that<T>(value: T) -> AssertThat<T> {
    AssertThat::new(value)
}

/// Specialized assertion for AnalysisResults
impl AssertThat<AnalysisResults> {
    pub fn has_suggestions(&self, count: usize) -> Result<&Self> {
        self.value.assert_suggestion_count(count)?;
        Ok(self)
    }

    pub fn has_errors(&self) -> Result<&Self> {
        self.value.assert_has_error_severity()?;
        Ok(self)
    }

    pub fn has_warnings(&self) -> Result<&Self> {
        self.value.assert_has_warning_severity()?;
        Ok(self)
    }

    pub fn has_category(&self, category: SuggestionCategory) -> Result<&Self> {
        self.value.assert_has_category(category)?;
        Ok(self)
    }

    pub fn contains_message(&self, pattern: &str) -> Result<&Self> {
        self.value.assert_contains_message(pattern)?;
        Ok(self)
    }

    pub fn processed_file(&self, file_path: &str) -> Result<&Self> {
        self.value.assert_file_processed(file_path)?;
        Ok(self)
    }

    pub fn completed_within(&self, max_ms: u64) -> Result<&Self> {
        self.value.assert_processing_time_within(max_ms)?;
        Ok(self)
    }

    pub fn has_no_suggestions(&self) -> Result<&Self> {
        self.value.assert_no_suggestions()?;
        Ok(self)
    }

    pub fn has_min_confidence(&self, min_score: f64) -> Result<&Self> {
        self.value.assert_min_confidence(min_score)?;
        Ok(self)
    }
}

/// Specialized assertion for MoonShineConfig
impl AssertThat<MoonShineConfig> {
    pub fn is_valid(&self) -> Result<&Self> {
        self.value.assert_valid()?;
        Ok(self)
    }

    pub fn has_cache_enabled(&self) -> Result<&Self> {
        self.value.assert_cache_enabled()?;
        Ok(self)
    }

    pub fn has_cache_disabled(&self) -> Result<&Self> {
        self.value.assert_cache_disabled()?;
        Ok(self)
    }

    pub fn has_ai_model(&self) -> Result<&Self> {
        self.value.assert_has_ai_model()?;
        Ok(self)
    }

    pub fn has_max_files_within(&self, min: usize, max: usize) -> Result<&Self> {
        self.value.assert_max_files_within(min, max)?;
        Ok(self)
    }
}

/// Convenience function for analysis result assertions
pub fn assert_moonshine(results: AnalysisResults) -> AssertThat<AnalysisResults> {
    assert_that(results)
}

/// Convenience function for config assertions
pub fn assert_config(config: MoonShineConfig) -> AssertThat<MoonShineConfig> {
    assert_that(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_suggestion() -> LintIssue {
        LintIssue {
            rule_name: "test-rule".to_string(),
            message: "Test suggestion message".to_string(),
            line: 1,
            column: 1,
            severity: LintSeverity::Warning,
            fix_available: true,
        }
    }

    fn create_test_results() -> AnalysisResults {
        AnalysisResults {
            suggestions: vec![create_test_suggestion()],
            semantic_warnings: vec![],
            tsdoc_coverage: None,
            quality_score: None,
            parse_errors: vec![],
            ignored_files: vec![],
            ai_model: Some("test-model".to_string()),
        }
    }

    #[test]
    fn test_suggestion_count_assertion() {
        let results = create_test_results();
        assert!(results.assert_suggestion_count(1).is_ok());
        assert!(results.assert_suggestion_count(2).is_err());
    }

    #[test]
    fn test_severity_assertions() {
        let results = create_test_results();
        assert!(results.assert_has_warning_severity().is_ok());
        assert!(results.assert_has_error_severity().is_err());
        assert!(results.assert_has_info_severity().is_err());
    }

    #[test]
    fn test_category_assertion() {
        let results = create_test_results();
        assert!(results.assert_has_category(SuggestionCategory::BestPractices).is_ok());
        assert!(results.assert_has_category(SuggestionCategory::TypeSafety).is_err());
    }

    #[test]
    fn test_message_pattern_assertion() {
        let results = create_test_results();
        assert!(results.assert_contains_message("Test suggestion").is_ok());
        assert!(results.assert_contains_message("Nonexistent").is_err());
    }

    #[test]
    fn test_processing_time_assertion() {
        let results = create_test_results();
        assert!(results.assert_processing_time_within(200).is_ok());
        assert!(results.assert_processing_time_within(50).is_err());
    }

    #[test]
    fn test_confidence_assertion() {
        let results = create_test_results();
        assert!(results.assert_min_confidence(0.8).is_ok());
        assert!(results.assert_min_confidence(0.95).is_err());
    }

    #[test]
    fn test_config_assertions() {
        let config = MoonShineConfig::london_test();
        assert!(config.assert_valid().is_ok());
        assert!(config.assert_has_ai_model().is_ok());
        assert!(config.assert_cache_disabled().is_ok());
    }

    #[test]
    fn test_fluent_assertions() {
        let results = create_test_results();
        let results_ref = &results;
        let assertion_result = assert_moonshine(results)
            .has_suggestions(1)
            .and_then(|a| a.has_warnings())
            .and_then(|a| a.has_category(SuggestionCategory::BestPractices))
            .and_then(|a| a.contains_message("Test suggestion"))
            .and_then(|a| a.completed_within(200))
            .and_then(|a| a.has_min_confidence(0.8));

        assert!(assertion_result.is_ok());
    }

    #[test]
    fn test_performance_assertions() {
        let metrics = PerformanceMetrics {
            execution_time: Duration::from_millis(100),
            memory_usage_mb: 50,
            operations_completed: 10,
        };

        assert!(metrics.assert_execution_time_within(Duration::from_millis(200)).is_ok());
        assert!(metrics.assert_memory_usage_within(100).is_ok());
        assert!(metrics.assert_throughput_at_least(50.0).is_ok()); // 10 ops / 0.1 sec = 100 ops/sec
    }
}
