//! Custom MoonShine rule for: C006 – Function names must be verbs or verb-noun phrases
//! Rule ID: moonshine/c006
//! Purpose: Enforce function naming convention using verbs or verb-noun phrases to clearly indicate actions
//!
//! Converted from JavaScript ESLint rule
//! @category code-quality-rules
//! @complexity medium

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Function, Expression, PropertyDefinition, PropertyKey, ArrowFunctionExpression, MethodDefinition, Span};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};

/// Configuration options for C006 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C006Config {
    /// Additional verbs to allow (beyond common ones)
    #[serde(default)]
    pub allowed_verbs: Vec<String>,
    /// Allowed verb prefixes (default: get, set, is, has, can, should, etc.)
    #[serde(default)]
    pub allowed_prefixes: Vec<String>,
    /// Allow constructor functions (PascalCase) (default: true)
    #[serde(default = "default_allow_constructors")]
    pub allow_constructors: bool,
}

fn default_allow_constructors() -> bool {
    true
}

impl Default for C006Config {
    fn default() -> Self {
        Self {
            allowed_verbs: Vec::new(),
            allowed_prefixes: Vec::new(),
            allow_constructors: true,
        }
    }
}

/// Main entry point for C006 rule checking
pub fn check_function_naming(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C006Config::default();
    let mut visitor = C006Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting function naming violations
struct C006Visitor<'a> {
    config: &'a C006Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
}

impl<'a> C006Visitor<'a> {
    fn new(config: &'a C006Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, function_name: &str, issue_type: &str) -> String {
        match issue_type {
            "not_verb" => format!("Function name '{}' should start with a verb or verb-noun phrase. Consider renaming to something like 'get{}', 'set{}', or 'process{}'", function_name, function_name, function_name, function_name),
            "noun_only" => format!("Function name '{}' appears to be a noun only. Function names should indicate actions - consider adding a verb like 'get{}', 'create{}', or 'process{}'", function_name, function_name, function_name, function_name),
            _ => format!("Function name '{}' does not follow verb-noun naming convention. Use action-oriented names that clearly indicate what the function does.", function_name),
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, function_name: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Common verb prefixes
        let verbs = ["get", "set", "is", "has", "can", "should", "create", "update", "delete", "process", "handle", "validate", "parse", "convert", "transform"];

        for verb in &verbs {
            suggestions.push(format!("{}{}", verb, function_name));
        }

        // If it starts with a lowercase letter, suggest PascalCase for constructor
        if function_name.chars().next().map_or(false, |c| c.is_lowercase()) {
            suggestions.push(function_name.to_string());
        }

        suggestions
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

    /// Check if a name starts with a verb prefix
    fn starts_with_verb(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let lower_name = name.to_lowercase();

        // Check common verb prefixes
        let verb_prefixes = [
            "get", "set", "fetch", "load", "save", "store", "update", "delete", "remove",
            "create", "make", "build", "generate", "produce", "construct",
            "add", "insert", "append", "push", "pop", "shift", "unshift",
            "find", "search", "filter", "sort", "map", "reduce", "transform",
            "validate", "verify", "check", "test", "confirm", "ensure",
            "calculate", "compute", "process", "parse", "format", "convert",
            "send", "receive", "transmit", "broadcast", "emit", "dispatch",
            "open", "close", "start", "stop", "begin", "end", "finish",
            "show", "hide", "display", "render", "draw", "paint",
            "connect", "disconnect", "link", "unlink", "attach", "detach",
            "enable", "disable", "activate", "deactivate", "toggle",
            "is", "has", "can", "should", "will", "must", "may", "does",
            "handle", "manage", "control", "execute", "run", "invoke",
            "reset", "clear", "clean", "refresh", "reload", "restore",
        ];

        verb_prefixes.iter().any(|prefix| lower_name.starts_with(prefix))
    }

    /// Check if a name follows verb-noun pattern
    fn is_verb_noun_pattern(&self, name: &str) -> bool {
        self.starts_with_verb(name)
    }

    /// Check if a name is likely a noun-only (no verb prefix)
    fn is_likely_noun_only(&self, name: &str) -> bool {
        !self.starts_with_verb(name) && name.chars().next().map_or(false, |c| c.is_lowercase())
    }

    /// Check if name uses generic verbs that should be flagged
    fn is_generic_verb_usage(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();
        matches!(lower_name.as_str(),
            "do" | "handle" | "process" | "manage" | "execute" | "work" | "stuff" | "thing" | "data" |
            "dosomething" | "handleSomething" | "processSomething" | "manageSomething" | "executeSomething" |
            "doStuff" | "handleStuff" | "processStuff" | "manageStuff" | "executeStuff" |
            "doData" | "handleData" | "processData" | "manageData" | "executeData" |
            "doWork" | "handleWork" | "processWork" | "manageWork" | "executeWork"
        )
    }

    /// Create lint issue for function naming violation with AI enhancement
    fn create_function_naming_issue(&self, function_name: &str, span: Span, issue_type: &str) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware suggestions
        let ai_enhanced_message = self.generate_ai_enhanced_message(function_name, issue_type);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(function_name);

        LintIssue {
            rule_name: "moonshine/c006".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Check if a name follows verb-noun pattern
    fn is_verb_noun_pattern(&self, name: &str) -> bool {
        self.starts_with_verb(name)
    }

    /// Check if a name is likely a noun-only (no verb prefix)
    fn is_likely_noun_only(&self, name: &str) -> bool {
        !self.starts_with_verb(name) && name.chars().next().map_or(false, |c| c.is_lowercase())
    }

    /// Check if name uses generic verbs that should be flagged
    fn is_generic_verb_usage(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();
        matches!(lower_name.as_str(),
            "do" | "handle" | "process" | "manage" | "execute" | "work" | "stuff" | "thing" | "data" |
            "dosomething" | "handleSomething" | "processSomething" | "manageSomething" | "executeSomething" |
            "doStuff" | "handleStuff" | "processStuff" | "manageStuff" | "executeStuff" |
            "doData" | "handleData" | "processData" | "manageData" | "executeData" |
            "doWork" | "handleWork" | "processWork" | "manageWork" | "executeWork"
        )
    }

    /// Create lint issue for function naming violation with AI enhancement
    fn create_function_naming_issue(&self, function_name: &str, span: Span, issue_type: &str) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware suggestions
        let ai_enhanced_message = self.generate_ai_enhanced_message(function_name, issue_type);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(function_name);

        LintIssue {
            rule_name: "moonshine/c006".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }
}

impl<'a> Visit<'a> for C006Visitor<'a> {
    /// Visit function declarations
    fn visit_function(&mut self, func: &Function<'a>) {
        if let Some(id) = &func.id {
            self.check_function_name(&id.name, func.span);
        }

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_function(self, func);
    }

    /// Visit arrow function expressions
    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        // For arrow functions, we need to check if they're assigned to variables
        // This is a simplified check - in a full implementation we'd need parent context
        // For now, we'll skip arrow functions without variable names

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_arrow_function_expression(self, arrow);
    }

    /// Visit method definitions in classes
    fn visit_method_definition(&mut self, method: &MethodDefinition<'a>) {
        if let PropertyKey::StaticIdentifier(key) = &method.key {
            if key.name != "constructor" {
                self.check_function_name(&key.name, method.span);
            }
        }

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_method_definition(self, method);
    }

    /// Visit property definitions that may contain function expressions
    fn visit_property_definition(&mut self, prop: &PropertyDefinition<'a>) {
        if let PropertyKey::StaticIdentifier(key) = &prop.key {
            // Check if it's a method (has function value)
            if matches!(prop.value, Some(Expression::FunctionExpression(_)) | Some(Expression::ArrowFunctionExpression(_))) {
                self.check_function_name(&key.name, prop.span);
            }
        }

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_property_definition(self, prop);
    }
}

impl<'a> C006Visitor<'a> {
    /// Check function name and create issues if needed
    fn check_function_name(&mut self, name: &str, span: Span) {
        // Safety checks
        if name.is_empty() {
            return;
        }

        // Allow constructor functions (PascalCase)
        if self.config.allow_constructors && self.is_pascal_case(name) {
            return;
        }

        // Skip very short names (likely okay: a, b, fn, etc.)
        if name.len() <= 2 {
            return;
        }

        // Check if it follows verb-noun pattern
        if self.is_verb_noun_pattern(name) {
            // But still check if it's using generic verbs that should be flagged
            if self.is_generic_verb_usage(name) {
                self.issues.push(self.create_function_naming_issue(name, span, "not_verb"));
                return;
            }
            return; // Good! Follows the pattern and not generic
        }

        // Check if it's likely a noun-only name
        if self.is_likely_noun_only(name) {
            self.issues.push(self.create_function_naming_issue(name, span, "noun_only"));
            return;
        }

        // General violation - doesn't start with verb
        self.issues.push(self.create_function_naming_issue(name, span, "not_verb"));
    }

