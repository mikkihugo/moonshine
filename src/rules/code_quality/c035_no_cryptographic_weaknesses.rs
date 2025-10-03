//! # C035: No Cryptographic Weaknesses Rule
//!
//! Detects usage of weak cryptographic algorithms and insecure random number generators
//! that could compromise application security. Promotes secure cryptographic practices
//! and helps prevent security vulnerabilities in production code.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use crate::rules::ai_integration::AIEnhancer;
use oxc_ast::ast::{Program, CallExpression, MemberExpression, Expression, Argument};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration options for the C035 rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct C035Config {
    /// Whether to allow weak algorithms in test files (default: true).
    pub allow_weak_in_tests: bool,
    /// A custom list of additional weak algorithms to detect.
    pub additional_weak_algorithms: Vec<String>,
    /// A custom list of secure algorithms that should be recommended.
    pub recommended_algorithms: Vec<String>,
    /// Whether to check for insecure random number generators (default: true).
    pub check_insecure_random: bool,
    /// Whether to check for hardcoded cryptographic keys/secrets (default: true).
    pub check_hardcoded_secrets: bool,
    /// Minimum key size requirements for various algorithms.
    pub minimum_key_sizes: HashMap<String, u32>,
}

/// The main entry point for the C035 rule checking.
pub fn check_no_cryptographic_weaknesses(program: &Program, _semantic: &Semantic, code: &str, _config: Option<&AIEnhancer>) -> Vec<LintIssue> {
    let config = C035Config::default();
    let mut visitor = CryptographicWeaknessVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.finalize_issues()
}

/// Holds information about a cryptographic algorithm.
#[derive(Debug, Clone)]
struct AlgorithmInfo {
    name: String,
    weakness: WeaknessType,
    recommendation: String,
}

/// The type of cryptographic weakness.
#[derive(Debug, Clone)]
enum WeaknessType {
    DeprecatedAlgorithm,
    WeakKeySize,
    InsecureRandom,
    HardcodedSecret,
    WeakHashing,
    InsecureProtocol,
}

/// An AST visitor for detecting cryptographic weakness violations.
struct CryptographicWeaknessVisitor<'a> {
    config: &'a C035Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
}

