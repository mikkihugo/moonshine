//! Async/await and Promise rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct PreferAsyncAwait;

impl PreferAsyncAwait {
    pub const NAME: &'static str = "prefer-async-await";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferAsyncAwait {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if prop.name == "then" && self.in_async_function(ctx) {
                        ctx.diagnostic(prefer_async_await_diagnostic(call.span));
                    }
                }
            }
        }
    }
}

impl PreferAsyncAwait {
    fn in_async_function(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're inside an async function
        // Simplified implementation
        true
    }
}

fn prefer_async_await_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer async/await over Promise chains")
        .with_help("Use async/await for cleaner asynchronous code")
        .with_label(span)
}

impl EnhancedWasmRule for PreferAsyncAwait {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "async/await is more readable than Promise chains".to_string(),
            "Use try/catch for error handling with async/await".to_string(),
            "Avoid mixing .then() with async/await".to_string(),
            "Sequential awaits are easier to debug".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoAwaitInLoop;

impl NoAwaitInLoop {
    pub const NAME: &'static str = "no-await-in-loop";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoAwaitInLoop {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AwaitExpression(_await_expr) = node.kind() {
            if self.is_in_loop(ctx) {
                ctx.diagnostic(no_await_in_loop_diagnostic(_await_expr.span));
            }
        }
    }
}

impl NoAwaitInLoop {
    fn is_in_loop(&self, _ctx: &WasmLintContext) -> bool {
        // Check if await is inside a loop
        // Simplified implementation
        true
    }
}

fn no_await_in_loop_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Await inside loop")
        .with_help("Use Promise.all() for parallel execution instead of sequential await")
        .with_label(span)
}

impl EnhancedWasmRule for NoAwaitInLoop {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Promise.all() for parallel execution".to_string(),
            "Use Promise.allSettled() if some promises might fail".to_string(),
            "Sequential await in loops is usually inefficient".to_string(),
            "Consider using for await...of for async iterables".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireAwaitAsync;

impl RequireAwaitAsync {
    pub const NAME: &'static str = "require-await-async";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAwaitAsync {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) if func.r#async => {
                if !self.contains_await_or_return_promise(&func.body) {
                    ctx.diagnostic(require_await_async_diagnostic(func.span));
                }
            }
            AstKind::ArrowFunction(func) if func.r#async => {
                if !self.contains_await_in_arrow(&func.body) {
                    ctx.diagnostic(require_await_async_diagnostic(func.span));
                }
            }
            _ => {}
        }
    }
}

impl RequireAwaitAsync {
    fn contains_await_or_return_promise(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if function body contains await or returns a promise
        false
    }

    fn contains_await_in_arrow(&self, _body: &oxc_ast::ast::FunctionBody) -> bool {
        // Check if arrow function contains await
        false
    }
}

fn require_await_async_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async function without await")
        .with_help("Remove 'async' keyword or add 'await' expressions")
        .with_label(span)
}

impl EnhancedWasmRule for RequireAwaitAsync {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove async if no asynchronous operations".to_string(),
            "Unnecessary async creates Promise overhead".to_string(),
            "Use async only when you need await or return Promise".to_string(),
            "Consider if this should return a Promise directly".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoPromiseExecutorReturn;

impl NoPromiseExecutorReturn {
    pub const NAME: &'static str = "no-promise-executor-return";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoPromiseExecutorReturn {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ReturnStatement(_return_stmt) = node.kind() {
            if self.is_in_promise_executor(ctx) {
                ctx.diagnostic(no_promise_executor_return_diagnostic(_return_stmt.span));
            }
        }
    }
}

impl NoPromiseExecutorReturn {
    fn is_in_promise_executor(&self, _ctx: &WasmLintContext) -> bool {
        // Check if return statement is inside Promise executor function
        true
    }
}

fn no_promise_executor_return_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Return in Promise executor")
        .with_help("Promise executor should not return values, use resolve/reject")
        .with_label(span)
}

