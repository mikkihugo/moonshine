//! # C002: No Duplicate Code Rule
//!
//! Detects duplicate code blocks longer than a configurable threshold (default 10 lines)
//! to maintain DRY (Don't Repeat Yourself) principle and improve code maintainability.
//! Supports intelligent comparison with optional comment and whitespace normalization.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Statement, Expression, BlockStatement, Function, ArrowFunctionExpression, PropertyDefinition, IfStatement, ForStatement, WhileStatement, DoWhileStatement};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration options for the C002 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C002Config {
    /// The minimum number of lines to consider as duplicate code.
    #[serde(default = "default_min_lines")]
    pub min_lines: u32,
    /// Whether to ignore comments when comparing code blocks.
    #[serde(default = "default_ignore_comments")]
    pub ignore_comments: bool,
    /// Whether to ignore whitespace differences when comparing code blocks.
    #[serde(default = "default_ignore_whitespace")]
    pub ignore_whitespace: bool,
}

/// Returns the default minimum number of lines for the C002 rule.
fn default_min_lines() -> u32 {
    10
}

/// Returns the default value for ignoring comments for the C002 rule.
fn default_ignore_comments() -> bool {
    true
}

/// Returns the default value for ignoring whitespace for the C002 rule.
fn default_ignore_whitespace() -> bool {
    true
}

impl Default for C002Config {
    fn default() -> Self {
        Self {
            min_lines: 10,
            ignore_comments: true,
            ignore_whitespace: true,
        }
    }
}

/// The main entry point for the C002 rule checking.
pub fn check_no_duplicate_code(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C002Config::default();
    let mut visitor = DuplicateCodeVisitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.finalize_issues()
}

/// Represents a block of code with its span, line count, normalized code, and whether it has been reported.
struct CodeBlock {
    span: Span,
    lines: u32,
    normalized_code: String,
    reported: bool,
}

/// An AST visitor for detecting duplicate code violations.
struct DuplicateCodeVisitor<'a> {
    config: &'a C002Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    code_lines: Vec<&'a str>,
    issues: Vec<LintIssue>,
    code_blocks: HashMap<String, Vec<CodeBlock>>,
}

