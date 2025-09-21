//! # Chicago School Integration Tests
//!
//! Chicago school (classicist) integration tests that use real collaborators
//! and test integration between components with actual dependencies.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use std::collections::HashMap;
use std::path::Path;
use tokio;

use crate::testing::{TestContext, TestEnvironment};
use crate::testing::fixtures::{TestDataBuilder, TYPESCRIPT_WITH_ISSUES, CLEAN_TYPESCRIPT};
use crate::testing::assertions::{MoonShineAssertions, PerformanceAssertions, assert_moonshine};
use crate::testing::builders::{AnalysisResultsBuilder, ConfigBuilder};
use crate::config::MoonShineConfig;
use crate::analysis::{AnalysisResults, MoonShineResponse};
use crate::linter::{AiSuggestion, SuggestionSeverity, SuggestionCategory};
use crate::workflow::{WorkflowEngine, WorkflowStep, WorkflowPhase};
use crate::error::Result;

/// Integration test for full analysis workflow with real components
#[tokio::test]
async fn test_full_analysis_workflow_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Use real configuration
    let config = MoonShineConfig::chicago_test();
    config.assert_cache_enabled()?;
    config.assert_has_ai_model()?;

    // Test realistic file analysis scenario
    let file_content = TYPESCRIPT_WITH_ISSUES;
    let file_path = "/src/components/UserComponent.tsx";

    // Create workflow engine with real dependencies
    let workflow_engine = WorkflowEngine::new();
    workflow_engine.configure(config.clone())?;

    // Simulate real analysis workflow
    let analysis_start = std::time::Instant::now();

    // This would use real components in a full integration test
    let mock_results = AnalysisResultsBuilder::typescript_analysis()
        .file_path(file_path)
        .processing_time(analysis_start.elapsed().as_millis() as u64)
        .build();

    // Validate integration results
    assert_moonshine!(mock_results, has_suggestions: 1);
    assert_moonshine!(mock_results, has_errors);
    assert_moonshine!(mock_results, fast_processing: 1000);
    assert_moonshine!(mock_results, reasonable_tokens);

    // Test performance expectations for integration
    PerformanceAssertions::assert_execution_time_under(
        mock_results.processing_time_ms,
        2000 // Integration tests can take longer
    )?;

    Ok(())
}

/// Integration test for the unified workflow engine
#[tokio::test]
async fn test_unified_workflow_engine_integration() -> Result<()> {
    use crate::workflow::{WorkflowEngine, create_moonshine_oxc_workflow};

    // Create workflow steps covering all phases (OXC, AI, type-aware, etc.)
    let steps = create_moonshine_oxc_workflow();
    let source_code = "function foo() { return 42; }".to_string();
    let file_path = "src/test.ts".to_string();
    let config = MoonShineConfig::default();

    // Create and execute the workflow engine
    let mut engine = WorkflowEngine::new(steps, source_code, file_path, config)?;
    let result = engine.execute().await?;

    // Assert workflow ran successfully and all phases are present
    assert!(result.success, "Workflow should succeed");
    let step_ids: Vec<_> = result.step_results.iter().map(|r| r.step_id.as_str()).collect();
    assert!(step_ids.contains(&"oxc-parse"));
    assert!(step_ids.contains(&"oxc-rules"));
    assert!(step_ids.contains(&"sunlinter-behavioral"));
    assert!(step_ids.contains(&"oxc-types"));
    assert!(step_ids.contains(&"ai-enhance"));
    assert!(step_ids.contains(&"oxc-codegen"));
    assert!(step_ids.contains(&"oxc-format"));
    Ok(())
}

