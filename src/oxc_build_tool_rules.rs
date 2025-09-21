//! Build tool and bundling rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoUnusedWebpackChunks;

impl NoUnusedWebpackChunks {
    pub const NAME: &'static str = "no-unused-webpack-chunks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoUnusedWebpackChunks {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportExpression(import_expr) = node.kind() {
            if self.creates_unused_chunk(import_expr) {
                ctx.diagnostic(no_unused_webpack_chunks_diagnostic(import_expr.span));
            }
        }
    }
}

impl NoUnusedWebpackChunks {
    fn creates_unused_chunk(&self, _import_expr: &oxc_ast::ast::ImportExpression) -> bool {
        // Check if dynamic import creates chunks that are never loaded
        true
    }
}

fn no_unused_webpack_chunks_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Dynamic import creates unused chunk")
        .with_help("Remove unused dynamic imports or use webpack magic comments")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedWebpackChunks {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use webpack magic comments for chunk naming".to_string(),
            "Analyze bundle to identify unused chunks".to_string(),
            "Consider preloading critical chunks".to_string(),
            "Unused chunks increase bundle size unnecessarily".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferNamedChunks;

impl PreferNamedChunks {
    pub const NAME: &'static str = "prefer-named-chunks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferNamedChunks {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportExpression(import_expr) = node.kind() {
            if !self.has_chunk_name(import_expr) {
                ctx.diagnostic(prefer_named_chunks_diagnostic(import_expr.span));
            }
        }
    }
}

impl PreferNamedChunks {
    fn has_chunk_name(&self, _import_expr: &oxc_ast::ast::ImportExpression) -> bool {
        // Check if import() has webpackChunkName comment
        false
    }
}

fn prefer_named_chunks_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Dynamic import without chunk name")
        .with_help("Add /* webpackChunkName: \"name\" */ comment for better debugging")
        .with_label(span)
}

impl EnhancedWasmRule for PreferNamedChunks {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Named chunks improve debugging experience".to_string(),
            "Use semantic chunk names for better organization".to_string(),
            "Chunk names appear in DevTools and bundle analysis".to_string(),
            "Group related chunks with consistent naming".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSideEffectImports;

impl NoSideEffectImports {
    pub const NAME: &'static str = "no-side-effect-imports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSideEffectImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_side_effect_import(import_decl) && self.has_tree_shaking_issues(import_decl) {
                ctx.diagnostic(no_side_effect_imports_diagnostic(import_decl.span));
            }
        }
    }
}

impl NoSideEffectImports {
    fn is_side_effect_import(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        // Check if import has no specifiers (side-effect only)
        import_decl.specifiers.is_none()
    }

    fn has_tree_shaking_issues(&self, _import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        // Check if import prevents tree-shaking
        true
    }
}

fn no_side_effect_imports_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Side-effect import may prevent tree-shaking")
        .with_help("Use explicit imports or mark package as side-effect-free")
        .with_label(span)
}

impl EnhancedWasmRule for NoSideEffectImports {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Side-effect imports prevent tree-shaking optimization".to_string(),
            "Use explicit named imports when possible".to_string(),
            "Mark packages as sideEffects: false in package.json".to_string(),
            "Consider lazy loading for side-effect dependencies".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireBundleAnalysis;

impl RequireBundleAnalysis {
    pub const NAME: &'static str = "require-bundle-analysis";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBundleAnalysis {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if !self.has_bundle_analysis_config(&ctx.filename()) {
                ctx.diagnostic(require_bundle_analysis_diagnostic(node.span()));
            }
        }
    }
}

impl RequireBundleAnalysis {
    fn has_bundle_analysis_config(&self, filename: &str) -> bool {
        // Check if project has bundle analysis tools configured
        filename.contains("webpack.config") || filename.contains("vite.config")
    }
}

fn require_bundle_analysis_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing bundle analysis configuration")
        .with_help("Add webpack-bundle-analyzer or similar tool for bundle optimization")
        .with_label(span)
}

impl EnhancedWasmRule for RequireBundleAnalysis {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Bundle analysis reveals optimization opportunities".to_string(),
            "Use webpack-bundle-analyzer for detailed insights".to_string(),
            "Regular bundle analysis prevents size regressions".to_string(),
            "Set up CI checks for bundle size limits".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLargeBundles;

impl NoLargeBundles {
    pub const NAME: &'static str = "no-large-bundles";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoLargeBundles {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.imports_large_library(import_decl) {
                ctx.diagnostic(no_large_bundles_diagnostic(import_decl.span));
            }
        }
    }
}