impl<'a> CryptographicWeaknessVisitor<'a> {
    /// Creates a new `CryptographicWeaknessVisitor`.
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a C035Config) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Replace weak cryptographic algorithms with secure alternatives".to_string(),
            "Use cryptographically secure random number generators".to_string(),
            "Avoid hardcoded cryptographic keys and secrets".to_string(),
            "Implement proper key management practices".to_string(),
        ]
    }

    /// Calculates the line and column from a byte offset.
    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

    /// Finalizes the issues by applying AI enhancements.
    fn finalize_issues(mut self) -> Vec<LintIssue> {
        // Apply AI enhancements to all issues
        let ai_suggestions = self.generate_ai_fix_suggestions();
        for issue in &mut self.issues {
            if let Some(ai_context) = ai_suggestions.first() {
                issue.message = format!("{} 💡 AI Suggestion: {}", issue.message, ai_context);
            }
        }
        self.issues
    }

    /// Returns a set of insecure function names.
    fn get_insecure_functions(&self) -> HashSet<String> {
        [
            "Math.random", "random", "rand", "srand", "time",
            "getpid", "clock", "Date.now", "performance.now"
        ].iter().map(|s| s.to_string()).collect()
    }

    /// Returns a map of weak algorithms and their information.
    fn get_weak_algorithms(&self) -> HashMap<String, AlgorithmInfo> {
        let mut weak_algorithms = HashMap::new();

        // Deprecated/weak cryptographic algorithms
        weak_algorithms.insert("MD5".to_string(), AlgorithmInfo {
            name: "MD5".to_string(),
            weakness: WeaknessType::WeakHashing,
            recommendation: "Use SHA-256, SHA-3, or bcrypt for secure hashing".to_string(),
        });

        weak_algorithms.insert("SHA1".to_string(), AlgorithmInfo {
            name: "SHA-1".to_string(),
            weakness: WeaknessType::WeakHashing,
            recommendation: "Use SHA-256 or SHA-3 for secure hashing".to_string(),
        });

        weak_algorithms.insert("DES".to_string(), AlgorithmInfo {
            name: "DES".to_string(),
            weakness: WeaknessType::DeprecatedAlgorithm,
            recommendation: "Use AES-256 for symmetric encryption".to_string(),
        });

        weak_algorithms.insert("3DES".to_string(), AlgorithmInfo {
            name: "3DES".to_string(),
            weakness: WeaknessType::DeprecatedAlgorithm,
            recommendation: "Use AES-256 for symmetric encryption".to_string(),
        });

        weak_algorithms.insert("RC4".to_string(), AlgorithmInfo {
            name: "RC4".to_string(),
            weakness: WeaknessType::DeprecatedAlgorithm,
            recommendation: "Use AES with GCM mode for stream encryption".to_string(),
        });

        weak_algorithms.insert("ECB".to_string(), AlgorithmInfo {
            name: "ECB mode".to_string(),
            weakness: WeaknessType::InsecureProtocol,
            recommendation: "Use CBC, GCM, or CTR mode instead of ECB".to_string(),
        });

        // Add additional weak algorithms from config
        for algo in &self.config.additional_weak_algorithms {
            weak_algorithms.insert(algo.clone(), AlgorithmInfo {
                name: algo.clone(),
                weakness: WeaknessType::DeprecatedAlgorithm,
                recommendation: "Use modern cryptographic algorithms".to_string(),
            });
        }

        weak_algorithms
    }

    /// Returns a map of minimum key sizes for various algorithms.
    fn get_minimum_key_sizes(&self) -> HashMap<String, u32> {
        let mut minimum_key_sizes = HashMap::new();
        minimum_key_sizes.insert("RSA".to_string(), 2048);
        minimum_key_sizes.insert("DSA".to_string(), 2048);
        minimum_key_sizes.insert("AES".to_string(), 256);
        minimum_key_sizes.insert("DH".to_string(), 2048);

        // Override with custom key sizes from config
        for (algo, size) in &self.config.minimum_key_sizes {
            minimum_key_sizes.insert(algo.clone(), *size);
        }

        minimum_key_sizes
    }

    /// Returns the name of a function from an expression.
    fn get_function_name(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            Expression::MemberExpression(member) => {
                self.get_member_expression_name(member)
            }
            _ => None,
        }
    }

    /// Returns the name of a member expression.
    fn get_member_expression_name(&self, member: &MemberExpression) -> Option<String> {
        let object_name = match &member.object {
            Expression::Identifier(ident) => ident.name.to_string(),
            _ => return None,
        };

        if let Some(property) = &member.property {
            if let Expression::Identifier(prop_ident) = property {
                return Some(format!("{}.{}", object_name, prop_ident.name));
            }
        }

        None
    }

    /// Checks for weak cryptographic algorithms.
    fn check_weak_algorithm(&mut self, algorithm_name: &str, span: Span) {
        if self.is_test_context() {
            return;
        }

        let algorithm_upper = algorithm_name.to_uppercase();

        if let Some(algo_info) = self.get_weak_algorithms().get(&algorithm_upper) {
            let (line, column) = self.calculate_line_column(span);

            let severity = match algo_info.weakness {
                WeaknessType::DeprecatedAlgorithm | WeaknessType::WeakHashing => LintSeverity::Error,
                WeaknessType::InsecureProtocol | WeaknessType::WeakKeySize => LintSeverity::Warning,
                _ => LintSeverity::Warning,
            };

            self.issues.push(LintIssue {
                rule_name: "C035".to_string(),
                severity,
                message: self.generate_ai_enhanced_message(
                    &format!(
                        "Weak cryptographic algorithm '{}' detected. {}",
                        algo_info.name,
                        algo_info.recommendation
                    ),
                    span,
                ),
                line,
                column,
                fix_available: true, // AI can suggest secure alternatives
            });
        }
    }

    /// Checks for insecure random number generators.
    fn check_insecure_random(&mut self, function_name: &str, span: Span) {
        if !self.config.check_insecure_random || self.is_test_context() {
            return;
        }

        if self.get_insecure_functions().contains(function_name) {
            let (line, column) = self.calculate_line_column(span);

            self.issues.push(LintIssue {
                rule_name: "C035".to_string(),
                severity: LintSeverity::Warning,
                message: format!(
                    "Insecure random number generator '{}' used for cryptographic purposes. Use crypto.getRandomValues() or a cryptographically secure random number generator.",
                    function_name
                ),
                line,
                column,
                fix_available: true, // AI can suggest secure alternatives
            });
        }
    }

    /// Checks for hardcoded secrets.
    fn check_hardcoded_secret(&mut self, value: &str, span: Span) {
        if !self.config.check_hardcoded_secrets || self.is_test_context() {
            return;
        }

        // Check for patterns that might indicate hardcoded secrets
        let suspicious_patterns = [
            (r"^[A-Za-z0-9+/]{40,}={0,2}$", "base64-encoded secret"),
            (r"^[a-fA-F0-9]{32,}$", "hexadecimal secret"),
            (r"^[A-Za-z0-9]{20,}$", "potential API key or token"),
        ];

        for (pattern_str, description) in &suspicious_patterns {
            if let Ok(pattern) = regex::Regex::new(pattern_str) {
                if pattern.is_match(value) && value.len() >= 20 {
                    let (line, column) = self.calculate_line_column(span);

                    self.issues.push(LintIssue {
                        rule_name: "C035".to_string(),
                        severity: LintSeverity::Warning,
                        message: self.generate_ai_enhanced_message(
                            &format!(
                                "Potential hardcoded cryptographic secret detected ({}). Use environment variables or secure key management.",
                                description
                            ),
                            span,
                        ),
                        line,
                        column,
                        fix_available: true, // AI can suggest secure storage methods
                    });
                    break;
                }
            }
        }
    }

    /// Checks for weak key sizes.
    fn check_key_size(&mut self, algorithm: &str, key_size_arg: &Expression, span: Span) {
        if self.is_test_context() {
            return;
        }

        if let Expression::NumericLiteral(literal) = key_size_arg {
            let key_size = literal.value as u32;
            let algorithm_upper = algorithm.to_uppercase();

            if let Some(&minimum_size) = self.get_minimum_key_sizes().get(&algorithm_upper) {
                if key_size < minimum_size {
                    let (line, column) = self.calculate_line_column(span);

                    self.issues.push(LintIssue {
                        rule_name: "C035".to_string(),
                        severity: LintSeverity::Warning,
                        message: self.generate_ai_enhanced_message(
                            &format!(
                                "Weak key size {} for {} algorithm. Minimum recommended size is {} bits.",
                                key_size, algorithm_upper, minimum_size
                            ),
                            span,
                        ),
                        line,
                        column,
                        fix_available: true, // AI can suggest appropriate key sizes
                    });
                }
            }
        }
    }
}

