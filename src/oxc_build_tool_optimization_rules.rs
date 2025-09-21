//! Build Tool Optimization Rules
//!
//! Advanced build tool optimization and configuration rules for modern web development.
//! Focuses on Webpack, Vite, Rollup, Parcel, and other bundlers with performance optimization.

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

/// Require tree-shaking configuration for bundle optimization
pub struct RequireTreeShaking;

impl RequireTreeShaking {
    pub const NAME: &'static str = "require-tree-shaking";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTreeShaking {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing tree-shaking configuration
        if code.contains("webpack.config") && !code.contains("optimization") {
            diagnostics.push(create_tree_shaking_diagnostic());
        }

        // Check for vite config without tree-shaking
        if code.contains("vite.config") && !code.contains("treeshake") {
            diagnostics.push(create_vite_tree_shaking_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTreeShaking {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Configure tree-shaking to eliminate dead code and reduce bundle size. Modern bundlers can remove unused exports automatically.".to_string(),
            confidence: 0.92,
        }).collect()
    }
}

fn create_tree_shaking_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTreeShaking::NAME.to_string(),
        message: "Build configuration should enable tree-shaking for optimal bundle size".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_vite_tree_shaking_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTreeShaking::NAME.to_string(),
        message: "Vite configuration should enable tree-shaking optimization".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require code splitting configuration for optimal loading
pub struct RequireCodeSplitting;

impl RequireCodeSplitting {
    pub const NAME: &'static str = "require-code-splitting";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireCodeSplitting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing code splitting in React apps
        if code.contains("React.lazy") && !code.contains("Suspense") {
            diagnostics.push(create_suspense_diagnostic());
        }

        // Check for webpack without split chunks
        if code.contains("webpack.config") && !code.contains("splitChunks") {
            diagnostics.push(create_split_chunks_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCodeSplitting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Implement code splitting to reduce initial bundle size and improve loading performance. Use dynamic imports for route-based splitting.".to_string(),
            confidence: 0.90,
        }).collect()
    }
}

fn create_suspense_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCodeSplitting::NAME.to_string(),
        message: "React.lazy components should be wrapped with Suspense boundary".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_split_chunks_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCodeSplitting::NAME.to_string(),
        message: "Webpack configuration should include splitChunks optimization".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require bundle analysis for performance monitoring
pub struct RequireBundleAnalysis;

impl RequireBundleAnalysis {
    pub const NAME: &'static str = "require-bundle-analysis";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBundleAnalysis {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing bundle analyzer
        if code.contains("webpack.config") && !code.contains("BundleAnalyzerPlugin") {
            diagnostics.push(create_bundle_analyzer_diagnostic());
        }

        // Check for missing rollup bundle analyzer
        if code.contains("rollup.config") && !code.contains("rollup-plugin-analyzer") {
            diagnostics.push(create_rollup_analyzer_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBundleAnalysis {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Add bundle analysis tools to monitor bundle size and identify optimization opportunities. Consider webpack-bundle-analyzer or similar tools.".to_string(),
            confidence: 0.88,
        }).collect()
    }
}

fn create_bundle_analyzer_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBundleAnalysis::NAME.to_string(),
        message: "Webpack configuration should include bundle analyzer for size monitoring".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_rollup_analyzer_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBundleAnalysis::NAME.to_string(),
        message: "Rollup configuration should include bundle analyzer plugin".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require compression configuration for production builds
pub struct RequireCompressionConfig;

impl RequireCompressionConfig {
    pub const NAME: &'static str = "require-compression-config";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireCompressionConfig {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing compression in webpack
        if code.contains("webpack.config") && code.contains("production") && !code.contains("CompressionPlugin") {
            diagnostics.push(create_compression_diagnostic());
        }

        // Check for vite without compression
        if code.contains("vite.config") && !code.contains("compress") {
            diagnostics.push(create_vite_compression_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCompressionConfig {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Enable compression (gzip/brotli) in production builds to reduce bundle transfer size by 60-80%.".to_string(),
            confidence: 0.94,
        }).collect()
    }
}

fn create_compression_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCompressionConfig::NAME.to_string(),
        message: "Production webpack config should enable compression plugin".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_vite_compression_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCompressionConfig::NAME.to_string(),
        message: "Vite production build should enable compression".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require asset optimization for web performance
