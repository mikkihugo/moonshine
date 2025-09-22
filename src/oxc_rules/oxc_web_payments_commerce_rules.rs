//! # OXC Web Payments and Commerce Rules
//!
//! This module implements WASM-safe OXC rules for web payment systems, e-commerce security,
//! financial transaction processing, and PCI DSS compliance patterns.
//!
//! ## Rule Categories:
//! - **Payment Security**: PCI DSS compliance and secure payment processing
//! - **Transaction Integrity**: Financial transaction validation and error handling
//! - **Sensitive Data Protection**: Credit card, banking, and personal information security
//! - **Payment Gateway Integration**: Secure integration with payment processors
//! - **Web Payments API**: Modern browser payment API best practices
//! - **Financial Calculations**: Precision and accuracy in monetary calculations
//! - **Checkout Process Security**: Secure shopping cart and checkout workflows
//! - **Regulatory Compliance**: GDPR, PCI DSS, and financial regulation compliance
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub rule_name: String,
    pub suggestion: String,
    pub confidence: f32,
    pub auto_fixable: bool,
}

// ================================================================================================
// Payment Security and PCI DSS Compliance Rules
// ================================================================================================

/// Prevents storage of sensitive payment data in client-side code
pub struct NoClientSideCardStorage;

impl NoClientSideCardStorage {
    pub const NAME: &'static str = "no-client-side-card-storage";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoClientSideCardStorage {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let sensitive_patterns = [
            "creditCard", "cardNumber", "cvv", "cvc", "expiry", "securityCode",
            "localStorage", "sessionStorage", "indexedDB"
        ];

        if sensitive_patterns.iter().any(|&pattern| code.contains(pattern)) &&
           code.contains("localStorage") || code.contains("sessionStorage") {
            diagnostics.push(create_client_side_card_storage_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoClientSideCardStorage {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Never store payment card data client-side. Use secure tokenization and server-side processing for PCI DSS compliance".to_string(),
            confidence: 0.98,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires HTTPS for all payment-related operations
pub struct RequireHttpsForPayments;

impl RequireHttpsForPayments {
    pub const NAME: &'static str = "require-https-for-payments";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireHttpsForPayments {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("payment") || code.contains("checkout") || code.contains("billing")) &&
           code.contains("http://") {
            diagnostics.push(create_https_for_payments_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireHttpsForPayments {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "All payment-related communications must use HTTPS to prevent interception of sensitive data".to_string(),
            confidence: 0.99,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Financial Calculation Precision Rules
// ================================================================================================

/// Prevents floating-point arithmetic for monetary calculations
pub struct NoFloatMoneyCalculations;

impl NoFloatMoneyCalculations {
    pub const NAME: &'static str = "no-float-money-calculations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoFloatMoneyCalculations {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let money_keywords = ["price", "amount", "total", "subtotal", "tax", "discount", "payment"];
        let float_operations = [" * ", " / ", " + ", " - "];

        if money_keywords.iter().any(|&keyword| code.contains(keyword)) &&
           float_operations.iter().any(|&op| code.contains(op)) &&
           !code.contains("BigNumber") && !code.contains("Decimal") {
            diagnostics.push(create_float_money_calculations_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoFloatMoneyCalculations {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use decimal libraries (like Decimal.js or BigNumber.js) for monetary calculations to avoid floating-point precision errors".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires proper currency formatting with locale support
pub struct RequireCurrencyFormatting;

impl RequireCurrencyFormatting {
    pub const NAME: &'static str = "require-currency-formatting";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCurrencyFormatting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("price") || code.contains("amount")) &&
           code.contains("toFixed") && !code.contains("Intl.NumberFormat") {
            diagnostics.push(create_currency_formatting_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCurrencyFormatting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use Intl.NumberFormat for proper currency formatting with locale and currency code support".to_string(),
            confidence: 0.89,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Payment Gateway Integration Rules
// ================================================================================================

/// Requires proper error handling for payment operations
pub struct RequirePaymentErrorHandling;

impl RequirePaymentErrorHandling {
    pub const NAME: &'static str = "require-payment-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequirePaymentErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("stripe") || code.contains("paypal") || code.contains("payment")) &&
           code.contains("async") && !code.contains("catch") && !code.contains("try") {
            diagnostics.push(create_payment_error_handling_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePaymentErrorHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement comprehensive error handling for payment operations with user-friendly error messages and retry logic".to_string(),
            confidence: 0.91,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents logging of sensitive payment information
pub struct NoPaymentLogging;

impl NoPaymentLogging {
    pub const NAME: &'static str = "no-payment-logging";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoPaymentLogging {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let sensitive_terms = ["cardNumber", "cvv", "cvc", "expiry", "paymentMethod"];
        let logging_functions = ["console.log", "console.error", "console.warn", "console.info"];

        if sensitive_terms.iter().any(|&term| code.contains(term)) &&
           logging_functions.iter().any(|&log_fn| code.contains(log_fn)) {
            diagnostics.push(create_payment_logging_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoPaymentLogging {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Never log sensitive payment information. Use sanitized logging that excludes payment data".to_string(),
            confidence: 0.97,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Web Payments API Rules
// ================================================================================================

/// Requires proper validation for Payment Request API
pub struct RequirePaymentValidation;

impl RequirePaymentValidation {
    pub const NAME: &'static str = "require-payment-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequirePaymentValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("new PaymentRequest") &&
           !code.contains("canMakePayment") && !code.contains("validate") {
            diagnostics.push(create_payment_validation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePaymentValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Validate payment methods availability using canMakePayment() before showing payment interface".to_string(),
            confidence: 0.88,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces secure shopping cart state management
pub struct RequireSecureCartState;

impl RequireSecureCartState {
    pub const NAME: &'static str = "require-secure-cart-state";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSecureCartState {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("cart") || code.contains("basket")) &&
           code.contains("localStorage") &&
           (code.contains("price") || code.contains("total")) {
            diagnostics.push(create_secure_cart_state_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureCartState {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Store cart items but validate pricing server-side to prevent price manipulation attacks".to_string(),
            confidence: 0.90,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_client_side_card_storage_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoClientSideCardStorage::NAME.to_string(),
        message: "Never store payment card data in client-side storage - PCI DSS violation".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use secure tokenization instead of storing raw card data".to_string()),
    }
}

fn create_https_for_payments_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireHttpsForPayments::NAME.to_string(),
        message: "Payment operations must use HTTPS for security".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Replace http:// with https:// for payment URLs".to_string()),
    }
}

fn create_float_money_calculations_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoFloatMoneyCalculations::NAME.to_string(),
        message: "Use decimal libraries for monetary calculations to avoid precision errors".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use Decimal.js or BigNumber.js for accurate monetary calculations".to_string()),
    }
}

fn create_currency_formatting_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCurrencyFormatting::NAME.to_string(),
        message: "Use Intl.NumberFormat for proper currency formatting".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace toFixed() with Intl.NumberFormat for currency display".to_string()),
    }
}

fn create_payment_error_handling_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePaymentErrorHandling::NAME.to_string(),
        message: "Payment operations must include comprehensive error handling".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add try-catch blocks for payment processing with user-friendly error messages".to_string()),
    }
}

fn create_payment_logging_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoPaymentLogging::NAME.to_string(),
        message: "Never log sensitive payment information".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Remove payment data from logs or use sanitized logging".to_string()),
    }
}

fn create_payment_validation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePaymentValidation::NAME.to_string(),
        message: "Validate payment method availability before showing payment interface".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add canMakePayment() check before creating PaymentRequest".to_string()),
    }
}

fn create_secure_cart_state_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureCartState::NAME.to_string(),
        message: "Cart pricing should be validated server-side to prevent manipulation".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Store cart items but recalculate pricing server-side".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_side_card_storage_detection() {
        let code = r#"localStorage.setItem('creditCard', cardNumber);"#;
        let rule = NoClientSideCardStorage;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoClientSideCardStorage::NAME);
    }

    #[test]
    fn test_https_for_payments_detection() {
        let code = r#"fetch('http://api.example.com/payment', { method: 'POST' });"#;
        let rule = RequireHttpsForPayments;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireHttpsForPayments::NAME);
    }

    #[test]
    fn test_float_money_calculations_detection() {
        let code = r#"const total = price * quantity + tax;"#;
        let rule = NoFloatMoneyCalculations;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoFloatMoneyCalculations::NAME);
    }

    #[test]
    fn test_currency_formatting_detection() {
        let code = r#"const formattedPrice = amount.toFixed(2);"#;
        let rule = RequireCurrencyFormatting;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireCurrencyFormatting::NAME);
    }

    #[test]
    fn test_payment_error_handling_detection() {
        let code = r#"const result = await stripe.createPaymentIntent(data);"#;
        let rule = RequirePaymentErrorHandling;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequirePaymentErrorHandling::NAME);
    }

    #[test]
    fn test_payment_logging_detection() {
        let code = r#"console.log('Payment data:', cardNumber, cvv);"#;
        let rule = NoPaymentLogging;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoPaymentLogging::NAME);
    }

    #[test]
    fn test_payment_validation_detection() {
        let code = r#"const paymentRequest = new PaymentRequest(methods, details);"#;
        let rule = RequirePaymentValidation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequirePaymentValidation::NAME);
    }

    #[test]
    fn test_secure_cart_state_detection() {
        let code = r#"localStorage.setItem('cart', JSON.stringify({ items, total: 99.99 }));"#;
        let rule = RequireSecureCartState;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireSecureCartState::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"localStorage.setItem('creditCard', cardData);"#;
        let rule = NoClientSideCardStorage;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.95);
        assert!(!suggestions[0].auto_fixable); // Security issue requires manual review
    }
}