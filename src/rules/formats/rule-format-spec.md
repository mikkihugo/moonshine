# OXC Rule Format Specification

## Overview

This document defines the complete format specification for OXC-compatible rules in the moon-shine WASM extension. All rules must conform to this specification to ensure consistency, maintainability, and proper integration.

## 🏗️ Core Rule Structure

### 1. Rule Definition

```rust
/// Brief description of what the rule detects and prevents
pub struct RuleName;

impl RuleName {
    /// Kebab-case rule identifier used in configuration and reporting
    pub const NAME: &'static str = "rule-name";

    /// Category classification for rule organization and filtering
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;

    /// Indicates the type of fix this rule can provide
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}
```

### 2. Core Trait Implementation

```rust
impl WasmRule for RuleName {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    /// Main rule logic - analyzes code and returns diagnostics
    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Pattern detection logic here
        if code.contains("problematic_pattern") {
            diagnostics.push(create_rule_diagnostic(line, column));
        }

        diagnostics
    }
}
```

### 3. AI Enhancement Implementation

```rust
impl EnhancedWasmRule for RuleName {
    /// Provides AI-powered suggestions for rule violations
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Specific improvement suggestion".to_string(),
            confidence: 0.95, // 0.0 to 1.0
            auto_fixable: true, // Whether fix can be applied automatically
        }).collect()
    }
}
```

## 📋 Required Components

### 1. Naming Conventions

#### Rule Names
- **Format**: `PascalCase` for struct names, `kebab-case` for NAME constant
- **Pattern**: `[Action][Subject][Qualifier]`
- **Examples**:
  - `RequireTypeScript` → `"require-typescript"`
  - `NoUnusedImports` → `"no-unused-imports"`
  - `PreferArrowFunctions` → `"prefer-arrow-functions"`

#### Action Prefixes
- **`Require`**: Mandates presence of something
- **`No`**: Prohibits specific patterns
- **`Prefer`**: Suggests better alternatives
- **`Enforce`**: Strict compliance requirements
- **`Prevent`**: Blocks dangerous patterns

### 2. Category Classification

```rust
pub enum WasmRuleCategory {
    /// Logic errors, type issues, definite bugs
    Correctness,

    /// Potentially problematic patterns that should be reviewed
    Suspicious,

    /// Strict style enforcement and nitpicky issues
    Pedantic,

    /// Performance optimization opportunities
    Perf,

    /// Security and safety restrictions
    Restriction,

    /// Code formatting and consistency preferences
    Style,

    /// Experimental or unstable rules under development
    Nursery,
}
```

### 3. Fix Status Classification

```rust
pub enum WasmFixStatus {
    /// Safe automatic fixes that don't change behavior
    Fix,

    /// Automatic fixes that might change runtime behavior
    FixDangerous,

    /// Manual intervention recommended - no automatic fix
    Suggestion,

    /// Context-dependent fixes requiring human judgment
    ConditionalFixSuggestion,
}
```

## 🔍 Diagnostic Format

### WasmRuleDiagnostic Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    /// Rule identifier matching the NAME constant
    pub rule_name: String,

    /// Human-readable description of the issue
    pub message: String,

    /// Line number where the issue occurs (1-indexed)
    pub line: usize,

    /// Column number where the issue occurs (0-indexed)
    pub column: usize,

    /// Severity level for the issue
    pub severity: String, // "error", "warning", "info"

    /// Optional fix suggestion for immediate resolution
    pub fix_suggestion: Option<String>,
}
```

### Diagnostic Creation Pattern

```rust
fn create_rule_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RuleName::NAME.to_string(),
        message: "Clear description of what's wrong".to_string(),
        line,
        column,
        severity: "error".to_string(), // or "warning", "info"
        fix_suggestion: Some("Specific fix instruction".to_string()),
    }
}
```

## 🤖 AI Enhancement Format

### AiSuggestion Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    /// Rule identifier for the suggestion
    pub rule_name: String,

    /// AI-generated improvement suggestion
    pub suggestion: String,

    /// Confidence score from 0.0 (low) to 1.0 (high)
    pub confidence: f32,

    /// Whether the suggestion can be auto-applied
    pub auto_fixable: bool,
}
```

### AI Enhancement Guidelines

#### Confidence Scoring
- **0.95-1.0**: Definitive improvements with minimal risk
- **0.85-0.94**: High confidence with some context dependency
- **0.70-0.84**: Good suggestions requiring validation
- **0.50-0.69**: Exploratory suggestions needing review
- **Below 0.50**: Low confidence, primarily educational

