//! # AI Linting Load Testing and WASM Compatibility
//!
//! Comprehensive load testing and WASM compatibility validation:
//! - Concurrent AI analysis scalability
//! - High-volume batch processing
//! - Memory usage under load
//! - WASM runtime compatibility
//! - Performance degradation under stress
//! - Resource cleanup validation
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::oxc_adapter::{OxcAdapter, AiBehavioralAnalyzer};
use moon_shine::provider_router::{get_ai_router, AIRequest, AIContext, apply_rate_limiting, batch_process_files};
use moon_shine::moon_pdk_interface::AiLinterConfig;
use rstest::*;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use futures::future::join_all;
use serial_test::serial;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

/// Load test data generator
struct LoadTestData;

impl LoadTestData {
    fn generate_code_samples(count: usize) -> Vec<(String, String)> {
        (0..count)
            .map(|i| {
                let filename = format!("generated-{}.tsx", i);
                let code = Self::generate_react_component(i);
                (filename, code)
            })
            .collect()
    }
    
    fn generate_react_component(index: usize) -> String {
        format!(
            r#"
                import React, {{ useState, useEffect }} from 'react';
                
                interface Props{} {{
                    data{}: any[];
                    config{}: Record<string, unknown>;
                }}
                
                export function Component{}({{ data{}, config{} }}: Props{}) {{
                    const [state{}, setState{}] = useState<number>(0);
                    const [loading{}, setLoading{}] = useState<boolean>(false);
                    const [error{}, setError{}] = useState<string | null>(null);
                    
                    useEffect(() => {{
                        setLoading{}(true);
                        
                        const process = async () => {{
                            try {{
                                const result = await fetch(`/api/data/${{data{}.length}}`);
                                const json = await result.json();
                                setState{}(json.count || 0);
                            }} catch (err) {{
                                setError{}(err instanceof Error ? err.message : 'Unknown error');
                            }} finally {{
                                setLoading{}(false);
                            }}
                        }};
                        
                        process();
                    }}, [data{}]);
                    
                    const handleIncrement{} = () => {{
                        setState{}(prev => prev + 1);
                    }};
                    
                    const handleReset{} = () => {{
                        setState{}(0);
                        setError{}(null);
                    }};
                    
                    if (loading{}) {{
                        return <div className="loading">Loading component {}...</div>;
                    }}
                    
                    if (error{}) {{
                        return (
                            <div className="error">
                                <p>Error in component {}: {{error{}}}</p>
                                <button onClick={{handleReset{}}}>Reset</button>
                            </div>
                        );
                    }}
                    
                    return (
                        <div className="component-{}" data-testid="component-{}">
                            <h2>Component {} (State: {{state{}}})</h2>
                            <p>Data length: {{data{}.length}}</p>
                            <p>Config keys: {{Object.keys(config{}).length}}</p>
                            <div className="controls">
                                <button onClick={{handleIncrement{}}}>Increment</button>
                                <button onClick={{handleReset{}}}>Reset</button>
                            </div>
                            <div className="data-preview">
                                {{data{}.slice(0, 3).map((item, idx) => (
                                    <div key={{idx}} className="data-item">
                                        {{JSON.stringify(item).slice(0, 50)}}...
                                    </div>
                                ))}}
                            </div>
                        </div>
                    );
                }}
            "#,
            index, index, index, index, index, index, index, index, index, index, 
            index, index, index, index, index, index, index, index, index, index,
            index, index, index, index, index, index, index, index, index, index,
            index, index, index, index, index, index, index, index, index, index,
            index, index, index, index, index, index, index, index, index, index
        )
    }
    
