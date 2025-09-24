//! # Multi-Language Analysis System
//!
//! Unified analysis system supporting both TypeScript/JavaScript (via OXC + StarCoder)
//! and Rust (via Clippy + full Rust toolchain) with AI-powered pattern detection.

use crate::oxc_adapter::{CodePatternDetector, LanguageModelConfig, MultiEngineAnalyzer, MultiEngineConfig, PatternAnalysisResult, RepetitivePatternLearner};
use crate::types::LintDiagnostic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Supported programming languages for analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedLanguage {
    /// TypeScript (.ts, .tsx files)
    TypeScript,
    /// JavaScript (.js, .jsx, .mjs, .cjs files)
    JavaScript,
    /// Rust (.rs files)
    Rust,
}

/// Language-specific analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Language type
    pub language: SupportedLanguage,
    /// Whether this language is enabled for analysis
    pub enabled: bool,
    /// Language-specific analyzer configuration
    pub analyzer_config: LanguageAnalyzerConfig,
    /// AI pattern detection configuration
    pub pattern_detection: ArtificialIntelligencePatternDetectionConfig,
}

/// Language-specific analyzer configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LanguageAnalyzerConfig {
    /// TypeScript/JavaScript configuration using OXC
    TypeScriptJavaScript {
        /// OXC multi-engine configuration
        oxc_config: MultiEngineConfig,
        /// Enable type checking for TypeScript
        enable_type_checking: bool,
        /// ESLint compatibility mode
        eslint_compat: bool,
    },
    /// Rust configuration using Clippy and toolchain
    Rust {
        /// Clippy lints configuration
        clippy_config: ClippyConfig,
        /// Rustfmt configuration
        rustfmt_config: RustfmtConfig,
        /// Cargo test configuration
        test_config: CargoTestConfig,
        /// Enable experimental lints
        enable_experimental: bool,
    },
}

/// Clippy configuration for Rust analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClippyConfig {
    /// Clippy lint level (allow, warn, deny, forbid)
    pub lint_level: String,
    /// Specific lints to enable/disable
    pub lints: HashMap<String, String>,
    /// Enable all lints (clippy::all)
    pub enable_all: bool,
    /// Enable pedantic lints (clippy::pedantic)
    pub enable_pedantic: bool,
    /// Enable nursery lints (clippy::nursery)
    pub enable_nursery: bool,
    /// Enable cargo lints (clippy::cargo)
    pub enable_cargo: bool,
    /// Additional clippy arguments
    pub extra_args: Vec<String>,
}

/// Rustfmt configuration for Rust formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustfmtConfig {
    /// Enable rustfmt
    pub enabled: bool,
    /// Rustfmt edition (2015, 2018, 2021)
    pub edition: String,
    /// Maximum line width
    pub max_width: u32,
    /// Hard tabs vs spaces
    pub hard_tabs: bool,
    /// Tab spaces
    pub tab_spaces: u8,
    /// Custom rustfmt.toml path
    pub config_path: Option<PathBuf>,
}

/// Cargo test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTestConfig {
    /// Enable cargo test integration
    pub enabled: bool,
    /// Run tests in parallel
    pub parallel: bool,
    /// Test timeout in seconds
    pub timeout_seconds: u64,
    /// Additional cargo test arguments
    pub extra_args: Vec<String>,
    /// Enable doctest
    pub enable_doctests: bool,
    /// Test workspace members
    pub workspace: bool,
}

/// Configuration for AI-powered pattern detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtificialIntelligencePatternDetectionConfig {
    /// Enable AI pattern detection for this language
    pub enabled: bool,
    /// Language model configuration
    pub language_model_config: LanguageModelConfig,
    /// Pattern frequency threshold for rule generation
    pub frequency_threshold: u32,
    /// Focus on specific pattern types
    pub pattern_types: Vec<String>,
}

/// Multi-language analysis result
#[derive(Debug)]
pub struct MultiLanguageAnalysisResult {
    /// Results per language
    pub language_results: HashMap<SupportedLanguage, LanguageAnalysisResult>,
    /// Cross-language pattern analysis
    pub cross_language_patterns: Vec<CrossLanguagePattern>,
    /// Overall analysis statistics
    pub stats: MultiLanguageStats,
}