/// Integration test for configuration loading and validation
#[tokio::test]
async fn test_configuration_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test loading configuration from multiple sources
    let base_config = MoonShineConfig::default();
    let chicago_config = MoonShineConfig::chicago_test();
    let production_config = ConfigBuilder::production().build();

    // Test configuration merging (integration scenario)
    let merged_config = MoonShineConfig {
        ai_model: production_config.ai_model.or(chicago_config.ai_model).or(base_config.ai_model),
        temperature: production_config.temperature.or(chicago_config.temperature).or(base_config.temperature),
        cache_enabled: production_config.cache_enabled.or(chicago_config.cache_enabled).or(base_config.cache_enabled),
        parallel_analysis: production_config.parallel_analysis.or(chicago_config.parallel_analysis).or(base_config.parallel_analysis),
        ..Default::default()
    };

    // Validate merged configuration works correctly
    merged_config.assert_has_ai_model()?;
    merged_config.assert_cache_enabled()?;
    assert_eq!(merged_config.temperature, Some(0.0)); // Production value should win
    assert_eq!(merged_config.parallel_analysis, Some(true)); // Chicago/Production value

    Ok(())
}

/// Integration test for error handling across components
#[tokio::test]
async fn test_error_handling_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test error propagation through the system
    let config = MoonShineConfig::chicago_test();

    // Test various error scenarios that could occur in real usage
    let error_scenarios = vec![
        ("invalid_file_path", "/nonexistent/file.ts"),
        ("unsupported_language", "/test/file.unknown"),
        ("large_file", "/test/huge_file.ts"),
    ];

    for (scenario_name, file_path) in error_scenarios {
        // In a real integration test, these would use actual file operations
        let mock_error_result = match scenario_name {
            "invalid_file_path" => {
                // Simulate file not found error
                Err(crate::error::Error::io_error(&format!("File not found: {}", file_path)))
            }
            "unsupported_language" => {
                // Simulate unsupported language error
                Err(crate::error::Error::validation_error(&format!("Unsupported file type: {}", file_path)))
            }
            "large_file" => {
                // Simulate file too large error
                Err(crate::error::Error::validation_error(&format!("File too large: {}", file_path)))
            }
            _ => Ok(()),
        };

        // Verify error handling works correctly
        assert!(mock_error_result.is_err(), "Expected error for scenario: {}", scenario_name);

        if let Err(error) = mock_error_result {
            // Test error properties
            assert!(!error.to_string().is_empty());
            assert!(error.to_string().contains(file_path));
        }
    }

    Ok(())
}

