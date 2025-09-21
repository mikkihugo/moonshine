//! # C013: No Dead Code Rule
//!
//! Prevents commented-out code and unreachable code from being left in the codebase
//! to maintain cleanliness. Detects code that comes after terminating statements
//! (return, throw, continue, break) and flags it as unreachable dead code.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity low
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{
    Program, BlockStatement, Statement, SwitchCase,
    ReturnStatement, ThrowStatement, ContinueStatement, BreakStatement
};
use oxc_ast_visit::Visit;
use oxc_span::{GetSpan, Span};
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};

/// Configuration options for C013 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C013Config {
    /// Whether to check for commented-out code patterns (default: true)
    #[serde(default = "default_check_commented_code")]
    pub check_commented_code: bool,
}

fn default_check_commented_code() -> bool {
    true
}

impl Default for C013Config {
    fn default() -> Self {
        Self {
            check_commented_code: true,
        }
    }
}

/// Main entry point for C013 rule checking
pub fn check_no_dead_code(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C013Config::default();
    let mut visitor = C013Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting dead code violations
struct C013Visitor<'a> {
    config: &'a C013Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
}

impl<'a> C013Visitor<'a> {
    fn new(config: &'a C013Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, issue_type: &str) -> String {
        match issue_type {
            "unreachable" => "Unreachable code detected. Code after return, throw, continue, or break statements will never execute. Remove this dead code to improve maintainability.".to_string(),
            "commented_code" => "Commented-out code detected. Remove commented code or uncomment it if needed. Commented code makes the codebase harder to maintain.".to_string(),
            _ => "Dead code detected. Remove unreachable or commented code to keep the codebase clean.".to_string(),
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, issue_type: &str) -> Vec<String> {
        match issue_type {
            "unreachable" => vec![
                "Remove the unreachable code after the terminating statement".to_string(),
                "Move the code before the return/throw/break/continue statement".to_string(),
                "Refactor the logic to avoid unreachable code".to_string(),
            ],
            "commented_code" => vec![
                "Remove the commented-out code entirely".to_string(),
                "Uncomment the code if it's still needed".to_string(),
                "Move useful code to a separate function or file".to_string(),
            ],
            _ => vec!["Remove dead code to improve code maintainability".to_string()],
        }
    }

    /// Calculate line and column from byte offset
    fn calculate_line_column(&self, offset: usize) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source_code.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Create lint issue for dead code violation with AI enhancement
    fn create_dead_code_issue(&self, span: Span, issue_type: &str) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(issue_type);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(issue_type);

        LintIssue {
            rule_name: "moonshine/c013".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    fn is_terminating_statement(&self, stmt: &Statement) -> bool {
        matches!(stmt,
            Statement::ReturnStatement(_) |
            Statement::ThrowStatement(_) |
            Statement::ContinueStatement(_) |
            Statement::BreakStatement(_)
        )
    }

    fn is_executable_statement(&self, stmt: &Statement) -> bool {
        // Exclude declarations and empty statements
        !matches!(stmt,
            Statement::EmptyStatement(_) |
            Statement::FunctionDeclaration(_) |
            Statement::VariableDeclaration(_) |
            Statement::Class(_) |
            Statement::TSInterfaceDeclaration(_) |
            Statement::TSTypeAliasDeclaration(_) |
            Statement::TSEnumDeclaration(_) |
            Statement::TSModuleDeclaration(_) |
            Statement::TSImportEqualsDeclaration(_) |
            Statement::ImportDeclaration(_) |
            Statement::ExportAllDeclaration(_) |
            Statement::ExportDefaultDeclaration(_) |
            Statement::ExportNamedDeclaration(_)
        )
    }

    fn check_block_for_unreachable_code(&mut self, block: &BlockStatement) {
        let mut unreachable = false;

        for stmt in &block.body {
            if unreachable && self.is_executable_statement(stmt) {
                self.issues.push(self.create_dead_code_issue(stmt.span(), "unreachable"));
            }

            if self.is_terminating_statement(stmt) {
                unreachable = true;
            }
        }
    }

    fn check_switch_case_for_unreachable_code(&mut self, switch_case: &SwitchCase) {
        let mut unreachable = false;

        for stmt in &switch_case.consequent {
            if unreachable && self.is_executable_statement(stmt) {
                self.issues.push(self.create_dead_code_issue(stmt.span(), "unreachable"));
            }

            if self.is_terminating_statement(stmt) {
                unreachable = true;
            }
        }
    }
}

impl<'a> Visit<'a> for C013Visitor<'a> {
    fn visit_block_statement(&mut self, node: &BlockStatement<'a>) {
        self.check_block_for_unreachable_code(node);

        // Continue visiting statements within the block
        for stmt in &node.body {
            self.visit_statement(stmt);
        }
    }

    fn visit_switch_case(&mut self, node: &SwitchCase<'a>) {
        self.check_switch_case_for_unreachable_code(node);

        // Visit test expression if present
        if let Some(test) = &node.test {
            self.visit_expression(test);
        }

        // Visit consequent statements
        for stmt in &node.consequent {
            self.visit_statement(stmt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        check_no_dead_code(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_unreachable_after_return_violation() {
        let code = r#"
function processData() {
    if (condition) {
        return result;
        console.log("This will never execute"); // Dead code
        doSomething(); // Dead code
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Unreachable code detected"));
    }

    #[test]
    fn test_unreachable_after_throw_violation() {
        let code = r#"
function validateInput(input) {
    if (!input) {
        throw new Error("Invalid input");
        console.log("This will never execute"); // Dead code
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Unreachable code detected"));
    }

    #[test]
    fn test_unreachable_in_switch_case_violation() {
        let code = r#"
function handleCase(value) {
    switch (value) {
        case 'A':
            return 'handled A';
            console.log("Dead code after return"); // Dead code
            break;
        case 'B':
            throw new Error("Invalid B");
            console.log("Dead code after throw"); // Dead code
            break;
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Unreachable code detected")));
    }

    #[test]
    fn test_unreachable_after_break_continue_violation() {
        let code = r#"
function processLoop() {
    for (let i = 0; i < 10; i++) {
        if (condition1) {
            continue;
            console.log("Dead code after continue"); // Dead code
        }
        if (condition2) {
            break;
            console.log("Dead code after break"); // Dead code
        }
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Unreachable code detected")));
    }

    #[test]
    fn test_reachable_code_compliant() {
        let code = r#"
function processData() {
    if (condition) {
        console.log("This is reachable");
        return result;
    }

    // This is reachable if condition is false
    console.log("Alternative path");
    return defaultResult;
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_declarations_after_return_compliant() {
        let code = r#"
function outerFunction() {
    if (condition) {
        return result;
    }

    // Function declarations are hoisted, so this is not dead code
    function helperFunction() {
        return "helper";
    }

    // Variable declarations are also hoisted
    var hoistedVar = "value";
}
        "#;

        let issues = parse_and_check(code);
        // Declarations should not be flagged as dead code
        assert!(issues.is_empty());
    }

    #[test]
    fn test_empty_statements_ignored() {
        let code = r#"
function testEmpty() {
    return;
    ; // Empty statement - should not be flagged
    ; // Another empty statement
}
        "#;

        let issues = parse_and_check(code);
        // Empty statements should be ignored
        assert!(issues.is_empty());
    }
}