/// Analysis result for a specific language
#[derive(Debug)]
pub struct LanguageAnalysisResult {
    /// Language analyzed
    pub language: SupportedLanguage,
    /// All diagnostics found
    pub diagnostics: Vec<LintDiagnostic>,
    /// AI pattern analysis results
    pub pattern_analysis: Option<PatternAnalysisResult>,
    /// Language-specific metrics
    pub metrics: LanguageMetrics,
    /// Files analyzed
    pub files_analyzed: Vec<String>,
}

/// Cross-language pattern detected across multiple languages
#[derive(Debug)]
pub struct CrossLanguagePattern {
    /// Pattern description
    pub description: String,
    /// Languages where this pattern appears
    pub languages: Vec<SupportedLanguage>,
    /// Pattern frequency across languages
    pub frequency: HashMap<SupportedLanguage, u32>,
    /// Suggested unified rule
    pub suggested_rule: Option<String>,
}

/// Performance metrics for language analysis
#[derive(Debug, Default)]
pub struct LanguageMetrics {
    /// Analysis duration in milliseconds
    pub analysis_duration_ms: u64,
    /// Number of files analyzed
    pub files_count: usize,
    /// Lines of code analyzed
    pub lines_analyzed: usize,
    /// Issues found
    pub issues_found: usize,
    /// Auto-fixable issues
    pub auto_fixable: usize,
}

/// Overall analysis statistics
#[derive(Debug, Default)]
pub struct MultiLanguageStats {
    /// Total analysis time
    pub total_duration_ms: u64,
    /// Files analyzed per language
    pub files_per_language: HashMap<SupportedLanguage, usize>,
    /// Issues per language
    pub issues_per_language: HashMap<SupportedLanguage, usize>,
    /// Cross-language patterns found
    pub cross_language_patterns_count: usize,
}

/// Multi-language analyzer coordinating all language-specific analyzers
pub struct MultiLanguageAnalyzer {
    /// Language configurations
    configs: HashMap<SupportedLanguage, LanguageConfig>,
    /// TypeScript/JavaScript analyzer
    js_ts_analyzer: Option<MultiEngineAnalyzer>,
    /// Rust analyzer components
    rust_analyzer: Option<RustAnalyzer>,
    /// AI pattern detectors per language
    pattern_detectors: HashMap<SupportedLanguage, CodePatternDetector>,
    /// Pattern learning system for cross-language patterns
    pattern_learner: RepetitivePatternLearner,
}

/// Rust-specific analyzer using Clippy and toolchain
pub struct RustAnalyzer {
    /// Clippy configuration
    clippy_config: ClippyConfig,
    /// Rustfmt configuration
    rustfmt_config: RustfmtConfig,
    /// Cargo test configuration
    test_config: CargoTestConfig,
    /// Cargo.toml path for workspace detection
    cargo_toml_path: Option<PathBuf>,
}

impl MultiLanguageAnalyzer {
    /// Create new multi-language analyzer
    pub fn new() -> Self {
        let mut configs = HashMap::new();

        // Default TypeScript/JavaScript configuration
        configs.insert(
            SupportedLanguage::TypeScript,
            LanguageConfig {
                language: SupportedLanguage::TypeScript,
                enabled: true,
                analyzer_config: LanguageAnalyzerConfig::TypeScriptJavaScript {
                    oxc_config: MultiEngineConfig::default(),
                    enable_type_checking: true,
                    eslint_compat: true,
                },
                pattern_detection: ArtificialIntelligencePatternDetectionConfig {
                    enabled: true,
                    language_model_config: LanguageModelConfig::default(),
                    frequency_threshold: 3,
                    pattern_types: vec!["ai_mistakes".to_string(), "type_errors".to_string(), "performance_issues".to_string()],
                },
            },
        );

        // Default JavaScript configuration
        configs.insert(
            SupportedLanguage::JavaScript,
            LanguageConfig {
                language: SupportedLanguage::JavaScript,
                enabled: true,
                analyzer_config: LanguageAnalyzerConfig::TypeScriptJavaScript {
                    oxc_config: MultiEngineConfig::default(),
                    enable_type_checking: false,
                    eslint_compat: true,
                },
                pattern_detection: ArtificialIntelligencePatternDetectionConfig {
                    enabled: true,
                    language_model_config: LanguageModelConfig::default(),
                    frequency_threshold: 3,
                    pattern_types: vec!["ai_mistakes".to_string(), "common_errors".to_string(), "performance_issues".to_string()],
                },
            },
        );

        // Default Rust configuration
        configs.insert(
            SupportedLanguage::Rust,
            LanguageConfig {
                language: SupportedLanguage::Rust,
                enabled: true,
                analyzer_config: LanguageAnalyzerConfig::Rust {
                    clippy_config: ClippyConfig::default(),
                    rustfmt_config: RustfmtConfig::default(),
                    test_config: CargoTestConfig::default(),
                    enable_experimental: false,
                },
                pattern_detection: ArtificialIntelligencePatternDetectionConfig {
                    enabled: true,
                    language_model_config: LanguageModelConfig::default(),
                    frequency_threshold: 3,
                    pattern_types: vec![
                        "ownership_issues".to_string(),
                        "performance_patterns".to_string(),
                        "safety_violations".to_string(),
                    ],
                },
            },
        );

        Self {
            configs,
            js_ts_analyzer: None,
            rust_analyzer: None,
            pattern_detectors: HashMap::new(),
            pattern_learner: RepetitivePatternLearner::new(crate::oxc_adapter::adaptive_pattern_analyzer::PatternLearningConfig::default()),
        }
    }

