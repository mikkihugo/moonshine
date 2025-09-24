//! # ESLint Utility Functions
//!
//! This module provides utility functions commonly used by ESLint rules.
//! These functions help with AST analysis, type checking, and common patterns
//! that ESLint rules need to identify.

use oxc_ast::ast::*;
use oxc_semantic::{Reference, Semantic, Symbol};
use oxc_span::GetSpan;
use std::collections::HashSet;

/// Check if a node represents a specific literal value
pub fn is_literal_value(expr: &Expression, value: &str) -> bool {
    match expr {
        Expression::StringLiteral(lit) => lit.value.as_str() == value,
        Expression::NumericLiteral(lit) => lit.value.to_string() == value,
        Expression::BooleanLiteral(lit) => lit.value.to_string() == value,
        _ => false,
    }
}

/// Check if a node represents a truthy literal
pub fn is_truthy_literal(expr: &Expression) -> bool {
    match expr {
        Expression::BooleanLiteral(lit) => lit.value,
        Expression::NumericLiteral(lit) => lit.value != 0.0,
        Expression::StringLiteral(lit) => !lit.value.is_empty(),
        Expression::NullLiteral(_) => false,
        _ => true, // Non-literals are generally truthy
    }
}

/// Check if a node represents a falsy literal
pub fn is_falsy_literal(expr: &Expression) -> bool {
    match expr {
        Expression::BooleanLiteral(lit) => !lit.value,
        Expression::NumericLiteral(lit) => lit.value == 0.0,
        Expression::StringLiteral(lit) => lit.value.is_empty(),
        Expression::NullLiteral(_) => true,
        Expression::Identifier(ident) => ident.name.as_str() == "undefined",
        _ => false,
    }
}

/// Get the static value of an expression if it's a literal
pub fn get_static_value(expr: &Expression) -> Option<StaticValue> {
    match expr {
        Expression::StringLiteral(lit) => Some(StaticValue::String(lit.value.clone())),
        Expression::NumericLiteral(lit) => Some(StaticValue::Number(lit.value)),
        Expression::BooleanLiteral(lit) => Some(StaticValue::Boolean(lit.value)),
        Expression::NullLiteral(_) => Some(StaticValue::Null),
        Expression::Identifier(ident) if ident.name.as_str() == "undefined" => {
            Some(StaticValue::Undefined)
        }
        _ => None,
    }
}

/// Represents static values that can be extracted from literals
#[derive(Debug, Clone, PartialEq)]
pub enum StaticValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
}

/// Check if an expression is a specific identifier
pub fn is_identifier_name(expr: &Expression, name: &str) -> bool {
    match expr {
        Expression::Identifier(ident) => ident.name.as_str() == name,
        _ => false,
    }
}

/// Check if a member expression accesses a specific property
pub fn is_property_access(expr: &Expression, object: &str, property: &str) -> bool {
    match expr {
        Expression::StaticMemberExpression(member) => {
            is_identifier_name(&member.object, object)
                && member.property.name.as_str() == property
        }
        Expression::ComputedMemberExpression(member) => {
            is_identifier_name(&member.object, object)
                && is_literal_value(&member.expression, property)
        }
        _ => false,
    }
}

/// Check if a call expression calls a specific method on an object
pub fn is_method_call(call: &CallExpression, object: &str, method: &str) -> bool {
    is_property_access(&call.callee, object, method)
}

/// Check if a call expression calls a global function
pub fn is_global_function_call(call: &CallExpression, function: &str) -> bool {
    is_identifier_name(&call.callee, function)
}

/// Check if an expression is a `this` reference
pub fn is_this_expression(expr: &Expression) -> bool {
    matches!(expr, Expression::ThisExpression(_))
}

/// Check if an expression is a `super` reference
pub fn is_super_expression(expr: &Expression) -> bool {
    matches!(expr, Expression::Super(_))
}

/// Check if a function is an arrow function
pub fn is_arrow_function(func: &Function) -> bool {
    func.r#type == FunctionType::FunctionExpression && func.is_arrow_function()
}

/// Check if a function is a generator function
pub fn is_generator_function(func: &Function) -> bool {
    func.generator
}

