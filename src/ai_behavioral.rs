//! # AI Behavioral Pattern Analysis
//!
//! Advanced behavioral pattern detection using AI to identify complex code smells,
//! architectural issues, and performance anti-patterns that static analysis cannot catch.

use crate::rule_types::RuleSeverity;
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_ast::ast::Program;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI behavioral pattern analyzer
pub struct AiBehavioralAnalyzer {
    patterns: Vec<BehavioralPattern>,
    ai_client: Option<Box<dyn AiAnalysisClient>>,
}

/// Behavioral pattern definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: RuleSeverity,
    pub pattern_type: BehavioralPatternType,
    pub ai_prompt: String,
    pub confidence_threshold: f32,
}

/// Types of behavioral patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehavioralPatternType {
    /// Complex cognitive patterns (nested conditions, deep inheritance)
    CognitiveComplexity,
    /// Performance anti-patterns (excessive re-renders, memory leaks)
    PerformanceAntiPattern,
    /// Security vulnerabilities (XSS, injection patterns)
    SecurityVulnerability,
    /// Architectural smells (circular dependencies, god objects)
    ArchitecturalSmell,
    /// Maintainability issues (dead code, duplication patterns)
    MaintainabilityIssue,
    /// Accessibility violations (missing ARIA, poor semantics)
    AccessibilityViolation,
    /// Testing anti-patterns (test coupling, brittle tests)
    TestingAntiPattern,
}

/// AI analysis client trait for different providers
pub trait AiAnalysisClient: Send + Sync {
    /// Analyze code for behavioral patterns using AI
    fn analyze_patterns(
        &self,
        code: &str,
        patterns: &[BehavioralPattern],
        context: &AnalysisContext,
    ) -> Result<Vec<AiPatternResult>, Box<dyn std::error::Error>>;

    /// Get suggested fix for a detected pattern
    fn suggest_fix(&self, code: &str, pattern: &BehavioralPattern, issue_span: (usize, usize)) -> Result<Option<String>, Box<dyn std::error::Error>>;
}

/// Context for AI analysis
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub file_path: String,
    pub file_type: SourceType,
    pub project_context: Option<ProjectContext>,
    pub dependencies: Vec<String>,
}

/// Project context for better AI analysis
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub framework: Option<String>,         // React, Vue, Angular, etc.
    pub build_tool: Option<String>,        // Vite, Webpack, etc.
    pub testing_framework: Option<String>, // Jest, Vitest, etc.
    pub package_json_dependencies: HashMap<String, String>,
}

/// AI pattern detection result
#[derive(Debug, Clone)]
pub struct AiPatternResult {
    pub pattern_id: String,
    pub confidence: f32,
    pub message: String,
    pub suggestion: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub related_patterns: Vec<String>,
}

impl AiBehavioralAnalyzer {
    /// Create new behavioral analyzer with default patterns
    pub fn new() -> Self {
        Self {
            patterns: Self::default_behavioral_patterns(),
            ai_client: None,
        }
    }

    /// Create analyzer with custom AI client
    pub fn with_ai_client(ai_client: Box<dyn AiAnalysisClient>) -> Self {
        Self {
            patterns: Self::default_behavioral_patterns(),
            ai_client: Some(ai_client),
        }
    }

