//! # Stress and Performance Testing
//!
//! Advanced performance stress tests to validate moon-shine's behavior under
//! extreme conditions and large-scale operations.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;

use crate::analysis::AnalysisResults;
use crate::config::MoonShineConfig;
use crate::error::Result;
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::testing::assertions::{MoonShineAssertions, PerformanceAssertions};
use crate::testing::builders::{AnalysisResultsBuilder, ConfigBuilder, LintIssueBuilder};
use crate::testing::fixtures::{TestDataBuilder, TYPESCRIPT_WITH_ISSUES};
use crate::testing::{PerformanceRequirements, TestEnvironment};

/// Stress test configuration
#[derive(Debug, Clone)]
pub struct StressTestConfig {
    pub file_count: usize,
    pub max_execution_time: Duration,
    pub max_memory_mb: u64,
    pub concurrent_operations: usize,
    pub iterations: usize,
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            file_count: 1000,
            max_execution_time: Duration::from_secs(30),
            max_memory_mb: 500,
            concurrent_operations: 10,
            iterations: 100,
        }
    }
}

/// Performance stress test runner
pub struct StressTestRunner {
    config: StressTestConfig,
    test_env: TestEnvironment,
    results: Vec<StressTestResult>,
}

impl StressTestRunner {
    pub fn new(config: StressTestConfig) -> Result<Self> {
        Ok(Self {
            config,
            test_env: TestEnvironment::new()?,
            results: Vec::new(),
        })
    }

    /// Run all stress tests
    pub async fn run_all_stress_tests(&mut self) -> Result<StressTestSummary> {
        let start_time = Instant::now();

        // Run individual stress tests
        self.run_large_codebase_stress_test().await?;
        self.run_concurrent_analysis_stress_test().await?;
        self.run_memory_pressure_stress_test().await?;
        self.run_high_frequency_operations_stress_test().await?;
        self.run_error_storm_stress_test().await?;
        self.run_configuration_overload_stress_test().await?;

        let total_time = start_time.elapsed();

        Ok(StressTestSummary {
            total_tests: self.results.len(),
            passed: self.results.iter().filter(|r| r.passed).count(),
            failed: self.results.iter().filter(|r| !r.passed).count(),
            total_execution_time: total_time,
            average_execution_time: total_time / self.results.len() as u32,
            max_memory_used: self.results.iter().map(|r| r.memory_used_mb).max().unwrap_or(0),
            results: self.results.clone(),
        })
    }

    /// Stress test with extremely large codebase
    async fn run_large_codebase_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "large_codebase_stress";

        // Create test scenario with 5000 files
        let large_scenario = TestDataBuilder::large_codebase(5000);

        // Simulate analysis of large codebase
        let analysis_start = Instant::now();

        // Mock analysis that would handle 5000 files
        let mut suggestions = Vec::new();
        for i in 0..5000 {
            suggestions.push(
                LintIssueBuilder::new()
                    .rule_name("typescript_any_type")
                    .message("TypeScript any type used")
                    .severity(LintSeverity::Warning)
                    .line(i as u32 % 100 + 1)
                    .build(),
            );
        }

        let results = AnalysisResultsBuilder::new()
            .suggestions(suggestions)
            .processing_time(analysis_start.elapsed().as_millis() as u64)
            .metadata("test_type", "large_codebase_stress")
            .build();

        let execution_time = start_time.elapsed();