#### Suggestion Quality Standards
- **Specific**: Provide exact steps or code changes
- **Contextual**: Consider the surrounding code environment
- **Educational**: Explain why the change improves the code
- **Actionable**: Give clear, implementable instructions

## 📝 Documentation Requirements

### 1. Module-Level Documentation

```rust
//! # Module Title
//!
//! Brief description of the module's purpose and scope.
//!
//! ## Rule Categories:
//! - **Category 1**: Description of rules in this category
//! - **Category 2**: Description of rules in this category
//!
//! ## Target Patterns:
//! Description of what code patterns this module analyzes.
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.
```

### 2. Rule-Level Documentation

```rust
/// Brief one-line description of what the rule does
///
/// Longer explanation of:
/// - Why this rule exists
/// - What problems it prevents
/// - When it should be applied
/// - Examples of violations and fixes
pub struct RuleName;
```

### 3. Function Documentation

```rust
/// Creates a diagnostic for [specific violation type]
///
/// # Arguments
/// * `line` - Line number where the violation occurs
/// * `column` - Column number where the violation occurs
///
/// # Returns
/// A diagnostic with appropriate message and fix suggestion
fn create_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic
```

## 🧪 Testing Requirements

### 1. Basic Detection Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_detection() {
        let code = r#"problematic code example"#;
        let rule = RuleName;
        let diagnostics = rule.run(code);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RuleName::NAME);
        assert_eq!(diagnostics[0].severity, "error");
    }

    #[test]
    fn test_no_false_positives() {
        let code = r#"correct code example"#;
        let rule = RuleName;
        let diagnostics = rule.run(code);

        assert_eq!(diagnostics.len(), 0);
    }
}
```

### 2. AI Enhancement Tests

```rust
#[test]
fn test_ai_enhancement() {
    let code = r#"problematic code"#;
    let rule = RuleName;
    let diagnostics = rule.run(code);
    let suggestions = rule.ai_enhance(code, &diagnostics);

    assert_eq!(suggestions.len(), 1);
    assert!(suggestions[0].confidence > 0.8);
    assert!(suggestions[0].suggestion.len() > 10);
}
```

### 3. Edge Case Tests

```rust
#[test]
fn test_edge_cases() {
    let edge_cases = vec![
        "", // Empty string
        "//", // Comments only
        "/* */", // Block comments
        "\n\n\n", // Whitespace only
    ];

    let rule = RuleName;
    for case in edge_cases {
        let diagnostics = rule.run(case);
        // Should not panic or produce false positives
        assert!(diagnostics.len() <= 1);
    }
}
```

## 🔧 Performance Requirements

### 1. Execution Time
- **Target**: < 1ms per rule per 1000 lines of code
- **Maximum**: < 10ms per rule per 1000 lines of code
- **Measurement**: Use `#[bench]` for performance testing

### 2. Memory Usage
- **Target**: < 1MB heap allocation per rule module
- **Maximum**: < 5MB heap allocation per rule module
- **Pattern**: Prefer stack allocation and string slicing

### 3. WASM Compatibility
- **No external dependencies** except approved crates
- **No file system access** beyond provided code string
- **No network operations** or external resource access
- **Deterministic execution** for consistent results

## 🔒 Security Considerations

### 1. Input Validation
- Assume all input code may be malicious
- Prevent infinite loops in pattern matching
- Limit recursion depth for complex analysis
- Validate input length and complexity

### 2. Memory Safety
- Use safe Rust patterns exclusively
- Avoid unsafe blocks unless absolutely necessary
- Prevent buffer overflows in string processing
- Manage memory allocation carefully in WASM

### 3. Information Disclosure
- Don't expose sensitive information in diagnostics
- Sanitize error messages and suggestions
- Avoid logging user code content
- Keep diagnostic messages generic when possible

## 📊 Integration Requirements

### 1. Registry Integration

```rust
// In unified_rule_registry.rs register_all_rules()
self.register_rule(RuleName {});
```

### 2. Module Export

```rust
// In lib.rs
pub mod oxc_module_name_rules;
```

### 3. Import Requirements

```rust
// Required imports for all rule modules
use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};
```

---

**Version**: 1.0
**Last Updated**: Current Date
**Status**: Production Standard
**Compliance**: All active rules must follow this specification