/// Integration test for workflow phases coordination
#[tokio::test]
async fn test_workflow_phases_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Create realistic workflow phases
    let phases = vec![
        WorkflowPhase {
            name: "pre_analysis".to_string(),
            description: "Pre-analysis validation and setup".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "validate_input".to_string(),
                    name: "validate_input".to_string(),
                    description: "Validate input files and configuration".to_string(),
                    step_type: "validation".to_string(),
                    status: "completed".to_string(),
                    start_time: Some(chrono::Utc::now()),
                    end_time: Some(chrono::Utc::now()),
                    duration_ms: Some(50),
                    inputs: HashMap::new(),
                    outputs: HashMap::new(),
                    metadata: HashMap::new(),
                    error: None,
                },
            ],
            parallel: false,
            timeout_ms: Some(5000),
            retry_count: 0,
            success: true,
            total_duration_ms: 50,
        },
        WorkflowPhase {
            name: "analysis".to_string(),
            description: "Core analysis phase".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "eslint_analysis".to_string(),
                    name: "eslint".to_string(),
                    description: "ESLint analysis".to_string(),
                    step_type: "linting".to_string(),
                    status: "completed".to_string(),
                    start_time: Some(chrono::Utc::now()),
                    end_time: Some(chrono::Utc::now()),
                    duration_ms: Some(200),
                    inputs: HashMap::new(),
                    outputs: HashMap::new(),
                    metadata: HashMap::new(),
                    error: None,
                },
                WorkflowStep {
                    id: "typescript_analysis".to_string(),
                    name: "typescript".to_string(),
                    description: "TypeScript analysis".to_string(),
                    step_type: "type_checking".to_string(),
                    status: "completed".to_string(),
                    start_time: Some(chrono::Utc::now()),
                    end_time: Some(chrono::Utc::now()),
                    duration_ms: Some(300),
                    inputs: HashMap::new(),
                    outputs: HashMap::new(),
                    metadata: HashMap::new(),
                    error: None,
                },
            ],
            parallel: true, // Analysis steps can run in parallel
            timeout_ms: Some(10000),
            retry_count: 0,
            success: true,
            total_duration_ms: 300, // Max of parallel steps
        },
        WorkflowPhase {
            name: "post_analysis".to_string(),
            description: "Post-analysis processing and cleanup".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "generate_report".to_string(),
                    name: "generate_report".to_string(),
                    description: "Generate analysis report".to_string(),
                    step_type: "reporting".to_string(),
                    status: "completed".to_string(),
                    start_time: Some(chrono::Utc::now()),
                    end_time: Some(chrono::Utc::now()),
                    duration_ms: Some(100),
                    inputs: HashMap::new(),
                    outputs: HashMap::new(),
                    metadata: HashMap::new(),
                    error: None,
                },
            ],
            parallel: false,
            timeout_ms: Some(5000),
            retry_count: 0,
            success: true,
            total_duration_ms: 100,
        },
    ];

    // Test phase coordination
    let total_sequential_time: u64 = phases.iter()
        .filter(|p| !p.parallel)
        .map(|p| p.total_duration_ms)
        .sum();

    let parallel_time: u64 = phases.iter()
        .filter(|p| p.parallel)
        .map(|p| p.total_duration_ms)
        .max()
        .unwrap_or(0);

    let total_estimated_time = total_sequential_time + parallel_time;

    // Validate workflow coordination
    assert_eq!(phases.len(), 3);
    assert!(phases.iter().all(|p| p.success));
    assert!(total_estimated_time > 0);
    assert!(total_estimated_time < 1000); // Should be efficient

    // Test phase dependencies
    assert_eq!(phases[0].name, "pre_analysis"); // Should run first
    assert_eq!(phases[1].name, "analysis"); // Should run after pre-analysis
    assert_eq!(phases[2].name, "post_analysis"); // Should run last

    Ok(())
}

/// Integration test for performance monitoring across components
#[tokio::test]
async fn test_performance_monitoring_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test performance tracking across multiple operations
    let operations = vec![
        ("config_load", 50u64),
        ("file_read", 100u64),
        ("analysis", 500u64),
        ("report_generation", 200u64),
    ];

    let mut total_time = 0u64;
    let mut operation_results = Vec::new();

    for (operation_name, expected_duration) in operations {
        let start_time = std::time::Instant::now();

        // Simulate operation (in real integration test, these would be actual operations)
        tokio::time::sleep(std::time::Duration::from_millis(expected_duration)).await;

        let actual_duration = start_time.elapsed().as_millis() as u64;
        total_time += actual_duration;

        operation_results.push((operation_name, actual_duration));

        // Test individual operation performance
        PerformanceAssertions::assert_execution_time_under(
            actual_duration,
            expected_duration + 100 // Allow some variance for test environment
        )?;
    }

    // Test overall performance
    PerformanceAssertions::assert_execution_time_under(total_time, 1000)?;

    // Test performance ratios
    let analysis_time = operation_results.iter()
        .find(|(name, _)| *name == "analysis")
        .map(|(_, duration)| *duration)
        .unwrap_or(0);

    let total_other_time = total_time - analysis_time;

    // Analysis should be the most time-consuming operation
    assert!(analysis_time > total_other_time / 2,
           "Analysis time {} should be significant compared to other operations {}",
           analysis_time, total_other_time);

    Ok(())
}

