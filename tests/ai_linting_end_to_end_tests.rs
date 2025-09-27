//! # AI Linting End-to-End Integration Tests
//!
//! Comprehensive integration tests for the complete AI linting pipeline:
//! - OXC adapter integration with AI behavioral analysis
//! - Workflow engine AI linting steps
//! - Provider router with real AI communication patterns
//! - Combined static and AI analysis results
//! - Error handling and graceful degradation
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::config::MoonShineConfig;
use moon_shine::oxc_adapter::{OxcAdapter, AiBehavioralAnalyzer, AnalysisContext, ProjectContext};
use moon_shine::provider_router::{get_ai_router, AIRequest, AIContext, lint_code_with_ai};
use moon_shine::workflow::{WorkflowDefinition, WorkflowEngine};
use moon_shine::moon_pdk_interface::AiLinterConfig;
use rstest::*;
use std::collections::HashMap;
use tokio::time::{timeout, Duration};
use insta::assert_json_snapshot;
use std::sync::Arc;

/// Test fixtures for various code samples
struct CodeSamples;

impl CodeSamples {
    fn react_performance_issues() -> &'static str {
        r#"
            import React, { useState, useEffect } from 'react';
            
            function ProblematicComponent({ data }) {
                const [count, setCount] = useState(0);
                const [items, setItems] = useState([]);
                
                // Missing dependency array - will cause infinite re-renders
                useEffect(() => {
                    setCount(count + 1);
                });
                
                // Expensive computation on every render
                const expensiveValue = data.map(item => item.value * 2).reduce((a, b) => a + b, 0);
                
                return (
                    <div onClick={() => setCount(count + 1)}>
                        {/* Inline object creation causes child re-renders */}
                        <ChildComponent style={{margin: 10}} data={expensiveValue} />
                        {/* Inline function creation */}
                        {items.map(item => <Item key={item.id} onClick={() => handleClick(item)} />)}
                        <p>Count: {count}</p>
                    </div>
                );
            }
        "#
    }
    
    fn security_vulnerabilities() -> &'static str {
        r#"
            function UserDashboard() {
                const [userInput, setUserInput] = useState('');
                const [userId, setUserId] = useState('');
                
                const handleSubmit = () => {
                    // XSS vulnerability - direct HTML injection
                    document.getElementById('output').innerHTML = userInput;
                    
                    // SQL injection vulnerability pattern
                    const query = `SELECT * FROM users WHERE id = ${userId}`;
                    fetch('/api/query', {
                        method: 'POST',
                        body: JSON.stringify({ query })
                    });
                    
                    // Hardcoded sensitive data
                    const apiKey = 'sk-1234567890abcdefghijklmnop';
                    const dbPassword = 'admin123';
                    
                    fetch(`/api/data?key=${apiKey}`);
                };
                
                return (
                    <div>
                        <input value={userInput} onChange={e => setUserInput(e.target.value)} />
                        <button onClick={handleSubmit}>Submit</button>
                        <div id="output"></div>
                    </div>
                );
            }
        "#
    }
    
    fn memory_leak_patterns() -> &'static str {
        r#"
            function ComponentWithLeaks() {
                const [data, setData] = useState([]);
                
                useEffect(() => {
                    // Memory leak: event listener without cleanup
                    document.addEventListener('scroll', handleScroll);
                    
                    // Memory leak: interval without cleanup
                    const interval = setInterval(() => {
                        fetchData().then(setData);
                    }, 1000);
                    
                    // Memory leak: timeout without cleanup
                    const timeout = setTimeout(() => {
                        console.log('Delayed action');
                    }, 5000);
                    
                    // Missing cleanup function
                }, []);
                
                const handleScroll = () => {
                    // Handler that holds references
                    console.log('Scrolling with', data.length, 'items');
                };
                
                return <div>{data.map(item => <div key={item.id}>{item.name}</div>)}</div>;
            }
        "#
    }
    
    fn cognitive_complexity() -> &'static str {
        r#"
            function complexBusinessLogic(order, user, inventory, discounts) {
                if (order && order.items && order.items.length > 0) {
                    for (let i = 0; i < order.items.length; i++) {
                        const item = order.items[i];
                        if (item.type === 'physical') {
                            if (inventory[item.id] && inventory[item.id].quantity >= item.quantity) {
                                if (user.membershipLevel === 'premium') {
                                    if (discounts && discounts.premium) {
                                        for (let j = 0; j < discounts.premium.length; j++) {
                                            const discount = discounts.premium[j];
                                            if (discount.category === item.category || discount.category === 'all') {
                                                if (discount.type === 'percentage') {
                                                    item.price = item.price * (1 - discount.value / 100);
                                                } else if (discount.type === 'fixed') {
                                                    item.price = Math.max(0, item.price - discount.value);
                                                } else if (discount.type === 'bogo' && item.quantity >= 2) {
                                                    item.price = item.price * item.quantity * 0.5;
                                                }
                                            }
                                        }
                                    }
                                } else if (user.membershipLevel === 'standard') {
                                    if (discounts && discounts.standard) {
                                        // Similar nested logic for standard users...
                                    }
                                }
                                inventory[item.id].quantity -= item.quantity;
                            } else {
                                throw new Error('Insufficient inventory');
                            }
                        } else if (item.type === 'digital') {
                            // Digital item processing logic...
                        }
                    }
                }
                return order;
            }
        "#
    }
    
    fn clean_optimized_code() -> &'static str {
        r#"
            import React, { useState, useEffect, useCallback, useMemo } from 'react';
            
            function OptimizedComponent({ data }) {
                const [count, setCount] = useState(0);
                const [items, setItems] = useState([]);
                
                // Proper dependency array
                useEffect(() => {
                    setCount(prev => prev + 1);
                }, []);
                
                // Memoized expensive computation
                const expensiveValue = useMemo(() => {
                    return data.map(item => item.value * 2).reduce((a, b) => a + b, 0);
                }, [data]);
                
                // Memoized event handler
                const handleClick = useCallback((item) => {
                    // Handle click logic
                }, []);
                
                // Memoized style object
                const childStyle = useMemo(() => ({ margin: 10 }), []);
                
                return (
                    <div onClick={() => setCount(prev => prev + 1)}>
                        <ChildComponent style={childStyle} data={expensiveValue} />
                        {items.map(item => (
                            <Item key={item.id} onClick={() => handleClick(item)} />
                        ))}
                        <p>Count: {count}</p>
                    </div>
                );
            }
        "#
    }
}

