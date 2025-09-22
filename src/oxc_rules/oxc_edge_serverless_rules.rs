//! Edge computing and serverless rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireEdgeRuntimeCompatibility;

impl RequireEdgeRuntimeCompatibility {
    pub const NAME: &'static str = "require-edge-runtime-compatibility";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireEdgeRuntimeCompatibility {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_edge_function(ctx) && self.uses_incompatible_api(&import_decl.source.value) {
                ctx.diagnostic(require_edge_runtime_compatibility_diagnostic(import_decl.span));
            }
        }
    }
}

impl RequireEdgeRuntimeCompatibility {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        let filename = ctx.filename();
        filename.contains("edge") || filename.contains("worker") || filename.contains("api/")
    }

    fn uses_incompatible_api(&self, module_name: &str) -> bool {
        // Check for Node.js APIs not available in edge runtime
        matches!(module_name, "fs" | "path" | "os" | "crypto" | "child_process")
    }
}

fn require_edge_runtime_compatibility_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Node.js API not available in edge runtime")
        .with_help("Use Web APIs or edge-compatible alternatives")
        .with_label(span)
}

impl EnhancedWasmRule for RequireEdgeRuntimeCompatibility {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Web Crypto API instead of Node.js crypto".to_string(),
            "Use fetch() instead of Node.js http module".to_string(),
            "Use URL API instead of Node.js path module".to_string(),
            "Check edge runtime documentation for supported APIs".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoBlockingOperationsInEdge;

impl NoBlockingOperationsInEdge {
    pub const NAME: &'static str = "no-blocking-operations-in-edge";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoBlockingOperationsInEdge {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_edge_function(ctx) && self.is_blocking_operation(call) {
                ctx.diagnostic(no_blocking_operations_in_edge_diagnostic(call.span));
            }
        }
    }
}

impl NoBlockingOperationsInEdge {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("edge") || ctx.filename().contains("worker")
    }

    fn is_blocking_operation(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for synchronous operations that block in edge functions
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name.ends_with("Sync");
            }
        }
        false
    }
}

fn no_blocking_operations_in_edge_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Blocking operation in edge function")
        .with_help("Use async alternatives for better edge performance")
        .with_label(span)
}

impl EnhancedWasmRule for NoBlockingOperationsInEdge {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Edge functions have strict execution time limits".to_string(),
            "Use async/await for all I/O operations".to_string(),
            "Minimize cold start time with smaller bundles".to_string(),
            "Use streaming responses for large data".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireServerlessTimeout;

impl RequireServerlessTimeout {
    pub const NAME: &'static str = "require-serverless-timeout";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireServerlessTimeout {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_serverless_function(ctx) && self.is_async_operation_without_timeout(call) {
                ctx.diagnostic(require_serverless_timeout_diagnostic(call.span));
            }
        }
    }
}

impl RequireServerlessTimeout {
    fn is_serverless_function(&self, ctx: &WasmLintContext) -> bool {
        let filename = ctx.filename();
        filename.contains("lambda") || filename.contains("function") || filename.contains("api/")
    }

    fn is_async_operation_without_timeout(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for fetch, database calls, etc. without timeout
        if let Some(ident) = call.callee.as_identifier() {
            return matches!(ident.name.as_str(), "fetch" | "query" | "execute");
        }
        false
    }
}

fn require_serverless_timeout_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async operation without timeout in serverless function")
        .with_help("Add timeout to prevent function from hanging")
        .with_label(span)
}

impl EnhancedWasmRule for RequireServerlessTimeout {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use AbortController with timeout for fetch requests".to_string(),
            "Set database connection timeouts".to_string(),
            "Serverless functions have execution time limits".to_string(),
            "Implement circuit breaker pattern for external services".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLargeBundlesInEdge;

impl NoLargeBundlesInEdge {
    pub const NAME: &'static str = "no-large-bundles-in-edge";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoLargeBundlesInEdge {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_edge_function(ctx) && self.is_large_dependency(&import_decl.source.value) {
                ctx.diagnostic(no_large_bundles_in_edge_diagnostic(import_decl.span));
            }
        }
    }
}

impl NoLargeBundlesInEdge {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("edge") || ctx.filename().contains("worker")
    }

    fn is_large_dependency(&self, module_name: &str) -> bool {
        // Check for large libraries that increase cold start time
        matches!(module_name, "lodash" | "moment" | "aws-sdk" | "firebase")
    }
}

fn no_large_bundles_in_edge_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Large dependency in edge function")
        .with_help("Use smaller alternatives or tree-shaking to reduce bundle size")
        .with_label(span)
}

impl EnhancedWasmRule for NoLargeBundlesInEdge {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use date-fns instead of moment for date manipulation".to_string(),
            "Use specific AWS SDK v3 clients instead of full SDK".to_string(),
            "Import only needed functions: import { debounce } from 'lodash'".to_string(),
            "Bundle size affects cold start performance".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireEdgeErrorHandling;

impl RequireEdgeErrorHandling {
    pub const NAME: &'static str = "require-edge-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireEdgeErrorHandling {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_edge_handler(func, ctx) && !self.has_error_handling(func) {
                ctx.diagnostic(require_edge_error_handling_diagnostic(func.span));
            }
        }
    }
}

impl RequireEdgeErrorHandling {
    fn is_edge_handler(&self, func: &oxc_ast::ast::Function, ctx: &WasmLintContext) -> bool {
        // Check if this is an edge function handler
        ctx.filename().contains("edge") && func.id.as_ref().map_or(false, |id| id.name == "handler")
    }

