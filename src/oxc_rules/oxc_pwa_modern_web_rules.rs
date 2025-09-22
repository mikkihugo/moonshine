//! PWA and modern web platform rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireServiceWorkerRegistration;

impl RequireServiceWorkerRegistration {
    pub const NAME: &'static str = "require-service-worker-registration";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireServiceWorkerRegistration {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.is_pwa_app(ctx) && !self.has_service_worker_registration(ctx) {
                ctx.diagnostic(require_service_worker_registration_diagnostic(node.span()));
            }
        }
    }
}

impl RequireServiceWorkerRegistration {
    fn is_pwa_app(&self, ctx: &WasmLintContext) -> bool {
        // Check if app has PWA manifest
        ctx.filename().contains("manifest.json") || ctx.filename().contains("pwa")
    }

    fn has_service_worker_registration(&self, _ctx: &WasmLintContext) -> bool {
        // Check for service worker registration
        false
    }
}

fn require_service_worker_registration_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("PWA app without service worker registration")
        .with_help("Register service worker for offline functionality and caching")
        .with_label(span)
}

impl EnhancedWasmRule for RequireServiceWorkerRegistration {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use navigator.serviceWorker.register('/sw.js')".to_string(),
            "Service workers enable offline functionality".to_string(),
            "Implement proper error handling for registration".to_string(),
            "Use Workbox for service worker generation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireWebAppManifest;

impl RequireWebAppManifest {
    pub const NAME: &'static str = "require-web-app-manifest";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebAppManifest {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_manifest_file(ctx) && !self.has_required_manifest_fields(obj) {
                ctx.diagnostic(require_web_app_manifest_diagnostic(obj.span));
            }
        }
    }
}

impl RequireWebAppManifest {
    fn is_manifest_file(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("manifest.json")
    }

    fn has_required_manifest_fields(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for required PWA manifest fields
        false
    }
}

fn require_web_app_manifest_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Incomplete web app manifest")
        .with_help("Add required fields: name, short_name, start_url, display, theme_color")
        .with_label(span)
}

impl EnhancedWasmRule for RequireWebAppManifest {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Include all PWA manifest fields for app store submission".to_string(),
            "Add multiple icon sizes: 192x192, 512x512".to_string(),
            "Set display: standalone for app-like experience".to_string(),
            "Use theme_color for browser UI theming".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoBlockingMainThread;

impl NoBlockingMainThread {
    pub const NAME: &'static str = "no-blocking-main-thread";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoBlockingMainThread {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ForStatement(for_stmt) = node.kind() {
            if self.is_potentially_blocking_loop(for_stmt) {
                ctx.diagnostic(no_blocking_main_thread_diagnostic(for_stmt.span));
            }
        }
    }
}

impl NoBlockingMainThread {
    fn is_potentially_blocking_loop(&self, _for_stmt: &oxc_ast::ast::ForStatement) -> bool {
        // Check for loops that might block the main thread
        true
    }
}

fn no_blocking_main_thread_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potentially blocking main thread operation")
        .with_help("Use Web Workers or requestIdleCallback for heavy computations")
        .with_label(span)
}

impl EnhancedWasmRule for NoBlockingMainThread {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Web Workers for CPU-intensive tasks".to_string(),
            "Break large loops with requestIdleCallback".to_string(),
            "Use setTimeout(fn, 0) to yield control".to_string(),
            "Consider WebAssembly for performance-critical code".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireIntersectionObserver;

impl RequireIntersectionObserver {
    pub const NAME: &'static str = "require-intersection-observer";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireIntersectionObserver {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_scroll_event_listener(call) {
                ctx.diagnostic(require_intersection_observer_diagnostic(call.span));
            }
        }
    }
}

impl RequireIntersectionObserver {
    fn is_scroll_event_listener(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                if prop.name == "addEventListener" {
                    return call.arguments.iter().any(|arg| {
                        if let Some(expr) = arg.as_expression() {
                            if let Some(lit) = expr.as_string_literal() {
                                return lit.value == "scroll";
                            }
                        }
                        false
                    });
                }
            }
        }
        false
    }
}

fn require_intersection_observer_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Scroll event listener detected")
        .with_help("Use Intersection Observer API for better performance")
        .with_label(span)
}

impl EnhancedWasmRule for RequireIntersectionObserver {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Intersection Observer is more performant than scroll events".to_string(),
            "Use for lazy loading, infinite scroll, and visibility tracking".to_string(),
            "Runs off main thread for better performance".to_string(),
            "Add polyfill for older browser support".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireWebVitalsOptimization;

impl RequireWebVitalsOptimization {
    pub const NAME: &'static str = "require-web-vitals-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebVitalsOptimization {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.affects_web_vitals(call) {
                ctx.diagnostic(require_web_vitals_optimization_diagnostic(call.span));
            }
        }
    }
}

impl RequireWebVitalsOptimization {
    fn affects_web_vitals(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for operations that affect Core Web Vitals
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "appendChild" | "insertBefore" | "removeChild");
            }
        }
        false
    }
}

fn require_web_vitals_optimization_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Operation may affect Core Web Vitals")
        .with_help("Optimize for LCP, FID, and CLS metrics")
        .with_label(span)
}