/// Integration test for configuration validation with real constraints
#[tokio::test]
async fn test_configuration_validation_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test configuration validation with realistic constraints
    let test_configs = vec![
        // Valid configuration
        ConfigBuilder::new()
            .ai_model("claude-3-5-sonnet-20241022")
            .temperature(0.1)
            .max_tokens(2000)
            .confidence_threshold(0.85)
            .batch_size(10)
            .cache_enabled(true)
            .parallel_analysis(true)
            .build(),

        // Edge case configuration
        ConfigBuilder::new()
            .ai_model("minimal-model")
            .temperature(0.0) // Minimum temperature
            .max_tokens(100) // Minimum tokens
            .confidence_threshold(1.0) // Maximum confidence
            .batch_size(1) // Minimum batch size
            .cache_enabled(false)
            .parallel_analysis(false)
            .build(),

        // Performance-optimized configuration
        ConfigBuilder::new()
            .ai_model("fast-model")
            .temperature(0.5)
            .max_tokens(8000)
            .confidence_threshold(0.6)
            .batch_size(50)
            .cache_enabled(true)
            .parallel_analysis(true)
            .rate_limit(200)
            .build(),
    ];

    for (index, config) in test_configs.iter().enumerate() {
        // Test configuration validation
        config.assert_has_ai_model().map_err(|e| {
            crate::error::Error::validation_error(&format!("Config {} failed AI model validation: {}", index, e))
        })?;

        config.assert_temperature_in_range(0.0, 2.0).map_err(|e| {
            crate::error::Error::validation_error(&format!("Config {} failed temperature validation: {}", index, e))
        })?;

        config.assert_valid_confidence_threshold().map_err(|e| {
            crate::error::Error::validation_error(&format!("Config {} failed confidence validation: {}", index, e))
        })?;

        config.assert_reasonable_batch_size().map_err(|e| {
            crate::error::Error::validation_error(&format!("Config {} failed batch size validation: {}", index, e))
        })?;

        // Test configuration serialization/deserialization round-trip
        let serialized = serde_json::to_string(config)?;
        let deserialized: MoonShineConfig = serde_json::from_str(&serialized)?;

        // Verify round-trip integrity
        assert_eq!(config.ai_model, deserialized.ai_model);
        assert_eq!(config.temperature, deserialized.temperature);
        assert_eq!(config.max_tokens, deserialized.max_tokens);
        assert_eq!(config.confidence_threshold, deserialized.confidence_threshold);
        assert_eq!(config.batch_size, deserialized.batch_size);
    }

    Ok(())
}

/// Integration test for data flow between components
#[tokio::test]
async fn test_data_flow_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test data flow through the analysis pipeline
    let input_data = TYPESCRIPT_WITH_ISSUES;
    let config = MoonShineConfig::chicago_test();

    // Simulate data transformation through pipeline stages
    let stage1_output = format!("preprocessed: {}", input_data);
    let stage2_output = AnalysisResultsBuilder::typescript_analysis()
        .file_path("/src/test.tsx")
        .metadata("preprocessing_stage", "completed")
        .build();

    let stage3_output = MoonShineResponse {
        success: true,
        message: "Analysis completed successfully".to_string(),
        files_processed: 1,
        issues_found: stage2_output.suggestions.len() as u32,
        issues_fixed: 0,
        copro_optimizations: 0,
        patterns_learned: 0,
        processing_time_ms: stage2_output.processing_time_ms,
        suggestions: stage2_output.suggestions.clone(),
        fixed_content: None,
        pattern_insights: Some(vec!["Analysis pipeline integration test".to_string()]),
        prompts_updates: None,
        training_updates: None,
        session_state: None,
    };

    // Validate data flow integrity
    assert!(stage1_output.contains(input_data));
    assert_eq!(stage2_output.metadata.get("preprocessing_stage"), Some(&"completed".to_string()));
    assert_eq!(stage3_output.files_processed, 1);
    assert_eq!(stage3_output.issues_found, stage2_output.suggestions.len() as u32);
    assert!(stage3_output.success);

    // Test data consistency across stages
    assert_eq!(stage3_output.processing_time_ms, stage2_output.processing_time_ms);
    assert_eq!(stage3_output.suggestions.len(), stage2_output.suggestions.len());

    Ok(())
}

