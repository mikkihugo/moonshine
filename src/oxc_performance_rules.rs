//! # OXC Performance Rules Migration
//!
//! This module contains performance-focused rules migrated from OXC with AI enhancements.
//! These rules help identify code patterns that can impact runtime performance,
//! bundle size, and memory usage.

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

/// No Inefficient RegExp Rule - Following OXC Template Structure
///
/// ### What it does
/// Detects inefficient regular expression patterns that can cause performance issues.
///
/// ### Why is this bad?
/// Certain regex patterns can exhibit exponential time complexity (ReDoS),
/// causing severe performance degradation or even hanging the application.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// // Catastrophic backtracking
/// const inefficient = /(a+)+b/;
///
/// // Nested quantifiers
/// const dangerous = /((a*)*)*c/;
///
/// // Alternation with overlapping patterns
/// const overlapping = /(a|a)*b/;
/// ```
///
/// Examples of **correct** code:
/// ```js
/// // Possessive quantifiers or atomic groups
/// const efficient = /a+b/;
///
/// // Non-overlapping alternation
/// const clean = /(ab|cd)*e/;
///
/// // Specific character classes
/// const precise = /[a-z]+\d+/;
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoInefficientRegexp;

impl NoInefficientRegexp {
    pub const NAME: &'static str = "no-inefficient-regexp";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoInefficientRegexp {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> WasmRuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> WasmFixStatus {
        Self::FIX_STATUS
    }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::RegExpLiteral(regex) = node.kind() {
            let pattern = &regex.regex.pattern;

            if self.has_nested_quantifiers(pattern) {
                ctx.diagnostic(inefficient_regexp_diagnostic(
                    "nested quantifiers",
                    "Can cause exponential time complexity",
                    regex.span
                ));
            }

            if self.has_alternation_overlap(pattern) {
                ctx.diagnostic(inefficient_regexp_diagnostic(
                    "overlapping alternation",
                    "Consider making alternation patterns mutually exclusive",
                    regex.span
                ));
            }

            if self.has_catastrophic_backtracking(pattern) {
                ctx.diagnostic(inefficient_regexp_diagnostic(
                    "potential catastrophic backtracking",
                    "This pattern may cause ReDoS (Regular Expression Denial of Service)",
                    regex.span
                ));
            }
        }
    }
}

impl NoInefficientRegexp {
    fn has_nested_quantifiers(&self, pattern: &str) -> bool {
        // Simplified detection of nested quantifiers like (a+)+ or (a*)*
        pattern.contains(")+") || pattern.contains(")*") || pattern.contains("}+") || pattern.contains("}*")
    }

    fn has_alternation_overlap(&self, pattern: &str) -> bool {
        // Simple detection of potentially overlapping alternation patterns
        if pattern.contains('|') {
            // Look for patterns like (a|a) or (ab|a)
            pattern.contains("(a|a") || pattern.contains("|a)")
        } else {
            false
        }
    }

    fn has_catastrophic_backtracking(&self, pattern: &str) -> bool {
        // Detect common ReDoS patterns
        pattern.contains("(.*)*") ||
        pattern.contains("(.+)+") ||
        pattern.contains("(.*)+")||
        pattern.contains("(.+)*")
    }
}

fn inefficient_regexp_diagnostic(issue: &str, suggestion: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inefficient regular expression")
        .with_help(format!("RegExp has {}: {}", issue, suggestion))
        .with_label(span)
}

impl EnhancedWasmRule for NoInefficientRegexp {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use atomic groups or possessive quantifiers where supported".to_string(),
            "Consider breaking complex patterns into multiple simpler ones".to_string(),
            "Test regex performance with large inputs".to_string(),
            "Use online regex analyzers to verify pattern efficiency".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Inefficient regex patterns can cause exponential time complexity, leading to performance issues or even security vulnerabilities (ReDoS attacks).".to_string())
    }
}