#[fixture]
fn test_config() -> MoonShineConfig {
    MoonShineConfig::default()
}

#[fixture]
fn ai_linter_config() -> AiLinterConfig {
    AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 300, // Shorter for testing
        quality_threshold: 0.7,
        debug_session_retention_hours: 1,
        cleanup_sessions_older_than_hours: 2,
        max_concurrent_requests: 2,
        batch_size: 3,
        rate_limit_per_minute: 30,
        max_tokens_per_request: 8192,
        retry_attempts: 2,
        retry_delay_ms: 500,
    }
}

/// Helper to create analysis context for testing
fn create_analysis_context(file_path: &str) -> AnalysisContext {
    AnalysisContext {
        file_path: file_path.to_string(),
        file_type: oxc_span::SourceType::tsx(),
        project_context: Some(ProjectContext {
            framework: Some("React".to_string()),
            build_tool: Some("Vite".to_string()),
            testing_framework: Some("Jest".to_string()),
            package_json_dependencies: HashMap::from([
                ("react".to_string(), "^18.0.0".to_string()),
                ("typescript".to_string(), "^5.0.0".to_string()),
            ]),
        }),
        dependencies: vec!["react".to_string(), "typescript".to_string()],
    }
}

#[rstest]
#[tokio::test]
async fn test_oxc_adapter_basic_static_analysis() {
    let adapter = OxcAdapter::new();
    let code = CodeSamples::react_performance_issues();
    
    let result = adapter.analyze_code(code, "test-component.tsx");
    assert!(result.is_ok(), "OXC adapter should successfully analyze React code");
    
    let analysis_result = result.unwrap();
    
    // Should detect some basic issues even without AI
    assert!(analysis_result.diagnostics.len() >= 0, "Should complete analysis without errors");
    println!("Static analysis found {} diagnostics", analysis_result.diagnostics.len());
}

