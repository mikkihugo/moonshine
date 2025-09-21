//! # C023: No Duplicate Variable Name in Scope Rule
//!
//! Prevents variable name shadowing and maintains clear variable scoping by detecting
//! duplicate variable declarations within the same scope. This rule helps avoid
//! confusion and potential bugs caused by variable name conflicts.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{
    Program, VariableDeclarator, Function,
    ArrowFunctionExpression, BlockStatement, CatchClause, ForStatement,
    ForInStatement, ForOfStatement, SwitchStatement, BindingIdentifier
};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration options for C023 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C023Config {
    /// Whether to allow variable shadowing between different scope levels (default: false)
    #[serde(default)]
    pub allow_shadowing: bool,
    /// Whether to check function parameters for duplicates (default: true)
    #[serde(default = "default_check_function_parameters")]
    pub check_function_parameters: bool,
    /// Whether to allow duplicate variable names in catch clauses (default: false)
    #[serde(default)]
    pub allow_catch_duplicates: bool,
}

fn default_check_function_parameters() -> bool {
    true
}

impl Default for C023Config {
    fn default() -> Self {
        Self {
            allow_shadowing: false,
            check_function_parameters: true,
            allow_catch_duplicates: false,
        }
    }
}

/// Main entry point for C023 rule checking
pub fn check_no_duplicate_variable_name(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C023Config::default();
    let mut visitor = DuplicateVariableVisitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

#[derive(Debug, Clone)]
struct VariableInfo {
    span: Span,
    scope_id: usize,
}

/// AST visitor for detecting duplicate variable name violations
struct DuplicateVariableVisitor<'a> {
    config: &'a C023Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    scope_stack: Vec<HashMap<String, VariableInfo>>,
    current_scope_id: usize,
}

impl<'a> DuplicateVariableVisitor<'a> {
    fn new(config: &'a C023Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            scope_stack: Vec::new(),
            current_scope_id: 0,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, variable_name: &str) -> String {
        format!("Variable '{}' is declared multiple times in the same scope, which can cause confusion and bugs. Variable shadowing makes code harder to understand and maintain. Consider using unique variable names or different scopes.", variable_name)
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Rename one of the variables to have a more specific name".to_string(),
            "Move one variable declaration to a different scope".to_string(),
            "Use different naming conventions (e.g., prefix/suffix) to distinguish variables".to_string(),
            "Consider extracting the duplicate logic into separate functions".to_string(),
            "Review the code logic to ensure both variables are actually needed".to_string(),
        ]
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

    fn enter_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
        self.current_scope_id += 1;
    }

    fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn check_variable(&mut self, name: &str, span: Span) {
        if self.scope_stack.is_empty() {
            return;
        }

        // Scope the mutable borrow to only the block where it's needed
        let already_declared = {
            let current_scope = self.scope_stack.last().unwrap();
            current_scope.get(name).is_some()
        };

        if already_declared {
            let ai_enhanced_message = self.generate_ai_enhanced_message(name);
            let (line, column) = self.calculate_line_column(span.start as usize);

            self.issues.push(LintIssue {
                rule_name: "moonshine/c023".to_string(),
                severity: LintSeverity::Error,
                message: ai_enhanced_message,
                line,
                column,
                fix_available: true,
            });
            return;
        }

        // Check for shadowing in parent scopes if not allowed
        if !self.config.allow_shadowing {
            // Only immutable borrow here, no mutable borrow active
            for scope in self.scope_stack.iter().rev().skip(1) {
                if scope.contains_key(name) {
                    let ai_enhanced_message = format!(
                        "Variable '{}' shadows a variable in outer scope. Variable shadowing can cause confusion and bugs. Consider using unique names or different scopes.",
                        name
                    );
                    let (line, column) = self.calculate_line_column(span.start as usize);

                    self.issues.push(LintIssue {
                        rule_name: "moonshine/c023".to_string(),
                        severity: LintSeverity::Warning,
                        message: ai_enhanced_message,
                        line,
                        column,
                        fix_available: true,
                    });
                    break;
                }
            }
        }

        // Now do the mutable insert
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.insert(
                name.to_string(),
                VariableInfo {
                    span,
                    scope_id: self.current_scope_id,
                },
            );
        }
    }

    fn check_binding_identifier(&mut self, ident: &BindingIdentifier) {
        self.check_variable(&ident.name, ident.span);
    }

    fn check_function_parameters(&mut self, params: &oxc_ast::ast::FormalParameters) {
        if !self.config.check_function_parameters {
            return;
        }

        let mut param_names = HashSet::new();
        for param in &params.items {
            if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &param.pattern.kind {
                let name = &ident.name;
                if param_names.contains(name) {
                    let (line, column) = self.calculate_line_column(ident.span);
                    self.issues.push(LintIssue {
                        rule_name: "C023".to_string(),
                        severity: LintSeverity::Error,
                        message: format!("Duplicate parameter name '{}'. Function parameters must have unique names to avoid confusion and maintain code clarity.", name),
                        line,
                        column,
                        fix_available: true, // AI can suggest parameter renaming
                    });
                } else {
                    param_names.insert(name.clone());
                    self.check_variable(name, ident.span);
                }
            }
        }
    }
}