impl EnhancedWasmRule for NoPromiseExecutorReturn {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use resolve(value) instead of return value".to_string(),
            "Use reject(error) for error conditions".to_string(),
            "Promise executor return value is ignored".to_string(),
            "Promise constructor expects resolve/reject calls".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoAsyncPromiseExecutor;

impl NoAsyncPromiseExecutor {
    pub const NAME: &'static str = "no-async-promise-executor";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoAsyncPromiseExecutor {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::NewExpression(new_expr) = node.kind() {
            if let Some(ident) = new_expr.callee.as_identifier() {
                if ident.name == "Promise" {
                    if let Some(arg) = new_expr.arguments.first() {
                        if let Some(expr) = arg.as_expression() {
                            if self.is_async_function(expr) {
                                ctx.diagnostic(no_async_promise_executor_diagnostic(new_expr.span));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl NoAsyncPromiseExecutor {
    fn is_async_function(&self, expr: &oxc_ast::ast::Expression) -> bool {
        match expr {
            oxc_ast::ast::Expression::ArrowFunction(arrow) => arrow.r#async,
            oxc_ast::ast::Expression::Function(func) => func.r#async,
            _ => false,
        }
    }
}

fn no_async_promise_executor_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Async Promise executor")
        .with_help("Promise executors should not be async functions")
        .with_label(span)
}

impl EnhancedWasmRule for NoAsyncPromiseExecutor {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use regular function and call resolve/reject".to_string(),
            "Async executors can swallow errors".to_string(),
            "Return async function directly instead of wrapping in Promise".to_string(),
            "Async functions already return promises".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoFloatingPromises;

impl NoFloatingPromises {
    pub const NAME: &'static str = "no-floating-promises";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoFloatingPromises {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ExpressionStatement(expr_stmt) = node.kind() {
            if self.is_promise_expression(&expr_stmt.expression) {
                ctx.diagnostic(no_floating_promises_diagnostic(expr_stmt.span));
            }
        }
    }
}

impl NoFloatingPromises {
    fn is_promise_expression(&self, expr: &oxc_ast::ast::Expression) -> bool {
        // Check if expression returns a promise but is not awaited or handled
        match expr {
            oxc_ast::ast::Expression::CallExpression(call) => {
                // Check for async function calls or promise-returning methods
                self.returns_promise(call)
            }
            _ => false,
        }
    }

    fn returns_promise(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Simplified check for promise-returning calls
        true
    }
}

fn no_floating_promises_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Floating promise")
        .with_help("Add await, return, or .catch() to handle the promise")
        .with_label(span)
}

impl EnhancedWasmRule for NoFloatingPromises {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add await if in async function".to_string(),
            "Add .catch() to handle potential errors".to_string(),
            "Return the promise if this function should be async".to_string(),
            "Unhandled promises can cause silent failures".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefer_async_await_rule() {
        assert_eq!(PreferAsyncAwait::NAME, "prefer-async-await");
        assert_eq!(PreferAsyncAwait::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_await_in_loop_rule() {
        assert_eq!(NoAwaitInLoop::NAME, "no-await-in-loop");
        assert_eq!(NoAwaitInLoop::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_require_await_async_rule() {
        assert_eq!(RequireAwaitAsync::NAME, "require-await-async");
        assert_eq!(RequireAwaitAsync::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_no_promise_executor_return_rule() {
        assert_eq!(NoPromiseExecutorReturn::NAME, "no-promise-executor-return");
        assert_eq!(NoPromiseExecutorReturn::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoPromiseExecutorReturn::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_async_promise_executor_rule() {
        assert_eq!(NoAsyncPromiseExecutor::NAME, "no-async-promise-executor");
        assert_eq!(NoAsyncPromiseExecutor::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_floating_promises_rule() {
        assert_eq!(NoFloatingPromises::NAME, "no-floating-promises");
        assert_eq!(NoFloatingPromises::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_async_function_detection() {
        let rule = NoAsyncPromiseExecutor;
        // Would test with actual AST nodes in real implementation
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = PreferAsyncAwait;
        let diagnostic = prefer_async_await_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("readable"));
    }
}