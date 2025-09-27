//! # Multi-Engine Analysis System
//!
//! Configurable analysis system that runs OXC linting with AI behavioral pattern detection,
//! all running directly in WASM without external processes.

use super::ai_behavioral::{AiBehavioralAnalyzer, AnalysisContext};
use super::oxc_formatter::{OxcFormatter, OxcFormatterConfig};
use super::oxc_linter::{OxcConfig, OxcLinter};
use super::oxc_transformer::{OxcTransformer, OxcTransformerConfig};
use crate::types::LintDiagnostic;
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};

/// Analysis result from multiple engines
#[derive(Debug)]
pub struct MultiEngineAnalysisResult {
    /// All diagnostics from all enabled engines
    pub diagnostics: Vec<LintDiagnostic>,
    /// Formatted code (if formatting was requested)
    pub formatted_code: Option<String>,
    /// Transformed code (if transformation was requested)
    pub transformed_code: Option<String>,
    /// OXC AST for additional processing
    pub ast_program: Option<Program<'static>>,
    /// Analysis statistics per engine
    pub stats: AnalysisStats,
}

/// Performance statistics for analysis engines
#[derive(Debug, Default)]
pub struct AnalysisStats {
    pub oxc_rules_executed: usize,
    pub ai_patterns_checked: usize,
    pub total_duration_ms: u64,
    pub oxc_duration_ms: u64,
    pub ai_duration_ms: u64,
    pub formatting_duration_ms: u64,
    pub transformation_duration_ms: u64,
    pub source_unchanged: bool,
    pub compression_ratio: Option<f32>,
}

/// Configuration for the multi-engine analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEngineConfig {
    // OXC configuration
    pub enable_oxc: bool,
    pub oxc_config: Option<OxcConfig>,

    // AI behavioral analysis
    pub enable_ai_behavioral: bool,
    pub ai_confidence_threshold: f32,

    // OXC Formatter (Beta)
    pub enable_formatting: bool,
    pub formatter_config: Option<OxcFormatterConfig>,

    // OXC Transformer
    pub enable_transformation: bool,
    pub transformer_config: Option<OxcTransformerConfig>,

    // Performance settings
    pub max_file_size_kb: Option<u32>,
    pub timeout_ms: Option<u32>,

    // Output control
    pub preserve_ast: bool,
}

impl Default for MultiEngineConfig {
    fn default() -> Self {
        Self {
            enable_oxc: true,
            oxc_config: Some(OxcConfig::default()),
            enable_ai_behavioral: true,
            ai_confidence_threshold: 0.8,
            enable_formatting: false, // Disabled by default (beta)
            formatter_config: Some(OxcFormatterConfig::default()),
            enable_transformation: false, // Disabled by default
            transformer_config: Some(OxcTransformerConfig::default()),
            max_file_size_kb: Some(1024), // 1MB limit
            timeout_ms: Some(5000),       // 5 second timeout
            preserve_ast: false,
        }
    }
}

/// Multi-engine analyzer using OXC + AI
pub struct MultiEngineAnalyzer {
    config: MultiEngineConfig,
    oxc_linter: Option<OxcLinter>,
    ai_analyzer: Option<AiBehavioralAnalyzer>,
    formatter: Option<OxcFormatter>,
    transformer: Option<OxcTransformer>,
    allocator: Allocator,
}

impl MultiEngineAnalyzer {
    /// Create analyzer with default configuration
    pub fn new() -> Self {
        Self::with_config(MultiEngineConfig::default())
    }