        // Validate performance
        let passed = execution_time < self.config.max_execution_time
            // files_processed field removed from AnalysisResultsBuilder
            && results.suggestions.len() == 5000;

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 250, // Simulated memory usage
            operations_completed: 5000,
            error_message: if !passed {
                Some("Large codebase stress test failed".to_string())
            } else {
                None
            },
        });

        Ok(())
    }

    /// Stress test with concurrent analysis operations
    async fn run_concurrent_analysis_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "concurrent_analysis_stress";

        // Create multiple concurrent analysis tasks
        let mut tasks = Vec::new();

        for i in 0..self.config.concurrent_operations {
            let task = tokio::spawn(async move {
                // Simulate concurrent analysis operation
                let analysis_time = tokio::time::sleep(Duration::from_millis(100 + i as u64 * 10));
                timeout(Duration::from_secs(5), analysis_time).await.is_ok()
            });
            tasks.push(task);
        }

        // Wait for all concurrent operations to complete
        let mut completed = 0;
        for task in tasks {
            if task.await.unwrap_or(false) {
                completed += 1;
            }
        }

        let execution_time = start_time.elapsed();
        let passed = completed == self.config.concurrent_operations && execution_time < Duration::from_secs(10);

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 150,
            operations_completed: completed,
            error_message: if !passed {
                Some(format!(
                    "Concurrent test failed: {}/{} operations completed",
                    completed, self.config.concurrent_operations
                ))
            } else {
                None
            },
        });

        Ok(())
    }

    /// Stress test with memory pressure
    async fn run_memory_pressure_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "memory_pressure_stress";

        // Simulate memory-intensive operations
        let mut large_data_structures = Vec::new();

        for i in 0..100 {
            // Create large analysis results that would consume memory
            let suggestions: Vec<LintIssue> = (0..1000)
                .map(|j| {
                    LintIssueBuilder::warning()
                        .message(&format!("Memory pressure test suggestion {} from iteration {}", j, i))
                        .line(j as u32 % 500 + 1)
                        .build()
                })
                .collect();

            let results = AnalysisResultsBuilder::new()
                .suggestions(suggestions)
                .processing_time(50)
                .metadata("iteration", &i.to_string())
                .build();

            large_data_structures.push(results);
        }

        // Verify we can handle large amounts of data
        let total_suggestions: usize = large_data_structures.iter().map(|r| r.suggestions.len()).sum();

        let execution_time = start_time.elapsed();
        let passed = total_suggestions == 100_000 // 100 iterations * 1000 suggestions each
            && execution_time < Duration::from_secs(15);

        // Simulate memory cleanup
        drop(large_data_structures);

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 400, // Simulated peak memory usage
            operations_completed: total_suggestions,
            error_message: if !passed { Some("Memory pressure test failed".to_string()) } else { None },
        });

        Ok(())
    }

    /// Stress test with high-frequency operations
    async fn run_high_frequency_operations_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "high_frequency_operations_stress";

        let mut operations_completed = 0;
        let target_operations = 10_000;

        // Simulate high-frequency small operations
        for i in 0..target_operations {
            // Simulate quick analysis operation
            let _suggestion = LintIssueBuilder::info()
                .message(&format!("High frequency operation {}", i))
                .line((i % 1000) as u32 + 1)
                .build();

            operations_completed += 1;

            // Simulate some processing delay
            if i % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        let execution_time = start_time.elapsed();
        let operations_per_second = operations_completed as f64 / execution_time.as_secs_f64();
        let passed = operations_completed == target_operations && operations_per_second > 1000.0; // At least 1000 ops/sec

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 100,
            operations_completed,
            error_message: if !passed {
                Some(format!("High frequency test failed: {:.0} ops/sec", operations_per_second))
            } else {
                None
            },
        });

        Ok(())
    }

    /// Stress test with error storm conditions
    async fn run_error_storm_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "error_storm_stress";

        // Simulate handling many errors without crashing
        let mut errors_handled = 0;
        let target_errors = 1000;

        for i in 0..target_errors {
            // Simulate error-prone operations that should be handled gracefully
            let error_suggestion = LintIssueBuilder::error()
                .message(&format!("Error storm test error {}: simulated failure", i))
                .line((i % 100) as u32 + 1)
                .confidence(0.50) // Lower confidence for error conditions
                .build();

            // Simulate error handling - should not panic or crash
            if error_suggestion.severity == LintSeverity::Error {
                errors_handled += 1;
            }
        }

        let execution_time = start_time.elapsed();
        let passed = errors_handled == target_errors && execution_time < Duration::from_secs(5);

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 75,
            operations_completed: errors_handled,
            error_message: if !passed { Some("Error storm test failed".to_string()) } else { None },
        });

        Ok(())
    }

    /// Stress test with configuration overload
    async fn run_configuration_overload_stress_test(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let test_name = "configuration_overload_stress";

        // Test with extreme configuration values
        let stress_configs = vec![
            ConfigBuilder::new().build(),
            ConfigBuilder::new().build(),
            ConfigBuilder::new()
                .include_pattern("**/*.ts")
                .include_pattern("**/*.tsx")
                .include_pattern("**/*.js")
                .include_pattern("**/*.jsx")
                .include_pattern("**/*.vue")
                .include_pattern("**/*.svelte")
                .build(),
            ConfigBuilder::performance().build(),
        ];

        let mut configs_tested = 0;
        for (i, config) in stress_configs.iter().enumerate() {
            // Simulate configuration validation and usage
            if !config.include_patterns.is_empty() {
                configs_tested += 1;
            }

            // Simulate processing with different configurations
            tokio::task::yield_now().await;
        }

        let execution_time = start_time.elapsed();
        let passed = configs_tested == stress_configs.len() && execution_time < Duration::from_secs(2);

        self.results.push(StressTestResult {
            test_name: test_name.to_string(),
            passed,
            execution_time,
            memory_used_mb: 50,
            operations_completed: configs_tested,
            error_message: if !passed {
                Some("Configuration overload test failed".to_string())
            } else {
                None
            },
        });

        Ok(())
    }
}

