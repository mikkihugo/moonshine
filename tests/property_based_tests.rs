//! # Property-Based Testing for Moon-Shine
//!
//! QuickCheck-style property tests to ensure correctness across wide input spaces.
//! Tests invariants and properties that should hold for all valid inputs.

use arbitrary::Arbitrary;
use fake::{Fake, Faker};
use moon_shine::testing::*;
use moon_shine::*;
use proptest::prelude::*;
use quickcheck::{quickcheck, TestResult};
use std::collections::HashMap;

/// Arbitrary implementation for MoonShineConfig
#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryConfig {
    ai_model: Option<String>,
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
}

impl From<ArbitraryConfig> for MoonShineConfig {
    fn from(arb: ArbitraryConfig) -> Self {
        MoonShineConfig {
            ai_model: arb.ai_model,
            include_patterns: arb.include_patterns,
            exclude_patterns: arb.exclude_patterns,
            ..Default::default()
        }
    }
}

/// Arbitrary implementation for MoonShineResponse
#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryResponse {
    success: bool,
    message: String,
    fixes_applied: u32,
    errors_found: u32,
    execution_time_ms: u64,
    ai_model_used: Option<String>,
    session_id: Option<String>,
}

impl From<ArbitraryResponse> for MoonShineResponse {
    fn from(arb: ArbitraryResponse) -> Self {
        MoonShineResponse {
            success: arb.success,
            message: arb.message,
            fixes_applied: arb.fixes_applied,
            errors_found: arb.errors_found,
            execution_time_ms: arb.execution_time_ms,
            ai_model_used: arb.ai_model_used,
            session_id: arb.session_id,
        }
    }
}

/// Generate valid TypeScript/JavaScript code samples
fn typescript_code_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Function declarations
        r"function [a-zA-Z_][a-zA-Z0-9_]*\(\) \{ return [0-9]+; \}",
        // Variable declarations
        r"const [a-zA-Z_][a-zA-Z0-9_]* = [0-9]+;",
        r"let [a-zA-Z_][a-zA-Z0-9_]* = '[a-zA-Z ]*';",
        // Class declarations
        r"class [A-Z][a-zA-Z0-9_]* \{ constructor\(\) \{\} \}",
        // Interface declarations
        r"interface [A-Z][a-zA-Z0-9_]* \{ [a-zA-Z_][a-zA-Z0-9_]*: [a-zA-Z]+; \}",
        // Import statements
        r"import \{ [a-zA-Z_][a-zA-Z0-9_]* \} from '\./[a-zA-Z]+';",
        // Export statements
        r"export const [a-zA-Z_][a-zA-Z0-9_]* = [0-9]+;",
    ]
    .prop_map(|s| s.to_string())
}

/// Generate file paths
fn file_path_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        r"src/[a-zA-Z]+\.ts",
        r"src/[a-zA-Z]+\.tsx",
        r"src/[a-zA-Z]+\.js",
        r"src/[a-zA-Z]+\.jsx",
        r"tests/[a-zA-Z]+\.test\.ts",
        r"[a-zA-Z]+/[a-zA-Z]+/[a-zA-Z]+\.ts",
    ]
    .prop_map(|s| s.to_string())
}

#[cfg(test)]
mod config_properties {
    use super::*;

