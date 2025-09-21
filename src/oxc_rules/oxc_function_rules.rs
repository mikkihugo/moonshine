//! Function definition and usage rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoFuncAssign;

impl NoFuncAssign {
    pub const NAME: &'static str = "no-func-assign";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoFuncAssign {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if let Some(ident) = assign.left.as_identifier() {
                if self.is_function_name(&ident.name, ctx) {
                    ctx.diagnostic(no_func_assign_diagnostic(&ident.name, assign.span));
                }
            }
        }
    }
}

impl NoFuncAssign {
    fn is_function_name(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if this identifier is a function declaration
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name &&
            // Check if symbol represents a function using proper SymbolFlags
            symbol.flags().contains(oxc_semantic::SymbolFlags::Function)
        })
    }
}

fn no_func_assign_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Function assignment")
        .with_help(format!("'{}' is a function, reassigning it can cause issues", name))
        .with_label(span)
}

impl EnhancedWasmRule for NoFuncAssign {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Use const for function expressions to prevent reassignment".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferArrowFunctions;

impl PreferArrowFunctions {
    pub const NAME: &'static str = "prefer-arrow-functions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferArrowFunctions {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Expression(expr) = node.kind() {
            if let oxc_ast::ast::Expression::Function(func) = expr {
                if !self.uses_this_or_arguments(expr) {
                    ctx.diagnostic(prefer_arrow_function_diagnostic(func.span));
                }
            }
        }
    }
}

impl PreferArrowFunctions {
    fn uses_this_or_arguments(&self, _expr: &oxc_ast::ast::Expression) -> bool {
        // Simplified check - would need to walk function body
        false
    }
}

fn prefer_arrow_function_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer arrow function")
        .with_help("Use arrow function for simpler syntax")
        .with_label(span)
}

impl EnhancedWasmRule for PreferArrowFunctions {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Arrow functions have lexical this binding".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoInnerDeclarations;

impl NoInnerDeclarations {
    pub const NAME: &'static str = "no-inner-declarations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoInnerDeclarations {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_in_nested_scope(&func.span, ctx) {
                ctx.diagnostic(no_inner_declaration_diagnostic(func.span));
            }
        }
    }
}

impl NoInnerDeclarations {
    fn is_in_nested_scope(&self, _span: &Span, _ctx: &WasmLintContext) -> bool {
        // Check if function is declared inside block or other nested scope
        true // Simplified for now
    }
}

fn no_inner_declaration_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inner function declaration")
        .with_help("Move function declaration to outer scope or use function expression")
        .with_label(span)
}

impl EnhancedWasmRule for NoInnerDeclarations {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Function declarations are hoisted, expressions are not".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireYield;

impl RequireYield {
    pub const NAME: &'static str = "require-yield";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireYield {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) if func.generator => {
                if !self.has_yield_expression(&func.body) {
                    ctx.diagnostic(require_yield_diagnostic(func.span));
                }
            }
            AstKind::FunctionDeclaration(func) if func.generator => {
                if !self.has_yield_expression(&func.body) {
                    ctx.diagnostic(require_yield_diagnostic(func.span));
                }
            }
            _ => {}
        }
    }
}

impl RequireYield {
    fn has_yield_expression(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Would need to walk the AST to find YieldExpression nodes
        false // Simplified for now
    }
}

fn require_yield_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Generator function without yield")
        .with_help("Generator functions should contain at least one yield expression")
        .with_label(span)
}

impl EnhancedWasmRule for RequireYield {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Consider if this should be a regular function instead".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoRedundantReturn;

impl NoRedundantReturn {
    pub const NAME: &'static str = "no-redundant-return";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoRedundantReturn {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ReturnStatement(ret) = node.kind() {
            if ret.argument.is_none() && self.is_at_end_of_function(ret, ctx) {
                ctx.diagnostic(redundant_return_diagnostic(ret.span));
            }
        }
    }
}

impl NoRedundantReturn {
    fn is_at_end_of_function(&self, _ret: &oxc_ast::ast::ReturnStatement, _ctx: &WasmLintContext) -> bool {
        // Check if this return is the last statement in the function
        true // Simplified for now
    }
}

fn redundant_return_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Redundant return statement")
        .with_help("Remove unnecessary return at end of function")
        .with_label(span)
}

impl EnhancedWasmRule for NoRedundantReturn {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Functions return undefined by default".to_string()]
    }
}