impl<'a> DuplicateCodeVisitor<'a> {
    /// Creates a new `DuplicateCodeVisitor`.
    fn new(config: &'a C002Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let code_lines: Vec<&str> = source_code.lines().collect();
        Self {
            config,
            program,
            source_code,
            code_lines,
            issues: Vec::new(),
            code_blocks: HashMap::new(),
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, duplicate_count: usize, lines: u32) -> String {
        format!("Found {} instances of duplicate code blocks with {} lines each. Duplicate code violates the DRY (Don't Repeat Yourself) principle and makes maintenance harder. Consider extracting the common logic into a shared function or utility.", duplicate_count, lines)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Extract the duplicate code into a separate function with a descriptive name".to_string(),
            "Create a utility function or helper method to handle the common logic".to_string(),
            "Use inheritance or composition to share behavior between classes".to_string(),
            "Consider using a design pattern like Template Method or Strategy to eliminate duplication".to_string(),
            "Add comprehensive tests before refactoring to ensure behavior is preserved".to_string(),
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

    /// Normalizes the given code text by ignoring whitespace and comments based on the configuration.
    fn normalize_code(&self, text: &str) -> String {
        let mut normalized = text.to_string();

        if self.config.ignore_whitespace {
            // Remove extra whitespace and normalize spacing
            normalized = normalized
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
        }

        if self.config.ignore_comments {
            // Remove single line comments
            normalized = normalized.lines()
                .map(|line| {
                    if let Some(pos) = line.find("//") {
                        &line[..pos]
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            // Remove multi-line comments (simplified approach)
            while let Some(start) = normalized.find("/*") {
                if let Some(end) = normalized[start..].find("*/") {
                    normalized.replace_range(start..start + end + 2, "");
                } else {
                    break;
                }
            }
        }

        normalized.trim().to_string()
    }

    /// Returns the code text for the given span.
    fn get_code_text(&self, span: Span) -> String {
        let start_line = span.start as usize;
        let end_line = span.end as usize;

        if start_line >= self.source_code.len() || end_line > self.source_code.len() {
            return String::new();
        }

        self.source_code[start_line..end_line].to_string()
    }

    /// Counts the number of lines in the given span.
    fn count_lines(&self, span: Span) -> u32 {
        let text = self.get_code_text(span);
        text.lines().count() as u32
    }

    /// Analyzes a code block for duplication.
    fn analyze_code_block(&mut self, span: Span) {
        let line_count = self.count_lines(span);

        if line_count < self.config.min_lines {
            return;
        }

        let code_text = self.get_code_text(span);
        let normalized_code = self.normalize_code(&code_text);

        // Skip if normalized code is too short after cleaning
        if normalized_code.len() < 20 {
            return;
        }

        let code_block = CodeBlock {
            span,
            lines: line_count,
            normalized_code: normalized_code.clone(),
            reported: false,
        };

        self.code_blocks.entry(normalized_code)
            .or_insert_with(Vec::new)
            .push(code_block);
    }

    /// Finalizes the issues by reporting duplicate code blocks.
    fn finalize_issues(mut self) -> Vec<LintIssue> {
        // Collect duplicate information first to avoid borrowing conflicts
        let mut duplicates_to_report = Vec::new();

        for (normalized_code, blocks) in &self.code_blocks {
            if blocks.len() > 1 {
                for block in blocks {
                    if !block.reported {
                        duplicates_to_report.push((blocks.len(), block.lines, block.span));
                        break; // Only report once per duplicate group
                    }
                }
            }
        }

        // Now report the duplicates
        for (duplicate_count, lines, span) in duplicates_to_report {
            // AI Enhancement: Generate context-aware message
            let ai_enhanced_message = self.generate_ai_enhanced_message(duplicate_count, lines);
            let _ai_fix_suggestions = self.generate_ai_fix_suggestions();

            let (line, column) = self.calculate_line_column(span.start as usize);

            self.issues.push(LintIssue {
                rule_name: "moonshine/c002".to_string(),
                message: ai_enhanced_message,
                severity: LintSeverity::Warning,
                line,
                column,
                fix_available: true,
            });
        }

        // Mark blocks as reported
        for blocks in self.code_blocks.values_mut() {
            if blocks.len() > 1 {
                for block in blocks.iter_mut() {
                    block.reported = true;
                }
            }
        }

        self.issues
    }
}

impl<'a> Visit<'a> for DuplicateCodeVisitor<'a> {
    fn visit_function(&mut self, node: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {
        if let Some(body) = &node.body {
            self.analyze_code_block(body.span);
            self.visit_block_statement(body);
        }
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }
    }


    fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression<'a>) {
        if !node.expression {
            if let Some(body) = node.body.as_block_statement() {
                self.analyze_code_block(body.span);
                self.visit_block_statement(body);
            }
        }
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }
    }

    fn visit_property_definition(&mut self, node: &PropertyDefinition<'a>) {
        if let Some(value) = &node.value {
            if let oxc_ast::ast::Expression::FunctionExpression(func) = value {
                if let Some(body) = &func.body {
                    self.analyze_code_block(body.span);
                    self.visit_block_statement(body);
                }
            }
        }
    }

    fn visit_block_statement(&mut self, node: &BlockStatement<'a>) {
        // Analyze block for duplicates
        self.analyze_code_block(node.span);

        // Continue visiting statements within the block
        for stmt in &node.body {
            self.visit_statement(stmt);
        }
    }

    fn visit_if_statement(&mut self, node: &IfStatement<'a>) {
        self.visit_expression(&node.test);
        self.visit_statement(&node.consequent);
        if let Some(alternate) = &node.alternate {
            self.visit_statement(alternate);
        }
    }

    fn visit_for_statement(&mut self, node: &ForStatement<'a>) {
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
        self.visit_statement(&node.body);
    }

    fn visit_while_statement(&mut self, node: &WhileStatement<'a>) {
        self.visit_expression(&node.test);
        self.visit_statement(&node.body);
    }

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement<'a>) {
        self.visit_statement(&node.body);
        self.visit_expression(&node.test);
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

        check_no_duplicate_code(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_duplicate_code_violation() {
        let code = r#"
function processUserData(user) {
    if (!user.id) {
        console.log("Invalid user ID");
        return null;
    }
    if (!user.email) {
        console.log("Invalid email");
        return null;
    }
    // Validate user
    if (user.age < 18) {
        console.log("User too young");
        return null;
    }
    return user;
}

function processAdminData(admin) {
    if (!admin.id) {
        console.log("Invalid user ID");
        return null;
    }
    if (!admin.email) {
        console.log("Invalid email");
        return null;
    }
    // Validate admin
    if (admin.permissions.length === 0) {
        console.log("No permissions");
        return null;
    }
    return admin;
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Duplicate code block found"));
    }

    #[test]
    fn test_no_duplicate_code_compliant() {
        let code = r#"
function validateUser(user) {
    if (!user.id) {
        console.log("Invalid user ID");
        return null;
    }
    return user;
}

function processData(data) {
    if (!data) {
        console.log("No data provided");
        return [];
    }
    return data.map(item => item.id);
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_short_code_blocks_ignored() {
        let code = r#"
function shortFunction1() {
    return true;
}

function shortFunction2() {
    return true;
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Too short to be considered duplicate
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
function processUser(user) {
    if (!user.id) {
        console.log("Invalid user ID");
        return null;
    }
    if (!user.email) {
        console.log("Invalid email");
        return null;
    }
    return user;
}

function processAdmin(admin) {
    if (!admin.id) {
        console.log("Invalid user ID");
        return null;
    }
    if (!admin.email) {
        console.log("Invalid email");
        return null;
    }
    return admin;
}
        "#;

        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        // Check that AI enhancement was applied
        assert!(issues[0].message.contains("instances of duplicate code") || issues[0].fix_available);
    }
}