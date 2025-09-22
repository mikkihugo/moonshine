//! Advanced Security Rules
//!
//! Comprehensive security rules for cryptography, secure coding practices, and security compliance.
//! Focuses on modern security threats, encryption standards, and defensive programming patterns.

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
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}

/// Require strong cryptographic algorithms and avoid weak ones
pub struct RequireStrongCryptography;

impl RequireStrongCryptography {
    pub const NAME: &'static str = "require-strong-cryptography";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for RequireStrongCryptography {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for weak hash algorithms
        if code.contains("md5") || code.contains("sha1") {
            diagnostics.push(create_weak_hash_diagnostic());
        }

        // Check for weak encryption
        if code.contains("DES") || code.contains("RC4") {
            diagnostics.push(create_weak_encryption_diagnostic());
        }

        // Check for insufficient key sizes
        if code.contains("RSA") && code.contains("1024") {
            diagnostics.push(create_weak_key_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireStrongCryptography {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Use strong cryptographic algorithms: SHA-256+ for hashing, AES-256 for encryption, RSA-2048+ or ECC for key exchange. Avoid deprecated algorithms like MD5, SHA-1, DES.".to_string(),
            confidence: 0.95,
        }).collect()
    }
}

fn create_weak_hash_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireStrongCryptography::NAME.to_string(),
        message: "Weak hash algorithm detected. Use SHA-256 or stronger".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_weak_encryption_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireStrongCryptography::NAME.to_string(),
        message: "Weak encryption algorithm detected. Use AES-256 or stronger".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_weak_key_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireStrongCryptography::NAME.to_string(),
        message: "Insufficient key size detected. Use RSA-2048+ or ECC-256+".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require secure random number generation
pub struct RequireSecureRandom;

impl RequireSecureRandom {
    pub const NAME: &'static str = "require-secure-random";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSecureRandom {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for insecure random usage
        if code.contains("Math.random()") && (code.contains("token") || code.contains("password") || code.contains("session")) {
            diagnostics.push(create_insecure_random_diagnostic());
        }

        // Check for proper crypto random usage
        if code.contains("crypto.getRandomValues") {
            // This is good, no diagnostic needed
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureRandom {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Use cryptographically secure random number generators: crypto.getRandomValues() in browsers, crypto.randomBytes() in Node.js. Avoid Math.random() for security-sensitive operations.".to_string(),
            confidence: 0.93,
        }).collect()
    }
}

fn create_insecure_random_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureRandom::NAME.to_string(),
        message: "Math.random() is not cryptographically secure. Use crypto.getRandomValues()".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require input sanitization and validation
pub struct RequireInputSanitization;

impl RequireInputSanitization {
    pub const NAME: &'static str = "require-input-sanitization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireInputSanitization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct innerHTML usage without sanitization
        if code.contains("innerHTML") && !code.contains("sanitize") {
            diagnostics.push(create_innerHTML_diagnostic());
        }

        // Check for SQL query construction without validation
        if code.contains("SELECT") && code.contains("${") {
            diagnostics.push(create_sql_injection_diagnostic());
        }

        // Check for eval usage
        if code.contains("eval(") {
            diagnostics.push(create_eval_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireInputSanitization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Always sanitize and validate user input. Use parameterized queries for SQL, sanitize HTML content, validate data types and ranges. Implement defense in depth.".to_string(),
            confidence: 0.91,
        }).collect()
    }
}

fn create_innerHTML_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireInputSanitization::NAME.to_string(),
        message: "innerHTML usage without sanitization detected. Risk of XSS vulnerability".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_sql_injection_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireInputSanitization::NAME.to_string(),
        message: "SQL query with string interpolation detected. Use parameterized queries".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_eval_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireInputSanitization::NAME.to_string(),
        message: "eval() usage detected. This is dangerous and should be avoided".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require secure authentication patterns
pub struct RequireSecureAuthentication;

impl RequireSecureAuthentication {
    pub const NAME: &'static str = "require-secure-authentication";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSecureAuthentication {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for plaintext password storage
        if code.contains("password") && code.contains("localStorage") {
            diagnostics.push(create_password_storage_diagnostic());
        }

        // Check for missing JWT verification
        if code.contains("jwt") && !code.contains("verify") {
            diagnostics.push(create_jwt_verification_diagnostic());
        }

        // Check for weak session management
        if code.contains("session") && !code.contains("secure") {
            diagnostics.push(create_session_security_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureAuthentication {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Implement secure authentication: hash passwords with bcrypt/scrypt, use secure JWT verification, implement proper session management with httpOnly and secure flags.".to_string(),
            confidence: 0.90,
        }).collect()
    }
}