impl<'a> Visit<'a> for DuplicateVariableVisitor<'a> {
    fn visit_program(&mut self, node: &Program<'a>) {
        self.enter_scope();

        // Visit all statements in the program
        for stmt in &node.body {
            self.visit_statement(stmt);
        }

        self.exit_scope();
    }

    fn visit_function(&mut self, node: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {
        // Check function name in current scope before entering function scope
        if let Some(id) = &node.id {
            self.check_binding_identifier(id);
        }

        self.enter_scope();

        // Check function parameters
        self.check_function_parameters(&node.params);

        // Visit function body
        if let Some(body) = &node.body {
            self.visit_block_statement(body);
        }

        self.exit_scope();
    }


    fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression<'a>) {
        self.enter_scope();

        // Check function parameters
        self.check_function_parameters(&node.params);

        // Visit function body
        match &node.body {
            oxc_ast::ast::FunctionBody::FunctionBody(body) => {
                self.visit_block_statement(body);
            }
            oxc_ast::ast::FunctionBody::Expression(expr) => {
                self.visit_expression(expr);
            }
        }

        self.exit_scope();
    }

    fn visit_block_statement(&mut self, node: &BlockStatement<'a>) {
        self.enter_scope();

        for stmt in &node.body {
            self.visit_statement(stmt);
        }

        self.exit_scope();
    }

    fn visit_catch_clause(&mut self, node: &CatchClause<'a>) {
        self.enter_scope();

        // Check catch parameter
        if let Some(param) = &node.param {
            if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &param.kind {
                if !self.config.allow_catch_duplicates {
                    self.check_variable(&ident.name, ident.span);
                }
            }
        }

        // Visit catch body
        self.visit_block_statement(&node.body);

        self.exit_scope();
    }

    fn visit_for_statement(&mut self, node: &ForStatement<'a>) {
        self.enter_scope();

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

        self.exit_scope();
    }

    fn visit_for_in_statement(&mut self, node: &ForInStatement<'a>) {
        self.enter_scope();

        // Visit left side (variable declaration or assignment target)
        match &node.left {
            oxc_ast::ast::ForStatementLeft::VariableDeclaration(var_decl) => {
                self.visit_variable_declaration(var_decl);
            }
            _ => {
                // Handle other assignment targets
            }
        }

        // Visit right side and body
        self.visit_expression(&node.right);
        self.visit_statement(&node.body);

        self.exit_scope();
    }

    fn visit_for_of_statement(&mut self, node: &ForOfStatement<'a>) {
        self.enter_scope();

        // Visit left side (variable declaration or assignment target)
        match &node.left {
            oxc_ast::ast::ForStatementLeft::VariableDeclaration(var_decl) => {
                self.visit_variable_declaration(var_decl);
            }
            _ => {
                // Handle other assignment targets
            }
        }

        // Visit right side and body
        self.visit_expression(&node.right);
        self.visit_statement(&node.body);

        self.exit_scope();
    }

    fn visit_switch_statement(&mut self, node: &SwitchStatement<'a>) {
        self.enter_scope();

        // Visit discriminant
        self.visit_expression(&node.discriminant);

        // Visit cases
        for case in &node.cases {
            self.visit_switch_case(case);
        }

        self.exit_scope();
    }

    fn visit_variable_declarator(&mut self, node: &VariableDeclarator<'a>) {
        // Check the variable name
        if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &node.id.kind {
            self.check_binding_identifier(ident);
        }

        // Visit initializer if present
        if let Some(init) = &node.init {
            self.visit_expression(init);
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

        check_no_duplicate_variable_name(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_duplicate_variables_same_scope_violation() {
        let code = r#"
function processData() {
    let data = "first";
    let data = "second"; // Duplicate in same scope

    const result = compute();
    const result = transform(); // Duplicate in same scope

    return result;
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("already declared")));
    }

    #[test]
    fn test_duplicate_function_parameters_violation() {
        let code = r#"
function processUser(user, settings, user) { // Duplicate parameter
    return user.name;
}

const processData = (data, options, data) => { // Duplicate parameter
    return data.value;
};
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Duplicate parameter")));
    }

    #[test]
    fn test_variable_shadowing_warning() {
        let code = r#"
let globalVar = "global";

function processData() {
    let globalVar = "local"; // Shadows global variable

    if (condition) {
        let globalVar = "nested"; // Shadows function variable
        console.log(globalVar);
    }

    return globalVar;
}
        "#;

        let issues = parse_and_check(code);
        // Shadowing should produce warnings
        assert!(issues.iter().any(|issue| issue.message.contains("shadows")));
    }

    #[test]
    fn test_different_scopes_compliant() {
        let code = r#"
function processUser() {
    let data = "user data";
    return data;
}

function processOrder() {
    let data = "order data"; // Different scope, ok
    return data;
}

function processPayment() {
    {
        let amount = 100;
    }
    {
        let amount = 200; // Different block scope, ok
    }
}
        "#;

        let issues = parse_and_check(code);
        // Different scopes should be allowed
        assert!(issues.is_empty());
    }

    #[test]
    fn test_for_loop_variables_compliant() {
        let code = r#"
function processItems() {
    for (let i = 0; i < 10; i++) {
        console.log(i);
    }

    for (let i = 0; i < 5; i++) { // Different scope, ok
        console.log(i);
    }

    for (const item of items) {
        console.log(item);
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_catch_clause_variables() {
        let code = r#"
function handleErrors() {
    try {
        riskyOperation();
    } catch (error) {
        console.log(error);
    }

    try {
        anotherOperation();
    } catch (error) { // Different catch scope, ok
        console.log(error);
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
function processData() {
    let data = "first";
    let data = "second"; // Duplicate
}
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = check_no_duplicate_variable_name(&parse_result.program, &semantic_result.semantic, code);

        assert!(!issues.is_empty());
        assert!(issues[0].fix_available);
    }
}