//! Security and safety rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoEval;

impl NoEval {
    pub const NAME: &'static str = "no-eval";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoEval {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(ident) = call.callee.as_identifier() {
                if ident.name == "eval" {
                    ctx.diagnostic(no_eval_diagnostic(call.span));
                }
            }
        }
    }
}

fn no_eval_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Use of eval() function")
        .with_help("eval() can execute arbitrary code and poses security risks")
        .with_label(span)
}

impl EnhancedWasmRule for NoEval {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use JSON.parse() for parsing JSON data".to_string(),
            "Use Function constructor for dynamic functions".to_string(),
            "Consider template literals for string interpolation".to_string(),
            "eval() can lead to code injection vulnerabilities".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoImpliedEval;

impl NoImpliedEval {
    pub const NAME: &'static str = "no-implied-eval";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoImpliedEval {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(ident) = call.callee.as_identifier() {
                if self.is_implied_eval_function(&ident.name) {
                    if self.has_string_argument(call) {
                        ctx.diagnostic(implied_eval_diagnostic(&ident.name, call.span));
                    }
                }
            }
        }
    }
}

impl NoImpliedEval {
    fn is_implied_eval_function(&self, name: &str) -> bool {
        matches!(name, "setTimeout" | "setInterval" | "setImmediate")
    }

    fn has_string_argument(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        call.arguments.first()
            .and_then(|arg| arg.as_expression())
            .map(|expr| expr.is_string_literal())
            .unwrap_or(false)
    }
}

fn implied_eval_diagnostic(func_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Implied eval usage")
        .with_help(format!("Pass function reference to {} instead of string", func_name))
        .with_label(span)
}

impl EnhancedWasmRule for NoImpliedEval {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use function references: setTimeout(() => {...}, 1000)".to_string(),
            "String arguments to timer functions are evaluated like eval()".to_string(),
            "Function references provide better performance and security".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnsafeRegex;

impl NoUnsafeRegex {
    pub const NAME: &'static str = "no-unsafe-regex";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoUnsafeRegex {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::RegExpLiteral(regex) = node.kind() {
            if self.is_potentially_unsafe(&regex.regex.pattern) {
                ctx.diagnostic(unsafe_regex_diagnostic(regex.span));
            }
        }
    }
}

impl NoUnsafeRegex {
    fn is_potentially_unsafe(&self, pattern: &str) -> bool {
        // Check for common ReDoS patterns (simplified)
        pattern.contains("(.*)*") ||
        pattern.contains("(.+)+") ||
        pattern.contains("([^x]*)*") ||
        pattern.contains("(x|x)*")
    }
}

fn unsafe_regex_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potentially unsafe regex")
        .with_help("This regex pattern may cause ReDoS (Regular Expression Denial of Service)")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnsafeRegex {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Avoid nested quantifiers in regex patterns".to_string(),
            "Use atomic groups or possessive quantifiers if available".to_string(),
            "Test regex with long input strings to check performance".to_string(),
            "Consider using string methods for simple text operations".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoScriptUrl;

impl NoScriptUrl {
    pub const NAME: &'static str = "no-script-url";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoScriptUrl {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if string_lit.value.starts_with("javascript:") {
                ctx.diagnostic(script_url_diagnostic(string_lit.span));
            }
        }
    }
}

fn script_url_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("JavaScript URL detected")
        .with_help("Avoid 'javascript:' URLs as they can lead to XSS vulnerabilities")
        .with_label(span)
}

impl EnhancedWasmRule for NoScriptUrl {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use event handlers instead of javascript: URLs".to_string(),
            "javascript: URLs can bypass CSP protections".to_string(),
            "Use proper href attributes with event listeners".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoInnerHtml;

impl NoInnerHtml {
    pub const NAME: &'static str = "no-inner-html";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoInnerHtml {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::AssignmentExpression(assign) = node.kind() {
            if let Some(member) = assign.left.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if prop.name == "innerHTML" {
                        ctx.diagnostic(inner_html_diagnostic(assign.span));
                    }
                }
            }
        }
    }
}

fn inner_html_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct innerHTML assignment")
        .with_help("Use textContent or sanitize HTML to prevent XSS attacks")
        .with_label(span)
}

impl EnhancedWasmRule for NoInnerHtml {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use textContent for plain text content".to_string(),
            "Sanitize HTML with DOMPurify before setting innerHTML".to_string(),
            "Consider using createElement and appendChild".to_string(),
            "innerHTML with user input can lead to XSS vulnerabilities".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoConsoleLog;

impl NoConsoleLog {
    pub const NAME: &'static str = "no-console";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoConsoleLog {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(obj) = member.object().as_identifier() {
                    if obj.name == "console" {
                        ctx.diagnostic(console_log_diagnostic(call.span));
                    }
                }
            }
        }
    }
}

fn console_log_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Console statement detected")
        .with_help("Remove console statements before production deployment")
        .with_label(span)
}

impl EnhancedWasmRule for NoConsoleLog {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use proper logging libraries for production".to_string(),
            "Consider environment-based logging configuration".to_string(),
            "Console statements can expose sensitive information".to_string(),
            "Use debugging tools instead of console.log".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_eval_rule() {
        assert_eq!(NoEval::NAME, "no-eval");
        assert_eq!(NoEval::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_no_implied_eval_rule() {
        assert_eq!(NoImpliedEval::NAME, "no-implied-eval");
        assert_eq!(NoImpliedEval::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_implied_eval_detection() {
        let rule = NoImpliedEval;
        assert!(rule.is_implied_eval_function("setTimeout"));
        assert!(rule.is_implied_eval_function("setInterval"));
        assert!(!rule.is_implied_eval_function("requestAnimationFrame"));
    }

    #[test]
    fn test_unsafe_regex_detection() {
        let rule = NoUnsafeRegex;
        assert!(rule.is_potentially_unsafe("(.*)*"));
        assert!(rule.is_potentially_unsafe("(.+)+"));
        assert!(!rule.is_potentially_unsafe("^[a-z]+$"));
    }

    #[test]
    fn test_no_script_url_rule() {
        assert_eq!(NoScriptUrl::NAME, "no-script-url");
        assert_eq!(NoScriptUrl::CATEGORY, WasmRuleCategory::Restriction);
        assert_eq!(NoScriptUrl::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_inner_html_rule() {
        assert_eq!(NoInnerHtml::NAME, "no-inner-html");
        assert_eq!(NoInnerHtml::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_no_console_rule() {
        assert_eq!(NoConsoleLog::NAME, "no-console");
        assert_eq!(NoConsoleLog::CATEGORY, WasmRuleCategory::Restriction);
        assert_eq!(NoConsoleLog::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoEval;
        let diagnostic = no_eval_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("JSON.parse"));
    }
}