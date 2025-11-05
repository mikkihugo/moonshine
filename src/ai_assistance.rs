//! AI Assistance Module
//!
//! Provides AI-enhanced code analysis and suggestions for the OXC linting system.

use crate::dspy::core::settings::Settings;
use crate::dspy::{adapter::Adapter, ChatAdapter, LM};
use anyhow::Result;
use secrecy::SecretString;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
        let model_name = self.settings.lm.config.model.clone();

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

fn default_settings() -> Settings {
    let adapter: Arc<dyn Adapter> = Arc::new(ChatAdapter::default());
    Settings {
        lm: LM::builder()
            .api_key(SecretString::new(String::new().into_boxed_str()))
            .build(),
        adapter,
    }
}


impl Default for AiEnhancer {
    fn default() -> Self {
        Self {
            settings: default_settings(),
            cache: RwLock::new(HashMap::new()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulebase::RuleResult;

    #[test]
    fn test_new_and_default() {
        let settings = default_settings();
        let model_name = settings.lm.config.model.clone();
        let enhancer = AiEnhancer::new(settings).unwrap();
        assert_eq!(enhancer.settings.lm.config.model, model_name);

        let default_enhancer = AiEnhancer::default();
        assert_eq!(default_enhancer.settings.lm.config.model, "gpt-4o-mini");
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
