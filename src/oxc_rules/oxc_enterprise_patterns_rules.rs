//! Enterprise Patterns Rules
//!
//! Domain-Driven Design, CQRS, Event Sourcing, and enterprise architecture pattern rules.
//! Focuses on microservices, distributed systems, and enterprise-grade application patterns.

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

/// Require Domain-Driven Design aggregate boundaries
pub struct RequireAggregateBoundaries;

impl RequireAggregateBoundaries {
    pub const NAME: &'static str = "require-aggregate-boundaries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAggregateBoundaries {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for aggregate crossing without proper interface
        if code.contains("aggregate") && code.contains("direct access") {
            diagnostics.push(create_aggregate_boundary_diagnostic());
        }

        // Check for missing aggregate root
        if code.contains("entity") && !code.contains("AggregateRoot") {
            diagnostics.push(create_aggregate_root_diagnostic());
        }

        // Check for repository pattern violations
        if code.contains("repository") && code.contains("business logic") {
            diagnostics.push(create_repository_pattern_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAggregateBoundaries {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "aggregate_boundaries".to_string(),
            confidence: 0.89,
            description: "Implement proper aggregate boundaries: define clear aggregate roots, enforce invariants within aggregates, use domain services for cross-aggregate operations.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_aggregate_boundary_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAggregateBoundaries::NAME.to_string(),
        message: "Aggregate boundaries should be respected. Use domain services for cross-aggregate operations".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_aggregate_root_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAggregateBoundaries::NAME.to_string(),
        message: "Entities should be accessed through aggregate roots".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_repository_pattern_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAggregateBoundaries::NAME.to_string(),
        message: "Repository should not contain business logic. Keep it focused on data access".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require CQRS command and query separation
pub struct RequireCqrsSeparation;

impl RequireCqrsSeparation {
    pub const NAME: &'static str = "require-cqrs-separation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireCqrsSeparation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for commands that return data
        if code.contains("Command") && code.contains("return") && code.contains("data") {
            diagnostics.push(create_command_return_diagnostic());
        }

        // Check for queries that modify state
        if code.contains("Query") && (code.contains("update") || code.contains("delete") || code.contains("insert")) {
            diagnostics.push(create_query_modification_diagnostic());
        }

        // Check for missing command handlers
        if code.contains("Command") && !code.contains("Handler") {
            diagnostics.push(create_command_handler_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCqrsSeparation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "cqrs_separation".to_string(),
            confidence: 0.92,
            description: "Implement CQRS properly: commands should not return data, queries should not modify state, use separate models for read and write operations.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_command_return_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCqrsSeparation::NAME.to_string(),
        message: "Commands should not return data. Use events or separate queries".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_query_modification_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCqrsSeparation::NAME.to_string(),
        message: "Queries should not modify state. Use commands for state changes".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_command_handler_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCqrsSeparation::NAME.to_string(),
        message: "Commands should have corresponding handlers".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require Event Sourcing immutable events
pub struct RequireImmutableEvents;

impl RequireImmutableEvents {
    pub const NAME: &'static str = "require-immutable-events";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireImmutableEvents {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for mutable event properties
        if code.contains("Event") && code.contains("set") {
            diagnostics.push(create_mutable_event_diagnostic());
        }

        // Check for missing event versioning
        if code.contains("Event") && !code.contains("version") {
            diagnostics.push(create_event_versioning_diagnostic());
        }

        // Check for event modification after creation
        if code.contains("event.") && code.contains("modify") {
            diagnostics.push(create_event_modification_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireImmutableEvents {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "immutable_events".to_string(),
            confidence: 0.94,
            description: "Implement immutable events: make event properties readonly, version events for schema evolution, never modify events after creation.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_mutable_event_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableEvents::NAME.to_string(),
        message: "Events should be immutable. Use readonly properties".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_event_versioning_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableEvents::NAME.to_string(),
        message: "Events should include version information for schema evolution".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_event_modification_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableEvents::NAME.to_string(),
        message: "Events should never be modified after creation".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require microservice boundaries and communication patterns
pub struct RequireMicroserviceBoundaries;

impl RequireMicroserviceBoundaries {
    pub const NAME: &'static str = "require-microservice-boundaries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMicroserviceBoundaries {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct database access between services
        if code.contains("service") && code.contains("database") && code.contains("direct") {
            diagnostics.push(create_database_boundary_diagnostic());
        }

        // Check for synchronous inter-service communication
        if code.contains("service") && code.contains("sync") && code.contains("call") {
            diagnostics.push(create_sync_communication_diagnostic());
        }

        // Check for shared data models
        if code.contains("shared") && code.contains("model") && code.contains("services") {
            diagnostics.push(create_shared_model_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMicroserviceBoundaries {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "microservice_boundaries".to_string(),
            confidence: 0.88,
            description: "Maintain microservice boundaries: use API contracts for communication, avoid shared databases, prefer async messaging, implement circuit breakers.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_database_boundary_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMicroserviceBoundaries::NAME.to_string(),
        message: "Services should not directly access other services' databases".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

fn create_sync_communication_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMicroserviceBoundaries::NAME.to_string(),
        message: "Prefer async communication between microservices to avoid coupling".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_shared_model_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMicroserviceBoundaries::NAME.to_string(),
        message: "Avoid shared data models between services. Use API contracts instead".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require saga pattern for distributed transactions
pub struct RequireSagaPattern;

impl RequireSagaPattern {
    pub const NAME: &'static str = "require-saga-pattern";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSagaPattern {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for distributed transactions without saga
        if code.contains("distributed") && code.contains("transaction") && !code.contains("saga") {
            diagnostics.push(create_saga_requirement_diagnostic());
        }

        // Check for missing compensation logic
        if code.contains("saga") && !code.contains("compensate") {
            diagnostics.push(create_compensation_diagnostic());
        }

        // Check for missing saga orchestration
        if code.contains("saga") && !code.contains("orchestrator") && !code.contains("choreography") {
            diagnostics.push(create_saga_orchestration_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSagaPattern {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "saga_pattern".to_string(),
            confidence: 0.86,
            description: "Implement saga pattern for distributed transactions: define compensation operations, choose orchestration vs choreography, handle partial failures gracefully.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_saga_requirement_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSagaPattern::NAME.to_string(),
        message: "Distributed transactions should use saga pattern instead of traditional ACID transactions".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_compensation_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSagaPattern::NAME.to_string(),
        message: "Saga implementation should include compensation logic for rollback".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_saga_orchestration_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSagaPattern::NAME.to_string(),
        message: "Saga should define orchestration or choreography pattern for coordination".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require circuit breaker pattern for resilience
pub struct RequireCircuitBreakerImplementation;

impl RequireCircuitBreakerImplementation {
    pub const NAME: &'static str = "require-circuit-breaker-implementation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireCircuitBreakerImplementation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for external service calls without circuit breaker
        if code.contains("external") && code.contains("service") && !code.contains("circuit") {
            diagnostics.push(create_circuit_breaker_requirement_diagnostic());
        }

        // Check for missing failure thresholds
        if code.contains("circuit") && !code.contains("threshold") {
            diagnostics.push(create_threshold_diagnostic());
        }

        // Check for missing timeout configuration
        if code.contains("circuit") && !code.contains("timeout") {
            diagnostics.push(create_timeout_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCircuitBreakerImplementation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "circuit_breaker".to_string(),
            confidence: 0.90,
            description: "Implement circuit breaker pattern: set failure thresholds, configure timeouts, implement half-open state for recovery testing, provide fallback mechanisms.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_circuit_breaker_requirement_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCircuitBreakerImplementation::NAME.to_string(),
        message: "External service calls should be protected with circuit breaker pattern".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_threshold_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCircuitBreakerImplementation::NAME.to_string(),
        message: "Circuit breaker should define failure threshold for state transitions".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_timeout_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCircuitBreakerImplementation::NAME.to_string(),
        message: "Circuit breaker should configure timeout for external calls".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require proper dependency injection patterns
pub struct RequireDependencyInjection;

impl RequireDependencyInjection {
    pub const NAME: &'static str = "require-dependency-injection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDependencyInjection {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for hard-coded dependencies
        if code.contains("new") && code.contains("Service") && !code.contains("inject") {
            diagnostics.push(create_hard_dependency_diagnostic());
        }

        // Check for singleton violations
        if code.contains("singleton") && code.contains("new") {
            diagnostics.push(create_singleton_violation_diagnostic());
        }

        // Check for missing interfaces
        if code.contains("dependency") && !code.contains("interface") {
            diagnostics.push(create_interface_requirement_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDependencyInjection {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "dependency_injection".to_string(),
            confidence: 0.87,
            description: "Implement dependency injection: use constructor injection, define clear interfaces, configure lifecycle management, avoid service locator anti-pattern.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_hard_dependency_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDependencyInjection::NAME.to_string(),
        message: "Avoid hard-coded dependencies. Use dependency injection instead".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_singleton_violation_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDependencyInjection::NAME.to_string(),
        message: "Singleton services should be managed by DI container, not instantiated directly".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_interface_requirement_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDependencyInjection::NAME.to_string(),
        message: "Dependencies should be defined through interfaces for loose coupling".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_aggregate_boundaries() {
        let rule = RequireAggregateBoundaries;

        // Test direct aggregate access
        let code_violation = "aggregate.directAccess();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper aggregate access
        let code_compliant = "aggregateRoot.performOperation();";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_cqrs_separation() {
        let rule = RequireCqrsSeparation;

        // Test command returning data
        let code_violation = r#"
            class CreateUserCommand {
                execute() {
                    return userData;
                }
            }
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper command
        let code_compliant = r#"
            class CreateUserCommand {
                execute() {
                    // Command with no return
                }
            }
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_immutable_events() {
        let rule = RequireImmutableEvents;

        // Test mutable event
        let code_violation = r#"
            class UserCreatedEvent {
                setUserId(id) {
                    this.userId = id;
                }
            }
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test immutable event
        let code_compliant = r#"
            class UserCreatedEvent {
                constructor(userId) {
                    this.readonly userId = userId;
                }
            }
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_microservice_boundaries() {
        let rule = RequireMicroserviceBoundaries;

        // Test direct database access
        let code_violation = "userService.directDatabaseAccess();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper service communication
        let code_compliant = "userService.apiCall();";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_saga_pattern() {
        let rule = RequireSagaPattern;

        // Test distributed transaction without saga
        let code_violation = "const distributedTransaction = new Transaction();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper saga implementation
        let code_compliant = r#"
            const saga = new OrderSaga();
            saga.compensate();
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_circuit_breaker_implementation() {
        let rule = RequireCircuitBreakerImplementation;

        // Test external service call without circuit breaker
        let code_violation = "externalService.call();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test with circuit breaker
        let code_compliant = "circuitBreaker.call(externalService);";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_dependency_injection() {
        let rule = RequireDependencyInjection;

        // Test hard-coded dependency
        let code_violation = "const service = new UserService();";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test dependency injection
        let code_compliant = "@inject('UserService') userService";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement() {
        let rule = RequireAggregateBoundaries;
        let diagnostics = vec![WasmRuleDiagnostic {
            rule_name: "require-aggregate-boundaries".to_string(),
            message: "Test message".to_string(),
            line: 1,
            column: 1,
            severity: "warning".to_string(),
        }];

        let suggestions = rule.ai_enhance("", &diagnostics);
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.8);
    }
}