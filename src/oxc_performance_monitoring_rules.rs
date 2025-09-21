//! # Performance Monitoring & Observability Rules
//!
//! Comprehensive rules for performance monitoring, observability, metrics collection,
//! distributed tracing, and system monitoring in modern JavaScript/TypeScript applications.
//!
//! ## Rule Categories:
//! - **Metrics Collection**: Performance metrics, telemetry, monitoring instrumentation
//! - **Distributed Tracing**: OpenTelemetry, trace correlation, span management
//! - **Logging Standards**: Structured logging, log levels, performance logging
//! - **Observability**: Health checks, monitoring endpoints, alerting
//! - **Performance Profiling**: Memory profiling, CPU monitoring, runtime performance
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Proper metrics collection
//! const metrics = require('@opentelemetry/metrics');
//! const meter = metrics.getMeter('app-metrics');
//! const requestCounter = meter.createCounter('http_requests_total');
//!
//! // ❌ Bad: Missing performance monitoring
//! app.get('/api/users', (req, res) => {
//!   const users = getUsersFromDatabase(); // No monitoring
//!   res.json(users);
//! });
//! ```

use serde::{Deserialize, Serialize};

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};

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

/// Rule: require-performance-metrics
/// Enforces performance metrics collection in critical application paths
#[derive(Clone)]
pub struct RequirePerformanceMetrics;

impl RequirePerformanceMetrics {
    pub const NAME: &'static str = "require-performance-metrics";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceMetrics {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for HTTP handlers without metrics
        if code.contains("app.get(") || code.contains("app.post(") {
            if !code.contains("metrics") && !code.contains("telemetry") {
                diagnostics.push(create_performance_metrics_diagnostic(
                    0, 0,
                    "HTTP handlers should include performance metrics collection"
                ));
            }
        }

        // Check for database operations without timing
        if (code.contains("query(") || code.contains("findBy") || code.contains("save(")) &&
           !code.contains("performance.mark") && !code.contains("console.time") {
            diagnostics.push(create_performance_metrics_diagnostic(
                0, 0,
                "Database operations should be instrumented with performance timing"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceMetrics {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.85,
            suggestion: "Add OpenTelemetry instrumentation: `const span = tracer.startSpan('operation'); span.end();`".to_string(),
            fix_code: Some("// Add performance.mark('start'); ... performance.measure('operation', 'start');".to_string()),
        }).collect()
    }
}

/// Rule: require-distributed-tracing
/// Enforces distributed tracing implementation for microservices
#[derive(Clone)]
pub struct RequireDistributedTracing;

impl RequireDistributedTracing {
    pub const NAME: &'static str = "require-distributed-tracing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDistributedTracing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for HTTP requests without trace context
        if code.contains("fetch(") || code.contains("axios.") {
            if !code.contains("x-trace-id") && !code.contains("traceId") {
                diagnostics.push(create_distributed_tracing_diagnostic(
                    0, 0,
                    "HTTP requests should include trace context headers"
                ));
            }
        }

        // Check for service calls without span creation
        if code.contains("service.") && code.contains("async") {
            if !code.contains("startSpan") && !code.contains("tracer") {
                diagnostics.push(create_distributed_tracing_diagnostic(
                    0, 0,
                    "Service calls should create spans for distributed tracing"
                ));
            }
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDistributedTracing {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Implement OpenTelemetry tracing: `const span = tracer.startSpan('operation', { parent: parentSpan });`".to_string(),
            fix_code: Some("headers['x-trace-id'] = trace.getActiveSpan()?.spanContext().traceId;".to_string()),
        }).collect()
    }
}

/// Rule: require-structured-logging
/// Enforces structured logging with proper log levels and metadata
#[derive(Clone)]
pub struct RequireStructuredLogging;

impl RequireStructuredLogging {
    pub const NAME: &'static str = "require-structured-logging";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireStructuredLogging {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for console.log usage instead of structured logging
        if code.contains("console.log(") && !code.contains("logger.") {
            diagnostics.push(create_structured_logging_diagnostic(
                0, 0,
                "Use structured logging instead of console.log for production code"
            ));
        }