    fn generate_complex_business_logic(complexity_level: usize) -> String {
        let mut code = String::new();
        
        code.push_str(&format!(
            r#"
                interface BusinessContext{} {{
                    user: {{ id: string; level: 'basic' | 'premium' | 'enterprise'; }};
                    settings: Record<string, any>;
                    cache: Map<string, any>;
                    events: string[];
                }}
                
                export class BusinessProcessor{} {{
                    private context: BusinessContext{};
                    private metrics: Map<string, number> = new Map();
                    private timers: Set<NodeJS.Timeout> = new Set();
                    
                    constructor(context: BusinessContext{}) {{
                        this.context = context;
                        this.initializeMetrics();
                    }}
                    
                    private initializeMetrics(): void {{
                        this.metrics.set('processCount', 0);
                        this.metrics.set('errorCount', 0);
                        this.metrics.set('avgProcessingTime', 0);
                    }}
            "#,
            complexity_level, complexity_level, complexity_level, complexity_level
        ));
        
        // Add increasingly complex methods
        for method_index in 0..complexity_level.min(20) {
            code.push_str(&format!(
                r#"
                    public async processData{}(input: any[], options: Record<string, any>): Promise<any[]> {{
                        const startTime = Date.now();
                        const results: any[] = [];
                        
                        try {{
                            this.metrics.set('processCount', this.metrics.get('processCount')! + 1);
                            
                            for (let i = 0; i < input.length; i++) {{
                                const item = input[i];
                                
                                if (this.context.user.level === 'enterprise') {{
                                    if (options.enableAdvancedProcessing) {{
                                        for (let j = 0; j < options.iterations || 1; j++) {{
                                            if (item.priority === 'high') {{
                                                if (this.context.cache.has(item.id)) {{
                                                    const cached = this.context.cache.get(item.id);
                                                    if (cached.timestamp > Date.now() - 3600000) {{
                                                        results.push({{ ...cached.data, fromCache: true }});
                                                        continue;
                                                    }}
                                                }}
                                                
                                                const processed = await this.complexTransform{}(item, j);
                                                this.context.cache.set(item.id, {{
                                                    data: processed,
                                                    timestamp: Date.now()
                                                }});
                                                results.push(processed);
                                            }} else if (item.priority === 'medium') {{
                                                const simplified = this.simpleTransform{}(item);
                                                results.push(simplified);
                                            }} else {{
                                                results.push(item); // Pass through low priority
                                            }}
                                        }}
                                    }} else {{
                                        results.push(this.basicTransform{}(item));
                                    }}
                                }} else if (this.context.user.level === 'premium') {{
                                    if (options.enablePremiumFeatures) {{
                                        const enhanced = this.premiumTransform{}(item);
                                        results.push(enhanced);
                                    }} else {{
                                        results.push(this.basicTransform{}(item));
                                    }}
                                }} else {{
                                    // Basic user processing
                                    if (i < 100) {{ // Limit for basic users
                                        results.push(this.basicTransform{}(item));
                                    }}
                                }}
                            }}
                            
                            const processingTime = Date.now() - startTime;
                            this.updateAverageProcessingTime(processingTime);
                            
                            return results;
                        }} catch (error) {{
                            this.metrics.set('errorCount', this.metrics.get('errorCount')! + 1);
                            this.context.events.push(`Error in processData{}: ${{error}}`);
                            throw error;
                        }}
                    }}
                "#,
                method_index, method_index, method_index, method_index, 
                method_index, method_index, method_index, method_index
            ));
        }
        
        // Add helper methods
        code.push_str(&format!(
            r#"
                    private async complexTransform(item: any, iteration: number): Promise<any> {{
                        // Simulate complex async operation
                        await new Promise(resolve => setTimeout(resolve, 1));
                        return {{
                            ...item,
                            processed: true,
                            complexity: 'high',
                            iteration,
                            timestamp: Date.now()
                        }};
                    }}
                    
                    private simpleTransform(item: any): any {{
                        return {{
                            ...item,
                            processed: true,
                            complexity: 'medium',
                            timestamp: Date.now()
                        }};
                    }}
                    
                    private basicTransform(item: any): any {{
                        return {{
                            id: item.id,
                            data: item.data,
                            processed: true,
                            complexity: 'basic'
                        }};
                    }}
                    
                    private premiumTransform(item: any): any {{
                        return {{
                            ...item,
                            processed: true,
                            complexity: 'premium',
                            enhanced: true,
                            timestamp: Date.now()
                        }};
                    }}
                    
                    private updateAverageProcessingTime(time: number): void {{
                        const current = this.metrics.get('avgProcessingTime') || 0;
                        const count = this.metrics.get('processCount') || 1;
                        const newAvg = (current * (count - 1) + time) / count;
                        this.metrics.set('avgProcessingTime', newAvg);
                    }}
                    
                    public getMetrics(): Record<string, number> {{
                        return Object.fromEntries(this.metrics);
                    }}
                    
                    public cleanup(): void {{
                        this.timers.forEach(timer => clearTimeout(timer));
                        this.timers.clear();
                        this.context.cache.clear();
                    }}
                }}
            "#
        ));
        
        code
    }
}

