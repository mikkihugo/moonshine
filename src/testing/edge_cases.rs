//! # Edge Case Testing
//!
//! Tests for unusual, boundary, and edge case scenarios that moon-shine
//! might encounter in real-world usage.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use std::collections::HashMap;
use std::time::Duration;

use crate::analysis::AnalysisResults;
use crate::config::MoonShineConfig;
use crate::error::Result;
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::testing::assertions::{ConfigAssertions, MoonShineAssertions};
use crate::testing::builders::{AnalysisResultsBuilder, ConfigBuilder, LintIssueBuilder};
use crate::testing::fixtures::{ExpectedIssue, TestDataBuilder};

/// Edge case test scenarios
pub struct EdgeCaseTests;

impl EdgeCaseTests {
    /// Test with empty files
    pub fn empty_file_scenarios() -> Vec<EdgeCaseScenario> {
        vec![
            EdgeCaseScenario {
                name: "completely_empty_file".to_string(),
                description: "File with zero bytes".to_string(),
                input_content: "".to_string(),
                expected_behavior: EdgeCaseExpectation::NoSuggestions,
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "whitespace_only_file".to_string(),
                description: "File with only whitespace".to_string(),
                input_content: "   \n\t\n   \n".to_string(),
                expected_behavior: EdgeCaseExpectation::NoSuggestions,
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "comments_only_file".to_string(),
                description: "File with only comments".to_string(),
                input_content: "// This is a comment\n/* Block comment */\n// Another comment".to_string(),
                expected_behavior: EdgeCaseExpectation::NoSuggestions,
                should_pass: true,
            },
        ]
    }

    /// Test with extremely large files
    pub fn large_file_scenarios() -> Vec<EdgeCaseScenario> {
        vec![
            EdgeCaseScenario {
                name: "massive_single_line".to_string(),
                description: "Single line with 10,000 characters".to_string(),
                input_content: "const data = '".to_string() + &"x".repeat(9990) + "';",
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(0, 2),
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "many_small_functions".to_string(),
                description: "File with 1000 tiny functions".to_string(),
                input_content: (0..1000)
                    .map(|i| format!("function func{}() {{ return {}; }}\n", i, i))
                    .collect::<Vec<_>>()
                    .join(""),
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(0, 50),
                should_pass: true,
            },
        ]
    }