        // Check for logging without context
        if code.contains("logger.error(") && !code.contains("metadata") && !code.contains("{") {
            diagnostics.push(create_structured_logging_diagnostic(
                0, 0,
                "Error logs should include structured metadata and context"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireStructuredLogging {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.92,
            suggestion: "Replace with structured logging: `logger.info('message', { userId, requestId, duration });`".to_string(),
            fix_code: Some("logger.info('Operation completed', { operation: 'operation_name', duration: endTime - startTime, userId });".to_string()),
        }).collect()
    }
}

/// Rule: require-health-checks
/// Enforces health check endpoints and monitoring capabilities
#[derive(Clone)]
pub struct RequireHealthChecks;

impl RequireHealthChecks {
    pub const NAME: &'static str = "require-health-checks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireHealthChecks {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for Express app without health endpoint
        if code.contains("express()") && !code.contains("/health") && !code.contains("/status") {
            diagnostics.push(create_health_checks_diagnostic(
                0, 0,
                "Express applications should include health check endpoints"
            ));
        }

        // Check for database connections without health monitoring
        if code.contains("createConnection") && !code.contains("healthcheck") {
            diagnostics.push(create_health_checks_diagnostic(
                0, 0,
                "Database connections should include health monitoring"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireHealthChecks {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Add health check endpoint: `app.get('/health', (req, res) => res.status(200).json({ status: 'healthy' }));`".to_string(),
            fix_code: Some("app.get('/health', async (req, res) => { const dbHealth = await checkDatabaseHealth(); res.status(dbHealth ? 200 : 503).json({ status: dbHealth ? 'healthy' : 'unhealthy' }); });".to_string()),
        }).collect()
    }
}

/// Rule: require-memory-profiling
/// Enforces memory profiling and leak detection capabilities
#[derive(Clone)]
pub struct RequireMemoryProfiling;

impl RequireMemoryProfiling {
    pub const NAME: &'static str = "require-memory-profiling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMemoryProfiling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for long-running processes without memory monitoring
        if code.contains("setInterval") && !code.contains("process.memoryUsage") {
            diagnostics.push(create_memory_profiling_diagnostic(
                0, 0,
                "Long-running processes should monitor memory usage"
            ));
        }

        // Check for large data processing without memory checks
        if code.contains("map(") && code.contains("forEach") && !code.contains("gc()") {
            diagnostics.push(create_memory_profiling_diagnostic(
                0, 0,
                "Large data processing should include memory profiling"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMemoryProfiling {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.83,
            suggestion: "Add memory monitoring: `const memUsage = process.memoryUsage(); logger.info('Memory usage', memUsage);`".to_string(),
            fix_code: Some("setInterval(() => { const mem = process.memoryUsage(); if (mem.heapUsed > threshold) logger.warn('High memory usage', mem); }, 30000);".to_string()),
        }).collect()
    }
}

/// Rule: require-alerting-integration
/// Enforces integration with alerting systems for critical events
#[derive(Clone)]
pub struct RequireAlertingIntegration;

impl RequireAlertingIntegration {
    pub const NAME: &'static str = "require-alerting-integration";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAlertingIntegration {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for error handling without alerting
        if code.contains("catch(") && !code.contains("alert") && !code.contains("notify") {
            diagnostics.push(create_alerting_integration_diagnostic(
                0, 0,
                "Error handling should include alerting for critical failures"
            ));
        }

        // Check for timeout errors without monitoring
        if code.contains("timeout") && !code.contains("metrics") {
            diagnostics.push(create_alerting_integration_diagnostic(
                0, 0,
                "Timeout errors should trigger monitoring alerts"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAlertingIntegration {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Add alerting integration: `await alertManager.sendAlert({ severity: 'critical', message: error.message });`".to_string(),
            fix_code: Some("if (error.severity === 'critical') { await notificationService.sendAlert(error); }".to_string()),
        }).collect()
    }
}

/// Rule: require-sla-monitoring
/// Enforces SLA (Service Level Agreement) monitoring and tracking
#[derive(Clone)]
pub struct RequireSLAMonitoring;

impl RequireSLAMonitoring {
    pub const NAME: &'static str = "require-sla-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSLAMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for API endpoints without SLA tracking
        if code.contains("router.") && !code.contains("responseTime") && !code.contains("sla") {
            diagnostics.push(create_sla_monitoring_diagnostic(
                0, 0,
                "API endpoints should track response times for SLA monitoring"
            ));
        }

