//! Smart Rule Strategy: AI-Enhanced vs Embedded Rules
//!
//! Instead of embedding 832 rules, use a hybrid approach:
//! 1. Core static rules (200-300 essential ones)
//! 2. AI enhancement for ANY rule violation
//! 3. Dynamic rule generation based on code patterns

use crate::types::{DiagnosticSeverity, LintDiagnostic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core static rules that are essential and fast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticRule {
    pub id: String,
    pub name: String,
    pub category: RuleCategory,
    pub severity: DiagnosticSeverity,
    pub has_autofix: bool,
    pub oxc_rule_name: Option<String>,     // Maps to OXC rule
    pub eslint_equivalent: Option<String>, // Maps to ESLint rule
}

/// AI-enhanced error with contextual suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiEnhancedError {
    pub base_diagnostic: LintDiagnostic,
    pub ai_suggestion: String,
    pub ai_explanation: String,
    pub ai_fix_code: Option<String>,
    pub confidence_score: f32,
    pub related_patterns: Vec<String>,
}

/// Smart rule engine that combines static + AI enhancement
pub struct SmartRuleEngine {
    // Core static rules (200-300 essential ones)
    core_rules: Vec<CoreStaticRule>,

    // AI enhancement for ANY violation
    ai_enhancer: AiErrorEnhancer,

    // Dynamic rule patterns learned from codebase
    adaptive_patterns: HashMap<String, u32>,
}

impl SmartRuleEngine {
    /// Analyze code with smart rule strategy
    pub async fn analyze_code(&self, source: &str, file_path: &str) -> Result<Vec<AiEnhancedError>, Box<dyn std::error::Error>> {
        // 1. Run core static analysis (fast)
        let static_diagnostics = self.run_core_static_analysis(source, file_path)?;

        // 2. Enhance each diagnostic with AI
        let mut enhanced_errors = Vec::new();
        for diagnostic in static_diagnostics {
            let enhanced = self.ai_enhancer.enhance_diagnostic(&diagnostic, source, file_path).await?;
            enhanced_errors.push(enhanced);
        }

        // 3. Detect new patterns for adaptive learning
        self.learn_from_patterns(source, &enhanced_errors);

        Ok(enhanced_errors)
    }

    /// Run only essential static rules
    fn run_core_static_analysis(&self, source: &str, file_path: &str) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        // Use OXC for core parsing + essential rules only
        let parse_result = crate::oxc_adapter::parse_code(source, file_path)?;

        // Convert to diagnostics
        let diagnostics = crate::oxc_adapter::convert_diagnostics(&parse_result.errors);

        // Apply only core static rules (not all 832!)
        let mut core_diagnostics = Vec::new();
        for diag in diagnostics {
            if self.is_core_rule(&diag.rule_name) {
                core_diagnostics.push(diag);
            }
        }

        Ok(core_diagnostics)
    }

    /// Check if rule is in our core set
    fn is_core_rule(&self, rule_name: &str) -> bool {
        self.core_rules.iter().any(|rule| rule.id == rule_name)
    }

    /// Learn patterns for adaptive rule generation
    fn learn_from_patterns(&mut self, source: &str, errors: &[AiEnhancedError]) {
        for error in errors {
            let pattern_key = format!("{}:{}", error.base_diagnostic.rule_name, error.base_diagnostic.message);
            *self.adaptive_patterns.entry(pattern_key).or_insert(0) += 1;
        }
    }
}

/// AI error enhancer that works with ANY rule violation
pub struct AiErrorEnhancer {
    pub model: String,
    pub max_tokens: u32,
}

impl AiErrorEnhancer {
    /// Enhance any diagnostic with AI context
    pub async fn enhance_diagnostic(&self, diagnostic: &LintDiagnostic, source: &str, file_path: &str) -> Result<AiEnhancedError, Box<dyn std::error::Error>> {
        // Create AI prompt for ANY rule violation
        let prompt = self.create_enhancement_prompt(diagnostic, source, file_path);

        // Call AI model (Claude, etc.)
        let ai_response = self.call_ai_model(&prompt).await?;

        // Parse AI response
        let enhanced = AiEnhancedError {
            base_diagnostic: diagnostic.clone(),
            ai_suggestion: ai_response.suggestion,
            ai_explanation: ai_response.explanation,
            ai_fix_code: ai_response.fix_code,
            confidence_score: ai_response.confidence,
            related_patterns: ai_response.related_patterns,
        };

        Ok(enhanced)
    }