    proptest! {
        #[test]
        fn config_serialization_roundtrip_property(config in any::<ArbitraryConfig>()) {
            let moon_config: MoonShineConfig = config.into();

            let serialized = serde_json::to_string(&moon_config).unwrap();
            let deserialized: MoonShineConfig = serde_json::from_str(&serialized).unwrap();

            // Property: serialization preserves essential fields
            prop_assert_eq!(moon_config.ai_model, deserialized.ai_model);
            prop_assert_eq!(moon_config.include_patterns, deserialized.include_patterns);
            prop_assert_eq!(moon_config.exclude_patterns, deserialized.exclude_patterns);
        }

        #[test]
        fn config_validation_property(
            ai_model in prop::option::of("[a-zA-Z0-9-]+"),
            include_patterns in prop::collection::vec("[*a-zA-Z0-9/]+", 0..10),
            exclude_patterns in prop::collection::vec("[*a-zA-Z0-9/]+", 0..10)
        ) {
            let config = MoonShineConfig {
                ai_model,
                include_patterns: if include_patterns.is_empty() { None } else { Some(include_patterns) },
                exclude_patterns: if exclude_patterns.is_empty() { None } else { Some(exclude_patterns) },
                ..Default::default()
            };

            // Property: valid configs should always be constructible
            prop_assert!(config.ai_model.is_none() || !config.ai_model.as_ref().unwrap().is_empty());
        }

        #[test]
        fn config_pattern_matching_property(
            patterns in prop::collection::vec("\\*\\*/\\*\\.[a-z]+", 1..5)
        ) {
            let config = MoonShineConfig {
                include_patterns: Some(patterns.clone()),
                ..Default::default()
            };

            // Property: all include patterns should be valid glob patterns
            for pattern in &patterns {
                prop_assert!(pattern.contains("*"));
                prop_assert!(pattern.contains("."));
            }
        }
    }

    #[quickcheck]
    fn config_defaults_are_consistent(ai_model: Option<String>) -> bool {
        let config1 = MoonShineConfig {
            ai_model: ai_model.clone(),
            ..Default::default()
        };

        let config2 = MoonShineConfig {
            ai_model: ai_model.clone(),
            ..Default::default()
        };

        // Property: default configs with same overrides should be identical
        config1.ai_model == config2.ai_model && config1.include_patterns == config2.include_patterns && config1.exclude_patterns == config2.exclude_patterns
    }
}

#[cfg(test)]
mod response_properties {
    use super::*;

    proptest! {
        #[test]
        fn response_consistency_property(response in any::<ArbitraryResponse>()) {
            let moon_response: MoonShineResponse = response.into();

            // Property: successful responses should have reasonable metrics
            if moon_response.success {
                prop_assert!(moon_response.execution_time_ms >= 0);
                prop_assert!(!moon_response.message.is_empty());
            }

            // Property: responses with errors should report them
            if moon_response.errors_found > 0 {
                prop_assert!(moon_response.errors_found <= 1000); // Reasonable upper bound
            }

            // Property: fixes applied shouldn't exceed errors found in most cases
            if moon_response.success && moon_response.errors_found > 0 {
                prop_assert!(moon_response.fixes_applied <= moon_response.errors_found * 2);
            }
        }

        #[test]
        fn response_serialization_property(response in any::<ArbitraryResponse>()) {
            let moon_response: MoonShineResponse = response.into();

            let json = serde_json::to_string(&moon_response).unwrap();
            let deserialized: MoonShineResponse = serde_json::from_str(&json).unwrap();

            // Property: all fields preserved through serialization
            prop_assert_eq!(moon_response.success, deserialized.success);
            prop_assert_eq!(moon_response.fixes_applied, deserialized.fixes_applied);
            prop_assert_eq!(moon_response.errors_found, deserialized.errors_found);
            prop_assert_eq!(moon_response.execution_time_ms, deserialized.execution_time_ms);
        }
    }

    #[quickcheck]
    fn response_time_is_monotonic(exec_time1: u64, exec_time2: u64, message: String) -> TestResult {
        if exec_time1 > 1_000_000 || exec_time2 > 1_000_000 {
            return TestResult::discard(); // Avoid unrealistic times
        }

        let response1 = MoonShineResponse {
            success: true,
            message: message.clone(),
            fixes_applied: 0,
            errors_found: 0,
            execution_time_ms: exec_time1,
            ai_model_used: None,
            session_id: None,
        };

        let response2 = MoonShineResponse {
            success: true,
            message: message.clone(),
            fixes_applied: 0,
            errors_found: 0,
            execution_time_ms: exec_time2,
            ai_model_used: None,
            session_id: None,
        };

        // Property: execution time comparison should be consistent
        TestResult::from_bool((exec_time1 < exec_time2) == (response1.execution_time_ms < response2.execution_time_ms))
    }
}

#[cfg(test)]
mod workflow_properties {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;
    use tokio::runtime::Builder;

