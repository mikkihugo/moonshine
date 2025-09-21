//! # Hybrid OXC + AI Linter (WASM-Safe)
//!
//! WASM-compatible linter that adapts OXC rule implementations with AI enhancements.
//! Uses OXC's AST parsing but implements rules in WASM-safe manner.

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::ai_assistance::AiEnhancer;
use crate::dspy::core::settings::Settings;
use crate::oxc_rules_adapter::{WasmRuleEngine, WasmSafeRule};

use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hybrid linter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridLinterConfig {
    /// Enable OXC's built-in rules
    pub enable_oxc_rules: bool,
    /// Enable AI-enhanced analysis
    pub enable_ai_enhancement: bool,
    /// Enable pure AI rules (SunLint style)
    pub enable_ai_rules: bool,
    /// OXC rule configuration
    pub oxc_config: OxcConfig,
    /// AI enhancement settings
    pub ai_config: AiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxcConfig {
    /// Enabled OXC rule categories
    pub enabled_categories: Vec<String>,
    /// Disabled specific rules
    pub disabled_rules: Vec<String>,
    /// Rule-specific configurations
    pub rule_configs: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// AI provider settings
    pub provider: String,
    /// Enhancement modes
    pub enhancement_modes: Vec<String>,
    /// AI rule configuration
    pub ai_rules: Vec<String>,
}

impl Default for HybridLinterConfig {
    fn default() -> Self {
        Self {
            enable_oxc_rules: true,
            enable_ai_enhancement: true,
            enable_ai_rules: true,
            oxc_config: OxcConfig::default(),
            ai_config: AiConfig::default(),
        }
    }
}

impl Default for OxcConfig {
    fn default() -> Self {
        Self {
            enabled_categories: vec![
                "eslint".to_string(),
                "import".to_string(),
                "typescript".to_string(),
                "react".to_string(),
                "unicorn".to_string(),
            ],
            disabled_rules: vec![],
            rule_configs: HashMap::new(),
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            enhancement_modes: vec![
                "suggestions".to_string(),
                "context".to_string(),
                "fixes".to_string(),
            ],
            ai_rules: vec!["C006".to_string(), "C042".to_string()],
        }
    }
}

/// Enhanced diagnostic that combines OXC diagnostics with AI insights
#[derive(Debug, Clone)]
pub struct EnhancedDiagnostic {
    /// Original OXC diagnostic (if any)
    pub oxc_diagnostic: Option<OxcDiagnostic>,
    /// AI-enhanced suggestions
    pub ai_suggestions: Vec<String>,
    /// AI-provided context
    pub ai_context: Option<String>,
    /// Automatic fix recommendations
    pub auto_fix: Option<String>,
    /// Rule source (OXC, AI, or Hybrid)
    pub source: DiagnosticSource,
}

#[derive(Debug, Clone)]
pub enum DiagnosticSource {
    Oxc(String),        // OXC rule name
    Ai(String),         // AI rule ID
    Hybrid(String),     // Enhanced OXC rule
}

/// OXC-based linting service wrapper
pub struct LintService {
    // Placeholder for OXC linter integration
}

/// Main hybrid linter that combines OXC with AI capabilities
pub struct HybridLinter {
    config: HybridLinterConfig,
    oxc_service: Option<LintService>,
    ai_enhancer: AiEnhancer,
}

impl HybridLinter {
    /// Create new hybrid linter with configuration
    pub fn new(config: HybridLinterConfig) -> anyhow::Result<Self> {
        let oxc_service = if config.enable_oxc_rules {
            Some(LintService::new())
        } else {
            None
        };

        let ai_enhancer = AiEnhancer::new(Settings::default())?;

        Ok(Self {
            config,
            oxc_service,
            ai_enhancer,
        })
    }

    /// Lint source code with hybrid analysis
    pub fn lint(&self, source: &str, filename: &str) -> anyhow::Result<Vec<EnhancedDiagnostic>> {
        let mut enhanced_diagnostics = Vec::new();

        // Parse with OXC for AST
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(filename)
            .unwrap_or_else(|_| SourceType::default().with_typescript(true));

        let parser_result = Parser::new(&allocator, source, source_type).parse();

        if !parser_result.errors.is_empty() {
            // Return parse errors as diagnostics
            for error in parser_result.errors {
                enhanced_diagnostics.push(EnhancedDiagnostic {
                    oxc_diagnostic: Some(OxcDiagnostic::error("Parse error")
                        .with_label(error.span)),
                    ai_suggestions: vec!["Fix syntax error".to_string()],
                    ai_context: Some("Syntax error prevents further analysis".to_string()),
                    auto_fix: None,
                    source: DiagnosticSource::Oxc("parser".to_string()),
                });
            }
            return Ok(enhanced_diagnostics);
        }

        let program = parser_result.program;

        // Build semantic model
        let semantic = SemanticBuilder::new().build(&program);

        // 1. Run OXC linter (if enabled)
        if let Some(ref oxc_service) = self.oxc_service {
            if self.config.enable_oxc_rules {
                let oxc_diagnostics = self.run_oxc_linter(&program, &semantic, source, filename)?;

                for diagnostic in oxc_diagnostics {
                    let enhanced = if self.config.enable_ai_enhancement {
                        self.enhance_oxc_diagnostic(diagnostic, &program, &semantic, source)?
                    } else {
                        EnhancedDiagnostic {
                            oxc_diagnostic: Some(diagnostic),
                            ai_suggestions: vec![],
                            ai_context: None,
                            auto_fix: None,
                            source: DiagnosticSource::Oxc("unknown".to_string()),
                        }
                    };
                    enhanced_diagnostics.push(enhanced);
                }
            }
        }

        // 2. Run pure AI rules (if enabled)
        if self.config.enable_ai_rules {
            let ai_diagnostics = self.run_ai_rules(&program, &semantic, source, filename)?;
            enhanced_diagnostics.extend(ai_diagnostics);
        }

        Ok(enhanced_diagnostics)
    }

    /// Run OXC's built-in linter
    fn run_oxc_linter(
        &self,
        program: &oxc_ast::ast::Program,
        semantic: &oxc_semantic::Semantic,
        source: &str,
        filename: &str,
    ) -> anyhow::Result<Vec<OxcDiagnostic>> {
        // This would use OXC's actual linter service
        // For now, return empty diagnostics as we need proper OXC integration
        Ok(vec![])
    }

    /// Enhance OXC diagnostic with AI insights
    fn enhance_oxc_diagnostic(
        &self,
        diagnostic: OxcDiagnostic,
        program: &oxc_ast::ast::Program,
        semantic: &oxc_semantic::Semantic,
        source: &str,
    ) -> anyhow::Result<EnhancedDiagnostic> {
        let ai_suggestions = self.ai_enhancer.generate_suggestions(&diagnostic, source)?;
        let ai_context = self.ai_enhancer.provide_context(&diagnostic, source)?;
        let auto_fix = self.ai_enhancer.suggest_auto_fix(&diagnostic, source)?;

        Ok(EnhancedDiagnostic {
            oxc_diagnostic: Some(diagnostic),
            ai_suggestions,
            ai_context,
            auto_fix,
            source: DiagnosticSource::Hybrid("enhanced".to_string()),
        })
    }

    /// Run pure AI-based rules
    fn run_ai_rules(
        &self,
        program: &oxc_ast::ast::Program,
        semantic: &oxc_semantic::Semantic,
        source: &str,
        filename: &str,
    ) -> anyhow::Result<Vec<EnhancedDiagnostic>> {
        let mut ai_diagnostics = Vec::new();

        for rule_id in &self.config.ai_config.ai_rules {
            let violations = self.ai_enhancer.run_ai_rule(rule_id, program, semantic, source)?;

            for violation in violations {
                ai_diagnostics.push(EnhancedDiagnostic {
                    oxc_diagnostic: None,
                    ai_suggestions: violation.suggestions,
                    ai_context: violation.context,
                    auto_fix: violation.auto_fix,
                    source: DiagnosticSource::Ai(rule_id.clone()),
                });
            }
        }

        Ok(ai_diagnostics)
    }

    /// Convert enhanced diagnostics to our LintIssue format
    pub fn to_lint_issues(&self, diagnostics: Vec<EnhancedDiagnostic>) -> Vec<LintIssue> {
        diagnostics
            .into_iter()
            .map(|diag| self.enhanced_diagnostic_to_lint_issue(diag))
            .collect()
    }

    fn enhanced_diagnostic_to_lint_issue(&self, diagnostic: EnhancedDiagnostic) -> LintIssue {
        let (message, line, column, severity, rule_id) = if let Some(ref oxc_diag) = diagnostic.oxc_diagnostic {
            (
                oxc_diag.message.clone(),
                1, // TODO: Extract from span
                1, // TODO: Extract from span
                LintSeverity::Warning, // TODO: Map from OXC severity
                match &diagnostic.source {
                    DiagnosticSource::Oxc(name) => name.clone(),
                    DiagnosticSource::Hybrid(name) => name.clone(),
                    DiagnosticSource::Ai(id) => id.clone(),
                }
            )
        } else {
            // Pure AI diagnostic
            (
                diagnostic.ai_context.unwrap_or_else(|| "AI-detected issue".to_string()),
                1,
                1,
                LintSeverity::Warning,
                match &diagnostic.source {
                    DiagnosticSource::Ai(id) => id.clone(),
                    _ => "ai-rule".to_string(),
                }
            )
        };

        LintIssue {
            rule_name: rule_id,
            message: if !diagnostic.ai_suggestions.is_empty() {
                format!("{} (AI: {})", message, diagnostic.ai_suggestions.join(", "))
            } else {
                message
            },
            line,
            column,
            severity,
            fix_available: diagnostic.auto_fix.is_some(),
        }
    }
}

/// AI violation result
#[derive(Debug, Clone)]
pub struct AiViolation {
    pub message: String,
    pub suggestions: Vec<String>,
    pub context: Option<String>,
    pub auto_fix: Option<String>,
    pub line: u32,
    pub column: u32,
}

/// Placeholder AI enhancer trait
impl AiEnhancer {
    pub fn generate_suggestions(
        &self,
        _diagnostic: &OxcDiagnostic,
        _source: &str,
    ) -> anyhow::Result<Vec<String>> {
        Ok(vec!["AI-generated suggestion".to_string()])
    }

    pub fn provide_context(
        &self,
        _diagnostic: &OxcDiagnostic,
        _source: &str,
    ) -> anyhow::Result<Option<String>> {
        Ok(Some("AI-provided context".to_string()))
    }

    pub fn suggest_auto_fix(
        &self,
        _diagnostic: &OxcDiagnostic,
        _source: &str,
    ) -> anyhow::Result<Option<String>> {
        Ok(Some("AI auto-fix suggestion".to_string()))
    }

    pub fn run_ai_rule(
        &self,
        _rule_id: &str,
        _program: &oxc_ast::ast::Program,
        _semantic: &oxc_semantic::Semantic,
        _source: &str,
    ) -> anyhow::Result<Vec<AiViolation>> {
        Ok(vec![])
    }
}