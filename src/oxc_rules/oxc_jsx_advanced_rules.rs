//! Advanced JSX and React rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct JSXNoUselessFragment;

impl JSXNoUselessFragment {
    pub const NAME: &'static str = "jsx-no-useless-fragment";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for JSXNoUselessFragment {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXElement(jsx_element) = node.kind() {
            if self.is_fragment(&jsx_element.opening_element) {
                if self.is_useless_fragment(jsx_element) {
                    ctx.diagnostic(jsx_no_useless_fragment_diagnostic(jsx_element.span));
                }
            }
        }
    }
}

impl JSXNoUselessFragment {
    fn is_fragment(&self, opening: &oxc_ast::ast::JSXOpeningElement) -> bool {
        match &opening.name {
            oxc_ast::ast::JSXElementName::Fragment(_) => true,
            oxc_ast::ast::JSXElementName::Identifier(ident) => ident.name == "Fragment",
            _ => false,
        }
    }

    fn is_useless_fragment(&self, jsx_element: &oxc_ast::ast::JSXElement) -> bool {
        // Fragment is useless if it has only one child and no key prop
        jsx_element.children.len() == 1 && !self.has_key_prop(&jsx_element.opening_element)
    }

    fn has_key_prop(&self, opening: &oxc_ast::ast::JSXOpeningElement) -> bool {
        opening.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return name.name == "key";
                }
            }
            false
        })
    }
}

fn jsx_no_useless_fragment_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Useless React Fragment")
        .with_help("Remove unnecessary Fragment wrapper")
        .with_label(span)
}

impl EnhancedWasmRule for JSXNoUselessFragment {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Fragments are only needed for multiple children".to_string(),
            "Use fragments when you need a key prop".to_string(),
            "Single child elements don't need Fragment wrapper".to_string(),
            "Unnecessary fragments add to the React tree".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSXNoTargetBlank;

impl JSXNoTargetBlank {
    pub const NAME: &'static str = "jsx-no-target-blank";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for JSXNoTargetBlank {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if let Some(name) = self.get_element_name(jsx) {
                if name == "a" && self.has_target_blank(jsx) && !self.has_rel_noopener(jsx) {
                    ctx.diagnostic(jsx_no_target_blank_diagnostic(jsx.span));
                }
            }
        }
    }
}

impl JSXNoTargetBlank {
    fn get_element_name(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> Option<String> {
        match &jsx.name {
            oxc_ast::ast::JSXElementName::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        }
    }

    fn has_target_blank(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    if name.name == "target" {
                        if let Some(value) = &attr.value {
                            return self.is_blank_value(value);
                        }
                    }
                }
            }
            false
        })
    }

    fn is_blank_value(&self, value: &oxc_ast::ast::JSXAttributeValue) -> bool {
        match value {
            oxc_ast::ast::JSXAttributeValue::StringLiteral(lit) => lit.value == "_blank",
            _ => false,
        }
    }

    fn has_rel_noopener(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    if name.name == "rel" {
                        if let Some(value) = &attr.value {
                            return self.contains_noopener(value);
                        }
                    }
                }
            }
            false
        })
    }

    fn contains_noopener(&self, value: &oxc_ast::ast::JSXAttributeValue) -> bool {
        match value {
            oxc_ast::ast::JSXAttributeValue::StringLiteral(lit) => {
                lit.value.contains("noopener") || lit.value.contains("noreferrer")
            }
            _ => false,
        }
    }
}

fn jsx_no_target_blank_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing rel=\"noopener noreferrer\"")
        .with_help("Add rel=\"noopener noreferrer\" to target=\"_blank\" links")
        .with_label(span)
}

impl EnhancedWasmRule for JSXNoTargetBlank {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Prevents reverse tabnabbing security vulnerability".to_string(),
            "noopener prevents access to window.opener".to_string(),
            "noreferrer prevents referrer information leakage".to_string(),
            "Required for security when opening external links".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSXBooleanValue;

impl JSXBooleanValue {
    pub const NAME: &'static str = "jsx-boolean-value";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for JSXBooleanValue {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttribute(attr) = node.kind() {
            if let Some(value) = &attr.value {
                if self.is_unnecessary_true_value(value) {
                    ctx.diagnostic(jsx_boolean_value_diagnostic(attr.span));
                }
            }
        }
    }
}

impl JSXBooleanValue {
    fn is_unnecessary_true_value(&self, value: &oxc_ast::ast::JSXAttributeValue) -> bool {
        match value {
            oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) => {
                matches!(container.expression,
                    oxc_ast::ast::JSXExpression::BooleanLiteral(ref lit) if lit.value
                )
            }
            _ => false,
        }
    }
}

fn jsx_boolean_value_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unnecessary boolean value")
        .with_help("Remove ={true}, boolean props are true by default")
        .with_label(span)
}

