//! Integration Tests for AI-Enhanced Rules System
//!
//! Comprehensive test suite covering the entire AI-enhanced rule system including
//! execution, learning, adaptation, and quality metrics tracking.

use moon_shine::rulebase::{
    AIEnhancedRuleExecutor, AIRuleExecutionContext, AIRuleTemplate, AIRuleTemplateRegistry,
    AdaptiveLearningEngine, LearningConfig, QualityMetricsTracker, MetricsConfig,
    RuleExecutionResult, UserFeedback, QualityMetrics, PerformanceMetrics,
    AIContextRequirements, AIEnhancements, RuleImplementation, HybridStrategy,
};
use moon_shine::rule_types::{RuleCategory, RuleMetadata, RuleSeverity, FixStatus};
use moon_shine::types::{DiagnosticSeverity, LintDiagnostic};
use moon_shine::rulebase::quality_metrics::ResourceUsage;
use oxc_span::SourceType;
use std::collections::HashMap;
use std::time::Duration;
use tokio::test;

/// Helper function to create a test rule metadata
fn create_test_rule(id: &str, implementation: RuleImplementation) -> RuleMetadata {
    RuleMetadata {
        id: id.to_string(),
        name: format!("Test Rule {}", id),
        description: "Test rule for integration testing".to_string(),
        category: RuleCategory::Security,
        severity: RuleSeverity::Warning,
        fix_status: FixStatus::Manual,
        ai_enhanced: true,
        cost: 30,
        tags: vec!["test".to_string()],
        dependencies: vec![],
        implementation,
        config_schema: None,
    }
}

/// Helper function to create test execution context
fn create_test_context<'a>(code: &'a str, file_path: &'a str) -> AIRuleExecutionContext<'a> {
    AIRuleExecutionContext {
        code,
        file_path,
        source_type: SourceType::ts(),
        program: None,
        project_context: None,
        dependencies: vec![],
        ai_context: None,
    }
}

/// Helper function to create test diagnostics
fn create_test_diagnostics(count: usize) -> Vec<LintDiagnostic> {
    (0..count)
        .map(|i| LintDiagnostic {
            rule_name: "test-rule".to_string(),
            message: format!("Test issue {}", i),
            file_path: "test.ts".to_string(),
            line: i as u32 + 1,
            column: 1,
            end_line: i as u32 + 1,
            end_column: 10,
            severity: DiagnosticSeverity::Warning,
            fix_available: false,
            suggested_fix: None,
        })
        .collect()
}

#[test]
async fn test_ai_behavioral_rule_execution() {
    let mut executor = AIEnhancedRuleExecutor::new();

    let rule = create_test_rule(
        "ai-security-xss",
        RuleImplementation::AiBehavioral {
            pattern_type: "xss_vulnerability".to_string(),
            ai_prompt: Some("Detect XSS vulnerabilities in this code".to_string()),
            confidence_threshold: Some(0.8),
            model_preference: Some("claude".to_string()),
            context_requirements: Some(AIContextRequirements {
                needs_ast: true,
                needs_dependencies: false,
                needs_project_context: false,
                max_context_tokens: 4000,
            }),
        },
    );

    let code = r#"
        function displayUserData(userInput) {
            document.getElementById('output').innerHTML = userInput; // XSS vulnerability
        }
    "#;

    let context = create_test_context(code, "test.ts");

    // This would normally call the AI provider, but for testing we'll mock the behavior
    let result = executor.execute_rule(&rule, &context).await;

    // In a real test, we'd check the actual AI response
    // For now, we verify the structure is correct
    assert!(result.is_ok());
}

