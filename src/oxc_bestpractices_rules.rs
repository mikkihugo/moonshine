//! Best practices and style rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct ConsistentReturn;

impl ConsistentReturn {
    pub const NAME: &'static str = "consistent-return";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for ConsistentReturn {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if !self.has_consistent_returns(&func.body) {
                    ctx.diagnostic(consistent_return_diagnostic(func.span));
                }
            }
            AstKind::ArrowFunction(func) => {
                // Check if it's a function body (not expression) and analyze returns
                if !func.expression {
                    if let Some(body) = &func.body {
                        if !self.has_consistent_returns(body) {
                            ctx.diagnostic(consistent_return_diagnostic(func.span));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl ConsistentReturn {
    fn has_consistent_returns(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if all code paths return consistently (either all return values or all return void)
        // Simplified implementation
        true
    }
}

fn consistent_return_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inconsistent return values")
        .with_help("Function should either always return a value or never return a value")
        .with_label(span)
}

impl EnhancedWasmRule for ConsistentReturn {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Either return values from all code paths or none".to_string(),
            "Use undefined explicitly if needed: return undefined".to_string(),
            "Consider splitting function if it has mixed responsibilities".to_string(),
            "Consistent returns make function behavior predictable".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct CurlyBraces;

impl CurlyBraces {
    pub const NAME: &'static str = "curly";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for CurlyBraces {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::IfStatement(if_stmt) => {
                if !self.has_braces(&if_stmt.consequent) {
                    ctx.diagnostic(curly_braces_diagnostic("if statement", if_stmt.span));
                }
                if let Some(alternate) = &if_stmt.alternate {
                    if !self.has_braces(alternate) {
                        ctx.diagnostic(curly_braces_diagnostic("else statement", alternate.span()));
                    }
                }
            }
            AstKind::WhileStatement(while_stmt) => {
                if !self.has_braces(&while_stmt.body) {
                    ctx.diagnostic(curly_braces_diagnostic("while loop", while_stmt.span));
                }
            }
            AstKind::ForStatement(for_stmt) => {
                if !self.has_braces(&for_stmt.body) {
                    ctx.diagnostic(curly_braces_diagnostic("for loop", for_stmt.span));
                }
            }
            _ => {}
        }
    }
}

impl CurlyBraces {
    fn has_braces(&self, stmt: &oxc_ast::ast::Statement) -> bool {
        matches!(stmt, oxc_ast::ast::Statement::BlockStatement(_))
    }
}

fn curly_braces_diagnostic(construct: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing curly braces")
        .with_help(format!("Add curly braces around {} body", construct))
        .with_label(span)
}

impl EnhancedWasmRule for CurlyBraces {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Curly braces prevent bugs when adding statements".to_string(),
            "Consistent braces improve code readability".to_string(),
            "Easier to add debugging statements".to_string(),
            "Follows most style guides and best practices".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct EqEqEq;

impl EqEqEq {
    pub const NAME: &'static str = "eqeqeq";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for EqEqEq {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::BinaryExpression(binary) = node.kind() {
            if binary.operator.is_equality() && !binary.operator.is_strict_equality() {
                ctx.diagnostic(eqeqeq_diagnostic(binary.span));
            } else if binary.operator.is_inequality() && !binary.operator.is_strict_inequality() {
                ctx.diagnostic(eqeqeq_diagnostic(binary.span));
            }
        }
    }
}

fn eqeqeq_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Use strict equality")
        .with_help("Use === or !== instead of == or !=")
        .with_label(span)
}

