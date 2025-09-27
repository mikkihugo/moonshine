//! # AI Snapshot Tests and Comprehensive Fixtures
//!
//! Snapshot testing for AI output validation and comprehensive test fixtures:
//! - AI response format validation with snapshots
//! - Regression testing for AI analysis outputs
//! - Test fixture builders and data generators
//! - Golden file testing for behavioral patterns
//! - Configuration validation snapshots
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::oxc_adapter::{AiBehavioralAnalyzer, BehavioralPattern, BehavioralPatternType};
use moon_shine::provider_router::{AIRouter, AIRequest, AIContext, AIResponse, ProviderCapabilities};
use moon_shine::rule_types::RuleSeverity;
use moon_shine::moon_pdk_interface::AiLinterConfig;
use moon_shine::types::{DiagnosticSeverity, LintDiagnostic};
use rstest::*;
use insta::{assert_json_snapshot, assert_yaml_snapshot, assert_snapshot};
use serde_json;
use std::collections::HashMap;

/// Comprehensive test fixture builders
pub struct TestFixtures;

impl TestFixtures {
    /// Build a complete AI response for testing
    pub fn ai_response() -> AIResponseBuilder {
        AIResponseBuilder::new()
    }
    
    /// Build behavioral patterns for testing
    pub fn behavioral_pattern() -> BehavioralPatternBuilder {
        BehavioralPatternBuilder::new()
    }
    
    /// Build AI linter configurations for testing
    pub fn ai_config() -> AiConfigBuilder {
        AiConfigBuilder::new()
    }
    
    /// Build lint diagnostics for testing
    pub fn lint_diagnostic() -> LintDiagnosticBuilder {
        LintDiagnosticBuilder::new()
    }
    
    /// Build AI requests for testing
    pub fn ai_request() -> AIRequestBuilder {
        AIRequestBuilder::new()
    }
}

/// Builder for AI responses
pub struct AIResponseBuilder {
    provider_used: String,
    content: String,
    session_id: String,
    success: bool,
    execution_time_ms: u64,
    error_message: Option<String>,
    routing_reason: String,
}

impl AIResponseBuilder {
    pub fn new() -> Self {
        Self {
            provider_used: "test-provider".to_string(),
            content: "Analysis complete".to_string(),
            session_id: "test-session".to_string(),
            success: true,
            execution_time_ms: 1000,
            error_message: None,
            routing_reason: "Selected for testing".to_string(),
        }
    }
    
    pub fn provider(mut self, provider: &str) -> Self {
        self.provider_used = provider.to_string();
        self
    }
    
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
    
    pub fn session_id(mut self, session_id: &str) -> Self {
        self.session_id = session_id.to_string();
        self
    }
    
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }
    
    pub fn execution_time(mut self, time_ms: u64) -> Self {
        self.execution_time_ms = time_ms;
        self
    }
    
    pub fn error(mut self, error: &str) -> Self {
        self.error_message = Some(error.to_string());
        self.success = false;
        self
    }
    
    pub fn routing_reason(mut self, reason: &str) -> Self {
        self.routing_reason = reason.to_string();
        self
    }
    
    pub fn build(self) -> AIResponse {
        AIResponse {
            provider_used: self.provider_used,
            content: self.content,
            session_id: self.session_id,
            success: self.success,
            execution_time_ms: self.execution_time_ms,
            error_message: self.error_message,
            routing_reason: self.routing_reason,
        }
    }
}

/// Builder for behavioral patterns
pub struct BehavioralPatternBuilder {
    id: String,
    name: String,
    description: String,
    category: String,
    severity: RuleSeverity,
    pattern_type: BehavioralPatternType,
    ai_prompt: String,
    confidence_threshold: f32,
}

impl BehavioralPatternBuilder {
    pub fn new() -> Self {
        Self {
            id: "test-pattern".to_string(),
            name: "Test Pattern".to_string(),
            description: "A test behavioral pattern".to_string(),
            category: "test".to_string(),
            severity: RuleSeverity::Warning,
            pattern_type: BehavioralPatternType::CognitiveComplexity,
            ai_prompt: "Test AI prompt".to_string(),
            confidence_threshold: 0.8,
        }
    }
    
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }
    
    pub fn severity(mut self, severity: RuleSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    pub fn pattern_type(mut self, pattern_type: BehavioralPatternType) -> Self {
        self.pattern_type = pattern_type;
        self
    }
    
    pub fn ai_prompt(mut self, prompt: &str) -> Self {
        self.ai_prompt = prompt.to_string();
        self
    }
    
    pub fn confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }
    
    pub fn build(self) -> BehavioralPattern {
        BehavioralPattern {
            id: self.id,
            name: self.name,
            description: self.description,
            category: self.category,
            severity: self.severity,
            pattern_type: self.pattern_type,
            ai_prompt: self.ai_prompt,
            confidence_threshold: self.confidence_threshold,
        }
    }
}

