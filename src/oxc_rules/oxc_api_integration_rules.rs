//! # OXC API Design and Integration Patterns Rules
//!
//! This module implements WASM-safe OXC rules for API design, service integration,
//! microservices communication patterns, and modern API development best practices.
//!
//! ## Rule Categories:
//! - **REST API Design**: RESTful principles, resource modeling, and HTTP semantics
//! - **GraphQL Federation**: Schema stitching, gateway patterns, and distributed GraphQL
//! - **API Security**: Authentication, authorization, rate limiting, and CORS policies
//! - **Service Communication**: Inter-service messaging, circuit breakers, and timeouts
//! - **API Documentation**: OpenAPI/Swagger specifications and API contracts
//! - **Versioning Strategies**: API versioning, backward compatibility, and deprecation
//! - **Integration Patterns**: Enterprise Integration Patterns and message brokers
//! - **Performance Optimization**: Caching, pagination, and response optimization
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
// REST API Design Rules
// ================================================================================================

/// Enforces proper HTTP status code usage
pub struct RequireProperHttpStatus;

impl RequireProperHttpStatus {
    pub const NAME: &'static str = "require-proper-http-status";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireProperHttpStatus {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("POST") || code.contains("PUT") || code.contains("DELETE")) &&
           code.contains("return") && code.contains("200") &&
           !code.contains("201") && !code.contains("204") {
            diagnostics.push(create_http_status_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperHttpStatus {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use appropriate HTTP status codes: 201 for creation, 204 for deletion, 200 for updates with response body".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires proper REST resource naming
pub struct RequireRestfulNaming;

impl RequireRestfulNaming {
    pub const NAME: &'static str = "require-restful-naming";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireRestfulNaming {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("'/api/") &&
           (code.contains("createUser") || code.contains("getUsers") || code.contains("deleteUser")) &&
           !code.contains("/users") {
            diagnostics.push(create_restful_naming_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireRestfulNaming {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use noun-based resource URLs (e.g., /users, /posts) with HTTP verbs for actions instead of verb-based URLs".to_string(),
            confidence: 0.89,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces proper API versioning
pub struct RequireApiVersioning;

impl RequireApiVersioning {
    pub const NAME: &'static str = "require-api-versioning";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireApiVersioning {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("'/api/") && !code.contains("/v1/") && !code.contains("/v2/") &&
           !code.contains("version") && !code.contains("Accept:") {
            diagnostics.push(create_api_versioning_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireApiVersioning {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Include API versioning in URLs (/api/v1/) or headers (Accept: application/vnd.api+json;version=1) for backward compatibility".to_string(),
            confidence: 0.91,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// API Security Rules
// ================================================================================================

/// Requires authentication for API endpoints
pub struct RequireApiAuthentication;

impl RequireApiAuthentication {
    pub const NAME: &'static str = "require-api-authentication";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireApiAuthentication {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("router.") || code.contains("app.")) &&
           (code.contains("POST") || code.contains("PUT") || code.contains("DELETE")) &&
           !code.contains("auth") && !code.contains("token") && !code.contains("middleware") {
            diagnostics.push(create_api_authentication_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireApiAuthentication {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add authentication middleware to protect API endpoints from unauthorized access - consider JWT, OAuth2, or API keys".to_string(),
            confidence: 0.95,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires rate limiting for public APIs
pub struct RequireRateLimiting;

impl RequireRateLimiting {
    pub const NAME: &'static str = "require-rate-limiting";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireRateLimiting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("public") && code.contains("api") &&
           !code.contains("rateLimit") && !code.contains("throttle") {
            diagnostics.push(create_rate_limiting_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireRateLimiting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement rate limiting for public APIs to prevent abuse and ensure fair usage - consider per-user, per-IP, or per-API key limits".to_string(),
            confidence: 0.88,
            auto_fixable: false,
        }).collect()
    }
}

/// Enforces proper CORS configuration
pub struct RequireProperCors;

impl RequireProperCors {
    pub const NAME: &'static str = "require-proper-cors";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireProperCors {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("cors") && code.contains("origin: '*'") &&
           !code.contains("credentials: false") {
            diagnostics.push(create_proper_cors_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperCors {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Avoid wildcard CORS origins in production - specify exact origins and disable credentials for public APIs to prevent security issues".to_string(),
            confidence: 0.96,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// GraphQL Federation Rules
// ================================================================================================

/// Requires proper GraphQL schema composition
pub struct RequireGraphqlComposition;

impl RequireGraphqlComposition {
    pub const NAME: &'static str = "require-graphql-composition";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireGraphqlComposition {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("@apollo/federation") && !code.contains("@key") &&
           code.contains("type") && code.contains("schema") {
            diagnostics.push(create_graphql_composition_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireGraphqlComposition {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use @key directive for entities in GraphQL Federation to enable proper schema composition and entity resolution".to_string(),
            confidence: 0.87,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents N+1 queries in GraphQL resolvers
pub struct NoGraphqlNPlusOne;

impl NoGraphqlNPlusOne {
    pub const NAME: &'static str = "no-graphql-n-plus-one";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoGraphqlNPlusOne {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("resolver") && code.contains("forEach") &&
           code.contains("findById") && !code.contains("dataloader") {
            diagnostics.push(create_graphql_n_plus_one_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoGraphqlNPlusOne {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use DataLoader or batch queries to prevent N+1 query problems in GraphQL resolvers - batch related database requests".to_string(),
            confidence: 0.92,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Service Communication Rules
// ================================================================================================

/// Requires timeout configuration for service calls
pub struct RequireServiceCallTimeouts;

impl RequireServiceCallTimeouts {
    pub const NAME: &'static str = "require-service-call-timeouts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireServiceCallTimeouts {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("axios") || code.contains("fetch")) &&
           code.contains("await") && !code.contains("timeout") &&
           !code.contains("AbortController") {
            diagnostics.push(create_service_timeouts_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireServiceCallTimeouts {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Configure timeouts for service calls to prevent hanging requests and improve system resilience - use AbortController or library timeouts".to_string(),
            confidence: 0.93,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires circuit breaker for external service calls
pub struct RequireCircuitBreakerPattern;

impl RequireCircuitBreakerPattern {
    pub const NAME: &'static str = "require-circuit-breaker-pattern";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCircuitBreakerPattern {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("external") && code.contains("service") &&
           (code.contains("http") || code.contains("api")) &&
           !code.contains("circuit") && !code.contains("breaker") {
            diagnostics.push(create_circuit_breaker_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCircuitBreakerPattern {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement circuit breaker pattern for external service calls to prevent cascading failures and improve system stability".to_string(),
            confidence: 0.86,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// API Documentation and Validation Rules
// ================================================================================================

/// Requires OpenAPI documentation for endpoints
pub struct RequireOpenApiDocs;

impl RequireOpenApiDocs {
    pub const NAME: &'static str = "require-openapi-docs";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireOpenApiDocs {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("router.") || code.contains("app.")) &&
           (code.contains("get") || code.contains("post")) &&
           !code.contains("swagger") && !code.contains("openapi") {
            diagnostics.push(create_openapi_docs_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireOpenApiDocs {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add OpenAPI/Swagger documentation for API endpoints to improve developer experience and enable API testing tools".to_string(),
            confidence: 0.82,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires input validation for API endpoints
pub struct RequireInputValidation;

impl RequireInputValidation {
    pub const NAME: &'static str = "require-input-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireInputValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("req.body") || code.contains("req.params")) &&
           !code.contains("validate") && !code.contains("joi") &&
           !code.contains("yup") && !code.contains("zod") {
            diagnostics.push(create_input_validation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireInputValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Validate all API inputs using schema validation libraries (Joi, Yup, Zod) to prevent injection attacks and data corruption".to_string(),
            confidence: 0.97,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_http_status_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperHttpStatus::NAME.to_string(),
        message: "Use appropriate HTTP status codes for different operations".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use 201 for creation, 204 for deletion, 200 for updates".to_string()),
    }
}

fn create_restful_naming_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireRestfulNaming::NAME.to_string(),
        message: "Use RESTful resource naming conventions".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use noun-based URLs like /users instead of verb-based URLs".to_string()),
    }
}

fn create_api_versioning_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireApiVersioning::NAME.to_string(),
        message: "API endpoints should include versioning information".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add version to URL path or headers".to_string()),
    }
}

fn create_api_authentication_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireApiAuthentication::NAME.to_string(),
        message: "API endpoints should require authentication".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add authentication middleware to protect endpoints".to_string()),
    }
}

fn create_rate_limiting_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireRateLimiting::NAME.to_string(),
        message: "Public APIs should implement rate limiting".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add rate limiting middleware to prevent abuse".to_string()),
    }
}

fn create_proper_cors_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperCors::NAME.to_string(),
        message: "CORS configuration should not use wildcard origins with credentials".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Specify exact origins or disable credentials for wildcard".to_string()),
    }
}

fn create_graphql_composition_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireGraphqlComposition::NAME.to_string(),
        message: "GraphQL Federation entities should use @key directive".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add @key directive for entity composition".to_string()),
    }
}

fn create_graphql_n_plus_one_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoGraphqlNPlusOne::NAME.to_string(),
        message: "Potential N+1 query problem in GraphQL resolver".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use DataLoader to batch database queries".to_string()),
    }
}

fn create_service_timeouts_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireServiceCallTimeouts::NAME.to_string(),
        message: "Service calls should have timeout configuration".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add timeout or AbortController to prevent hanging requests".to_string()),
    }
}

fn create_circuit_breaker_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCircuitBreakerPattern::NAME.to_string(),
        message: "External service calls should use circuit breaker pattern".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement circuit breaker for resilience".to_string()),
    }
}

fn create_openapi_docs_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireOpenApiDocs::NAME.to_string(),
        message: "API endpoints should have OpenAPI documentation".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add Swagger/OpenAPI annotations".to_string()),
    }
}

fn create_input_validation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireInputValidation::NAME.to_string(),
        message: "API inputs should be validated to prevent security issues".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add input validation using schema libraries".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_detection() {
        let code = r#"app.post('/users', (req, res) => { return res.status(200).json(user); });"#;
        let rule = RequireProperHttpStatus;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireProperHttpStatus::NAME);
    }

    #[test]
    fn test_restful_naming_detection() {
        let code = r#"app.get('/api/getUsers', handler);"#;
        let rule = RequireRestfulNaming;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireRestfulNaming::NAME);
    }

    #[test]
    fn test_api_authentication_detection() {
        let code = r#"router.post('/sensitive-data', (req, res) => { /* handler */ });"#;
        let rule = RequireApiAuthentication;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireApiAuthentication::NAME);
    }

    #[test]
    fn test_proper_cors_detection() {
        let code = r#"app.use(cors({ origin: '*', credentials: true }));"#;
        let rule = RequireProperCors;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireProperCors::NAME);
    }

    #[test]
    fn test_service_timeouts_detection() {
        let code = r#"const response = await axios.get('https://api.example.com/data');"#;
        let rule = RequireServiceCallTimeouts;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireServiceCallTimeouts::NAME);
    }

    #[test]
    fn test_input_validation_detection() {
        let code = r#"app.post('/users', (req, res) => { const user = req.body; });"#;
        let rule = RequireInputValidation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireInputValidation::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"app.post('/api/users', (req, res) => res.status(200));"#;
        let rule = RequireProperHttpStatus;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].auto_fixable);
    }
}