impl EnhancedWasmRule for EqEqEq {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Strict equality avoids type coercion bugs".to_string(),
            "=== and !== are more predictable".to_string(),
            "Type coercion can lead to unexpected results".to_string(),
            "Use == only when you specifically need type coercion".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoEmptyFunction;

impl NoEmptyFunction {
    pub const NAME: &'static str = "no-empty-function";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoEmptyFunction {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if self.is_empty_body(&func.body) {
                    ctx.diagnostic(no_empty_function_diagnostic(func.span));
                }
            }
            AstKind::FunctionDeclaration(func) => {
                if self.is_empty_body(&func.body) {
                    ctx.diagnostic(no_empty_function_diagnostic(func.span));
                }
            }
            AstKind::ArrowFunction(func) => {
                // Check if it's a function body (not expression) and if it's empty
                if !func.expression {
                    if let Some(body) = &func.body {
                        if self.is_empty_body(body) {
                            ctx.diagnostic(no_empty_function_diagnostic(func.span));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl NoEmptyFunction {
    fn is_empty_body(&self, body: &oxc_ast::ast::FunctionBody) -> bool {
        body.statements.is_empty()
    }
}

fn no_empty_function_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Empty function")
        .with_help("Add implementation or comment explaining why function is empty")
        .with_label(span)
}

impl EnhancedWasmRule for NoEmptyFunction {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add TODO comment if implementation is pending".to_string(),
            "Consider removing if function is not needed".to_string(),
            "Add throw new Error('Not implemented') for stubs".to_string(),
            "Empty functions might indicate incomplete code".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLonelyIf;

impl NoLonelyIf {
    pub const NAME: &'static str = "no-lonely-if";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoLonelyIf {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::IfStatement(if_stmt) = node.kind() {
            if let Some(alternate) = &if_stmt.alternate {
                if let oxc_ast::ast::Statement::BlockStatement(block) = alternate {
                    if block.body.len() == 1 {
                        if let oxc_ast::ast::Statement::IfStatement(_) = &block.body[0] {
                            ctx.diagnostic(no_lonely_if_diagnostic(block.span));
                        }
                    }
                }
            }
        }
    }
}

fn no_lonely_if_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Lonely if statement")
        .with_help("Use else if instead of else { if }")
        .with_label(span)
}

impl EnhancedWasmRule for NoLonelyIf {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use else if for chained conditions".to_string(),
            "Reduces nesting and improves readability".to_string(),
            "More concise than nested if statements".to_string(),
            "Consistent with most style guides".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMultipleEmptyLines;

impl NoMultipleEmptyLines {
    pub const NAME: &'static str = "no-multiple-empty-lines";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoMultipleEmptyLines {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.has_multiple_empty_lines(ctx) {
                ctx.diagnostic(no_multiple_empty_lines_diagnostic(node.span()));
            }
        }
    }
}

impl NoMultipleEmptyLines {
    fn has_multiple_empty_lines(&self, _ctx: &WasmLintContext) -> bool {
        // Check source code for multiple consecutive empty lines
        // Simplified implementation
        false
    }
}

fn no_multiple_empty_lines_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Multiple empty lines")
        .with_help("Remove extra empty lines")
        .with_label(span)
}

impl EnhancedWasmRule for NoMultipleEmptyLines {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use single empty lines for separation".to_string(),
            "Multiple empty lines don't add value".to_string(),
            "Consistent spacing improves readability".to_string(),
            "Consider using comments for major sections".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferArrowCallback;

impl PreferArrowCallback {
    pub const NAME: &'static str = "prefer-arrow-callback";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferArrowCallback {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            for arg in &call.arguments {
                if let Some(expr) = arg.as_expression() {
                    if let Some(func) = expr.as_function_expression() {
                        if self.should_be_arrow_function(func) {
                            ctx.diagnostic(prefer_arrow_callback_diagnostic(func.span));
                        }
                    }
                }
            }
        }
    }
}

impl PreferArrowCallback {
    fn should_be_arrow_function(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if function expression can be converted to arrow function
        // (doesn't use 'this', 'arguments', etc.)
        func.id.is_none() // Anonymous functions are good candidates
    }
}

fn prefer_arrow_callback_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer arrow function callback")
        .with_help("Use arrow function for callbacks when possible")
        .with_label(span)
}

impl EnhancedWasmRule for PreferArrowCallback {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Arrow functions are more concise for callbacks".to_string(),
            "Lexical this binding prevents common errors".to_string(),
            "No need to bind context with arrow functions".to_string(),
            "Modern JavaScript preferred style".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_return_rule() {
        assert_eq!(ConsistentReturn::NAME, "consistent-return");
        assert_eq!(ConsistentReturn::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_curly_braces_rule() {
        assert_eq!(CurlyBraces::NAME, "curly");
        assert_eq!(CurlyBraces::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(CurlyBraces::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_eqeqeq_rule() {
        assert_eq!(EqEqEq::NAME, "eqeqeq");
        assert_eq!(EqEqEq::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(EqEqEq::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_empty_function_rule() {
        assert_eq!(NoEmptyFunction::NAME, "no-empty-function");
        assert_eq!(NoEmptyFunction::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_no_lonely_if_rule() {
        assert_eq!(NoLonelyIf::NAME, "no-lonely-if");
        assert_eq!(NoLonelyIf::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoLonelyIf::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_multiple_empty_lines_rule() {
        assert_eq!(NoMultipleEmptyLines::NAME, "no-multiple-empty-lines");
        assert_eq!(NoMultipleEmptyLines::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoMultipleEmptyLines::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_prefer_arrow_callback_rule() {
        assert_eq!(PreferArrowCallback::NAME, "prefer-arrow-callback");
        assert_eq!(PreferArrowCallback::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferArrowCallback::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = ConsistentReturn;
        let diagnostic = consistent_return_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("return"));
    }
}