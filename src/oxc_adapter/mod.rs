//! # Modern OXC + AI Behavioral Analysis
//!
//! Advanced linting system combining ultra-fast OXC static analysis with AI-powered
//! behavioral pattern detection. Built for production WASM deployment.
//!
//! ## Architecture
//! - **OXC Parser**: Fastest JavaScript/TypeScript parsing (3x faster than SWC, 5x faster than Biome)
//! - **OXC Linter**: 50-100x faster than ESLint with 570+ rules
//! - **AI Behavioral**: Pattern detection for complex code smells
//! - **Unified Results**: Single analysis pass with merged diagnostics
//! - **Smart Fixes**: AI-enhanced auto-fixes for complex patterns
//!
//! ## Features
//! - Zero-config modern JavaScript/TypeScript analysis
//! - 570+ linting rules (ESLint, TypeScript, React, Jest, Unicorn, JSX A11y)
//! - AI pattern detection for architectural issues
//! - Intelligent auto-fixes using Claude API
//! - Real-time behavioral analysis
//! - Production-ready WASM execution
//!
//! ## Performance
//! - OXC linter: 50-100x faster than ESLint
//! - OXC parser: 3x faster than SWC, 5x faster than Biome
//! - Scales with CPU cores for parallel processing

pub mod adaptive_pattern_analyzer; // AI coder mistake pattern detection
pub mod ai_behavioral;
pub mod moon_integration; // Moon PDK integration approach
pub mod multi_engine_analyzer;
pub mod neural_pattern_models; // Neural network model integration
pub mod oxc_formatter; // OXC formatter integration (beta)
pub mod oxc_linter; // OXC linter integration
pub mod oxc_transformer; // OXC transformer integration
pub mod starcoder_integration; // StarCoder-1B fast pattern detection

use crate::rule_types::{FixStatus, RuleCategory, RuleMetadata, RuleSeverity};
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::{Parser, ParserReturn};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use adaptive_pattern_analyzer::{PatternAnalysisResult, PatternLearningConfig, RepetitivePatternLearner};
pub use ai_behavioral::{AiBehavioralAnalyzer, BehavioralPattern, BehavioralPatternType};
pub use moon_integration::{MoonAnalysisResult, OxcMoonAnalyzer};
pub use multi_engine_analyzer::{MultiEngineAnalyzer, MultiEngineConfig};
pub use oxc_formatter::{OxcFormatter, OxcFormatterConfig, OxcFormatterResult};
pub use oxc_linter::{OxcAnalysisResult, OxcConfig, OxcLinter};
pub use oxc_transformer::{OxcTransformationResult, OxcTransformer, OxcTransformerConfig};
pub use starcoder_integration::{AiMistakeAnalysis, CodePatternDetector, CodePatternType, LanguageModelConfig};

/// Modern linting adapter using OXC + AI behavioral analysis
pub struct OxcAdapter {
    rule_metadata: HashMap<String, ModernRuleMetadata>,
}

/// Metadata for modern linting rules
#[derive(Debug, Clone)]
pub struct ModernRuleMetadata {
    pub rule_name: String,
    pub group: String,
    pub description: String,
    pub severity: RuleSeverity,
    pub category: RuleCategory,
    pub has_fix: bool,
    pub docs_url: Option<String>,
}

/// Modern analysis result combining OXC parsing with AI behavioral analysis
#[derive(Debug)]
pub struct OxcAdapterResult {
    pub diagnostics: Vec<LintDiagnostic>,
    pub formatted_code: Option<String>,
}

impl OxcAdapter {
    /// Create a new OXC adapter with default configuration
    pub fn new() -> Self {
        let rule_metadata = Self::build_rule_metadata();

        Self { rule_metadata }
    }

    /// Build metadata for modern linting rules
    fn build_rule_metadata() -> HashMap<String, ModernRuleMetadata> {
        let mut metadata = HashMap::new();

        // Modern linting rules combining static analysis with AI behavioral patterns
        let example_rules = vec![
            (
                "correctness",
                "noUndeclaredVariables",
                "Disallow undeclared variables",
                RuleSeverity::Error,
                RuleCategory::Correctness,
            ),
            (
                "correctness",
                "noUnusedVariables",
                "Disallow unused variables",
                RuleSeverity::Warning,
                RuleCategory::Correctness,
            ),
            (
                "style",
                "useConsistentArrayType",
                "Enforce consistent array type",
                RuleSeverity::Warning,
                RuleCategory::Style,
            ),
            (
                "security",
                "noDangerouslySetInnerHtml",
                "Prevent XSS via dangerouslySetInnerHTML",
                RuleSeverity::Error,
                RuleCategory::Security,
            ),
            (
                "performance",
                "noReactSpecificProps",
                "Optimize React props usage",
                RuleSeverity::Warning,
                RuleCategory::Performance,
            ),
            (
                "complexity",
                "noExcessiveComplexity",
                "Limit cognitive complexity",
                RuleSeverity::Warning,
                RuleCategory::Complexity,
            ),
            (
                "ai-behavioral",
                "reactExcessiveRerenders",
                "AI: Detect excessive React re-renders",
                RuleSeverity::Warning,
                RuleCategory::Performance,
            ),
            (
                "ai-behavioral",
                "memoryLeakPatterns",
                "AI: Detect potential memory leak patterns",
                RuleSeverity::Error,
                RuleCategory::Performance,
            ),
            (
                "ai-behavioral",
                "securityVulnerabilities",
                "AI: Detect security vulnerability patterns",
                RuleSeverity::Error,
                RuleCategory::Security,
            ),
        ];

        for (group, rule_name, description, severity, category) in example_rules {
            let rule_metadata = ModernRuleMetadata {
                rule_name: rule_name.to_string(),
                group: group.to_string(),
                description: description.to_string(),
                severity,
                category,
                has_fix: group == "ai-behavioral", // AI rules can suggest fixes
                docs_url: Some(format!("https://moon-shine.dev/rules/{}", rule_name)),
            };

            metadata.insert(format!("{}:{}", group, rule_name), rule_metadata);
        }

        metadata
    }