        // Check for external service calls without SLA monitoring
        if code.contains("fetch(") && !code.contains("timeout") && !code.contains("sla") {
            diagnostics.push(create_sla_monitoring_diagnostic(
                0, 0,
                "External service calls should include SLA monitoring and timeouts"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSLAMonitoring {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Add SLA monitoring: `const start = Date.now(); await operation(); slaTracker.record(Date.now() - start);`".to_string(),
            fix_code: Some("const slaTimer = slaMonitor.startTimer('api_call'); try { const result = await apiCall(); } finally { slaTimer.end(); }".to_string()),
        }).collect()
    }
}

/// Rule: require-error-rate-monitoring
/// Enforces error rate monitoring and threshold alerting
#[derive(Clone)]
pub struct RequireErrorRateMonitoring;

impl RequireErrorRateMonitoring {
    pub const NAME: &'static str = "require-error-rate-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireErrorRateMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for request handlers without error rate tracking
        if code.contains("app.") && code.contains("(req, res") && !code.contains("errorRate") {
            diagnostics.push(create_error_rate_monitoring_diagnostic(
                0, 0,
                "Request handlers should track error rates for monitoring"
            ));
        }

        // Check for batch processing without error monitoring
        if code.contains("Promise.all") && !code.contains("errorCount") {
            diagnostics.push(create_error_rate_monitoring_diagnostic(
                0, 0,
                "Batch operations should monitor and report error rates"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireErrorRateMonitoring {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Add error rate monitoring: `errorRateTracker.increment(); if (errorRateTracker.getRate() > threshold) alert();`".to_string(),
            fix_code: Some("try { await operation(); successCounter.increment(); } catch (error) { errorCounter.increment(); throw error; }".to_string()),
        }).collect()
    }
}

/// Rule: require-custom-metrics
/// Enforces custom business metrics collection and reporting
#[derive(Clone)]
pub struct RequireCustomMetrics;

impl RequireCustomMetrics {
    pub const NAME: &'static str = "require-custom-metrics";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCustomMetrics {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for business logic without metrics
        if code.contains("purchase") || code.contains("order") || code.contains("payment") {
            if !code.contains("metrics.") && !code.contains("counter") {
                diagnostics.push(create_custom_metrics_diagnostic(
                    0, 0,
                    "Business logic should include custom metrics collection"
                ));
            }
        }

        // Check for user actions without tracking
        if code.contains("login") || code.contains("signup") {
            if !code.contains("track") && !code.contains("metrics") {
                diagnostics.push(create_custom_metrics_diagnostic(
                    0, 0,
                    "User actions should be tracked with custom metrics"
                ));
            }
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCustomMetrics {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.84,
            suggestion: "Add custom metrics: `businessMetrics.incrementCounter('user_purchases', { category: 'electronics' });`".to_string(),
            fix_code: Some("metricsCollector.recordBusinessEvent('purchase_completed', { amount, category, userId });".to_string()),
        }).collect()
    }
}

/// Rule: require-performance-budgets
/// Enforces performance budget monitoring and alerting
#[derive(Clone)]
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

        // Check for bundle size without monitoring
        if code.contains("webpack") && !code.contains("budget") && !code.contains("size-limit") {
            diagnostics.push(create_performance_budgets_diagnostic(
                0, 0,
                "Webpack builds should include performance budget monitoring"
            ));
        }

        // Check for API responses without performance budgets
        if code.contains("response") && code.contains("json") && !code.contains("timing") {
            diagnostics.push(create_performance_budgets_diagnostic(
                0, 0,
                "API responses should be monitored against performance budgets"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceBudgets {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.85,
            suggestion: "Add performance budget monitoring: `if (responseTime > BUDGET_THRESHOLD) { alert('Performance budget exceeded'); }`".to_string(),
            fix_code: Some("const budget = performanceBudgets.get('api_response_time'); if (duration > budget) { budgetMonitor.recordViolation('api_response_time', duration); }".to_string()),
        }).collect()
    }
}

/// Rule: require-monitoring-dashboards
/// Enforces monitoring dashboard integration and visualization
#[derive(Clone)]
pub struct RequireMonitoringDashboards;

impl RequireMonitoringDashboards {
    pub const NAME: &'static str = "require-monitoring-dashboards";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMonitoringDashboards {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for metrics without dashboard integration
        if code.contains("metrics.") && !code.contains("dashboard") && !code.contains("grafana") {
            diagnostics.push(create_monitoring_dashboards_diagnostic(
                0, 0,
                "Metrics collection should be integrated with monitoring dashboards"
            ));
        }

