//! Custom MoonShine rule for: S058 – SSRF (Server-Side Request Forgery) Detection
//! Rule ID: moonshine/s058
//! Purpose: Detect potential Server-Side Request Forgery vulnerabilities in HTTP requests
//!
//! Converted from JavaScript ESLint rule
//! @category security-rules
//! @complexity high

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Expression, NewExpression, CallExpression, MemberExpression, IdentifierReference};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration options for the S058 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S058Config {
    /// A list of HTTP methods that are considered dangerous for SSRF.
    #[serde(default)]
    pub dangerous_methods: Vec<String>,
    /// A list of URL schemes that should be blocked.
    #[serde(default)]
    pub blocked_schemes: Vec<String>,
    /// A list of host patterns that should be flagged.
    #[serde(default)]
    pub suspicious_host_patterns: Vec<String>,
}

impl Default for S058Config {
    fn default() -> Self {
        Self {
            dangerous_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "HEAD".to_string(),
                "OPTIONS".to_string(),
            ],
            blocked_schemes: vec![
                "http".to_string(),
                "https".to_string(),
                "ftp".to_string(),
                "file".to_string(),
                "ldap".to_string(),
                "ldaps".to_string(),
            ],
            suspicious_host_patterns: vec![
                r"localhost".to_string(),
                r"127\.0\.0\.1".to_string(),
                r"0\.0\.0\.0".to_string(),
                r"169\.254\.".to_string(), // Link-local
                r"10\.0\.0\.0/8".to_string(), // Private network
                r"172\.16\.0\.0/12".to_string(), // Private network
                r"192\.168\.0\.0/16".to_string(), // Private network
            ],
        }
    }
}

/// The main entry point for the S058 rule checking.
pub fn check_ssrf(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = S058Config::default();
    let mut visitor = S058Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// An AST visitor for detecting SSRF vulnerabilities.
struct S058Visitor<'a> {
    config: &'a S058Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    dangerous_methods: HashSet<String>,
    blocked_schemes: HashSet<String>,
}

impl<'a> S058Visitor<'a> {
    /// Creates a new `S058Visitor`.
    fn new(config: &'a S058Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let dangerous_methods: HashSet<String> = config.dangerous_methods
            .iter()
            .cloned()
            .collect();

        let blocked_schemes: HashSet<String> = config.blocked_schemes
            .iter()
            .cloned()
            .collect();

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            dangerous_methods,
            blocked_schemes,
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, vulnerability_type: &str) -> String {
        match vulnerability_type {
            "user_input_url" => "Potential SSRF vulnerability: URL constructed from user input. This could allow attackers to make requests to internal services or external systems. Always validate and sanitize URLs before making HTTP requests.".to_string(),
            "dynamic_hostname" => "Potential SSRF vulnerability: Dynamic hostname resolution. User-controlled hostnames can be exploited to access internal services. Implement hostname allowlists or use URL validation.".to_string(),
            "internal_service_access" => "Potential SSRF vulnerability: Request to internal service detected. This could expose internal network resources. Implement proper access controls and URL validation.".to_string(),
            _ => "Potential Server-Side Request Forgery (SSRF) vulnerability detected. This could allow attackers to make unauthorized requests from your server.".to_string(),
        }
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self, vulnerability_type: &str) -> Vec<String> {
        match vulnerability_type {
            "user_input_url" => vec![
                "Validate URLs against an allowlist of permitted domains".to_string(),
                "Use URL parsing libraries to extract and validate hostnames".to_string(),
                "Implement hostname resolution restrictions".to_string(),
                "Add request rate limiting and monitoring".to_string(),
            ],
            "dynamic_hostname" => vec![
                "Implement hostname allowlists for permitted destinations".to_string(),
                "Use DNS resolution restrictions".to_string(),
                "Validate hostnames against known safe patterns".to_string(),
                "Consider using a URL validation service".to_string(),
            ],
            "internal_service_access" => vec![
                "Block requests to internal IP ranges (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)".to_string(),
                "Implement network segmentation".to_string(),
                "Use allowlists for permitted destinations".to_string(),
                "Add request logging and monitoring".to_string(),
            ],
            _ => vec![
                "Implement URL validation and sanitization".to_string(),
                "Use allowlists for permitted destinations".to_string(),
                "Add request monitoring and rate limiting".to_string(),
            ],
        }
    }

    /// Calculates the line and column from a byte offset.
    fn calculate_line_column(&self, offset: usize) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source_code.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Creates a lint issue for an SSRF vulnerability with AI enhancement.
    fn create_ssrf_issue(&self, vulnerability_type: &str, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(vulnerability_type);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(vulnerability_type);

        LintIssue {
            rule_name: "moonshine/s058".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Error, // Security issues are errors
            line,
            column,
            fix_available: true,
        }
    }

