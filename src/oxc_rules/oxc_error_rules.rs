//! Error handling and exception rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoEmptyCatch;

impl NoEmptyCatch {
    pub const NAME: &'static str = "no-empty-catch";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoEmptyCatch {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CatchClause(catch) = node.kind() {
            if catch.body.body.is_empty() {
                ctx.diagnostic(empty_catch_diagnostic(catch.span));
            }
        }
    }
}

fn empty_catch_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Empty catch block")
        .with_help("Add error handling logic or at least log the error")
        .with_label(span)
}

impl EnhancedWasmRule for NoEmptyCatch {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Log errors for debugging: console.error(err)".to_string(),
            "Consider re-throwing if error cannot be handled".to_string(),
            "Add meaningful error recovery logic".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnhandledRejection;

impl NoUnhandledRejection {
    pub const NAME: &'static str = "no-unhandled-rejection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoUnhandledRejection {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_promise_call(call) && !self.has_catch_handler(call, ctx) {
                ctx.diagnostic(unhandled_rejection_diagnostic(call.span));
            }
        }
    }
}

impl NoUnhandledRejection {
    fn is_promise_call(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "then" || prop.name == "catch";
            }
        }
        false
    }

    fn has_catch_handler(&self, _call: &oxc_ast::ast::CallExpression, _ctx: &WasmLintContext) -> bool {
        // Check if promise chain has catch handler
        // Simplified implementation - would need to analyze the promise chain
        false
    }
}

fn unhandled_rejection_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unhandled promise rejection")
        .with_help("Add .catch() handler or use try/catch with async/await")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnhandledRejection {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use .catch() to handle promise rejections".to_string(),
            "Consider using async/await with try/catch".to_string(),
            "Unhandled rejections can crash Node.js applications".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferPromiseRejectWithError;

impl PreferPromiseRejectWithError {
    pub const NAME: &'static str = "prefer-promise-reject-errors";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferPromiseRejectWithError {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_promise_reject(call) {
                if let Some(arg) = call.arguments.first() {
                    if !self.is_error_object(&arg.as_expression().unwrap()) {
                        ctx.diagnostic(promise_reject_error_diagnostic(call.span));
                    }
                }
            }
        }
    }
}

impl PreferPromiseRejectWithError {
    fn is_promise_reject(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                if obj.name == "Promise" {
                    if let Some(prop) = member.property().as_identifier() {
                        return prop.name == "reject";
                    }
                }
            }
        }
        false
    }

    fn is_error_object(&self, expr: &oxc_ast::ast::Expression) -> bool {
        // Check if expression creates an Error object
        if let Some(call) = expr.as_call_expression() {
            if let Some(ident) = call.callee.as_identifier() {
                return ident.name.ends_with("Error");
            }
        }
        false
    }
}

fn promise_reject_error_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Promise reject with non-Error")
        .with_help("Use Error objects when rejecting promises")
        .with_label(span)
}

impl EnhancedWasmRule for PreferPromiseRejectWithError {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Error objects provide stack traces".to_string(),
            "Use new Error('message') for rejections".to_string(),
            "Consistent error types improve debugging".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoThrowLiteral;

impl NoThrowLiteral {
    pub const NAME: &'static str = "no-throw-literal";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoThrowLiteral {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ThrowStatement(throw) = node.kind() {
            if self.is_literal(&throw.argument) {
                ctx.diagnostic(throw_literal_diagnostic(throw.span));
            }
        }
    }
}

impl NoThrowLiteral {
    fn is_literal(&self, expr: &oxc_ast::ast::Expression) -> bool {
        matches!(expr,
            oxc_ast::ast::Expression::StringLiteral(_) |
            oxc_ast::ast::Expression::NumericLiteral(_) |
            oxc_ast::ast::Expression::BooleanLiteral(_) |
            oxc_ast::ast::Expression::NullLiteral(_)
        )
    }
}

fn throw_literal_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Throwing literal value")
        .with_help("Throw Error objects instead of literals")
        .with_label(span)
}

impl EnhancedWasmRule for NoThrowLiteral {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Error objects for better stack traces".to_string(),
            "throw new Error('message') provides more context".to_string(),
            "Error objects are caught consistently".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireAwait;

impl RequireAwait {
    pub const NAME: &'static str = "require-await";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAwait {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) if func.r#async => {
                if !self.has_await_expression(&func.body) {
                    ctx.diagnostic(require_await_diagnostic(func.span));
                }
            }
            AstKind::ArrowFunction(func) if func.r#async => {
                if !self.has_await_in_arrow_body(&func.body) {
                    ctx.diagnostic(require_await_diagnostic(func.span));
                }
            }
            _ => {}
        }
    }
}

impl RequireAwait {
    fn has_await_expression(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if function body contains await expressions
        // Simplified implementation - would need to walk AST
        false
    }

    fn has_await_in_arrow_body(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if arrow function body contains await
        false
    }
}

fn require_await_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async function without await")
        .with_help("Remove 'async' keyword or add 'await' expressions")
        .with_label(span)
}

impl EnhancedWasmRule for RequireAwait {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove async if no await is needed".to_string(),
            "Unnecessary async creates promise overhead".to_string(),
            "Consider if this should be a regular function".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_empty_catch_rule() {
        assert_eq!(NoEmptyCatch::NAME, "no-empty-catch");
        assert_eq!(NoEmptyCatch::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_no_unhandled_rejection_rule() {
        assert_eq!(NoUnhandledRejection::NAME, "no-unhandled-rejection");
        assert_eq!(NoUnhandledRejection::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_prefer_promise_reject_errors_rule() {
        assert_eq!(PreferPromiseRejectWithError::NAME, "prefer-promise-reject-errors");
        assert_eq!(PreferPromiseRejectWithError::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_throw_literal_rule() {
        assert_eq!(NoThrowLiteral::NAME, "no-throw-literal");
        assert_eq!(NoThrowLiteral::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(NoThrowLiteral::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_await_rule() {
        assert_eq!(RequireAwait::NAME, "require-await");
        assert_eq!(RequireAwait::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_literal_detection() {
        let rule = NoThrowLiteral;
        // Would test with actual AST nodes in real implementation
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoEmptyCatch;
        let diagnostic = empty_catch_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Log errors"));
    }
}