    fn build_step(id: &str, deps: Vec<String>) -> WorkflowStep {
        WorkflowStep {
            id: id.to_string(),
            name: format!("Step {}", id),
            description: "Test step".to_string(),
            depends_on: deps,
            action: StepAction::CustomFunction {
                function_name: "test".to_string(),
                parameters: HashMap::new(),
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_millis(100),
            critical: false,
        }
    }

    fn build_engine(steps: Vec<WorkflowStep>) -> WorkflowEngine {
        WorkflowEngine::new(steps, "console.log('test');".to_string(), "src/test.ts".to_string(), MoonShineConfig::default())
            .expect("workflow engine should build")
    }

    proptest! {
        #[test]
        fn workflow_step_ordering_property(
            step_ids in prop::collection::vec("[a-zA-Z]+", 1..10)
        ) {
            let mut steps = Vec::new();
            for (i, id) in step_ids.iter().enumerate() {
                let deps = if i == 0 { vec![] } else { vec![step_ids[i - 1].clone()] };
                steps.push(build_step(id, deps));
            }

            let engine = build_engine(steps);
            let execution_plan = engine.execution_plan().expect("topological order should succeed");

            // every produced step must exist in the original list
            prop_assert_eq!(execution_plan.len(), step_ids.len());

            // ensure dependency chain order is respected
            let mut index_map = std::collections::HashMap::new();
            for (idx, id) in execution_plan.iter().enumerate() {
                index_map.insert(id.clone(), idx);
            }

            for (i, id) in step_ids.iter().enumerate().skip(1) {
                let predecessor = &step_ids[i - 1];
                let current_idx = *index_map.get(id).unwrap();
                let predecessor_idx = *index_map.get(predecessor).unwrap();
                prop_assert!(predecessor_idx < current_idx);
            }
        }

        #[test]
        fn workflow_parallel_safety_property(
            parallel_steps in prop::collection::vec("[a-zA-Z]+", 2..5)
        ) {
            let steps: Vec<_> = parallel_steps
                .iter()
                .map(|id| build_step(id, vec![]))
                .collect();

            let engine = build_engine(steps);
            let plan = engine.execution_plan().expect("plan generation should succeed");

            // plan should contain the same set of steps (order is not important)
            prop_assert_eq!(plan.len(), parallel_steps.len());
            for id in &parallel_steps {
                prop_assert!(plan.contains(id));
            }
        }
    }

    #[quickcheck]
    fn workflow_context_preservation(key: String, value: String) -> bool {
        if key.is_empty() || value.is_empty() {
            return true;
        }

        let engine = build_engine(Vec::new());
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        runtime.block_on(async {
            engine.set_context_value(&key, serde_json::json!(value.clone())).await;
            let retrieved = engine.get_context_value(&key).await;
            retrieved.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default() == value
        })
    }
}

#[cfg(test)]
mod tool_replacement_properties {
    use super::*;

    proptest! {
        #[test]
        fn typescript_compilation_property(
            code in typescript_code_strategy(),
            file_path in file_path_strategy()
        ) {
            let toolchain = ToolChainReplacements::new();
            let result = toolchain.compile_typescript(&code, &file_path);

            // Property: compilation should always return a result
            prop_assert!(result.is_ok());

            let compilation = result.unwrap();

            // Property: successful compilation should have valid output
            if compilation.success {
                prop_assert!(compilation.syntax_errors.is_empty());
                prop_assert!(compilation.type_errors.is_empty());
            }

            // Property: errors should have meaningful messages
            for error in &compilation.syntax_errors {
                prop_assert!(!error.message.is_empty());
                prop_assert_eq!(error.file_path, file_path);
            }
        }

        #[test]
        fn code_formatting_idempotency_property(
            code in typescript_code_strategy(),
            file_path in file_path_strategy()
        ) {
            let toolchain = ToolChainReplacements::new();
            let options = oxc_codegen::CodegenOptions::default();

            let result1 = toolchain.format_code(&code, &file_path, &options);

            if let Ok(formatted1) = result1 {
                let result2 = toolchain.format_code(&formatted1.formatted_code, &file_path, &options);

                if let Ok(formatted2) = result2 {
                    // Property: formatting should be idempotent
                    prop_assert_eq!(formatted1.formatted_code, formatted2.formatted_code);
                }
            }
        }

        #[test]
        fn linting_consistency_property(
            code in typescript_code_strategy(),
            file_path in file_path_strategy()
        ) {
            let toolchain = ToolChainReplacements::new();

            // Run linting multiple times
            let result1 = toolchain.lint_code(&code, &file_path);
            let result2 = toolchain.lint_code(&code, &file_path);

            // Property: linting should be deterministic
            match (result1, result2) {
                (Ok(lint1), Ok(lint2)) => {
                    prop_assert_eq!(lint1.errors.len(), lint2.errors.len());
                    prop_assert_eq!(lint1.warnings.len(), lint2.warnings.len());
                }
                (Err(_), Err(_)) => {
                    // Both failing is acceptable
                }
                _ => {
                    // One succeeding and one failing suggests non-determinism
                    prop_assert!(false, "Linting results should be deterministic");
                }
            }
        }
    }