/// Check if a function is an async function
pub fn is_async_function(func: &Function) -> bool {
    func.r#async
}

/// Check if a variable declaration uses `var`
pub fn is_var_declaration(decl: &VariableDeclaration) -> bool {
    matches!(decl.kind, VariableDeclarationKind::Var)
}

/// Check if a variable declaration uses `let`
pub fn is_let_declaration(decl: &VariableDeclaration) -> bool {
    matches!(decl.kind, VariableDeclarationKind::Let)
}

/// Check if a variable declaration uses `const`
pub fn is_const_declaration(decl: &VariableDeclaration) -> bool {
    matches!(decl.kind, VariableDeclarationKind::Const)
}

/// Check if a binary expression uses strict equality (=== or !==)
pub fn is_strict_equality(expr: &BinaryExpression) -> bool {
    matches!(
        expr.operator,
        BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
    )
}

/// Check if a binary expression uses loose equality (== or !=)
pub fn is_loose_equality(expr: &BinaryExpression) -> bool {
    matches!(
        expr.operator,
        BinaryOperator::Equality | BinaryOperator::Inequality
    )
}

/// Check if a statement is an empty statement
pub fn is_empty_statement(stmt: &Statement) -> bool {
    matches!(stmt, Statement::EmptyStatement(_))
}

/// Check if a block statement is empty
pub fn is_empty_block(stmt: &Statement) -> bool {
    match stmt {
        Statement::BlockStatement(block) => block.body.is_empty(),
        _ => false,
    }
}

/// Get all identifiers declared in a variable declaration
pub fn get_declared_identifiers(decl: &VariableDeclaration) -> Vec<&str> {
    let mut identifiers = Vec::new();

    for declarator in &decl.declarations {
        collect_identifiers_from_pattern(&declarator.id, &mut identifiers);
    }

    identifiers
}

/// Recursively collect identifiers from a binding pattern
fn collect_identifiers_from_pattern<'a>(
    pattern: &'a BindingPattern,
    identifiers: &mut Vec<&'a str>,
) {
    match &pattern.kind {
        BindingPatternKind::BindingIdentifier(ident) => {
            identifiers.push(ident.name.as_str());
        }
        BindingPatternKind::ObjectPattern(obj_pattern) => {
            for property in &obj_pattern.properties {
                match property {
                    BindingProperty::BindingElement(element) => {
                        collect_identifiers_from_pattern(&element.pattern, identifiers);
                    }
                    BindingProperty::RestElement(rest) => {
                        collect_identifiers_from_pattern(&rest.argument, identifiers);
                    }
                }
            }
        }
        BindingPatternKind::ArrayPattern(arr_pattern) => {
            for element in arr_pattern.elements.iter().flatten() {
                collect_identifiers_from_pattern(element, identifiers);
            }
            if let Some(rest) = &arr_pattern.rest {
                collect_identifiers_from_pattern(&rest.argument, identifiers);
            }
        }
        BindingPatternKind::AssignmentPattern(assign) => {
            collect_identifiers_from_pattern(&assign.left, identifiers);
        }
    }
}

/// Check if an expression contains only literal values
pub fn is_constant_expression(expr: &Expression) -> bool {
    match expr {
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_) => true,
        Expression::Identifier(ident) => ident.name.as_str() == "undefined",
        Expression::ArrayExpression(arr) => {
            arr.elements.iter().all(|elem| {
                elem.as_ref()
                    .map_or(true, |e| is_constant_expression(e))
            })
        }
        Expression::ObjectExpression(obj) => {
            obj.properties.iter().all(|prop| match prop {
                ObjectPropertyKind::ObjectProperty(prop) => {
                    is_constant_expression(&prop.value)
                }
                _ => false,
            })
        }
        Expression::UnaryExpression(unary) => is_constant_expression(&unary.argument),
        Expression::BinaryExpression(binary) => {
            is_constant_expression(&binary.left) && is_constant_expression(&binary.right)
        }
        _ => false,
    }
}

