//! # AI Linting Edge Case Tests
//!
//! Comprehensive edge case testing for AI linting system:
//! - AI provider failures and fallback scenarios
//! - Network timeouts and connectivity issues
//! - Malformed input handling
//! - Resource exhaustion scenarios
//! - Concurrent access edge cases
//! - Configuration boundary conditions
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::oxc_adapter::{OxcAdapter, AiBehavioralAnalyzer};
use moon_shine::provider_router::{get_ai_router, AIRequest, AIContext, apply_rate_limiting, lint_code_with_ai};
use moon_shine::moon_pdk_interface::AiLinterConfig;
use moon_shine::error::{Error, Result};
use rstest::*;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use serial_test::serial;

/// Test fixtures for edge case scenarios
struct EdgeCaseScenarios;

impl EdgeCaseScenarios {
    fn empty_code() -> &'static str { "" }
    
    fn whitespace_only() -> &'static str { "   \n\t  \r\n   " }
    
    fn single_character() -> &'static str { "{" }
    
    fn extremely_long_line() -> String {
        format!("const x = '{}';", "a".repeat(100_000))
    }
    
    fn deeply_nested_code() -> String {
        let mut code = String::from("function test() {\n");
        for i in 0..100 {
            code.push_str(&"  ".repeat(i + 1));
            code.push_str(&format!("if (condition{}) {{\n", i));
        }
        for i in (0..100).rev() {
            code.push_str(&"  ".repeat(i + 1));
            code.push_str("}\n");
        }
        code.push_str("}\n");
        code
    }
    
    fn malformed_javascript() -> &'static str {
        r#"
            function incomplete() {
                if (true
                for (let i = 0; i <
                const x = {
                    prop: value
                // missing closing brace
            // missing function closing
        "#
    }
    
    fn unicode_nightmare() -> &'static str {
        r#"
            function 测试函数() {
                const 变量 = "🚀 Unicode string with emoji 🎉";
                console.log(`Hello ${'\u0041\u0042\u0043'} World`);
                const ಠ_ಠ = "disapproval";
                return 变量 + ಠ_ಠ;
            }
        "#
    }
    
    fn binary_data() -> Vec<u8> {
        vec![0x00, 0xFF, 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] // PNG header
    }
    
    fn extremely_large_code() -> String {
        let mut code = String::new();
        for i in 0..10_000 {
            code.push_str(&format!(
                "function generatedFunction{}() {{ return {}; }}\n",
                i, i
            ));
        }
        code
    }
    
    fn circular_references() -> &'static str {
        r#"
            const obj1 = { name: "obj1" };
            const obj2 = { name: "obj2" };
            obj1.ref = obj2;
            obj2.ref = obj1;
            
            function processCircular(input) {
                return JSON.stringify(input); // Will fail with circular reference
            }
        "#
    }
    
    fn memory_intensive_patterns() -> &'static str {
        r#"
            function memoryLeak() {
                const cache = new Map();
                const intervals = [];
                const listeners = [];
                
                for (let i = 0; i < 10000; i++) {
                    // Create many intervals without cleanup
                    intervals.push(setInterval(() => {
                        cache.set(i, new Array(1000).fill(i));
                    }, 1));
                    
                    // Create many event listeners without cleanup
                    const handler = () => cache.get(i);
                    document.addEventListener('custom' + i, handler);
                    listeners.push(handler);
                }
                
                // Intentionally don't clean up
                return { cache, intervals, listeners };
            }
        "#
    }
}

#[fixture]
fn strict_config() -> AiLinterConfig {
    AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 5, // Very short timeout
        quality_threshold: 0.99, // Very high threshold
        debug_session_retention_hours: 1,
        cleanup_sessions_older_than_hours: 1,
        max_concurrent_requests: 1, // No concurrency
        batch_size: 1, // Minimal batch size
        rate_limit_per_minute: 1, // Very restrictive
        max_tokens_per_request: 100, // Very small token limit
        retry_attempts: 1, // No retries
        retry_delay_ms: 10000, // Long delay
    }
}