fn create_password_storage_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureAuthentication::NAME.to_string(),
        message: "Password stored in localStorage detected. Use secure server-side storage".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_jwt_verification_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureAuthentication::NAME.to_string(),
        message: "JWT usage without verification detected. Always verify JWT signatures".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_session_security_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureAuthentication::NAME.to_string(),
        message: "Session configuration should include secure and httpOnly flags".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require secure HTTP headers
pub struct RequireSecurityHeaders;

impl RequireSecurityHeaders {
    pub const NAME: &'static str = "require-security-headers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSecurityHeaders {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing CSP header
        if code.contains("express") && !code.contains("Content-Security-Policy") {
            diagnostics.push(create_csp_diagnostic());
        }

        // Check for missing HSTS header
        if code.contains("https") && !code.contains("Strict-Transport-Security") {
            diagnostics.push(create_hsts_diagnostic());
        }

        // Check for missing X-Frame-Options
        if code.contains("app.use") && !code.contains("X-Frame-Options") {
            diagnostics.push(create_frame_options_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecurityHeaders {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Implement security headers: Content-Security-Policy, Strict-Transport-Security, X-Frame-Options, X-Content-Type-Options. Use helmet.js for Express applications.".to_string(),
            confidence: 0.88,
        }).collect()
    }
}

fn create_csp_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecurityHeaders::NAME.to_string(),
        message: "Missing Content-Security-Policy header. Implement CSP to prevent XSS".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_hsts_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecurityHeaders::NAME.to_string(),
        message: "Missing Strict-Transport-Security header for HTTPS enforcement".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_frame_options_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecurityHeaders::NAME.to_string(),
        message: "Missing X-Frame-Options header. Prevent clickjacking attacks".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require secure data transmission
pub struct RequireSecureTransmission;

impl RequireSecureTransmission {
    pub const NAME: &'static str = "require-secure-transmission";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSecureTransmission {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for HTTP URLs in production
        if code.contains("http://") && !code.contains("localhost") {
            diagnostics.push(create_http_url_diagnostic());
        }

        // Check for unencrypted API calls
        if code.contains("api.") && code.contains("http://") {
            diagnostics.push(create_unencrypted_api_diagnostic());
        }

        // Check for sensitive data in URLs
        if code.contains("password") && code.contains("GET") {
            diagnostics.push(create_sensitive_url_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureTransmission {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Use HTTPS for all data transmission. Encrypt sensitive data at rest and in transit. Avoid sending sensitive information in URLs or query parameters.".to_string(),
            confidence: 0.94,
        }).collect()
    }
}

fn create_http_url_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureTransmission::NAME.to_string(),
        message: "HTTP URL detected. Use HTTPS for secure data transmission".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_unencrypted_api_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureTransmission::NAME.to_string(),
        message: "Unencrypted API call detected. Use HTTPS for API communications".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_sensitive_url_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureTransmission::NAME.to_string(),
        message: "Sensitive data in URL detected. Use POST body or secure headers".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require secure error handling
pub struct RequireSecureErrorHandling;

impl RequireSecureErrorHandling {
    pub const NAME: &'static str = "require-secure-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSecureErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for error messages exposing sensitive info
        if code.contains("throw new Error") && (code.contains("password") || code.contains("token")) {
            diagnostics.push(create_sensitive_error_diagnostic());
        }

        // Check for stack traces in production
        if code.contains("error.stack") && code.contains("production") {
            diagnostics.push(create_stack_trace_diagnostic());
        }

        // Check for detailed database errors
        if code.contains("catch") && code.contains("sql") {
            diagnostics.push(create_database_error_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureErrorHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Implement secure error handling: sanitize error messages, log detailed errors server-side only, return generic messages to users. Avoid exposing system internals.".to_string(),
            confidence: 0.89,
        }).collect()
    }
}

fn create_sensitive_error_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureErrorHandling::NAME.to_string(),
        message: "Error message may expose sensitive information. Use generic messages".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_stack_trace_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureErrorHandling::NAME.to_string(),
        message: "Stack trace exposure in production detected. Log server-side only".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_database_error_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureErrorHandling::NAME.to_string(),
        message: "Database error handling should not expose schema information".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require secure dependency management
pub struct RequireSecureDependencies;

impl RequireSecureDependencies {
    pub const NAME: &'static str = "require-secure-dependencies";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSecureDependencies {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for outdated security libraries
        if code.contains("crypto-js") && code.contains("3.1") {
            diagnostics.push(create_outdated_crypto_diagnostic());
        }

