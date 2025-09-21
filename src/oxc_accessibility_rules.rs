//! Accessibility (a11y) rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireAltText;

impl RequireAltText {
    pub const NAME: &'static str = "jsx-a11y-alt-text";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireAltText {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if let Some(name) = self.get_element_name(jsx) {
                if self.requires_alt_text(&name) && !self.has_alt_attribute(jsx) {
                    ctx.diagnostic(alt_text_diagnostic(&name, jsx.span));
                }
            }
        }
    }
}

impl RequireAltText {
    fn get_element_name(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> Option<String> {
        match &jsx.name {
            oxc_ast::ast::JSXElementName::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        }
    }

    fn requires_alt_text(&self, element_name: &str) -> bool {
        matches!(element_name, "img" | "area" | "input")
    }

    fn has_alt_attribute(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return name.name == "alt";
                }
            }
            false
        })
    }
}

fn alt_text_diagnostic(element_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing alt text")
        .with_help(format!("Add 'alt' attribute to {} element for accessibility", element_name))
        .with_label(span)
}

impl EnhancedWasmRule for RequireAltText {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Alt text describes images for screen readers".to_string(),
            "Use empty alt=\"\" for decorative images".to_string(),
            "Alt text should be descriptive but concise".to_string(),
            "Required for WCAG compliance".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireAriaLabel;

impl RequireAriaLabel {
    pub const NAME: &'static str = "jsx-a11y-aria-label";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAriaLabel {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if let Some(name) = self.get_element_name(jsx) {
                if self.requires_aria_label(&name) && !self.has_accessible_name(jsx) {
                    ctx.diagnostic(aria_label_diagnostic(&name, jsx.span));
                }
            }
        }
    }
}

impl RequireAriaLabel {
    fn get_element_name(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> Option<String> {
        match &jsx.name {
            oxc_ast::ast::JSXElementName::Identifier(ident) => Some(ident.name.to_string()),
            _ => None,
        }
    }

    fn requires_aria_label(&self, element_name: &str) -> bool {
        matches!(element_name, "button" | "input" | "select" | "textarea")
    }

    fn has_accessible_name(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return matches!(name.name.as_str(),
                        "aria-label" | "aria-labelledby" | "title" | "placeholder"
                    );
                }
            }
            false
        })
    }
}

fn aria_label_diagnostic(element_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing accessible name")
        .with_help(format!("Add aria-label or aria-labelledby to {} element", element_name))
        .with_label(span)
}

impl EnhancedWasmRule for RequireAriaLabel {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use aria-label for invisible labels".to_string(),
            "Use aria-labelledby to reference existing text".to_string(),
            "Accessible names help screen reader users".to_string(),
            "Consider using visible labels when possible".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoAutofocus;

impl NoAutofocus {
    pub const NAME: &'static str = "jsx-a11y-no-autofocus";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoAutofocus {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttribute(attr) = node.kind() {
            if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                if name.name == "autoFocus" || name.name == "autofocus" {
                    ctx.diagnostic(no_autofocus_diagnostic(attr.span));
                }
            }
        }
    }
}

fn no_autofocus_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Autofocus attribute detected")
        .with_help("Avoid autofocus as it can disrupt screen reader navigation")
        .with_label(span)
}

impl EnhancedWasmRule for NoAutofocus {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Autofocus can confuse screen reader users".to_string(),
            "Let users decide where to focus".to_string(),
            "Use programmatic focus management when needed".to_string(),
            "Consider focus management on page load instead".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct ValidAriaProps;

impl ValidAriaProps {
    pub const NAME: &'static str = "jsx-a11y-aria-props";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for ValidAriaProps {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttribute(attr) = node.kind() {
            if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                if name.name.starts_with("aria-") && !self.is_valid_aria_prop(&name.name) {
                    ctx.diagnostic(invalid_aria_prop_diagnostic(&name.name, attr.span));
                }
            }
        }
    }
}

impl ValidAriaProps {
    fn is_valid_aria_prop(&self, prop_name: &str) -> bool {
        // Simplified list of valid ARIA properties
        matches!(prop_name,
            "aria-label" | "aria-labelledby" | "aria-describedby" | "aria-hidden" |
            "aria-expanded" | "aria-checked" | "aria-selected" | "aria-disabled" |
            "aria-required" | "aria-invalid" | "aria-live" | "aria-atomic" |
            "aria-relevant" | "aria-busy" | "aria-dropeffect" | "aria-grabbed" |
            "aria-level" | "aria-multiline" | "aria-multiselectable" | "aria-orientation" |
            "aria-readonly" | "aria-sort" | "aria-valuemax" | "aria-valuemin" |
            "aria-valuenow" | "aria-valuetext" | "aria-controls" | "aria-flowto" |
            "aria-owns" | "aria-posinset" | "aria-setsize" | "aria-activedescendant"
        )
    }
}