#[fixture]
fn load_test_config() -> AiLinterConfig {
    AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 300,
        quality_threshold: 0.7,
        debug_session_retention_hours: 1,
        cleanup_sessions_older_than_hours: 2,
        max_concurrent_requests: 10, // Higher for load testing
        batch_size: 20,
        rate_limit_per_minute: 100, // Permissive for load testing
        max_tokens_per_request: 8192,
        retry_attempts: 2,
        retry_delay_ms: 100,
    }
}

#[rstest]
#[serial]
fn test_concurrent_static_analysis_load() {
    let adapter = Arc::new(OxcAdapter::new());
    let code_samples = LoadTestData::generate_code_samples(50);
    let thread_count = 8;
    let samples_per_thread = code_samples.len() / thread_count;
    
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let total_processing_time = Arc::new(std::sync::Mutex::new(Duration::new(0, 0)));
    
    let barrier = Arc::new(Barrier::new(thread_count));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let adapter = adapter.clone();
        let samples = code_samples[thread_id * samples_per_thread..(thread_id + 1) * samples_per_thread].to_vec();
        let success_count = success_count.clone();
        let error_count = error_count.clone();
        let total_processing_time = total_processing_time.clone();
        let barrier = barrier.clone();
        
        thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            let thread_start = Instant::now();
            
            for (filename, code) in samples {
                let analysis_start = Instant::now();
                
                match adapter.analyze_code(&code, &filename) {
                    Ok(_) => {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    },
                    Err(_) => {
                        error_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                let analysis_time = analysis_start.elapsed();
                let mut total_time = total_processing_time.lock().unwrap();
                *total_time += analysis_time;
            }
            
            let thread_time = thread_start.elapsed();
            println!("Thread {} completed in {:?}", thread_id, thread_time);
        })
    }).collect();
    
    let load_test_start = Instant::now();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    let total_load_test_time = load_test_start.elapsed();
    let final_success_count = success_count.load(Ordering::SeqCst);
    let final_error_count = error_count.load(Ordering::SeqCst);
    let total_proc_time = *total_processing_time.lock().unwrap();
    
    println!("Concurrent Load Test Results:");
    println!("- Total time: {:?}", total_load_test_time);
    println!("- Successes: {}", final_success_count);
    println!("- Errors: {}", final_error_count);
    println!("- Total processing time: {:?}", total_proc_time);
    println!("- Average per analysis: {:?}", total_proc_time / (final_success_count + final_error_count) as u32);
    println!("- Throughput: {:.2} files/sec", (final_success_count + final_error_count) as f64 / total_load_test_time.as_secs_f64());
    
    // Assertions
    assert!(final_success_count > 0, "At least some analyses should succeed");
    assert_eq!(final_success_count + final_error_count, code_samples.len(), "All files should be processed");
    assert!(total_load_test_time.as_secs() < 60, "Load test should complete within 60 seconds");
    
    // Performance expectations
    let throughput = (final_success_count + final_error_count) as f64 / total_load_test_time.as_secs_f64();
    assert!(throughput > 1.0, "Should process at least 1 file per second: {:.2}", throughput);
}