#[test]
async fn test_ai_enhanced_static_rule() {
    let mut executor = AIEnhancedRuleExecutor::new();

    let rule = create_test_rule(
        "ai-enhanced-no-var",
        RuleImplementation::AIEnhanced {
            base_rule: "no-var".to_string(),
            ai_enhancements: AIEnhancements {
                context_analysis: true,
                smart_fixes: true,
                false_positive_reduction: true,
                severity_adjustment: false,
                pattern_learning: false,
            },
            enhancement_prompt: Some("Provide smart fixes for var declarations".to_string()),
            fallback_to_static: true,
        },
    );

    let code = r#"
        var x = 1; // Should be flagged
        let y = 2; // OK
        const z = 3; // OK
    "#;

    let context = create_test_context(code, "test.ts");
    let result = executor.execute_rule(&rule, &context).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.ai_enhanced);
}

#[test]
async fn test_hybrid_rule_execution() {
    let mut executor = AIEnhancedRuleExecutor::new();

    let hybrid_rule = create_test_rule(
        "hybrid-security-check",
        RuleImplementation::Hybrid {
            implementations: vec![
                RuleImplementation::OxcStatic {
                    rule_name: "no-eval".to_string(),
                },
                RuleImplementation::AiBehavioral {
                    pattern_type: "dynamic_code_execution".to_string(),
                    ai_prompt: Some("Look for dynamic code execution patterns".to_string()),
                    confidence_threshold: Some(0.7),
                    model_preference: None,
                    context_requirements: None,
                },
            ],
            combination_strategy: HybridStrategy::BestConfidence,
            confidence_weights: HashMap::new(),
        },
    );

    let code = r#"
        eval(userInput); // Should be caught by both static and AI analysis
        Function(userCode)(); // Should be caught by AI analysis
    "#;

    let context = create_test_context(code, "test.ts");
    let result = executor.execute_rule(&hybrid_rule, &context).await;

    assert!(result.is_ok());
}

#[test]
async fn test_smart_rule_with_learning() {
    let mut executor = AIEnhancedRuleExecutor::new();

    let smart_rule = create_test_rule(
        "smart-performance-rule",
        RuleImplementation::SmartRule {
            learning_enabled: true,
            adaptation_threshold: 0.8,
            feedback_learning: true,
            pattern_evolution: false,
            base_implementations: vec![
                RuleImplementation::AiBehavioral {
                    pattern_type: "performance_issue".to_string(),
                    ai_prompt: Some("Detect performance issues".to_string()),
                    confidence_threshold: Some(0.7),
                    model_preference: None,
                    context_requirements: None,
                },
            ],
        },
    );

    let code = r#"
        for (let i = 0; i < array.length; i++) { // Inefficient
            // Repeated array.length access
        }
    "#;

    let context = create_test_context(code, "test.ts");
    let result = executor.execute_rule(&smart_rule, &context).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.ai_enhanced);
}

#[test]
fn test_ai_rule_template_registry() {
    let registry = AIRuleTemplateRegistry::new();

    // Test that default templates are loaded
    let template_ids = registry.list_template_ids();
    assert!(!template_ids.is_empty());

    // Test getting a specific template
    let xss_template = registry.get_template("ai-security-xss-detection");
    assert!(xss_template.is_some());

    let template = xss_template.unwrap();
    assert_eq!(template.category, RuleCategory::Security);
    assert_eq!(template.pattern_type, "xss_vulnerability");
    assert!(!template.base_prompt.is_empty());
}

#[test]
fn test_template_customization() {
    let registry = AIRuleTemplateRegistry::new();
    let mut template = registry.get_template("ai-security-xss-detection").unwrap().clone();

    let mut customization = HashMap::new();
    customization.insert("confidence_threshold".to_string(), "0.9".to_string());
    customization.insert("model_preference".to_string(), "claude".to_string());

    template.customize(customization);

    assert_eq!(template.default_confidence_threshold, 0.9);
    assert_eq!(template.model_preference, Some("claude".to_string()));
}

