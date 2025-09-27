//! # AI Linting Performance Benchmarks
//!
//! Comprehensive performance benchmarks comparing:
//! - AI analysis vs static analysis performance
//! - Different AI providers speed and accuracy
//! - Batch processing efficiency
//! - Memory usage and resource consumption
//! - Concurrent analysis scalability
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use moon_shine::oxc_adapter::{OxcAdapter, AiBehavioralAnalyzer, AnalysisContext, ProjectContext};
use moon_shine::provider_router::{get_ai_router, AIRequest, AIContext, apply_rate_limiting};
use moon_shine::moon_pdk_interface::AiLinterConfig;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Test data generator for benchmarks
struct BenchmarkCodeSamples;

impl BenchmarkCodeSamples {
    fn small_component() -> &'static str {
        r#"
            function SmallComponent({ value }) {
                const [state, setState] = useState(value);
                return <div onClick={() => setState(state + 1)}>{state}</div>;
            }
        "#
    }
    
    fn medium_component() -> String {
        let mut code = String::new();
        code.push_str("import React, { useState, useEffect } from 'react';\n\n");
        
        for i in 0..10 {
            code.push_str(&format!(
                r#"
                function Component{}({{ data{} }}) {{
                    const [state{}, setState{}] = useState(0);
                    const [loading{}, setLoading{}] = useState(false);
                    
                    useEffect(() => {{
                        setLoading{}(true);
                        setTimeout(() => {{
                            setState{}(data{}.length);
                            setLoading{}(false);
                        }}, 100);
                    }}, [data{}]);
                    
                    const handleClick{} = () => {{
                        setState{}(prev => prev + 1);
                    }};
                    
                    return (
                        <div onClick={{handleClick{}}}>
                            {{loading{} ? 'Loading...' : state{}}}
                        </div>
                    );
                }}
                "#,
                i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i
            ));
        }
        
        code
    }
    
    fn large_component() -> String {
        let mut code = String::new();
        code.push_str("import React, { useState, useEffect, useCallback, useMemo } from 'react';\n\n");
        
        for i in 0..50 {
            code.push_str(&format!(
                r#"
                function LargeComponent{}({{ data{}, config{}, handlers{} }}) {{
                    const [state{}, setState{}] = useState({{}});
                    const [cache{}, setCache{}] = useState(new Map());
                    const [errors{}, setErrors{}] = useState([]);
                    
                    const processedData{} = useMemo(() => {{
                        return data{}.filter(item => item.active)
                                   .map(item => ({{ ...item, processed: true }}))
                                   .sort((a, b) => a.priority - b.priority);
                    }}, [data{}]);
                    
                    const handleUpdate{} = useCallback((id, updates) => {{
                        setState{}(prev => ({{
                            ...prev,
                            [id]: {{ ...prev[id], ...updates }}
                        }}));
                        
                        if (cache{}.has(id)) {{
                            const newCache = new Map(cache{});
                            newCache.delete(id);
                            setCache{}(newCache);
                        }}
                    }}, [cache{}]);
                    
                    useEffect(() => {{
                        const validateData = async () => {{
                            const newErrors = [];
                            for (const item of processedData{}) {{
                                if (!item.id || !item.name) {{
                                    newErrors.push(`Invalid item: ${{item.id || 'unknown'}}`);
                                }}
                            }}
                            setErrors{}(newErrors);
                        }};
                        
                        validateData();
                    }}, [processedData{}]);
                    
                    return (
                        <div className="large-component-{{}}">
                            {{errors{}.length > 0 && (
                                <div className="errors">
                                    {{errors{}.map((error, idx) => (
                                        <div key={{idx}} className="error">{{error}}</div>
                                    ))}}
                                </div>
                            )}}
                            <div className="content">
                                {{processedData{}.map(item => (
                                    <div key={{item.id}} onClick={{() => handleUpdate{}(item.id, {{ clicked: true }})}}>
                                        {{item.name}} - {{state{}[item.id]?.clicked ? 'Clicked' : 'Not clicked'}}
                                    </div>
                                ))}}
                            </div>
                        </div>
                    );
                }}
                "#,
                i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i
            ));
        }
        
        code
    }
    
    fn complex_business_logic() -> String {
        let mut code = String::new();
        
        for i in 0..20 {
            code.push_str(&format!(
                r#"
                function businessLogic{}(input, context) {{
                    if (input && input.type === 'order') {{
                        for (let j = 0; j < input.items.length; j++) {{
                            const item = input.items[j];
                            if (item.category === 'electronics') {{
                                if (context.user.membershipLevel === 'premium') {{
                                    if (context.discounts && context.discounts.electronics) {{
                                        for (let k = 0; k < context.discounts.electronics.length; k++) {{
                                            const discount = context.discounts.electronics[k];
                                            if (discount.type === 'percentage') {{
                                                item.price = item.price * (1 - discount.value / 100);
                                            }} else if (discount.type === 'fixed') {{
                                                item.price = Math.max(0, item.price - discount.value);
                                            }} else if (discount.type === 'bogo' && item.quantity >= 2) {{
                                                item.price = item.price * item.quantity * 0.5;
                                            }}
                                        }}
                                    }}
                                }} else if (context.user.membershipLevel === 'standard') {{
                                    // Standard user logic
                                    if (item.price > 100) {{
                                        item.price = item.price * 0.95;
                                    }}
                                }}
                            }} else if (item.category === 'books') {{
                                // Book-specific logic
                                if (item.author && context.preferences.favoriteAuthors.includes(item.author)) {{
                                    item.priority = 1;
                                }}
                            }}
                        }}
                    }} else if (input && input.type === 'subscription') {{
                        // Subscription logic
                        const plan = context.plans[input.planId];
                        if (plan) {{
                            input.monthlyPrice = plan.basePrice;
                            if (context.user.membershipLevel === 'premium') {{
                                input.monthlyPrice = input.monthlyPrice * 0.8;
                            }}
                        }}
                    }}
                    return input;
                }}
                "#,
                i
            ));
        }
        
        code
    }
}

