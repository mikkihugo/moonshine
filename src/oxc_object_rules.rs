//! Object and array pattern rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoDuplicateKeys;

impl NoDuplicateKeys {
    pub const NAME: &'static str = "no-dupe-keys";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDuplicateKeys {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            let mut seen_keys = std::collections::HashSet::new();

            for prop in &obj.properties {
                if let Some(key) = self.get_property_key(prop) {
                    if seen_keys.contains(&key) {
                        ctx.diagnostic(duplicate_key_diagnostic(&key, prop.span()));
                    } else {
                        seen_keys.insert(key);
                    }
                }
            }
        }
    }
}

impl NoDuplicateKeys {
    fn get_property_key(&self, prop: &oxc_ast::ast::ObjectPropertyKind) -> Option<String> {
        use oxc_ast::ast::ObjectPropertyKind;
        match prop {
            ObjectPropertyKind::ObjectProperty(prop) => {
                match &prop.key {
                    oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => Some(ident.name.to_string()),
                    oxc_ast::ast::PropertyKey::StringLiteral(lit) => Some(lit.value.to_string()),
                    oxc_ast::ast::PropertyKey::NumericLiteral(lit) => Some(lit.value.to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn duplicate_key_diagnostic(key: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Duplicate object key")
        .with_help(format!("Key '{}' appears multiple times", key))
        .with_label(span)
}

impl EnhancedWasmRule for NoDuplicateKeys {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Later values will overwrite earlier ones".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferObjectSpread;

impl PreferObjectSpread {
    pub const NAME: &'static str = "prefer-object-spread";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for PreferObjectSpread {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_object_assign_call(call) {
                ctx.diagnostic(prefer_object_spread_diagnostic(call.span));
            }
        }
    }
}

impl PreferObjectSpread {
    fn is_object_assign_call(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                if obj.name == "Object" {
                    if let Some(prop) = member.property().as_identifier() {
                        return prop.name == "assign";
                    }
                }
            }
        }
        false
    }
}

fn prefer_object_spread_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer object spread over Object.assign")
        .with_help("Use {...obj} instead of Object.assign")
        .with_label(span)
}

impl EnhancedWasmRule for PreferObjectSpread {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Object spread is more concise and readable".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSparseArrays;

impl NoSparseArrays {
    pub const NAME: &'static str = "no-sparse-arrays";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoSparseArrays {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ArrayExpression(array) = node.kind() {
            for (i, element) in array.elements.iter().enumerate() {
                if element.is_none() {
                    // Check if this is a trailing comma (last element)
                    if i < array.elements.len() - 1 {
                        ctx.diagnostic(sparse_array_diagnostic(array.span));
                        break;
                    }
                }
            }
        }
    }
}

fn sparse_array_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Sparse array detected")
        .with_help("Avoid holes in arrays, use explicit undefined instead")
        .with_label(span)
}

impl EnhancedWasmRule for NoSparseArrays {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Sparse arrays can cause unexpected behavior".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoPrototypeBuiltins;

impl NoPrototypeBuiltins {
    pub const NAME: &'static str = "no-prototype-builtins";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoPrototypeBuiltins {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if let Some(member) = call.callee.as_member_expression() {
                if let Some(prop) = member.property().as_identifier() {
                    if self.is_prototype_builtin(&prop.name) {
                        ctx.diagnostic(no_prototype_builtin_diagnostic(&prop.name, call.span));
                    }
                }
            }
        }
    }
}

impl NoPrototypeBuiltins {
    fn is_prototype_builtin(&self, method_name: &str) -> bool {
        matches!(method_name, "hasOwnProperty" | "isPrototypeOf" | "propertyIsEnumerable")
    }
}

fn no_prototype_builtin_diagnostic(method: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Direct use of prototype builtin")
        .with_help(format!("Use Object.prototype.{}.call() instead", method))
        .with_label(span)
}

impl EnhancedWasmRule for NoPrototypeBuiltins {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Safer to use Object.prototype methods directly".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferDestructuring;

impl PreferDestructuring {
    pub const NAME: &'static str = "prefer-destructuring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferDestructuring {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclarator(declarator) = node.kind() {
            if let Some(init) = &declarator.init {
                if let Some(member) = init.as_member_expression() {
                    if self.could_use_destructuring(member) {
                        ctx.diagnostic(prefer_destructuring_diagnostic(declarator.span));
                    }
                }
            }
        }
    }
}

impl PreferDestructuring {
    fn could_use_destructuring(&self, member: &oxc_ast::ast::MemberExpression) -> bool {
        // Simple property access that could be destructured
        member.property().as_identifier().is_some()
    }
}

fn prefer_destructuring_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Consider using destructuring")
        .with_help("Use destructuring assignment for cleaner code")
        .with_label(span)
}

impl EnhancedWasmRule for PreferDestructuring {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Destructuring is more expressive and concise".to_string()]
    }
}