    /// Default behavioral patterns for modern JavaScript/TypeScript
    fn default_behavioral_patterns() -> Vec<BehavioralPattern> {
        vec![
            BehavioralPattern {
                id: "react-excessive-rerenders".to_string(),
                name: "Excessive Component Re-renders".to_string(),
                description: "Component may be re-rendering unnecessarily due to improper dependency arrays or object/function recreation".to_string(),
                category: "performance".to_string(),
                severity: RuleSeverity::Warning,
                pattern_type: BehavioralPatternType::PerformanceAntiPattern,
                ai_prompt: "Analyze this React component for excessive re-rendering patterns. Look for: 1) useEffect with missing dependencies, 2) inline object/function creation in JSX, 3) unnecessary state updates, 4) missing useMemo/useCallback optimization opportunities.".to_string(),
                confidence_threshold: 0.7,
            },
            BehavioralPattern {
                id: "cognitive-complexity-high".to_string(),
                name: "High Cognitive Complexity".to_string(),
                description: "Function has high cognitive complexity making it difficult to understand and maintain".to_string(),
                category: "complexity".to_string(),
                severity: RuleSeverity::Warning,
                pattern_type: BehavioralPatternType::CognitiveComplexity,
                ai_prompt: "Analyze this function for cognitive complexity. Count: nested loops (+1 each level), nested conditionals (+1 each level), catch blocks (+1), switch cases (+1), logical operators in conditions (+1). Suggest refactoring if total > 15.".to_string(),
                confidence_threshold: 0.8,
            },
            BehavioralPattern {
                id: "xss-vulnerability-pattern".to_string(),
                name: "Potential XSS Vulnerability".to_string(),
                description: "Code pattern may be vulnerable to cross-site scripting attacks".to_string(),
                category: "security".to_string(),
                severity: RuleSeverity::Error,
                pattern_type: BehavioralPatternType::SecurityVulnerability,
                ai_prompt: "Analyze for XSS vulnerabilities. Look for: 1) dangerouslySetInnerHTML usage, 2) unescaped user input in DOM, 3) dynamic script tag generation, 4) eval() with user data, 5) innerHTML with concatenated strings.".to_string(),
                confidence_threshold: 0.9,
            },
            BehavioralPattern {
                id: "god-object-pattern".to_string(),
                name: "God Object Anti-Pattern".to_string(),
                description: "Class or object has too many responsibilities and should be decomposed".to_string(),
                category: "architecture".to_string(),
                severity: RuleSeverity::Warning,
                pattern_type: BehavioralPatternType::ArchitecturalSmell,
                ai_prompt: "Analyze this class/object for god object pattern. Check: 1) number of methods (>20 is concerning), 2) number of properties (>15 is concerning), 3) multiple unrelated responsibilities, 4) high coupling with many dependencies.".to_string(),
                confidence_threshold: 0.75,
            },
            BehavioralPattern {
                id: "memory-leak-pattern".to_string(),
                name: "Potential Memory Leak".to_string(),
                description: "Code pattern may cause memory leaks due to uncleaned event listeners or timers".to_string(),
                category: "performance".to_string(),
                severity: RuleSeverity::Error,
                pattern_type: BehavioralPatternType::PerformanceAntiPattern,
                ai_prompt: "Analyze for memory leak patterns. Look for: 1) addEventListener without removeEventListener, 2) setInterval/setTimeout without clear, 3) useEffect without cleanup, 4) closure holding large objects, 5) circular references.".to_string(),
                confidence_threshold: 0.8,
            },
            BehavioralPattern {
                id: "accessibility-missing-aria".to_string(),
                name: "Missing Accessibility Attributes".to_string(),
                description: "Interactive elements lack proper ARIA labels or semantic HTML".to_string(),
                category: "accessibility".to_string(),
                severity: RuleSeverity::Warning,
                pattern_type: BehavioralPatternType::AccessibilityViolation,
                ai_prompt: "Analyze for accessibility issues. Check: 1) buttons without aria-label, 2) images without alt text, 3) form inputs without labels, 4) custom components without proper ARIA roles, 5) missing keyboard navigation support.".to_string(),
                confidence_threshold: 0.7,
            },
            BehavioralPattern {
                id: "test-coupling-high".to_string(),
                name: "High Test Coupling".to_string(),
                description: "Tests are tightly coupled to implementation details making them brittle".to_string(),
                category: "testing".to_string(),
                severity: RuleSeverity::Warning,
                pattern_type: BehavioralPatternType::TestingAntiPattern,
                ai_prompt: "Analyze test code for brittleness. Look for: 1) testing implementation details vs behavior, 2) excessive mocking, 3) tests breaking on refactoring, 4) hard-coded selectors, 5) testing framework internals.".to_string(),
                confidence_threshold: 0.75,
            },
        ]
    }

    /// Analyze code for behavioral patterns
    pub async fn analyze_behavioral_patterns(
        &self,
        code: &str,
        program: &Program<'_>,
        context: &AnalysisContext,
    ) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        let mut diagnostics = Vec::new();

        // First, run fast heuristic checks
        let heuristic_results = self.run_heuristic_analysis(code, program, context)?;
        diagnostics.extend(heuristic_results);

        // Then, run AI analysis if client is available
        if let Some(ai_client) = &self.ai_client {
            let ai_results = ai_client.analyze_patterns(code, &self.patterns, context)?;

            for result in ai_results {
                if result.confidence >= self.get_pattern_threshold(&result.pattern_id) {
                    let pattern = self.patterns.iter().find(|p| p.id == result.pattern_id).unwrap();

                    let (line, column) = self.calculate_position(code, result.start_offset);

                    diagnostics.push(LintDiagnostic {
                        rule_name: format!("ai-behavioral:{}", pattern.id),
                        severity: Self::convert_severity(&pattern.severity),
                        message: format!("{} (AI confidence: {:.1}%)", result.message, result.confidence * 100.0),
                        file_path: context.file_path.clone(),
                        line: line as u32,
                        column: column as u32,
                        end_line: line as u32,
                        end_column: (column + 10) as u32,
                        fix_available: result.suggestion.is_some(),
                        suggested_fix: result.suggestion,
                    });
                }
            }
        }