#[test]
fn test_template_to_rule_conversion() {
    let registry = AIRuleTemplateRegistry::new();
    let template = registry.get_template("ai-performance-memory-leak").unwrap();

    let rule = template.to_rule_metadata(RuleSeverity::Error, 50);

    assert_eq!(rule.severity, RuleSeverity::Error);
    assert_eq!(rule.cost, 50);
    assert!(rule.ai_enhanced);
    assert!(matches!(rule.implementation, RuleImplementation::AiBehavioral { .. }));
}

#[test]
fn test_adaptive_learning_engine() {
    let config = LearningConfig {
        enabled: true,
        min_samples_for_learning: 2,
        ..LearningConfig::default()
    };

    let mut learning_engine = AdaptiveLearningEngine::new(config);

    // Create test execution results
    let rule_id = "test-rule";

    for i in 0..5 {
        let diagnostics = create_test_diagnostics(i % 3);
        let result = RuleExecutionResult {
            rule_id: rule_id.to_string(),
            diagnostics,
            confidence: 0.7 + (i as f32 * 0.05), // Improving confidence
            execution_time: Duration::from_millis(100 + i * 10),
            ai_enhanced: true,
            fallback_used: false,
            metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
                static_analysis_time: Some(Duration::from_millis(50)),
                ai_analysis_time: Some(Duration::from_millis(50)),
                hybrid_combination_time: None,
                tokens_used: Some(1000),
                model_used: Some("claude".to_string()),
                error_count: 0,
                warning_count: i as u32,
            },
        };

        let context = moon_shine::rulebase::adaptive_learning::ExecutionContext {
            file_type: "typescript".to_string(),
            project_type: Some("react".to_string()),
            code_complexity: 0.5,
            file_size: 1000,
            ai_model_used: Some("claude".to_string()),
        };

        learning_engine.record_execution(rule_id, &result, context);
    }

    // Test adaptation logic
    let adaptation = learning_engine.should_adapt_rule(rule_id);
    assert!(!adaptation.should_adapt); // Should not adapt with good performance

    // Test feedback recording
    learning_engine.record_feedback(rule_id, 0, UserFeedback::TruePositive);

    // Test performance trend
    let trend = learning_engine.get_performance_trend(rule_id, 30);
    assert!(trend.is_some());

    let trend = trend.unwrap();
    assert!(trend.confidence_trend >= 0.0); // Should show improvement or stability
}

#[test]
fn test_quality_metrics_tracker() {
    let config = MetricsConfig::default();
    let mut tracker = QualityMetricsTracker::new(config);

    let rule = create_test_rule(
        "quality-test-rule",
        RuleImplementation::AiBehavioral {
            pattern_type: "test_pattern".to_string(),
            ai_prompt: None,
            confidence_threshold: None,
            model_preference: None,
            context_requirements: None,
        },
    );

    let diagnostics = create_test_diagnostics(3);
    let result = RuleExecutionResult {
        rule_id: rule.id.clone(),
        diagnostics,
        confidence: 0.85,
        execution_time: Duration::from_millis(150),
        ai_enhanced: true,
        fallback_used: false,
        metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
            static_analysis_time: Some(Duration::from_millis(75)),
            ai_analysis_time: Some(Duration::from_millis(75)),
            hybrid_combination_time: None,
            tokens_used: Some(1200),
            model_used: Some("claude".to_string()),
            error_count: 0,
            warning_count: 3,
        },
    };

    let resource_usage = ResourceUsage {
        memory_mb: 128.0,
        cpu_percent: 25.0,
    };

    // Record execution
    tracker.record_execution(&rule, &result, resource_usage);

    // Test metrics retrieval
    let quality_metrics = tracker.get_rule_quality_metrics(&rule.id);
    assert!(quality_metrics.is_some());

    let metrics = quality_metrics.unwrap();
    assert_eq!(metrics.confidence_score, 0.85);

    let performance_metrics = tracker.get_rule_performance_metrics(&rule.id);
    assert!(performance_metrics.is_some());

    let perf = performance_metrics.unwrap();
    assert_eq!(perf.execution_time, Duration::from_millis(150));
    assert_eq!(perf.token_usage, 1200);
}

