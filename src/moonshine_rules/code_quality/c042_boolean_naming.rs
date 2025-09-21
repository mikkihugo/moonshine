//! # C042: Boolean Naming Conventions
//!
//! This rule enforces proper naming conventions for boolean variables and functions
//! to improve code readability and maintain consistent coding standards.
//!
//! @category code-quality
//! @safe team
//! @mvp core
//! @complexity low
//! @since 1.0.0

use oxc_ast::ast::{Program, BindingPatternKind, Expression, Function, PropertyKey};
use oxc_ast_visit::Visit;
use oxc_semantic::{Semantic, ScopeFlags};
use oxc_span::Span;

/// OXC-based implementation for C042 boolean naming rule
pub struct BooleanNamingRule;

impl BooleanNamingRule {
    /// Create a new boolean naming rule instance
    pub fn new() -> Self {
        Self
    }

    /// Check boolean naming conventions using OXC AST visitor pattern
    /// Production implementation using proper AST traversal instead of regex patterns
    pub fn check_boolean_naming(&self, program: &Program, semantic: &Semantic, code: &str) -> Vec<String> {
        let mut visitor = BooleanNamingVisitor::new(program, code);
        visitor.visit_program(program);
        visitor.violations
    }

}

/// OXC AST visitor for boolean naming analysis
struct BooleanNamingVisitor<'a> {
    program: &'a Program<'a>,
    code: &'a str,
    violations: Vec<String>,
    acceptable_prefixes: Vec<&'static str>,
    acceptable_suffixes: Vec<&'static str>,
}

impl<'a> BooleanNamingVisitor<'a> {
    fn new(program: &'a Program<'a>, code: &'a str) -> Self {
        Self {
            program,
            code,
            violations: Vec::new(),
            acceptable_prefixes: vec![
                "is", "has", "can", "will", "should", "must", "does", "did", "was",
                "are", "were", "allow", "enable", "disable", "hide", "show",
                "supports", "contains", "includes", "excludes", "requires",
                "needs", "wants", "auto", "manual", "force", "strict",
            ],
            acceptable_suffixes: vec![
                "ed", "able", "ible", "ing", "Flag", "Enabled", "Disabled",
                "Available", "Required", "Optional", "Valid", "Invalid",
            ],
        }
    }

    fn check_boolean_name(&mut self, name: &str, span: Span, context: &str) {
        if name.is_empty() || name.len() <= 2 {
            return;
        }

        if !self.follows_boolean_naming_convention(name) {
            let (line, column) = self.span_to_line_col(span);
            let suggestions = self.generate_suggestions(name);

            self.violations.push(format!(
                "Line {}: Boolean identifier '{}' should follow naming conventions. Consider: {}",
                line, name, suggestions
            ));
        }
    }

    fn follows_boolean_naming_convention(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();

        // Check prefixes
        if self.acceptable_prefixes.iter().any(|prefix| lower_name.starts_with(prefix)) {
            return true;
        }

        // Check suffixes
        if self.acceptable_suffixes.iter().any(|suffix| lower_name.ends_with(&suffix.to_lowercase())) {
            return true;
        }

        // Allow common boolean abbreviations
        let acceptable_abbreviations = [
            "ok", "done", "ready", "active", "busy", "idle", "open", "closed",
            "on", "off", "up", "down", "live", "dead", "full", "empty",
        ];

        acceptable_abbreviations.contains(&lower_name.as_str())
    }

    fn generate_suggestions(&self, name: &str) -> String {
        let pascal_name = self.to_pascal_case(name);
        vec![
            format!("is{}", pascal_name),
            format!("has{}", pascal_name),
            format!("can{}", pascal_name),
            format!("should{}", pascal_name),
        ].join(", ")
    }

    fn to_pascal_case(&self, name: &str) -> String {
        if name.is_empty() {
            return name.to_string();
        }

        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn span_to_line_col(&self, span: Span) -> (usize, usize) {
        let source_text = self.code;
        let offset = span.start as usize;

        let mut line = 1;
        let mut column = 1;

        for (i, ch) in source_text.char_indices() {
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

    fn is_boolean_literal(&self, expr: &Expression) -> bool {
        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::UnaryExpression(unary) => {
                matches!(unary.operator, oxc_ast::ast::UnaryOperator::LogicalNot)
            },
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
}

impl<'a> Visit<'a> for BooleanNamingVisitor<'a> {
    fn visit_variable_declarator(&mut self, declarator: &oxc_ast::ast::VariableDeclarator<'a>) {
        if let BindingPatternKind::BindingIdentifier(ident) = &declarator.id.kind {
            // Check if this is a boolean variable
            if let Some(init) = &declarator.init {
                if self.is_boolean_literal(init) {
                    self.check_boolean_name(&ident.name, ident.span, "variable declaration");
                }
            }
        }

        // Continue visiting
        self.visit_binding_pattern(&declarator.id);
        if let Some(init) = &declarator.init {
            self.visit_expression(init);
        }
    }

    fn visit_function(&mut self, func: &Function<'a>, _flags: ScopeFlags) {
        // Check function parameters
        for param in &func.params.items {
            if let BindingPatternKind::BindingIdentifier(ident) = &param.pattern.kind {
                // We would need semantic analysis to determine if this parameter is boolean-typed
                // For now, we can check naming patterns for parameters that look boolean-ish
                let name = &ident.name;
                if name.starts_with("is") || name.starts_with("has") || name.starts_with("should") {
                    self.check_boolean_name(&ident.name, ident.span, "function parameter");
                }
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
                    self.check_boolean_name(&ident.name, ident.span, "property definition");
                }
            }
        }

        // Continue visiting
        if let Some(value) = &prop.value {
            self.visit_expression(value);
        }
    }
}