    /// Create enhancement prompt for any rule violation
    fn create_enhancement_prompt(&self, diagnostic: &LintDiagnostic, source: &str, file_path: &str) -> String {
        format!(
            "Analyze this code issue and provide enhanced feedback:\n\n\
            File: {}\n\
            Rule: {}\n\
            Message: {}\n\
            Line: {}, Column: {}\n\
            Severity: {:?}\n\n\
            Code Context:\n```typescript\n{}\n```\n\n\
            Please provide:\n\
            1. Clear explanation of why this is an issue\n\
            2. Specific suggestion for improvement\n\
            3. Fixed code example if applicable\n\
            4. Related patterns to watch for\n\
            5. Confidence score (0-1)",
            file_path, diagnostic.rule_name, diagnostic.message, diagnostic.line, diagnostic.column, diagnostic.severity, source
        )
    }

    /// Call AI model (placeholder - integrate with actual AI provider)
    async fn call_ai_model(&self, prompt: &str) -> Result<AiResponse, Box<dyn std::error::Error>> {
        // TODO: Integrate with Claude/Gemini/etc.
        // For now, return mock response
        Ok(AiResponse {
            suggestion: "Consider refactoring this code for better maintainability".to_string(),
            explanation: "This pattern can lead to maintenance issues".to_string(),
            fix_code: None,
            confidence: 0.8,
            related_patterns: vec!["similar-pattern-1".to_string()],
        })
    }
}

#[derive(Debug)]
struct AiResponse {
    suggestion: String,
    explanation: String,
    fix_code: Option<String>,
    confidence: f32,
    related_patterns: Vec<String>,
}

/// Essential rule categories we need to embed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    // Core syntax and parsing
    Syntax,
    Parsing,

    // Essential code quality
    CodeQuality,
    Performance,
    Security,

    // TypeScript specific
    TypeScript,

    // AI-enhanced categories (not embedded, generated dynamically)
    AiBehavioral,
    AiPattern,
    AiArchitecture,
}

/// Recommended core rules to embed (200-300 instead of 832)
pub fn get_core_rules() -> Vec<CoreStaticRule> {
    vec![
        // Syntax & Parsing (50 rules)
        CoreStaticRule {
            id: "syntax-error".to_string(),
            name: "Syntax Error".to_string(),
            category: RuleCategory::Syntax,
            severity: DiagnosticSeverity::Error,
            has_autofix: false,
            oxc_rule_name: Some("syntax-error".to_string()),
            eslint_equivalent: None,
        },
        // Security (30 rules)
        CoreStaticRule {
            id: "no-eval".to_string(),
            name: "No Eval".to_string(),
            category: RuleCategory::Security,
            severity: DiagnosticSeverity::Error,
            has_autofix: true,
            oxc_rule_name: Some("no-eval".to_string()),
            eslint_equivalent: Some("no-eval".to_string()),
        },
        // Performance (40 rules)
        CoreStaticRule {
            id: "prefer-const".to_string(),
            name: "Prefer Const".to_string(),
            category: RuleCategory::Performance,
            severity: DiagnosticSeverity::Warning,
            has_autofix: true,
            oxc_rule_name: Some("prefer-const".to_string()),
            eslint_equivalent: Some("prefer-const".to_string()),
        },
        // TypeScript (50 rules)
        CoreStaticRule {
            id: "no-unused-vars".to_string(),
            name: "No Unused Variables".to_string(),
            category: RuleCategory::TypeScript,
            severity: DiagnosticSeverity::Warning,
            has_autofix: true,
            oxc_rule_name: Some("no-unused-vars".to_string()),
            eslint_equivalent: Some("no-unused-vars".to_string()),
        },
        // Code Quality (30 rules)
        CoreStaticRule {
            id: "no-console".to_string(),
            name: "No Console".to_string(),
            category: RuleCategory::CodeQuality,
            severity: DiagnosticSeverity::Warning,
            has_autofix: true,
            oxc_rule_name: Some("no-console".to_string()),
            eslint_equivalent: Some("no-console".to_string()),
        },
        // Total: ~200 core rules instead of 832
    ]
}

/// Benefits of this approach:
/// 1. Smaller binary size (200 rules vs 832)
/// 2. AI enhancement for ANY rule violation
/// 3. Dynamic rule generation based on codebase patterns
/// 4. Better error messages with context
/// 5. Easier maintenance and updates
/// 6. Can still integrate with ESLint ecosystem
pub fn get_benefits() -> Vec<&'static str> {
    vec![
        "Smaller binary size (200 rules vs 832)",
        "AI enhancement for ANY rule violation",
        "Dynamic rule generation based on codebase patterns",
        "Better error messages with context",
        "Easier maintenance and updates",
        "Can still integrate with ESLint ecosystem",
    ]
}
