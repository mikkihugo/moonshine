//! Custom MoonShine rule for: C042 – Boolean variable names should start with descriptive prefixes
//! Rule ID: moonshine/c042
//! Purpose: Improve code readability by enforcing boolean naming conventions like 'is', 'has', 'should'
//!
//! Converted from JavaScript ESLint rule
//! @category code-quality-rules
//! @complexity high

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, BindingPatternKind, Expression, Function, PropertyKey, MethodDefinitionKind};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration options for C042 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C042Config {
    /// Allowed boolean prefixes (default: ["is", "has", "should", "can", "will", "must", "may", "was", "were"])
    #[serde(default)]
    pub allowed_prefixes: Vec<String>,
    /// Whether to enforce strict mode (default: false)
    #[serde(default)]
    pub strict_mode: bool,
    /// Names to ignore (default: ["flag", "enabled", "disabled", "active", "valid", "ok"])
    #[serde(default)]
    pub ignored_names: Vec<String>,
    /// Whether to check return types (default: true)
    #[serde(default = "default_check_return_types")]
    pub check_return_types: bool,
}

fn default_check_return_types() -> bool {
    true
}

impl Default for C042Config {
    fn default() -> Self {
        Self {
            allowed_prefixes: vec![
                "is".to_string(), "has".to_string(), "should".to_string(), "can".to_string(),
                "will".to_string(), "must".to_string(), "may".to_string(), "was".to_string(), "were".to_string()
            ],
            strict_mode: false,
            ignored_names: vec!["flag".to_string(), "enabled".to_string(), "disabled".to_string(),
                               "active".to_string(), "valid".to_string(), "ok".to_string()],
            check_return_types: true,
        }
    }
}

/// Main entry point for C042 rule checking
pub fn check_boolean_naming(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C042Config::default();
    let mut visitor = C042Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting boolean naming violations
struct C042Visitor<'a> {
    config: &'a C042Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    ignored_names: HashSet<String>,
}

