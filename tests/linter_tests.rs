//! WASM-compatible tests for the AI linter module
//!
//! These tests focus on core data structures and logic without Extism PDK dependencies

use moon_shine::linter::*;
use std::collections::HashMap;

/// Test AI suggestion creation and validation
#[test]
fn test_ai_suggestion_creation() {
  let suggestion = AiSuggestion {
    line: 42,
    column: 15,
    message:
      "Consider using optional chaining (?.) instead of manual null check"
        .to_string(),
    severity: SuggestionSeverity::Warning,
    rule_id: Some("prefer-optional-chaining".to_string()),
    suggested_fix: Some(
      "Replace `item && item.property` with `item?.property`".to_string(),
    ),
    category: SuggestionCategory::Modernization,
    confidence: 0.87,
    auto_fixable: true,
    impact_score: 5,
    related_suggestions: vec![],
  };

  assert_eq!(suggestion.line, 42);
  assert_eq!(suggestion.column, 15);
  assert_eq!(suggestion.severity, SuggestionSeverity::Warning);
  assert_eq!(suggestion.category, SuggestionCategory::Modernization);
  assert!((suggestion.confidence - 0.87).abs() < f32::EPSILON);
  assert!(suggestion.auto_fixable);
  assert_eq!(suggestion.impact_score, 5);
}

/// Test suggestion severity ordering and comparison
#[test]
fn test_suggestion_severity_ordering() {
  assert!(SuggestionSeverity::Critical < SuggestionSeverity::Error);
  assert!(SuggestionSeverity::Error < SuggestionSeverity::Warning);
  assert!(SuggestionSeverity::Warning < SuggestionSeverity::Info);
  assert!(SuggestionSeverity::Info < SuggestionSeverity::Hint);

  assert_eq!(SuggestionSeverity::Warning, SuggestionSeverity::Warning);
  assert_ne!(SuggestionSeverity::Error, SuggestionSeverity::Warning);
}

/// Test suggestion categories and classification
#[test]
fn test_suggestion_categories() {
  let categories = vec![
    SuggestionCategory::Compilation,
    SuggestionCategory::TypeSafety,
    SuggestionCategory::Performance,
    SuggestionCategory::Security,
    SuggestionCategory::Style,
    SuggestionCategory::Maintainability,
    SuggestionCategory::Documentation,
    SuggestionCategory::Modernization,
    SuggestionCategory::BestPractices,
  ];

  assert_eq!(categories.len(), 9);

  // Test serialization/deserialization
  for category in categories {
    let serialized =
      serde_json::to_string(&category).expect("Should serialize");
    let deserialized: SuggestionCategory =
      serde_json::from_str(&serialized).expect("Should deserialize");
    assert_eq!(category, deserialized);
  }
}

/// Test AI linter configuration creation
#[test]
fn test_ai_linter_configuration() {
  let default_linter = AiLinter::default();
  assert_eq!(default_linter.max_suggestions, 50);
  assert_eq!(default_linter.min_confidence, 0.7);
  assert!(default_linter.enable_auto_fix);
  assert_eq!(default_linter.session_id.len(), 36); // UUID length

  let mut custom_linter = AiLinter::default();
  custom_linter.max_suggestions = 100;
  custom_linter.min_confidence = 0.9;
  custom_linter.enable_auto_fix = false;
  custom_linter.session_id = "custom-session-123".to_string();

  custom_linter.language_preferences = {
    let mut prefs = HashMap::new();
    prefs.insert("typescript".to_string(), 0.95);
    prefs.insert("javascript".to_string(), 0.85);
    prefs
  };

  custom_linter.rule_overrides = {
    let mut overrides = HashMap::new();
    overrides.insert("prefer-const".to_string(), true);
    overrides.insert("no-console".to_string(), false);
    overrides
  };

  assert_eq!(custom_linter.max_suggestions, 100);
  assert_eq!(custom_linter.min_confidence, 0.9);
  assert!(!custom_linter.enable_auto_fix);
  assert_eq!(custom_linter.session_id, "custom-session-123");
  assert_eq!(custom_linter.language_preferences.len(), 2);
  assert_eq!(custom_linter.rule_overrides.len(), 2);
}