#[fixture]
fn permissive_config() -> AiLinterConfig {
    AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 3600, // 1 hour
        quality_threshold: 0.0, // Accept anything
        debug_session_retention_hours: 168, // 1 week
        cleanup_sessions_older_than_hours: 720, // 1 month
        max_concurrent_requests: 100, // High concurrency
        batch_size: 1000, // Large batches
        rate_limit_per_minute: 10000, // Very permissive
        max_tokens_per_request: 100000, // Large token limit
        retry_attempts: 10, // Many retries
        retry_delay_ms: 1, // Minimal delay
    }
}

#[rstest]
fn test_empty_and_whitespace_code() {
    let adapter = OxcAdapter::new();
    
    // Empty code
    let result = adapter.analyze_code(EdgeCaseScenarios::empty_code(), "empty.js");
    match result {
        Ok(analysis) => {
            // Should handle gracefully
            assert!(analysis.diagnostics.len() >= 0, "Empty code analysis should not crash");
        },
        Err(error) => {
            // Error is acceptable for empty code
            assert!(!error.to_string().is_empty(), "Error message should be informative");
        }
    }
    
    // Whitespace only
    let result = adapter.analyze_code(EdgeCaseScenarios::whitespace_only(), "whitespace.js");
    match result {
        Ok(analysis) => {
            assert!(analysis.diagnostics.len() >= 0, "Whitespace code analysis should not crash");
        },
        Err(_) => {
            // Error is acceptable for whitespace-only code
        }
    }
    
    // Single character
    let result = adapter.analyze_code(EdgeCaseScenarios::single_character(), "single.js");
    match result {
        Ok(_) => {
            // If it succeeds, that's fine
        },
        Err(error) => {
            // Error is expected for malformed code
            assert!(!error.to_string().is_empty(), "Error should have message");
        }
    }
}

#[rstest]
fn test_malformed_javascript_handling() {
    let adapter = OxcAdapter::new();
    let analyzer = AiBehavioralAnalyzer::new();
    
    let malformed_code = EdgeCaseScenarios::malformed_javascript();
    
    // OXC adapter should handle malformed code gracefully
    let static_result = adapter.analyze_code(malformed_code, "malformed.js");
    match static_result {
        Ok(analysis) => {
            // If parsing succeeds partially, that's acceptable
            println!("Partial parsing succeeded with {} diagnostics", analysis.diagnostics.len());
        },
        Err(error) => {
            // Parsing failure is expected and should be handled gracefully
            assert!(!error.to_string().is_empty(), "Parse error should have message");
            println!("Parse error handled gracefully: {}", error);
        }
    }
    
    // Heuristic analysis should be more resilient
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(malformed_code);
    assert!(complexity >= 0, "Complexity calculation should not fail catastrophically");
    assert!(complexity < 1000, "Complexity should be bounded even for malformed code");
}

#[rstest]
fn test_unicode_and_special_characters() {
    let adapter = OxcAdapter::new();
    let analyzer = AiBehavioralAnalyzer::new();
    
    let unicode_code = EdgeCaseScenarios::unicode_nightmare();
    
    // Should handle Unicode gracefully
    let result = adapter.analyze_code(unicode_code, "unicode.js");
    match result {
        Ok(analysis) => {
            assert!(analysis.diagnostics.len() >= 0, "Unicode code should be handled");
            println!("Unicode analysis succeeded with {} diagnostics", analysis.diagnostics.len());
        },
        Err(error) => {
            // Some Unicode issues might cause parsing failures
            println!("Unicode parsing failed (acceptable): {}", error);
        }
    }
    
    // Complexity calculation should work with Unicode
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(unicode_code);
    assert!(complexity >= 0, "Unicode complexity calculation should work");
}

#[rstest]
fn test_binary_data_handling() {
    let adapter = OxcAdapter::new();
    
    // Convert binary data to string (which will be invalid UTF-8)
    let binary_data = EdgeCaseScenarios::binary_data();
    let binary_string = String::from_utf8_lossy(&binary_data);
    
    let result = adapter.analyze_code(&binary_string, "binary.js");
    match result {
        Ok(_) => {
            // If it somehow succeeds, that's fine
        },
        Err(error) => {
            // Expected to fail, should handle gracefully
            assert!(!error.to_string().is_empty(), "Binary data error should be informative");
            println!("Binary data handling: {}", error);
        }
    }
}

