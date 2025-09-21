/*!
 * OXC-native port of MoonShine rule: C003_no_vague_abbreviations
 * Original JS: moonshine-rules/common/C003_no_vague_abbreviations/analyzer.js
 */
//! # C003: No Vague Abbreviations Rule
//!
//! Ensures clear, understandable variable names without arbitrary abbreviations.
//! Promotes code readability by detecting single character variables, unclear
//! abbreviations, and generic/vague variable names while allowing common patterns
//! like loop counters and mathematical contexts.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{
    Program, VariableDeclarator, Function, ArrowFunctionExpression,
    CatchClause, BindingPattern, BindingPatternKind, FormalParameter, AssignmentTarget
};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use regex::Regex;

/// Configuration options for C003 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C003Config {
    /// Single character variables that are allowed (default: i, j, k, x, y, z)
    #[serde(default)]
    pub allowed_single_char: Vec<String>,
    /// Common abbreviations that are allowed
    #[serde(default)]
    pub allowed_abbreviations: Vec<String>,
    /// Minimum variable name length (default: 2)
    #[serde(default = "default_min_length")]
    pub min_length: u32,
}

fn default_min_length() -> u32 {
    2
}

impl Default for C003Config {
    fn default() -> Self {
        Self {
            allowed_single_char: vec!["i".to_string(), "j".to_string(), "k".to_string(),
                                    "x".to_string(), "y".to_string(), "z".to_string()],
            allowed_abbreviations: vec![
                "id".to_string(), "url".to_string(), "api".to_string(), "ui".to_string(),
                "db".to_string(), "config".to_string(), "env".to_string(), "app".to_string(),
                "btn".to_string(), "img".to_string(), "src".to_string(), "dest".to_string(),
                "req".to_string(), "res".to_string(), "ctx".to_string(), "min".to_string(),
                "max".to_string(), "len".to_string(), "num".to_string(), "str".to_string(),
                "json".to_string()
            ],
            min_length: 2,
        }
    }
}

/// Main entry point for C003 rule checking
pub fn check_no_vague_abbreviations(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C003Config::default();
    let mut visitor = C003Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting vague abbreviation violations
struct C003Visitor<'a> {
    config: &'a C003Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    allowed_single_char: HashSet<String>,
    allowed_abbreviations: HashSet<String>,
    unclear_names: HashSet<String>,
    suspicious_patterns: Vec<Regex>,
}

impl<'a> C003Visitor<'a> {
    fn new(config: &'a C003Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let allowed_single_char: HashSet<String> = config.allowed_single_char
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        let allowed_abbreviations: HashSet<String> = config.allowed_abbreviations
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        let unclear_names = [
            "data", "info", "item", "element", "object", "value", "result",
            "response", "request", "temp", "tmp", "var", "variable",
            "stuff", "thing", "something", "anything", "everything",
            "flag", "check", "test", "validate", "process", "handle",
            "obj", "arg", "val", "fn"
        ].iter().map(|s| s.to_string()).collect();

        let suspicious_patterns = vec![
            Regex::new(r"^[a-z]{1,2}[0-9]*$").unwrap(), // e.g., 'u', 'usr', 'n1', 'v2'
            Regex::new(r"^[a-z]*[aeiou]*[bcdfghjklmnpqrstvwxyz]{3,}$").unwrap(), // too many consonants
            Regex::new(r"^[bcdfghjklmnpqrstvwxyz]{3,}[aeiou]*$").unwrap(), // consonants at start
            Regex::new(r"^(tmp|temp|val|var|data|info|item|elem|el|obj|arr)([A-Z0-9].*)?$").unwrap(), // generic names
            Regex::new(r"^[a-z]+(Mgr|Ctrl|Svc|Repo|Util|Hlpr|Mngr)$").unwrap(), // manager/helper patterns
        ];

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            allowed_single_char,
            allowed_abbreviations,
            unclear_names,
            suspicious_patterns,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, variable_name: &str, issue_type: &str) -> String {
        match issue_type {
            "single_char" => format!("Variable '{}' is only 1 character long. Single-character variables should be reserved for loop counters (i, j, k) or mathematical contexts (x, y, z). Consider using a more descriptive name like 'index', 'counter', or 'coordinate'.", variable_name),
            "too_short" => format!("Variable '{}' is too short ({} characters). Use descriptive names with at least {} characters to improve code readability.", variable_name, variable_name.len(), self.config.min_length),
            "unclear_name" => format!("Variable '{}' is unclear or uses a generic name. Replace with a specific, descriptive name that explains what this variable represents in your code.", variable_name),
            "suspicious_abbrev" => format!("Variable '{}' appears to be an unclear abbreviation. Use full, descriptive names instead of abbreviations to make your code more readable and maintainable.", variable_name),
            _ => format!("Variable '{}' violates naming conventions. Use clear, descriptive names that explain the variable's purpose.", variable_name),
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, variable_name: &str, issue_type: &str) -> Vec<String> {
        match issue_type {
            "single_char" => vec![
                "index".to_string(),
                "counter".to_string(),
                "coordinate".to_string(),
                format!("{}Index", variable_name.to_uppercase()),
            ],
            "too_short" => vec![
                format!("{}Value", variable_name),
                format!("{}Data", variable_name),
                format!("{}Result", variable_name),
            ],
            "unclear_name" => vec![
                "userData".to_string(),
                "configuration".to_string(),
                "resultValue".to_string(),
                "processedItem".to_string(),
            ],
            "suspicious_abbrev" => vec![
                "userManager".to_string(),
                "dataController".to_string(),
                "serviceHelper".to_string(),
                "repository".to_string(),
            ],
            _ => vec![format!("{}Value", variable_name)],
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

    /// Check if name is in math context
    fn is_math_context(&self, name: &str) -> bool {
        // Math variable patterns
        let math_patterns = [
            Regex::new(r"^[xyz][12]$").unwrap(),    // x1, y1, x2, y2
            Regex::new(r"^d[xyztr]$").unwrap(),     // dx, dy, dt, dr
            Regex::new(r"^[abc]$").unwrap(),        // a, b, c in equations
            Regex::new(r"^v[xyz]$").unwrap(),       // vx, vy, vz
            Regex::new(r"^p\d+$").unwrap(),         // p1, p2
        ];

        math_patterns.iter().any(|pattern| pattern.is_match(name))
    }

    /// Check if a variable name violates vague abbreviation rules
    fn check_variable_name(&mut self, name: &str, span: Span) {
        let name_lower = name.to_lowercase();

        // Skip if allowed
        if self.allowed_single_char.contains(&name_lower) ||
           self.allowed_abbreviations.contains(&name_lower) ||
           self.is_math_context(&name_lower) {
            return;
        }

        // Check single character variables
        if name.len() == 1 {
            self.issues.push(self.create_vague_abbreviation_issue(name, span, "single_char"));
            return;
        }

        // Check minimum length
        if name.len() < self.config.min_length as usize {
            self.issues.push(self.create_vague_abbreviation_issue(name, span, "too_short"));
            return;
        }

        // Check for unclear names
        if self.unclear_names.contains(&name_lower) {
            self.issues.push(self.create_vague_abbreviation_issue(name, span, "unclear_name"));
            return;
        }

        // Check suspicious patterns
        for pattern in &self.suspicious_patterns {
            if pattern.is_match(&name_lower) {
                self.issues.push(self.create_vague_abbreviation_issue(name, span, "suspicious_abbrev"));
                return;
            }
        }
    }

    /// Extract variable name from binding pattern
    fn extract_variable_name(&self, binding_pattern: &BindingPattern) -> Option<String> {
        match &binding_pattern.kind {
            BindingPatternKind::BindingIdentifier(ident) => Some(ident.name.to_string()),
            _ => None, // Skip destructuring patterns for now
        }
    }
}

impl<'a> Visit<'a> for C003Visitor<'a> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(name) = self.extract_variable_name(&declarator.id) {
            self.check_variable_name(&name, declarator.id.span());
        }
    }

