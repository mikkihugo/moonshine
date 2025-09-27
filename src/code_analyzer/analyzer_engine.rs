//! Main AST analyzer engine using OXC for TypeScript/JavaScript analysis
//!
//! Self-documenting analyzer that provides semantic analysis, complexity metrics,
//! and automated fixes with high performance and accuracy.

use crate::code_analyzer::{analysis_types::*, complexity_metrics::*, config_types::*, security_analysis::*};
use crate::error::{Error, Result};
use crate::moon_pdk_interface::{get_moon_config_safe, write_file_atomic};

use dashmap::DashMap;
use glob::Pattern;
use ignore::WalkBuilder;
use lru::LruCache;
use oxc_allocator::Allocator;
use oxc_diagnostics::{
    reporter::{DiagnosticReporter, DiagnosticResult},
    DiagnosticService,
};
use oxc_resolver::{ResolveOptions, Resolver};
use parking_lot::RwLock;
use petgraph::Graph;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
struct NoopDiagnosticReporter;

impl DiagnosticReporter for NoopDiagnosticReporter {
    fn finish(&mut self, _result: &DiagnosticResult) -> Option<String> {
        None
    }

    fn render_error(&mut self, _error: oxc_diagnostics::Error) -> Option<String> {
        None
    }
}

/// Production-grade AST auto-fix engine using complete OXC toolchain
pub struct AstAutoFixEngine {
    config: AstAutoFixConfig,
    resolver: Resolver,
    diagnostic_service: DiagnosticService,
    eslint_config: Option<EslintConfig>,
    ignore_matcher: ignore::gitignore::Gitignore,

    // Caching for performance
    complexity_cache: RwLock<LruCache<String, ComplexityMetrics>>,
    analysis_cache: DashMap<String, AstAutoFixResult>,
    dependency_graph: RwLock<Graph<String, String>>,
}

impl AstAutoFixEngine {
    /// Create new AST auto-fix engine with OXC toolchain, ESLint config, and .gitignore support
    pub fn new() -> Result<Self> {
        let config = Self::load_config_from_moon()?;

        // Initialize OXC resolver for module analysis
        let resolver = Resolver::new(ResolveOptions {
            extensions: vec![".ts".into(), ".tsx".into(), ".js".into(), ".jsx".into()],
            main_fields: vec!["types".into(), "module".into(), "main".into()],
            condition_names: vec!["types".into(), "import".into(), "require".into()],
            ..Default::default()
        });

        let (diagnostic_service, _diagnostic_sender) = DiagnosticService::new(Box::new(NoopDiagnosticReporter::default()));

        // Load ESLint configuration from project
        let eslint_config = Self::load_eslint_config().ok();

        // Initialize ignore matcher for .gitignore and other ignore patterns
        let ignore_matcher = Self::build_ignore_matcher()?;

        // Initialize caches for performance
        let complexity_cache = RwLock::new(LruCache::new(std::num::NonZeroUsize::new(1000).unwrap()));
        let analysis_cache = DashMap::new();
        let dependency_graph = RwLock::new(Graph::new());

        Ok(Self {
            config,
            resolver,
            diagnostic_service,
            eslint_config,
            ignore_matcher,
            complexity_cache,
            analysis_cache,
            dependency_graph,
        })
    }

    /// Load configuration from Moon workspace
    fn load_config_from_moon() -> Result<AstAutoFixConfig> {
        if let Ok(Some(config_str)) = get_moon_config_safe("moon-shine.analyzer") {
            serde_json::from_str(&config_str).map_err(|e| Error::config(format!("Invalid analyzer config: {}", e)))
        } else {
            Ok(AstAutoFixConfig::default())
        }
    }

    /// Load ESLint configuration from project files (.eslintrc.json, .eslintrc.yml, etc.)
    fn load_eslint_config() -> Result<EslintConfig> {
        // Try common ESLint config file locations
        let config_files = [
            ".eslintrc.json",
            ".eslintrc.yml",
            ".eslintrc.yaml",
            ".eslintrc.js",
            "eslint.config.js",
            "package.json", // eslintConfig field
        ];

        for config_file in &config_files {
            if let Ok(Some(config_content)) = get_moon_config_safe(&format!("file_content:{}", config_file)) {
                if let Ok(eslint_config) = Self::parse_eslint_config(&config_content, config_file) {
                    return Ok(eslint_config);
                }
            }
        }

        // Return default ESLint config if none found
        Ok(EslintConfig {
            extends: vec!["@typescript-eslint/recommended".to_string()],
            rules: HashMap::new(),
            parser_options: EslintParserOptions {
                ecma_version: Some(2022),
                source_type: Some("module".to_string()),
                ecma_features: HashMap::new(),
            },
            env: HashMap::from([("es2022".to_string(), true), ("node".to_string(), true)]),
            globals: HashMap::new(),
        })
    }