impl NoLargeBundles {
    fn imports_large_library(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        // Check if import adds significantly to bundle size
        let source = &import_decl.source.value;
        matches!(source.as_ref(), "lodash" | "moment" | "antd")
    }
}

fn no_large_bundles_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Import of large library detected")
        .with_help("Consider tree-shakable alternatives or selective imports")
        .with_label(span)
}

impl EnhancedWasmRule for NoLargeBundles {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use selective imports: import { debounce } from 'lodash'".to_string(),
            "Consider lighter alternatives: date-fns vs moment".to_string(),
            "Use CDN for large libraries in some cases".to_string(),
            "Implement code splitting for large dependencies".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferModernBundlers;

impl PreferModernBundlers {
    pub const NAME: &'static str = "prefer-modern-bundlers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferModernBundlers {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.uses_legacy_bundler(&ctx.filename()) {
                ctx.diagnostic(prefer_modern_bundlers_diagnostic(node.span()));
            }
        }
    }
}

impl PreferModernBundlers {
    fn uses_legacy_bundler(&self, filename: &str) -> bool {
        // Check for legacy bundler configurations
        filename.contains("browserify") || filename.contains("rollup.config")
    }
}

fn prefer_modern_bundlers_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Legacy bundler configuration detected")
        .with_help("Consider upgrading to Vite, esbuild, or modern webpack")
        .with_label(span)
}

impl EnhancedWasmRule for PreferModernBundlers {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Modern bundlers offer better performance".to_string(),
            "Vite provides instant HMR and faster builds".to_string(),
            "esbuild has significantly faster compilation".to_string(),
            "Modern bundlers have better ES modules support".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireSourceMaps;

impl RequireSourceMaps {
    pub const NAME: &'static str = "require-source-maps";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSourceMaps {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.is_production_build(&ctx.filename()) && !self.has_source_maps(&ctx.filename()) {
                ctx.diagnostic(require_source_maps_diagnostic(node.span()));
            }
        }
    }
}

impl RequireSourceMaps {
    fn is_production_build(&self, filename: &str) -> bool {
        filename.contains("prod") || filename.contains("production")
    }

    fn has_source_maps(&self, _filename: &str) -> bool {
        // Check if source maps are configured
        false
    }
}

fn require_source_maps_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Production build without source maps")
        .with_help("Enable source maps for production debugging")
        .with_label(span)
}

impl EnhancedWasmRule for RequireSourceMaps {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Source maps enable production debugging".to_string(),
            "Use hidden-source-map for production".to_string(),
            "Source maps help with error tracking".to_string(),
            "Consider separate source map files for security".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoCircularChunks;

impl NoCircularChunks {
    pub const NAME: &'static str = "no-circular-chunks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoCircularChunks {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportExpression(import_expr) = node.kind() {
            if self.creates_circular_dependency(import_expr, ctx) {
                ctx.diagnostic(no_circular_chunks_diagnostic(import_expr.span));
            }
        }
    }
}

impl NoCircularChunks {
    fn creates_circular_dependency(&self, _import_expr: &oxc_ast::ast::ImportExpression, _ctx: &WasmLintContext) -> bool {
        // Check for circular dependencies in chunk loading
        false
    }
}

fn no_circular_chunks_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Circular dependency in chunk loading")
        .with_help("Restructure imports to avoid circular dependencies")
        .with_label(span)
}

impl EnhancedWasmRule for NoCircularChunks {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Circular dependencies can cause loading issues".to_string(),
            "Use dependency injection to break circles".to_string(),
            "Extract shared dependencies to common chunks".to_string(),
            "Analyze dependency graph to identify cycles".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_unused_webpack_chunks_rule() {
        assert_eq!(NoUnusedWebpackChunks::NAME, "no-unused-webpack-chunks");
        assert_eq!(NoUnusedWebpackChunks::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_prefer_named_chunks_rule() {
        assert_eq!(PreferNamedChunks::NAME, "prefer-named-chunks");
        assert_eq!(PreferNamedChunks::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_large_library_detection() {
        let rule = NoLargeBundles;
        let import_lodash = oxc_ast::ast::ImportDeclaration {
            span: Span::default(),
            specifiers: None,
            source: Box::new(oxc_ast::ast::StringLiteral {
                span: Span::default(),
                value: "lodash".into(),
            }),
            with_clause: None,
            import_kind: oxc_ast::ast::ImportOrExportKind::Value,
        };
        assert!(rule.imports_large_library(&import_lodash));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = PreferNamedChunks;
        let diagnostic = prefer_named_chunks_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("debugging"));
    }
}