//! Node.js specific rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoSyncMethods;

impl NoSyncMethods {
    pub const NAME: &'static str = "node-no-sync";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSyncMethods {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if self.is_sync_method(&prop.name) {
                        ctx.diagnostic(no_sync_methods_diagnostic(&prop.name, call.span));
                    }
                }
            }
        }
    }
}

impl NoSyncMethods {
    fn is_sync_method(&self, method_name: &str) -> bool {
        matches!(method_name,
            "readFileSync" | "writeFileSync" | "appendFileSync" | "copyFileSync" |
            "mkdirSync" | "rmdirSync" | "unlinkSync" | "statSync" | "lstatSync" |
            "readdirSync" | "realpathSync" | "accessSync" | "chmodSync" | "chownSync"
        )
    }
}

fn no_sync_methods_diagnostic(method_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Synchronous file system operation")
        .with_help(format!("Use async version: {} blocks the event loop", method_name))
        .with_label(span)
}

impl EnhancedWasmRule for NoSyncMethods {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use async versions with await or promises".to_string(),
            "Sync methods block the entire event loop".to_string(),
            "Consider using fs.promises API for cleaner async code".to_string(),
            "Only use sync methods in CLI tools or initialization code".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoProcessExit;

impl NoProcessExit {
    pub const NAME: &'static str = "node-no-process-exit";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoProcessExit {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(obj) = member.object().as_identifier() {
                    if obj.name == "process" {
                        if let Some(prop) = member.property().as_identifier() {
                            if prop.name == "exit" {
                                ctx.diagnostic(no_process_exit_diagnostic(call.span));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn no_process_exit_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("process.exit() call detected")
        .with_help("Avoid process.exit(), let the process end naturally or throw an error")
        .with_label(span)
}

impl EnhancedWasmRule for NoProcessExit {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Throw an error instead of calling process.exit()".to_string(),
            "Let async operations complete before exit".to_string(),
            "Use proper error handling and return codes".to_string(),
            "process.exit() prevents graceful shutdown".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferGlobalBuffer;

impl PreferGlobalBuffer {
    pub const NAME: &'static str = "node-prefer-global-buffer";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferGlobalBuffer {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(ident) = call.callee.as_identifier() {
                if ident.name == "require" {
                    if let Some(arg) = call.arguments.first() {
                        if let Some(expr) = arg.as_expression() {
                            if let Some(string_lit) = expr.as_string_literal() {
                                if string_lit.value == "buffer" {
                                    ctx.diagnostic(prefer_global_buffer_diagnostic(call.span));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn prefer_global_buffer_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer global Buffer")
        .with_help("Use global Buffer instead of require('buffer')")
        .with_label(span)
}

impl EnhancedWasmRule for PreferGlobalBuffer {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Buffer is available globally in Node.js".to_string(),
            "No need to require('buffer') for Buffer constructor".to_string(),
            "Use Buffer.from() for creating buffers".to_string(),
            "Global Buffer is more performant".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDeprecatedApi;

impl NoDeprecatedApi {
    pub const NAME: &'static str = "node-no-deprecated-api";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDeprecatedApi {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if self.is_deprecated_api(&prop.name) {
                        ctx.diagnostic(no_deprecated_api_diagnostic(&prop.name, call.span));
                    }
                }
            }
        }
    }
}

impl NoDeprecatedApi {
    fn is_deprecated_api(&self, method_name: &str) -> bool {
        matches!(method_name,
            "createCredentials" | "createCipher" | "createDecipher" |
            "exists" | "pummel" | "_linklist" | "createHash"
        )
    }
}

fn no_deprecated_api_diagnostic(api_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Deprecated Node.js API")
        .with_help(format!("API '{}' is deprecated, use modern alternative", api_name))
        .with_label(span)
}

impl EnhancedWasmRule for NoDeprecatedApi {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check Node.js documentation for modern alternatives".to_string(),
            "Deprecated APIs may be removed in future versions".to_string(),
            "Use crypto.createCipheriv instead of createCipher".to_string(),
            "Use fs.access instead of fs.exists".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMixedRequires;

impl NoMixedRequires {
    pub const NAME: &'static str = "node-no-mixed-requires";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoMixedRequires {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclaration(var_decl) = node.kind() {
            let mut has_require = false;
            let mut has_non_require = false;

            for declarator in &var_decl.declarations {
                if let Some(init) = &declarator.init {
                    if self.is_require_call(init) {
                        has_require = true;
                    } else {
                        has_non_require = true;
                    }
                }
            }

            if has_require && has_non_require {
                ctx.diagnostic(no_mixed_requires_diagnostic(var_decl.span));
            }
        }
    }
}

impl NoMixedRequires {
    fn is_require_call(&self, expr: &oxc_ast::ast::Expression) -> bool {
        if let Some(call) = expr.as_call_expression() {
            if let Some(ident) = call.callee.as_identifier() {
                return ident.name == "require";
            }
        }
        false
    }
}

fn no_mixed_requires_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Mixed require and other declarations")
        .with_help("Separate require() calls from other variable declarations")
        .with_label(span)
}

impl EnhancedWasmRule for NoMixedRequires {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Group require statements together at the top".to_string(),
            "Separate module imports from variable declarations".to_string(),
            "Consider using ES6 import statements".to_string(),
            "Organize imports by type: core, external, internal".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferPromises;

impl PreferPromises {
    pub const NAME: &'static str = "node-prefer-promises";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferPromises {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if self.has_callback_version(&prop.name) && !self.is_promises_version(member) {
                        ctx.diagnostic(prefer_promises_diagnostic(&prop.name, call.span));
                    }
                }
            }
        }
    }
}

