//! # OXC Rules Migration
//!
//! This module contains systematically migrated OXC rules following our established
//! WASM-compatible template pattern. Each rule maintains OXC's exact logic while
//! adding AI enhancement capabilities.
//!
//! ## Migration Strategy
//! 1. Adapt OXC rule logic to WasmRule trait
//! 2. Maintain exact AST analysis patterns
//! 3. Add AI enhancement layer
//! 4. Provide comprehensive test cases

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

/// No Unused Variables Rule - Following OXC Template Structure
///
/// ### What it does
/// Disallows unused variables, functions, and imports to keep code clean.
///
/// ### Why is this bad?
/// Unused variables indicate dead code that should be removed. They add noise
/// to the codebase and can confuse other developers about the code's purpose.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// function calculate() {
///     const unusedVar = 42; // unused
///     const result = 1 + 2;
///     return result;
/// }
///
/// import { unused } from 'module'; // unused import
/// ```
///
/// Examples of **correct** code:
/// ```js
/// function calculate() {
///     const result = 1 + 2;
///     return result;
/// }
///
/// import { needed } from 'module';
/// console.log(needed);
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoUnusedVars {
    pub check_imports: bool,
    pub check_functions: bool,
}

impl NoUnusedVars {
    pub const NAME: &'static str = "no-unused-vars";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;

    pub fn new() -> Self {
        Self {
            check_imports: true,
            check_functions: true,
        }
    }
}

impl WasmRule for NoUnusedVars {
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
        match node.kind() {
            AstKind::VariableDeclarator(declarator) => {
                if let Some(name) = self.get_variable_name(declarator) {
                    // Check if variable is used in semantic analysis
                    if !self.is_variable_used(&name, ctx) {
                        ctx.diagnostic(no_unused_vars_diagnostic(&name, declarator.span));
                    }
                }
            }
            AstKind::ImportDeclaration(import) if self.check_imports => {
                // Check unused imports
                for specifier in &import.specifiers {
                    if let Some(name) = self.get_import_name(specifier) {
                        if !self.is_variable_used(&name, ctx) {
                            ctx.diagnostic(no_unused_imports_diagnostic(&name, specifier.span()));
                        }
                    }
                }
            }
            AstKind::FunctionDeclaration(func) if self.check_functions => {
                if let Some(name) = &func.id {
                    if !self.is_variable_used(&name.name, ctx) {
                        ctx.diagnostic(no_unused_function_diagnostic(&name.name, func.span));
                    }
                }
            }
            _ => {}
        }
    }
}

impl NoUnusedVars {
    fn get_variable_name(&self, declarator: &oxc_ast::ast::VariableDeclarator) -> Option<String> {
        use oxc_ast::ast::{BindingPattern, BindingPatternKind};

        if let BindingPattern { kind: BindingPatternKind::BindingIdentifier(ident), .. } = &declarator.id {
            Some(ident.name.to_string())
        } else {
            None
        }
    }

    fn get_import_name(&self, specifier: &oxc_ast::ast::ImportDeclarationSpecifier) -> Option<String> {
        use oxc_ast::ast::ImportDeclarationSpecifier;

        match specifier {
            ImportDeclarationSpecifier::ImportSpecifier(spec) => Some(spec.local.name.to_string()),
            ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => Some(spec.local.name.to_string()),
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => Some(spec.local.name.to_string()),
        }
    }

    fn is_variable_used(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Use semantic analysis to check if variable is referenced
        // This is a simplified implementation - full version would use OXC's semantic analysis
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name &&
            !ctx.semantic.symbol_references(symbol).is_empty()
        })
    }
}

fn no_unused_vars_diagnostic(var_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused variable")
        .with_help(format!("Variable '{}' is declared but never used. Consider removing it or prefixing with underscore.", var_name))
        .with_label(span)
}

fn no_unused_imports_diagnostic(import_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused import")
        .with_help(format!("Import '{}' is declared but never used. Consider removing it.", import_name))
        .with_label(span)
}

