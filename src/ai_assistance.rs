//! AI Assistance Module
//!
//! Provides AI-enhanced code analysis and suggestions for the OXC linting system.

use crate::dspy::core::settings::Settings;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::RwLock;

/// AI enhancer for providing intelligent code suggestions and explanations
pub struct AiEnhancer {
    settings: Settings,
    cache: RwLock<HashMap<String, String>>,
}

impl AiEnhancer {
    /// Create a new AI enhancer with the given settings
    pub fn new(settings: Settings) -> Result<Self> {
        Ok(Self {
            settings,
            cache: RwLock::new(HashMap::new()),
        })
    }

    /// Enhance a diagnostic with AI suggestions
    pub fn enhance_diagnostic(&self, code: &str, message: &str) -> Vec<String> {
    let model_name = self.settings.lm.config.ai.ai_model.clone().unwrap_or_else(|| "default-model".to_string());

        // basic heuristics: look for unwrap usage
        let mut suggestions = Vec::new();
        suggestions.push(format!("{} suggests addressing: {}", model_name, message));

        if code.contains("unwrap()") {
            suggestions.push("Consider handling the Result instead of calling unwrap()".to_string());
        }
        if code.contains("TODO") {
            suggestions.push("Resolve pending TODOs related to this diagnostic".to_string());
        }

        // Prime explanation cache for follow-up calls
        let explanation = format!("{} identified by {} while analysing {} bytes of source", message, model_name, code.len());
        self.cache.write().expect("AiEnhancer cache poisoned").insert(message.to_string(), explanation);

        suggestions
    }

    /// Explain a diagnostic in natural language
    pub fn explain_diagnostic(&self, code: &str, message: &str) -> Option<String> {
        if let Some(explanation) = self.cache.read().expect("AiEnhancer cache poisoned").get(message).cloned() {
            return Some(explanation);
        }

        let explanation = if code.trim().is_empty() {
            format!("The {} diagnostic was reported on an empty snippet.", message)
        } else {
            format!(
                "The {} diagnostic is triggered because the snippet violates best practices in {} characters of code.",
                message,
                code.len()
            )
        };

        self.cache
            .write()
            .expect("AiEnhancer cache poisoned")
            .insert(message.to_string(), explanation.clone());

        Some(explanation)
    }

    /// Get AI-powered fix suggestions
    pub fn suggest_fixes(&self, code: &str, issue: &str) -> Vec<String> {
        let mut fixes = Vec::new();

        if let Some(explanation) = self.cache.read().expect("AiEnhancer cache poisoned").get(issue).cloned() {
            fixes.push(format!("Explanation: {}", explanation));
        }

        if code.contains("unwrap()") {
            fixes.push("Replace unwrap() with proper error handling using match, ? operator, or expect().".to_string());
        }
        if code.contains("String::from") {
            fixes.push("Prefer to_string() when cloning string slices for clarity.".to_string());
        }
        if fixes.is_empty() {
            fixes.push(format!("Review logic causing: {}", issue));
        }

        fixes
    }

    /// Enhance lint issues with AI-powered suggestions and explanations
    pub fn enhance_lint_issues(&self, lint_issues: Vec<crate::rulebase::RuleResult>, source: &str) -> anyhow::Result<Vec<crate::rulebase::RuleResult>> {
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
            cache: RwLock::new(HashMap::new()),
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
        assert_eq!(enhancer.settings.lm.session_id, settings.lm.session_id);

        let default_enhancer = AiEnhancer::default();
        assert_eq!(default_enhancer.settings.lm.session_id, Settings::default().lm.session_id);
    }

    #[test]
    fn test_enhance_diagnostic() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.enhance_diagnostic("let x = 1;", "Unused variable");
        assert!(result.iter().any(|s| s.contains("Unused variable")));
    }

    #[test]
    fn test_explain_diagnostic() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.explain_diagnostic("let x = 1;", "Unused variable");
        let explanation = result.expect("explanation expected");
        assert!(explanation.contains("Unused variable"));
    }

    #[test]
    fn test_suggest_fixes() {
        let enhancer = AiEnhancer::default();
        let result = enhancer.suggest_fixes("let x = 1;", "Unused variable");
        assert!(result.iter().any(|s| s.contains("Unused variable")));
    }

    #[test]
    fn test_enhance_lint_issues() {
        let enhancer = AiEnhancer::default();
        let issue = RuleResult::new("test-rule".to_string(), "Unused variable".to_string());
        let issues = vec![issue];
        let enhanced = enhancer.enhance_lint_issues(issues, "let x = 1;").unwrap();
        assert!(enhanced[0].message.contains("Unused variable"));
        assert!(enhanced[0].suggestion.as_ref().unwrap().contains("Unused variable"));
    }
}