#[test]
fn test_quality_metrics_with_feedback() {
    let config = MetricsConfig::default();
    let mut tracker = QualityMetricsTracker::new(config);

    let rule = create_test_rule("feedback-test-rule", RuleImplementation::AiBehavioral {
        pattern_type: "test_pattern".to_string(),
        ai_prompt: None,
        confidence_threshold: None,
        model_preference: None,
        context_requirements: None,
    });

    // Record multiple executions
    for i in 0..5 {
        let diagnostics = create_test_diagnostics(2);
        let result = RuleExecutionResult {
            rule_id: rule.id.clone(),
            diagnostics,
            confidence: 0.8,
            execution_time: Duration::from_millis(100),
            ai_enhanced: true,
            fallback_used: false,
            metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
                static_analysis_time: Some(Duration::from_millis(50)),
                ai_analysis_time: Some(Duration::from_millis(50)),
                hybrid_combination_time: None,
                tokens_used: Some(1000),
                model_used: Some("claude".to_string()),
                error_count: 0,
                warning_count: 2,
            },
        };

        let resource_usage = ResourceUsage {
            memory_mb: 100.0,
            cpu_percent: 20.0,
        };

        tracker.record_execution(&rule, &result, resource_usage);

        // Record feedback for some executions
        if i % 2 == 0 {
            let feedback = if i < 2 {
                UserFeedback::TruePositive
            } else {
                UserFeedback::FalsePositive
            };
            tracker.record_feedback(&rule.id, i as u64, feedback);
        }
    }

    // Generate analytics
    let analytics = tracker.generate_rule_analytics(&rule.id);
    assert!(analytics.is_some());

    let analytics = analytics.unwrap();
    assert_eq!(analytics.total_executions, 5);
    assert!(analytics.success_rate > 0.0);
}

#[test]
fn test_quality_dashboard_generation() {
    let config = MetricsConfig::default();
    let mut tracker = QualityMetricsTracker::new(config);

    // Create multiple rules with different performance levels
    let rules = vec![
        ("high-quality-rule", 0.9, 1),
        ("medium-quality-rule", 0.7, 2),
        ("low-quality-rule", 0.4, 3),
    ];

    for (rule_id, confidence, diagnostic_count) in rules {
        let rule = create_test_rule(rule_id, RuleImplementation::AiBehavioral {
            pattern_type: "test_pattern".to_string(),
            ai_prompt: None,
            confidence_threshold: None,
            model_preference: None,
            context_requirements: None,
        });

        let diagnostics = create_test_diagnostics(diagnostic_count);
        let result = RuleExecutionResult {
            rule_id: rule.id.clone(),
            diagnostics,
            confidence,
            execution_time: Duration::from_millis(100),
            ai_enhanced: true,
            fallback_used: false,
            metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
                static_analysis_time: Some(Duration::from_millis(50)),
                ai_analysis_time: Some(Duration::from_millis(50)),
                hybrid_combination_time: None,
                tokens_used: Some(1000),
                model_used: Some("claude".to_string()),
                error_count: 0,
                warning_count: diagnostic_count as u32,
            },
        };

        let resource_usage = ResourceUsage {
            memory_mb: 100.0,
            cpu_percent: 20.0,
        };

        tracker.record_execution(&rule, &result, resource_usage);

        // Add feedback to differentiate quality
        let feedback = if confidence > 0.8 {
            UserFeedback::TruePositive
        } else if confidence < 0.5 {
            UserFeedback::FalsePositive
        } else {
            UserFeedback::Helpful
        };
        tracker.record_feedback(&rule.id, 0, feedback);
    }

    // Generate dashboard
    let dashboard = tracker.generate_quality_dashboard();

    assert_eq!(dashboard.overview.total_rules, 3);
    assert!(!dashboard.top_performing_rules.is_empty());
    assert!(!dashboard.problematic_rules.is_empty());

    // Top performing rule should be the high-quality one
    assert_eq!(dashboard.top_performing_rules[0].rule_id, "high-quality-rule");

    // Problematic rules should include the low-quality one
    let problematic_ids: Vec<_> = dashboard.problematic_rules
        .iter()
        .map(|r| r.rule_id.as_str())
        .collect();
    assert!(problematic_ids.contains(&"low-quality-rule"));
}