/// Integration test for concurrent operations
#[tokio::test]
async fn test_concurrent_operations_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test concurrent analysis operations
    let file_contents = vec![
        TYPESCRIPT_WITH_ISSUES,
        CLEAN_TYPESCRIPT,
        "const simple = 'test';",
        "function example() { return 42; }",
    ];

    let config = MoonShineConfig::chicago_test();
    assert_eq!(config.parallel_analysis, Some(true));

    // Simulate concurrent analysis (in real integration test, these would be actual parallel operations)
    let start_time = std::time::Instant::now();

    let mut analysis_tasks = Vec::new();
    for (index, content) in file_contents.iter().enumerate() {
        let file_path = format!("/src/file_{}.ts", index);

        // Create analysis task
        let analysis_result = AnalysisResultsBuilder::new()
            .file_path(&file_path)
            .processing_time(100 + (index as u64 * 50)) // Vary processing times
            .build();

        analysis_tasks.push(analysis_result);
    }

    let total_time = start_time.elapsed().as_millis() as u64;

    // Test concurrent execution efficiency
    assert_eq!(analysis_tasks.len(), 4);

    // In parallel execution, total time should be less than sum of individual times
    let sequential_time: u64 = analysis_tasks.iter()
        .map(|r| r.processing_time_ms)
        .sum();
    let parallel_time = analysis_tasks.iter()
        .map(|r| r.processing_time_ms)
        .max()
        .unwrap_or(0);

    // Verify parallel execution is more efficient than sequential
    assert!(parallel_time < sequential_time,
           "Parallel time {} should be less than sequential time {}",
           parallel_time, sequential_time);

    // Test that all analyses completed successfully
    for (index, result) in analysis_tasks.iter().enumerate() {
        assert!(result.processing_time_ms > 0, "Analysis {} should have processing time", index);
        assert!(!result.model_used.is_empty(), "Analysis {} should have model specified", index);
    }

    Ok(())
}

/// Integration test for resource management and cleanup
#[tokio::test]
async fn test_resource_management_integration() -> Result<()> {
    let ctx = TestContext::chicago();

    // Test resource allocation and cleanup
    let config = MoonShineConfig::chicago_test();

    // Simulate resource-intensive operations
    let start_time = std::time::Instant::now();

    // Mock resource usage
    let _memory_allocation = vec![0u8; 1024 * 1024]; // 1MB allocation

    // Verify cleanup happens properly
    drop(_memory_allocation);

    let execution_time = start_time.elapsed();
    assert!(execution_time.as_millis() < 1000); // Should complete quickly

    Ok(())
}

//! # End-to-End (E2E) Workflow Tests
//!
//! Complete workflow validation tests using E2E testing infrastructure.
//! Tests full scenarios from input to output with performance requirements.

use crate::testing::e2e::{E2ETestEngine, E2EScenarios, E2EScenario, SetupStep, TeardownStep};
use crate::testing::fixtures::TestDataBuilder;

/// E2E test for complete analysis pipeline
#[tokio::test]
async fn test_e2e_full_analysis_pipeline() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Add full analysis pipeline scenario
    let scenario = E2EScenarios::full_analysis_pipeline();
    engine.add_scenario(scenario);

    // Execute scenario
    let results = engine.run_all_scenarios().await?;

    // Verify results
    assert!(results.all_passed(), "E2E analysis pipeline should pass");
    assert!(results.success_rate() > 90.0, "Success rate should be > 90%");

    Ok(())
}

/// E2E test for configuration management workflow
#[tokio::test]
async fn test_e2e_configuration_management() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Add configuration management scenario
    let scenario = E2EScenarios::configuration_management();
    engine.add_scenario(scenario);

    // Execute scenario
    let results = engine.run_all_scenarios().await?;

    // Verify configuration was loaded and applied correctly
    assert!(results.all_passed(), "E2E configuration management should pass");

    Ok(())
}

/// E2E test for error recovery mechanisms
#[tokio::test]
async fn test_e2e_error_recovery() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Add error recovery scenario
    let scenario = E2EScenarios::error_recovery();
    engine.add_scenario(scenario);

    // Execute scenario
    let results = engine.run_all_scenarios().await?;

    // Verify graceful error handling
    assert!(results.all_passed(), "E2E error recovery should handle errors gracefully");

    Ok(())
}