impl<'a> Visit<'a> for CryptographicWeaknessVisitor<'a> {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        if let Some(function_name) = self.get_function_name(&node.callee) {
            // Check for weak cryptographic algorithms
            let algorithm_keywords = ["createHash", "createCipher", "createDecipher", "createSign", "createVerify"];

            for keyword in &algorithm_keywords {
                if function_name.contains(keyword) {
                    // Check the first argument for algorithm name
                    if let Some(first_arg) = node.arguments.first() {
                        if let Argument::StringLiteral(literal) = first_arg {
                            self.check_weak_algorithm(&literal.value, node.span);
                        }
                    }
                }
            }

            // Check for insecure random number generators
            self.check_insecure_random(&function_name, node.span);

            // Check for key generation with weak key sizes
            if function_name.contains("generateKey") || function_name.contains("createKey") {
                if node.arguments.len() >= 2 {
                    if let (Argument::StringLiteral(algo), Some(key_size_arg)) = (&node.arguments[0], node.arguments.get(1)) {
                        if let Argument::Expression(key_expr) = key_size_arg {
                            self.check_key_size(&algo.value, key_expr, node.span);
                        }
                    }
                }
            }
        }

        // Check string literals for hardcoded secrets
        for arg in &node.arguments {
            if let Argument::StringLiteral(literal) = arg {
                if literal.value.len() >= 20 { // Only check longer strings
                    self.check_hardcoded_secret(&literal.value, literal.span);
                }
            }
        }

        // Continue visiting child nodes
        for arg in &node.arguments {
            match arg {
                Argument::Expression(expr) => self.visit_expression(expr),
                Argument::SpreadElement(spread) => self.visit_spread_element(spread),
                _ => {}
            }
        }
        self.visit_expression(&node.callee);
    }

    fn visit_member_expression(&mut self, node: &MemberExpression<'a>) {
        // Check for direct algorithm references
        if let Some(member_name) = self.get_member_expression_name(node) {
            // Check for crypto module usage with weak algorithms
            if member_name.starts_with("crypto.") {
                let algorithm = member_name.strip_prefix("crypto.").unwrap_or("");
                self.check_weak_algorithm(algorithm, node.span);
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.object);
        if let Some(property) = &node.property {
            self.visit_expression(property);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        check_no_cryptographic_weaknesses(&parse_result.program, &semantic_result.semantic, code, None)
    }

    #[test]
    fn test_weak_hashing_algorithms_violation() {
        let code = r#"
import crypto from 'crypto';

function hashPassword(password) {
    return crypto.createHash('md5').update(password).digest('hex');
}

function checksum(data) {
    return crypto.createHash('sha1').update(data).digest('hex');
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("MD5")));
        assert!(issues.iter().any(|issue| issue.message.contains("SHA-1")));
    }

    #[test]
    fn test_weak_encryption_algorithms_violation() {
        let code = r#"
import crypto from 'crypto';

function encryptData(data, key) {
    const cipher = crypto.createCipher('des', key);
    return cipher.update(data, 'utf8', 'hex') + cipher.final('hex');
}

function encryptSensitive(data, key) {
    const cipher = crypto.createCipher('rc4', key);
    return cipher.update(data, 'utf8', 'hex') + cipher.final('hex');
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("DES")));
        assert!(issues.iter().any(|issue| issue.message.contains("RC4")));
    }

    #[test]
    fn test_insecure_random_violation() {
        let code = r#"
function generateToken() {
    return Math.random().toString(36).substring(2);
}

function generateSecret() {
    return Date.now().toString() + Math.random().toString();
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Math.random")));
        assert!(issues.iter().any(|issue| issue.message.contains("Date.now")));
    }

    #[test]
    fn test_weak_key_size_violation() {
        let code = r#"
import crypto from 'crypto';

function generateWeakKey() {
    return crypto.generateKeyPair('rsa', 1024); // Too small
}

function generateWeakAESKey() {
    return crypto.generateKey('aes', 128); // Too small
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Weak key size")));
    }

    #[test]
    fn test_secure_cryptography_compliant() {
        let code = r#"
import crypto from 'crypto';

function hashPassword(password, salt) {
    return crypto.scrypt(password, salt, 64);
}

function encryptData(data, key) {
    const cipher = crypto.createCipher('aes-256-gcm', key);
    return cipher.update(data, 'utf8', 'hex') + cipher.final('hex');
}

function generateSecureRandom() {
    return crypto.getRandomValues(new Uint8Array(32));
}

function generateStrongKey() {
    return crypto.generateKeyPair('rsa', 2048);
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_test_context_allowed() {
        let code = r#"
// test.spec.js
import crypto from 'crypto';

describe('Crypto tests', () => {
    it('should allow weak crypto in tests', () => {
        const hash = crypto.createHash('md5').update('test').digest('hex');
        expect(hash).toBeDefined();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Should be allowed in test context
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
import crypto from 'crypto';

function unsafeHash(data) {
    return crypto.createHash('md5').update(data).digest('hex');
}
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest secure cryptographic alternatives".to_string());

        let issues = check_no_cryptographic_weaknesses(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        assert!(!issues.is_empty());
        assert!(issues[0].fix_available);
        assert!(issues[0].message.contains("MD5"));
    }
}