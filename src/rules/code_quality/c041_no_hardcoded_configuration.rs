//! # C041: No Hardcoded Configuration Values Rule
//!
//! Detects hardcoded configuration values that should be externalized to environment
//! variables, configuration files, or dependency injection. Promotes flexible and
//! maintainable application configuration management across different environments.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col;
use oxc_ast::ast::{Program, VariableDeclarator, AssignmentExpression, PropertyDefinition, Expression, ObjectProperty};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct C041Config {
    /// Whether to allow hardcoded config in test files (default: true)
    pub allow_hardcoded_in_tests: bool,
    /// Configuration value patterns to detect (default: URLs, ports, timeouts, etc.)
    pub config_patterns: Vec<String>,
    /// Variable names that suggest configuration values
    pub config_variable_names: Vec<String>,
    /// Object property names that suggest configuration
    pub config_property_names: Vec<String>,
    /// Whether to check for hardcoded URLs (default: true)
    pub check_urls: bool,
    /// Whether to check for hardcoded ports (default: true)
    pub check_ports: bool,
    /// Whether to check for hardcoded timeouts (default: true)
    pub check_timeouts: bool,
    /// Whether to check for hardcoded API keys/tokens (default: true)
    pub check_secrets: bool,
    /// Allowed hardcoded values (e.g., default ports, standard timeouts)
    pub allowed_values: Vec<String>,
    /// Maximum string length to consider for configuration detection
    pub max_string_length: u32,
}

/// C041 rule implementation with AI enhancement
pub fn check_no_hardcoded_configuration(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C041Config::default();
    let mut visitor = HardcodedConfigVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.finalize_issues()
}

#[derive(Debug, Clone)]
enum ConfigType {
    Url,
    Port,
    Timeout,
    ApiKey,
    DatabaseConnection,
    EmailConfig,
    CacheConfig,
    RetryConfig,
    SecurityConfig,
    FeatureFlag,
}

impl ConfigType {
    fn description(&self) -> &'static str {
        match self {
            ConfigType::Url => "URL endpoint",
            ConfigType::Port => "network port",
            ConfigType::Timeout => "timeout value",
            ConfigType::ApiKey => "API key or token",
            ConfigType::DatabaseConnection => "database connection string",
            ConfigType::EmailConfig => "email configuration",
            ConfigType::CacheConfig => "cache configuration",
            ConfigType::RetryConfig => "retry configuration",
            ConfigType::SecurityConfig => "security configuration",
            ConfigType::FeatureFlag => "feature flag",
        }
    }

    fn recommendation(&self) -> &'static str {
        match self {
            ConfigType::Url => "Use process.env.API_URL or configuration file",
            ConfigType::Port => "Use process.env.PORT or configuration file",
            ConfigType::Timeout => "Use process.env.TIMEOUT or configuration file",
            ConfigType::ApiKey => "Use process.env.API_KEY with proper secret management",
            ConfigType::DatabaseConnection => "Use process.env.DATABASE_URL with secure storage",
            ConfigType::EmailConfig => "Use environment variables for email settings",
            ConfigType::CacheConfig => "Use configuration file for cache settings",
            ConfigType::RetryConfig => "Use configuration file for retry policies",
            ConfigType::SecurityConfig => "Use environment variables for security settings",
            ConfigType::FeatureFlag => "Use feature flag service or environment variables",
        }
    }
}

struct HardcodedConfigVisitor<'a> {
    config: &'a C041Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
}