/// Test suggestion filtering logic (without AI calls)
#[test]
fn test_suggestion_filtering_logic() {
  let suggestions = vec![
    AiSuggestion {
      line: 1,
      column: 0,
      message: "High confidence suggestion".to_string(),
      severity: SuggestionSeverity::Error,
      rule_id: Some("rule1".to_string()),
      suggested_fix: Some("Fix 1".to_string()),
      category: SuggestionCategory::Compilation,
      confidence: 0.95,
      auto_fixable: true,
      impact_score: 9,
      related_suggestions: vec![],
    },
    AiSuggestion {
      line: 2,
      column: 0,
      message: "Low confidence suggestion".to_string(),
      severity: SuggestionSeverity::Warning,
      rule_id: Some("rule2".to_string()),
      suggested_fix: Some("Fix 2".to_string()),
      category: SuggestionCategory::Style,
      confidence: 0.5, // Below default threshold
      auto_fixable: false,
      impact_score: 3,
      related_suggestions: vec![],
    },
    AiSuggestion {
      line: 3,
      column: 0,
      message: "Medium confidence suggestion".to_string(),
      severity: SuggestionSeverity::Info,
      rule_id: Some("rule3".to_string()),
      suggested_fix: Some("Fix 3".to_string()),
      category: SuggestionCategory::Performance,
      confidence: 0.85,
      auto_fixable: true,
      impact_score: 7,
      related_suggestions: vec![],
    },
  ];

  // Test filtering by confidence threshold
  let high_confidence: Vec<_> =
    suggestions.iter().filter(|s| s.confidence >= 0.8).collect();
  assert_eq!(high_confidence.len(), 2);

  // Test sorting by severity
  let mut sorted = suggestions.clone();
  sorted.sort_by(|a, b| a.severity.cmp(&b.severity));
  assert_eq!(sorted[0].severity, SuggestionSeverity::Error);
  assert_eq!(sorted[1].severity, SuggestionSeverity::Warning);
  assert_eq!(sorted[2].severity, SuggestionSeverity::Info);
}

/// Test rule override logic
#[test]
fn test_rule_override_logic() {
  let mut rule_overrides = HashMap::new();
  rule_overrides.insert("no-console".to_string(), false); // Disable rule
  rule_overrides.insert("prefer-const".to_string(), true); // Enable rule

  let console_suggestion = AiSuggestion {
    line: 10,
    column: 0,
    message: "Remove console.log".to_string(),
    severity: SuggestionSeverity::Warning,
    rule_id: Some("no-console".to_string()),
    suggested_fix: Some("Remove or replace with logger".to_string()),
    category: SuggestionCategory::BestPractices,
    confidence: 0.8,
    auto_fixable: true,
    impact_score: 4,
    related_suggestions: vec![],
  };

  let const_suggestion = AiSuggestion {
    line: 15,
    column: 0,
    message: "Use const instead of let".to_string(),
    severity: SuggestionSeverity::Info,
    rule_id: Some("prefer-const".to_string()),
    suggested_fix: Some("Change let to const".to_string()),
    category: SuggestionCategory::BestPractices,
    confidence: 0.85,
    auto_fixable: true,
    impact_score: 3,
    related_suggestions: vec![],
  };

  // Test rule filtering logic
  let console_enabled = console_suggestion
    .rule_id
    .as_ref()
    .and_then(|rule| rule_overrides.get(rule))
    .copied()
    .unwrap_or(true);
  assert!(!console_enabled);

  let const_enabled = const_suggestion
    .rule_id
    .as_ref()
    .and_then(|rule| rule_overrides.get(rule))
    .copied()
    .unwrap_or(true);
  assert!(const_enabled);
}

/// Test suggestion grouping by rule
#[test]
fn test_suggestion_grouping_logic() {
  let suggestions = vec![
    AiSuggestion {
      line: 10,
      column: 0,
      message: "Missing semicolon".to_string(),
      severity: SuggestionSeverity::Error,
      rule_id: Some("semicolon".to_string()),
      suggested_fix: Some("Add semicolon".to_string()),
      category: SuggestionCategory::Style,
      confidence: 0.9,
      auto_fixable: true,
      impact_score: 2,
      related_suggestions: vec![],
    },
    AiSuggestion {
      line: 15,
      column: 0,
      message: "Missing semicolon".to_string(),
      severity: SuggestionSeverity::Error,
      rule_id: Some("semicolon".to_string()),
      suggested_fix: Some("Add semicolon".to_string()),
      category: SuggestionCategory::Style,
      confidence: 0.9,
      auto_fixable: true,
      impact_score: 2,
      related_suggestions: vec![],
    },
    AiSuggestion {
      line: 20,
      column: 0,
      message: "Prefer const".to_string(),
      severity: SuggestionSeverity::Info,
      rule_id: Some("prefer-const".to_string()),
      suggested_fix: Some("Change to const".to_string()),
      category: SuggestionCategory::BestPractices,
      confidence: 0.8,
      auto_fixable: true,
      impact_score: 3,
      related_suggestions: vec![],
    },
  ];

  let mut grouped: HashMap<String, Vec<&AiSuggestion>> = HashMap::new();
  for suggestion in &suggestions {
    if let Some(rule_id) = &suggestion.rule_id {
      grouped.entry(rule_id.clone()).or_default().push(suggestion);
    }
  }

  assert_eq!(grouped.len(), 2);
  assert_eq!(grouped.get("semicolon").unwrap().len(), 2);
  assert_eq!(grouped.get("prefer-const").unwrap().len(), 1);
}