    /// Create analyzer with custom configuration
    pub fn with_config(config: MultiEngineConfig) -> Self {
        let oxc_linter = if config.enable_oxc {
            config.oxc_config.as_ref().map(|oxc_config| OxcLinter::new(oxc_config.clone()))
        } else {
            None
        };

        let ai_analyzer = if config.enable_ai_behavioral {
            Some(AiBehavioralAnalyzer::new())
        } else {
            None
        };

        let formatter = if config.enable_formatting {
            config
                .formatter_config
                .as_ref()
                .map(|formatter_config| OxcFormatter::new(formatter_config.clone()))
        } else {
            None
        };

        let transformer = if config.enable_transformation {
            config
                .transformer_config
                .as_ref()
                .map(|transformer_config| OxcTransformer::new(transformer_config.clone()))
        } else {
            None
        };

        Self {
            config,
            oxc_linter,
            ai_analyzer,
            formatter,
            transformer,
            allocator: Allocator::default(),
        }
    }

    /// Analyze code with all enabled engines
    pub async fn analyze_code(&mut self, source_code: &str, file_path: &str) -> Result<MultiEngineAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut diagnostics = Vec::new();
        let mut stats = AnalysisStats::default();

        // Check file size limits
        if let Some(max_kb) = self.config.max_file_size_kb {
            let size_kb = source_code.len() / 1024;
            if size_kb > max_kb as usize {
                return Err(format!("File too large: {}KB > {}KB", size_kb, max_kb).into());
            }
        }

        // Parse with OXC for AST generation
        let source_type = self.detect_source_type(file_path);
        let ParserReturn { program, errors, .. } = Parser::new(&self.allocator, source_code, source_type).parse();

        if !errors.is_empty() {
            return Err(format!("Parse errors in {}: {} errors", file_path, errors.len()).into());
        }

        // Run OXC static analysis
        if let Some(oxc_linter) = &self.oxc_linter {
            let oxc_start = std::time::Instant::now();
            match oxc_linter.analyze_code(source_code, file_path) {
                Ok(oxc_result) => {
                    diagnostics.extend(oxc_result.diagnostics);
                    stats.oxc_rules_executed = oxc_result.rules_executed;
                    stats.oxc_duration_ms = oxc_start.elapsed().as_millis() as u64;
                }
                Err(e) => {
                    log::warn!("OXC analysis failed for {}: {}", file_path, e);
                }
            }
        }

        // Run AI behavioral analysis
        if let Some(ai_analyzer) = &self.ai_analyzer {
            let ai_start = std::time::Instant::now();
            let context = AnalysisContext {
                file_path: file_path.to_string(),
                file_type: source_type,
                project_context: None, // TODO: Implement project context detection
                dependencies: vec![],  // TODO: Extract dependencies from AST
            };

            match ai_analyzer.analyze_behavioral_patterns(source_code, &program, &context).await {
                Ok(ai_diagnostics) => {
                    let filtered_diagnostics: Vec<_> = ai_diagnostics
                        .into_iter()
                        .filter(|_d| {
                            // Filter by confidence if the diagnostic has AI confidence info
                            true // For now, accept all heuristic diagnostics
                        })
                        .collect();

                    stats.ai_patterns_checked = filtered_diagnostics.len();
                    diagnostics.extend(filtered_diagnostics);
                    stats.ai_duration_ms = ai_start.elapsed().as_millis() as u64;
                }
                Err(e) => {
                    log::warn!("AI behavioral analysis failed for {}: {}", file_path, e);
                }
            }
        }

        stats.total_duration_ms = start_time.elapsed().as_millis() as u64;

        // Run formatting if enabled
        let mut formatted_code = None;
        if let Some(formatter) = &self.formatter {
            let format_start = std::time::Instant::now();
            match formatter.format_code(source_code, file_path) {
                Ok(format_result) => {
                    formatted_code = Some(format_result.formatted_code);
                    stats.formatting_duration_ms = format_start.elapsed().as_millis() as u64;
                    stats.source_unchanged = format_result.source_unchanged;
                }
                Err(e) => {
                    log::warn!("Formatting failed for {}: {}", file_path, e);
                }
            }
        }

