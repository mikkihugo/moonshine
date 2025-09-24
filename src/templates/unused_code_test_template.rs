#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParserReturn};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_analyze(source: &str) -> (Allocator, Program, Semantic) {
        let allocator = Allocator::default();
        let source_type = SourceType::tsx();
        let ParserReturn { program, .. } = Parser::new(&allocator, source, source_type).parse();
        let semantic = SemanticBuilder::new().build(&program).semantic;
        (allocator, program, semantic)
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_detects_unused_variable() {
        let source = r#"
            const unused = 42;
            console.log("hello");
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        // Visit the AST
        rule.visit_program(&program);

        // In a real implementation, we would collect diagnostics
        // and assert they contain the expected violation
        // assert_eq!(diagnostics.len(), 1);
        // assert!(diagnostics[0].message.contains("unused"));
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_allows_used_variable() {
        let source = r#"
            const used = 42;
            console.log(used);
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        // Visit the AST
        rule.visit_program(&program);

        // Should not generate any diagnostics
        // assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_detects_unused_function() {
        let source = r#"
            function unusedFunction() {
                return 42;
            }
            console.log("hello");
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        rule.visit_program(&program);

        // Should detect unused function
        // assert_eq!(diagnostics.len(), 1);
        // assert!(diagnostics[0].message.contains("unusedFunction"));
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_detects_unused_import() {
        let source = r#"
            import { unusedImport } from './module';
            console.log("hello");
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        rule.visit_program(&program);

        // Should detect unused import
        // assert_eq!(diagnostics.len(), 1);
        // assert!(diagnostics[0].message.contains("unusedImport"));
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_allows_used_import() {
        let source = r#"
            import { usedImport } from './module';
            console.log(usedImport);
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        rule.visit_program(&program);

        // Should not generate any diagnostics
        // assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_handles_destructuring() {
        let source = r#"
            const { used, unused } = getData();
            console.log(used);
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        rule.visit_program(&program);

        // Should detect only the unused destructured variable
        // assert_eq!(diagnostics.len(), 1);
        // assert!(diagnostics[0].message.contains("unused"));
    }

    #[test]
    fn test_{{RULE_TEST_NAME}}_handles_function_parameters() {
        let source = r#"
            function myFunction(used, unused) {
                return used;
            }
        "#;

        let (_allocator, program, semantic) = parse_and_analyze(source);
        let mut rule = {{RULE_STRUCT_NAME}}::new();
        rule.semantic = Some(semantic);

        rule.visit_program(&program);

        // Should detect unused parameter
        // assert_eq!(diagnostics.len(), 1);
        // assert!(diagnostics[0].message.contains("unused"));
    }
}