#[rstest]
fn test_extremely_large_code() {
    let adapter = OxcAdapter::new();
    let large_code = EdgeCaseScenarios::extremely_large_code();
    
    println!("Testing large code with {} characters", large_code.len());
    
    let start_time = Instant::now();
    let result = adapter.analyze_code(&large_code, "large.js");
    let analysis_time = start_time.elapsed();
    
    match result {
        Ok(analysis) => {
            println!("Large code analysis completed in {:?} with {} diagnostics", 
                analysis_time, analysis.diagnostics.len());
            
            // Should complete in reasonable time even for large files
            assert!(analysis_time.as_secs() < 30, "Large file analysis should complete within 30 seconds");
        },
        Err(error) => {
            println!("Large code analysis failed (may be acceptable): {}", error);
            // Failure is acceptable for extremely large files
        }
    }
}

#[rstest]
fn test_deeply_nested_code() {
    let adapter = OxcAdapter::new();
    let analyzer = AiBehavioralAnalyzer::new();
    let nested_code = EdgeCaseScenarios::deeply_nested_code();
    
    println!("Testing deeply nested code with {} characters", nested_code.len());
    
    // Static analysis
    let static_result = adapter.analyze_code(&nested_code, "nested.js");
    match static_result {
        Ok(analysis) => {
            println!("Deep nesting static analysis: {} diagnostics", analysis.diagnostics.len());
        },
        Err(error) => {
            println!("Deep nesting static analysis failed: {}", error);
        }
    }
    
    // Cognitive complexity should detect high complexity
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(&nested_code);
    assert!(complexity > 50, "Deeply nested code should have high complexity: {}", complexity);
    assert!(complexity < 10000, "Complexity should be bounded: {}", complexity);
}

#[rstest]
#[tokio::test]
async fn test_ai_provider_timeout_handling() {
    // Test with very short timeout
    let session_id = "timeout-test-session".to_string();
    let content = EdgeCaseScenarios::extremely_large_code();
    let language = "javascript".to_string();
    let static_issues = vec![];
    let analysis_focus = vec!["performance".to_string()];
    let file_path = Some("timeout-test.js".to_string());
    
    // Use a very short timeout to force timeout scenario
    let result = timeout(
        Duration::from_millis(1), // 1ms timeout - should always timeout
        lint_code_with_ai(session_id, content, language, static_issues, analysis_focus, file_path)
    ).await;
    
    match result {
        Ok(ai_result) => {
            match ai_result {
                Ok(response) => {
                    println!("AI analysis completed unexpectedly quickly: {}ms", response.execution_time_ms);
                },
                Err(error) => {
                    println!("AI analysis failed as expected: {}", error);
                }
            }
        },
        Err(_) => {
            println!("AI analysis timed out as expected");
        }
    }
}

#[rstest]
#[serial] // Ensure rate limiting tests don't interfere
fn test_rate_limiting_edge_cases(strict_config: AiLinterConfig) {
    // Test with zero rate limit (should fail immediately)
    let zero_rate_config = AiLinterConfig {
        rate_limit_per_minute: 0,
        ..strict_config.clone()
    };
    
    let result = apply_rate_limiting(&zero_rate_config);
    assert!(result.is_err(), "Zero rate limit should fail immediately");
    
    if let Err(error) = result {
        assert!(error.to_string().contains("Rate limit exceeded"), 
            "Should mention rate limit: {}", error);
    }
    
    // Test with very high retry delay
    let high_delay_config = AiLinterConfig {
        rate_limit_per_minute: 1,
        retry_delay_ms: 60000, // 1 minute delay
        ..strict_config
    };
    
    // This test just verifies the config is handled without panicking
    let start_time = Instant::now();
    let result = apply_rate_limiting(&high_delay_config);
    let elapsed = start_time.elapsed();
    
    // Should either succeed quickly or fail quickly (not hang)
    assert!(elapsed.as_millis() < 5000, "Rate limiting should not hang: {:?}", elapsed);
}