impl EnhancedWasmRule for JSXBooleanValue {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Boolean props are true when present".to_string(),
            "Use disabled instead of disabled={true}".to_string(),
            "More concise and follows HTML conventions".to_string(),
            "Only use ={false} when you need to explicitly disable".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSXCurlyBracePresence;

impl JSXCurlyBracePresence {
    pub const NAME: &'static str = "jsx-curly-brace-presence";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for JSXCurlyBracePresence {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttributeValue(value) = node.kind() {
            if let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = value {
                if self.is_unnecessary_expression(&container.expression) {
                    ctx.diagnostic(jsx_curly_brace_presence_diagnostic(container.span));
                }
            }
        }
    }
}

impl JSXCurlyBracePresence {
    fn is_unnecessary_expression(&self, expr: &oxc_ast::ast::JSXExpression) -> bool {
        // Check if expression container wraps a simple string literal
        matches!(expr, oxc_ast::ast::JSXExpression::StringLiteral(_))
    }
}

fn jsx_curly_brace_presence_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unnecessary JSX expression container")
        .with_help("Use string literal instead of expression container for static strings")
        .with_label(span)
}

impl EnhancedWasmRule for JSXCurlyBracePresence {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use prop=\"value\" instead of prop={\"value\"}".to_string(),
            "Expression containers are for dynamic values".to_string(),
            "Static strings don't need curly braces".to_string(),
            "Cleaner syntax improves readability".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSXNoComment;

impl JSXNoComment {
    pub const NAME: &'static str = "jsx-no-comment-textnodes";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for JSXNoComment {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXText(jsx_text) = node.kind() {
            if self.looks_like_comment(&jsx_text.value) {
                ctx.diagnostic(jsx_no_comment_diagnostic(jsx_text.span));
            }
        }
    }
}

impl JSXNoComment {
    fn looks_like_comment(&self, text: &str) -> bool {
        let trimmed = text.trim();
        trimmed.starts_with("//") || (trimmed.starts_with("/*") && trimmed.ends_with("*/"))
    }
}

fn jsx_no_comment_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Comment in JSX text")
        .with_help("Use {/* comment */} for comments in JSX")
        .with_label(span)
}

impl EnhancedWasmRule for JSXNoComment {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use {/* comment */} for JSX comments".to_string(),
            "Text that looks like comments will be rendered".to_string(),
            "JSX comments must be wrapped in expression containers".to_string(),
            "Comments outside JSX elements work normally".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSXNoUndef;

impl JSXNoUndef {
    pub const NAME: &'static str = "jsx-no-undef";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for JSXNoUndef {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if let oxc_ast::ast::JSXElementName::Identifier(ident) = &jsx.name {
                if !self.is_component_defined(&ident.name, ctx) {
                    ctx.diagnostic(jsx_no_undef_diagnostic(&ident.name, jsx.span));
                }
            }
        }
    }
}

impl JSXNoUndef {
    fn is_component_defined(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if component is imported or defined
        // Also check for lowercase (HTML elements)
        if name.chars().next().map_or(false, |c| c.is_lowercase()) {
            return true; // HTML elements are always valid
        }

        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name
        })
    }
}

fn jsx_no_undef_diagnostic(component_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Undefined JSX component")
        .with_help(format!("'{}' is not defined, import or define the component", component_name))
        .with_label(span)
}

impl EnhancedWasmRule for JSXNoUndef {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Import the component from its module".to_string(),
            "Check for typos in component name".to_string(),
            "Ensure component is in scope".to_string(),
            "Use lowercase for HTML elements".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsx_no_useless_fragment_rule() {
        assert_eq!(JSXNoUselessFragment::NAME, "jsx-no-useless-fragment");
        assert_eq!(JSXNoUselessFragment::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(JSXNoUselessFragment::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_jsx_no_target_blank_rule() {
        assert_eq!(JSXNoTargetBlank::NAME, "jsx-no-target-blank");
        assert_eq!(JSXNoTargetBlank::CATEGORY, WasmRuleCategory::Restriction);
        assert_eq!(JSXNoTargetBlank::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_jsx_boolean_value_rule() {
        assert_eq!(JSXBooleanValue::NAME, "jsx-boolean-value");
        assert_eq!(JSXBooleanValue::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(JSXBooleanValue::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_jsx_curly_brace_presence_rule() {
        assert_eq!(JSXCurlyBracePresence::NAME, "jsx-curly-brace-presence");
        assert_eq!(JSXCurlyBracePresence::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(JSXCurlyBracePresence::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_jsx_no_comment_rule() {
        assert_eq!(JSXNoComment::NAME, "jsx-no-comment-textnodes");
        assert_eq!(JSXNoComment::CATEGORY, WasmRuleCategory::Suspicious);
    }

    #[test]
    fn test_jsx_no_undef_rule() {
        assert_eq!(JSXNoUndef::NAME, "jsx-no-undef");
        assert_eq!(JSXNoUndef::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_comment_detection() {
        let rule = JSXNoComment;
        assert!(rule.looks_like_comment("// this is a comment"));
        assert!(rule.looks_like_comment("/* this is a comment */"));
        assert!(!rule.looks_like_comment("regular text"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = JSXNoUselessFragment;
        let diagnostic = jsx_no_useless_fragment_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("multiple children"));
    }
}