/// Builder for AI configurations
pub struct AiConfigBuilder {
    config: AiLinterConfig,
}

impl AiConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: AiLinterConfig::default(),
        }
    }
    
    pub fn claude_enabled(mut self, enabled: bool) -> Self {
        self.config.enable_claude_ai = enabled;
        self
    }
    
    pub fn model(mut self, model: &str) -> Self {
        self.config.claude_model = model.to_string();
        self
    }
    
    pub fn rate_limit(mut self, limit: u32) -> Self {
        self.config.rate_limit_per_minute = limit;
        self
    }
    
    pub fn batch_size(mut self, size: u32) -> Self {
        self.config.batch_size = size;
        self
    }
    
    pub fn quality_threshold(mut self, threshold: f32) -> Self {
        self.config.quality_threshold = threshold;
        self
    }
    
    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.config.max_tokens_per_request = tokens;
        self
    }
    
    pub fn build(self) -> AiLinterConfig {
        self.config
    }
}

/// Builder for lint diagnostics
pub struct LintDiagnosticBuilder {
    rule_name: String,
    severity: DiagnosticSeverity,
    message: String,
    file_path: String,
    line: u32,
    column: u32,
    end_line: u32,
    end_column: u32,
    fix_available: bool,
    suggested_fix: Option<String>,
}

impl LintDiagnosticBuilder {
    pub fn new() -> Self {
        Self {
            rule_name: "test-rule".to_string(),
            severity: DiagnosticSeverity::Warning,
            message: "Test diagnostic message".to_string(),
            file_path: "test.ts".to_string(),
            line: 1,
            column: 1,
            end_line: 1,
            end_column: 10,
            fix_available: false,
            suggested_fix: None,
        }
    }
    
    pub fn rule_name(mut self, name: &str) -> Self {
        self.rule_name = name.to_string();
        self
    }
    
    pub fn severity(mut self, severity: DiagnosticSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }
    
    pub fn file_path(mut self, path: &str) -> Self {
        self.file_path = path.to_string();
        self
    }
    
    pub fn location(mut self, line: u32, column: u32) -> Self {
        self.line = line;
        self.column = column;
        self
    }
    
    pub fn range(mut self, start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Self {
        self.line = start_line;
        self.column = start_col;
        self.end_line = end_line;
        self.end_column = end_col;
        self
    }
    
    pub fn with_fix(mut self, fix: &str) -> Self {
        self.fix_available = true;
        self.suggested_fix = Some(fix.to_string());
        self
    }
    
    pub fn build(self) -> LintDiagnostic {
        LintDiagnostic {
            rule_name: self.rule_name,
            severity: self.severity,
            message: self.message,
            file_path: self.file_path,
            line: self.line,
            column: self.column,
            end_line: self.end_line,
            end_column: self.end_column,
            fix_available: self.fix_available,
            suggested_fix: self.suggested_fix,
        }
    }
}

/// Builder for AI requests
pub struct AIRequestBuilder {
    prompt: String,
    session_id: String,
    file_path: Option<String>,
    context: AIContext,
    preferred_providers: Vec<String>,
}

impl AIRequestBuilder {
    pub fn new() -> Self {
        Self {
            prompt: "Test prompt".to_string(),
            session_id: "test-session".to_string(),
            file_path: Some("test.ts".to_string()),
            context: AIContext::General,
            preferred_providers: vec![],
        }
    }
    
    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();
        self
    }
    
    pub fn session_id(mut self, session_id: &str) -> Self {
        self.session_id = session_id.to_string();
        self
    }
    
    pub fn file_path(mut self, path: Option<&str>) -> Self {
        self.file_path = path.map(|s| s.to_string());
        self
    }
    
    pub fn code_fix_context(mut self, language: &str, content: &str) -> Self {
        self.context = AIContext::CodeFix {
            language: language.to_string(),
            content: content.to_string(),
        };
        self
    }
    