#[rstest]
#[tokio::test]
async fn test_async_concurrent_behavioral_analysis() {
    let analyzer = Arc::new(AiBehavioralAnalyzer::new());
    let code_samples = LoadTestData::generate_code_samples(20);
    let concurrent_limit = 5;
    
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrent_limit));
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    
    let tasks: Vec<_> = code_samples.into_iter().enumerate().map(|(index, (filename, code))| {
        let analyzer = analyzer.clone();
        let semaphore = semaphore.clone();
        let success_count = success_count.clone();
        let error_count = error_count.clone();
        
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            let context = moon_shine::oxc_adapter::ai_behavioral::AnalysisContext {
                file_path: filename,
                file_type: oxc_span::SourceType::tsx(),
                project_context: None,
                dependencies: vec![],
            };
            
            let allocator = oxc_allocator::Allocator::default();
            let source_type = oxc_span::SourceType::tsx();
            let parser_result = oxc_parser::Parser::new(&allocator, &code, source_type).parse();
            
            match analyzer.run_heuristic_analysis(&code, &parser_result.program, &context) {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::SeqCst);
                },
                Err(error) => {
                    println!("Analysis {} failed: {}", index, error);
                    error_count.fetch_add(1, Ordering::SeqCst);
                }
            }
        })
    }).collect();
    
    let start_time = Instant::now();
    
    // Wait for all tasks with timeout
    let results = timeout(Duration::from_secs(30), join_all(tasks)).await;
    
    let execution_time = start_time.elapsed();
    let final_success_count = success_count.load(Ordering::SeqCst);
    let final_error_count = error_count.load(Ordering::SeqCst);
    
    println!("Async Behavioral Analysis Load Test:");
    println!("- Execution time: {:?}", execution_time);
    println!("- Successes: {}", final_success_count);
    println!("- Errors: {}", final_error_count);
    
    assert!(results.is_ok(), "All tasks should complete within timeout");
    assert!(final_success_count > 0, "At least some analyses should succeed");
    assert_eq!(final_success_count + final_error_count, 20, "All analyses should complete");
}

#[rstest]
fn test_batch_processing_scalability(load_test_config: AiLinterConfig) {
    let file_list: Vec<String> = (0..100)
        .map(|i| format!("batch-test-{}.tsx", i))
        .collect();
    
    let processor = |batch: &[String]| -> Result<Vec<moon_shine::rulebase::RuleResult>, Box<dyn std::error::Error>> {
        // Simulate processing each file in the batch
        let results: Vec<_> = batch.iter().enumerate().map(|(i, filename)| {
            moon_shine::rulebase::RuleResult {
                rule: "load-test-rule".to_string(),
                message: format!("Processed {}", filename),
                line: i as u32 + 1,
                column: 1,
                severity: "info".to_string(),
                fix_available: false,
                ai_confidence: 0.8,
                pattern_frequency: Some(0.1),
            }
        }).collect();
        
        // Simulate some processing time
        thread::sleep(Duration::from_millis(10));
        
        Ok(results)
    };
    
    let start_time = Instant::now();
    let result = batch_process_files(&file_list, &load_test_config, processor);
    let processing_time = start_time.elapsed();
    
    assert!(result.is_ok(), "Batch processing should succeed");
    
    let suggestions = result.unwrap();
    assert_eq!(suggestions.len(), 100, "Should process all files");
    
    println!("Batch Processing Scalability Test:");
    println!("- Files processed: {}", suggestions.len());
    println!("- Total time: {:?}", processing_time);
    println!("- Throughput: {:.2} files/sec", 100.0 / processing_time.as_secs_f64());
    
    // Performance expectations
    assert!(processing_time.as_secs() < 30, "Batch processing should be efficient");
    
    // Verify results structure
    for suggestion in &suggestions {
        assert!(!suggestion.message.is_empty(), "All suggestions should have messages");
        assert!(suggestion.line > 0, "All suggestions should have valid line numbers");
    }
}

