use oxc_allocator::Allocator;
use oxc_parser::{Parser, ParseOptions};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use crate::rules::code_quality::c029_catch_block_logging::check_catch_block_logging;

fn main() {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_module(true).with_jsx(true);

    // Test case 1: Empty catch block (should trigger violation)
    let code1 = r#"
        try {
            riskyOperation();
        } catch (error) {
            // Silent failure
        }
    "#;

    let parse_result1 = Parser::new(&allocator, code1, source_type).parse();
    if let Ok(program1) = &parse_result1.program {
        let semantic_result1 = SemanticBuilder::new()
            .with_trivias(&parse_result1.trivias)
            .build(&parse_result1.program);
        
        if let Ok(semantic1) = &semantic_result1.semantic {
            let issues1 = check_catch_block_logging(&program1, &semantic1, code1);
            println!("Test 1 - Empty catch block: {} issues found", issues1.len());
            for issue in issues1 {
                println!("  - {}", issue.message);
            }
        }
    }

    // Test case 2: Catch block with logging (should pass)
    let code2 = r#"
        try {
            riskyOperation();
        } catch (error) {
            console.error("Operation failed:", error);
        }
    "#;

    let parse_result2 = Parser::new(&allocator, code2, source_type).parse();
    if let Ok(program2) = &parse_result2.program {
        let semantic_result2 = SemanticBuilder::new()
            .with_trivias(&parse_result2.trivias)
            .build(&parse_result2.program);
        
        if let Ok(semantic2) = &semantic_result2.semantic {
            let issues2 = check_catch_block_logging(&program2, &semantic2, code2);
            println!("Test 2 - Catch block with logging: {} issues found", issues2.len());
        }
    }

    println!("C029 rule test completed successfully!");
}