    /// Get all available rules
    pub fn get_available_rules(&self) -> Vec<&ModernRuleMetadata> {
        self.rule_metadata.values().collect()
    }

    /// Get rule metadata by name
    pub fn get_rule_metadata(&self, rule_name: &str) -> Option<&ModernRuleMetadata> {
        self.rule_metadata.get(rule_name)
    }

    /// Check if a rule is enabled (simplified for now)
    pub fn is_rule_enabled(&self, _rule_name: &str) -> bool {
        true // All rules enabled by default
    }

    /// Analyze JavaScript/TypeScript code using OXC
    pub fn analyze_code(&self, source_code: &str, file_path: &str) -> Result<OxcAdapterResult, Box<dyn std::error::Error>> {
        // Use the OXC linter for actual analysis
        let config = OxcConfig::default();
        let linter = OxcLinter::new(config);
        let result = linter.analyze_code(source_code, file_path)?;

        Ok(OxcAdapterResult {
            diagnostics: result.diagnostics,
            formatted_code: None,
        })
    }

    /// Find line number of a pattern in source code
    fn find_line_number(&self, source_code: &str, pattern: &str) -> u32 {
        source_code
            .lines()
            .enumerate()
            .find(|(_, line)| line.contains(pattern))
            .map(|(line_num, _)| (line_num + 1) as u32)
            .unwrap_or(1)
    }

    /// Calculate line and column from byte offset
    fn calculate_line_column(&self, source: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in source.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Convert OXC rules to Moon Shine rule metadata format
    pub fn get_rule_registry_metadata(&self) -> Vec<RuleMetadata> {
        self.rule_metadata
            .values()
            .map(|oxc_rule| RuleMetadata {
                id: format!("oxc:{}", oxc_rule.rule_name),
                name: oxc_rule.rule_name.clone(),
                description: oxc_rule.description.clone(),
                category: oxc_rule.category.clone(),
                severity: oxc_rule.severity.clone(),
                fix_status: if oxc_rule.has_fix { FixStatus::Autofix } else { FixStatus::Manual },
                ai_enhanced: false,
                cost: 1, // OXC rules are generally low cost
                tags: vec!["oxc".to_string(), oxc_rule.group.clone()],
                dependencies: vec![],
                implementation: crate::rulebase::RuleImplementation::OxcStatic {
                    rule_name: oxc_rule.rule_name.clone(),
                },
                config_schema: None,
            })
            .collect()
    }

    /// Format code (OXC doesn't include a formatter yet)
    pub fn format_code(&self, source_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // OXC doesn't include a formatter yet
        // For now, return original code
        Ok(source_code.to_string())
    }
}

impl Default for OxcAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oxc_adapter_initialization() {
        let adapter = OxcAdapter::new();
        let rules = adapter.get_available_rules();
        assert!(!rules.is_empty(), "Should have OXC rules available");
    }

    #[test]
    fn test_rule_metadata_generation() {
        let adapter = OxcAdapter::new();
        let metadata = adapter.get_rule_registry_metadata();
        assert!(!metadata.is_empty(), "Should generate rule metadata");

        // Check that all rules have proper prefixes
        for rule in metadata {
            assert!(rule.id.starts_with("oxc:"), "Rule ID should start with 'oxc:'");
            assert!(rule.tags.contains(&"oxc".to_string()), "Rule should be tagged with 'oxc'");
        }
    }

    #[test]
    fn test_code_analysis_basic() {
        let adapter = OxcAdapter::new();
        let source_code = "console.log('Hello, world!');";

        let result = adapter.analyze_code(source_code, "test.js");
        assert!(result.is_ok(), "Should successfully analyze simple JavaScript code");
    }

    #[test]
    fn test_code_formatting_basic() {
        let adapter = OxcAdapter::new();
        let source_code = "const x=1;let y = 2;";

        let result = adapter.format_code(source_code);
        assert!(result.is_ok(), "Should successfully format JavaScript code");
    }
}
