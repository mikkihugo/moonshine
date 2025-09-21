# Testing Standards for OXC Rules

## Overview

This document defines comprehensive testing standards for OXC-compatible rules in the moon-shine WASM extension. All rules must pass these testing requirements to ensure reliability, performance, and maintainability.

## 🎯 Testing Objectives

### Primary Goals
- **Correctness**: Rules detect intended patterns accurately
- **No False Positives**: Rules don't trigger on valid code
- **Performance**: Rules execute efficiently in WASM environment
- **Robustness**: Rules handle edge cases gracefully
- **AI Quality**: Enhancement suggestions are meaningful and accurate

### Quality Metrics
- **Code Coverage**: 100% line coverage for rule logic
- **Test Coverage**: Minimum 5 test cases per rule
- **Performance**: < 1ms execution per 1000 lines of code
- **Memory**: < 1MB heap allocation per rule module

## 📋 Required Test Categories

### 1. Basic Detection Tests

```rust
#[test]
fn test_rule_detection() {
    let code = r#"
        // Code that should trigger the rule
        problematic_pattern();
    "#;
    let rule = RuleName;
    let diagnostics = rule.run(code);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_name, RuleName::NAME);
    assert_eq!(diagnostics[0].severity, "error");
    assert!(diagnostics[0].message.len() > 10);
    assert!(diagnostics[0].fix_suggestion.is_some());
}
```

### 2. False Positive Prevention

```rust
#[test]
fn test_no_false_positives() {
    let valid_cases = vec![
        r#"// Valid pattern 1"#,
        r#"// Valid pattern 2"#,
        r#"// Edge case that shouldn't trigger"#,
    ];

    let rule = RuleName;
    for code in valid_cases {
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 0, "False positive in: {}", code);
    }
}
```

### 3. Multiple Violations

```rust
#[test]
fn test_multiple_violations() {
    let code = r#"
        problematic_pattern(); // First violation
        some_other_code();
        problematic_pattern(); // Second violation
    "#;
    let rule = RuleName;
    let diagnostics = rule.run(code);

    assert_eq!(diagnostics.len(), 2);

    // Verify all diagnostics are from this rule
    for diagnostic in &diagnostics {
        assert_eq!(diagnostic.rule_name, RuleName::NAME);
    }

    // Verify line numbers are different
    assert_ne!(diagnostics[0].line, diagnostics[1].line);
}
```

### 4. Edge Case Handling

```rust
#[test]
fn test_edge_cases() {
    let edge_cases = vec![
        ("", "empty string"),
        ("//", "comment only"),
        ("/* */", "block comment"),
        ("\n\n\n", "whitespace only"),
        ("a".repeat(10000), "very long string"),
        ("unicode: 你好世界", "unicode characters"),
        ("nested /* /* */ */", "nested comments"),
    ];

    let rule = RuleName;
    for (code, description) in edge_cases {
        let result = std::panic::catch_unwind(|| {
            rule.run(&code)
        });

        assert!(result.is_ok(), "Rule panicked on: {}", description);

        if let Ok(diagnostics) = result {
            // Sanity check - shouldn't produce excessive diagnostics
            assert!(diagnostics.len() < 100, "Too many diagnostics for: {}", description);
        }
    }
}
```

### 5. AI Enhancement Quality

```rust
#[test]
fn test_ai_enhancement_quality() {
    let code = r#"problematic_pattern();"#;
    let rule = RuleName;
    let diagnostics = rule.run(code);
    let suggestions = rule.ai_enhance(code, &diagnostics);

    assert_eq!(suggestions.len(), diagnostics.len());

    for suggestion in &suggestions {
        // Confidence should be reasonable
        assert!(suggestion.confidence >= 0.5 && suggestion.confidence <= 1.0);

        // Suggestion should be meaningful
        assert!(suggestion.suggestion.len() > 20);
        assert!(!suggestion.suggestion.is_empty());

        // Should match the rule
        assert_eq!(suggestion.rule_name, RuleName::NAME);
    }
}
```