/// Helper to create analysis context for benchmarks
fn create_benchmark_context(file_path: &str) -> AnalysisContext {
    AnalysisContext {
        file_path: file_path.to_string(),
        file_type: oxc_span::SourceType::tsx(),
        project_context: Some(ProjectContext {
            framework: Some("React".to_string()),
            build_tool: Some("Vite".to_string()),
            testing_framework: Some("Jest".to_string()),
            package_json_dependencies: HashMap::new(),
        }),
        dependencies: vec![],
    }
}

/// Benchmark static analysis performance
fn bench_static_analysis(c: &mut Criterion) {
    let adapter = OxcAdapter::new();
    
    let test_cases = vec![
        ("small", BenchmarkCodeSamples::small_component()),
        ("medium", &BenchmarkCodeSamples::medium_component()),
        ("large", &BenchmarkCodeSamples::large_component()),
        ("complex", &BenchmarkCodeSamples::complex_business_logic()),
    ];
    
    let mut group = c.benchmark_group("static_analysis");
    
    for (size, code) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("oxc_analysis", size),
            &code,
            |b, code| {
                b.iter(|| {
                    let result = adapter.analyze_code(
                        black_box(code),
                        black_box(&format!("test-{}.tsx", size))
                    );
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark behavioral analysis performance
fn bench_behavioral_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let analyzer = AiBehavioralAnalyzer::new();
    
    let test_cases = vec![
        ("small", BenchmarkCodeSamples::small_component()),
        ("medium", &BenchmarkCodeSamples::medium_component()),
        ("large", &BenchmarkCodeSamples::large_component()),
    ];
    
    let mut group = c.benchmark_group("behavioral_analysis");
    group.sample_size(10); // Fewer samples for potentially slower AI analysis
    group.measurement_time(Duration::from_secs(30));
    
    for (size, code) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("heuristic_analysis", size),
            &code,
            |b, code| {
                b.iter(|| {
                    let context = create_benchmark_context(&format!("bench-{}.tsx", size));
                    let allocator = oxc_allocator::Allocator::default();
                    let source_type = oxc_span::SourceType::tsx();
                    let parser_result = oxc_parser::Parser::new(&allocator, code, source_type).parse();
                    
                    let result = analyzer.run_heuristic_analysis(
                        black_box(code),
                        black_box(&parser_result.program),
                        black_box(&context)
                    );
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark cognitive complexity calculation
fn bench_cognitive_complexity(c: &mut Criterion) {
    let analyzer = AiBehavioralAnalyzer::new();
    
    let test_cases = vec![
        ("simple", "function simple() { return 1; }"),
        ("medium", "function medium(x) { if (x > 0) { for (let i = 0; i < x; i++) { if (i % 2) { return i; } } } return 0; }"),
        ("complex", &BenchmarkCodeSamples::complex_business_logic()),
    ];
    
    let mut group = c.benchmark_group("cognitive_complexity");
    
    for (complexity, code) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("calculate_complexity", complexity),
            &code,
            |b, code| {
                b.iter(|| {
                    let score = analyzer.calculate_cognitive_complexity_heuristic(black_box(code));
                    black_box(score)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark provider selection performance
fn bench_provider_selection(c: &mut Criterion) {
    let router = get_ai_router();
    
    let test_requests = vec![
        (
            "code_fix",
            AIRequest {
                prompt: "Fix this TypeScript code".to_string(),
                session_id: "bench-session".to_string(),
                file_path: Some("test.ts".to_string()),
                context: AIContext::CodeFix {
                    language: "typescript".to_string(),
                    content: BenchmarkCodeSamples::small_component().to_string(),
                },
                preferred_providers: vec![],
            }
        ),
        (
            "ai_linting",
            AIRequest {
                prompt: "Analyze for behavioral patterns".to_string(),
                session_id: "bench-session-2".to_string(),
                file_path: Some("component.tsx".to_string()),
                context: AIContext::AiLinting {
                    language: "typescript".to_string(),
                    content: BenchmarkCodeSamples::medium_component(),
                    static_issues: vec!["Missing dependency".to_string()],
                    analysis_focus: vec!["performance".to_string(), "react".to_string()],
                },
                preferred_providers: vec![],
            }
        ),
        (
            "code_analysis",
            AIRequest {
                prompt: "Analyze this complex code".to_string(),
                session_id: "bench-session-3".to_string(),
                file_path: Some("business-logic.ts".to_string()),
                context: AIContext::CodeAnalysis {
                    language: "typescript".to_string(),
                    content: BenchmarkCodeSamples::complex_business_logic(),
                },
                preferred_providers: vec![],
            }
        ),
    ];
    
    let mut group = c.benchmark_group("provider_selection");
    
    for (request_type, request) in test_requests {
        group.bench_with_input(
            BenchmarkId::new("select_provider", request_type),
            &request,
            |b, request| {
                b.iter(|| {
                    let result = router.select_provider(black_box(request));
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark rate limiting performance
fn bench_rate_limiting(c: &mut Criterion) {
    let configs = vec![
        (
            "permissive",
            AiLinterConfig {
                rate_limit_per_minute: 100,
                retry_delay_ms: 10,
                ..AiLinterConfig::default()
            }
        ),
        (
            "moderate",
            AiLinterConfig {
                rate_limit_per_minute: 30,
                retry_delay_ms: 100,
                ..AiLinterConfig::default()
            }
        ),
        (
            "strict",
            AiLinterConfig {
                rate_limit_per_minute: 10,
                retry_delay_ms: 500,
                ..AiLinterConfig::default()
            }
        ),
    ];
    
    let mut group = c.benchmark_group("rate_limiting");
    
    for (config_name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("apply_rate_limiting", config_name),
            &config,
            |b, config| {
                b.iter(|| {
                    let result = apply_rate_limiting(black_box(config));
                    black_box(result)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark batch processing efficiency
fn bench_batch_processing(c: &mut Criterion) {
    let adapter = OxcAdapter::new();
    
    let batch_sizes = vec![1, 5, 10, 20, 50];
    let code_samples: Vec<String> = (0..50)
        .map(|i| format!(
            "function component{}() {{ return <div>Component {}</div>; }}",
            i, i
        ))
        .collect();
    
    let mut group = c.benchmark_group("batch_processing");
    
    for batch_size in batch_sizes {
        group.bench_with_input(
            BenchmarkId::new("analyze_batch", batch_size),
            &batch_size,
            |b, &batch_size| {
                b.iter(|| {
                    let batch: Vec<_> = code_samples.iter()
                        .take(batch_size)
                        .enumerate()
                        .collect();
                    
                    let results: Vec<_> = batch.iter()
                        .map(|(i, code)| {
                            adapter.analyze_code(
                                black_box(code),
                                black_box(&format!("batch-{}.tsx", i))
                            )
                        })
                        .collect();
                    
                    black_box(results)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("create_destroy_analyzer", |b| {
        b.iter(|| {
            let analyzer = black_box(AiBehavioralAnalyzer::new());
            let patterns = black_box(analyzer.get_patterns());
            black_box(patterns.len())
        });
    });
    
    group.bench_function("create_destroy_oxc_adapter", |b| {
        b.iter(|| {
            let adapter = black_box(OxcAdapter::new());
            let rules = black_box(adapter.get_available_rules());
            black_box(rules.len())
        });
    });
    
    group.bench_function("parse_large_code", |b| {
        let large_code = BenchmarkCodeSamples::large_component();
        b.iter(|| {
            let allocator = oxc_allocator::Allocator::default();
            let source_type = oxc_span::SourceType::tsx();
            let parser_result = oxc_parser::Parser::new(&allocator, &large_code, source_type).parse();
            black_box(parser_result.program.body.len())
        });
    });
    
    group.finish();
}

/// Benchmark concurrent analysis
fn bench_concurrent_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let adapter = Arc::new(OxcAdapter::new());
    
    let thread_counts = vec![1, 2, 4, 8];
    let code_samples: Vec<String> = (0..32)
        .map(|i| format!(
            "function asyncComponent{}() {{ const [state, setState] = useState({}); return <div>{{}}</div>; }}",
            i, i
        ))
        .collect();
    
    let mut group = c.benchmark_group("concurrent_analysis");
    group.sample_size(10);
    
    for thread_count in thread_counts {
        group.bench_with_input(
            BenchmarkId::new("parallel_analysis", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    rt.block_on(async {
                        let tasks: Vec<_> = (0..thread_count)
                            .map(|i| {
                                let adapter = adapter.clone();
                                let code = code_samples[i % code_samples.len()].clone();
                                tokio::spawn(async move {
                                    adapter.analyze_code(
                                        &code,
                                        &format!("concurrent-{}.tsx", i)
                                    )
                                })
                            })
                            .collect();
                        
                        let results = futures::future::join_all(tasks).await;
                        black_box(results.len())
                    })
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark JSON serialization/deserialization performance
fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    // Create sample data structures
    let ai_response = moon_shine::provider_router::AIResponse {
        provider_used: "test-provider".to_string(),
        content: BenchmarkCodeSamples::medium_component(),
        session_id: "benchmark-session".to_string(),
        success: true,
        execution_time_ms: 1500,
        error_message: None,
        routing_reason: "Selected for benchmark testing with high performance".to_string(),
    };
    
    let ai_config = AiLinterConfig::default();
    
    group.bench_function("serialize_ai_response", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&ai_response)).unwrap();
            black_box(json)
        });
    });
    
    group.bench_function("deserialize_ai_response", |b| {
        let json = serde_json::to_string(&ai_response).unwrap();
        b.iter(|| {
            let response: moon_shine::provider_router::AIResponse = 
                serde_json::from_str(black_box(&json)).unwrap();
            black_box(response)
        });
    });
    
    group.bench_function("serialize_ai_config", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&ai_config)).unwrap();
            black_box(json)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_static_analysis,
    bench_behavioral_analysis,
    bench_cognitive_complexity,
    bench_provider_selection,
    bench_rate_limiting,
    bench_batch_processing,
    bench_memory_usage,
    bench_concurrent_analysis,
    bench_serialization
);
criterion_main!(benches);