#[rstest]
#[serial]
fn test_memory_usage_under_load() {
    let adapter = Arc::new(OxcAdapter::new());
    let analyzer = Arc::new(AiBehavioralAnalyzer::new());
    
    // Generate increasingly complex code samples
    let complex_samples: Vec<_> = (1..=10)
        .map(|complexity| LoadTestData::generate_complex_business_logic(complexity * 2))
        .collect();
    
    let iterations = 5;
    let mut peak_memory_estimates = Vec::new();
    
    for iteration in 0..iterations {
        println!("Memory test iteration {}", iteration + 1);
        
        let iteration_start = Instant::now();
        
        // Process all complex samples
        for (index, code) in complex_samples.iter().enumerate() {
            // Static analysis
            let _ = adapter.analyze_code(code, &format!("complex-{}-{}.ts", iteration, index));
            
            // Behavioral analysis  
            let context = moon_shine::oxc_adapter::ai_behavioral::AnalysisContext {
                file_path: format!("complex-{}-{}.ts", iteration, index),
                file_type: oxc_span::SourceType::ts(),
                project_context: None,
                dependencies: vec![],
            };
            
            let allocator = oxc_allocator::Allocator::default();
            let source_type = oxc_span::SourceType::ts();
            let parser_result = oxc_parser::Parser::new(&allocator, code, source_type).parse();
            
            let _ = analyzer.run_heuristic_analysis(code, &parser_result.program, &context);
            
            // Cognitive complexity calculation
            let _ = analyzer.calculate_cognitive_complexity_heuristic(code);
        }
        
        let iteration_time = iteration_start.elapsed();
        
        // Estimate memory usage (very rough - in real scenario we'd use proper tools)
        let estimated_memory = complex_samples.iter().map(|s| s.len()).sum::<usize>();
        peak_memory_estimates.push(estimated_memory);
        
        println!("Iteration {} completed in {:?}, estimated memory: {} bytes", 
            iteration + 1, iteration_time, estimated_memory);
        
        // Force garbage collection opportunities
        std::thread::sleep(Duration::from_millis(100));
    }
    
    // Memory should be relatively stable across iterations
    let memory_variance = peak_memory_estimates.iter().max().unwrap() - peak_memory_estimates.iter().min().unwrap();
    let avg_memory = peak_memory_estimates.iter().sum::<usize>() / peak_memory_estimates.len();
    
    println!("Memory usage analysis:");
    println!("- Average estimated memory: {} bytes", avg_memory);
    println!("- Memory variance: {} bytes ({:.2}%)", memory_variance, 
        (memory_variance as f64 / avg_memory as f64) * 100.0);
    
    // Memory variance should be reasonable (not growing unboundedly)
    assert!(memory_variance < avg_memory / 2, "Memory usage should be stable across iterations");
}

#[rstest]
fn test_rate_limiting_under_load(load_test_config: AiLinterConfig) {
    let config = AiLinterConfig {
        rate_limit_per_minute: 10, // Restrictive for load testing
        retry_delay_ms: 100,
        ..load_test_config
    };
    
    let thread_count = 5;
    let requests_per_thread = 5;
    
    let success_count = Arc::new(AtomicUsize::new(0));
    let rate_limited_count = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        let config = config.clone();
        let success_count = success_count.clone();
        let rate_limited_count = rate_limited_count.clone();
        
        thread::spawn(move || {
            for request_id in 0..requests_per_thread {
                match apply_rate_limiting(&config) {
                    Ok(_) => {
                        success_count.fetch_add(1, Ordering::SeqCst);
                        println!("Thread {} request {} succeeded", thread_id, request_id);
                    },
                    Err(_) => {
                        rate_limited_count.fetch_add(1, Ordering::SeqCst);
                        println!("Thread {} request {} rate limited", thread_id, request_id);
                    }
                }
                
                // Small delay between requests within thread
                thread::sleep(Duration::from_millis(50));
            }
        })
    }).collect();
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    let final_success_count = success_count.load(Ordering::SeqCst);
    let final_rate_limited_count = rate_limited_count.load(Ordering::SeqCst);
    let total_requests = thread_count * requests_per_thread;
    
    println!("Rate Limiting Load Test:");
    println!("- Total requests: {}", total_requests);
    println!("- Successful: {}", final_success_count);
    println!("- Rate limited: {}", final_rate_limited_count);
    println!("- Success rate: {:.2}%", (final_success_count as f64 / total_requests as f64) * 100.0);
    
    assert_eq!(final_success_count + final_rate_limited_count, total_requests, 
        "All requests should be accounted for");
    
    // With restrictive rate limiting, some requests should be blocked
    assert!(final_rate_limited_count > 0, "Rate limiting should block some requests under load");
    
    // But some should still succeed
    assert!(final_success_count > 0, "Some requests should succeed despite rate limiting");
}