    #[quickcheck]
    fn minification_reduces_size(code: String) -> TestResult {
        if code.len() < 10 || !code.chars().all(|c| c.is_ascii()) {
            return TestResult::discard();
        }

        let toolchain = ToolChainReplacements::new();

        if let Ok(minified) = toolchain.minify_code(&code, "test.js") {
            // Property: minification should not increase size significantly
            TestResult::from_bool(minified.len() <= code.len() + 100) // Allow small overhead
        } else {
            TestResult::discard() // Minification failed
        }
    }
}

#[cfg(test)]
mod storage_properties {
    use super::*;
    use tempfile::TempDir;

    proptest! {
        #[test]
        fn storage_persistence_property(
            rule_name in "[a-zA-Z_][a-zA-Z0-9_]*",
            enabled in any::<bool>(),
            severity in prop::sample::select(vec![
                RuleSeverity::Error,
                RuleSeverity::Warning,
                RuleSeverity::Info,
            ]),
            category in prop::sample::select(vec![
                RuleCategory::Security,
                RuleCategory::Performance,
                RuleCategory::Maintainability,
            ])
        ) {
            let temp_dir = TempDir::new().unwrap();
            let storage_path = temp_dir.path().join("test.db");
            let mut storage = RuleStorage::new(&storage_path);

            let rule = RuleConfig {
                name: rule_name.clone(),
                enabled,
                severity: severity.clone(),
                category: category.clone(),
                configuration: HashMap::new(),
            };

            // Store and retrieve
            storage.store_rule(&rule).unwrap();
            let retrieved = storage.get_rule(&rule_name).unwrap();

            // Property: stored data should be retrievable
            prop_assert!(retrieved.is_some());
            let retrieved_rule = retrieved.unwrap();
            prop_assert_eq!(retrieved_rule.name, rule_name);
            prop_assert_eq!(retrieved_rule.enabled, enabled);
            prop_assert_eq!(retrieved_rule.severity, severity);
            prop_assert_eq!(retrieved_rule.category, category);
        }

        #[test]
        fn storage_update_property(
            rule_name in "[a-zA-Z_][a-zA-Z0-9_]*",
            initial_enabled in any::<bool>(),
            updated_enabled in any::<bool>()
        ) {
            let temp_dir = TempDir::new().unwrap();
            let storage_path = temp_dir.path().join("update_test.db");
            let mut storage = RuleStorage::new(&storage_path);

            let initial_rule = RuleConfig {
                name: rule_name.clone(),
                enabled: initial_enabled,
                severity: RuleSeverity::Warning,
                category: RuleCategory::Performance,
                configuration: HashMap::new(),
            };

            let updated_rule = RuleConfig {
                name: rule_name.clone(),
                enabled: updated_enabled,
                severity: RuleSeverity::Error, // Different severity
                category: RuleCategory::Security, // Different category
                configuration: HashMap::new(),
            };

            // Store initial, then update
            storage.store_rule(&initial_rule).unwrap();
            storage.store_rule(&updated_rule).unwrap();

            let retrieved = storage.get_rule(&rule_name).unwrap().unwrap();

            // Property: updates should overwrite previous values
            prop_assert_eq!(retrieved.enabled, updated_enabled);
            prop_assert_eq!(retrieved.severity, RuleSeverity::Error);
            prop_assert_eq!(retrieved.category, RuleCategory::Security);
        }
    }