impl<'a> HardcodedConfigVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a C041Config) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Replace hardcoded configuration with environment variables".to_string(),
            "Use configuration files for environment-specific settings".to_string(),
            "Implement dependency injection for configuration values".to_string(),
            "Use secure secret management for sensitive configuration".to_string(),
        ]
    }

    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

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

    fn get_config_variable_names(&self) -> HashSet<String> {
        let mut names = HashSet::from([
            "url", "endpoint", "host", "port", "timeout", "retries", "apikey", "token",
            "secret", "password", "database", "db", "redis", "cache", "smtp", "email",
            "config", "settings", "options", "env", "environment", "debug", "enabled",
            "disabled", "threshold", "limit", "max", "min", "interval", "delay"
        ].map(String::from));

        names.extend(self.config.config_variable_names.iter().cloned());
        names
    }

    fn get_config_property_names(&self) -> HashSet<String> {
        let mut names = HashSet::from([
            "baseURL", "apiUrl", "serviceUrl", "endpoint", "host", "hostname", "port",
            "timeout", "retryCount", "retryDelay", "apiKey", "accessToken", "secretKey",
            "connectionString", "databaseUrl", "smtpHost", "smtpPort", "cacheSize",
            "maxRetries", "enabled", "disabled", "threshold", "batchSize", "pageSize"
        ].map(String::from));

        names.extend(self.config.config_property_names.iter().cloned());
        names
    }

    fn get_config_patterns(&self) -> Vec<Regex> {
        let mut patterns = Vec::new();

        // URL patterns
        if let Ok(url_pattern) = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$") {
            patterns.push(url_pattern);
        }

        // Database connection string patterns
        if let Ok(db_pattern) = Regex::new(r"^(mongodb|postgres|mysql|redis)://.*$") {
            patterns.push(db_pattern);
        }

        // Email patterns
        if let Ok(email_pattern) = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
            patterns.push(email_pattern);
        }

        // Add custom patterns
        for pattern_str in &self.config.config_patterns {
            if let Ok(pattern) = Regex::new(pattern_str) {
                patterns.push(pattern);
            }
        }

        patterns
    }

    fn get_allowed_values(&self) -> HashSet<String> {
        let mut values = HashSet::from([
            "localhost", "127.0.0.1", "0.0.0.0", "3000", "8080", "80", "443",
            "5000", "1000", "5000", "10000", "30000", "true", "false", "dev",
            "development", "production", "test", "staging", "utf8", "utf-8"
        ].map(String::from));

        values.extend(self.config.allowed_values.iter().cloned());
        values
    }

    fn is_test_context(&self) -> bool {
        self.config.allow_hardcoded_in_tests && (
            self.source_code.contains(".test.") ||
            self.source_code.contains(".spec.") ||
            self.source_code.contains("__tests__") ||
            self.source_code.contains("describe(") ||
            self.source_code.contains("it(") ||
            self.source_code.contains("test(")
        )
    }

    fn is_configuration_variable(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.get_config_variable_names().iter().any(|config_name| {
            name_lower.contains(config_name) || config_name.contains(&name_lower)
        })
    }

    fn is_configuration_property(&self, name: &str) -> bool {
        let config_props = self.get_config_property_names();
        config_props.contains(name) ||
        config_props.iter().any(|config_prop| {
            name.to_lowercase().contains(&config_prop.to_lowercase())
        })
    }

    fn detect_config_type(&self, value: &str, context: &str) -> Option<ConfigType> {
        let value_lower = value.to_lowercase();
        let context_lower = context.to_lowercase();

        // URL detection
        if self.config.check_urls && self.get_config_patterns().iter().any(|p| p.is_match(value)) {
            if value.starts_with("http") || value.starts_with("ws") || value.starts_with("ftp") {
                return Some(ConfigType::Url);
            }
        }

        // Port detection
        if self.config.check_ports {
            if let Ok(port) = value.parse::<u16>() {
                if port > 1024 && !self.get_allowed_values().contains(value) {
                    return Some(ConfigType::Port);
                }
            }
        }

        // Timeout detection
        if self.config.check_timeouts && (context_lower.contains("timeout") || context_lower.contains("delay")) {
            if let Ok(timeout) = value.parse::<u32>() {
                if timeout > 1000 && !self.get_allowed_values().contains(value) {
                    return Some(ConfigType::Timeout);
                }
            }
        }

        // API key/token detection
        if self.config.check_secrets {
            if (context_lower.contains("key") || context_lower.contains("token") || context_lower.contains("secret")) &&
               value.len() >= 16 && !self.get_allowed_values().contains(value) {
                return Some(ConfigType::ApiKey);
            }
        }

        // Database connection
        if value.contains("://") && (value.contains("mongodb") || value.contains("postgres") || value.contains("mysql")) {
            return Some(ConfigType::DatabaseConnection);
        }

        // Email configuration
        if value.contains("@") && value.contains(".") && self.config_patterns.iter().any(|p| p.is_match(value)) {
            return Some(ConfigType::EmailConfig);
        }

        // Feature flags
        if (value == "true" || value == "false") && context_lower.contains("enable") {
            return Some(ConfigType::FeatureFlag);
        }

        None
    }

    fn create_config_issue(&self, config_type: ConfigType, value: &str, context: &str, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span);

        let base_message = format!(
            "Hardcoded {} '{}' in '{}' should be externalized. {}",
            config_type.description(),
            if value.len() > 30 { &format!("{}...", &value[..30]) } else { value },
            context,
            config_type.recommendation()
        );

        LintIssue {
            rule_name: "C041".to_string(),
            severity: match config_type {
                ConfigType::ApiKey | ConfigType::DatabaseConnection => LintSeverity::Error,
                _ => LintSeverity::Warning,
            },
            message: self.generate_ai_enhanced_message(&base_message, span),
            line,
            column,
            fix_available: true, // AI can suggest environment variable patterns
        }
    }

    fn check_string_value(&mut self, value: &str, context: &str, span: Span) {
        if self.is_test_context() || value.len() > self.config.max_string_length as usize {
            return;
        }

        if self.get_allowed_values().contains(value) {
            return;
        }

        if let Some(config_type) = self.detect_config_type(value, context) {
            let issue = self.create_config_issue(config_type, value, context, span);
            self.issues.push(issue);
        }
    }

    fn check_numeric_value(&mut self, value: f64, context: &str, span: Span) {
        if self.is_test_context() {
            return;
        }

        let value_str = value.to_string();
        if self.get_allowed_values().contains(&value_str) {
            return;
        }

        if let Some(config_type) = self.detect_config_type(&value_str, context) {
            let issue = self.create_config_issue(config_type, &value_str, context, span);
            self.issues.push(issue);
        }
    }
}

