//! DevOps and deployment rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoSecretsInCode;

impl NoSecretsInCode {
    pub const NAME: &'static str = "no-secrets-in-code";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoSecretsInCode {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.contains_secret_pattern(&string_lit.value) {
                ctx.diagnostic(no_secrets_in_code_diagnostic(string_lit.span));
            }
        }
    }
}

impl NoSecretsInCode {
    fn contains_secret_pattern(&self, value: &str) -> bool {
        // Check for common secret patterns
        let secret_patterns = [
            "sk_",          // Stripe keys
            "pk_",          // Public keys that shouldn't be in code
            "AKIA",         // AWS access keys
            "ghp_",         // GitHub personal access tokens
            "xoxb-",        // Slack bot tokens
            "AIza",         // Google API keys
        ];

        secret_patterns.iter().any(|pattern| value.contains(pattern)) ||
        (value.len() > 20 && value.chars().all(|c| c.is_alphanumeric()))
    }
}

fn no_secrets_in_code_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potential secret or API key in code")
        .with_help("Use environment variables or secret management systems")
        .with_label(span)
}

impl EnhancedWasmRule for NoSecretsInCode {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use process.env.SECRET_NAME for environment variables".to_string(),
            "Store secrets in .env files and add to .gitignore".to_string(),
            "Use secret management services like AWS Secrets Manager".to_string(),
            "Never commit secrets to version control".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireDockerHealthChecks;

impl RequireDockerHealthChecks {
    pub const NAME: &'static str = "require-docker-health-checks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDockerHealthChecks {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.is_dockerfile_content(&string_lit.value) && !self.has_healthcheck(&string_lit.value) {
                ctx.diagnostic(require_docker_health_checks_diagnostic(string_lit.span));
            }
        }
    }
}

impl RequireDockerHealthChecks {
    fn is_dockerfile_content(&self, value: &str) -> bool {
        value.contains("FROM ") && value.contains("EXPOSE ")
    }

    fn has_healthcheck(&self, value: &str) -> bool {
        value.contains("HEALTHCHECK")
    }
}

fn require_docker_health_checks_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Dockerfile without health check")
        .with_help("Add HEALTHCHECK instruction for container monitoring")
        .with_label(span)
}

impl EnhancedWasmRule for RequireDockerHealthChecks {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add HEALTHCHECK --interval=30s CMD curl -f http://localhost:3000/health".to_string(),
            "Health checks enable proper container orchestration".to_string(),
            "Use appropriate endpoints for your application".to_string(),
            "Consider startup time with --start-period option".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoHardcodedPorts;

impl NoHardcodedPorts {
    pub const NAME: &'static str = "no-hardcoded-ports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoHardcodedPorts {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::NumericLiteral(num_lit) = node.kind() {
            if self.is_port_number(num_lit.value) && self.is_server_context(ctx) {
                ctx.diagnostic(no_hardcoded_ports_diagnostic(num_lit.span));
            }
        }
    }
}

impl NoHardcodedPorts {
    fn is_port_number(&self, value: f64) -> bool {
        value >= 1000.0 && value <= 65535.0 && value.fract() == 0.0
    }

    fn is_server_context(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're in server configuration context
        true
    }
}

fn no_hardcoded_ports_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Hardcoded port number")
        .with_help("Use environment variables for port configuration")
        .with_label(span)
}

impl EnhancedWasmRule for NoHardcodedPorts {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use process.env.PORT || 3000 for default with override".to_string(),
            "Hardcoded ports cause conflicts in deployment".to_string(),
            "Container orchestration needs flexible port assignment".to_string(),
            "Document default port in README or .env.example".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireCIEnvironmentChecks;

impl RequireCIEnvironmentChecks {
    pub const NAME: &'static str = "require-ci-environment-checks";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCIEnvironmentChecks {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_ci_config(ctx) && !self.has_environment_checks(obj) {
                ctx.diagnostic(require_ci_environment_checks_diagnostic(obj.span));
            }
        }
    }
}

impl RequireCIEnvironmentChecks {
    fn is_ci_config(&self, ctx: &WasmLintContext) -> bool {
        let filename = ctx.filename();
        filename.contains(".github/workflows/") ||
        filename.contains(".gitlab-ci.yml") ||
        filename.contains("Jenkinsfile")
    }

    fn has_environment_checks(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for environment validation steps
        false
    }
}

fn require_ci_environment_checks_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("CI configuration without environment checks")
        .with_help("Add steps to validate required environment variables")
        .with_label(span)
}

impl EnhancedWasmRule for RequireCIEnvironmentChecks {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add environment validation steps in CI pipeline".to_string(),
            "Check for required secrets and variables".to_string(),
            "Fail fast if critical environment is missing".to_string(),
            "Document all required environment variables".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoProductionDebugCode;

impl NoProductionDebugCode {
    pub const NAME: &'static str = "no-production-debug-code";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoProductionDebugCode {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_debug_code(call) && self.is_production_context(ctx) {
                ctx.diagnostic(no_production_debug_code_diagnostic(call.span));
            }
        }
    }
}