    pub fn ai_linting_context(mut self, language: &str, content: &str, static_issues: Vec<String>, analysis_focus: Vec<String>) -> Self {
        self.context = AIContext::AiLinting {
            language: language.to_string(),
            content: content.to_string(),
            static_issues,
            analysis_focus,
        };
        self
    }
    
    pub fn preferred_providers(mut self, providers: Vec<&str>) -> Self {
        self.preferred_providers = providers.into_iter().map(|s| s.to_string()).collect();
        self
    }
    
    pub fn build(self) -> AIRequest {
        AIRequest {
            prompt: self.prompt,
            session_id: self.session_id,
            file_path: self.file_path,
            context: self.context,
            preferred_providers: self.preferred_providers,
        }
    }
}

/// Sample code collections for testing
pub struct CodeSamples;

impl CodeSamples {
    pub fn react_performance_issue() -> &'static str {
        r#"
            import React, { useState, useEffect } from 'react';
            
            function PerformanceIssue({ data }) {
                const [count, setCount] = useState(0);
                
                // Missing dependency array - infinite re-renders
                useEffect(() => {
                    setCount(count + 1);
                });
                
                return (
                    <div onClick={() => setCount(count + 1)}>
                        {/* Inline object creation causes re-renders */}
                        <ExpensiveChild style={{margin: 10}} />
                        Count: {count}
                    </div>
                );
            }
        "#
    }
    
    pub fn security_vulnerability() -> &'static str {
        r#"
            function SecurityIssue() {
                const userInput = document.getElementById('input').value;
                
                // XSS vulnerability
                document.innerHTML = userInput;
                
                // SQL injection pattern
                const query = "SELECT * FROM users WHERE id = " + userId;
                
                // Hardcoded secret
                const apiKey = "sk-1234567890abcdef";
                
                return { query, apiKey };
            }
        "#
    }
    
    pub fn memory_leak_pattern() -> &'static str {
        r#"
            function MemoryLeak() {
                useEffect(() => {
                    // Event listener without cleanup
                    document.addEventListener('scroll', handleScroll);
                    
                    // Interval without cleanup
                    const interval = setInterval(updateData, 1000);
                    
                    // Missing cleanup function
                }, []);
                
                return <div>Component with leaks</div>;
            }
        "#
    }
    
    pub fn cognitive_complexity() -> &'static str {
        r#"
            function complexFunction(order, user, discounts) {
                if (order && order.items) {
                    for (let i = 0; i < order.items.length; i++) {
                        const item = order.items[i];
                        if (user.level === 'premium') {
                            if (discounts && discounts.premium) {
                                for (let j = 0; j < discounts.premium.length; j++) {
                                    if (discounts.premium[j].category === item.category) {
                                        if (discounts.premium[j].type === 'percentage') {
                                            item.price *= (1 - discounts.premium[j].value / 100);
                                        } else if (discounts.premium[j].type === 'fixed') {
                                            item.price -= discounts.premium[j].value;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return order;
            }
        "#
    }
    
    pub fn clean_optimized_code() -> &'static str {
        r#"
            import React, { useState, useEffect, useCallback, useMemo } from 'react';
            
            function OptimizedComponent({ data }) {
                const [count, setCount] = useState(0);
                
                // Proper dependency array
                useEffect(() => {
                    setCount(prev => prev + 1);
                }, []);
                
                // Memoized computation
                const expensiveValue = useMemo(() => {
                    return data.reduce((sum, item) => sum + item.value, 0);
                }, [data]);
                
                // Memoized callback
                const handleClick = useCallback(() => {
                    setCount(prev => prev + 1);
                }, []);
                
                return (
                    <div onClick={handleClick}>
                        <ExpensiveChild value={expensiveValue} />
                        Count: {count}
                    </div>
                );
            }
        "#
    }
}

// Snapshot tests for AI responses
#[rstest]
fn test_ai_response_snapshot_success() {
    let response = TestFixtures::ai_response()
        .provider("claude")
        .content("Found 3 performance issues:\n1. Missing useEffect dependency\n2. Inline object creation\n3. Unnecessary re-renders")
        .session_id("perf-analysis-123")
        .execution_time(1250)
        .routing_reason("Selected Claude for code analysis (score: 0.95)")
        .build();
    
    assert_json_snapshot!("ai_response_success", response);
}

#[rstest]
fn test_ai_response_snapshot_error() {
    let response = TestFixtures::ai_response()
        .provider("claude")
        .content("")
        .session_id("error-session-456")
        .execution_time(500)
        .error("Rate limit exceeded: 429 Too Many Requests")
        .routing_reason("Provider selection failed after 3 attempts")
        .build();
    
    assert_json_snapshot!("ai_response_error", response);
}

#[rstest]
fn test_behavioral_pattern_snapshots() {
    let patterns = vec![
        TestFixtures::behavioral_pattern()
            .id("react-excessive-rerenders")
            .name("Excessive Component Re-renders")
            .description("Component may be re-rendering unnecessarily")
            .category("performance")
            .severity(RuleSeverity::Warning)
            .pattern_type(BehavioralPatternType::PerformanceAntiPattern)
            .ai_prompt("Analyze for React re-render patterns")
            .confidence_threshold(0.85)
            .build(),
        
        TestFixtures::behavioral_pattern()
            .id("xss-vulnerability")
            .name("XSS Vulnerability Pattern")
            .description("Potential cross-site scripting vulnerability")
            .category("security")
            .severity(RuleSeverity::Error)
            .pattern_type(BehavioralPatternType::SecurityVulnerability)
            .ai_prompt("Check for XSS vulnerabilities")
            .confidence_threshold(0.95)
            .build(),
        
        TestFixtures::behavioral_pattern()
            .id("cognitive-complexity-high")
            .name("High Cognitive Complexity")
            .description("Function has high cognitive complexity")
            .category("maintainability")
            .severity(RuleSeverity::Warning)
            .pattern_type(BehavioralPatternType::CognitiveComplexity)
            .ai_prompt("Analyze cognitive complexity")
            .confidence_threshold(0.80)
            .build(),
    ];
    
    assert_yaml_snapshot!("behavioral_patterns_collection", patterns);
}

#[rstest]
fn test_ai_config_snapshots() {
    let configs = vec![
        ("development", TestFixtures::ai_config()
            .claude_enabled(true)
            .model("sonnet")
            .rate_limit(60)
            .batch_size(10)
            .quality_threshold(0.7)
            .max_tokens(8192)
            .build()),
        
        ("production", TestFixtures::ai_config()
            .claude_enabled(true)
            .model("opus")
            .rate_limit(30)
            .batch_size(5)
            .quality_threshold(0.9)
            .max_tokens(16384)
            .build()),
        
        ("ci", TestFixtures::ai_config()
            .claude_enabled(false)
            .model("sonnet")
            .rate_limit(10)
            .batch_size(1)
            .quality_threshold(0.95)
            .max_tokens(4096)
            .build()),
    ];
    
    for (env, config) in configs {
        assert_json_snapshot!(format!("ai_config_{}", env), config);
    }
}

#[rstest]
fn test_lint_diagnostic_snapshots() {
    let diagnostics = vec![
        TestFixtures::lint_diagnostic()
            .rule_name("react/exhaustive-deps")
            .severity(DiagnosticSeverity::Warning)
            .message("React Hook useEffect has a missing dependency: 'count'")
            .file_path("src/components/Counter.tsx")
            .location(12, 8)
            .range(12, 8, 12, 25)
            .with_fix("Add 'count' to the dependency array")
            .build(),
        
        TestFixtures::lint_diagnostic()
            .rule_name("security/detect-unsafe-regex")
            .severity(DiagnosticSeverity::Error)
            .message("Unsafe regular expression detected")
            .file_path("src/utils/validation.ts")
            .location(45, 20)
            .range(45, 20, 45, 35)
            .with_fix("Use a safer regex pattern")
            .build(),
        
        TestFixtures::lint_diagnostic()
            .rule_name("complexity/cognitive-complexity")
            .severity(DiagnosticSeverity::Warning)
            .message("Function has cognitive complexity of 23 (threshold: 15)")
            .file_path("src/business/processor.ts")
            .location(78, 1)
            .range(78, 1, 120, 1)
            .with_fix("Consider breaking this function into smaller functions")
            .build(),
    ];
    
    assert_json_snapshot!("lint_diagnostics_collection", diagnostics);
}

#[rstest]
fn test_ai_request_context_snapshots() {
    let requests = vec![
        ("code_fix", TestFixtures::ai_request()
            .prompt("Fix the TypeScript errors in this component")
            .session_id("fix-session-789")
            .file_path(Some("src/components/Button.tsx"))
            .code_fix_context("typescript", CodeSamples::react_performance_issue())
            .preferred_providers(vec!["claude", "google"])
            .build()),
        
        ("ai_linting", TestFixtures::ai_request()
            .prompt("Analyze this code for behavioral patterns and performance issues")
            .session_id("linting-session-101")
            .file_path(Some("src/security/auth.ts"))
            .ai_linting_context(
                "typescript",
                CodeSamples::security_vulnerability(),
                vec!["Missing input sanitization".to_string()],
                vec!["security".to_string(), "performance".to_string()]
            )
            .build()),
    ];
    
    for (context_type, request) in requests {
        assert_json_snapshot!(format!("ai_request_{}", context_type), request);
    }
}

#[rstest]
fn test_provider_capabilities_snapshot() {
    let capabilities = vec![
        ("claude", ProviderCapabilities {
            code_analysis: 0.95,
            code_generation: 0.85,
            complex_reasoning: 0.95,
            speed: 0.75,
            context_length: 200000,
            supports_sessions: true,
        }),
        
        ("google", ProviderCapabilities {
            code_analysis: 0.85,
            code_generation: 0.80,
            complex_reasoning: 0.85,
            speed: 0.90,
            context_length: 100000,
            supports_sessions: true,
        }),
        
        ("openai", ProviderCapabilities {
            code_analysis: 0.88,
            code_generation: 0.95,
            complex_reasoning: 0.85,
            speed: 0.85,
            context_length: 200000,
            supports_sessions: true,
        }),
    ];
    
    assert_yaml_snapshot!("provider_capabilities", capabilities);
}

#[rstest]
fn test_code_analysis_results_snapshot() {
    let analyzer = AiBehavioralAnalyzer::new();
    
    let test_results = vec![
        ("performance_issue", {
            let code = CodeSamples::react_performance_issue();
            let complexity = analyzer.calculate_cognitive_complexity_heuristic(code);
            serde_json::json!({
                "code_type": "react_performance",
                "cognitive_complexity": complexity,
                "has_useeffect": code.contains("useEffect"),
                "has_inline_objects": code.contains("style={{"),
                "line_count": code.lines().count(),
                "char_count": code.len()
            })
        }),
        
        ("security_vulnerability", {
            let code = CodeSamples::security_vulnerability();
            let complexity = analyzer.calculate_cognitive_complexity_heuristic(code);
            serde_json::json!({
                "code_type": "security_vulnerability",
                "cognitive_complexity": complexity,
                "has_innerhtml": code.contains("innerHTML"),
                "has_sql_concat": code.contains("SELECT * FROM"),
                "has_hardcoded_secret": code.contains("sk-"),
                "line_count": code.lines().count(),
                "char_count": code.len()
            })
        }),
        
        ("clean_code", {
            let code = CodeSamples::clean_optimized_code();
            let complexity = analyzer.calculate_cognitive_complexity_heuristic(code);
            serde_json::json!({
                "code_type": "clean_optimized",
                "cognitive_complexity": complexity,
                "has_usememo": code.contains("useMemo"),
                "has_usecallback": code.contains("useCallback"),
                "has_proper_deps": code.contains("[], []"),
                "line_count": code.lines().count(),
                "char_count": code.len()
            })
        }),
    ];
    
    assert_json_snapshot!("code_analysis_results", test_results);
}

#[rstest]
fn test_error_scenarios_snapshot() {
    let error_scenarios = vec![
        serde_json::json!({
            "scenario": "rate_limit_exceeded",
            "error_type": "RateLimitError",
            "message": "Rate limit exceeded: 20 requests per minute",
            "retry_after": 45,
            "provider": "claude"
        }),
        
        serde_json::json!({
            "scenario": "provider_unavailable",
            "error_type": "ProviderError",
            "message": "AI provider temporarily unavailable",
            "fallback_available": true,
            "suggested_action": "retry_with_fallback"
        }),
        
        serde_json::json!({
            "scenario": "context_too_large",
            "error_type": "ContextSizeError",
            "message": "Request context exceeds maximum size",
            "max_size": 200000,
            "actual_size": 250000,
            "suggestion": "reduce_context_or_split_request"
        }),
        
        serde_json::json!({
            "scenario": "invalid_configuration",
            "error_type": "ConfigError",
            "message": "Invalid AI linter configuration",
            "invalid_fields": ["rate_limit_per_minute", "quality_threshold"],
            "corrections": {
                "rate_limit_per_minute": "must be > 0",
                "quality_threshold": "must be between 0.0 and 1.0"
            }
        }),
    ];
    
    assert_json_snapshot!("error_scenarios", error_scenarios);
}

#[rstest]
fn test_performance_metrics_snapshot() {
    // Simulate performance metrics for different scenarios
    let metrics = vec![
        serde_json::json!({
            "scenario": "small_file_analysis",
            "file_size_bytes": 1024,
            "static_analysis_ms": 15,
            "behavioral_analysis_ms": 45,
            "ai_analysis_ms": 1200,
            "total_time_ms": 1260,
            "diagnostics_found": 2,
            "memory_usage_mb": 12.5
        }),
        
        serde_json::json!({
            "scenario": "large_file_analysis",
            "file_size_bytes": 51200,
            "static_analysis_ms": 180,
            "behavioral_analysis_ms": 320,
            "ai_analysis_ms": 2800,
            "total_time_ms": 3300,
            "diagnostics_found": 15,
            "memory_usage_mb": 45.2
        }),
        
        serde_json::json!({
            "scenario": "batch_processing",
            "files_processed": 25,
            "total_size_bytes": 128000,
            "batch_time_ms": 5400,
            "avg_time_per_file_ms": 216,
            "total_diagnostics": 42,
            "rate_limit_hits": 2,
            "memory_peak_mb": 78.9
        }),
    ];
    
    assert_json_snapshot!("performance_metrics", metrics);
}

#[rstest]
fn test_regression_analysis_snapshot() {
    // Test that analysis results remain consistent over time
    let analyzer = AiBehavioralAnalyzer::new();
    
    let regression_tests = vec![
        {
            let code = "function simple() { return 1; }";
            serde_json::json!({
                "test_name": "simple_function",
                "code": code,
                "cognitive_complexity": analyzer.calculate_cognitive_complexity_heuristic(code),
                "pattern_count": analyzer.get_patterns().len(),
                "expected_complexity_range": [0, 5]
            })
        },
        
        {
            let code = "function complex() { if (a) { for (let i = 0; i < 10; i++) { if (b && c) { switch (d) { case 1: break; } } } } }";
            serde_json::json!({
                "test_name": "complex_function",
                "code": code,
                "cognitive_complexity": analyzer.calculate_cognitive_complexity_heuristic(code),
                "pattern_count": analyzer.get_patterns().len(),
                "expected_complexity_range": [8, 15]
            })
        },
    ];
    
    assert_json_snapshot!("regression_analysis", regression_tests);
}

#[rstest]
fn test_comprehensive_workflow_snapshot() {
    // Simulate a complete AI linting workflow
    let workflow = serde_json::json!({
        "workflow_id": "comprehensive-analysis-20241201",
        "input": {
            "file_path": "src/components/UserDashboard.tsx",
            "file_size_bytes": 3847,
            "language": "typescript",
            "framework": "react"
        },
        "static_analysis": {
            "duration_ms": 127,
            "rules_executed": 45,
            "diagnostics_found": 8,
            "severity_breakdown": {
                "error": 1,
                "warning": 5,
                "info": 2,
                "hint": 0
            }
        },
        "behavioral_analysis": {
            "duration_ms": 89,
            "patterns_checked": 7,
            "heuristic_matches": 3,
            "ai_analysis_triggered": true
        },
        "ai_analysis": {
            "provider_used": "claude",
            "model": "sonnet",
            "duration_ms": 1450,
            "tokens_used": 2847,
            "confidence_scores": [0.92, 0.87, 0.95],
            "patterns_detected": [
                "react-excessive-rerenders",
                "memory-leak-potential",
                "cognitive-complexity-high"
            ]
        },
        "results": {
            "total_issues": 14,
            "fixable_issues": 11,
            "estimated_fix_time_minutes": 25,
            "quality_score": 0.73,
            "recommendations": [
                "Add missing useEffect dependencies",
                "Implement cleanup for event listeners",
                "Extract complex logic into separate functions"
            ]
        },
        "performance": {
            "total_duration_ms": 1666,
            "peak_memory_mb": 24.7,
            "cache_hits": 12,
            "rate_limit_remaining": 47
        }
    });
    
    assert_json_snapshot!("comprehensive_workflow", workflow);
}