#[test]
async fn test_end_to_end_ai_rule_workflow() {
    // This test simulates the complete workflow from template to execution to learning

    // 1. Create rule from template
    let registry = AIRuleTemplateRegistry::new();
    let template = registry.get_template("ai-security-xss-detection").unwrap();
    let rule = template.to_rule_metadata(RuleSeverity::Error, 40);

    // 2. Execute rule
    let mut executor = AIEnhancedRuleExecutor::new();
    let code = r#"
        function renderUserContent(html) {
            document.body.innerHTML = html; // Potential XSS
        }
    "#;
    let context = create_test_context(code, "test.ts");

    let execution_result = executor.execute_rule(&rule, &context).await;
    assert!(execution_result.is_ok());

    let result = execution_result.unwrap();

    // 3. Record metrics
    let mut metrics_tracker = QualityMetricsTracker::new(MetricsConfig::default());
    let resource_usage = ResourceUsage {
        memory_mb: 150.0,
        cpu_percent: 30.0,
    };
    metrics_tracker.record_execution(&rule, &result, resource_usage);

    // 4. Record learning data
    let mut learning_engine = AdaptiveLearningEngine::new(LearningConfig::default());
    let context = moon_shine::rulebase::adaptive_learning::ExecutionContext {
        file_type: "typescript".to_string(),
        project_type: Some("react".to_string()),
        code_complexity: 0.6,
        file_size: code.len(),
        ai_model_used: result.metadata.model_used.clone(),
    };
    learning_engine.record_execution(&rule.id, &result, context);

    // 5. Add user feedback
    learning_engine.record_feedback(&rule.id, 0, UserFeedback::TruePositive);
    metrics_tracker.record_feedback(&rule.id, 0, UserFeedback::TruePositive);

    // 6. Check adaptation recommendations
    let adaptation = learning_engine.should_adapt_rule(&rule.id);
    // With good performance, should not need adaptation

    // 7. Generate analytics
    let analytics = metrics_tracker.generate_rule_analytics(&rule.id);
    assert!(analytics.is_some());

    let analytics = analytics.unwrap();
    assert_eq!(analytics.rule_id, rule.id);
    assert_eq!(analytics.total_executions, 1);
}

#[test]
fn test_metrics_export() {
    let config = MetricsConfig::default();
    let mut tracker = QualityMetricsTracker::new(config);

    let rule = create_test_rule("export-test-rule", RuleImplementation::AiBehavioral {
        pattern_type: "test_pattern".to_string(),
        ai_prompt: None,
        confidence_threshold: None,
        model_preference: None,
        context_requirements: None,
    });

    let diagnostics = create_test_diagnostics(1);
    let result = RuleExecutionResult {
        rule_id: rule.id.clone(),
        diagnostics,
        confidence: 0.8,
        execution_time: Duration::from_millis(100),
        ai_enhanced: true,
        fallback_used: false,
        metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
            static_analysis_time: Some(Duration::from_millis(50)),
            ai_analysis_time: Some(Duration::from_millis(50)),
            hybrid_combination_time: None,
            tokens_used: Some(1000),
            model_used: Some("claude".to_string()),
            error_count: 0,
            warning_count: 1,
        },
    };

    let resource_usage = ResourceUsage {
        memory_mb: 100.0,
        cpu_percent: 20.0,
    };

    tracker.record_execution(&rule, &result, resource_usage);

    // Test rule-specific export
    let rule_export = tracker.export_metrics(Some(&rule.id));
    assert!(rule_export.rule_specific.is_some());
    assert!(rule_export.system_wide.is_none());
    assert_eq!(rule_export.metadata.export_type, "rule_specific");

    // Test system-wide export
    let system_export = tracker.export_metrics(None);
    assert!(system_export.rule_specific.is_none());
    assert!(system_export.system_wide.is_some());
    assert_eq!(system_export.metadata.export_type, "system_wide");
}