    /// Helper function to check if a name is PascalCase (likely a constructor)
    fn is_pascal_case(&self, name: &str) -> bool {
        let mut chars = name.chars();
        match chars.next() {
            Some(first) => first.is_uppercase(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    /// Helper function to parse code and run the rule
    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path("test.ts").unwrap();
        let parser = Parser::new(&allocator, code, source_type);
        let parse_result = parser.parse();

        if !parse_result.errors.is_empty() {
            panic!("Parse errors: {:?}", parse_result.errors);
        }

        let semantic = SemanticBuilder::new(code, source_type)
            .build(&parse_result.program)
            .semantic;

        check_function_naming(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_function_naming_violation() {
        let code = "function userData() { return {}; }";
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c006");
        assert!(issues[0].message.contains("Function name 'userData' appears to be a noun only"));
    }

    #[test]
    fn test_function_naming_compliant() {
        let code = "function getUserData() { return {}; }";
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_multiple_functions_mixed() {
        let code = r#"
            function getUser() { return {}; }
            function user() { return {}; }
            function setUserData() { return {}; }
            function data() { return {}; }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 2);
        assert!(issues[0].message.contains("user"));
        assert!(issues[1].message.contains("data"));
    }

    #[test]
    fn test_method_definitions() {
        let code = r#"
            class UserService {
                user() { return {}; }
                getUser() { return {}; }
                constructor() { }
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("user"));
    }

    #[test]
    fn test_generic_verb_violation() {
        let code = "function handle() { return {}; }";
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c006");
        assert!(issues[0].message.contains("handle"));
    }

    #[test]
    fn test_pascal_case_constructor_allowed() {
        let code = "function UserService() { return {}; }";
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_short_function_names_allowed() {
        let code = r#"
            function a() { return 1; }
            function fn() { return 2; }
            function x() { return 3; }
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_various_verb_prefixes() {
        let code = r#"
            function isValid() { return true; }
            function hasData() { return true; }
            function canProcess() { return true; }
            function shouldUpdate() { return true; }
            function validateInput() { return true; }
            function calculateTotal() { return 0; }
            function processData() { return {}; }
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }
}