### 6. Contextual AI Enhancement

```rust
#[test]
fn test_contextual_ai_enhancement() {
    let contexts = vec![
        ("problematic_pattern() in production", "production context"),
        ("problematic_pattern() in test", "test context"),
        ("problematic_pattern() // legacy code", "legacy context"),
    ];

    let rule = RuleName;

    for (code, context_desc) in contexts {
        let diagnostics = rule.run(code);
        if !diagnostics.is_empty() {
            let suggestions = rule.ai_enhance(code, &diagnostics);

            // AI should provide context-aware suggestions
            assert!(!suggestions.is_empty(), "No suggestions for: {}", context_desc);

            // Suggestions should reference the context when relevant
            let suggestion_text = &suggestions[0].suggestion.to_lowercase();
            if code.contains("production") {
                assert!(
                    suggestion_text.contains("production") ||
                    suggestion_text.contains("deployment") ||
                    suggestion_text.contains("live"),
                    "Production context not reflected in suggestion"
                );
            }
        }
    }
}
```

### 7. Performance Tests

```rust
#[test]
fn test_performance() {
    let small_code = "normal code\n".repeat(100);
    let medium_code = "normal code\n".repeat(1000);
    let large_code = "normal code\n".repeat(10000);

    let rule = RuleName;

    // Small code should be very fast
    let start = std::time::Instant::now();
    let _diagnostics = rule.run(&small_code);
    assert!(start.elapsed().as_millis() < 10);

    // Medium code should still be fast
    let start = std::time::Instant::now();
    let _diagnostics = rule.run(&medium_code);
    assert!(start.elapsed().as_millis() < 50);

    // Large code should complete within reasonable time
    let start = std::time::Instant::now();
    let _diagnostics = rule.run(&large_code);
    assert!(start.elapsed().as_millis() < 500);
}
```

### 8. Memory Usage Tests

```rust
#[test]
fn test_memory_usage() {
    let large_code = "function test() { return 'hello'; }\n".repeat(1000);
    let rule = RuleName;

    // Run multiple times to check for memory leaks
    for _ in 0..100 {
        let _diagnostics = rule.run(&large_code);
    }

    // This is a basic test - in practice, you might use more sophisticated
    // memory profiling tools for WASM environments
}
```

### 9. Framework-Specific Tests

```rust
#[test]
fn test_framework_specific_patterns() {
    // Example for React-specific rules
    let react_cases = vec![
        (r#"function Component() { return <div>Hello</div>; }"#, false),
        (r#"function Component() { /* problematic React pattern */ }"#, true),
    ];

    let rule = ReactSpecificRule;

    for (code, should_trigger) in react_cases {
        let diagnostics = rule.run(code);
        if should_trigger {
            assert!(diagnostics.len() > 0, "Should trigger on: {}", code);
        } else {
            assert_eq!(diagnostics.len(), 0, "Should not trigger on: {}", code);
        }
    }
}
```

### 10. Integration Tests

```rust
#[test]
fn test_registry_integration() {
    let registry = UnifiedRuleRegistry::new();

    // Rule should be registered
    assert!(registry.get_rule(RuleName::NAME).is_some());

    // Rule should be in correct category
    let category_rules = registry.get_rules_by_category(RuleName::CATEGORY);
    assert!(category_rules.iter().any(|r| r.name() == RuleName::NAME));

    // Rule should be enabled by default
    assert!(registry.is_rule_enabled(RuleName::NAME));
}
```

## 🔧 Test Organization Patterns

### Module Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod detection_tests {
        use super::*;
        // Basic detection tests
    }

    mod false_positive_tests {
        use super::*;
        // False positive prevention tests
    }

    mod edge_case_tests {
        use super::*;
        // Edge case handling tests
    }

    mod ai_enhancement_tests {
        use super::*;
        // AI enhancement quality tests
    }

    mod performance_tests {
        use super::*;
        // Performance and memory tests
    }
}
```

### Test Data Organization

```rust
// Test data constants
const VALID_PATTERNS: &[&str] = &[
    r#"// Valid pattern 1"#,
    r#"// Valid pattern 2"#,
];