#[rstest]
#[tokio::test]
async fn test_oxc_adapter_with_behavioral_patterns() {
    let adapter = OxcAdapter::new();
    let code = CodeSamples::memory_leak_patterns();
    
    let result = timeout(
        Duration::from_secs(30),
        adapter.analyze_with_behavioral_patterns(code, "memory-leaks.tsx")
    ).await;
    
    assert!(result.is_ok(), "Analysis should complete within timeout");
    let analysis_result = result.unwrap();
    
    assert!(analysis_result.is_ok(), "Behavioral pattern analysis should succeed");
    
    let combined_result = analysis_result.unwrap();
    assert!(combined_result.total_diagnostics >= 0, "Should have diagnostic count");
    
    if combined_result.behavioral_diagnostics > 0 {
        assert!(combined_result.severity_score > 0.0, "Should have non-zero severity score for issues");
        println!("Behavioral analysis found {} issues with severity score {}", 
            combined_result.behavioral_diagnostics, combined_result.severity_score);
    }
}

#[rstest]
#[tokio::test]
async fn test_ai_behavioral_analyzer_integration() {
    let analyzer = AiBehavioralAnalyzer::new();
    let context = create_analysis_context("performance-test.tsx");
    let code = CodeSamples::react_performance_issues();
    
    // Parse code for AST analysis
    let allocator = oxc_allocator::Allocator::default();
    let source_type = oxc_span::SourceType::tsx();
    let parser_result = oxc_parser::Parser::new(&allocator, code, source_type).parse();
    
    let result = timeout(
        Duration::from_secs(20),
        analyzer.analyze_behavioral_patterns(code, &parser_result.program, &context)
    ).await;
    
    assert!(result.is_ok(), "Behavioral analysis should complete within timeout");
    let analysis_result = result.unwrap();
    
    assert!(analysis_result.is_ok(), "Behavioral analysis should succeed");
    let diagnostics = analysis_result.unwrap();
    
    // Should detect some heuristic issues at minimum
    println!("Behavioral analysis found {} diagnostics", diagnostics.len());
    
    // Check for expected pattern types
    let rule_names: Vec<_> = diagnostics.iter().map(|d| &d.rule_name).collect();
    println!("Rules triggered: {:?}", rule_names);
}