    /// Parse ESLint configuration from file content
    fn parse_eslint_config(content: &str, filename: &str) -> Result<EslintConfig> {
        match filename {
            f if f.ends_with(".json") => serde_json::from_str(content).map_err(|e| Error::config(format!("Invalid ESLint JSON config: {}", e))),
            f if f.ends_with(".yml") || f.ends_with(".yaml") => {
                serde_yaml::from_str(content).map_err(|e| Error::config(format!("Invalid ESLint YAML config: {}", e)))
            }
            "package.json" => {
                let package_json: serde_json::Value = serde_json::from_str(content).map_err(|e| Error::config(format!("Invalid package.json: {}", e)))?;

                if let Some(eslint_config) = package_json.get("eslintConfig") {
                    serde_json::from_value(eslint_config.clone()).map_err(|e| Error::config(format!("Invalid ESLint config in package.json: {}", e)))
                } else {
                    Err(Error::config("No eslintConfig found in package.json".to_string()))
                }
            }
            _ => {
                // For .js files, we'd need to execute them, which is complex in WASM
                // For now, return a default config
                Err(Error::config("JavaScript ESLint configs not supported in WASM".to_string()))
            }
        }
    }

    /// Build ignore matcher from .gitignore and other ignore files
    fn build_ignore_matcher() -> Result<ignore::gitignore::Gitignore> {
        let mut builder = ignore::gitignore::GitignoreBuilder::new(".");

        // Add common ignore patterns
        let ignore_files = [
            ".gitignore",
            ".eslintignore",
            ".prettierignore",
            ".moonignore", // Custom ignore file for moon-shine
        ];

        for ignore_file in &ignore_files {
            if let Ok(Some(content)) = get_moon_config_safe(&format!("file_content:{}", ignore_file)) {
                builder
                    .add_line(None, &content)
                    .map_err(|e| Error::config(format!("Invalid ignore pattern in {}: {}", ignore_file, e)))?;
            }
        }

        // Add default ignore patterns for common build artifacts
        let default_patterns = ["node_modules/", "dist/", "build/", ".moon/"];
        for pattern in &default_patterns {
            builder
                .add_line(None, pattern)
                .map_err(|e| Error::config(format!("Failed to add default ignore pattern: {}", e)))?;
        }

        builder.build().map_err(|e| Error::config(format!("Failed to build ignore matcher: {}", e)))
    }

    /// Discover files to process respecting .gitignore and ESLint ignore patterns
    pub fn discover_files(&self, root_path: &str, patterns: &[&str]) -> Result<Vec<String>> {
        let mut files = Vec::new();

        // Use the ignore crate to efficiently walk the directory tree
        let walker = WalkBuilder::new(root_path)
            .ignore(true) // Respect .gitignore
            .git_ignore(true) // Respect git ignore files
            .git_exclude(true) // Respect git exclude files
            .hidden(false) // Include hidden files for now
            .build();

        for result in walker {
            match result {
                Ok(entry) => {
                    let path = entry.path();

                    // Only process files (not directories)
                    if path.is_file() {
                        let path_str = path.to_string_lossy();

                        // Check if file matches any of the provided patterns
                        let matches_pattern = patterns.is_empty()
                            || patterns.iter().any(|pattern| {
                                if let Ok(glob_pattern) = Pattern::new(pattern) {
                                    glob_pattern.matches(&path_str)
                                } else {
                                    // Fallback to simple extension check
                                    path_str.ends_with(pattern)
                                }
                            });

                        // Check if file should be ignored
                        let is_ignored = self.ignore_matcher.matched(path, false).is_ignore();

                        if matches_pattern && !is_ignored {
                            files.push(path_str.to_string());
                        }
                    }
                }
                Err(err) => {
                    // Log error but continue processing
                    eprintln!("Warning: Failed to read directory entry: {}", err);
                }
            }
        }

        Ok(files)
    }

    /// Get cached analysis result or return None if not found
    pub fn get_cached_result(&self, file_path: &str) -> Option<AstAutoFixResult> {
        self.analysis_cache.get(file_path).map(|entry| entry.clone())
    }

    /// Cache analysis result for future use
    pub fn cache_result(&self, file_path: String, result: AstAutoFixResult) {
        self.analysis_cache.insert(file_path, result);
    }

    /// Clear all caches (useful for testing or memory management)
    pub fn clear_caches(&self) {
        self.analysis_cache.clear();
        if let Ok(mut cache) = self.complexity_cache.write() {
            cache.clear();
        }
        if let Ok(mut graph) = self.dependency_graph.write() {
            graph.clear();
        }
    }
}