    /// Initialize analyzers for enabled languages
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Initializing multi-language analyzer");

        // Initialize TypeScript/JavaScript analyzer
        if self.is_language_enabled(&SupportedLanguage::TypeScript) || self.is_language_enabled(&SupportedLanguage::JavaScript) {
            let config = self.get_oxc_config();
            self.js_ts_analyzer = Some(MultiEngineAnalyzer::with_config(config));
            log::info!("TypeScript/JavaScript analyzer initialized");
        }

        // Initialize Rust analyzer
        if self.is_language_enabled(&SupportedLanguage::Rust) {
            let rust_config = self.get_rust_config();
            self.rust_analyzer = Some(RustAnalyzer::new(rust_config).await?);
            log::info!("Rust analyzer initialized");
        }

        // Initialize AI pattern detectors
        for (language, config) in &self.configs {
            if config.enabled && config.pattern_detection.enabled {
                let mut detector = CodePatternDetector::new(config.pattern_detection.language_model_config.clone());
                detector.load_model().await?;
                self.pattern_detectors.insert(language.clone(), detector);
                log::info!("Pattern detector initialized for {:?}", language);
            }
        }

        Ok(())
    }

    /// Analyze files across multiple languages
    pub async fn analyze_files(&mut self, file_paths: &[String]) -> Result<MultiLanguageAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut language_results = HashMap::new();
        let mut stats = MultiLanguageStats::default();

        // Group files by language
        let files_by_language = self.group_files_by_language(file_paths);

        // Analyze each language
        for (language, files) in files_by_language {
            if !self.is_language_enabled(&language) {
                continue;
            }

            log::info!("Analyzing {} files for {:?}", files.len(), language);
            let language_start = std::time::Instant::now();

            let result = match language {
                SupportedLanguage::TypeScript | SupportedLanguage::JavaScript => self.analyze_js_ts_files(&files).await?,
                SupportedLanguage::Rust => self.analyze_rust_files(&files).await?,
            };

            let duration = language_start.elapsed().as_millis() as u64;
            stats.files_per_language.insert(language.clone(), files.len());
            stats.issues_per_language.insert(language.clone(), result.diagnostics.len());

            language_results.insert(language, result);
        }

        // Detect cross-language patterns
        let cross_language_patterns = self.detect_cross_language_patterns(&language_results).await?;
        stats.cross_language_patterns_count = cross_language_patterns.len();

        stats.total_duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(MultiLanguageAnalysisResult {
            language_results,
            cross_language_patterns,
            stats,
        })
    }

    /// Analyze TypeScript/JavaScript files
    async fn analyze_js_ts_files(&mut self, files: &[String]) -> Result<LanguageAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut all_diagnostics = Vec::new();
        let mut lines_analyzed = 0;

        if let Some(analyzer) = &mut self.js_ts_analyzer {
            for file_path in files {
                log::debug!("Analyzing JS/TS file: {}", file_path);

                // Read file content
                let source_code = std::fs::read_to_string(file_path)?;
                lines_analyzed += source_code.lines().count();

                // Run OXC + AI analysis
                let result = analyzer.analyze_code(&source_code, file_path).await?;
                all_diagnostics.extend(result.diagnostics);

                // Run AI pattern detection if enabled
                if let Some(detector) = self.pattern_detectors.get_mut(&SupportedLanguage::TypeScript) {
                    let patterns = detector
                        .detect_code_patterns(
                            &source_code,
                            file_path,
                            &[crate::oxc_adapter::starcoder_integration::CodePatternType::ArtificialIntelligenceGeneratedCodeIssues],
                        )
                        .await?;

                    // Convert patterns to diagnostics
                    for pattern in patterns {
                        all_diagnostics.push(LintDiagnostic {
                            rule_name: format!("ai-pattern:{}", pattern.pattern_type),
                            message: pattern
                                .suggested_fix
                                .unwrap_or_else(|| format!("AI detected pattern: {}", pattern.pattern_type)),
                            file_path: file_path.clone(),
                            line: pattern.line_number,
                            column: 1,
                            end_line: pattern.line_number,
                            end_column: 1,
                            fix_available: false,
                            severity: crate::types::DiagnosticSeverity::Warning,
                            suggested_fix: None,
                        });
                    }
                }
            }
        }

        let duration = start_time.elapsed().as_millis() as u64;
        let auto_fixable = all_diagnostics.iter().filter(|d| d.suggested_fix.is_some()).count();
        let issues_found = all_diagnostics.len();

        Ok(LanguageAnalysisResult {
            language: SupportedLanguage::TypeScript, // Will be corrected based on file
            diagnostics: all_diagnostics,
            pattern_analysis: None, // TODO: Implement pattern analysis
            metrics: LanguageMetrics {
                analysis_duration_ms: duration,
                files_count: files.len(),
                lines_analyzed,
                issues_found,
                auto_fixable,
            },
            files_analyzed: files.to_vec(),
        })
    }

    /// Analyze Rust files
    async fn analyze_rust_files(&mut self, files: &[String]) -> Result<LanguageAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut all_diagnostics = Vec::new();
        let mut lines_analyzed = 0;

        if let Some(rust_analyzer) = &mut self.rust_analyzer {
            // Run Clippy analysis
            let clippy_diagnostics = rust_analyzer.run_clippy(files).await?;
            all_diagnostics.extend(clippy_diagnostics);

            // Run rustfmt check
            let fmt_diagnostics = rust_analyzer.run_rustfmt_check(files).await?;
            all_diagnostics.extend(fmt_diagnostics);

            // Run cargo test if enabled
            if rust_analyzer.test_config.enabled {
                let test_diagnostics = rust_analyzer.run_cargo_test().await?;
                all_diagnostics.extend(test_diagnostics);
            }

            // Count lines analyzed
            for file_path in files {
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    lines_analyzed += content.lines().count();
                }
            }

            // Run AI pattern detection for Rust-specific patterns
            if let Some(detector) = self.pattern_detectors.get_mut(&SupportedLanguage::Rust) {
                for file_path in files {
                    if let Ok(source_code) = std::fs::read_to_string(file_path) {
                        let patterns = detector
                            .detect_code_patterns(
                                &source_code,
                                file_path,
                                &[crate::oxc_adapter::starcoder_integration::CodePatternType::PerformanceAntiPatterns],
                            )
                            .await?;

                        // Convert to diagnostics
                        for pattern in patterns {
                            all_diagnostics.push(LintDiagnostic {
                                rule_name: format!("rust-ai-pattern:{}", pattern.pattern_type),
                                message: pattern.suggested_fix.unwrap_or_else(|| format!("Rust AI pattern: {}", pattern.pattern_type)),
                                file_path: file_path.clone(),
                                line: pattern.line_number,
                                column: 1,
                                end_line: pattern.line_number,
                                end_column: 1,
                                fix_available: false,
                                severity: crate::types::DiagnosticSeverity::Warning,
                                suggested_fix: None,
                            });
                        }
                    }
                }
            }
        }

        let duration = start_time.elapsed().as_millis() as u64;
        let auto_fixable = all_diagnostics.iter().filter(|d| d.suggested_fix.is_some()).count();
        let issues_found = all_diagnostics.len();

        Ok(LanguageAnalysisResult {
            language: SupportedLanguage::Rust,
            diagnostics: all_diagnostics,
            pattern_analysis: None,
            metrics: LanguageMetrics {
                analysis_duration_ms: duration,
                files_count: files.len(),
                lines_analyzed,
                issues_found,
                auto_fixable,
            },
            files_analyzed: files.to_vec(),
        })
    }

    /// Detect patterns that appear across multiple languages
    async fn detect_cross_language_patterns(
        &self,
        language_results: &HashMap<SupportedLanguage, LanguageAnalysisResult>,
    ) -> Result<Vec<CrossLanguagePattern>, Box<dyn std::error::Error>> {
        let mut cross_patterns = Vec::new();

        // Look for similar patterns across languages
        let mut pattern_counts: HashMap<String, HashMap<SupportedLanguage, u32>> = HashMap::new();

        for (language, result) in language_results {
            for diagnostic in &result.diagnostics {
                // Extract pattern type from rule name
                let pattern_type = self.extract_pattern_type(&diagnostic.rule_name);

                pattern_counts
                    .entry(pattern_type)
                    .or_insert_with(HashMap::new)
                    .entry(language.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        // Identify patterns that appear in multiple languages
        for (pattern_type, language_counts) in pattern_counts {
            if language_counts.len() > 1 {
                // Pattern appears in multiple languages
                let languages: Vec<SupportedLanguage> = language_counts.keys().cloned().collect();

                cross_patterns.push(CrossLanguagePattern {
                    description: format!("Cross-language pattern: {}", pattern_type),
                    languages,
                    frequency: language_counts,
                    suggested_rule: Some(format!("unified-{}", pattern_type)),
                });
            }
        }

        Ok(cross_patterns)
    }

    /// Group files by programming language based on file extension
    fn group_files_by_language(&self, files: &[String]) -> HashMap<SupportedLanguage, Vec<String>> {
        let mut grouped = HashMap::new();

        for file_path in files {
            if let Some(language) = self.detect_language(file_path) {
                grouped.entry(language).or_insert_with(Vec::new).push(file_path.clone());
            }
        }

        grouped
    }

    /// Detect programming language from file extension
    fn detect_language(&self, file_path: &str) -> Option<SupportedLanguage> {
        let path = Path::new(file_path);
        let extension = path.extension()?.to_str()?;

        match extension {
            "ts" | "tsx" => Some(SupportedLanguage::TypeScript),
            "js" | "jsx" | "mjs" | "cjs" => Some(SupportedLanguage::JavaScript),
            "rs" => Some(SupportedLanguage::Rust),
            _ => None,
        }
    }

    /// Check if a language is enabled for analysis
    fn is_language_enabled(&self, language: &SupportedLanguage) -> bool {
        self.configs.get(language).map(|config| config.enabled).unwrap_or(false)
    }

    /// Get OXC configuration for TypeScript/JavaScript
    fn get_oxc_config(&self) -> MultiEngineConfig {
        // Get from TypeScript config (JavaScript uses similar config)
        if let Some(config) = self.configs.get(&SupportedLanguage::TypeScript) {
            if let LanguageAnalyzerConfig::TypeScriptJavaScript { oxc_config, .. } = &config.analyzer_config {
                return oxc_config.clone();
            }
        }
        MultiEngineConfig::default()
    }

    /// Get Rust analyzer configuration
    fn get_rust_config(&self) -> (ClippyConfig, RustfmtConfig, CargoTestConfig) {
        if let Some(config) = self.configs.get(&SupportedLanguage::Rust) {
            if let LanguageAnalyzerConfig::Rust {
                clippy_config,
                rustfmt_config,
                test_config,
                ..
            } = &config.analyzer_config
            {
                return (clippy_config.clone(), rustfmt_config.clone(), test_config.clone());
            }
        }
        (ClippyConfig::default(), RustfmtConfig::default(), CargoTestConfig::default())
    }

    /// Extract pattern type from rule name for cross-language analysis
    fn extract_pattern_type(&self, rule_name: &str) -> String {
        // Extract base pattern from rule names like "ai-pattern:unused_variable" -> "unused_variable"
        if let Some(colon_pos) = rule_name.find(':') {
            rule_name[colon_pos + 1..].to_string()
        } else {
            rule_name.to_string()
        }
    }

    /// Update language configuration
    pub fn update_language_config(&mut self, language: SupportedLanguage, config: LanguageConfig) {
        self.configs.insert(language, config);
    }

    /// Get analysis statistics
    pub fn get_stats(&self) -> HashMap<SupportedLanguage, String> {
        let mut stats = HashMap::new();

        for (language, config) in &self.configs {
            let status = if config.enabled { "Enabled" } else { "Disabled" };
            stats.insert(language.clone(), status.to_string());
        }

        stats
    }
}

impl Default for ClippyConfig {
    fn default() -> Self {
        Self {
            lint_level: "warn".to_string(),
            lints: HashMap::new(),
            enable_all: true,
            enable_pedantic: false,
            enable_nursery: false,
            enable_cargo: true,
            extra_args: vec!["--".to_string(), "-D".to_string(), "warnings".to_string()],
        }
    }
}

impl Default for RustfmtConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            edition: "2021".to_string(),
            max_width: 100,
            hard_tabs: false,
            tab_spaces: 4,
            config_path: None,
        }
    }
}