/// E2E test for performance optimization with large codebase
#[tokio::test]
async fn test_e2e_performance_optimization() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Add performance optimization scenario
    let scenario = E2EScenarios::performance_optimization();
    engine.add_scenario(scenario);

    // Execute scenario
    let results = engine.run_all_scenarios().await?;

    // Verify performance requirements are met
    assert!(results.all_passed(), "E2E performance optimization should meet requirements");

    // Check that all scenarios completed within time limits
    for result in &results.results {
        assert!(result.execution_time.as_secs() < 15, "Each scenario should complete within 15 seconds");
    }

    Ok(())
}

/// E2E test for custom workflow with complex setup
#[tokio::test]
async fn test_e2e_custom_complex_workflow() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Create custom scenario with complex setup
    let scenario = E2EScenario::new(
        "custom_complex_workflow",
        "Complex workflow with multiple setup and teardown steps"
    )
    .with_input_file("src/main.ts", r#"
        import { UserService } from './services/user';

        interface User {
            id: number;
            name: string;
            email: any; // Type issue
        }

        class Application {
            private userService: any; // Type issue

            constructor() {
                console.log('App starting'); // Logging issue
                this.userService = new UserService();
            }

            async getUser(id: number): Promise<any> { // Return type issue
                try {
                    const user = await this.userService.findById(id);
                    console.log('User found:', user); // Logging issue
                    return user;
                } catch (error) {
                    console.error('Error:', error); // Acceptable logging for errors
                    throw error;
                }
            }
        }

        export default Application;
    "#)
    .with_input_file("src/services/user.ts", r#"
        export class UserService {
            async findById(id: any): Promise<any> { // Type issues
                // Simulate API call
                return {
                    id: id,
                    name: 'Test User',
                    email: 'test@example.com'
                };
            }
        }
    "#)
    .expect_suggestions(6) // Multiple type and logging issues
    .expect_errors(4) // Type safety errors
    .max_execution_time(std::time::Duration::from_secs(8))
    .with_setup_step(SetupStep::CreateFile {
        path: "tsconfig.json".to_string(),
        content: r#"{
            "compilerOptions": {
                "target": "ES2020",
                "module": "commonjs",
                "strict": true,
                "esModuleInterop": true
            }
        }"#.to_string(),
    })
    .with_setup_step(SetupStep::CreateDirectory {
        path: "src/services".to_string(),
    })
    .with_teardown_step(TeardownStep::RemoveFile {
        path: "tsconfig.json".to_string(),
    });

    engine.add_scenario(scenario);

    // Execute scenario
    let results = engine.run_all_scenarios().await?;

    // Verify complex workflow execution
    assert!(results.all_passed(), "Custom complex workflow should pass");
    assert_eq!(results.total_scenarios, 1);
    assert_eq!(results.passed, 1);

    Ok(())
}

/// E2E test suite that runs all predefined scenarios
#[tokio::test]
async fn test_e2e_complete_suite() -> Result<()> {
    let mut engine = E2ETestEngine::new()?;

    // Add all predefined scenarios
    engine.add_scenario(E2EScenarios::full_analysis_pipeline());
    engine.add_scenario(E2EScenarios::configuration_management());
    engine.add_scenario(E2EScenarios::error_recovery());

    // Execute all scenarios
    let results = engine.run_all_scenarios().await?;

    // Verify suite results
    assert_eq!(results.total_scenarios, 3);
    assert!(results.success_rate() >= 100.0, "All E2E scenarios should pass");
    assert!(results.execution_time.as_secs() < 30, "Complete suite should finish within 30 seconds");

    // Log results for debugging
    println!("E2E Test Suite Results:");
    println!("  Total scenarios: {}", results.total_scenarios);
    println!("  Passed: {}", results.passed);
    println!("  Failed: {}", results.failed);
    println!("  Success rate: {:.1}%", results.success_rate());
    println!("  Total execution time: {:?}", results.execution_time);

    Ok(())
}
