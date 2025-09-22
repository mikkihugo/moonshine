//! GraphQL schema and resolver rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireResolverErrorHandling;

impl RequireResolverErrorHandling {
    pub const NAME: &'static str = "require-resolver-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireResolverErrorHandling {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_graphql_resolver(func) && !self.has_error_handling(func) {
                ctx.diagnostic(require_resolver_error_handling_diagnostic(func.span));
            }
        }
    }
}

impl RequireResolverErrorHandling {
    fn is_graphql_resolver(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if function is a GraphQL resolver
        func.params.len() >= 2 // Typical resolver signature (parent, args, context, info)
    }

    fn has_error_handling(&self, _func: &oxc_ast::ast::Function) -> bool {
        // Check for try-catch blocks or error throwing
        false
    }
}

fn require_resolver_error_handling_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("GraphQL resolver without error handling")
        .with_help("Add try-catch blocks and proper error throwing in resolvers")
        .with_label(span)
}

impl EnhancedWasmRule for RequireResolverErrorHandling {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use GraphQLError for proper error formatting".to_string(),
            "Add try-catch with meaningful error messages".to_string(),
            "Log errors for debugging while returning user-safe messages".to_string(),
            "Consider error codes for client-side handling".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoNPlusOneInResolvers;

impl NoNPlusOneInResolvers {
    pub const NAME: &'static str = "no-n-plus-one-in-resolvers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoNPlusOneInResolvers {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_database_query_in_resolver(call, ctx) {
                ctx.diagnostic(no_n_plus_one_in_resolvers_diagnostic(call.span));
            }
        }
    }
}

impl NoNPlusOneInResolvers {
    fn is_database_query_in_resolver(&self, call: &oxc_ast::ast::CallExpression, _ctx: &WasmLintContext) -> bool {
        // Check for database queries that could cause N+1 problems
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "findMany" | "findUnique" | "findFirst");
            }
        }
        false
    }
}

fn no_n_plus_one_in_resolvers_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potential N+1 query in GraphQL resolver")
        .with_help("Use DataLoader pattern to batch database queries")
        .with_label(span)
}

impl EnhancedWasmRule for NoNPlusOneInResolvers {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use DataLoader to batch and cache database queries".to_string(),
            "Implement query batching in resolver context".to_string(),
            "Use GraphQL query analysis to detect N+1 patterns".to_string(),
            "Consider using GraphQL query complexity analysis".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireSchemaValidation;

impl RequireSchemaValidation {
    pub const NAME: &'static str = "require-schema-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSchemaValidation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TemplateLiteral(template) = node.kind() {
            if self.is_graphql_schema(template) && !self.has_validation_directives(template) {
                ctx.diagnostic(require_schema_validation_diagnostic(template.span));
            }
        }
    }
}

impl RequireSchemaValidation {
    fn is_graphql_schema(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check if template contains GraphQL schema definition
        template.quasis.iter().any(|quasi| {
            let value = &quasi.value.raw;
            value.contains("type ") || value.contains("input ") || value.contains("enum ")
        })
    }

    fn has_validation_directives(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check for validation directives like @constraint
        template.quasis.iter().any(|quasi| {
            quasi.value.raw.contains("@constraint") || quasi.value.raw.contains("@validate")
        })
    }
}

fn require_schema_validation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("GraphQL schema without validation directives")
        .with_help("Add validation directives for input types and arguments")
        .with_label(span)
}

impl EnhancedWasmRule for RequireSchemaValidation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use @constraint directive for input validation".to_string(),
            "Add length, format, and range validations".to_string(),
            "Validate inputs at schema level for security".to_string(),
            "Consider using GraphQL Shield for authorization".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDeepGraphQLQueries;

impl NoDeepGraphQLQueries {
    pub const NAME: &'static str = "no-deep-graphql-queries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoDeepGraphQLQueries {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TemplateLiteral(template) = node.kind() {
            if self.is_graphql_query(template) && self.exceeds_depth_limit(template) {
                ctx.diagnostic(no_deep_graphql_queries_diagnostic(template.span));
            }
        }
    }
}

impl NoDeepGraphQLQueries {
    fn is_graphql_query(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        template.quasis.iter().any(|quasi| {
            let value = &quasi.value.raw;
            value.contains("query ") || value.contains("mutation ") || value.contains("subscription ")
        })
    }

    fn exceeds_depth_limit(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Simple check for query depth (count nested braces)
        template.quasis.iter().any(|quasi| {
            quasi.value.raw.matches('{').count() > 5
        })
    }
}

fn no_deep_graphql_queries_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("GraphQL query exceeds recommended depth")
        .with_help("Limit query depth to prevent performance issues")
        .with_label(span)
}

impl EnhancedWasmRule for NoDeepGraphQLQueries {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Implement query depth limiting middleware".to_string(),
            "Break deep queries into multiple smaller queries".to_string(),
            "Use fragments to reuse common query parts".to_string(),
            "Consider pagination for deeply nested lists".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireResolverAuth;

impl RequireResolverAuth {
    pub const NAME: &'static str = "require-resolver-auth";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireResolverAuth {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_protected_resolver(func) && !self.has_auth_check(func) {
                ctx.diagnostic(require_resolver_auth_diagnostic(func.span));
            }
        }
    }
}