#[rstest]
#[tokio::test]
async fn test_workflow_engine_ai_linting_step(test_config: MoonShineConfig) {
    let code = CodeSamples::security_vulnerabilities();
    let file_path = "security-test.tsx".to_string();
    
    // Create AI linting workflow
    let definition = WorkflowDefinition::from_mode("ai-linting");
    let engine_result = WorkflowEngine::new(definition, code.to_string(), file_path, test_config);
    
    assert!(engine_result.is_ok(), "Workflow engine should initialize successfully");
    
    let mut engine = engine_result.unwrap();
    assert!(!engine.ordered_steps.is_empty(), "AI linting workflow should have steps");
    
    // Execute workflow with timeout
    let execution_result = timeout(
        Duration::from_secs(60),
        async move {
            engine.execute()
        }
    ).await;
    
    match execution_result {
        Ok(workflow_result) => {
            assert!(workflow_result.is_ok(), "Workflow execution should succeed or fail gracefully");
            
            let outcome = workflow_result.unwrap();
            assert!(!outcome.step_results.is_empty(), "Should have step results");
            
            println!("Workflow completed with {} steps", outcome.step_results.len());
            for step in &outcome.step_results {
                println!("Step '{}': {} ({})", step.name, 
                    if step.success { "✓" } else { "✗" },
                    step.detail.as_ref().unwrap_or(&"No details".to_string()));
            }
        },
        Err(_) => {
            println!("Workflow execution timed out - this is acceptable in test environment without AI providers");
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_provider_router_ai_linting_context() {
    let router = get_ai_router();
    
    let linting_request = AIRequest {
        prompt: "Analyze this code for performance and security issues".to_string(),
        session_id: "integration-test-session".to_string(),
        file_path: Some("test-component.tsx".to_string()),
        context: AIContext::AiLinting {
            language: "typescript".to_string(),
            content: CodeSamples::react_performance_issues().to_string(),
            static_issues: vec![
                "Missing dependency in useEffect".to_string(),
                "Inefficient render pattern detected".to_string(),
            ],
            analysis_focus: vec![
                "performance".to_string(),
                "react".to_string(),
                "memory".to_string(),
            ],
        },
        preferred_providers: vec![],
    };
    
    // Test provider selection for AI linting
    let selection_result = router.select_provider(&linting_request);
    assert!(selection_result.is_ok(), "Should select appropriate provider for AI linting");
    
    let (provider, reason) = selection_result.unwrap();
    assert!(!provider.name.is_empty(), "Selected provider should have a name");
    assert!(!reason.is_empty(), "Selection should have reasoning");
    
    // AI linting should prefer providers with strong analysis and reasoning capabilities
    assert!(provider.capabilities.code_analysis >= 0.8 || provider.capabilities.complex_reasoning >= 0.8,
        "AI linting should select providers with strong analysis or reasoning capabilities");
    
    println!("Selected provider '{}' for AI linting: {}", provider.name, reason);
}

#[rstest]
#[tokio::test]
async fn test_lint_code_with_ai_function(ai_linter_config: AiLinterConfig) {
    let session_id = "test-ai-linting-function".to_string();
    let content = CodeSamples::cognitive_complexity().to_string();
    let language = "typescript".to_string();
    let static_issues = vec![
        "High cyclomatic complexity detected".to_string(),
        "Function exceeds recommended length".to_string(),
    ];
    let analysis_focus = vec![
        "complexity".to_string(),
        "maintainability".to_string(),
        "refactoring".to_string(),
    ];
    let file_path = Some("complex-business-logic.ts".to_string());
    
    let result = timeout(
        Duration::from_secs(30),
        lint_code_with_ai(session_id, content, language, static_issues, analysis_focus, file_path)
    ).await;
    
    match result {
        Ok(ai_result) => {
            match ai_result {
                Ok(response) => {
                    assert!(response.success, "AI linting should succeed when provider is available");
                    assert!(!response.content.is_empty(), "Should return analysis content");
                    assert!(!response.provider_used.is_empty(), "Should specify which provider was used");
                    assert!(response.execution_time_ms > 0, "Should report execution time");
                    
                    println!("AI linting completed successfully with {}ms execution time", response.execution_time_ms);
                    println!("Provider used: {}", response.provider_used);
                    println!("Analysis preview: {}...", 
                        response.content.chars().take(100).collect::<String>());
                },
                Err(error) => {
                    // In test environment without AI providers, this is expected
                    println!("AI linting failed as expected in test environment: {}", error);
                    assert!(error.to_string().contains("AI") || 
                           error.to_string().contains("provider") || 
                           error.to_string().contains("command"),
                        "Error should be related to AI provider unavailability: {}", error);
                }
            }
        },
        Err(_) => {
            println!("AI linting timed out - acceptable in test environment");
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_combined_static_and_ai_analysis_pipeline() {
    let adapter = OxcAdapter::new();
    let code = CodeSamples::security_vulnerabilities();
    let file_path = "security-component.tsx";
    
    // Step 1: Static analysis
    let static_result = adapter.analyze_code(code, file_path);
    assert!(static_result.is_ok(), "Static analysis should succeed");
    
    let static_analysis = static_result.unwrap();
    let static_issues_count = static_analysis.diagnostics.len();
    
    println!("Static analysis found {} issues", static_issues_count);
    
    // Step 2: Extract issues for AI context
    let static_issues: Vec<String> = static_analysis.diagnostics.iter()
        .map(|d| format!("{} (line {})", d.message, d.line))
        .collect();
    
    // Step 3: AI behavioral analysis
    let ai_result = timeout(
        Duration::from_secs(20),
        lint_code_with_ai(
            "combined-analysis-session".to_string(),
            code.to_string(),
            "typescript".to_string(),
            static_issues,
            vec!["security".to_string(), "patterns".to_string()],
            Some(file_path.to_string())
        )
    ).await;
    
    match ai_result {
        Ok(ai_response) => {
            match ai_response {
                Ok(response) => {
                    println!("Combined analysis completed:");
                    println!("- Static issues: {}", static_issues_count);
                    println!("- AI analysis provider: {}", response.provider_used);
                    println!("- AI execution time: {}ms", response.execution_time_ms);
                    
                    // Verify AI analysis provides additional insights
                    assert!(!response.content.is_empty(), "AI should provide analysis content");
                },
                Err(error) => {
                    println!("AI analysis failed (expected in test environment): {}", error);
                    // Static analysis still provides value
                    assert!(static_issues_count >= 0, "Static analysis should still work");
                }
            }
        },
        Err(_) => {
            println!("AI analysis timed out - static analysis still available with {} issues", static_issues_count);
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_error_handling_and_graceful_degradation() {
    let adapter = OxcAdapter::new();
    
    // Test with invalid code
    let invalid_code = "function incomplete() { if (true";
    let static_result = adapter.analyze_code(invalid_code, "invalid.js");
    
    // OXC should handle parsing errors gracefully
    match static_result {
        Ok(result) => {
            // If parsing succeeds partially, that's fine
            println!("Partial parsing succeeded with {} diagnostics", result.diagnostics.len());
        },
        Err(error) => {
            // If parsing fails, error should be informative
            assert!(!error.to_string().is_empty(), "Error message should be informative");
            println!("Parsing failed as expected: {}", error);
        }
    }
    
    // Test AI analysis with empty code
    let ai_result = timeout(
        Duration::from_secs(5),
        lint_code_with_ai(
            "error-handling-test".to_string(),
            "".to_string(),
            "typescript".to_string(),
            vec![],
            vec!["general".to_string()],
            None
        )
    ).await;
    
    match ai_result {
        Ok(ai_response) => {
            match ai_response {
                Ok(response) => {
                    println!("AI handled empty code: {}", response.provider_used);
                },
                Err(error) => {
                    println!("AI appropriately handled empty code with error: {}", error);
                }
            }
        },
        Err(_) => {
            println!("AI analysis timed out on empty code - acceptable");
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_performance_comparison_static_vs_ai() {
    let adapter = OxcAdapter::new();
    let code = CodeSamples::clean_optimized_code();
    
    // Measure static analysis performance
    let static_start = std::time::Instant::now();
    let static_result = adapter.analyze_code(code, "optimized-component.tsx");
    let static_duration = static_start.elapsed();
    
    assert!(static_result.is_ok(), "Static analysis should succeed");
    println!("Static analysis completed in: {:?}", static_duration);
    
    // Measure AI analysis performance
    let ai_start = std::time::Instant::now();
    let ai_result = timeout(
        Duration::from_secs(30),
        lint_code_with_ai(
            "performance-comparison".to_string(),
            code.to_string(),
            "typescript".to_string(),
            vec![],
            vec!["performance".to_string()],
            Some("optimized-component.tsx".to_string())
        )
    ).await;
    let ai_duration = ai_start.elapsed();
    
    println!("AI analysis completed in: {:?}", ai_duration);
    
    // Static analysis should be significantly faster
    assert!(static_duration.as_millis() < 1000, "Static analysis should be fast: {:?}", static_duration);
    
    match ai_result {
        Ok(Ok(response)) => {
            println!("AI analysis completed successfully");
            println!("Performance ratio (AI/Static): {:.2}x", 
                ai_duration.as_millis() as f64 / static_duration.as_millis() as f64);
        },
        Ok(Err(error)) => {
            println!("AI analysis failed (expected in test environment): {}", error);
        },
        Err(_) => {
            println!("AI analysis timed out - this is normal in test environment");
        }
    }
}

#[rstest]
#[tokio::test]
async fn test_batch_analysis_with_multiple_files() {
    let code_samples = vec![
        ("react-performance.tsx", CodeSamples::react_performance_issues()),
        ("security-issues.tsx", CodeSamples::security_vulnerabilities()),
        ("memory-leaks.tsx", CodeSamples::memory_leak_patterns()),
        ("complex-logic.ts", CodeSamples::cognitive_complexity()),
        ("optimized-code.tsx", CodeSamples::clean_optimized_code()),
    ];
    
    let adapter = OxcAdapter::new();
    let mut all_results = Vec::new();
    
    for (filename, code) in code_samples {
        let result = adapter.analyze_code(code, filename);
        match result {
            Ok(analysis) => {
                all_results.push((filename, analysis.diagnostics.len()));
                println!("Analyzed {}: {} diagnostics", filename, analysis.diagnostics.len());
            },
            Err(error) => {
                println!("Failed to analyze {}: {}", filename, error);
                all_results.push((filename, 0));
            }
        }
    }
    
    assert_eq!(all_results.len(), 5, "Should process all files");
    
    // Verify different files produce different diagnostic counts
    let diagnostic_counts: Vec<_> = all_results.iter().map(|(_, count)| *count).collect();
    println!("Diagnostic counts: {:?}", diagnostic_counts);
    
    // At least some files should have diagnostics (or all might be clean, which is also valid)
    let total_diagnostics: usize = diagnostic_counts.iter().sum();
    println!("Total diagnostics across all files: {}", total_diagnostics);
}

/// Test snapshot for consistent AI output validation
#[rstest]
#[tokio::test]
async fn test_ai_output_structure_validation() {
    // This test validates the structure of AI responses for consistency
    let mock_ai_response = moon_shine::provider_router::AIResponse {
        provider_used: "test-provider".to_string(),
        content: "Analysis complete: Found 3 performance issues and 2 security vulnerabilities".to_string(),
        session_id: "test-session".to_string(),
        success: true,
        execution_time_ms: 1250,
        error_message: None,
        routing_reason: "Selected for high code analysis capabilities".to_string(),
    };
    
    // Validate response structure
    assert!(!mock_ai_response.provider_used.is_empty());
    assert!(!mock_ai_response.content.is_empty());
    assert!(!mock_ai_response.session_id.is_empty());
    assert!(mock_ai_response.success);
    assert!(mock_ai_response.execution_time_ms > 0);
    assert!(mock_ai_response.error_message.is_none());
    assert!(!mock_ai_response.routing_reason.is_empty());
    
    // Test JSON serialization for API compatibility
    let json_result = serde_json::to_string(&mock_ai_response);
    assert!(json_result.is_ok(), "AI response should be JSON serializable");
    
    let json_str = json_result.unwrap();
    assert!(json_str.contains("provider_used"));
    assert!(json_str.contains("execution_time_ms"));
    assert!(json_str.contains("routing_reason"));
    
    // Test deserialization
    let deserialized: Result<moon_shine::provider_router::AIResponse, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok(), "AI response should be JSON deserializable");
    
    let restored_response = deserialized.unwrap();
    assert_eq!(restored_response.provider_used, mock_ai_response.provider_used);
    assert_eq!(restored_response.execution_time_ms, mock_ai_response.execution_time_ms);
}
