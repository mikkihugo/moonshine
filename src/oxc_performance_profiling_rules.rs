//! Performance Profiling Rules
//!
//! Advanced performance profiling, memory analysis, and benchmarking rules.
//! Focuses on performance monitoring, memory optimization, and automated performance testing.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}

/// Require memory leak detection and monitoring
pub struct RequireMemoryLeakDetection;

impl RequireMemoryLeakDetection {
    pub const NAME: &'static str = "require-memory-leak-detection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMemoryLeakDetection {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for event listener without cleanup
        if code.contains("addEventListener") && !code.contains("removeEventListener") {
            diagnostics.push(create_event_listener_leak_diagnostic());
        }

        // Check for timer without cleanup
        if code.contains("setInterval") && !code.contains("clearInterval") {
            diagnostics.push(create_timer_leak_diagnostic());
        }

        // Check for missing component cleanup
        if code.contains("useEffect") && code.contains("setInterval") && !code.contains("return ()") {
            diagnostics.push(create_react_cleanup_diagnostic());
        }

        // Check for circular references
        if code.contains("parent") && code.contains("child") && code.contains("parent.child") && code.contains("child.parent") {
            diagnostics.push(create_circular_reference_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMemoryLeakDetection {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "memory_management".to_string(),
            confidence: 0.93,
            description: "Implement proper cleanup: remove event listeners, clear timers, avoid circular references, use WeakMap/WeakSet for object references, implement disposal patterns.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_event_listener_leak_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryLeakDetection::NAME.to_string(),
        message: "Event listener without cleanup detected. Remove listeners to prevent memory leaks".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_timer_leak_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryLeakDetection::NAME.to_string(),
        message: "Timer without cleanup detected. Clear intervals/timeouts to prevent memory leaks".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_react_cleanup_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryLeakDetection::NAME.to_string(),
        message: "React useEffect with timer should return cleanup function".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_circular_reference_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryLeakDetection::NAME.to_string(),
        message: "Potential circular reference detected. Use WeakMap or break references manually".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require performance benchmarking and measurement
pub struct RequirePerformanceBenchmarks;

impl RequirePerformanceBenchmarks {
    pub const NAME: &'static str = "require-performance-benchmarks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceBenchmarks {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for performance-critical code without measurement
        if (code.contains("algorithm") || code.contains("sort") || code.contains("search")) && !code.contains("performance.") {
            diagnostics.push(create_measurement_diagnostic());
        }

        // Check for missing benchmark tests
        if code.contains("function") && code.contains("critical") && !code.contains("benchmark") {
            diagnostics.push(create_benchmark_test_diagnostic());
        }

        // Check for missing performance budget
        if code.contains("bundle") && !code.contains("performance-budget") {
            diagnostics.push(create_performance_budget_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceBenchmarks {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "performance_benchmarking".to_string(),
            confidence: 0.88,
            description: "Implement performance benchmarking: use performance.mark/measure, create benchmark tests, set performance budgets, monitor metrics over time.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_measurement_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBenchmarks::NAME.to_string(),
        message: "Performance-critical code should include measurement with performance.mark/measure".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_benchmark_test_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBenchmarks::NAME.to_string(),
        message: "Critical functions should have corresponding benchmark tests".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_performance_budget_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBenchmarks::NAME.to_string(),
        message: "Bundle configuration should include performance budgets".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require CPU profiling for optimization
pub struct RequireCpuProfiling;

impl RequireCpuProfiling {
    pub const NAME: &'static str = "require-cpu-profiling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCpuProfiling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for expensive operations without profiling
        if code.contains("for") && code.contains("for") && !code.contains("console.time") {
            diagnostics.push(create_nested_loop_profiling_diagnostic());
        }

        // Check for recursive functions without monitoring
        if code.contains("function") && code.contains("recursive") && !code.contains("performance") {
            diagnostics.push(create_recursive_profiling_diagnostic());
        }

        // Check for heavy computation without worker
        if code.contains("compute") && code.contains("heavy") && !code.contains("Worker") {
            diagnostics.push(create_worker_suggestion_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCpuProfiling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "cpu_profiling".to_string(),
            confidence: 0.85,
            description: "Implement CPU profiling: profile nested loops, monitor recursive functions, use Web Workers for heavy computation, implement time-slicing for long operations.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_nested_loop_profiling_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCpuProfiling::NAME.to_string(),
        message: "Nested loops should be profiled for performance optimization".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_recursive_profiling_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCpuProfiling::NAME.to_string(),
        message: "Recursive functions should include performance monitoring".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_worker_suggestion_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCpuProfiling::NAME.to_string(),
        message: "Heavy computation should use Web Workers to avoid blocking main thread".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require memory usage optimization
pub struct RequireMemoryOptimization;

impl RequireMemoryOptimization {
    pub const NAME: &'static str = "require-memory-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireMemoryOptimization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for large object creation in loops
        if code.contains("for") && code.contains("new") && (code.contains("Array") || code.contains("Object")) {
            diagnostics.push(create_object_pooling_diagnostic());
        }

        // Check for string concatenation in loops
        if code.contains("for") && code.contains("+=") && code.contains("string") {
            diagnostics.push(create_string_optimization_diagnostic());
        }

        // Check for DOM manipulation in loops
        if code.contains("for") && (code.contains("createElement") || code.contains("appendChild")) {
            diagnostics.push(create_dom_optimization_diagnostic());
        }

        // Check for missing memoization
        if code.contains("expensive") && code.contains("function") && !code.contains("memo") {
            diagnostics.push(create_memoization_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMemoryOptimization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "memory_optimization".to_string(),
            confidence: 0.90,
            description: "Optimize memory usage: implement object pooling, use StringBuilder for concatenation, batch DOM operations, implement memoization for expensive calculations.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_object_pooling_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryOptimization::NAME.to_string(),
        message: "Object creation in loops can cause memory pressure. Consider object pooling".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_string_optimization_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryOptimization::NAME.to_string(),
        message: "String concatenation in loops is inefficient. Use array join or template literals".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_dom_optimization_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryOptimization::NAME.to_string(),
        message: "DOM manipulation in loops causes reflow. Use DocumentFragment or batch operations".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_memoization_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryOptimization::NAME.to_string(),
        message: "Expensive functions should implement memoization for repeated calls".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require performance monitoring integration
pub struct RequirePerformanceMonitoring;

impl RequirePerformanceMonitoring {
    pub const NAME: &'static str = "require-performance-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing Core Web Vitals monitoring
        if code.contains("web") && !code.contains("CLS") && !code.contains("LCP") && !code.contains("FID") {
            diagnostics.push(create_web_vitals_diagnostic());
        }

        // Check for missing error performance tracking
        if code.contains("catch") && !code.contains("performance") {
            diagnostics.push(create_error_performance_diagnostic());
        }

        // Check for missing user timing API
        if code.contains("critical-path") && !code.contains("performance.mark") {
            diagnostics.push(create_user_timing_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceMonitoring {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "performance_monitoring".to_string(),
            confidence: 0.87,
            description: "Implement comprehensive performance monitoring: track Core Web Vitals, monitor error performance impact, use User Timing API, integrate with RUM tools.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_web_vitals_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Web application should monitor Core Web Vitals (CLS, LCP, FID)".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_error_performance_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Error handling should include performance impact measurement".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_user_timing_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Critical path operations should use User Timing API for measurement".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require load testing and stress testing
pub struct RequireLoadTesting;

impl RequireLoadTesting {
    pub const NAME: &'static str = "require-load-testing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireLoadTesting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for API endpoints without load tests
        if code.contains("app.get") || code.contains("app.post") && !code.contains("load-test") {
            diagnostics.push(create_api_load_test_diagnostic());
        }

        // Check for database operations without stress testing
        if code.contains("database") && code.contains("production") && !code.contains("stress-test") {
            diagnostics.push(create_database_stress_test_diagnostic());
        }

        // Check for missing performance regression tests
        if code.contains("performance") && !code.contains("regression-test") {
            diagnostics.push(create_regression_test_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireLoadTesting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "load_testing".to_string(),
            confidence: 0.84,
            description: "Implement comprehensive load testing: test API endpoints under load, stress test database operations, create performance regression tests, use tools like k6 or Artillery.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_api_load_test_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLoadTesting::NAME.to_string(),
        message: "API endpoints should include load testing to verify performance under stress".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_database_stress_test_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLoadTesting::NAME.to_string(),
        message: "Database operations should be stress tested for production workloads".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_regression_test_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLoadTesting::NAME.to_string(),
        message: "Performance optimizations should include regression tests to prevent degradation".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require caching strategies for performance
pub struct RequireCachingStrategies;

impl RequireCachingStrategies {
    pub const NAME: &'static str = "require-caching-strategies";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireCachingStrategies {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for expensive API calls without caching
        if code.contains("fetch") && code.contains("expensive") && !code.contains("cache") {
            diagnostics.push(create_api_caching_diagnostic());
        }

        // Check for computed values without memoization
        if code.contains("calculate") && code.contains("expensive") && !code.contains("useMemo") {
            diagnostics.push(create_computation_caching_diagnostic());
        }

        // Check for missing browser caching headers
        if code.contains("static") && code.contains("assets") && !code.contains("Cache-Control") {
            diagnostics.push(create_browser_caching_diagnostic());
        }

        // Check for repeated database queries
        if code.contains("SELECT") && code.contains("loop") && !code.contains("cache") {
            diagnostics.push(create_database_caching_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCachingStrategies {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "caching_strategies".to_string(),
            confidence: 0.91,
            description: "Implement comprehensive caching: cache expensive API calls, memoize computations, set proper browser cache headers, implement database query caching.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_api_caching_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCachingStrategies::NAME.to_string(),
        message: "Expensive API calls should implement caching to improve performance".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_computation_caching_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCachingStrategies::NAME.to_string(),
        message: "Expensive computations should use memoization (useMemo, useCallback)".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_browser_caching_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCachingStrategies::NAME.to_string(),
        message: "Static assets should include proper Cache-Control headers".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_database_caching_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCachingStrategies::NAME.to_string(),
        message: "Repeated database queries should implement result caching".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require performance budgets and alerts
pub struct RequirePerformanceBudgets;

impl RequirePerformanceBudgets {
    pub const NAME: &'static str = "require-performance-budgets";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceBudgets {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing bundle size budgets
        if code.contains("webpack") && !code.contains("performance.maxAssetSize") {
            diagnostics.push(create_bundle_budget_diagnostic());
        }

        // Check for missing runtime performance budgets
        if code.contains("performance-critical") && !code.contains("budget") {
            diagnostics.push(create_runtime_budget_diagnostic());
        }

        // Check for missing monitoring alerts
        if code.contains("monitoring") && !code.contains("alert") && !code.contains("threshold") {
            diagnostics.push(create_alert_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceBudgets {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "performance_budgets".to_string(),
            confidence: 0.86,
            description: "Implement performance budgets: set bundle size limits, define runtime performance thresholds, configure alerts for budget violations, track metrics over time.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_bundle_budget_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBudgets::NAME.to_string(),
        message: "Webpack configuration should include performance budgets for asset sizes".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_runtime_budget_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBudgets::NAME.to_string(),
        message: "Performance-critical code should define runtime performance budgets".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_alert_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBudgets::NAME.to_string(),
        message: "Performance monitoring should include alerts for threshold violations".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_memory_leak_detection() {
        let rule = RequireMemoryLeakDetection;

        // Test event listener without cleanup
        let code_violation = r#"
            element.addEventListener('click', handler);
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper cleanup
        let code_compliant = r#"
            element.addEventListener('click', handler);
            element.removeEventListener('click', handler);
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_performance_benchmarks() {
        let rule = RequirePerformanceBenchmarks;

        // Test algorithm without measurement
        let code_violation = "function sortAlgorithm(arr) { /* complex sorting */ }";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with performance measurement
        let code_compliant = r#"
            function sortAlgorithm(arr) {
                performance.mark('sort-start');
                /* complex sorting */
                performance.mark('sort-end');
                performance.measure('sort', 'sort-start', 'sort-end');
            }
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_cpu_profiling() {
        let rule = RequireCpuProfiling;

        // Test nested loop without profiling
        let code_violation = r#"
            for (let i = 0; i < n; i++) {
                for (let j = 0; j < m; j++) {
                    /* computation */
                }
            }
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with profiling
        let code_compliant = r#"
            console.time('nested-loop');
            for (let i = 0; i < n; i++) {
                for (let j = 0; j < m; j++) {
                    /* computation */
                }
            }
            console.timeEnd('nested-loop');
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_memory_optimization() {
        let rule = RequireMemoryOptimization;

        // Test object creation in loop
        let code_violation = r#"
            for (let i = 0; i < 1000; i++) {
                const obj = new Object();
                /* use obj */
            }
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with object pooling concept
        let code_compliant = r#"
            const objectPool = [];
            for (let i = 0; i < 1000; i++) {
                const obj = objectPool.pop() || {};
                /* use obj */
                objectPool.push(obj);
            }
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_performance_monitoring() {
        let rule = RequirePerformanceMonitoring;

        // Test web app without vitals monitoring
        let code_violation = "const webApp = createApp();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with Core Web Vitals
        let code_compliant = r#"
            const webApp = createApp();
            monitorCLS();
            monitorLCP();
            monitorFID();
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_load_testing() {
        let rule = RequireLoadTesting;

        // Test API without load test
        let code_violation = "app.get('/api/users', handler);";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with load test reference
        let code_compliant = r#"
            app.get('/api/users', handler);
            // load-test: k6 run load-test-users.js
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_caching_strategies() {
        let rule = RequireCachingStrategies;

        // Test expensive fetch without caching
        let code_violation = "const data = await fetch('/expensive-api');";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with caching
        let code_compliant = r#"
            const cached = cache.get('/expensive-api');
            const data = cached || await fetch('/expensive-api');
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_performance_budgets() {
        let rule = RequirePerformanceBudgets;

        // Test webpack without budgets
        let code_violation = r#"
            module.exports = {
                entry: './src/index.js'
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with performance budgets
        let code_compliant = r#"
            module.exports = {
                entry: './src/index.js',
                performance: {
                    maxAssetSize: 250000
                }
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement() {
        let rule = RequireMemoryLeakDetection;
        let diagnostics = vec![WasmRuleDiagnostic {
            rule_name: "require-memory-leak-detection".to_string(),
            message: "Test message".to_string(),
            line: 1,
            column: 1,
            severity: "warning".to_string(),
        }];

        let suggestions = rule.ai_enhance("", &diagnostics);
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.8);
    }
}