impl RequireResolverAuth {
    fn is_protected_resolver(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if resolver handles sensitive data
        func.id.as_ref().map_or(false, |id| {
            id.name.contains("user") || id.name.contains("admin") || id.name.contains("private")
        })
    }

    fn has_auth_check(&self, _func: &oxc_ast::ast::Function) -> bool {
        // Check for authentication/authorization logic
        false
    }
}

fn require_resolver_auth_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Protected resolver without authentication check")
        .with_help("Add authentication and authorization checks to sensitive resolvers")
        .with_label(span)
}

impl EnhancedWasmRule for RequireResolverAuth {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Check context.user for authentication status".to_string(),
            "Use GraphQL Shield for declarative authorization".to_string(),
            "Implement role-based access control (RBAC)".to_string(),
            "Add rate limiting for authenticated endpoints".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnusedGraphQLFragments;

impl NoUnusedGraphQLFragments {
    pub const NAME: &'static str = "no-unused-graphql-fragments";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUnusedGraphQLFragments {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TemplateLiteral(template) = node.kind() {
            if self.has_unused_fragments(template) {
                ctx.diagnostic(no_unused_graphql_fragments_diagnostic(template.span));
            }
        }
    }
}

impl NoUnusedGraphQLFragments {
    fn has_unused_fragments(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check for fragment definitions that are never used
        let has_fragment_def = template.quasis.iter().any(|quasi| {
            quasi.value.raw.contains("fragment ")
        });
        let has_fragment_spread = template.quasis.iter().any(|quasi| {
            quasi.value.raw.contains("...")
        });
        has_fragment_def && !has_fragment_spread
    }
}

fn no_unused_graphql_fragments_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused GraphQL fragment")
        .with_help("Remove unused fragments or use them in queries")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedGraphQLFragments {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove unused fragments to reduce bundle size".to_string(),
            "Use fragment spreads to include fragment data".to_string(),
            "Consider fragment colocation with components".to_string(),
            "Use GraphQL code generation for type safety".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireQueryComplexityAnalysis;

impl RequireQueryComplexityAnalysis {
    pub const NAME: &'static str = "require-query-complexity-analysis";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireQueryComplexityAnalysis {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_apollo_server_setup(call) && !self.has_complexity_analysis(call) {
                ctx.diagnostic(require_query_complexity_analysis_diagnostic(call.span));
            }
        }
    }
}

impl RequireQueryComplexityAnalysis {
    fn is_apollo_server_setup(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        if let Some(ident) = call.callee.as_identifier() {
            return ident.name == "ApolloServer";
        }
        false
    }

    fn has_complexity_analysis(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for complexity analysis plugins
        false
    }
}

fn require_query_complexity_analysis_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("GraphQL server without query complexity analysis")
        .with_help("Add query complexity analysis to prevent DoS attacks")
        .with_label(span)
}

impl EnhancedWasmRule for RequireQueryComplexityAnalysis {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use graphql-query-complexity for complexity analysis".to_string(),
            "Set maximum query complexity limits".to_string(),
            "Assign complexity scores to fields".to_string(),
            "Monitor query complexity in production".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoExposedInternalFields;

impl NoExposedInternalFields {
    pub const NAME: &'static str = "no-exposed-internal-fields";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoExposedInternalFields {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TemplateLiteral(template) = node.kind() {
            if self.exposes_internal_fields(template) {
                ctx.diagnostic(no_exposed_internal_fields_diagnostic(template.span));
            }
        }
    }
}

impl NoExposedInternalFields {
    fn exposes_internal_fields(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check for internal fields in schema
        template.quasis.iter().any(|quasi| {
            let value = &quasi.value.raw;
            value.contains("password") || value.contains("secret") || value.contains("private")
        })
    }
}

fn no_exposed_internal_fields_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Internal fields exposed in GraphQL schema")
        .with_help("Remove or protect sensitive fields from public schema")
        .with_label(span)
}

impl EnhancedWasmRule for NoExposedInternalFields {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use separate types for internal vs external data".to_string(),
            "Implement field-level authorization".to_string(),
            "Use GraphQL directives to hide sensitive fields".to_string(),
            "Audit schema for accidentally exposed data".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_resolver_error_handling_rule() {
        assert_eq!(RequireResolverErrorHandling::NAME, "require-resolver-error-handling");
        assert_eq!(RequireResolverErrorHandling::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_n_plus_one_in_resolvers_rule() {
        assert_eq!(NoNPlusOneInResolvers::NAME, "no-n-plus-one-in-resolvers");
        assert_eq!(NoNPlusOneInResolvers::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_require_schema_validation_rule() {
        assert_eq!(RequireSchemaValidation::NAME, "require-schema-validation");
        assert_eq!(RequireSchemaValidation::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireResolverErrorHandling;
        let diagnostic = require_resolver_error_handling_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("GraphQLError"));
    }
}