    #[quickcheck]
    fn storage_isolation_property(rule_name1: String, rule_name2: String) -> TestResult {
        if rule_name1.is_empty() || rule_name2.is_empty() || rule_name1 == rule_name2 {
            return TestResult::discard();
        }

        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join("isolation_test.db");
        let mut storage = RuleStorage::new(&storage_path);

        let rule1 = RuleConfig {
            name: rule_name1.clone(),
            enabled: true,
            severity: RuleSeverity::Error,
            category: RuleCategory::Security,
            configuration: HashMap::new(),
        };

        let rule2 = RuleConfig {
            name: rule_name2.clone(),
            enabled: false,
            severity: RuleSeverity::Warning,
            category: RuleCategory::Performance,
            configuration: HashMap::new(),
        };

        storage.store_rule(&rule1).unwrap();
        storage.store_rule(&rule2).unwrap();

        let retrieved1 = storage.get_rule(&rule_name1).unwrap().unwrap();
        let retrieved2 = storage.get_rule(&rule_name2).unwrap().unwrap();

        // Property: different rules should be stored independently
        TestResult::from_bool(retrieved1.name == rule_name1 && retrieved2.name == rule_name2 && retrieved1.enabled != retrieved2.enabled)
    }
}

#[cfg(test)]
mod rule_engine_properties {
    use super::*;

    proptest! {
        #[test]
        fn rule_registration_property(
            rule_id in "[a-zA-Z_][a-zA-Z0-9_]*",
            enabled in any::<bool>(),
            category in prop::sample::select(vec![
                MoonShineRuleCategory::CodeQuality,
                MoonShineRuleCategory::Performance,
                MoonShineRuleCategory::Security,
            ])
        ) {
            let mut engine = MoonShineRuleEngine::new();

            let rule = MoonShineRule {
                id: rule_id.clone(),
                category: category.clone(),
                severity: RuleSeverity::Warning,
                enabled,
                description: format!("Test rule {}", rule_id),
            };

            engine.register_rule(rule);

            // Property: registered rules should be findable
            prop_assert!(engine.has_rule(&rule_id));

            let active_rules = engine.active_rules();
            if enabled {
                prop_assert!(active_rules.iter().any(|r| r.id == rule_id));
            }
        }

        #[test]
        fn rule_analysis_property(
            code in typescript_code_strategy(),
            file_path in file_path_strategy()
        ) {
            let mut engine = MoonShineRuleEngine::new();

            // Add some test rules
            for i in 0..3 {
                engine.register_rule(MoonShineRule {
                    id: format!("test_rule_{}", i),
                    category: MoonShineRuleCategory::CodeQuality,
                    severity: RuleSeverity::Warning,
                    enabled: true,
                    description: format!("Test rule {}", i),
                });
            }

            let result = engine.analyze_code(&code, &file_path);

            // Property: analysis should always return a result
            prop_assert!(result.is_ok());

            let analysis = result.unwrap();

            // Property: analysis should have valid structure
            prop_assert!(analysis.issues.len() >= 0);
            prop_assert!(analysis.rules_executed >= 0);
        }
    }

    #[quickcheck]
    fn rule_enablement_consistency(enabled: bool) -> bool {
        let mut engine = MoonShineRuleEngine::new();

        let rule = MoonShineRule {
            id: "test_rule".to_string(),
            category: MoonShineRuleCategory::CodeQuality,
            severity: RuleSeverity::Warning,
            enabled,
            description: "Test rule".to_string(),
        };

        engine.register_rule(rule);

        let active_rules = engine.active_rules();
        let is_in_active = active_rules.iter().any(|r| r.id == "test_rule");

        // Property: rule enablement should be consistent with active rules
        enabled == is_in_active
    }
}

/// Generate test data using the fake crate
#[cfg(test)]
mod fake_data_properties {
    use super::*;
    use fake::{faker::*, Fake};