const INVALID_PATTERNS: &[&str] = &[
    r#"// Invalid pattern 1"#,
    r#"// Invalid pattern 2"#,
];

const EDGE_CASES: &[(&str, &str)] = &[
    ("", "empty"),
    ("//", "comment"),
    ("\n", "newline"),
];
```

## 📊 Test Metrics and Reporting

### Coverage Requirements

- **Line Coverage**: 100% of rule logic
- **Branch Coverage**: 95% of conditional paths
- **Test Cases**: Minimum 5 per rule
- **Documentation**: Test purpose and expected behavior

### Performance Benchmarks

```rust
#[bench]
fn bench_rule_performance(b: &mut Bencher) {
    let code = include_str!("../test_data/large_file.js");
    let rule = RuleName;

    b.iter(|| {
        black_box(rule.run(code))
    });
}
```

### Quality Gates

- All tests must pass
- Performance benchmarks must meet targets
- Memory usage must stay within limits
- AI enhancement confidence scores must be reasonable
- No panics on any input

## 🚀 Testing Best Practices

### 1. Test Naming

```rust
// Good: Descriptive and specific
#[test]
fn test_require_typescript_detects_js_files_in_ts_project() {}

// Bad: Vague and generic
#[test]
fn test_rule() {}
```

### 2. Test Independence

```rust
// Each test should be independent
#[test]
fn test_independent_case_1() {
    let rule = RuleName; // Create fresh instance
    // Test logic
}

#[test]
fn test_independent_case_2() {
    let rule = RuleName; // Create fresh instance
    // Different test logic
}
```

### 3. Clear Assertions

```rust
// Good: Clear assertion with context
assert_eq!(
    diagnostics.len(),
    2,
    "Expected 2 violations but got {}: {:?}",
    diagnostics.len(),
    diagnostics
);

// Bad: Unclear assertion
assert!(diagnostics.len() > 0);
```

### 4. Test Documentation

```rust
/// Tests that the rule correctly identifies unused imports
/// in TypeScript files while ignoring type-only imports
#[test]
fn test_unused_imports_typescript() {
    // Test implementation
}
```

## 🔍 Debugging Test Failures

### Common Issues

1. **False Positives**: Rule triggers on valid code
   - Review pattern matching logic
   - Add exception cases
   - Test with more diverse code samples

2. **False Negatives**: Rule misses problematic code
   - Expand pattern detection
   - Test with variations of the problem pattern
   - Consider edge cases in pattern matching

3. **Performance Issues**: Rule is too slow
   - Optimize pattern matching algorithms
   - Reduce string allocations
   - Use more efficient data structures

4. **Memory Issues**: Rule uses too much memory
   - Avoid storing large strings
   - Use string slicing instead of cloning
   - Clean up temporary allocations

### Debug Utilities

```rust
#[cfg(test)]
fn debug_rule_output(rule: &dyn WasmRule, code: &str) {
    let diagnostics = rule.run(code);
    println!("Code:\n{}", code);
    println!("Diagnostics: {:#?}", diagnostics);
}
```

## 📋 Test Checklist

Before submitting a new rule or changes:

- [ ] All detection tests pass
- [ ] No false positives on valid code
- [ ] Edge cases handled gracefully
- [ ] AI enhancement provides quality suggestions
- [ ] Performance meets requirements (< 1ms per 1000 lines)
- [ ] Memory usage is reasonable (< 1MB per module)
- [ ] Integration tests pass
- [ ] Documentation is clear and complete
- [ ] Test coverage is 100%
- [ ] All tests are independent and repeatable

---

**Version**: 1.0
**Last Updated**: Current Date
**Compliance**: All rules must meet these testing standards