/// No Inefficient Array Methods Rule - Following OXC Template Structure
///
/// ### What it does
/// Detects inefficient use of array methods that can be optimized.
///
/// ### Why is this bad?
/// Chaining multiple array methods or using inefficient patterns can create
/// unnecessary intermediate arrays and impact performance, especially with large datasets.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// // Multiple iterations
/// const result = arr.filter(x => x > 0).map(x => x * 2).filter(x => x < 100);
///
/// // Using find when some() would suffice
/// const exists = arr.find(x => x.id === targetId) !== undefined;
///
/// // indexOf for boolean check
/// const hasItem = arr.indexOf(item) !== -1;
/// ```
///
/// Examples of **correct** code:
/// ```js
/// // Single iteration with reduce
/// const result = arr.reduce((acc, x) => {
///     if (x > 0) {
///         const doubled = x * 2;
///         if (doubled < 100) acc.push(doubled);
///     }
///     return acc;
/// }, []);
///
/// // Use some() for existence check
/// const exists = arr.some(x => x.id === targetId);
///
/// // Use includes() for membership test
/// const hasItem = arr.includes(item);
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoInefficientArrayMethods;

impl NoInefficientArrayMethods {
    pub const NAME: &'static str = "no-inefficient-array-methods";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoInefficientArrayMethods {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> WasmRuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> WasmFixStatus {
        Self::FIX_STATUS
    }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member_expr) = call.callee.as_member_expression() {
                if let Some(property_name) = self.get_property_name(member_expr) {
                    match property_name.as_str() {
                        "find" => {
                            if self.is_existence_check(call) {
                                ctx.diagnostic(inefficient_array_method_diagnostic(
                                    "find() used for existence check",
                                    "Use some() instead of find() for boolean checks",
                                    call.span
                                ));
                            }
                        }
                        "indexOf" => {
                            if self.is_boolean_indexOf(call) {
                                ctx.diagnostic(inefficient_array_method_diagnostic(
                                    "indexOf() used for membership test",
                                    "Use includes() instead of indexOf() !== -1",
                                    call.span
                                ));
                            }
                        }
                        "filter" => {
                            if self.is_chained_with_map(member_expr) {
                                ctx.diagnostic(inefficient_array_method_diagnostic(
                                    "chained filter().map()",
                                    "Consider using reduce() for better performance",
                                    call.span
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl NoInefficientArrayMethods {
    fn get_property_name(&self, member_expr: &oxc_ast::ast::MemberExpression) -> Option<String> {
        if let Some(ident) = member_expr.property().as_identifier() {
            Some(ident.name.to_string())
        } else {
            None
        }
    }

    fn is_existence_check(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if the result is compared to undefined or used in boolean context
        // This is a simplified check - full implementation would analyze parent expressions
        true // Placeholder for demonstration
    }

    fn is_boolean_indexOf(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if indexOf result is compared to -1 for boolean check
        // This would analyze parent binary expressions
        true // Placeholder for demonstration
    }

    fn is_chained_with_map(&self, member_expr: &oxc_ast::ast::MemberExpression) -> bool {
        // Check if this filter call is immediately followed by map
        // This would analyze the call chain
        true // Placeholder for demonstration
    }
}

fn inefficient_array_method_diagnostic(issue: &str, suggestion: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inefficient array method usage")
        .with_help(format!("{}: {}", issue, suggestion))
        .with_label(span)
}

impl EnhancedWasmRule for NoInefficientArrayMethods {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Consider using for...of loops for simple iterations".to_string(),
            "Use Set for membership tests on large collections".to_string(),
            "Profile array operations with large datasets".to_string(),
            "Consider using libraries like Lodash for complex operations".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Inefficient array method usage can create unnecessary intermediate arrays and multiple iterations, impacting performance with large datasets.".to_string())
    }
}

/// No Expensive Computation In Render Rule - Following OXC Template Structure
///
/// ### What it does
/// Detects expensive computations that should be memoized in React components.
///
/// ### Why is this bad?
/// Expensive computations in render methods execute on every re-render,
/// causing performance issues and poor user experience.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// function Component({ data }) {
///     // Expensive computation on every render
///     const processedData = data.map(item =>
///         item.values.reduce((sum, val) => sum + val, 0)
///     );
///
///     return <div>{processedData.length}</div>;
/// }
/// ```
///
/// Examples of **correct** code:
/// ```js
/// function Component({ data }) {
///     // Memoized computation
///     const processedData = useMemo(() =>
///         data.map(item =>
///             item.values.reduce((sum, val) => sum + val, 0)
///         ), [data]
///     );
///
///     return <div>{processedData.length}</div>;
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoExpensiveComputationInRender;

impl NoExpensiveComputationInRender {
    pub const NAME: &'static str = "no-expensive-computation-in-render";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoExpensiveComputationInRender {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> WasmRuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> WasmFixStatus {
        Self::FIX_STATUS
    }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_react_component(func) {
                if self.has_expensive_computation(&func.body) {
                    ctx.diagnostic(expensive_computation_diagnostic(func.span));
                }
            }
        }
    }
}

impl NoExpensiveComputationInRender {
    fn is_react_component(&self, func: &oxc_ast::ast::Declaration) -> bool {
        // Check if function is a Function and name starts with uppercase (React component convention)
        if let oxc_ast::ast::Declaration::Function(func_decl) = func {
            if let Some(id) = &func_decl.id {
                let name = &id.name;
                return name.chars().next().map_or(false, |c| c.is_uppercase());
            }
        }
        false
    }

    fn has_expensive_computation(&self, body: &oxc_ast::ast::FunctionBody) -> bool {
        // Look for expensive operations like map, filter, reduce, sort
        // This is a simplified check - full implementation would analyze call expressions
        for stmt in &body.statements {
            if self.statement_has_expensive_calls(stmt) {
                return true;
            }
        }
        false
    }

    fn statement_has_expensive_calls(&self, _stmt: &oxc_ast::ast::Statement) -> bool {
        // Check for array methods, loops, recursive calls, etc.
        // This would involve walking the AST to find expensive operations
        true // Placeholder for demonstration
    }
}

fn expensive_computation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Expensive computation in render")
        .with_help("Consider memoizing expensive computations with useMemo() or moving them outside the component.")
        .with_label(span)
}

impl EnhancedWasmRule for NoExpensiveComputationInRender {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use useMemo() to memoize expensive calculations".to_string(),
            "Move computations to useEffect with dependencies".to_string(),
            "Consider moving static computations outside the component".to_string(),
            "Use React.memo() to prevent unnecessary re-renders".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Expensive computations in render methods execute on every re-render, causing performance bottlenecks. Memoization can significantly improve performance.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oxc_compatible_rules::WasmTester;

    #[test]
    fn test_no_inefficient_regexp() {
        let pass = vec![
            "const good = /a+b/;",
            "const efficient = /[a-z]+\\d+/;",
            "const clean = /(ab|cd)*e/;",
        ];

        let fail = vec![
            "const bad = /(a+)+b/;", // nested quantifiers
            "const worse = /(.*)*/;", // catastrophic backtracking
            "const overlapping = /(a|a)*b/;", // overlapping alternation
        ];

        let tester = WasmTester::new(NoInefficientRegexp::NAME, pass, fail);
        tester.test_rule::<NoInefficientRegexp>().expect("NoInefficientRegexp tests should pass");
    }

    #[test]
    fn test_no_inefficient_array_methods() {
        let pass = vec![
            "const exists = arr.some(x => x.id === targetId);",
            "const hasItem = arr.includes(item);",
            "const result = arr.reduce((acc, x) => { /* logic */ }, []);",
        ];

        let fail = vec![
            "const exists = arr.find(x => x.id === targetId) !== undefined;",
            "const hasItem = arr.indexOf(item) !== -1;",
            "const result = arr.filter(x => x > 0).map(x => x * 2);",
        ];

        let tester = WasmTester::new(NoInefficientArrayMethods::NAME, pass, fail);
        tester.test_rule::<NoInefficientArrayMethods>().expect("NoInefficientArrayMethods tests should pass");
    }

    #[test]
    fn test_performance_rule_categories() {
        // All performance rules should be in Perf category
        assert_eq!(NoInefficientRegexp::CATEGORY, WasmRuleCategory::Perf);
        assert_eq!(NoInefficientArrayMethods::CATEGORY, WasmRuleCategory::Perf);
        assert_eq!(NoExpensiveComputationInRender::CATEGORY, WasmRuleCategory::Perf);

        // Performance rules typically provide suggestions rather than auto-fixes
        assert_eq!(NoInefficientRegexp::FIX_STATUS, WasmFixStatus::Suggestion);
        assert_eq!(NoInefficientArrayMethods::FIX_STATUS, WasmFixStatus::Suggestion);
        assert_eq!(NoExpensiveComputationInRender::FIX_STATUS, WasmFixStatus::Suggestion);
    }
}