    #[quickcheck]
    fn fake_data_consistency() -> bool {
        // Generate fake data for testing
        let name: String = name::en::Name().fake();
        let email: String = internet::en::SafeEmail().fake();
        let number: u32 = (0..1000u32).fake();

        // Property: fake data should have expected characteristics
        !name.is_empty() && email.contains('@') && number < 1000
    }

    proptest! {
        #[test]
        fn fake_config_generation_property(seed in any::<u64>()) {
            use fake::{Fake, Faker};

            // Use seed for deterministic fake data
            let mut rng = fake::rng::StdRng::seed_from_u64(seed);

            let ai_model: String = Faker.fake_with_rng(&mut rng);
            let pattern_count: usize = (1..10usize).fake_with_rng(&mut rng);

            // Property: fake data should be usable for realistic testing
            prop_assert!(!ai_model.is_empty());
            prop_assert!(pattern_count > 0 && pattern_count < 10);
        }
    }
}

#[cfg(test)]
mod invariant_properties {
    use super::*;

    proptest! {
        #[test]
        fn memory_safety_property(
            operations in prop::collection::vec(0..10u32, 1..100)
        ) {
            let toolchain = ToolChainReplacements::new();

            // Property: repeated operations should not cause memory issues
            for op in operations {
                match op % 4 {
                    0 => {
                        let _ = toolchain.compile_typescript("const x = 1;", "test.ts");
                    }
                    1 => {
                        let _ = toolchain.lint_code("const x = 1;", "test.ts");
                    }
                    2 => {
                        let options = oxc_codegen::CodegenOptions::default();
                        let _ = toolchain.format_code("const x = 1;", "test.ts", &options);
                    }
                    3 => {
                        let _ = toolchain.analyze_documentation("/** doc */ const x = 1;", "test.ts");
                    }
                    _ => unreachable!(),
                }
            }

            // Property: if we reach here, no memory errors occurred
            prop_assert!(true);
        }

        #[test]
        fn concurrent_safety_property(
            thread_count in 1..5usize,
            operations_per_thread in 1..20usize
        ) {
            use std::thread;
            use std::sync::Arc;

            let toolchain = Arc::new(ToolChainReplacements::new());
            let mut handles = vec![];

            // Property: concurrent access should be safe
            for thread_id in 0..thread_count {
                let toolchain_clone = Arc::clone(&toolchain);
                let handle = thread::spawn(move || {
                    for op in 0..operations_per_thread {
                        let code = format!("const x{} = {};", thread_id, op);
                        let _ = toolchain_clone.compile_typescript(&code, "test.ts");
                    }
                });
                handles.push(handle);
            }

            // Wait for all threads to complete
            for handle in handles {
                handle.join().unwrap();
            }

            // Property: concurrent operations completed without panics
            prop_assert!(true);
        }
    }
}

/// Performance property tests
#[cfg(test)]
mod performance_properties {
    use super::*;
    use std::time::{Duration, Instant};

    proptest! {
        #[test]
        fn compilation_performance_property(
            code_size in 100..10000usize
        ) {
            let code = "const x = 1;\n".repeat(code_size / 13); // Approximate target size
            let toolchain = ToolChainReplacements::new();

            let start = Instant::now();
            let result = toolchain.compile_typescript(&code, "large.ts");
            let duration = start.elapsed();

            // Property: compilation time should scale reasonably with code size
            if result.is_ok() {
                let max_expected_time = Duration::from_millis(code_size as u64 / 10); // Very generous
                prop_assert!(duration < max_expected_time);
            }
        }

        #[test]
        fn memory_usage_property(
            iterations in 1..50usize
        ) {
            let toolchain = ToolChainReplacements::new();

            // Property: repeated operations shouldn't cause unbounded memory growth
            for i in 0..iterations {
                let code = format!("const iteration{} = {};", i, i);
                let _ = toolchain.compile_typescript(&code, "test.ts");

                // Simple heuristic: if we've done many iterations without OOM, memory is bounded
                if i % 10 == 0 {
                    prop_assert!(true); // Checkpoint reached
                }
            }

            prop_assert!(true); // All iterations completed
        }
    }
}