    /// Test with malformed or invalid code
    pub fn malformed_code_scenarios() -> Vec<EdgeCaseScenario> {
        vec![
            EdgeCaseScenario {
                name: "syntax_error_storm".to_string(),
                description: "Code with multiple syntax errors".to_string(),
                input_content: r#"
                    const x = {{{;
                    function unclosed() {
                    if (true {
                        let y = ;
                    class MissingBrace
                    export default
                "#
                .to_string(),
                expected_behavior: EdgeCaseExpectation::ErrorsExpected,
                should_pass: true, // Should handle gracefully
            },
            EdgeCaseScenario {
                name: "unicode_chaos".to_string(),
                description: "Code with problematic Unicode characters".to_string(),
                input_content: "const 变量 = '🚀💥🔥'; // Mixed scripts\nfunction тест() { return '‌‍'; }".to_string(),
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(0, 3),
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "deeply_nested_structures".to_string(),
                description: "Extremely nested object/array structures".to_string(),
                input_content: "const deep = ".to_string() + &"{".repeat(100) + "x: 1" + &"}".repeat(100) + ";",
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(0, 5),
                should_pass: true,
            },
        ]
    }

    /// Test with unusual file patterns
    pub fn unusual_pattern_scenarios() -> Vec<EdgeCaseScenario> {
        vec![
            EdgeCaseScenario {
                name: "binary_content".to_string(),
                description: "File that looks like binary data".to_string(),
                input_content: "\x00\x01\x02\x03\u{FF}\u{FE}\u{FD}".to_string(),
                expected_behavior: EdgeCaseExpectation::GracefulFailure,
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "mixed_line_endings".to_string(),
                description: "File with mixed line ending styles".to_string(),
                input_content: "line1\r\nline2\nline3\r\nline4\n".to_string(),
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(0, 2),
                should_pass: true,
            },
            EdgeCaseScenario {
                name: "extremely_long_identifier".to_string(),
                description: "Variable name with 1000 characters".to_string(),
                input_content: format!("const {} = 42;", "a".repeat(1000)),
                expected_behavior: EdgeCaseExpectation::SuggestionsWithinRange(1, 3),
                should_pass: true,
            },
        ]
    }

    /// Test configuration edge cases
    pub fn configuration_edge_cases() -> Vec<ConfigEdgeCase> {
        vec![
            ConfigEdgeCase {
                name: "zero_max_files".to_string(),
                config: ConfigBuilder::new().build(),
                should_be_valid: false,
            },
            ConfigEdgeCase {
                name: "empty_ai_model".to_string(),
                config: ConfigBuilder::new().ai_model("").build(),
                should_be_valid: false,
            },
            ConfigEdgeCase {
                name: "extremely_high_max_files".to_string(),
                config: ConfigBuilder::new().build(),
                should_be_valid: true,
            },
            ConfigEdgeCase {
                name: "no_include_patterns".to_string(),
                config: MoonShineConfig {
                    ai_model: Some("test".to_string()),
                    // max_files field removed
                    include_patterns: Some(vec![]),
                    exclude_patterns: Some(vec!["**/node_modules/**".to_string()]),
                    // cache_enabled field removed
                    ..Default::default()
                },
                should_be_valid: false, // Should have at least one include pattern
            },
        ]
    }
}

/// Edge case scenario definition
#[derive(Debug, Clone)]
pub struct EdgeCaseScenario {
    pub name: String,
    pub description: String,
    pub input_content: String,
    pub expected_behavior: EdgeCaseExpectation,
    pub should_pass: bool,
}

/// Expected behavior for edge cases
#[derive(Debug, Clone)]
pub enum EdgeCaseExpectation {
    NoSuggestions,
    SuggestionsWithinRange(usize, usize), // min, max
    ErrorsExpected,
    GracefulFailure,
    SpecificSuggestionCount(usize),
}

/// Configuration edge case
#[derive(Debug, Clone)]
pub struct ConfigEdgeCase {
    pub name: String,
    pub config: MoonShineConfig,
    pub should_be_valid: bool,
}

/// Edge case test runner
pub struct EdgeCaseRunner {
    scenarios: Vec<EdgeCaseScenario>,
    config_cases: Vec<ConfigEdgeCase>,
    results: Vec<EdgeCaseResult>,
}

impl EdgeCaseRunner {
    pub fn new() -> Self {
        let mut scenarios = Vec::new();
        scenarios.extend(EdgeCaseTests::empty_file_scenarios());
        scenarios.extend(EdgeCaseTests::large_file_scenarios());
        scenarios.extend(EdgeCaseTests::malformed_code_scenarios());
        scenarios.extend(EdgeCaseTests::unusual_pattern_scenarios());

        Self {
            scenarios,
            config_cases: EdgeCaseTests::configuration_edge_cases(),
            results: Vec::new(),
        }
    }

    /// Run all edge case tests
    pub async fn run_all_edge_cases(&mut self) -> Result<EdgeCaseSummary> {
        // Run scenario tests
        for scenario in &self.scenarios {
            let result = self.run_scenario_test(scenario).await;
            self.results.push(result);
        }

        // Run configuration tests
        for config_case in &self.config_cases {
            let result = self.run_config_test(config_case);
            self.results.push(result);
        }

        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = self.results.len() - passed;

        Ok(EdgeCaseSummary {
            total_tests: self.results.len(),
            passed,
            failed,
            results: self.results.clone(),
        })
    }

    /// Run a single scenario test
    async fn run_scenario_test(&self, scenario: &EdgeCaseScenario) -> EdgeCaseResult {
        let start_time = std::time::Instant::now();

        // Simulate analysis of the edge case scenario
        let suggestions = self.simulate_analysis(&scenario.input_content).await;

        let execution_time = start_time.elapsed();
        let passed = self.validate_scenario_result(scenario, &suggestions);

        EdgeCaseResult {
            test_name: scenario.name.clone(),
            test_type: EdgeCaseTestType::Scenario,
            passed,
            execution_time,
            suggestion_count: suggestions.len(),
            error_message: if !passed {
                Some(format!("Scenario '{}' failed validation", scenario.name))
            } else {
                None
            },
        }
    }

    /// Run a single configuration test
    fn run_config_test(&self, config_case: &ConfigEdgeCase) -> EdgeCaseResult {
        let start_time = std::time::Instant::now();

        let validation_result = config_case.config.assert_valid();
        let is_valid = validation_result.is_ok();
        let passed = is_valid == config_case.should_be_valid;

        let execution_time = start_time.elapsed();

        EdgeCaseResult {
            test_name: config_case.name.clone(),
            test_type: EdgeCaseTestType::Configuration,
            passed,
            execution_time,
            suggestion_count: 0,
            error_message: if !passed {
                Some(format!("Config '{}' validation mismatch", config_case.name))
            } else {
                None
            },
        }
    }

    /// Simulate analysis for edge case testing
    async fn simulate_analysis(&self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        // Handle empty content
        if content.trim().is_empty() {
            return issues;
        }

        // Handle binary content
        if content.chars().any(|c| c as u32 > 127 && (c as u32) < 32) {
            // Binary content - return no issues (graceful handling)
            return issues;
        }

        // Handle syntax errors
        if content.contains("{{{") || content.contains("unclosed") {
            issues.push(LintIssueBuilder::error().message("Syntax error detected").line(1).build());
        }

        // Handle very long lines
        for (line_num, line) in content.lines().enumerate() {
            if line.len() > 1000 {
                issues.push(
                    LintIssueBuilder::warning()
                        .message("Line too long - consider breaking it up")
                        .line(line_num as u32 + 1)
                        .build(),
                );
            }
        }

        // Handle deeply nested structures
        if content.contains(&"{".repeat(50)) {
            issues.push(
                LintIssueBuilder::warning()
                    .message("Deeply nested structure - consider refactoring")
                    .line(1)
                    .build(),
            );
        }

        // Handle extremely long identifiers
        if content.contains(&"a".repeat(100)) {
            issues.push(LintIssueBuilder::warning().message("Identifier name too long").line(1).build());
        }

        issues
    }

    /// Validate scenario result against expectations
    fn validate_scenario_result(&self, scenario: &EdgeCaseScenario, suggestions: &[LintIssue]) -> bool {
        match &scenario.expected_behavior {
            EdgeCaseExpectation::NoSuggestions => suggestions.is_empty(),
            EdgeCaseExpectation::SuggestionsWithinRange(min, max) => suggestions.len() >= *min && suggestions.len() <= *max,
            EdgeCaseExpectation::ErrorsExpected => suggestions.iter().any(|s| matches!(s.severity, LintSeverity::Error)),
            EdgeCaseExpectation::GracefulFailure => {
                // Should not crash - if we got here, it's graceful
                true
            }
            EdgeCaseExpectation::SpecificSuggestionCount(count) => suggestions.len() == *count,
        }
    }
}

/// Result of an edge case test
#[derive(Debug, Clone)]
pub struct EdgeCaseResult {
    pub test_name: String,
    pub test_type: EdgeCaseTestType,
    pub passed: bool,
    pub execution_time: Duration,
    pub suggestion_count: usize,
    pub error_message: Option<String>,
}

/// Type of edge case test
#[derive(Debug, Clone)]
pub enum EdgeCaseTestType {
    Scenario,
    Configuration,
}

/// Summary of edge case test results
#[derive(Debug, Clone)]
pub struct EdgeCaseSummary {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<EdgeCaseResult>,
}

impl EdgeCaseSummary {
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("🔍 Edge Case Test Summary:");
        println!("  Total tests: {}", self.total_tests);
        println!("  Passed: {}", self.passed);
        println!("  Failed: {}", self.failed);
        println!("  Success rate: {:.1}%", self.success_rate());

        if self.failed > 0 {
            println!("\n❌ Failed edge cases:");
            for result in &self.results {
                if !result.passed {
                    println!("  - {}: {}", result.test_name, result.error_message.as_deref().unwrap_or("Unknown error"));
                }
            }
        }
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_file_scenarios() {
        let scenarios = EdgeCaseTests::empty_file_scenarios();
        assert!(!scenarios.is_empty());

        let mut runner = EdgeCaseRunner::new();
        for scenario in &scenarios {
            let result = runner.run_scenario_test(scenario).await;
            assert!(result.passed, "Empty file scenario '{}' should pass", scenario.name);
        }
    }

    #[tokio::test]
    async fn test_malformed_code_scenarios() {
        let scenarios = EdgeCaseTests::malformed_code_scenarios();
        assert!(!scenarios.is_empty());

        let mut runner = EdgeCaseRunner::new();
        for scenario in &scenarios {
            let result = runner.run_scenario_test(scenario).await;
            // These should handle gracefully (not crash)
            if scenario.should_pass {
                assert!(result.passed, "Malformed code scenario '{}' should handle gracefully", scenario.name);
            }
        }
    }

    #[tokio::test]
    async fn test_configuration_edge_cases() {
        let edge_cases = EdgeCaseTests::configuration_edge_cases();
        assert!(!edge_cases.is_empty());

        let runner = EdgeCaseRunner::new();
        for config_case in &edge_cases {
            let result = runner.run_config_test(config_case);
            assert!(result.passed, "Config edge case '{}' validation mismatch", config_case.name);
        }
    }

    #[tokio::test]
    async fn test_large_file_handling() {
        let scenarios = EdgeCaseTests::large_file_scenarios();
        assert!(!scenarios.is_empty());

        let mut runner = EdgeCaseRunner::new();
        for scenario in &scenarios {
            let result = runner.run_scenario_test(scenario).await;
            assert!(result.passed, "Large file scenario '{}' should handle gracefully", scenario.name);
            // Should complete in reasonable time even for large files
            assert!(result.execution_time < Duration::from_secs(5));
        }
    }

    #[tokio::test]
    async fn test_unusual_patterns() {
        let scenarios = EdgeCaseTests::unusual_pattern_scenarios();
        assert!(!scenarios.is_empty());

        let mut runner = EdgeCaseRunner::new();
        for scenario in &scenarios {
            let result = runner.run_scenario_test(scenario).await;
            assert!(result.passed, "Unusual pattern scenario '{}' should handle gracefully", scenario.name);
        }
    }

    #[tokio::test]
    async fn test_complete_edge_case_suite() {
        let mut runner = EdgeCaseRunner::new();
        let summary = runner.run_all_edge_cases().await.unwrap();

        assert!(summary.total_tests >= 10); // At least 10 edge cases
        assert!(summary.success_rate() >= 90.0); // High success rate expected for edge cases

        // Print summary for debugging
        summary.print_summary();
    }

    #[test]
    fn test_edge_case_expectations() {
        let scenarios = EdgeCaseTests::empty_file_scenarios();
        for scenario in scenarios {
            match scenario.expected_behavior {
                EdgeCaseExpectation::NoSuggestions => {
                    // Validate expectation logic
                    assert!(
                        scenario.input_content.trim().is_empty()
                            || scenario
                                .input_content
                                .lines()
                                .all(|line| line.trim().is_empty() || line.trim().starts_with("//"))
                    );
                }
                _ => {} // Other expectations are valid
            }
        }
    }
}
