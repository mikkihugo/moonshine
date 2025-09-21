//! # MoonShine Rule Engine Integration Test
//!
//! Tests the complete rule engine with OXC semantic analysis
//!
//! @category testing
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

#[cfg(test)]
mod tests {
    use crate::rules::engine::MoonShineRuleEngine;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    #[test]
    fn test_rule_engine_with_catch_block() {
        let engine = MoonShineRuleEngine::new();

        let code = r#"
            try {
                doSomething();
            } catch (e) {
                // Silent catch - should trigger C029
            }
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = engine.check_all_rules(
            &parse_result.program,
            &semantic_result.semantic,
            code,
            "test.js"
        );

        // Should detect the empty catch block
        assert!(!issues.is_empty(), "Should detect empty catch block");
        assert!(issues.iter().any(|issue| issue.rule_name == "C029"), "Should trigger C029 rule");
    }

    #[test]
    fn test_rule_engine_with_boolean_naming() {
        let engine = MoonShineRuleEngine::new();

        let code = r#"
            const active = true;
            const isEnabled = false;
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = engine.check_all_rules(
            &parse_result.program,
            &semantic_result.semantic,
            code,
            "test.js"
        );

        // Should detect the boolean naming issue with 'active'
        let boolean_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule_name == "C042")
            .collect();

        assert!(!boolean_issues.is_empty(), "Should detect boolean naming issue");
        assert!(boolean_issues.iter().any(|issue| issue.message.contains("active")),
               "Should flag 'active' variable");
    }

    #[test]
    fn test_rule_engine_with_generic_error() {
        let engine = MoonShineRuleEngine::new();

        let code = r#"
            throw new Error("Something went wrong");
            throw new ValidationError("Custom error is OK");
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = engine.check_all_rules(
            &parse_result.program,
            &semantic_result.semantic,
            code,
            "test.js"
        );

        // Should detect only the generic Error, not ValidationError
        let error_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule_name == "C030")
            .collect();

        assert_eq!(error_issues.len(), 1, "Should detect exactly one generic Error usage");
    }

    #[test]
    fn test_rule_engine_comprehensive() {
        let engine = MoonShineRuleEngine::new();

        let code = r#"
            class Example {
                constructor(name, active, config) {
                    this.name = name;
                    this.active = active;  // C042: Boolean naming
                    this.config = config;

                    // C017: Complex constructor logic
                    this.setupDatabase();
                    this.initializeLogging();
                    this.validateInputs();
                    this.startServer();
                }

                process() {
                    try {
                        this.doWork();
                    } catch (e) {
                        // C029: Silent catch
                    }

                    throw new Error("Generic error");  // C030: Generic error
                }
            }
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = engine.check_all_rules(
            &parse_result.program,
            &semantic_result.semantic,
            code,
            "test.js"
        );

        println!("Found {} issues:", issues.len());
        for issue in &issues {
            println!("  {} - {}", issue.rule_name, issue.message);
        }

        // Should detect multiple rule violations
        assert!(!issues.is_empty(), "Should detect multiple rule violations");

        // Check for specific rule types
        let rule_types: std::collections::HashSet<_> = issues.iter()
            .map(|issue| &issue.rule_name)
            .collect();

        println!("Detected rule types: {:?}", rule_types);

        // We expect at least some of our rules to trigger
        assert!(rule_types.len() > 0, "Should detect at least one type of rule violation");
    }
}