        // Check for custom metrics without visualization
        if code.contains("createCounter") && !code.contains("export") {
            diagnostics.push(create_monitoring_dashboards_diagnostic(
                0, 0,
                "Custom metrics should be exported for dashboard visualization"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMonitoringDashboards {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.82,
            suggestion: "Add dashboard integration: `metricsExporter.exportToPrometheus(metrics);` for Grafana visualization".to_string(),
            fix_code: Some("const prometheusRegistry = require('prom-client').register; prometheusRegistry.registerMetric(customMetric);".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_performance_metrics_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMetrics::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_distributed_tracing_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDistributedTracing::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_structured_logging_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireStructuredLogging::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_health_checks_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireHealthChecks::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_memory_profiling_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryProfiling::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_alerting_integration_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAlertingIntegration::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_sla_monitoring_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSLAMonitoring::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_error_rate_monitoring_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireErrorRateMonitoring::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_custom_metrics_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCustomMetrics::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_performance_budgets_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceBudgets::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_monitoring_dashboards_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMonitoringDashboards::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_performance_metrics_violation() {
        let code = r#"
        app.get('/api/users', (req, res) => {
            const users = getUsersFromDatabase();
            res.json(users);
        });
        "#;

        let rule = RequirePerformanceMetrics;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("performance metrics"));
    }

    #[test]
    fn test_require_performance_metrics_compliant() {
        let code = r#"
        const metrics = require('@opentelemetry/metrics');
        app.get('/api/users', (req, res) => {
            const start = performance.mark('start');
            const users = getUsersFromDatabase();
            performance.measure('db-query', 'start');
            res.json(users);
        });
        "#;

        let rule = RequirePerformanceMetrics;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_distributed_tracing_violation() {
        let code = r#"
        const response = await fetch('/api/data');
        const result = await response.json();
        "#;

        let rule = RequireDistributedTracing;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("trace context"));
    }

    #[test]
    fn test_require_distributed_tracing_compliant() {
        let code = r#"
        const span = tracer.startSpan('api-call');
        const response = await fetch('/api/data', {
            headers: { 'x-trace-id': span.spanContext().traceId }
        });
        span.end();
        "#;

        let rule = RequireDistributedTracing;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_structured_logging_violation() {
        let code = r#"
        function processOrder(order) {
            console.log('Processing order', order.id);
            // process order
        }
        "#;

        let rule = RequireStructuredLogging;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("structured logging"));
    }

    #[test]
    fn test_require_structured_logging_compliant() {
        let code = r#"
        function processOrder(order) {
            logger.info('Processing order', { orderId: order.id, userId: order.userId });
            // process order
        }
        "#;

        let rule = RequireStructuredLogging;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_health_checks_violation() {
        let code = r#"
        const app = express();
        app.get('/api/users', (req, res) => res.json(users));
        "#;

        let rule = RequireHealthChecks;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("health check"));
    }

    #[test]
    fn test_require_health_checks_compliant() {
        let code = r#"
        const app = express();
        app.get('/health', (req, res) => res.status(200).json({ status: 'healthy' }));
        app.get('/api/users', (req, res) => res.json(users));
        "#;

        let rule = RequireHealthChecks;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_memory_profiling_violation() {
        let code = r#"
        setInterval(() => {
            processLargeDataSet();
        }, 1000);
        "#;

        let rule = RequireMemoryProfiling;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("memory usage"));
    }

    #[test]
    fn test_require_memory_profiling_compliant() {
        let code = r#"
        setInterval(() => {
            const memBefore = process.memoryUsage();
            processLargeDataSet();
            const memAfter = process.memoryUsage();
            logger.info('Memory usage', { before: memBefore, after: memAfter });
        }, 1000);
        "#;

        let rule = RequireMemoryProfiling;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_ai_enhancement_performance_metrics() {
        let rule = RequirePerformanceMetrics;
        let diagnostics = vec![create_performance_metrics_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].suggestion.contains("OpenTelemetry"));
    }

    #[test]
    fn test_ai_enhancement_distributed_tracing() {
        let rule = RequireDistributedTracing;
        let diagnostics = vec![create_distributed_tracing_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].suggestion.contains("tracer.startSpan"));
    }
}