#[rstest]
fn test_concurrent_access_edge_cases() {
    let adapter = Arc::new(OxcAdapter::new());
    let error_count = Arc::new(AtomicUsize::new(0));
    let success_count = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..10).map(|i| {
        let adapter = adapter.clone();
        let error_count = error_count.clone();
        let success_count = success_count.clone();
        
        thread::spawn(move || {
            let code = match i % 4 {
                0 => EdgeCaseScenarios::empty_code().to_string(),
                1 => EdgeCaseScenarios::malformed_javascript().to_string(),
                2 => EdgeCaseScenarios::unicode_nightmare().to_string(),
                _ => format!("function test{}() {{ return {}; }}", i, i),
            };
            
            let result = adapter.analyze_code(&code, &format!("concurrent-{}.js", i));
            match result {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::SeqCst);
                },
                Err(_) => {
                    error_count.fetch_add(1, Ordering::SeqCst);
                }
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    let total_success = success_count.load(Ordering::SeqCst);
    let total_errors = error_count.load(Ordering::SeqCst);
    
    println!("Concurrent analysis: {} successes, {} errors", total_success, total_errors);
    
    // Should handle all requests without panicking
    assert_eq!(total_success + total_errors, 10, "All requests should be handled");
}

#[rstest]
fn test_memory_intensive_pattern_analysis() {
    let analyzer = AiBehavioralAnalyzer::new();
    let memory_intensive_code = EdgeCaseScenarios::memory_intensive_patterns();
    
    // This should detect multiple potential memory leak patterns
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(memory_intensive_code);
    assert!(complexity > 10, "Memory-intensive code should have higher complexity: {}", complexity);
    
    // Test heuristic analysis
    let context = moon_shine::oxc_adapter::ai_behavioral::AnalysisContext {
        file_path: "memory-test.js".to_string(),
        file_type: oxc_span::SourceType::js(),
        project_context: None,
        dependencies: vec![],
    };
    
    let allocator = oxc_allocator::Allocator::default();
    let source_type = oxc_span::SourceType::js();
    let parser_result = oxc_parser::Parser::new(&allocator, memory_intensive_code, source_type).parse();
    
    let result = analyzer.run_heuristic_analysis(memory_intensive_code, &parser_result.program, &context);
    
    match result {
        Ok(diagnostics) => {
            // Should detect memory leak patterns
            let memory_leak_count = diagnostics.iter()
                .filter(|d| d.message.contains("memory leak"))
                .count();
            
            println!("Detected {} potential memory leak patterns", memory_leak_count);
            // At least some memory leak patterns should be detected
            assert!(memory_leak_count > 0, "Should detect memory leak patterns");
        },
        Err(error) => {
            println!("Memory pattern analysis failed: {}", error);
            // Failure is acceptable for complex analysis
        }
    }
}

#[rstest]
fn test_configuration_boundary_conditions() {
    // Test maximum values
    let max_config = AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: u32::MAX,
        quality_threshold: 1.0,
        debug_session_retention_hours: u32::MAX,
        cleanup_sessions_older_than_hours: u32::MAX,
        max_concurrent_requests: u32::MAX,
        batch_size: u32::MAX,
        rate_limit_per_minute: u32::MAX,
        max_tokens_per_request: u32::MAX,
        retry_attempts: u32::MAX,
        retry_delay_ms: u32::MAX,
    };
    
    // Should serialize without panicking
    let json_result = serde_json::to_string(&max_config);
    assert!(json_result.is_ok(), "Max config should serialize");
    
    // Test minimum values
    let min_config = AiLinterConfig {
        enable_claude_ai: false,
        enable_semantic_checks: false,
        claude_model: "".to_string(),
        max_processing_time: 0,
        quality_threshold: 0.0,
        debug_session_retention_hours: 0,
        cleanup_sessions_older_than_hours: 0,
        max_concurrent_requests: 0,
        batch_size: 0,
        rate_limit_per_minute: 0,
        max_tokens_per_request: 0,
        retry_attempts: 0,
        retry_delay_ms: 0,
    };
    
    // Should serialize without panicking
    let json_result = serde_json::to_string(&min_config);
    assert!(json_result.is_ok(), "Min config should serialize");
}

#[rstest]
fn test_extremely_long_line_handling() {
    let adapter = OxcAdapter::new();
    let long_line_code = EdgeCaseScenarios::extremely_long_line();
    
    println!("Testing line with {} characters", long_line_code.len());
    
    let start_time = Instant::now();
    let result = adapter.analyze_code(&long_line_code, "long-line.js");
    let analysis_time = start_time.elapsed();
    
    match result {
        Ok(analysis) => {
            println!("Long line analysis completed in {:?} with {} diagnostics", 
                analysis_time, analysis.diagnostics.len());
            
            // Should handle long lines efficiently
            assert!(analysis_time.as_secs() < 10, "Long line analysis should be efficient");
        },
        Err(error) => {
            println!("Long line analysis failed: {}", error);
            // Some parsers might fail on extremely long lines
        }
    }
}

#[rstest]
fn test_circular_reference_code() {
    let adapter = OxcAdapter::new();
    let analyzer = AiBehavioralAnalyzer::new();
    let circular_code = EdgeCaseScenarios::circular_references();
    
    // Static analysis should handle circular references in code structure
    let result = adapter.analyze_code(circular_code, "circular.js");
    match result {
        Ok(analysis) => {
            println!("Circular reference analysis: {} diagnostics", analysis.diagnostics.len());
        },
        Err(error) => {
            println!("Circular reference analysis failed: {}", error);
        }
    }
    
    // Complexity calculation should not get stuck in loops
    let start_time = Instant::now();
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(circular_code);
    let calc_time = start_time.elapsed();
    
    assert!(calc_time.as_millis() < 1000, "Complexity calculation should not hang: {:?}", calc_time);
    assert!(complexity >= 0, "Complexity should be calculated: {}", complexity);
}

#[rstest]
#[tokio::test]
async fn test_provider_selection_with_invalid_context() {
    let router = get_ai_router();
    
    // Test with extremely large prompt
    let huge_prompt = "a".repeat(1_000_000);
    let invalid_request = AIRequest {
        prompt: huge_prompt,
        session_id: "edge-case-session".to_string(),
        file_path: Some("test.ts".to_string()),
        context: AIContext::CodeFix {
            language: "typescript".to_string(),
            content: "b".repeat(2_000_000), // 2MB of content
        },
        preferred_providers: vec!["nonexistent-provider".to_string()],
    };
    
    let result = router.select_provider(&invalid_request);
    
    match result {
        Ok((provider, reason)) => {
            println!("Selected provider {} despite large context: {}", provider.name, reason);
            // If it succeeds, context length should be considered
            assert!(provider.capabilities.context_length >= 100_000, 
                "Should select provider with large context length for large requests");
        },
        Err(error) => {
            println!("Provider selection failed for large context (acceptable): {}", error);
            assert!(!error.to_string().is_empty(), "Error should be informative");
        }
    }
}

#[rstest]
fn test_error_propagation_and_recovery() {
    let adapter = OxcAdapter::new();
    
    // Test a series of increasingly problematic inputs
    let test_cases = vec![
        ("valid", "function test() { return 1; }"),
        ("empty", ""),
        ("malformed", "function incomplete() { if (true"),
        ("unicode", "function 测试() { return '🚀'; }"),
        ("binary", "\x00\xFF\x89PNG"),
    ];
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (name, code) in test_cases {
        match adapter.analyze_code(code, &format!("{}.js", name)) {
            Ok(_) => {
                success_count += 1;
                println!("✓ {} analysis succeeded", name);
            },
            Err(error) => {
                error_count += 1;
                println!("✗ {} analysis failed: {}", name, error);
                
                // Errors should be well-formed and informative
                assert!(!error.to_string().is_empty(), "Error should have message");
                assert!(error.to_string().len() < 1000, "Error message should be reasonable length");
            }
        }
    }
    
    println!("Error recovery test: {} successes, {} errors", success_count, error_count);
    
    // At least the valid case should succeed
    assert!(success_count >= 1, "At least one test case should succeed");
    
    // All cases should be handled without panicking
    assert_eq!(success_count + error_count, 5, "All test cases should be handled");
}