fn invalid_aria_prop_diagnostic(prop_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Invalid ARIA property")
        .with_help(format!("'{}' is not a valid ARIA property", prop_name))
        .with_label(span)
}

impl EnhancedWasmRule for ValidAriaProps {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check ARIA specification for valid properties".to_string(),
            "Common typos: aria-labeledby should be aria-labelledby".to_string(),
            "Invalid ARIA properties are ignored by screen readers".to_string(),
            "Use WAI-ARIA authoring practices guide".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClickEventsHaveKeyEvents;

impl ClickEventsHaveKeyEvents {
    pub const NAME: &'static str = "jsx-a11y-click-events-have-key-events";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for ClickEventsHaveKeyEvents {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXOpeningElement(jsx) = node.kind() {
            if self.has_click_handler(jsx) && !self.has_key_handler(jsx) && !self.is_interactive_element(jsx) {
                ctx.diagnostic(click_events_key_events_diagnostic(jsx.span));
            }
        }
    }
}

impl ClickEventsHaveKeyEvents {
    fn has_click_handler(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return name.name == "onClick";
                }
            }
            false
        })
    }

    fn has_key_handler(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        jsx.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(name) = &attr.name {
                    return matches!(name.name.as_str(), "onKeyDown" | "onKeyUp" | "onKeyPress");
                }
            }
            false
        })
    }

    fn is_interactive_element(&self, jsx: &oxc_ast::ast::JSXOpeningElement) -> bool {
        if let oxc_ast::ast::JSXElementName::Identifier(ident) = &jsx.name {
            return matches!(ident.name.as_str(), "button" | "a" | "input" | "select" | "textarea");
        }
        false
    }
}

fn click_events_key_events_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Click handler without keyboard event")
        .with_help("Add onKeyDown or onKeyUp handler for keyboard accessibility")
        .with_label(span)
}

impl EnhancedWasmRule for ClickEventsHaveKeyEvents {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add onKeyDown handler: onKeyDown={(e) => e.key === 'Enter' && handleClick()}".to_string(),
            "Keyboard users need equivalent interaction methods".to_string(),
            "Consider using button element for clickable items".to_string(),
            "Use role=\"button\" with tabIndex=\"0\" for custom buttons".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_alt_text_rule() {
        assert_eq!(RequireAltText::NAME, "jsx-a11y-alt-text");
        assert_eq!(RequireAltText::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(RequireAltText::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_alt_text_requirements() {
        let rule = RequireAltText;
        assert!(rule.requires_alt_text("img"));
        assert!(rule.requires_alt_text("area"));
        assert!(rule.requires_alt_text("input"));
        assert!(!rule.requires_alt_text("div"));
    }

    #[test]
    fn test_require_aria_label_rule() {
        assert_eq!(RequireAriaLabel::NAME, "jsx-a11y-aria-label");
        assert_eq!(RequireAriaLabel::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_autofocus_rule() {
        assert_eq!(NoAutofocus::NAME, "jsx-a11y-no-autofocus");
        assert_eq!(NoAutofocus::CATEGORY, WasmRuleCategory::Suspicious);
        assert_eq!(NoAutofocus::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_valid_aria_props_rule() {
        assert_eq!(ValidAriaProps::NAME, "jsx-a11y-aria-props");
        assert_eq!(ValidAriaProps::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_aria_prop_validation() {
        let rule = ValidAriaProps;
        assert!(rule.is_valid_aria_prop("aria-label"));
        assert!(rule.is_valid_aria_prop("aria-labelledby"));
        assert!(!rule.is_valid_aria_prop("aria-invalid-prop"));
    }

    #[test]
    fn test_click_events_have_key_events_rule() {
        assert_eq!(ClickEventsHaveKeyEvents::NAME, "jsx-a11y-click-events-have-key-events");
        assert_eq!(ClickEventsHaveKeyEvents::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireAltText;
        let diagnostic = alt_text_diagnostic("img", Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("screen readers"));
    }
}