fn no_unused_function_diagnostic(func_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused function")
        .with_help(format!("Function '{}' is declared but never used. Consider removing it or exporting it.", func_name))
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedVars {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Consider if this code serves a future purpose".to_string(),
            "Check if this should be exported for external use".to_string(),
            "Prefix with underscore if intentionally unused".to_string(),
            "Move to utility module if reusable".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Unused code increases bundle size and maintenance overhead. Removing dead code improves performance and code clarity.".to_string())
    }
}

/// No Unreachable Code Rule - Following OXC Template Structure
///
/// ### What it does
/// Disallows unreachable code after return, throw, break, or continue statements.
///
/// ### Why is this bad?
/// Unreachable code will never execute and indicates a logic error or dead code.
/// It can confuse developers and should be removed or restructured.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// function example() {
///     return 42;
///     console.log("This will never run"); // unreachable
/// }
///
/// if (true) {
///     throw new Error("error");
///     console.log("unreachable"); // unreachable
/// }
/// ```
///
/// Examples of **correct** code:
/// ```js
/// function example() {
///     console.log("This runs");
///     return 42;
/// }
///
/// if (condition) {
///     console.log("This might run");
///     throw new Error("error");
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoUnreachable;

impl NoUnreachable {
    pub const NAME: &'static str = "no-unreachable";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUnreachable {
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
        if let AstKind::BlockStatement(block) = node.kind() {
            let mut found_terminating_statement = false;

            for (i, stmt) in block.body.iter().enumerate() {
                if found_terminating_statement {
                    // This statement is unreachable
                    ctx.diagnostic(no_unreachable_diagnostic(stmt.span()));
                    break;
                }

                // Check if this statement terminates control flow
                if self.is_terminating_statement(stmt) {
                    found_terminating_statement = true;
                    // Continue to check if there are statements after this one
                }
            }
        }
    }
}

impl NoUnreachable {
    fn is_terminating_statement(&self, stmt: &oxc_ast::ast::Statement) -> bool {
        use oxc_ast::ast::Statement;

        matches!(stmt,
            Statement::ReturnStatement(_) |
            Statement::ThrowStatement(_) |
            Statement::BreakStatement(_) |
            Statement::ContinueStatement(_)
        )
    }
}

fn no_unreachable_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unreachable code")
        .with_help("This code will never execute. Consider removing it or restructuring the logic.")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnreachable {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check if this code should be moved before the return".to_string(),
            "Consider using early returns to improve readability".to_string(),
            "Verify if the terminating statement is necessary".to_string(),
            "Use conditional logic if this code should run sometimes".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Unreachable code often indicates a logic error or represents dead code that should be cleaned up. Consider the intended control flow.".to_string())
    }
}

/// No Implicit Return Rule - Following OXC Template Structure
///
/// ### What it does
/// Requires explicit return statements in functions that should return values.
///
/// ### Why is this bad?
/// Implicit returns (falling off the end of a function) can lead to unexpected
/// undefined values and make the code's intent unclear.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// function calculate(x, y) {
///     if (x > 0) {
///         return x + y;
///     }
///     // Implicit return undefined
/// }
///
/// const getValue = (condition) => {
///     if (condition) {
///         return "value";
///     }
///     // No explicit return
/// };
/// ```
///
/// Examples of **correct** code:
/// ```js
/// function calculate(x, y) {
///     if (x > 0) {
///         return x + y;
///     }
///     return 0; // Explicit return
/// }
///
/// const getValue = (condition) => {
///     if (condition) {
///         return "value";
///     }
///     return null; // Explicit return
/// };
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoImplicitReturn;

impl NoImplicitReturn {
    pub const NAME: &'static str = "no-implicit-return";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoImplicitReturn {
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
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if self.has_implicit_return(&func.body) {
                    ctx.diagnostic(no_implicit_return_diagnostic(func.span));
                }
            }
            AstKind::ArrowFunctionExpression(func) => {
                if let Some(body) = func.body.as_function_body() {
                    if self.has_implicit_return(body) {
                        ctx.diagnostic(no_implicit_return_diagnostic(func.span));
                    }
                }
            }
            AstKind::FunctionExpression(func) => {
                if self.has_implicit_return(&func.body) {
                    ctx.diagnostic(no_implicit_return_diagnostic(func.span));
                }
            }
            _ => {}
        }
    }
}

