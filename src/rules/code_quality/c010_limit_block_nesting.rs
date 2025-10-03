//! # C010: Limit Block Nesting Rule
//!
//! Limits nested blocks (if/for/while/switch) to maximum 3 levels to improve
//! readability and maintainability. Deeply nested code is harder to understand,
//! test, and maintain. This rule encourages extraction of nested logic into
//! separate functions.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity low
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{
    Program, IfStatement, ForStatement, ForInStatement, ForOfStatement,
    WhileStatement, DoWhileStatement, SwitchStatement, TryStatement, WithStatement
};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};

/// Configuration options for the C010 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C010Config {
    /// The maximum allowed nesting depth (default: 3).
    #[serde(default = "default_max_depth")]
    pub max_depth: u32,
}

/// Returns the default maximum nesting depth.
fn default_max_depth() -> u32 {
    3
}

impl Default for C010Config {
    fn default() -> Self {
        Self {
            max_depth: 3,
        }
    }
}

/// The main entry point for the C010 rule checking.
pub fn check_limit_block_nesting(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C010Config::default();
    let mut visitor = C010Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// An AST visitor for detecting excessive block nesting.
struct C010Visitor<'a> {
    config: &'a C010Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    current_depth: u32,
}

impl<'a> C010Visitor<'a> {
    /// Creates a new `C010Visitor`.
    fn new(config: &'a C010Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            current_depth: 0,
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, depth: u32) -> String {
        format!("Block nesting is too deep (level {}). Maximum allowed is {} levels. Deeply nested code is harder to read, test, and maintain. Consider extracting nested logic into separate functions or using early returns.", depth, self.config.max_depth)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Extract nested logic into a separate function".to_string(),
            "Use early returns to reduce nesting".to_string(),
            "Consider using guard clauses".to_string(),
            "Refactor complex conditions into variables".to_string(),
        ]
    }

    /// Calculates the line and column from a byte offset.
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

    /// Creates a lint issue for excessive nesting with AI enhancement.
    fn create_nesting_issue(&self, span: Span, depth: u32) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(depth);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions();

        LintIssue {
            rule_name: "moonshine/c010".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Enters a block, increments the current depth, and checks if the maximum depth has been exceeded.
    fn enter_block(&mut self, span: Span) {
        self.current_depth += 1;

        if self.current_depth > self.config.max_depth {
            self.issues.push(self.create_nesting_issue(span, self.current_depth));
        }
    }

    /// Exits a block and decrements the current depth.
    fn exit_block(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
}

impl<'a> Visit<'a> for C010Visitor<'a> {
    fn visit_if_statement(&mut self, node: &IfStatement<'a>) {
        self.enter_block(node.span);

        // Visit the consequent
        self.visit_statement(&node.consequent);

        // Visit the alternate if it exists
        if let Some(alternate) = &node.alternate {
            self.visit_statement(alternate);
        }

        self.exit_block();
    }

    fn visit_for_statement(&mut self, node: &ForStatement<'a>) {
        self.enter_block(node.span);

        // Visit init, test, update first
        if let Some(init) = &node.init {
            match init {
                oxc_ast::ast::ForStatementInit::VariableDeclaration(var_decl) => {
                    self.visit_variable_declaration(var_decl);
                }
                oxc_ast::ast::ForStatementInit::UsingDeclaration(using_decl) => {
                    self.visit_using_declaration(using_decl);
                }
                oxc_ast::ast::ForStatementInit::Expression(expr) => {
                    self.visit_expression(expr);
                }
            }
        }

        if let Some(test) = &node.test {
            self.visit_expression(test);
        }

        if let Some(update) = &node.update {
            self.visit_expression(update);
        }

        // Visit body
        self.visit_statement(&node.body);

        self.exit_block();
    }

    fn visit_for_in_statement(&mut self, node: &ForInStatement<'a>) {
        self.enter_block(node.span);

        // Visit left and right first
        match &node.left {
            oxc_ast::ast::ForStatementLeft::VariableDeclaration(var_decl) => {
                self.visit_variable_declaration(var_decl);
            }
            oxc_ast::ast::ForStatementLeft::UsingDeclaration(using_decl) => {
                self.visit_using_declaration(using_decl);
            }
            oxc_ast::ast::ForStatementLeft::AssignmentTargetIdentifier(ident) => {
                self.visit_identifier_reference(ident);
            }
            oxc_ast::ast::ForStatementLeft::TSAsExpression(ts_as) => {
                self.visit_ts_as_expression(ts_as);
            }
            oxc_ast::ast::ForStatementLeft::TSSatisfiesExpression(ts_satisfies) => {
                self.visit_ts_satisfies_expression(ts_satisfies);
            }
            oxc_ast::ast::ForStatementLeft::TSNonNullExpression(ts_non_null) => {
                self.visit_ts_non_null_expression(ts_non_null);
            }
            oxc_ast::ast::ForStatementLeft::TSTypeAssertion(ts_type_assertion) => {
                self.visit_ts_type_assertion(ts_type_assertion);
            }
            oxc_ast::ast::ForStatementLeft::TSInstantiationExpression(ts_instantiation) => {
                self.visit_ts_instantiation_expression(ts_instantiation);
            }
            _ => {}
        }

        self.visit_expression(&node.right);

        // Visit body
        self.visit_statement(&node.body);

        self.exit_block();
    }

    fn visit_for_of_statement(&mut self, node: &ForOfStatement<'a>) {
        self.enter_block(node.span);

        // Visit left and right first
        match &node.left {
            oxc_ast::ast::ForStatementLeft::VariableDeclaration(var_decl) => {
                self.visit_variable_declaration(var_decl);
            }
            oxc_ast::ast::ForStatementLeft::UsingDeclaration(using_decl) => {
                self.visit_using_declaration(using_decl);
            }
            oxc_ast::ast::ForStatementLeft::AssignmentTargetIdentifier(ident) => {
                self.visit_identifier_reference(ident);
            }
            oxc_ast::ast::ForStatementLeft::TSAsExpression(ts_as) => {
                self.visit_ts_as_expression(ts_as);
            }
            oxc_ast::ast::ForStatementLeft::TSSatisfiesExpression(ts_satisfies) => {
                self.visit_ts_satisfies_expression(ts_satisfies);
            }
            oxc_ast::ast::ForStatementLeft::TSNonNullExpression(ts_non_null) => {
                self.visit_ts_non_null_expression(ts_non_null);
            }
            oxc_ast::ast::ForStatementLeft::TSTypeAssertion(ts_type_assertion) => {
                self.visit_ts_type_assertion(ts_type_assertion);
            }
            oxc_ast::ast::ForStatementLeft::TSInstantiationExpression(ts_instantiation) => {
                self.visit_ts_instantiation_expression(ts_instantiation);
            }
            _ => {}
        }

        self.visit_expression(&node.right);

        // Visit body
        self.visit_statement(&node.body);

        self.exit_block();
    }

    fn visit_while_statement(&mut self, node: &WhileStatement<'a>) {
        self.enter_block(node.span);

        // Visit test first
        self.visit_expression(&node.test);

        // Visit body
        self.visit_statement(&node.body);

        self.exit_block();
    }

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement<'a>) {
        self.enter_block(node.span);

        // Visit body first
        self.visit_statement(&node.body);

        // Visit test
        self.visit_expression(&node.test);

        self.exit_block();
    }

    fn visit_switch_statement(&mut self, node: &SwitchStatement<'a>) {
        self.enter_block(node.span);

        // Visit discriminant first
        self.visit_expression(&node.discriminant);

        // Visit cases
        for case in &node.cases {
            self.visit_switch_case(case);
        }

        self.exit_block();
    }

    fn visit_try_statement(&mut self, node: &TryStatement<'a>) {
        self.enter_block(node.span);

        // Visit block
        self.visit_block_statement(&node.block);

        // Visit handler if exists
        if let Some(handler) = &node.handler {
            self.visit_catch_clause(handler);
        }

        // Visit finalizer if exists
        if let Some(finalizer) = &node.finalizer {
            self.visit_block_statement(finalizer);
        }

        self.exit_block();
    }

    fn visit_with_statement(&mut self, node: &WithStatement<'a>) {
        self.enter_block(node.span);

        // Visit object first
        self.visit_expression(&node.object);

        // Visit body
        self.visit_statement(&node.body);

        self.exit_block();
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

        check_limit_block_nesting(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_deep_nesting_violation() {
        let code = r#"
function processData() {
    if (condition1) {           // Level 1
        for (let i = 0; i < 10; i++) {  // Level 2
            while (condition2) {         // Level 3
                if (condition3) {        // Level 4 - should trigger violation
                    doSomething();
                }
            }
        }
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Block nesting is too deep"));
        assert!(issues[0].message.contains("level 4"));
    }

    #[test]
    fn test_acceptable_nesting_compliant() {
        let code = r#"
function processData() {
    if (condition1) {           // Level 1
        for (let i = 0; i < 10; i++) {  // Level 2
            while (condition2) {         // Level 3 - within limit
                doSomething();
            }
        }
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_switch_statement_nesting() {
        let code = r#"
function handleInput(type) {
    if (isValid) {              // Level 1
        switch (type) {         // Level 2
            case 'A':
                for (let i = 0; i < 5; i++) {  // Level 3
                    if (condition) {           // Level 4 - violation
                        process();
                    }
                }
                break;
        }
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Block nesting is too deep"));
    }

    #[test]
    fn test_try_catch_nesting() {
        let code = r#"
function handleError() {
    try {                       // Level 1
        if (risky) {           // Level 2
            for (let item of items) {  // Level 3
                while (processing) {   // Level 4 - violation
                    doRiskyOperation();
                }
            }
        }
    } catch (error) {
        console.error(error);
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Block nesting is too deep"));
    }

    #[test]
    fn test_sequential_blocks_compliant() {
        let code = r#"
function processSequentially() {
    // These are sequential, not nested
    if (condition1) {
        doSomething();
    }

    if (condition2) {
        doSomethingElse();
    }

    for (let i = 0; i < 10; i++) {
        process(i);
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }
}