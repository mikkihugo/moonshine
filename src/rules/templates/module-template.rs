//! # [TEMPLATE] Module Title
//!
//! [TEMPLATE] This module implements WASM-safe OXC rules for [specific domain/technology].
//! Provide a clear description of the module's purpose and scope.
//!
//! ## Rule Categories:
//! - **Category 1**: Description of rules in this category
//! - **Category 2**: Description of rules in this category
//! - **Category 3**: Description of rules in this category
//!
//! ## Target Patterns:
//! Describe what types of code patterns this module analyzes and what problems it solves.
//! Include examples of the technologies, frameworks, or patterns covered.
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.

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
// [TEMPLATE] Category 1 Rules - Replace with your actual category
// ================================================================================================

/// [TEMPLATE] Description of the first rule in this module
pub struct FirstExampleRule;

impl FirstExampleRule {
    pub const NAME: &'static str = "first-example-rule";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for FirstExampleRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // [TEMPLATE] Replace with actual pattern detection logic
        if code.contains("first_pattern") {
            diagnostics.push(create_first_example_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for FirstExampleRule {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "AI suggestion for first rule".to_string(),
            confidence: 0.95,
            auto_fixable: true,
        }).collect()
    }
}

/// [TEMPLATE] Second rule in the same category
pub struct SecondExampleRule;

impl SecondExampleRule {
    pub const NAME: &'static str = "second-example-rule";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for SecondExampleRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // [TEMPLATE] Different pattern detection
        if code.contains("second_pattern") && !code.contains("exception") {
            diagnostics.push(create_second_example_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for SecondExampleRule {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "AI suggestion for second rule".to_string(),
            confidence: 0.88,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// [TEMPLATE] Category 2 Rules - Replace with your second category
// ================================================================================================

/// [TEMPLATE] Rule from a different category
pub struct ThirdExampleRule;

impl ThirdExampleRule {
    pub const NAME: &'static str = "third-example-rule";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for ThirdExampleRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // [TEMPLATE] More complex pattern detection
        let lines: Vec<&str> = code.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            if line.contains("performance_issue") {
                diagnostics.push(create_third_example_diagnostic(line_num + 1, 0));
            }
        }

        diagnostics
    }
}

impl EnhancedWasmRule for ThirdExampleRule {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Performance optimization suggestion with careful consideration of side effects".to_string(),
            confidence: 0.82,
            auto_fixable: false, // Dangerous fixes often need manual review
        }).collect()
    }
}

/// [TEMPLATE] Another rule in the performance category
pub struct FourthExampleRule;

impl FourthExampleRule {
    pub const NAME: &'static str = "fourth-example-rule";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::ConditionalFixSuggestion;
}

impl WasmRule for FourthExampleRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // [TEMPLATE] Context-sensitive detection
        if code.contains("restricted_pattern") &&
           (code.contains("production") || code.contains("sensitive")) {
            diagnostics.push(create_fourth_example_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for FourthExampleRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: if code.contains("production") {
                "In production context: Use secure alternative pattern"
            } else {
                "Consider security implications and use appropriate safeguards"
            }.to_string(),
            confidence: 0.90,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_first_example_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: FirstExampleRule::NAME.to_string(),
        message: "Description of the first rule violation".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Specific fix instruction for first rule".to_string()),
    }
}

fn create_second_example_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: SecondExampleRule::NAME.to_string(),
        message: "Description of the second rule violation".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Style improvement suggestion".to_string()),
    }
}

fn create_third_example_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: ThirdExampleRule::NAME.to_string(),
        message: "Performance issue detected that may impact application speed".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Optimize for better performance but review side effects".to_string()),
    }
}