impl Default for CargoTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            parallel: true,
            timeout_seconds: 60,
            extra_args: vec!["--".to_string(), "--nocapture".to_string()],
            enable_doctests: true,
            workspace: true,
        }
    }
}

impl RustAnalyzer {
    /// Create new Rust analyzer
    async fn new((clippy_config, rustfmt_config, test_config): (ClippyConfig, RustfmtConfig, CargoTestConfig)) -> Result<Self, Box<dyn std::error::Error>> {
        // Detect Cargo.toml in workspace
        let cargo_toml_path = Self::find_cargo_toml().await?;

        Ok(Self {
            clippy_config,
            rustfmt_config,
            test_config,
            cargo_toml_path,
        })
    }

    /// Find Cargo.toml file in current directory or parent directories
    async fn find_cargo_toml() -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let mut dir = current_dir.as_path();

        loop {
            let cargo_toml = dir.join("Cargo.toml");
            if cargo_toml.exists() {
                return Ok(Some(cargo_toml));
            }

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                break;
            }
        }

        Ok(None)
    }

    /// Run Clippy analysis on Rust files
    async fn run_clippy(&self, _files: &[String]) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        log::info!("Running Clippy analysis");

        // TODO: Implement actual Clippy execution
        // This would run: cargo clippy --all-targets --all-features -- -D warnings

        Ok(Vec::new())
    }

    /// Run rustfmt check on Rust files
    async fn run_rustfmt_check(&self, _files: &[String]) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        if !self.rustfmt_config.enabled {
            return Ok(Vec::new());
        }

        log::info!("Running rustfmt check");

        // TODO: Implement rustfmt check
        // This would run: cargo fmt --all -- --check

        Ok(Vec::new())
    }

    /// Run cargo test
    async fn run_cargo_test(&self) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        if !self.test_config.enabled {
            return Ok(Vec::new());
        }

        log::info!("Running cargo test");

        // TODO: Implement cargo test execution
        // This would run: cargo test --workspace

        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        let analyzer = MultiLanguageAnalyzer::new();

        assert_eq!(analyzer.detect_language("test.ts"), Some(SupportedLanguage::TypeScript));
        assert_eq!(analyzer.detect_language("test.tsx"), Some(SupportedLanguage::TypeScript));
        assert_eq!(analyzer.detect_language("test.js"), Some(SupportedLanguage::JavaScript));
        assert_eq!(analyzer.detect_language("test.jsx"), Some(SupportedLanguage::JavaScript));
        assert_eq!(analyzer.detect_language("test.rs"), Some(SupportedLanguage::Rust));
        assert_eq!(analyzer.detect_language("test.py"), None);
    }

    #[test]
    fn test_file_grouping() {
        let analyzer = MultiLanguageAnalyzer::new();
        let files = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "client/app.ts".to_string(),
            "client/component.tsx".to_string(),
            "scripts/build.js".to_string(),
        ];

        let grouped = analyzer.group_files_by_language(&files);

        assert_eq!(grouped[&SupportedLanguage::Rust].len(), 2);
        assert_eq!(grouped[&SupportedLanguage::TypeScript].len(), 2);
        assert_eq!(grouped[&SupportedLanguage::JavaScript].len(), 1);
    }

    #[test]
    fn test_pattern_extraction() {
        let analyzer = MultiLanguageAnalyzer::new();

        assert_eq!(analyzer.extract_pattern_type("ai-pattern:unused_variable"), "unused_variable");
        assert_eq!(analyzer.extract_pattern_type("rust-ai-pattern:ownership_issue"), "ownership_issue");
        assert_eq!(analyzer.extract_pattern_type("no-unused-vars"), "no-unused-vars");
    }
}
