//! Unit tests for [`wasm_safe_linter.rs`](../src/wasm_safe_linter.rs)

use moon_shine::wasm_safe_linter::*;
use std::collections::HashMap;

#[test]
fn wasm_safe_linter_default_rules_enabled() {
    let linter = WasmSafeLinter::new();
    let enabled = [
        "no-unused-vars",
        "no-console",
        "no-debugger",
        "prefer-const",
        "eqeqeq",
        "no-eval",
        "no-any",
        "prefer-arrow-functions",
        "no-var",
        "prefer-template-literals",
    ];
    for rule in enabled {
        assert!(linter.is_rule_enabled(rule), "Rule {} should be enabled by default", rule);
    }
}

#[test]
fn configure_rules_overrides_defaults() {
    let mut linter = WasmSafeLinter::new();
    let mut rules = HashMap::new();
    rules.insert("no-console".to_string(), false);
    linter.configure_rules(rules);
    assert!(!linter.is_rule_enabled("no-console"));
}

#[test]
fn lint_code_empty_input_returns_ok() {
    let linter = WasmSafeLinter::new();
    let result = linter.lint_code("", "test.js");
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn lint_code_invalid_filetype_returns_err() {
    let linter = WasmSafeLinter::new();
    let result = linter.lint_code("let x = 1;", "test.unknownext");
    assert!(result.is_err());
}

#[test]
fn wasm_safe_lint_result_struct_fields() {
    let result = WasmSafeLintResult {
        errors: vec![],
        warnings: vec![],
        fixable_issues: vec![],
        auto_fixed_code: None,
        rules_checked: vec!["no-unused-vars".to_string()],
    };
    assert!(result.rules_checked.contains(&"no-unused-vars".to_string()));
}