fn create_fourth_example_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: FourthExampleRule::NAME.to_string(),
        message: "Restricted pattern usage detected in sensitive context".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Replace with security-approved alternative".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // [TEMPLATE] Tests for First Rule
    #[test]
    fn test_first_example_rule_detection() {
        let code = r#"first_pattern"#;
        let rule = FirstExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, FirstExampleRule::NAME);
    }

    #[test]
    fn test_first_example_rule_no_false_positives() {
        let code = r#"correct_pattern"#;
        let rule = FirstExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 0);
    }

    // [TEMPLATE] Tests for Second Rule
    #[test]
    fn test_second_example_rule_detection() {
        let code = r#"second_pattern"#;
        let rule = SecondExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, SecondExampleRule::NAME);
    }

    #[test]
    fn test_second_example_rule_exception() {
        let code = r#"second_pattern with exception"#;
        let rule = SecondExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 0);
    }

    // [TEMPLATE] Tests for Third Rule
    #[test]
    fn test_third_example_rule_detection() {
        let code = r#"
            line 1
            line with performance_issue
            line 3
        "#;
        let rule = ThirdExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, 2);
    }

    // [TEMPLATE] Tests for Fourth Rule
    #[test]
    fn test_fourth_example_rule_production_context() {
        let code = r#"restricted_pattern in production"#;
        let rule = FourthExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, FourthExampleRule::NAME);
    }

    #[test]
    fn test_fourth_example_rule_no_context() {
        let code = r#"restricted_pattern"#;
        let rule = FourthExampleRule;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 0);
    }

    // [TEMPLATE] AI Enhancement Tests
    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"first_pattern"#;
        let rule = FirstExampleRule;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].auto_fixable);
    }

    #[test]
    fn test_conditional_ai_enhancement() {
        let code = r#"restricted_pattern in production"#;
        let rule = FourthExampleRule;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].suggestion.contains("production"));
    }

    // [TEMPLATE] Edge Case Tests
    #[test]
    fn test_edge_cases() {
        let edge_cases = vec![
            "", // Empty string
            "//", // Comments only
            "/* */", // Block comments
            "\n\n\n", // Whitespace only
            "a".repeat(10000), // Very long string
        ];

        let rules: Vec<Box<dyn WasmRule>> = vec![
            Box::new(FirstExampleRule),
            Box::new(SecondExampleRule),
            Box::new(ThirdExampleRule),
            Box::new(FourthExampleRule),
        ];

        for rule in rules {
            for case in &edge_cases {
                let diagnostics = rule.run(case);
                // Should not panic and diagnostics should be reasonable
                assert!(diagnostics.len() < 100); // Sanity check
            }
        }
    }

    // [TEMPLATE] Performance Tests
    #[test]
    fn test_performance() {
        let large_code = "normal code\n".repeat(1000);
        let rule = FirstExampleRule;

        let start = std::time::Instant::now();
        let _diagnostics = rule.run(&large_code);
        let duration = start.elapsed();

        // Should complete within reasonable time (adjust as needed)
        assert!(duration.as_millis() < 100);
    }
}

// ================================================================================================
// Module Integration Instructions
// ================================================================================================

/*
To integrate this module:

1. **File Setup**:
   - Copy this template to a new file: `oxc_your_module_rules.rs`
   - Replace all [TEMPLATE] placeholders with actual content

2. **Customize Rules**:
   - Replace example rules with your actual rules
   - Update rule names, categories, and fix statuses
   - Implement proper pattern detection logic
   - Add meaningful AI enhancement suggestions

3. **Add to lib.rs**:
   ```rust
   pub mod oxc_your_module_rules; // Your module description
   ```

4. **Add to unified_rule_registry.rs**:
   ```rust
   use crate::oxc_your_module_rules::*;

   // In register_all_rules():
   self.register_rule(FirstExampleRule {});
   self.register_rule(SecondExampleRule {});
   self.register_rule(ThirdExampleRule {});
   self.register_rule(FourthExampleRule {});
   ```

5. **Update Documentation**:
   - Update the main README.md with your module
   - Add examples and usage notes
   - Document any special considerations

6. **Testing**:
   - Run all tests to ensure no regressions
   - Add integration tests if needed
   - Benchmark performance with large codebases

Remember to:
- Follow naming conventions (kebab-case for NAME constants)
- Choose appropriate categories and fix statuses
- Provide clear, actionable diagnostic messages
- Include comprehensive AI enhancement suggestions
- Test edge cases and performance scenarios
- Maintain WASM compatibility (no external dependencies)
*/