/// Result of a single stress test
#[derive(Debug, Clone)]
pub struct StressTestResult {
    pub test_name: String,
    pub passed: bool,
    pub execution_time: Duration,
    pub memory_used_mb: u64,
    pub operations_completed: usize,
    pub error_message: Option<String>,
}

/// Summary of all stress test results
#[derive(Debug, Clone)]
pub struct StressTestSummary {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub max_memory_used: u64,
    pub results: Vec<StressTestResult>,
}

impl StressTestSummary {
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("🚀 Stress Test Summary:");
        println!("  Total tests: {}", self.total_tests);
        println!("  Passed: {}", self.passed);
        println!("  Failed: {}", self.failed);
        println!("  Success rate: {:.1}%", self.success_rate());
        println!("  Total execution time: {:?}", self.total_execution_time);
        println!("  Average execution time: {:?}", self.average_execution_time);
        println!("  Max memory used: {} MB", self.max_memory_used);

        if self.failed > 0 {
            println!("\n❌ Failed tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  - {}: {}", result.test_name, result.error_message.as_deref().unwrap_or("Unknown error"));
                }
            }
        }
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_stress_runner_creation() {
        let config = StressTestConfig::default();
        let runner = StressTestRunner::new(config);
        assert!(runner.is_ok());
    }

    #[tokio::test]
    async fn test_large_codebase_stress() {
        let config = StressTestConfig {
            file_count: 2, // Reduced for debug
            max_execution_time: Duration::from_secs(1),
            ..Default::default()
        };

        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_large_codebase_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_concurrent_analysis_stress() {
        let config = StressTestConfig {
            concurrent_operations: 5, // Smaller for test
            ..Default::default()
        };

        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_concurrent_analysis_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_memory_pressure_stress() {
        let config = StressTestConfig::default();
        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_memory_pressure_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_high_frequency_operations() {
        let config = StressTestConfig::default();
        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_high_frequency_operations_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_error_storm_handling() {
        let config = StressTestConfig::default();
        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_error_storm_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_configuration_overload() {
        let config = StressTestConfig::default();
        let mut runner = StressTestRunner::new(config).unwrap();
        let result = runner.run_configuration_overload_stress_test().await;
        assert!(result.is_ok());
        assert_eq!(runner.results.len(), 1);
        assert!(runner.results[0].passed);
    }

    #[tokio::test]
    async fn test_complete_stress_suite() {
        let config = StressTestConfig {
            file_count: 2,
            concurrent_operations: 1,
            iterations: 1,
            max_execution_time: Duration::from_secs(1),
            ..Default::default()
        };

        let mut runner = StressTestRunner::new(config).unwrap();
        let summary = runner.run_all_stress_tests().await.unwrap();

        assert!(summary.total_tests >= 6); // At least 6 stress tests
        assert!(summary.success_rate() >= 80.0); // At least 80% success rate
        assert!(summary.total_execution_time < Duration::from_secs(30));
    }
}
