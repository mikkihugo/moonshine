//! # OXC Cloud-Native and Container Orchestration Rules
//!
//! This module implements WASM-safe OXC rules for cloud-native development,
//! container orchestration, Kubernetes patterns, and distributed system best practices.
//!
//! ## Rule Categories:
//! - **Container Security**: Docker best practices and secure container patterns
//! - **Kubernetes Patterns**: Pod, Service, and Deployment configuration
//! - **Service Mesh**: Istio, Linkerd, and traffic management patterns
//! - **Cloud Platform Integration**: AWS, GCP, Azure specific patterns
//! - **Observability**: Logging, metrics, and distributed tracing
//! - **Resource Management**: CPU, memory, and scaling optimization
//! - **Network Security**: Service-to-service communication and policies
//! - **Configuration Management**: ConfigMaps, Secrets, and environment handling
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
// Container Security and Docker Rules
// ================================================================================================

/// Prevents running containers as root user
pub struct NoRootContainers;

impl NoRootContainers {
    pub const NAME: &'static str = "no-root-containers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoRootContainers {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("FROM") && !code.contains("USER") &&
           !code.contains("runAsUser") && !code.contains("runAsNonRoot") {
            diagnostics.push(create_root_containers_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoRootContainers {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add USER instruction in Dockerfile or set runAsNonRoot: true in Kubernetes SecurityContext to prevent privilege escalation attacks".to_string(),
            confidence: 0.97,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires resource limits for containers
pub struct RequireContainerLimits;

impl RequireContainerLimits {
    pub const NAME: &'static str = "require-container-limits";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireContainerLimits {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("containers:") && !code.contains("resources:") &&
           !code.contains("limits:") && !code.contains("requests:") {
            diagnostics.push(create_container_limits_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireContainerLimits {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Set CPU and memory limits/requests for containers to prevent resource exhaustion and enable proper scheduling".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents privileged containers
pub struct NoPrivilegedContainers;

impl NoPrivilegedContainers {
    pub const NAME: &'static str = "no-privileged-containers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoPrivilegedContainers {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("privileged: true") || code.contains("--privileged") {
            diagnostics.push(create_privileged_containers_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoPrivilegedContainers {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Remove privileged mode and use specific capabilities instead - privileged containers have full host access and pose security risks".to_string(),
            confidence: 0.98,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Kubernetes Configuration Rules
// ================================================================================================

/// Requires health checks for Kubernetes pods
pub struct RequireK8sHealthChecks;

impl RequireK8sHealthChecks {
    pub const NAME: &'static str = "require-k8s-health-checks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireK8sHealthChecks {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("kind: Pod") || code.contains("kind: Deployment") &&
           !code.contains("livenessProbe") && !code.contains("readinessProbe") {
            diagnostics.push(create_k8s_health_checks_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireK8sHealthChecks {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add livenessProbe and readinessProbe to containers for proper health monitoring and graceful deployments".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces proper Service configuration
pub struct RequireProperServices;

impl RequireProperServices {
    pub const NAME: &'static str = "require-proper-services";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireProperServices {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("kind: Service") && code.contains("type: LoadBalancer") &&
           !code.contains("selector:") {
            diagnostics.push(create_proper_services_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperServices {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Kubernetes Services should have proper selectors to match target pods and consider using ClusterIP with Ingress instead of LoadBalancer for cost optimization".to_string(),
            confidence: 0.88,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires proper namespace usage
pub struct RequireNamespaces;

impl RequireNamespaces {
    pub const NAME: &'static str = "require-namespaces";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireNamespaces {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("kind: Pod") || code.contains("kind: Deployment")) &&
           !code.contains("namespace:") && !code.contains("default") {
            diagnostics.push(create_namespaces_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireNamespaces {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use explicit namespaces for better resource organization, security boundaries, and multi-tenancy support".to_string(),
            confidence: 0.85,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Service Mesh and Network Security Rules
// ================================================================================================

/// Requires network policies for pod communication
pub struct RequireNetworkPolicies;

impl RequireNetworkPolicies {
    pub const NAME: &'static str = "require-network-policies";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireNetworkPolicies {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("kind: Deployment") && !code.contains("NetworkPolicy") &&
           !code.contains("network-policy") {
            diagnostics.push(create_network_policies_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireNetworkPolicies {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement NetworkPolicies to restrict pod-to-pod communication and follow principle of least privilege for network access".to_string(),
            confidence: 0.89,
            auto_fixable: false,
        }).collect()
    }
}

/// Enforces mTLS for service communication
pub struct RequireMutualTLS;

impl RequireMutualTLS {
    pub const NAME: &'static str = "require-mutual-tls";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMutualTLS {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("istio") || code.contains("linkerd")) &&
           !code.contains("PeerAuthentication") && !code.contains("STRICT") {
            diagnostics.push(create_mutual_tls_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMutualTLS {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Enable strict mTLS in service mesh for encrypted service-to-service communication and identity verification".to_string(),
            confidence: 0.91,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Cloud Platform Integration Rules
// ================================================================================================

/// Requires proper IAM roles instead of admin access
pub struct RequireProperIAM;

impl RequireProperIAM {
    pub const NAME: &'static str = "require-proper-iam";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for RequireProperIAM {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("admin") || code.contains("AdministratorAccess") || code.contains("*:*")) &&
           (code.contains("Role") || code.contains("Policy")) {
            diagnostics.push(create_proper_iam_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProperIAM {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use principle of least privilege - create specific IAM roles with minimal required permissions instead of admin access".to_string(),
            confidence: 0.96,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires encryption at rest for cloud storage
pub struct RequireEncryptionAtRest;

impl RequireEncryptionAtRest {
    pub const NAME: &'static str = "require-encryption-at-rest";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireEncryptionAtRest {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("S3") || code.contains("StorageClass") || code.contains("Volume")) &&
           !code.contains("encrypted") && !code.contains("encryption") {
            diagnostics.push(create_encryption_at_rest_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireEncryptionAtRest {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Enable encryption at rest for storage resources to protect sensitive data and meet compliance requirements".to_string(),
            confidence: 0.93,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Configuration and Secret Management Rules
// ================================================================================================

/// Prevents secrets in configuration files
pub struct NoHardcodedSecrets;

impl NoHardcodedSecrets {
    pub const NAME: &'static str = "no-hardcoded-secrets";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoHardcodedSecrets {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let secret_patterns = ["password:", "token:", "key:", "secret:", "api_key:"];

        if secret_patterns.iter().any(|&pattern| code.contains(pattern)) &&
           !code.contains("secretKeyRef") && !code.contains("Secret") {
            diagnostics.push(create_hardcoded_secrets_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoHardcodedSecrets {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use Kubernetes Secrets or external secret management systems (Vault, AWS Secrets Manager) instead of hardcoded values".to_string(),
            confidence: 0.98,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires proper ConfigMap usage
pub struct RequireConfigMaps;

impl RequireConfigMaps {
    pub const NAME: &'static str = "require-config-maps";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireConfigMaps {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("env:") && code.contains("value:") &&
           !code.contains("configMapKeyRef") && !code.contains("ConfigMap") {
            diagnostics.push(create_config_maps_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireConfigMaps {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use ConfigMaps for non-sensitive configuration data to separate configuration from application code".to_string(),
            confidence: 0.86,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_root_containers_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoRootContainers::NAME.to_string(),
        message: "Containers should not run as root user for security".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add USER instruction or set runAsNonRoot: true".to_string()),
    }
}

fn create_container_limits_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireContainerLimits::NAME.to_string(),
        message: "Containers must have resource limits and requests defined".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add resources.limits and resources.requests".to_string()),
    }
}

fn create_privileged_containers_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoPrivilegedContainers::NAME.to_string(),
        message: "Privileged containers pose significant security risks".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Remove privileged mode and use specific capabilities".to_string()),
    }
}

fn create_k8s_health_checks_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireK8sHealthChecks::NAME.to_string(),
        message: "Kubernetes pods should have health checks configured".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add livenessProbe and readinessProbe".to_string()),
    }
}

fn create_proper_services_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperServices::NAME.to_string(),
        message: "Kubernetes Services should have proper configuration".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add proper selectors and consider service type optimization".to_string()),
    }
}

fn create_namespaces_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireNamespaces::NAME.to_string(),
        message: "Use explicit namespaces for better resource organization".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Specify namespace in metadata".to_string()),
    }
}

fn create_network_policies_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireNetworkPolicies::NAME.to_string(),
        message: "Implement NetworkPolicies for network security".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add NetworkPolicy resources to restrict traffic".to_string()),
    }
}

fn create_mutual_tls_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMutualTLS::NAME.to_string(),
        message: "Enable mTLS for secure service-to-service communication".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Configure PeerAuthentication with STRICT mode".to_string()),
    }
}

fn create_proper_iam_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProperIAM::NAME.to_string(),
        message: "Avoid admin access - use principle of least privilege".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Create specific IAM roles with minimal permissions".to_string()),
    }
}

fn create_encryption_at_rest_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireEncryptionAtRest::NAME.to_string(),
        message: "Enable encryption at rest for storage resources".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add encryption configuration to storage resources".to_string()),
    }
}

fn create_hardcoded_secrets_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoHardcodedSecrets::NAME.to_string(),
        message: "Never hardcode secrets in configuration files".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use Kubernetes Secrets or external secret management".to_string()),
    }
}

fn create_config_maps_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireConfigMaps::NAME.to_string(),
        message: "Use ConfigMaps for configuration data".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace direct values with configMapKeyRef".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_containers_detection() {
        let code = r#"FROM ubuntu:20.04
                      RUN apt-get update"#;
        let rule = NoRootContainers;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoRootContainers::NAME);
    }

    #[test]
    fn test_container_limits_detection() {
        let code = r#"spec:
                        containers:
                        - name: app
                          image: nginx"#;
        let rule = RequireContainerLimits;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireContainerLimits::NAME);
    }

    #[test]
    fn test_privileged_containers_detection() {
        let code = r#"securityContext:
                        privileged: true"#;
        let rule = NoPrivilegedContainers;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoPrivilegedContainers::NAME);
    }

    #[test]
    fn test_k8s_health_checks_detection() {
        let code = r#"apiVersion: apps/v1
                      kind: Deployment
                      spec:
                        template:
                          spec:
                            containers:
                            - name: app"#;
        let rule = RequireK8sHealthChecks;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireK8sHealthChecks::NAME);
    }

    #[test]
    fn test_proper_iam_detection() {
        let code = r#"Role:
                        Effect: Allow
                        Action: "*:*"
                        Resource: "*""#;
        let rule = RequireProperIAM;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireProperIAM::NAME);
    }

    #[test]
    fn test_hardcoded_secrets_detection() {
        let code = r#"env:
                      - name: DATABASE_PASSWORD
                        value: "super_secret_password""#;
        let rule = NoHardcodedSecrets;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoHardcodedSecrets::NAME);
    }

    #[test]
    fn test_config_maps_detection() {
        let code = r#"env:
                      - name: APP_CONFIG
                        value: "production""#;
        let rule = RequireConfigMaps;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireConfigMaps::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"FROM ubuntu:20.04"#;
        let rule = NoRootContainers;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.95);
        assert!(suggestions[0].auto_fixable);
    }
}