impl<'a> C042Visitor<'a> {
    fn new(config: &'a C042Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let ignored_names: HashSet<String> = config.ignored_names
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            ignored_names,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, variable_name: &str, is_boolean: bool) -> String {
        if is_boolean {
            format!("Boolean variable '{}' should start with a descriptive prefix like 'is', 'has', or 'should' to clearly indicate its boolean nature. Consider: is{}, has{}, should{}",
                   variable_name, variable_name, variable_name, variable_name)
        } else {
            format!("Variable '{}' uses a boolean prefix but is not assigned a boolean value. This can be misleading - consider removing the prefix or changing the value type.",
                   variable_name)
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, variable_name: &str, is_boolean: bool) -> Vec<String> {
        if is_boolean {
            self.config.allowed_prefixes.iter()
                .map(|prefix| format!("{}{}", prefix, variable_name))
                .collect()
        } else {
            vec![
                variable_name.to_string(),
                format!("{}_flag", variable_name),
                format!("{}_enabled", variable_name),
            ]
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

    /// Check if name starts with boolean prefix
    fn starts_with_boolean_prefix(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();
        self.config.allowed_prefixes.iter().any(|prefix| lower_name.starts_with(&prefix.to_lowercase()))
    }

    /// Check if expression is a boolean literal
    fn is_boolean_literal(&self, expr: &Expression) -> bool {
        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::UnaryExpression(unary) => unary.operator == oxc_ast::ast::UnaryOperator::LogicalNot,
            Expression::BinaryExpression(binary) => {
                matches!(binary.operator,
                    oxc_ast::ast::BinaryOperator::StrictEquality |
                    oxc_ast::ast::BinaryOperator::StrictInequality |
                    oxc_ast::ast::BinaryOperator::Equality |
                    oxc_ast::ast::BinaryOperator::Inequality |
                    oxc_ast::ast::BinaryOperator::LessThan |
                    oxc_ast::ast::BinaryOperator::LessEqualThan |
                    oxc_ast::ast::BinaryOperator::GreaterThan |
                    oxc_ast::ast::BinaryOperator::GreaterEqualThan
                )
            },
            Expression::LogicalExpression(logical) => {
                matches!(logical.operator,
                    oxc_ast::ast::LogicalOperator::And |
                    oxc_ast::ast::LogicalOperator::Or
                )
            },
            _ => false,
        }
    }

    /// Check if expression is definitely not boolean
    fn is_definitely_not_boolean(&self, expr: &Expression) -> bool {
        match expr {
            Expression::StringLiteral(_) |
            Expression::NumericLiteral(_) |
            Expression::BigIntLiteral(_) => true,
            _ => false,
        }
    }

    /// Create lint issue for boolean naming violation with AI enhancement
    fn create_boolean_naming_issue(&self, variable_name: &str, span: oxc_span::Span, is_boolean: bool) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware suggestions
        let ai_enhanced_message = self.generate_ai_enhanced_message(variable_name, is_boolean);
        let ai_fix_suggestions = self.generate_ai_fix_suggestions(variable_name, is_boolean);
        let primary_fix = ai_fix_suggestions.first().cloned();

        LintIssue {
            rule_name: "moonshine/c042".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Check variable name for boolean naming violations
    fn check_variable_name(&mut self, name: &str, span: oxc_span::Span, init: Option<&Expression>) {
        if name.is_empty() || name.len() <= 2 {
            return;
        }

        if self.ignored_names.contains(&name.to_lowercase()) {
            return;
        }

        let has_boolean_prefix = self.starts_with_boolean_prefix(name);
        let is_boolean_value = init.map_or(false, |expr| self.is_boolean_literal(expr));

        if is_boolean_value && !has_boolean_prefix {
            self.issues.push(self.create_boolean_naming_issue(name, span, true));
        } else if has_boolean_prefix && !is_boolean_value && init.is_some() {
            // Check for improper prefix usage
            if let Some(expr) = init {
                if self.is_definitely_not_boolean(expr) {
                    self.issues.push(self.create_boolean_naming_issue(name, span, false));
                }
            }
        }
    }
}

impl<'a> Visit<'a> for C042Visitor<'a> {
    fn visit_variable_declarator(&mut self, declarator: &oxc_ast::ast::VariableDeclarator<'a>) {
        match &declarator.id.kind {
            BindingPatternKind::BindingIdentifier(ident) => {
                self.check_variable_name(&ident.name, ident.span, declarator.init.as_ref());
            },
            BindingPatternKind::ObjectPattern(obj_pattern) => {
                for prop in &obj_pattern.properties {
                    match prop {
                        oxc_ast::ast::BindingProperty::BindingElement(binding_elem) => {
                            if let BindingPatternKind::BindingIdentifier(ident) = &binding_elem.pattern.kind {
                                self.check_variable_name(&ident.name, ident.span, None);
                            }
                        },
                        _ => {}
                    }
                }
            },
            BindingPatternKind::ArrayPattern(arr_pattern) => {
                for element in arr_pattern.elements.iter().flatten() {
                    if let BindingPatternKind::BindingIdentifier(ident) = &element.kind {
                        self.check_variable_name(&ident.name, ident.span, None);
                    }
                }
            },
            _ => {}
        }

        // Continue visiting
        self.visit_binding_pattern(&declarator.id);
        if let Some(init) = &declarator.init {
            self.visit_expression(init);
        }
    }

    fn visit_function(&mut self, func: &Function<'a>, _: oxc_semantic::ScopeFlags) {
        // Check parameters
        for param in &func.params.items {
            if let BindingPatternKind::BindingIdentifier(ident) = &param.pattern.kind {
                self.check_variable_name(&ident.name, ident.span, None);
            }
        }

        // Continue visiting
        if let Some(body) = &func.body {
            self.visit_function_body(body);
        }
    }

    fn visit_property_definition(&mut self, prop: &oxc_ast::ast::PropertyDefinition<'a>) {
        if let PropertyKey::StaticIdentifier(ident) = &prop.key {
            if let Some(value) = &prop.value {
                if self.is_boolean_literal(value) {
                    self.check_variable_name(&ident.name, ident.span, Some(value));
                }
            }
        }

        // Continue visiting
        if let Some(value) = &prop.value {
            self.visit_expression(value);
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

        check_boolean_naming(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_boolean_without_prefix_violation() {
        let code = "const active = true;";
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/c042");
        assert!(issues[0].message.contains("Boolean variable 'active' should start with a descriptive prefix"));
    }

    #[test]
    fn test_boolean_with_proper_prefix_compliant() {
        let code = "const isActive = true;";
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_multiple_boolean_variables_mixed() {
        let code = r#"
            const isValid = true;
            const active = false;
            const hasPermission = true;
            const enabled = true;
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 2);
        assert!(issues[0].message.contains("active"));
        assert!(issues[1].message.contains("enabled"));
    }

    #[test]
    fn test_boolean_comparisons() {
        let code = "const isGreater = x > 5;";
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_ignored_names() {
        let code = r#"
            const flag = true;
            const enabled = true;
            const valid = true;
        "#;
        let issues = parse_and_check(code);

        // Only 'enabled' should trigger (flag and valid are ignored by default)
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("enabled"));
    }

    #[test]
    fn test_function_parameters() {
        let code = "function checkStatus(active: boolean) { return active; }";
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("active"));
    }

    #[test]
    fn test_object_properties() {
        let code = "const config = { active: true, isEnabled: false };";
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("active"));
    }
}