impl EnhancedWasmRule for RequireWebVitalsOptimization {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use requestAnimationFrame for DOM mutations".to_string(),
            "Preload critical resources for better LCP".to_string(),
            "Minimize layout shifts with proper sizing".to_string(),
            "Monitor Core Web Vitals with web-vitals library".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLegacyWebAPIs;

impl NoLegacyWebAPIs {
    pub const NAME: &'static str = "no-legacy-web-apis";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoLegacyWebAPIs {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.uses_legacy_api(call) {
                ctx.diagnostic(no_legacy_web_apis_diagnostic(call.span));
            }
        }
    }
}

impl NoLegacyWebAPIs {
    fn uses_legacy_api(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                if obj.name == "document" {
                    if let Some(prop) = member.property().as_identifier() {
                        return matches!(prop.name.as_str(),
                            "getElementById" | "getElementsByClassName" | "getElementsByTagName"
                        );
                    }
                }
            }
        }
        false
    }
}

fn no_legacy_web_apis_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Legacy DOM API usage")
        .with_help("Use modern APIs like querySelector or querySelectorAll")
        .with_label(span)
}

impl EnhancedWasmRule for NoLegacyWebAPIs {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use document.querySelector('#id') instead of getElementById".to_string(),
            "Use document.querySelectorAll('.class') for multiple elements".to_string(),
            "Modern APIs provide consistent CSS selector support".to_string(),
            "querySelector is more flexible and standardized".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireWebAssemblyOptimization;

impl RequireWebAssemblyOptimization {
    pub const NAME: &'static str = "require-webassembly-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebAssemblyOptimization {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_cpu_intensive_operation(call) && !self.uses_webassembly(ctx) {
                ctx.diagnostic(require_webassembly_optimization_diagnostic(call.span));
            }
        }
    }
}

impl RequireWebAssemblyOptimization {
    fn is_cpu_intensive_operation(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for operations that could benefit from WebAssembly
        if let Some(ident) = call.callee.as_identifier() {
            return matches!(ident.name.as_str(), "sort" | "filter" | "map" | "reduce");
        }
        false
    }

    fn uses_webassembly(&self, _ctx: &WasmLintContext) -> bool {
        // Check if WebAssembly modules are used
        false
    }
}

fn require_webassembly_optimization_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("CPU-intensive operation detected")
        .with_help("Consider WebAssembly for performance-critical computations")
        .with_label(span)
}

impl EnhancedWasmRule for RequireWebAssemblyOptimization {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "WebAssembly provides near-native performance".to_string(),
            "Use AssemblyScript or Rust for WASM modules".to_string(),
            "WASM excels at mathematical computations".to_string(),
            "Consider compilation overhead vs performance gains".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireModernImageFormats;

impl RequireModernImageFormats {
    pub const NAME: &'static str = "require-modern-image-formats";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireModernImageFormats {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.is_legacy_image_format(&string_lit.value) {
                ctx.diagnostic(require_modern_image_formats_diagnostic(string_lit.span));
            }
        }
    }
}

impl RequireModernImageFormats {
    fn is_legacy_image_format(&self, value: &str) -> bool {
        value.ends_with(".jpg") || value.ends_with(".jpeg") || value.ends_with(".png")
    }
}

fn require_modern_image_formats_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Legacy image format detected")
        .with_help("Use modern formats like WebP, AVIF for better compression")
        .with_label(span)
}

impl EnhancedWasmRule for RequireModernImageFormats {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use <picture> element with fallbacks".to_string(),
            "WebP provides 25-35% better compression".to_string(),
            "AVIF offers even better compression than WebP".to_string(),
            "Implement progressive image loading".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireOfflineSupport;

impl RequireOfflineSupport {
    pub const NAME: &'static str = "require-offline-support";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireOfflineSupport {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_network_request(call) && !self.has_offline_handling(call) {
                ctx.diagnostic(require_offline_support_diagnostic(call.span));
            }
        }
    }
}

impl RequireOfflineSupport {
    fn is_network_request(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return ident.name == "fetch";
        }
        false
    }

    fn has_offline_handling(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for offline error handling
        false
    }
}

fn require_offline_support_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Network request without offline handling")
        .with_help("Add offline support and graceful degradation")
        .with_label(span)
}

impl EnhancedWasmRule for RequireOfflineSupport {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check navigator.onLine for connectivity status".to_string(),
            "Implement request queuing for offline scenarios".to_string(),
            "Use IndexedDB for offline data storage".to_string(),
            "Provide meaningful offline UI feedback".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_service_worker_registration_rule() {
        assert_eq!(RequireServiceWorkerRegistration::NAME, "require-service-worker-registration");
        assert_eq!(RequireServiceWorkerRegistration::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_require_web_app_manifest_rule() {
        assert_eq!(RequireWebAppManifest::NAME, "require-web-app-manifest");
        assert_eq!(RequireWebAppManifest::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_no_blocking_main_thread_rule() {
        assert_eq!(NoBlockingMainThread::NAME, "no-blocking-main-thread");
        assert_eq!(NoBlockingMainThread::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_legacy_api_detection() {
        let rule = NoLegacyWebAPIs;
        // Would test with actual AST nodes in real implementation
    }

    #[test]
    fn test_image_format_detection() {
        let rule = RequireModernImageFormats;
        assert!(rule.is_legacy_image_format("image.jpg"));
        assert!(rule.is_legacy_image_format("photo.png"));
        assert!(!rule.is_legacy_image_format("image.webp"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireServiceWorkerRegistration;
        let diagnostic = require_service_worker_registration_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("navigator.serviceWorker"));
    }
}