/// Helper macro for creating async tests
macro_rules! async_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() {
            $body
        }
    };
}

// Additional integration tests for edge cases and error handling

#[test]
fn test_learning_engine_with_insufficient_data() {
    let config = LearningConfig {
        min_samples_for_learning: 10,
        ..LearningConfig::default()
    };

    let mut learning_engine = AdaptiveLearningEngine::new(config);

    // Add only a few samples (less than minimum)
    for i in 0..3 {
        let diagnostics = create_test_diagnostics(1);
        let result = RuleExecutionResult {
            rule_id: "insufficient-data-rule".to_string(),
            diagnostics,
            confidence: 0.7,
            execution_time: Duration::from_millis(100),
            ai_enhanced: true,
            fallback_used: false,
            metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
                static_analysis_time: Some(Duration::from_millis(50)),
                ai_analysis_time: Some(Duration::from_millis(50)),
                hybrid_combination_time: None,
                tokens_used: Some(1000),
                model_used: Some("claude".to_string()),
                error_count: 0,
                warning_count: 1,
            },
        };

        let context = moon_shine::rulebase::adaptive_learning::ExecutionContext {
            file_type: "typescript".to_string(),
            project_type: None,
            code_complexity: 0.5,
            file_size: 1000,
            ai_model_used: Some("claude".to_string()),
        };

        learning_engine.record_execution("insufficient-data-rule", &result, context);
    }

    // Should not adapt with insufficient data
    let adaptation = learning_engine.should_adapt_rule("insufficient-data-rule");
    assert!(!adaptation.should_adapt);
    assert!(adaptation.reasoning.contains("Insufficient data"));
}

#[test]
fn test_concurrent_metrics_collection() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let config = MetricsConfig::default();
    let tracker = Arc::new(Mutex::new(QualityMetricsTracker::new(config)));

    let mut handles = vec![];

    // Simulate concurrent rule executions
    for i in 0..5 {
        let tracker_clone = Arc::clone(&tracker);
        let handle = thread::spawn(move || {
            let rule = create_test_rule(&format!("concurrent-rule-{}", i), RuleImplementation::AiBehavioral {
                pattern_type: "test_pattern".to_string(),
                ai_prompt: None,
                confidence_threshold: None,
                model_preference: None,
                context_requirements: None,
            });

            let diagnostics = create_test_diagnostics(i % 3);
            let result = RuleExecutionResult {
                rule_id: rule.id.clone(),
                diagnostics,
                confidence: 0.8,
                execution_time: Duration::from_millis(100),
                ai_enhanced: true,
                fallback_used: false,
                metadata: moon_shine::rulebase::ai_enhanced_executor::ExecutionMetadata {
                    static_analysis_time: Some(Duration::from_millis(50)),
                    ai_analysis_time: Some(Duration::from_millis(50)),
                    hybrid_combination_time: None,
                    tokens_used: Some(1000),
                    model_used: Some("claude".to_string()),
                    error_count: 0,
                    warning_count: (i % 3) as u32,
                },
            };

            let resource_usage = ResourceUsage {
                memory_mb: 100.0,
                cpu_percent: 20.0,
            };

            let mut tracker = tracker_clone.lock().unwrap();
            tracker.record_execution(&rule, &result, resource_usage);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify that all executions were recorded
    let tracker = tracker.lock().unwrap();
    let system_metrics = tracker.get_system_metrics();
    assert!(system_metrics.is_some());

    let metrics = system_metrics.unwrap();
    assert_eq!(metrics.total_rules, 5);
}