pub struct RequireAssetOptimization;

impl RequireAssetOptimization {
    pub const NAME: &'static str = "require-asset-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireAssetOptimization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing image optimization
        if code.contains("webpack.config") && !code.contains("imagemin") {
            diagnostics.push(create_image_optimization_diagnostic());
        }

        // Check for missing CSS optimization
        if code.contains("production") && !code.contains("css-minimizer") {
            diagnostics.push(create_css_optimization_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAssetOptimization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Optimize assets (images, CSS, fonts) during build process to improve loading performance and reduce bandwidth usage.".to_string(),
            confidence: 0.89,
        }).collect()
    }
}

fn create_image_optimization_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAssetOptimization::NAME.to_string(),
        message: "Build configuration should include image optimization plugins".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_css_optimization_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAssetOptimization::NAME.to_string(),
        message: "Production build should optimize and minimize CSS assets".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require source maps for debugging in production
pub struct RequireSourceMaps;

impl RequireSourceMaps {
    pub const NAME: &'static str = "require-source-maps";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSourceMaps {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing source maps in development
        if code.contains("development") && !code.contains("devtool") {
            diagnostics.push(create_devtool_diagnostic());
        }

        // Check for inappropriate source maps in production
        if code.contains("production") && code.contains("eval") {
            diagnostics.push(create_production_sourcemap_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSourceMaps {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Configure appropriate source maps: detailed for development (eval-source-map), minimal for production (source-map).".to_string(),
            confidence: 0.91,
        }).collect()
    }
}

fn create_devtool_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSourceMaps::NAME.to_string(),
        message: "Development build should configure devtool for source maps".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_production_sourcemap_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSourceMaps::NAME.to_string(),
        message: "Production build should not use eval-based source maps for security".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require modern build target for optimal output
pub struct RequireModernBuildTarget;

impl RequireModernBuildTarget {
    pub const NAME: &'static str = "require-modern-build-target";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireModernBuildTarget {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for outdated babel targets
        if code.contains("@babel/preset-env") && code.contains("ie 11") {
            diagnostics.push(create_outdated_target_diagnostic());
        }

        // Check for missing browserslist
        if code.contains("webpack.config") && !code.contains("browserslist") {
            diagnostics.push(create_browserslist_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireModernBuildTarget {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Target modern browsers (last 2 versions) to enable smaller bundles and better performance. Consider separate legacy builds if needed.".to_string(),
            confidence: 0.87,
        }).collect()
    }
}

fn create_outdated_target_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireModernBuildTarget::NAME.to_string(),
        message: "Build target includes outdated browsers, consider modern-only builds".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_browserslist_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireModernBuildTarget::NAME.to_string(),
        message: "Configure browserslist for consistent browser targeting across tools".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require parallel processing for faster builds
pub struct RequireParallelProcessing;

impl RequireParallelProcessing {
    pub const NAME: &'static str = "require-parallel-processing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireParallelProcessing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing parallel webpack
        if code.contains("webpack.config") && !code.contains("parallel-webpack") {
            diagnostics.push(create_parallel_webpack_diagnostic());
        }

        // Check for missing thread-loader
        if code.contains("babel-loader") && !code.contains("thread-loader") {
            diagnostics.push(create_thread_loader_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireParallelProcessing {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Enable parallel processing to utilize multiple CPU cores and significantly reduce build times on multi-core machines.".to_string(),
            confidence: 0.86,
        }).collect()
    }
}

fn create_parallel_webpack_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireParallelProcessing::NAME.to_string(),
        message: "Consider using parallel-webpack for faster multi-config builds".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_thread_loader_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireParallelProcessing::NAME.to_string(),
        message: "Use thread-loader to parallelize expensive loader operations".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require caching configuration for incremental builds
pub struct RequireBuildCaching;

impl RequireBuildCaching {
    pub const NAME: &'static str = "require-build-caching";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireBuildCaching {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing cache in webpack
        if code.contains("webpack.config") && !code.contains("cache") {
            diagnostics.push(create_webpack_cache_diagnostic());
        }

