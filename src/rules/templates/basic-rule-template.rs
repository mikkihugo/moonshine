//! Basic Rule Template
//!
//! This template provides the minimal structure for implementing a new OXC-compatible rule.
//! Copy this template and modify it according to your specific rule requirements.

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
    pub rule_name: String,
    pub suggestion: String,
    pub confidence: f32,
    pub auto_fixable: bool,
}

// ================================================================================================
// TEMPLATE: Replace "TemplateRule" with your actual rule name
// ================================================================================================

/// [TEMPLATE] Brief description of what this rule detects and prevents
///
/// **Rationale**: Explain why this rule exists and what problems it solves
///
/// **Examples**:
/// ```javascript
/// // ❌ Bad - problematic pattern
/// const bad = "example";
///
/// // ✅ Good - recommended pattern
/// const good = "example";
/// ```
pub struct TemplateRule;

impl TemplateRule {
    /// [TEMPLATE] Change to kebab-case rule identifier
    pub const NAME: &'static str = "template-rule";

    /// [TEMPLATE] Choose appropriate category:
    /// - Correctness: Logic errors, type issues, definite bugs
    /// - Suspicious: Potentially problematic patterns
    /// - Pedantic: Strict style enforcement
    /// - Perf: Performance optimization opportunities
    /// - Restriction: Security and safety restrictions
    /// - Style: Code formatting and consistency
    /// - Nursery: Experimental or unstable rules
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;

    /// [TEMPLATE] Choose appropriate fix status:
    /// - Fix: Safe automatic fixes
    /// - FixDangerous: Automatic fixes that might change behavior
    /// - Suggestion: Manual intervention recommended
    /// - ConditionalFixSuggestion: Context-dependent fixes
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for TemplateRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // [TEMPLATE] Replace with your pattern detection logic
        // Example: Simple string matching
        if code.contains("problematic_pattern") {
            // [TEMPLATE] You may need to implement actual line/column detection
            // For simple cases, using 1, 0 is acceptable
            diagnostics.push(create_template_rule_diagnostic(1, 0));
        }

        // [TEMPLATE] For more complex patterns, consider:
        // - Regular expressions for pattern matching
        // - Multi-line analysis for complex structures
        // - Context-aware detection for framework-specific patterns
        // - AST-style parsing for syntactic analysis

        diagnostics
    }
}

impl EnhancedWasmRule for TemplateRule {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),

            // [TEMPLATE] Provide specific, actionable improvement suggestions
            suggestion: "Replace with recommended pattern because [specific reason]".to_string(),

            // [TEMPLATE] Set confidence based on suggestion quality:
            // 0.95-1.0: Definitive improvements, minimal risk
            // 0.85-0.94: High confidence, some context dependency
            // 0.70-0.84: Good suggestions requiring validation
            // 0.50-0.69: Exploratory suggestions needing review
            confidence: 0.95,

            // [TEMPLATE] Set to true if fix can be applied automatically
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Function
// ================================================================================================

/// [TEMPLATE] Creates a diagnostic for the template rule violation
fn create_template_rule_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: TemplateRule::NAME.to_string(),

        // [TEMPLATE] Clear, concise description of the problem
        message: "Description of what's wrong and why it's problematic".to_string(),

        line,
        column,

        // [TEMPLATE] Choose severity:
        // "error": Definite problems that should block builds
        // "warning": Issues that should be addressed but don't block
        // "info": Suggestions and improvements
        severity: "error".to_string(),

        // [TEMPLATE] Provide specific fix instruction
        fix_suggestion: Some("Specific instruction on how to fix this issue".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rule_detection() {
        // [TEMPLATE] Test that the rule detects the problematic pattern
        let code = r#"problematic_pattern"#;
        let rule = TemplateRule;
        let diagnostics = rule.run(code);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, TemplateRule::NAME);
        assert_eq!(diagnostics[0].severity, "error");
    }

    #[test]
    fn test_template_rule_no_false_positives() {
        // [TEMPLATE] Test that the rule doesn't trigger on correct code
        let code = r#"correct_pattern"#;
        let rule = TemplateRule;
        let diagnostics = rule.run(code);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_template_rule_edge_cases() {
        // [TEMPLATE] Test edge cases to ensure robustness
        let edge_cases = vec![
            "", // Empty string
            "//", // Comments only
            "/* */", // Block comments
            "\n\n\n", // Whitespace only
        ];

        let rule = TemplateRule;
        for case in edge_cases {
            let diagnostics = rule.run(case);
            // Should not panic and should not produce false positives
            assert!(diagnostics.len() <= 1);
        }
    }

    #[test]
    fn test_ai_enhancement_quality() {
        // [TEMPLATE] Test that AI enhancement provides quality suggestions
        let code = r#"problematic_pattern"#;
        let rule = TemplateRule;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].suggestion.len() > 10);
        assert!(!suggestions[0].suggestion.is_empty());
    }

    #[test]
    fn test_multiple_violations() {
        // [TEMPLATE] Test handling of multiple violations in one code sample
        let code = r#"
            problematic_pattern
            some other code
            problematic_pattern
        "#;
        let rule = TemplateRule;
        let diagnostics = rule.run(code);

        // [TEMPLATE] Adjust expected count based on your rule logic
        assert!(diagnostics.len() >= 1);

        // Ensure all diagnostics are from this rule
        for diagnostic in &diagnostics {
            assert_eq!(diagnostic.rule_name, TemplateRule::NAME);
        }
    }
}

// ================================================================================================
// Usage Instructions
// ================================================================================================

/*
To use this template:

1. **Copy the file**: Create a new file for your rule
2. **Replace placeholders**: Search and replace "TemplateRule" and "template-rule"
3. **Update constants**: Set NAME, CATEGORY, and FIX_STATUS appropriately
4. **Implement detection**: Replace the pattern detection logic in `run()`
5. **Add AI enhancement**: Provide meaningful suggestions in `ai_enhance()`
6. **Create diagnostics**: Update the diagnostic creation function
7. **Write tests**: Update all test functions with your rule logic
8. **Add documentation**: Update the rule documentation comments

Example replacements:
- TemplateRule → NoUnusedVariables
- template-rule → no-unused-variables
- "problematic_pattern" → variable detection logic
- Update diagnostic messages and suggestions

Remember to:
- Follow the naming conventions in the rule format specification
- Ensure WASM compatibility (no external dependencies)
- Test thoroughly with edge cases
- Provide clear, actionable AI suggestions
- Document the rule's purpose and examples
*/