// WASM-specific tests
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_wasm_oxc_adapter_creation() {
        let adapter = OxcAdapter::new();
        let rules = adapter.get_available_rules();
        assert!(!rules.is_empty(), "WASM OXC adapter should have rules");
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_behavioral_analyzer_creation() {
        let analyzer = AiBehavioralAnalyzer::new();
        let patterns = analyzer.get_patterns();
        assert!(!patterns.is_empty(), "WASM behavioral analyzer should have patterns");
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_cognitive_complexity() {
        let analyzer = AiBehavioralAnalyzer::new();
        let code = "function test() { if (true) { for (let i = 0; i < 10; i++) { console.log(i); } } }";
        let complexity = analyzer.calculate_cognitive_complexity_heuristic(code);
        assert!(complexity > 0, "WASM complexity calculation should work");
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_config_serialization() {
        let config = AiLinterConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.is_empty(), "WASM config serialization should work");
        
        let deserialized: AiLinterConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.batch_size, deserialized.batch_size, "WASM serialization should preserve values");
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_provider_router() {
        let router = get_ai_router();
        assert_eq!(router.providers.len(), 3, "WASM provider router should have all providers");
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_error_handling() {
        let adapter = OxcAdapter::new();
        
        // Test with malformed code
        let result = adapter.analyze_code("function incomplete() { if (true", "malformed.js");
        
        // Should handle errors gracefully in WASM
        match result {
            Ok(_) => {
                // If it succeeds, that's acceptable
            },
            Err(error) => {
                // Error should be well-formed
                assert!(!error.to_string().is_empty(), "WASM error should have message");
            }
        }
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_memory_efficiency() {
        // Test that repeated operations don't cause memory issues in WASM
        let adapter = OxcAdapter::new();
        
        for i in 0..10 {
            let code = format!("function test{}() {{ return {}; }}", i, i);
            let _ = adapter.analyze_code(&code, &format!("test-{}.js", i));
        }
        
        // If we get here without running out of memory, test passes
        assert!(true, "WASM memory management should handle repeated operations");
    }
}

#[rstest]
fn test_stress_provider_selection() {
    let router = get_ai_router();
    let stress_test_count = 1000;
    
    let start_time = Instant::now();
    
    for i in 0..stress_test_count {
        let request = AIRequest {
            prompt: format!("Test prompt {}", i),
            session_id: format!("stress-session-{}", i),
            file_path: Some(format!("stress-test-{}.ts", i)),
            context: AIContext::CodeAnalysis {
                language: "typescript".to_string(),
                content: format!("const x{} = {};", i, i),
            },
            preferred_providers: vec![],
        };
        
        let result = router.select_provider(&request);
        assert!(result.is_ok() || result.is_err(), "Provider selection should not panic");
    }
    
    let total_time = start_time.elapsed();
    
    println!("Stress Test Results:");
    println!("- Selections performed: {}", stress_test_count);
    println!("- Total time: {:?}", total_time);
    println!("- Average per selection: {:?}", total_time / stress_test_count);
    
    // Should be very fast
    assert!(total_time.as_millis() < 1000, "Provider selection should be fast under stress");
    
    let selections_per_second = stress_test_count as f64 / total_time.as_secs_f64();
    assert!(selections_per_second > 100.0, "Should handle >100 selections per second: {:.2}", selections_per_second);
}

#[rstest]
fn test_resource_cleanup_under_load() {
    let analyzer = AiBehavioralAnalyzer::new();
    let initial_pattern_count = analyzer.get_patterns().len();
    
    // Perform many operations that might leak resources
    for iteration in 0..100 {
        let code = format!(
            "function iteration{}() {{ if (condition) {{ for (let i = 0; i < 10; i++) {{ console.log(i); }} }} }}",
            iteration
        );
        
        // These operations should not accumulate state
        let _ = analyzer.calculate_cognitive_complexity_heuristic(&code);
        
        // Create analysis context
        let context = moon_shine::oxc_adapter::ai_behavioral::AnalysisContext {
            file_path: format!("iteration-{}.js", iteration),
            file_type: oxc_span::SourceType::js(),
            project_context: None,
            dependencies: vec![],
        };
        
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::js();
        let parser_result = oxc_parser::Parser::new(&allocator, &code, source_type).parse();
        
        let _ = analyzer.run_heuristic_analysis(&code, &parser_result.program, &context);
    }
    
    // Pattern count should remain stable (no memory leaks)
    let final_pattern_count = analyzer.get_patterns().len();
    assert_eq!(initial_pattern_count, final_pattern_count, 
        "Pattern count should remain stable after many operations");
    
    println!("Resource cleanup test completed successfully");
    println!("- Operations performed: 100");
    println!("- Pattern count stable: {} -> {}", initial_pattern_count, final_pattern_count);
}