        // Check for missing babel cache
        if code.contains("babel-loader") && !code.contains("cacheDirectory") {
            diagnostics.push(create_babel_cache_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBuildCaching {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Enable build caching to dramatically speed up incremental builds. Webpack 5 has built-in filesystem caching.".to_string(),
            confidence: 0.93,
        }).collect()
    }
}

fn create_webpack_cache_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBuildCaching::NAME.to_string(),
        message: "Webpack configuration should enable cache for faster rebuilds".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_babel_cache_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBuildCaching::NAME.to_string(),
        message: "Babel loader should enable cacheDirectory for faster compilation".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_tree_shaking() {
        let rule = RequireTreeShaking;

        // Test webpack config without optimization
        let code_violation = r#"
            module.exports = {
                entry: './src/index.js',
                output: {
                    filename: 'bundle.js'
                }
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper vite config
        let code_compliant = r#"
            export default {
                build: {
                    rollupOptions: {
                        treeshake: true
                    }
                }
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_code_splitting() {
        let rule = RequireCodeSplitting;

        // Test React.lazy without Suspense
        let code_violation = r#"
            const LazyComponent = React.lazy(() => import('./Component'));
            function App() {
                return <LazyComponent />;
            }
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper Suspense usage
        let code_compliant = r#"
            const LazyComponent = React.lazy(() => import('./Component'));
            function App() {
                return (
                    <Suspense fallback={<div>Loading...</div>}>
                        <LazyComponent />
                    </Suspense>
                );
            }
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_bundle_analysis() {
        let rule = RequireBundleAnalysis;

        // Test webpack config without analyzer
        let code_violation = r#"
            const webpack = require('webpack');
            module.exports = {
                plugins: []
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper analyzer usage
        let code_compliant = r#"
            const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;
            module.exports = {
                plugins: [new BundleAnalyzerPlugin()]
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_compression_config() {
        let rule = RequireCompressionConfig;

        // Test production config without compression
        let code_violation = r#"
            module.exports = {
                mode: 'production',
                plugins: []
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper compression setup
        let code_compliant = r#"
            const CompressionPlugin = require('compression-webpack-plugin');
            module.exports = {
                mode: 'production',
                plugins: [new CompressionPlugin()]
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_asset_optimization() {
        let rule = RequireAssetOptimization;

        // Test webpack config without image optimization
        let code_violation = r#"
            module.exports = {
                module: {
                    rules: []
                }
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper asset optimization
        let code_compliant = r#"
            const ImageminPlugin = require('imagemin-webpack-plugin').default;
            module.exports = {
                plugins: [new ImageminPlugin()]
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_source_maps() {
        let rule = RequireSourceMaps;

        // Test development config without source maps
        let code_violation = r#"
            module.exports = {
                mode: 'development'
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper source map configuration
        let code_compliant = r#"
            module.exports = {
                mode: 'development',
                devtool: 'eval-source-map'
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_modern_build_target() {
        let rule = RequireModernBuildTarget;

        // Test outdated browser targets
        let code_violation = r#"
            module.exports = {
                presets: [
                    ['@babel/preset-env', {
                        targets: 'ie 11'
                    }]
                ]
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test modern targets
        let code_compliant = r#"
            module.exports = {
                presets: [
                    ['@babel/preset-env', {
                        targets: 'last 2 versions'
                    }]
                ]
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_parallel_processing() {
        let rule = RequireParallelProcessing;

        // Test babel-loader without threading
        let code_violation = r#"
            module.exports = {
                module: {
                    rules: [{
                        test: /\.js$/,
                        use: 'babel-loader'
                    }]
                }
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test threaded loader configuration
        let code_compliant = r#"
            module.exports = {
                module: {
                    rules: [{
                        test: /\.js$/,
                        use: ['thread-loader', 'babel-loader']
                    }]
                }
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_build_caching() {
        let rule = RequireBuildCaching;

        // Test webpack config without cache
        let code_violation = r#"
            module.exports = {
                mode: 'development'
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper cache configuration
        let code_compliant = r#"
            module.exports = {
                cache: {
                    type: 'filesystem'
                }
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement() {
        let rule = RequireTreeShaking;
        let diagnostics = vec![WasmRuleDiagnostic {
            rule_name: "require-tree-shaking".to_string(),
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