        Ok(diagnostics)
    }

    /// Run fast heuristic analysis without AI
    fn run_heuristic_analysis(&self, code: &str, program: &Program<'_>, context: &AnalysisContext) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        let mut diagnostics = Vec::new();

        // Example: Simple cognitive complexity heuristic
        let complexity_score = self.calculate_cognitive_complexity_heuristic(code);
        if complexity_score > 15 {
            diagnostics.push(LintDiagnostic {
                rule_name: "heuristic:high-cognitive-complexity".to_string(),
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "High cognitive complexity detected (score: {}). Consider breaking down this function.",
                    complexity_score
                ),
                file_path: context.file_path.clone(),
                line: 1,
                column: 1,
                end_line: 1,
                end_column: 1,
                fix_available: true,
                suggested_fix: Some("Consider extracting complex logic into separate functions".to_string()),
            });
        }

        // Example: Basic memory leak detection heuristic
        if code.contains("addEventListener") && !code.contains("removeEventListener") {
            diagnostics.push(LintDiagnostic {
                rule_name: "heuristic:potential-memory-leak".to_string(),
                severity: DiagnosticSeverity::Warning,
                message: "Potential memory leak: addEventListener without corresponding removeEventListener".to_string(),
                file_path: context.file_path.clone(),
                line: 1,
                column: 1,
                end_line: 1,
                end_column: 1,
                fix_available: true,
                suggested_fix: Some("Add corresponding removeEventListener call".to_string()),
            });
        }

        Ok(diagnostics)
    }

    /// Simple cognitive complexity calculation
    fn calculate_cognitive_complexity_heuristic(&self, code: &str) -> u32 {
        let mut score = 0u32;

        // Count nesting indicators
        let nesting_chars = code.matches('{').count() as u32;
        score += nesting_chars;

        // Count conditionals
        score += code.matches("if ").count() as u32;
        score += code.matches("else if").count() as u32;
        score += code.matches("switch").count() as u32;
        score += code.matches("case ").count() as u32;

        // Count loops
        score += code.matches("for ").count() as u32;
        score += code.matches("while ").count() as u32;
        score += code.matches("do ").count() as u32;

        // Count logical operators in conditions
        score += code.matches(" && ").count() as u32;
        score += code.matches(" || ").count() as u32;

        score
    }

    /// Get confidence threshold for a pattern
    fn get_pattern_threshold(&self, pattern_id: &str) -> f32 {
        self.patterns.iter().find(|p| p.id == pattern_id).map(|p| p.confidence_threshold).unwrap_or(0.8)
    }

    /// Calculate line and column from byte offset
    fn calculate_position(&self, source: &str, offset: usize) -> (usize, usize) {
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

    /// Convert RuleSeverity to DiagnosticSeverity
    fn convert_severity(severity: &RuleSeverity) -> DiagnosticSeverity {
        match severity {
            RuleSeverity::Error => DiagnosticSeverity::Error,
            RuleSeverity::Warning => DiagnosticSeverity::Warning,
            RuleSeverity::Info => DiagnosticSeverity::Info,
            RuleSeverity::Hint => DiagnosticSeverity::Hint,
            RuleSeverity::Custom(_) => DiagnosticSeverity::Warning,
        }
    }

    /// Add custom behavioral pattern
    pub fn add_pattern(&mut self, pattern: BehavioralPattern) {
        self.patterns.push(pattern);
    }

    /// Get all configured patterns
    pub fn get_patterns(&self) -> &[BehavioralPattern] {
        &self.patterns
    }

    /// Set AI analysis client
    pub fn set_ai_client(&mut self, client: Box<dyn AiAnalysisClient>) {
        self.ai_client = Some(client);
    }
}

impl Default for AiBehavioralAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_complexity_heuristic() {
        let analyzer = AiBehavioralAnalyzer::new();

        let simple_code = "function simple() { return 1; }";
        assert_eq!(analyzer.calculate_cognitive_complexity_heuristic(simple_code), 1);

        let complex_code = r#"
            function complex() {
                if (condition1) {
                    for (let i = 0; i < 10; i++) {
                        if (condition2 && condition3) {
                            switch (value) {
                                case 1:
                                    break;
                                case 2:
                                    break;
                            }
                        }
                    }
                }
            }
        "#;
        let score = analyzer.calculate_cognitive_complexity_heuristic(complex_code);
        assert!(score > 10, "Complex code should have high complexity score");
    }

    #[test]
    fn test_default_patterns() {
        let analyzer = AiBehavioralAnalyzer::new();
        let patterns = analyzer.get_patterns();

        assert!(!patterns.is_empty(), "Should have default patterns");
        assert!(patterns.iter().any(|p| p.pattern_type == BehavioralPatternType::PerformanceAntiPattern));
        assert!(patterns.iter().any(|p| p.pattern_type == BehavioralPatternType::SecurityVulnerability));
        assert!(patterns.iter().any(|p| p.pattern_type == BehavioralPatternType::CognitiveComplexity));
    }

    #[test]
    fn test_pattern_confidence_thresholds() {
        let analyzer = AiBehavioralAnalyzer::new();

        // Security patterns should have high confidence thresholds
        let security_pattern = analyzer
            .patterns
            .iter()
            .find(|p| matches!(p.pattern_type, BehavioralPatternType::SecurityVulnerability))
            .unwrap();
        assert!(security_pattern.confidence_threshold >= 0.8);
    }
}