        // Run transformation if enabled
        let mut transformed_code = None;
        if let Some(transformer) = &self.transformer {
            let transform_start = std::time::Instant::now();
            let source_to_transform_string = source_code.to_string();
            let source_to_transform = formatted_code.as_ref().unwrap_or(&source_to_transform_string);
            match transformer.transform_code(source_to_transform, file_path) {
                Ok(transform_result) => {
                    transformed_code = Some(transform_result.transformed_code);
                    stats.transformation_duration_ms = transform_start.elapsed().as_millis() as u64;
                    stats.compression_ratio = Some(transform_result.compression_ratio);
                }
                Err(e) => {
                    log::warn!("Transformation failed for {}: {}", file_path, e);
                }
            }
        }

        stats.total_duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(MultiEngineAnalysisResult {
            diagnostics,
            formatted_code,
            transformed_code,
            ast_program: if self.config.preserve_ast {
                // Note: We can't store the program directly due to lifetime issues
                // This would need unsafe transmutation or a different approach
                None
            } else {
                None
            },
            stats,
        })
    }

    /// Update analyzer configuration
    pub fn update_config(&mut self, config: MultiEngineConfig) {
        // Recreate components if configuration changed
        if config.enable_oxc != self.config.enable_oxc || config.oxc_config != self.config.oxc_config {
            self.oxc_linter = if config.enable_oxc {
                config.oxc_config.as_ref().map(|oxc_config| OxcLinter::new(oxc_config.clone()))
            } else {
                None
            };
        }

        if config.enable_ai_behavioral != self.config.enable_ai_behavioral {
            self.ai_analyzer = if config.enable_ai_behavioral {
                Some(AiBehavioralAnalyzer::new())
            } else {
                None
            };
        }

        self.config = config;
    }

    /// Detect source type from file extension
    fn detect_source_type(&self, file_path: &str) -> SourceType {
        let path = std::path::Path::new(file_path);

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("ts") => SourceType::ts(),
            Some("tsx") => SourceType::tsx(),
            Some("jsx") => SourceType::jsx(),
            Some("mjs") => SourceType::mjs(),
            Some("cjs") => SourceType::cjs(),
            _ => SourceType::default(),
        }
    }

    /// Get current configuration
    pub fn config(&self) -> &MultiEngineConfig {
        &self.config
    }

    /// Check if OXC analysis is enabled
    pub fn is_oxc_enabled(&self) -> bool {
        self.oxc_linter.is_some()
    }

    /// Check if AI behavioral analysis is enabled
    pub fn is_ai_enabled(&self) -> bool {
        self.ai_analyzer.is_some()
    }
}

impl Default for MultiEngineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// #[cfg(test)]
// mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multi_engine_basic_analysis() {
        let mut analyzer = MultiEngineAnalyzer::new();

        let source_code = r#"
            function testFunction() {
                debugger; // Should be caught by OXC
                console.log("test"); // Should be caught by OXC
                let emptyArray = []; // Should be caught by OXC
            }
        "#;

        let result = analyzer.analyze_code(source_code, "test.js").await;
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(!analysis.diagnostics.is_empty(), "Should find diagnostics");
        assert!(analysis.stats.oxc_rules_executed > 0, "Should execute OXC rules");
    }

    #[tokio::test]
    async fn test_configuration_updates() {
        let mut analyzer = MultiEngineAnalyzer::new();
        assert!(analyzer.is_oxc_enabled());
        assert!(analyzer.is_ai_enabled());

        // Disable OXC
        let mut config = MultiEngineConfig::default();
        config.enable_oxc = false;
        analyzer.update_config(config);

        assert!(!analyzer.is_oxc_enabled());
        assert!(analyzer.is_ai_enabled());
    }

    #[test]
    fn test_source_type_detection() {
        let analyzer = MultiEngineAnalyzer::new();

        // Test source type detection (simplified test)
        let ts_type = analyzer.detect_source_type("test.ts");
        let js_type = analyzer.detect_source_type("test.js");
        assert!(ts_type != js_type); // Different file types should be detected differently
    }
// }