        // Check for known vulnerable packages
        if code.contains("lodash") && code.contains("4.17.15") {
            diagnostics.push(create_vulnerable_package_diagnostic());
        }

        // Check for missing integrity checks
        if code.contains("script src") && !code.contains("integrity") {
            diagnostics.push(create_missing_integrity_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureDependencies {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Keep dependencies updated, audit for vulnerabilities regularly, use integrity checks for CDN resources, implement dependency scanning in CI/CD pipeline.".to_string(),
            confidence: 0.86,
        }).collect()
    }
}

fn create_outdated_crypto_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureDependencies::NAME.to_string(),
        message: "Outdated cryptographic library detected. Update to latest version".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_vulnerable_package_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureDependencies::NAME.to_string(),
        message: "Known vulnerable package version detected. Update immediately".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_missing_integrity_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureDependencies::NAME.to_string(),
        message: "External script without integrity check. Add SRI hash".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require secure configuration management
pub struct RequireSecureConfiguration;

impl RequireSecureConfiguration {
    pub const NAME: &'static str = "require-secure-configuration";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSecureConfiguration {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for hardcoded secrets
        if code.contains("api_key") && code.contains("=") && !code.contains("process.env") {
            diagnostics.push(create_hardcoded_secret_diagnostic());
        }

        // Check for debug mode in production
        if code.contains("debug: true") && code.contains("production") {
            diagnostics.push(create_debug_mode_diagnostic());
        }

        // Check for default credentials
        if code.contains("admin") && code.contains("password") {
            diagnostics.push(create_default_credentials_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureConfiguration {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            message: "Use environment variables for secrets, implement proper configuration management, rotate credentials regularly, disable debug modes in production.".to_string(),
            confidence: 0.92,
        }).collect()
    }
}

fn create_hardcoded_secret_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureConfiguration::NAME.to_string(),
        message: "Hardcoded secret detected. Use environment variables or secure vault".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_debug_mode_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureConfiguration::NAME.to_string(),
        message: "Debug mode enabled in production. Disable for security".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_default_credentials_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureConfiguration::NAME.to_string(),
        message: "Default credentials detected. Change before deployment".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_strong_cryptography() {
        let rule = RequireStrongCryptography;

        // Test weak hash algorithm
        let code_violation = "const hash = crypto.createHash('md5');";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test strong hash algorithm
        let code_compliant = "const hash = crypto.createHash('sha256');";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_random() {
        let rule = RequireSecureRandom;

        // Test insecure random for tokens
        let code_violation = "const token = Math.random().toString();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test secure random
        let code_compliant = "const token = crypto.getRandomValues(new Uint8Array(32));";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_input_sanitization() {
        let rule = RequireInputSanitization;

        // Test innerHTML without sanitization
        let code_violation = "element.innerHTML = userInput;";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper sanitization
        let code_compliant = "element.innerHTML = sanitize(userInput);";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_authentication() {
        let rule = RequireSecureAuthentication;

        // Test password in localStorage
        let code_violation = "localStorage.setItem('password', userPassword);";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper password handling
        let code_compliant = "const hashedPassword = bcrypt.hash(userPassword);";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_security_headers() {
        let rule = RequireSecurityHeaders;

        // Test Express without security headers
        let code_violation = "const app = express();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper security headers
        let code_compliant = "app.use(helmet()); // includes Content-Security-Policy";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_transmission() {
        let rule = RequireSecureTransmission;

        // Test HTTP URL
        let code_violation = "fetch('http://api.example.com/data');";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test HTTPS URL
        let code_compliant = "fetch('https://api.example.com/data');";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_error_handling() {
        let rule = RequireSecureErrorHandling;

        // Test error with sensitive info
        let code_violation = "throw new Error('Password validation failed for: ' + password);";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test generic error message
        let code_compliant = "throw new Error('Authentication failed');";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_dependencies() {
        let rule = RequireSecureDependencies;

        // Test vulnerable package
        let code_violation = "\"lodash\": \"4.17.15\"";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test updated package
        let code_compliant = "\"lodash\": \"4.17.21\"";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_secure_configuration() {
        let rule = RequireSecureConfiguration;

        // Test hardcoded API key
        let code_violation = "const api_key = 'sk-1234567890abcdef';";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test environment variable
        let code_compliant = "const api_key = process.env.API_KEY;";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement() {
        let rule = RequireStrongCryptography;
        let diagnostics = vec![WasmRuleDiagnostic {
            rule_name: "require-strong-cryptography".to_string(),
            message: "Test message".to_string(),
            line: 1,
            column: 1,
            severity: "error".to_string(),
        }];

        let suggestions = rule.ai_enhance("", &diagnostics);
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
    }
}