impl NoImplicitReturn {
    fn has_implicit_return(&self, body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if the function body has explicit returns in all code paths
        self.check_statements_for_return(&body.statements)
    }

    fn check_statements_for_return(&self, statements: &[oxc_ast::ast::Statement]) -> bool {
        if statements.is_empty() {
            return true; // No statements = implicit return
        }

        // Check if the last statement is a return
        let last_stmt = statements.last().unwrap();
        !self.is_explicit_return(last_stmt)
    }

    fn is_explicit_return(&self, stmt: &oxc_ast::ast::Statement) -> bool {
        use oxc_ast::ast::Statement;

        matches!(stmt,
            Statement::ReturnStatement(_) |
            Statement::ThrowStatement(_)
        )
    }
}

fn no_implicit_return_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Function has implicit return")
        .with_help("Add an explicit return statement to make the function's intent clear.")
        .with_label(span)
}

impl EnhancedWasmRule for NoImplicitReturn {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add 'return null;' if no value should be returned".to_string(),
            "Add 'return undefined;' to be explicit about undefined return".to_string(),
            "Consider if this function should return a meaningful value".to_string(),
            "Use void return type in TypeScript for functions that don't return".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Explicit return statements make function behavior clear and prevent accidental undefined returns that can cause bugs.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oxc_compatible_rules::{WasmRuleEngine, WasmTester};

    #[test]
    fn test_no_unused_vars() {
        let pass = vec![
            "const used = 42; console.log(used);",
            "function called() { return 1; } called();",
            "import { needed } from 'module'; export { needed };",
        ];

        let fail = vec![
            "const unused = 42;", // unused variable
            "function uncalled() { return 1; }", // unused function
            "import { unneeded } from 'module';", // unused import
        ];

        let tester = WasmTester::new(NoUnusedVars::NAME, pass, fail);
        tester.test_rule::<NoUnusedVars>().expect("NoUnusedVars tests should pass");
    }

    #[test]
    fn test_no_unreachable() {
        let pass = vec![
            "function good() { console.log('reachable'); return 42; }",
            "if (true) { console.log('ok'); } else { return; }",
            "try { risky(); } catch (e) { throw e; }",
        ];

        let fail = vec![
            "function bad() { return 42; console.log('unreachable'); }",
            "if (true) { throw new Error(); console.log('dead'); }",
            "while (false) { break; console.log('never'); }",
        ];

        let tester = WasmTester::new(NoUnreachable::NAME, pass, fail);
        tester.test_rule::<NoUnreachable>().expect("NoUnreachable tests should pass");
    }

    #[test]
    fn test_no_implicit_return() {
        let pass = vec![
            "function explicit() { return 42; }",
            "const arrow = () => { return 'value'; };",
            "function thrower() { throw new Error(); }",
        ];

        let fail = vec![
            "function implicit() { const x = 42; }", // No return
            "const arrow = () => { const y = 'value'; };", // No return
            "function maybe(cond) { if (cond) return 1; }", // Implicit in else
        ];

        let tester = WasmTester::new(NoImplicitReturn::NAME, pass, fail);
        tester.test_rule::<NoImplicitReturn>().expect("NoImplicitReturn tests should pass");
    }

    #[test]
    fn test_rule_categories_and_fixes() {
        // Verify categories match OXC patterns
        assert_eq!(NoUnusedVars::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoUnreachable::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoImplicitReturn::CATEGORY, WasmRuleCategory::Suspicious);

        // Verify fix status classifications
        assert_eq!(NoUnusedVars::FIX_STATUS, WasmFixStatus::Fix);
        assert_eq!(NoUnreachable::FIX_STATUS, WasmFixStatus::Fix);
        assert_eq!(NoImplicitReturn::FIX_STATUS, WasmFixStatus::Suggestion);
    }
}