    fn has_error_handling(&self, _func: &oxc_ast::ast::Function) -> bool {
        // Check for try-catch blocks or error handling
        false
    }
}

fn require_edge_error_handling_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Edge function without error handling")
        .with_help("Add try-catch blocks and proper error responses")
        .with_label(span)
}

impl EnhancedWasmRule for RequireEdgeErrorHandling {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Return proper HTTP status codes for errors".to_string(),
            "Log errors for debugging without exposing details".to_string(),
            "Implement graceful degradation for service failures".to_string(),
            "Use structured error responses for API endpoints".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoStatefulOperationsInEdge;

impl NoStatefulOperationsInEdge {
    pub const NAME: &'static str = "no-stateful-operations-in-edge";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoStatefulOperationsInEdge {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclarator(var) = node.kind() {
            if self.is_edge_function(ctx) && self.is_global_state(var) {
                ctx.diagnostic(no_stateful_operations_in_edge_diagnostic(var.span));
            }
        }
    }
}

impl NoStatefulOperationsInEdge {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("edge") || ctx.filename().contains("worker")
    }

    fn is_global_state(&self, var: &oxc_ast::ast::VariableDeclarator) -> bool {
        // Check for global variables that maintain state
        var.id.get_identifier().map_or(false, |id| {
            !id.starts_with('_') && id.chars().any(char::is_uppercase)
        })
    }
}

fn no_stateful_operations_in_edge_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Stateful operation in edge function")
        .with_help("Edge functions should be stateless for proper scaling")
        .with_label(span)
}

impl EnhancedWasmRule for NoStatefulOperationsInEdge {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Store state in external databases or cache".to_string(),
            "Edge functions are stateless and can be terminated anytime".to_string(),
            "Use request-scoped variables instead of global state".to_string(),
            "Store session data in cookies or JWT tokens".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireStreamingResponse;

impl RequireStreamingResponse {
    pub const NAME: &'static str = "require-streaming-response";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireStreamingResponse {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_edge_function(ctx) && self.returns_large_response(call) {
                ctx.diagnostic(require_streaming_response_diagnostic(call.span));
            }
        }
    }
}

impl RequireStreamingResponse {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("edge") || ctx.filename().contains("api/")
    }

    fn returns_large_response(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for operations that might return large data
        if let Some(ident) = call.callee.as_identifier() {
            return matches!(ident.name.as_str(), "json" | "text" | "arrayBuffer");
        }
        false
    }
}

fn require_streaming_response_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Large response without streaming")
        .with_help("Use streaming responses for large data to improve performance")
        .with_label(span)
}

impl EnhancedWasmRule for RequireStreamingResponse {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use ReadableStream for large responses".to_string(),
            "Implement pagination for large datasets".to_string(),
            "Stream data as it becomes available".to_string(),
            "Reduce memory usage and improve perceived performance".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoEdgeEnvironmentLeakage;

impl NoEdgeEnvironmentLeakage {
    pub const NAME: &'static str = "no-edge-environment-leakage";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoEdgeEnvironmentLeakage {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::MemberExpression(member) = node.kind() {
            if self.is_edge_function(ctx) && self.accesses_sensitive_env(member) {
                ctx.diagnostic(no_edge_environment_leakage_diagnostic(member.span));
            }
        }
    }
}

impl NoEdgeEnvironmentLeakage {
    fn is_edge_function(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("edge") || ctx.filename().contains("worker")
    }

    fn accesses_sensitive_env(&self, member: &oxc_ast::ast::MemberExpression) -> bool {
        // Check for process.env access in edge functions
        if let Some(obj) = member.object().as_identifier() {
            if obj.name == "process" {
                if let Some(prop) = member.property().as_identifier() {
                    return prop.name == "env";
                }
            }
        }
        false
    }
}

fn no_edge_environment_leakage_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Environment variable access in edge function")
        .with_help("Validate and sanitize environment variables for edge runtime")
        .with_label(span)
}

impl EnhancedWasmRule for NoEdgeEnvironmentLeakage {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Edge runtimes may have limited environment variable access".to_string(),
            "Use build-time environment variable injection".to_string(),
            "Validate environment variables at startup".to_string(),
            "Consider using edge runtime-specific configuration".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_edge_runtime_compatibility_rule() {
        assert_eq!(RequireEdgeRuntimeCompatibility::NAME, "require-edge-runtime-compatibility");
        assert_eq!(RequireEdgeRuntimeCompatibility::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_blocking_operations_in_edge_rule() {
        assert_eq!(NoBlockingOperationsInEdge::NAME, "no-blocking-operations-in-edge");
        assert_eq!(NoBlockingOperationsInEdge::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_incompatible_api_detection() {
        let rule = RequireEdgeRuntimeCompatibility;
        assert!(rule.uses_incompatible_api("fs"));
        assert!(rule.uses_incompatible_api("child_process"));
        assert!(!rule.uses_incompatible_api("crypto-js"));
    }

    #[test]
    fn test_large_dependency_detection() {
        let rule = NoLargeBundlesInEdge;
        assert!(rule.is_large_dependency("lodash"));
        assert!(rule.is_large_dependency("moment"));
        assert!(!rule.is_large_dependency("date-fns"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireEdgeRuntimeCompatibility;
        let diagnostic = require_edge_runtime_compatibility_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Web Crypto API"));
    }
}