impl<'a> Visit<'a> for HardcodedConfigVisitor<'a> {
    fn visit_variable_declarator(&mut self, node: &VariableDeclarator<'a>) {
        if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &node.id.kind {
            let var_name = &ident.name;

            if self.is_configuration_variable(var_name) {
                if let Some(init) = &node.init {
                    match init {
                        Expression::StringLiteral(literal) => {
                            self.check_string_value(&literal.value, var_name, literal.span);
                        }
                        Expression::NumericLiteral(literal) => {
                            self.check_numeric_value(literal.value, var_name, literal.span);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Continue visiting child nodes
        if let Some(init) = &node.init {
            self.visit_expression(init);
        }
    }

    fn visit_assignment_expression(&mut self, node: &AssignmentExpression<'a>) {
        // Check assignments to configuration-like properties
        if let Expression::MemberExpression(member) = &node.left {
            if let Some(property) = &member.property {
                if let Expression::Identifier(prop_ident) = property {
                    let prop_name = &prop_ident.name;

                    if self.is_configuration_property(prop_name) {
                        match &node.right {
                            Expression::StringLiteral(literal) => {
                                self.check_string_value(&literal.value, prop_name, literal.span);
                            }
                            Expression::NumericLiteral(literal) => {
                                self.check_numeric_value(literal.value, prop_name, literal.span);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.left);
        self.visit_expression(&node.right);
    }

    fn visit_property_definition(&mut self, node: &PropertyDefinition<'a>) {
        if let Some(key) = &node.key {
            if let Expression::Identifier(key_ident) = key {
                let prop_name = &key_ident.name;

                if self.is_configuration_property(prop_name) {
                    if let Some(value) = &node.value {
                        match value {
                            Expression::StringLiteral(literal) => {
                                self.check_string_value(&literal.value, prop_name, literal.span);
                            }
                            Expression::NumericLiteral(literal) => {
                                self.check_numeric_value(literal.value, prop_name, literal.span);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Continue visiting child nodes
        if let Some(key) = &node.key {
            self.visit_expression(key);
        }
        if let Some(value) = &node.value {
            self.visit_expression(value);
        }
    }

    fn visit_object_property(&mut self, node: &ObjectProperty<'a>) {
        let prop_name = match &node.key {
            Expression::Identifier(ident) => ident.name.as_str(),
            Expression::StringLiteral(literal) => literal.value.as_str(),
            _ => return,
        };

        if self.is_configuration_property(prop_name) {
            match &node.value {
                Expression::StringLiteral(literal) => {
                    self.check_string_value(&literal.value, prop_name, literal.span);
                }
                Expression::NumericLiteral(literal) => {
                    self.check_numeric_value(literal.value, prop_name, literal.span);
                }
                _ => {}
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.key);
        self.visit_expression(&node.value);
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

        check_no_hardcoded_configuration(&parse_result.program, &semantic_result.semantic, code, None)
    }

    #[test]
    fn test_hardcoded_urls_violation() {
        let code = r#"
const config = {
    apiUrl: "https://api.example.com/v1",
    wsEndpoint: "ws://websocket.example.com:8080",
    serviceUrl: "http://localhost:3000/api"
};

const endpoint = "https://production-api.company.com";
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("URL endpoint")));
    }

    #[test]
    fn test_hardcoded_ports_violation() {
        let code = r#"
const server = {
    port: 8080,
    adminPort: 9090,
    metricsPort: 3001
};

const databasePort = 5432;
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("network port")));
    }

    #[test]
    fn test_hardcoded_secrets_violation() {
        let code = r#"
const auth = {
    apiKey: "sk-1234567890abcdef1234567890abcdef",
    secretKey: "supersecretkey123456789",
    token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
};
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("API key")));
    }

    #[test]
    fn test_hardcoded_database_connection_violation() {
        let code = r#"
const dbConfig = {
    connectionString: "mongodb://user:pass@localhost:27017/mydb",
    url: "postgres://user:password@host:5432/database"
};
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("database connection")));
    }

    #[test]
    fn test_externalized_configuration_compliant() {
        let code = r#"
const config = {
    apiUrl: process.env.API_URL || "http://localhost:3000",
    port: process.env.PORT || 3000,
    timeout: parseInt(process.env.TIMEOUT) || 5000,
    apiKey: process.env.API_KEY,
    dbUrl: process.env.DATABASE_URL
};

const settings = {
    debug: process.env.NODE_ENV === 'development',
    logLevel: process.env.LOG_LEVEL || 'info'
};
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_allowed_default_values_compliant() {
        let code = r#"
const config = {
    host: "localhost",
    port: 3000,
    timeout: 5000,
    enabled: true,
    environment: "development"
};
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // These are allowed default values
    }

    #[test]
    fn test_test_context_allowed() {
        let code = r#"
// config.test.js
describe('Configuration tests', () => {
    it('should handle hardcoded values in tests', () => {
        const testConfig = {
            apiUrl: "https://api.test.example.com",
            port: 9999,
            apiKey: "test-key-12345"
        };
        expect(testConfig).toBeDefined();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Should be allowed in test context
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
const config = {
    apiUrl: "https://api.production.com/v2",
    timeout: 30000
};
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let issues = check_no_hardcoded_configuration(&parse_result.program, &semantic_result.semantic, code);

        assert!(!issues.is_empty());
        assert!(issues[0].fix_available);
        assert!(issues[0].message.contains("process.env"));
    }
}