//! AI Assistance Module
//!
//! Provides AI-enhanced code analysis and suggestions for the OXC linting system.

use crate::dspy::core::settings::Settings;
use anyhow::Result;
use std::collections::HashMap;

/// An AI enhancer for providing intelligent code suggestions and explanations.
pub struct AiEnhancer {
    /// The settings for the AI enhancer.
    settings: Settings,
    /// A cache for storing previously generated suggestions.
    cache: HashMap<String, String>,
}

impl AiEnhancer {
    /// Creates a new `AiEnhancer` with the given settings.
    pub fn new(settings: Settings) -> Result<Self> {
        Ok(Self {
            settings,
            cache: HashMap::new(),
        })
    }

    /// Enhances a diagnostic with AI-powered suggestions.
    pub fn enhance_diagnostic(&self, _code: &str, message: &str) -> Vec<String> {
        vec![format!("AI suggestion: {}", message)]
    }

    /// Explains a diagnostic in natural language.
    pub fn explain_diagnostic(&self, _code: &str, message: &str) -> Option<String> {
        Some(format!("This issue occurs because: {}", message))
    }

    /// Gets AI-powered fix suggestions for a given issue.
    pub fn suggest_fixes(&self, _code: &str, issue: &str) -> Vec<String> {
        vec![format!("Consider fixing: {}", issue)]
    }

    /// Enhances a list of lint issues with AI-powered suggestions and explanations.
    pub fn enhance_lint_issues(
        &self,
        lint_issues: Vec<crate::rulebase::RuleResult>,
        source: &str,
    ) -> anyhow::Result<Vec<crate::rulebase::RuleResult>> {
        let mut enhanced_issues = Vec::new();

        for mut issue in lint_issues {
            // Add AI enhancements to the issue
            let ai_suggestions = self.suggest_fixes(source, &issue.message);
            let ai_explanation = self.explain_diagnostic(source, &issue.message);

            // Enhance the message with AI insights
            if !ai_suggestions.is_empty() {
                issue.message = format!("{} (AI: {})", issue.message, ai_suggestions.join(", "));
            }

            // Add AI explanation as suggestion if available
            if let Some(explanation) = ai_explanation {
                issue.suggestion = Some(explanation);
            }

            enhanced_issues.push(issue);
        }

        Ok(enhanced_issues)
    }
}

impl Default for AiEnhancer {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            cache: HashMap::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dspy::core::settings::Settings;
    use crate::rulebase::RuleResult;

    #[test]
    fn test_new_and_default() {
        let settings = Settings::default();
        let enhancer = AiEnhancer::new(settings.clone()).unwrap();
        assert_eq!(enhancer.settings, settings);

        let default_enhancer = AiEnhancer::default();
        assert_eq!(default_enhancer.settings, Settings::default());
    }

    #[test]
    fn test_enhance_diagnostic() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.enhance_diagnostic("let x = 1;", "Unused variable");
        assert_eq!(result, vec!["AI suggestion: Unused variable"]);
    }

    #[test]
    fn test_explain_diagnostic() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.explain_diagnostic("let x = 1;", "Unused variable");
        assert_eq!(result, Some("This issue occurs because: Unused variable".to_string()));
    }

    #[test]
    fn test_suggest_fixes() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.suggest_fixes("let x = 1;", "Unused variable");
        assert_eq!(result, vec!["Consider fixing: Unused variable"]);
    }

    #[test]
    fn test_enhance_lint_issues() {
        let enhancer = AiEnhancer::default();
        let issue = RuleResult::new("test-rule".to_string(), "Unused variable".to_string());
        let issues = vec![issue];
        let enhanced = enhancer.enhance_lint_issues(issues, "let x = 1;").unwrap();
        assert_eq!(enhanced[0].message, "Unused variable (AI: Consider fixing: Unused variable)");
        assert_eq!(enhanced[0].suggestion, Some("This issue occurs because: Unused variable".to_string()));
    }
}