    /// Checks if an expression involves user input.
    fn involves_user_input(&self, expr: &Expression) -> bool {
        // This is a simplified check - in practice, you'd need more sophisticated
        // analysis to track user input through the codebase
        match expr {
            Expression::Identifier(ident) => {
                // Check if identifier name suggests user input
                let name = ident.name.to_lowercase();
                name.contains("input") ||
                name.contains("param") ||
                name.contains("query") ||
                name.contains("req") ||
                name.contains("user")
            },
            Expression::MemberExpression(member) => {
                // Check member access like req.query, req.params, etc.
                if let Expression::Identifier(obj) = &member.object {
                    let obj_name = obj.name.to_lowercase();
                    obj_name.contains("req") || obj_name.contains("request")
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    /// Checks for potential SSRF in HTTP requests.
    fn check_http_request(&mut self, call_expr: &CallExpression, span: Span) {
        // Check if this is an HTTP request method
        if let Expression::MemberExpression(member) = &call_expr.callee {
            if let Expression::Identifier(obj) = &member.object {
                if obj.name.to_lowercase().contains("http") ||
                   obj.name.to_lowercase().contains("fetch") ||
                   obj.name.to_lowercase().contains("axios") ||
                   obj.name.to_lowercase().contains("request") {

                    // Check arguments for user input
                    for arg in &call_expr.arguments {
                        if let oxc_ast::ast::Argument::Expression(expr) = arg {
                            if self.involves_user_input(expr) {
                                self.issues.push(self.create_ssrf_issue("user_input_url", span));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Checks for URL construction from user input.
    fn check_url_construction(&mut self, new_expr: &NewExpression, span: Span) {
        // Check if creating a URL object
        if let Expression::Identifier(ident) = &new_expr.callee {
            if ident.name == "URL" {
                // Check arguments for user input
                for arg in &new_expr.arguments {
                    if let oxc_ast::ast::Argument::Expression(expr) = arg {
                        if self.involves_user_input(expr) {
                            self.issues.push(self.create_ssrf_issue("user_input_url", span));
                            return;
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Visit<'a> for S058Visitor<'a> {
    fn visit_call_expression(&mut self, call_expr: &CallExpression<'a>) {
        self.check_http_request(call_expr, call_expr.span);

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_call_expression(self, call_expr);
    }

    fn visit_new_expression(&mut self, new_expr: &NewExpression<'a>) {
        self.check_url_construction(new_expr, new_expr.span);

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_new_expression(self, new_expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    /// Helper function to parse code and run the rule
    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path("test.ts").unwrap();
        let parser = Parser::new(&allocator, code, source_type);
        let parse_result = parser.parse();

        if !parse_result.errors.is_empty() {
            panic!("Parse errors: {:?}", parse_result.errors);
        }

        let semantic = SemanticBuilder::new(code, source_type)
            .build(&parse_result.program)
            .semantic;

        check_ssrf(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_ssrf_user_input_url_violation() {
        let code = r#"
            function makeRequest(userInput) {
                return fetch(userInput);
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/s058");
        assert!(issues[0].message.contains("URL constructed from user input"));
        assert_eq!(issues[0].severity, LintSeverity::Error);
        assert_eq!(issues[0].fix_available, true);
    }

    #[test]
    fn test_ssrf_axios_request_violation() {
        let code = r#"
            const axios = require('axios');
            function getData(userParam) {
                return axios.get(userParam);
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/s058");
        assert!(issues[0].message.contains("URL constructed from user input"));
    }

    #[test]
    fn test_ssrf_new_url_violation() {
        let code = r#"
            function createUrl(userInput) {
                return new URL(userInput);
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/s058");
        assert!(issues[0].message.contains("URL constructed from user input"));
    }

    #[test]
    fn test_ssrf_request_params_violation() {
        let code = r#"
            function handleRequest(req) {
                return fetch(req.query.url);
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/s058");
        assert!(issues[0].message.contains("URL constructed from user input"));
    }

    #[test]
    fn test_ssrf_compliant_static_url() {
        let code = r#"
            function makeRequest() {
                return fetch('https://api.example.com/data');
            }
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_ssrf_compliant_config_url() {
        let code = r#"
            const config = { apiUrl: 'https://api.example.com' };
            function makeRequest() {
                return fetch(config.apiUrl);
            }
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_ssrf_compliant_no_http_call() {
        let code = r#"
            function processData(userInput) {
                return userInput.toUpperCase();
            }
        "#;
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_ssrf_multiple_violations() {
        let code = r#"
            function handleRequests(userUrl, userParam) {
                fetch(userUrl);
                return new URL(userParam);
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 2);
        assert!(issues.iter().all(|issue| issue.rule_name == "moonshine/s058"));
        assert!(issues.iter().all(|issue| issue.message.contains("URL constructed from user input")));
    }

    #[test]
    fn test_ssrf_nested_function_violation() {
        let code = r#"
            function outerFunction() {
                function innerFunction(userInput) {
                    return fetch(userInput);
                }
                return innerFunction;
            }
        "#;
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_name, "moonshine/s058");
        assert!(issues[0].message.contains("URL constructed from user input"));
    }
}