/// Test language-specific confidence adjustment
#[test]
fn test_language_confidence_adjustment() {
  let mut language_preferences = HashMap::new();
  language_preferences.insert("typescript".to_string(), 0.95);
  language_preferences.insert("javascript".to_string(), 0.85);

  let base_confidence = 0.9;

  // Test TypeScript adjustment
  let ts_multiplier = language_preferences
    .get("typescript")
    .copied()
    .unwrap_or(1.0);
  let ts_adjusted = base_confidence * ts_multiplier;
  assert!((ts_adjusted - 0.855_f32).abs() < 0.001_f32); // 0.9 * 0.95

  // Test JavaScript adjustment
  let js_multiplier = language_preferences
    .get("javascript")
    .copied()
    .unwrap_or(1.0);
  let js_adjusted = base_confidence * js_multiplier;
  assert!((js_adjusted - 0.765_f32).abs() < 0.001_f32); // 0.9 * 0.85

  // Test unknown language (no adjustment)
  let unknown_multiplier =
    language_preferences.get("python").copied().unwrap_or(1.0);
  let unknown_adjusted = base_confidence * unknown_multiplier;
  assert!((unknown_adjusted - 0.9_f32).abs() < f32::EPSILON);
}

/// Test batch processing performance with large datasets
#[test]
fn test_large_batch_processing() {
  let mut large_suggestions = Vec::new();
  for i in 0..1000 {
    large_suggestions.push(AiSuggestion {
      line: i + 1,
      column: 0,
      message: format!("Suggestion {}", i),
      severity: if i % 3 == 0 {
        SuggestionSeverity::Error
      } else if i % 2 == 0 {
        SuggestionSeverity::Warning
      } else {
        SuggestionSeverity::Info
      },
      rule_id: Some(format!("rule_{}", i % 10)),
      suggested_fix: Some(format!("Fix {}", i)),
      category: SuggestionCategory::Style,
      confidence: 0.5 + ((i % 50) as f32 / 100.0),
      auto_fixable: i % 4 == 0,
      impact_score: (i % 10) + 1,
      related_suggestions: vec![],
    });
  }

  let min_confidence = 0.7;
  let filtered: Vec<_> = large_suggestions
    .into_iter()
    .filter(|s| s.confidence >= min_confidence)
    .collect();

  assert!(filtered.len() < 1000); // Some should be filtered
  assert!(filtered.len() > 200); // But not too many

  // Test that high confidence suggestions are preserved
  let high_confidence_count =
    filtered.iter().filter(|s| s.confidence >= 0.9).count();
  assert!(high_confidence_count > 0);
}

/// Test OXC parser error conversion and metadata preservation
#[test]
fn test_oxc_parser_error_conversion() {
  use moon_shine::oxc_unified_workflow::OxcUnifiedWorkflow;
  use oxc_allocator::Allocator;
  use oxc_parser::Parser;
  use oxc_span::SourceType;

  // Intentionally invalid JS code to trigger a parser error
  let code = "function () { let = ; }";
  let allocator = Allocator::default();
  let source_type = SourceType::default().with_module(true).with_jsx(true);

  let parse_result = Parser::new(&allocator, code, source_type).parse();
  assert!(!parse_result.errors.is_empty(), "Should produce parser errors");

  let workflow = OxcUnifiedWorkflow::default();
  let diag = workflow.convert_parser_error(parse_result.errors[0].clone());

  assert_eq!(diag.severity, moon_shine::oxc_unified_workflow::DiagnosticSeverity::Error);
  assert_eq!(diag.rule_name.as_deref(), Some("parser"));
  assert!(diag.message.contains("error") || diag.message.contains("invalid"), "Message should describe error");
  assert!(diag.span.line > 0, "Line should be > 0");
  assert!(diag.span.column > 0, "Column should be > 0");
}