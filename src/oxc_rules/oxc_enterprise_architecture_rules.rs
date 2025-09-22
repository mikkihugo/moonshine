//! # OXC Enterprise Architecture Patterns Rules
//!
//! This module implements WASM-safe OXC rules for enterprise architecture patterns,
//! system integration, service orchestration, and large-scale application design.
//!
//! ## Rule Categories:
//! - **Domain-Driven Design**: Aggregate, Entity, Value Object, and Repository patterns
//! - **CQRS and Event Sourcing**: Command Query Responsibility Segregation patterns
//! - **Service Integration**: Service-oriented architecture and API gateway patterns
//! - **Enterprise Integration Patterns**: Message routing, transformation, and mediation
//! - **Microservices Governance**: Service boundaries, communication, and resilience
//! - **Event-Driven Architecture**: Event choreography and orchestration patterns
//! - **Distributed System Patterns**: Circuit breaker, saga, and timeout patterns
//! - **Enterprise Security**: Authentication, authorization, and audit patterns
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
// Domain-Driven Design (DDD) Rules
// ================================================================================================

/// Enforces proper aggregate boundary design in DDD implementations
pub struct RequireAggregateBoundaries;

impl RequireAggregateBoundaries {
    pub const NAME: &'static str = "require-aggregate-boundaries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAggregateBoundaries {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("class") && code.contains("Aggregate") &&
           !code.contains("private") && code.contains("public") {
            diagnostics.push(create_aggregate_boundaries_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAggregateBoundaries {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Encapsulate aggregate internals with private fields and expose behavior through public methods to maintain consistency boundaries".to_string(),
            confidence: 0.87,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents direct entity modification outside aggregates
pub struct NoDirectEntityModification;

impl NoDirectEntityModification {
    pub const NAME: &'static str = "no-direct-entity-modification";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDirectEntityModification {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("entity.") && code.contains(" = ") &&
           !code.contains("aggregate.") && !code.contains("setEntity") {
            diagnostics.push(create_direct_entity_modification_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoDirectEntityModification {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Modify entities through their containing aggregate to maintain business invariants and consistency".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// CQRS and Event Sourcing Rules
// ================================================================================================

/// Enforces separation between commands and queries in CQRS pattern
pub struct RequireCqrsSeparation;

impl RequireCqrsSeparation {
    pub const NAME: &'static str = "require-cqrs-separation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCqrsSeparation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("Command") || code.contains("Query")) &&
           code.contains("return") && code.contains("save") {
            diagnostics.push(create_cqrs_separation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCqrsSeparation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Separate commands (write operations) from queries (read operations) - commands should not return data, queries should not modify state".to_string(),
            confidence: 0.89,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires immutable events in event sourcing implementations
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

        if code.contains("Event") && code.contains("class") &&
           !code.contains("readonly") && !code.contains("Object.freeze") {
            diagnostics.push(create_immutable_events_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireImmutableEvents {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Make event properties readonly or use Object.freeze() to ensure events are immutable once created".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Service Integration and API Gateway Rules
// ================================================================================================

/// Requires timeout configuration for service calls
pub struct RequireServiceTimeouts;

impl RequireServiceTimeouts {
    pub const NAME: &'static str = "require-service-timeouts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireServiceTimeouts {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("fetch(") || code.contains("axios.")) &&
           !code.contains("timeout") && !code.contains("AbortSignal") {
            diagnostics.push(create_service_timeouts_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireServiceTimeouts {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Configure appropriate timeouts for service calls to prevent hanging requests and improve system resilience".to_string(),
            confidence: 0.91,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces circuit breaker pattern for external service calls
pub struct RequireCircuitBreaker;

impl RequireCircuitBreaker {
    pub const NAME: &'static str = "require-circuit-breaker";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCircuitBreaker {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("external") && code.contains("service") &&
           code.contains("fetch") && !code.contains("circuit") && !code.contains("breaker") {
            diagnostics.push(create_circuit_breaker_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCircuitBreaker {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement circuit breaker pattern for external service calls to prevent cascading failures in distributed systems".to_string(),
            confidence: 0.86,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Event-Driven Architecture Rules
// ================================================================================================

/// Requires proper event versioning for backward compatibility
pub struct RequireEventVersioning;

impl RequireEventVersioning {
    pub const NAME: &'static str = "require-event-versioning";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireEventVersioning {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("publishEvent") && !code.contains("version") &&
           !code.contains("schemaVersion") {
            diagnostics.push(create_event_versioning_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireEventVersioning {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Include version information in events to enable backward compatibility and schema evolution".to_string(),
            confidence: 0.88,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents synchronous processing in event handlers
pub struct NoSyncEventProcessing;

impl NoSyncEventProcessing {
    pub const NAME: &'static str = "no-sync-event-processing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoSyncEventProcessing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("eventHandler") && !code.contains("async") &&
           code.contains("process") {
            diagnostics.push(create_sync_event_processing_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoSyncEventProcessing {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Make event handlers async to prevent blocking the event loop and improve system responsiveness".to_string(),
            confidence: 0.93,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Enterprise Security and Governance Rules
// ================================================================================================

/// Requires proper audit trail for enterprise operations
pub struct RequireAuditTrail;

impl RequireAuditTrail {
    pub const NAME: &'static str = "require-audit-trail";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAuditTrail {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let sensitive_operations = ["delete", "update", "create", "modify"];

        if sensitive_operations.iter().any(|&op| code.contains(op)) &&
           !code.contains("audit") && !code.contains("log") {
            diagnostics.push(create_audit_trail_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAuditTrail {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add audit logging for sensitive operations to maintain compliance and traceability in enterprise environments".to_string(),
            confidence: 0.90,
            auto_fixable: false,
        }).collect()
    }
}

/// Enforces proper service boundary isolation
pub struct RequireServiceBoundaries;

impl RequireServiceBoundaries {
    pub const NAME: &'static str = "require-service-boundaries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireServiceBoundaries {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("import") && code.contains("../") &&
           code.contains("service") && code.contains("../service") {
            diagnostics.push(create_service_boundaries_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireServiceBoundaries {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Maintain clear service boundaries by avoiding direct cross-service imports - use well-defined APIs and interfaces instead".to_string(),
            confidence: 0.85,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_aggregate_boundaries_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAggregateBoundaries::NAME.to_string(),
        message: "Aggregate should encapsulate internal state and expose behavior through methods".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Make aggregate fields private and provide public methods for behavior".to_string()),
    }
}

fn create_direct_entity_modification_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoDirectEntityModification::NAME.to_string(),
        message: "Entities should be modified through their containing aggregate".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use aggregate methods to modify entities".to_string()),
    }
}

fn create_cqrs_separation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCqrsSeparation::NAME.to_string(),
        message: "Commands should not return data, queries should not modify state".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Separate command and query responsibilities".to_string()),
    }
}

fn create_immutable_events_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableEvents::NAME.to_string(),
        message: "Events should be immutable once created".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Make event properties readonly or use Object.freeze()".to_string()),
    }
}

fn create_service_timeouts_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireServiceTimeouts::NAME.to_string(),
        message: "Service calls should include timeout configuration".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add timeout parameter or AbortSignal to service calls".to_string()),
    }
}

fn create_circuit_breaker_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCircuitBreaker::NAME.to_string(),
        message: "External service calls should use circuit breaker pattern".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement circuit breaker for external service resilience".to_string()),
    }
}

fn create_event_versioning_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireEventVersioning::NAME.to_string(),
        message: "Events should include version information for backward compatibility".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add version or schemaVersion field to events".to_string()),
    }
}

fn create_sync_event_processing_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoSyncEventProcessing::NAME.to_string(),
        message: "Event handlers should be asynchronous to prevent blocking".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Make event handler function async".to_string()),
    }
}

fn create_audit_trail_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAuditTrail::NAME.to_string(),
        message: "Sensitive operations should include audit logging".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add audit logging for compliance and traceability".to_string()),
    }
}

fn create_service_boundaries_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireServiceBoundaries::NAME.to_string(),
        message: "Avoid direct cross-service imports - use defined APIs instead".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace direct imports with proper service interfaces".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_boundaries_detection() {
        let code = r#"class UserAggregate { public name: string; public email: string; }"#;
        let rule = RequireAggregateBoundaries;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireAggregateBoundaries::NAME);
    }

    #[test]
    fn test_direct_entity_modification_detection() {
        let code = r#"entity.status = 'updated';"#;
        let rule = NoDirectEntityModification;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoDirectEntityModification::NAME);
    }

    #[test]
    fn test_cqrs_separation_detection() {
        let code = r#"class UpdateUserCommand { execute() { save(); return user; } }"#;
        let rule = RequireCqrsSeparation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireCqrsSeparation::NAME);
    }

    #[test]
    fn test_immutable_events_detection() {
        let code = r#"class UserCreatedEvent { userId: string; name: string; }"#;
        let rule = RequireImmutableEvents;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireImmutableEvents::NAME);
    }

    #[test]
    fn test_service_timeouts_detection() {
        let code = r#"const response = await fetch('/api/users');"#;
        let rule = RequireServiceTimeouts;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireServiceTimeouts::NAME);
    }

    #[test]
    fn test_circuit_breaker_detection() {
        let code = r#"const externalService = await fetch('https://external-api.com/data');"#;
        let rule = RequireCircuitBreaker;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireCircuitBreaker::NAME);
    }

    #[test]
    fn test_event_versioning_detection() {
        let code = r#"publishEvent({ type: 'UserCreated', data: userData });"#;
        let rule = RequireEventVersioning;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireEventVersioning::NAME);
    }

    #[test]
    fn test_sync_event_processing_detection() {
        let code = r#"function eventHandler(event) { process(event); }"#;
        let rule = NoSyncEventProcessing;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoSyncEventProcessing::NAME);
    }

    #[test]
    fn test_audit_trail_detection() {
        let code = r#"function deleteUser(id) { users.delete(id); }"#;
        let rule = RequireAuditTrail;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireAuditTrail::NAME);
    }

    #[test]
    fn test_service_boundaries_detection() {
        let code = r#"import { UserService } from '../user-service/user.service';"#;
        let rule = RequireServiceBoundaries;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireServiceBoundaries::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"entity.status = 'modified';"#;
        let rule = NoDirectEntityModification;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].auto_fixable);
    }
}