    fn visit_formal_parameter(&mut self, param: &FormalParameter<'a>) {
        if let Some(name) = self.extract_variable_name(&param.pattern) {
            self.check_variable_name(&name, param.pattern.span());
        }
    }

    fn visit_function(&mut self, func: &Function<'a>) {
        // Check function parameters
        for param in &func.params.items {
            if let Some(name) = self.extract_variable_name(&param.pattern) {
                self.check_variable_name(&name, param.pattern.span());
            }
        }

        // Continue visiting function body
        oxc_ast_visit::walk::walk_function(self, func);
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        // Check arrow function parameters
        for param in &arrow.params.items {
            if let Some(name) = self.extract_variable_name(&param.pattern) {
                self.check_variable_name(&name, param.pattern.span());
            }
        }

        // Continue visiting arrow function body
        oxc_ast_visit::walk::walk_arrow_function_expression(self, arrow);
    }

    fn visit_catch_clause(&mut self, catch: &CatchClause<'a>) {
        if let Some(param) = &catch.param {
            if let Some(name) = self.extract_variable_name(param) {
                self.check_variable_name(&name, param.span());
            }
        }

        // Continue visiting catch clause body
        oxc_ast_visit::walk::walk_catch_clause(self, catch);
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

        check_no_vague_abbreviations(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_single_character_violation() {
        let code = r#"
function processData() {
    let u = getUserData(); // Bad: single character
    let v = validateInput(u); // Bad: single character
    return v;
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("only 1 character long")));
    }

    #[test]
    fn test_loop_counters_allowed() {
        let code = r#"
function processArray(items) {
    for (let i = 0; i < items.length; i++) {
        for (let j = 0; j < items[i].length; j++) {
            console.log(items[i][j]);
        }
    }
}
        "#;

        let issues = parse_and_check(code);
        // Loop counters i, j should be allowed
        assert!(issues.is_empty());
    }

    #[test]
    fn test_math_context_allowed() {
        let code = r#"
function calculateDistance(x1, y1, x2, y2) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    return Math.sqrt(dx * dx + dy * dy);
}
        "#;

        let issues = parse_and_check(code);
        // Math variables should be allowed
        assert!(issues.is_empty());
    }

    #[test]
    fn test_unclear_names_violation() {
        let code = r#"
function processData() {
    let data = getData(); // Bad: unclear name
    let obj = createObject(); // Bad: unclear name
    let temp = processTemp(data); // Bad: unclear name
    return temp;
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("unclear or ambiguous")));
    }

    #[test]
    fn test_suspicious_abbreviations_violation() {
        let code = r#"
function createUser() {
    let usr = "john"; // Bad: suspicious abbreviation
    let n1 = 42; // Bad: suspicious abbreviation
    let mgr = new Manager(); // Bad: suspicious abbreviation
    return { usr, n1, mgr };
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("unclear abbreviation")));
    }

    #[test]
    fn test_allowed_abbreviations_compliant() {
        let code = r#"
function createUser() {
    let id = generateId();
    let url = getApiUrl();
    let config = loadConfig();
    return { id, url, config };
}
        "#;

        let issues = parse_and_check(code);
        // Common abbreviations should be allowed
        assert!(issues.is_empty());
    }

    #[test]
    fn test_descriptive_names_compliant() {
        let code = r#"
function calculateUserScore(userProfile, gameSession) {
    let baseScore = gameSession.points;
    let multiplier = userProfile.level;
    let finalScore = baseScore * multiplier;
    return finalScore;
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }
}