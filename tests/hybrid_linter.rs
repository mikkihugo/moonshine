//! Unit tests for [`hybrid_linter.rs`](../src/hybrid_linter.rs)

use moon_shine::hybrid_linter::*;
use moon_shine::ai_assistance::AiEnhancer;
use moon_shine::hybrid_linter::{HybridLinter, HybridLinterConfig, OxcConfig, AiConfig, EnhancedDiagnostic, DiagnosticSource};
use std::collections::HashMap;

#[test]
fn default_config_is_valid() {
    let config = HybridLinterConfig::default();
    assert!(config.enable_oxc_rules);
    assert!(config.enable_ai_enhancement);
    assert!(config.enable_ai_rules);
    assert_eq!(config.oxc_config.enabled_categories.len(), 5);
    assert_eq!(config.ai_config.provider, "openai");
}

#[test]
fn oxc_config_default_values() {
    let oxc = OxcConfig::default();
    assert!(oxc.enabled_categories.contains(&"eslint".to_string()));
    assert!(oxc.disabled_rules.is_empty());
    assert!(oxc.rule_configs.is_empty());
}

#[test]
fn ai_config_default_values() {
    let ai = AiConfig::default();
    assert_eq!(ai.provider, "openai");
    assert!(ai.enhancement_modes.contains(&"suggestions".to_string()));
    assert!(ai.ai_rules.contains(&"C006".to_string()));
}

#[test]
fn enhanced_diagnostic_struct_fields() {
    let diag = EnhancedDiagnostic {
        oxc_diagnostic: None,
        ai_suggestions: vec!["Try using const".to_string()],
        ai_context: Some("Variable never reassigned".to_string()),
        auto_fix: Some("Replace let with const".to_string()),
        source: DiagnosticSource::Hybrid("prefer-const".to_string()),
    };
    assert_eq!(diag.ai_suggestions.len(), 1);
    assert!(diag.ai_context.is_some());
    assert!(diag.auto_fix.is_some());
    match diag.source {
        DiagnosticSource::Hybrid(ref s) => assert_eq!(s, "prefer-const"),
        _ => panic!("Expected Hybrid source"),
    }
}

#[test]
fn hybrid_linter_new_and_lint_smoke() {
    let config = HybridLinterConfig::default();
    let linter = HybridLinter::new(config.clone());
    assert!(linter.is_ok());
    let linter = linter.unwrap();
    // Minimal smoke test: should not panic on empty input
    let result = linter.lint("", "test.js");
    assert!(result.is_ok());
    let diagnostics = result.unwrap();
    assert!(diagnostics.is_empty() || diagnostics.iter().all(|d| d.ai_suggestions.is_empty()));
}