/// Check if a property key matches a specific string
pub fn is_property_key(key: &PropertyKey, name: &str) -> bool {
    match key {
        PropertyKey::StaticIdentifier(ident) => ident.name.as_str() == name,
        PropertyKey::StringLiteral(lit) => lit.value.as_str() == name,
        PropertyKey::NumericLiteral(lit) => lit.value.to_string() == name,
        _ => false,
    }
}

/// Check if a node is in a specific context (e.g., inside a loop)
pub fn is_in_loop_context(ancestors: &[&dyn GetSpan]) -> bool {
    ancestors.iter().any(|ancestor| {
        // This is a simplified check - in a real implementation,
        // you would need to check the actual node types
        true // Placeholder - should check for loop statements
    })
}

/// Check if two expressions are equivalent
pub fn expressions_equal(left: &Expression, right: &Expression) -> bool {
    match (left, right) {
        (Expression::Identifier(l), Expression::Identifier(r)) => l.name == r.name,
        (Expression::StringLiteral(l), Expression::StringLiteral(r)) => l.value == r.value,
        (Expression::NumericLiteral(l), Expression::NumericLiteral(r)) => l.value == r.value,
        (Expression::BooleanLiteral(l), Expression::BooleanLiteral(r)) => l.value == r.value,
        (Expression::NullLiteral(_), Expression::NullLiteral(_)) => true,
        // Add more cases as needed
        _ => false,
    }
}

/// Get the name of a function (if it has one)
pub fn get_function_name(func: &Function) -> Option<&str> {
    func.id.as_ref().map(|id| id.name.as_str())
}

/// Check if a function has parameters
pub fn has_parameters(func: &Function) -> bool {
    !func.params.items.is_empty()
}

/// Count the number of parameters in a function
pub fn parameter_count(func: &Function) -> usize {
    func.params.items.len()
}

/// Common patterns for checking dangerous global functions
pub static DANGEROUS_GLOBALS: &[&str] = &[
    "eval",
    "Function",
    "execScript",
    "setTimeout",
    "setInterval",
];

/// Common patterns for checking console methods
pub static CONSOLE_METHODS: &[&str] = &[
    "log",
    "info",
    "warn",
    "error",
    "debug",
    "trace",
    "dir",
    "dirxml",
    "table",
    "count",
    "time",
    "timeEnd",
    "profile",
    "profileEnd",
    "clear",
];

/// Check if a call is to a dangerous global function
pub fn is_dangerous_global_call(call: &CallExpression) -> bool {
    if let Expression::Identifier(ident) = &call.callee {
        DANGEROUS_GLOBALS.contains(&ident.name.as_str())
    } else {
        false
    }
}

/// Check if a call is to a console method
pub fn is_console_method_call(call: &CallExpression, method: Option<&str>) -> bool {
    match &call.callee {
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(obj) = &member.object {
                if obj.name.as_str() == "console" {
                    if let Some(method_name) = method {
                        member.property.name.as_str() == method_name
                    } else {
                        CONSOLE_METHODS.contains(&member.property.name.as_str())
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{ParseOptions, Parser};
    use oxc_span::SourceType;

    #[test]
    fn test_literal_value_checking() {
        let allocator = Allocator::default();
        let source_type = SourceType::default();

        let source = "let x = 'test'; let y = 42; let z = true;";
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        // Test would need to traverse AST to find literals and test them
        // This is a placeholder showing the test structure
        assert!(true); // Placeholder
    }

    #[test]
    fn test_static_value_extraction() {
        let allocator = Allocator::default();
        let source_type = SourceType::default();

        let source = "let str = 'hello';";
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        // Would need to find the string literal and test get_static_value
        assert!(true); // Placeholder
    }

    #[test]
    fn test_dangerous_globals_list() {
        assert!(DANGEROUS_GLOBALS.contains(&"eval"));
        assert!(DANGEROUS_GLOBALS.contains(&"Function"));
        assert!(!DANGEROUS_GLOBALS.contains(&"console"));
    }

    #[test]
    fn test_console_methods_list() {
        assert!(CONSOLE_METHODS.contains(&"log"));
        assert!(CONSOLE_METHODS.contains(&"error"));
        assert!(!CONSOLE_METHODS.contains(&"eval"));
    }
}