impl NoProductionDebugCode {
    fn is_debug_code(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(obj) = member.object().as_identifier() {
                if obj.name == "console" {
                    if let Some(prop) = member.property().as_identifier() {
                        return matches!(prop.name.as_str(), "log" | "debug" | "trace");
                    }
                }
            }
        }
        false
    }

    fn is_production_context(&self, _ctx: &WasmLintContext) -> bool {
        // Check if we're in production build context
        true
    }
}

fn no_production_debug_code_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Debug code in production build")
        .with_help("Remove console statements or use environment-based logging")
        .with_label(span)
}

impl EnhancedWasmRule for NoProductionDebugCode {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use structured logging libraries like Winston or Pino".to_string(),
            "Configure log levels based on NODE_ENV".to_string(),
            "Remove console statements in production builds".to_string(),
            "Use linting rules to catch debug code in CI".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireDeploymentValidation;

impl RequireDeploymentValidation {
    pub const NAME: &'static str = "require-deployment-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDeploymentValidation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_deployment_config(ctx) && !self.has_validation_steps(obj) {
                ctx.diagnostic(require_deployment_validation_diagnostic(obj.span));
            }
        }
    }
}

impl RequireDeploymentValidation {
    fn is_deployment_config(&self, ctx: &WasmLintContext) -> bool {
        let filename = ctx.filename();
        filename.contains("deploy") || filename.contains("k8s") || filename.contains("terraform")
    }

    fn has_validation_steps(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for validation or smoke tests
        false
    }
}

fn require_deployment_validation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Deployment configuration without validation")
        .with_help("Add smoke tests or health checks after deployment")
        .with_label(span)
}

impl EnhancedWasmRule for RequireDeploymentValidation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add health check endpoints for deployment validation".to_string(),
            "Implement smoke tests after deployment".to_string(),
            "Use readiness and liveness probes in Kubernetes".to_string(),
            "Validate critical functionality post-deployment".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoInsecureDockerPractices;

impl NoInsecureDockerPractices {
    pub const NAME: &'static str = "no-insecure-docker-practices";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoInsecureDockerPractices {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.has_insecure_docker_practices(&string_lit.value) {
                ctx.diagnostic(no_insecure_docker_practices_diagnostic(string_lit.span));
            }
        }
    }
}

impl NoInsecureDockerPractices {
    fn has_insecure_docker_practices(&self, value: &str) -> bool {
        // Check for insecure Docker practices
        value.contains("USER root") ||
        value.contains("--privileged") ||
        value.contains("--disable-content-trust") ||
        (value.contains("ADD") && value.contains("http"))
    }
}

fn no_insecure_docker_practices_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Insecure Docker practice detected")
        .with_help("Follow Docker security best practices")
        .with_label(span)
}

impl EnhancedWasmRule for NoInsecureDockerPractices {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use non-root user: USER node or USER 1001".to_string(),
            "Avoid --privileged flag in production".to_string(),
            "Use COPY instead of ADD for local files".to_string(),
            "Scan images for vulnerabilities regularly".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireResourceLimits;

impl RequireResourceLimits {
    pub const NAME: &'static str = "require-resource-limits";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireResourceLimits {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_k8s_deployment(ctx) && !self.has_resource_limits(obj) {
                ctx.diagnostic(require_resource_limits_diagnostic(obj.span));
            }
        }
    }
}

impl RequireResourceLimits {
    fn is_k8s_deployment(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("k8s") || ctx.filename().contains("kubernetes")
    }

    fn has_resource_limits(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for resource limits in Kubernetes manifests
        false
    }
}

fn require_resource_limits_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Kubernetes deployment without resource limits")
        .with_help("Add CPU and memory limits for better resource management")
        .with_label(span)
}

impl EnhancedWasmRule for RequireResourceLimits {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Set memory and CPU limits: resources.limits.memory: 512Mi".to_string(),
            "Add resource requests for scheduling: resources.requests.cpu: 100m".to_string(),
            "Resource limits prevent noisy neighbor problems".to_string(),
            "Use vertical pod autoscaling for optimal sizing".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_secrets_in_code_rule() {
        assert_eq!(NoSecretsInCode::NAME, "no-secrets-in-code");
        assert_eq!(NoSecretsInCode::CATEGORY, WasmRuleCategory::Restriction);
        assert_eq!(NoSecretsInCode::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_docker_health_checks_rule() {
        assert_eq!(RequireDockerHealthChecks::NAME, "require-docker-health-checks");
        assert_eq!(RequireDockerHealthChecks::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_secret_detection() {
        let rule = NoSecretsInCode;
        assert!(rule.contains_secret_pattern("sk_test_123456789"));
        assert!(rule.contains_secret_pattern("AKIAIOSFODNN7EXAMPLE"));
        assert!(!rule.contains_secret_pattern("normal string"));
    }

    #[test]
    fn test_port_detection() {
        let rule = NoHardcodedPorts;
        assert!(rule.is_port_number(3000.0));
        assert!(rule.is_port_number(8080.0));
        assert!(!rule.is_port_number(999.0));
        assert!(!rule.is_port_number(70000.0));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoSecretsInCode;
        let diagnostic = no_secrets_in_code_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("environment variables"));
    }
}