impl PreferPromises {
    fn has_callback_version(&self, method_name: &str) -> bool {
        matches!(method_name,
            "readFile" | "writeFile" | "appendFile" | "copyFile" |
            "mkdir" | "rmdir" | "unlink" | "stat" | "lstat" |
            "readdir" | "realpath" | "access" | "chmod" | "chown"
        )
    }

    fn is_promises_version(&self, member: &oxc_ast::ast::MemberExpression) -> bool {
        if let Some(obj_member) = member.object().as_member_expression() {
            if let Some(prop) = obj_member.property().as_identifier() {
                return prop.name == "promises";
            }
        }
        false
    }
}

fn prefer_promises_diagnostic(method_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer promises API")
        .with_help(format!("Use fs.promises.{} instead of callback version", method_name))
        .with_label(span)
}

impl EnhancedWasmRule for PreferPromises {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use fs.promises for cleaner async code".to_string(),
            "Promises work better with async/await".to_string(),
            "Avoid callback hell with promise-based APIs".to_string(),
            "fs.promises provides better error handling".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_sync_methods_rule() {
        assert_eq!(NoSyncMethods::NAME, "node-no-sync");
        assert_eq!(NoSyncMethods::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_sync_method_detection() {
        let rule = NoSyncMethods;
        assert!(rule.is_sync_method("readFileSync"));
        assert!(rule.is_sync_method("writeFileSync"));
        assert!(!rule.is_sync_method("readFile"));
    }

    #[test]
    fn test_no_process_exit_rule() {
        assert_eq!(NoProcessExit::NAME, "node-no-process-exit");
        assert_eq!(NoProcessExit::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_prefer_global_buffer_rule() {
        assert_eq!(PreferGlobalBuffer::NAME, "node-prefer-global-buffer");
        assert_eq!(PreferGlobalBuffer::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(PreferGlobalBuffer::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_deprecated_api_rule() {
        assert_eq!(NoDeprecatedApi::NAME, "node-no-deprecated-api");
        assert_eq!(NoDeprecatedApi::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_deprecated_api_detection() {
        let rule = NoDeprecatedApi;
        assert!(rule.is_deprecated_api("createCredentials"));
        assert!(rule.is_deprecated_api("exists"));
        assert!(!rule.is_deprecated_api("createReadStream"));
    }

    #[test]
    fn test_no_mixed_requires_rule() {
        assert_eq!(NoMixedRequires::NAME, "node-no-mixed-requires");
        assert_eq!(NoMixedRequires::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_prefer_promises_rule() {
        assert_eq!(PreferPromises::NAME, "node-prefer-promises");
        assert_eq!(PreferPromises::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoSyncMethods;
        let diagnostic = no_sync_methods_diagnostic("readFileSync", Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("async"));
    }
}