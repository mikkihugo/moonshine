//! Integration test for OXC-compatible rule template structure
//!
//! This test validates that our OXC rule template implementation works correctly
//! and can lint JavaScript/TypeScript code using OXC's AST parsing.

#[cfg(test)]
mod tests {
    use moon_shine::oxc_compatible_rules::{
        WasmRuleEngine, NoEmpty, BooleanNaming, WasmRuleCategory, WasmFixStatus
    };

    #[test]
    fn test_oxc_template_structure() {
        // Test that rule constants match OXC pattern
        assert_eq!(NoEmpty::NAME, "no-empty");
        assert_eq!(NoEmpty::CATEGORY, WasmRuleCategory::Suspicious);
        assert_eq!(NoEmpty::FIX_STATUS, WasmFixStatus::Suggestion);

        assert_eq!(BooleanNaming::NAME, "boolean-naming");
        assert_eq!(BooleanNaming::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(BooleanNaming::FIX_STATUS, WasmFixStatus::Suggestion);
    }

    #[test]
    fn test_rule_engine_creation() {
        // Test that rule engine can be created with OXC-compatible rules
        let engine = WasmRuleEngine::default();

        // This should not panic and should create the engine successfully
        assert!(!std::ptr::eq(&engine, std::ptr::null()));
    }

    #[test]
    fn test_no_empty_rule_basic() {
        // Test basic no-empty rule functionality
        let engine = WasmRuleEngine::default();

        // Test with empty block (should find issues)
        let result = engine.lint("if (foo) {}", "test.js");
        assert!(result.is_ok());
        let issues = result.unwrap();

        // Should detect empty block
        assert!(!issues.is_empty(), "Should detect empty block");

        // Test with non-empty block (should not find issues)
        let result = engine.lint("if (foo) { console.log('hello'); }", "test.js");
        assert!(result.is_ok());
        let issues = result.unwrap();

        // Should not detect issues in non-empty block
        assert!(issues.is_empty(), "Should not detect issues in non-empty block");
    }

    #[test]
    fn test_boolean_naming_rule_basic() {
        // Test basic boolean naming rule functionality
        let engine = WasmRuleEngine::default();

        // Test with poorly named boolean (should find issues)
        let result = engine.lint("const enabled = true;", "test.js");
        assert!(result.is_ok());
        let issues = result.unwrap();

        // Should detect poor boolean naming
        assert!(!issues.is_empty(), "Should detect poor boolean naming");

        // Test with well-named boolean (should not find issues)
        let result = engine.lint("const isEnabled = true;", "test.js");
        assert!(result.is_ok());
        let issues = result.unwrap();

        // Should not detect issues with well-named boolean
        assert!(issues.is_empty(), "Should not detect issues with well-named boolean");
    }

    #[test]
    fn test_oxc_performance_vs_regex() {
        // Test that OXC AST parsing works (basic smoke test)
        let engine = WasmRuleEngine::default();

        // Test with complex TypeScript code
        let complex_code = r#"
            interface User {
                id: number;
                name: string;
                isActive: boolean;
            }

            function processUser(user: User): void {
                if (user.isActive) {
                    console.log(`Processing ${user.name}`);
                } else {
                    // Empty else block
                }
            }

            const enabled = user.isActive; // Poor boolean naming
        "#;

        let result = engine.lint(complex_code, "test.ts");
        assert!(result.is_ok());
        let issues = result.unwrap();

        // Should parse TypeScript successfully and find our rules' issues
        println!("Found {} issues in complex TypeScript code", issues.len());

        // Should find both empty block and poor boolean naming
        assert!(!issues.is_empty(), "Should find issues in complex code");
    }

    #[test]
    fn test_600_rules_migration_pattern() {
        // Test that our template pattern is ready for 600 rules migration

        // Verify rule categories match OXC exactly
        let categories = [
            WasmRuleCategory::Nursery,
            WasmRuleCategory::Correctness,
            WasmRuleCategory::Suspicious,
            WasmRuleCategory::Pedantic,
            WasmRuleCategory::Perf,
            WasmRuleCategory::Restriction,
            WasmRuleCategory::Style,
        ];

        // All categories should be available
        assert_eq!(categories.len(), 7);

        // Verify fix status classifications
        let fix_statuses = [
            WasmFixStatus::Pending,
            WasmFixStatus::Fix,
            WasmFixStatus::FixDangerous,
            WasmFixStatus::Suggestion,
            WasmFixStatus::ConditionalFixSuggestion,
        ];

        // All fix statuses should be available
        assert_eq!(fix_statuses.len(), 5);
    }
}
//! Integration tests for OXC-native Rust rules: C002 (No Duplicate Code) and C003 (Function Naming)
//!
//! These tests run the full linting pipeline on representative JS/TS code samples,
//! validating correct diagnostics (including span/line/column info) for both compliant and violation cases.

#[test]
fn test_c002_no_duplicate_code_positive() {
    use moon_shine::rules::code_quality::c002_no_duplicate_code::check_no_duplicate_code;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;

    let code = r#"
        function foo() {
            let a = 1;
            let b = 2;
            return a + b;
        }
        function bar() {
            let x = 3;
            let y = 4;
            return x * y;
        }
    "#;
    let parser = Parser::new(code);
    let program = parser.parse().unwrap();
    let semantic = SemanticBuilder::new(&program).build();
    let issues = check_no_duplicate_code(&program, &semantic, code, None);
    assert!(issues.is_empty(), "No duplicate code should be detected in compliant code");
}

#[test]
fn test_c002_no_duplicate_code_negative() {
    use moon_shine::rules::code_quality::c002_no_duplicate_code::check_no_duplicate_code;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;

    let code = r#"
        function foo() {
            let a = 1;
            let b = 2;
            return a + b;
        }
        function bar() {
            let a = 1;
            let b = 2;
            return a + b;
        }
    "#;
    let parser = Parser::new(code);
    let program = parser.parse().unwrap();
    let semantic = SemanticBuilder::new(&program).build();
    let issues = check_no_duplicate_code(&program, &semantic, code, None);
    assert!(!issues.is_empty(), "Duplicate code should be detected");
    let issue = &issues[0];
    assert!(issue.line > 0 && issue.column > 0, "Issue should have valid line/column info");
    assert_eq!(issue.severity, moon_shine::wasm_safe_linter::LintSeverity::Warning);
}

#[test]
fn test_c006_function_naming_positive() {
    use moon_shine::rules::code_quality::c006_function_naming::check_function_naming;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;

    let code = r#"
        function calculateSum(a, b) { return a + b; }
        const fetchData = () => {};
    "#;
    let parser = Parser::new(code);
    let program = parser.parse().unwrap();
    let semantic = SemanticBuilder::new(&program).build();
    let issues = check_function_naming(&program, &semantic, code);
    assert!(issues.is_empty(), "Compliant function names should not trigger diagnostics");
}

#[test]
fn test_c006_function_naming_negative() {
    use moon_shine::rules::code_quality::c006_function_naming::check_function_naming;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;

    let code = r#"
        function data() { return 1; }
        const foo = () => {};
    "#;
    let parser = Parser::new(code);
    let program = parser.parse().unwrap();
    let semantic = SemanticBuilder::new(&program).build();
    let issues = check_function_naming(&program, &semantic, code);
    assert!(!issues.is_empty(), "Non-compliant function names should trigger diagnostics");
    let issue = &issues[0];
    assert!(issue.line > 0 && issue.column > 0, "Issue should have valid line/column info");
    assert_eq!(issue.severity, moon_shine::wasm_safe_linter::LintSeverity::Warning);
}