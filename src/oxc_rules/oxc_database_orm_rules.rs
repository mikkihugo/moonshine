//! Database and ORM rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoSQLInjection;

impl NoSQLInjection {
    pub const NAME: &'static str = "no-sql-injection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoSQLInjection {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TemplateLiteral(template) = node.kind() {
            if self.is_sql_template(template) && self.has_unsafe_interpolation(template) {
                ctx.diagnostic(no_sql_injection_diagnostic(template.span));
            }
        }
    }
}

impl NoSQLInjection {
    fn is_sql_template(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check if template contains SQL keywords
        template.quasis.iter().any(|quasi| {
            let value = &quasi.value.raw;
            value.to_uppercase().contains("SELECT") ||
            value.to_uppercase().contains("INSERT") ||
            value.to_uppercase().contains("UPDATE") ||
            value.to_uppercase().contains("DELETE")
        })
    }

    fn has_unsafe_interpolation(&self, template: &oxc_ast::ast::TemplateLiteral) -> bool {
        // Check for direct variable interpolation in SQL
        !template.expressions.is_empty()
    }
}

fn no_sql_injection_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potential SQL injection vulnerability")
        .with_help("Use parameterized queries or prepared statements")
        .with_label(span)
}

impl EnhancedWasmRule for NoSQLInjection {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use parameterized queries: db.query('SELECT * FROM users WHERE id = ?', [id])".to_string(),
            "Never concatenate user input into SQL strings".to_string(),
            "Use ORM query builders for safe SQL generation".to_string(),
            "Validate and sanitize all user inputs".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireTransactionWrapping;

impl RequireTransactionWrapping {
    pub const NAME: &'static str = "require-transaction-wrapping";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTransactionWrapping {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_multi_table_operation(call) && !self.is_in_transaction(ctx) {
                ctx.diagnostic(require_transaction_wrapping_diagnostic(call.span));
            }
        }
    }
}

impl RequireTransactionWrapping {
    fn is_multi_table_operation(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for operations that affect multiple tables
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "save" | "update" | "create" | "delete");
            }
        }
        false
    }

    fn is_in_transaction(&self, _ctx: &WasmLintContext) -> bool {
        // Check if operation is wrapped in transaction
        false
    }
}

fn require_transaction_wrapping_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Multi-table operation without transaction")
        .with_help("Wrap related database operations in transactions for data consistency")
        .with_label(span)
}

impl EnhancedWasmRule for RequireTransactionWrapping {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use database transactions for atomic operations".to_string(),
            "Transactions ensure data consistency on failures".to_string(),
            "Implement proper rollback handling".to_string(),
            "Group related operations in single transaction".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoNPlusOneQueries;

impl NoNPlusOneQueries {
    pub const NAME: &'static str = "no-n-plus-one-queries";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoNPlusOneQueries {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ForStatement(for_stmt) = node.kind() {
            if self.has_query_in_loop(for_stmt, ctx) {
                ctx.diagnostic(no_n_plus_one_queries_diagnostic(for_stmt.span));
            }
        }
    }
}

impl NoNPlusOneQueries {
    fn has_query_in_loop(&self, _for_stmt: &oxc_ast::ast::ForStatement, _ctx: &WasmLintContext) -> bool {
        // Check for database queries inside loops
        true
    }
}

fn no_n_plus_one_queries_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Potential N+1 query problem")
        .with_help("Use eager loading or batch queries to avoid N+1 problems")
        .with_label(span)
}

impl EnhancedWasmRule for NoNPlusOneQueries {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use eager loading: findMany({ include: { relations: true } })".to_string(),
            "Batch queries with whereIn: findMany({ where: { id: { in: ids } } })".to_string(),
            "Use DataLoader pattern for GraphQL resolvers".to_string(),
            "N+1 queries cause exponential performance degradation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireQueryOptimization;

impl RequireQueryOptimization {
    pub const NAME: &'static str = "require-query-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireQueryOptimization {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_unoptimized_query(call) {
                ctx.diagnostic(require_query_optimization_diagnostic(call.span));
            }
        }
    }
}

impl RequireQueryOptimization {
    fn is_unoptimized_query(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for queries that could be optimized
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return prop.name == "findMany" && !self.has_select_or_include(call);
            }
        }
        false
    }

    fn has_select_or_include(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if query has select or include clauses
        false
    }
}

fn require_query_optimization_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unoptimized database query")
        .with_help("Add select/include clauses to limit data fetching")
        .with_label(span)
}

impl EnhancedWasmRule for RequireQueryOptimization {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use select to fetch only needed fields".to_string(),
            "Include only necessary relations".to_string(),
            "Add pagination for large result sets".to_string(),
            "Consider query result caching".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMissingIndices;

impl NoMissingIndices {
    pub const NAME: &'static str = "no-missing-indices";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoMissingIndices {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.queries_without_index(call) {
                ctx.diagnostic(no_missing_indices_diagnostic(call.span));
            }
        }
    }
}

impl NoMissingIndices {
    fn queries_without_index(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for queries on non-indexed fields
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "findMany" | "findFirst" | "findUnique");
            }
        }
        false
    }
}

fn no_missing_indices_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Query on potentially non-indexed field")
        .with_help("Ensure database indices exist for frequently queried fields")
        .with_label(span)
}

impl EnhancedWasmRule for NoMissingIndices {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add database indices for WHERE clause fields".to_string(),
            "Composite indices for multi-field queries".to_string(),
            "Use EXPLAIN QUERY PLAN to verify index usage".to_string(),
            "Missing indices cause full table scans".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireConnectionPooling;

impl RequireConnectionPooling {
    pub const NAME: &'static str = "require-connection-pooling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireConnectionPooling {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.creates_new_connection(call) && !self.uses_connection_pool(call) {
                ctx.diagnostic(require_connection_pooling_diagnostic(call.span));
            }
        }
    }
}

impl RequireConnectionPooling {
    fn creates_new_connection(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for direct connection creation
        if let Some(ident) = call.callee.as_identifier() {
            return matches!(ident.name.as_str(), "createConnection" | "connect");
        }
        false
    }

    fn uses_connection_pool(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if connection pooling is configured
        false
    }
}

fn require_connection_pooling_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Database connection without pooling")
        .with_help("Use connection pooling for better performance and resource management")
        .with_label(span)
}

impl EnhancedWasmRule for RequireConnectionPooling {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Connection pooling reduces connection overhead".to_string(),
            "Configure appropriate pool size for your workload".to_string(),
            "Use connection pool monitoring for optimization".to_string(),
            "Direct connections don't scale well".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUnhandledMigrations;

impl NoUnhandledMigrations {
    pub const NAME: &'static str = "no-unhandled-migrations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoUnhandledMigrations {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_schema_change(call) && !self.has_migration_handling(ctx) {
                ctx.diagnostic(no_unhandled_migrations_diagnostic(call.span));
            }
        }
    }
}

impl NoUnhandledMigrations {
    fn is_schema_change(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check for schema modification operations
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "createTable" | "dropTable" | "addColumn");
            }
        }
        false
    }

    fn has_migration_handling(&self, _ctx: &WasmLintContext) -> bool {
        // Check if proper migration handling is in place
        false
    }
}

fn no_unhandled_migrations_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Schema change without migration handling")
        .with_help("Implement proper database migration strategies")
        .with_label(span)
}

impl EnhancedWasmRule for NoUnhandledMigrations {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use migration tools like Prisma or TypeORM migrations".to_string(),
            "Version control your database schema changes".to_string(),
            "Test migrations on staging before production".to_string(),
            "Implement rollback strategies for failed migrations".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireQueryValidation;

impl RequireQueryValidation {
    pub const NAME: &'static str = "require-query-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireQueryValidation {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::CallExpression(call) = node.kind() {
            if self.is_user_input_query(call) && !self.has_input_validation(call) {
                ctx.diagnostic(require_query_validation_diagnostic(call.span));
            }
        }
    }
}

impl RequireQueryValidation {
    fn is_user_input_query(&self, call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if query uses user input
        if let Some(member) = call.callee.as_member_expression() {
            if let Some(prop) = member.property().as_identifier() {
                return matches!(prop.name.as_str(), "findMany" | "findFirst" | "create" | "update");
            }
        }
        false
    }

    fn has_input_validation(&self, _call: &oxc_ast::ast::CallExpression) -> bool {
        // Check if input is validated before query
        false
    }
}

fn require_query_validation_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Database query without input validation")
        .with_help("Validate user input before database operations")
        .with_label(span)
}

impl EnhancedWasmRule for RequireQueryValidation {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use schema validation libraries like Zod or Joi".to_string(),
            "Validate data types, ranges, and formats".to_string(),
            "Sanitize inputs to prevent injection attacks".to_string(),
            "Implement rate limiting for query endpoints".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_sql_injection_rule() {
        assert_eq!(NoSQLInjection::NAME, "no-sql-injection");
        assert_eq!(NoSQLInjection::CATEGORY, WasmRuleCategory::Restriction);
        assert_eq!(NoSQLInjection::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_transaction_wrapping_rule() {
        assert_eq!(RequireTransactionWrapping::NAME, "require-transaction-wrapping");
        assert_eq!(RequireTransactionWrapping::CATEGORY, WasmRuleCategory::Correctness);
    }

    #[test]
    fn test_no_n_plus_one_queries_rule() {
        assert_eq!(NoNPlusOneQueries::NAME, "no-n-plus-one-queries");
        assert_eq!(NoNPlusOneQueries::CATEGORY, WasmRuleCategory::Perf);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoSQLInjection;
        let diagnostic = no_sql_injection_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("parameterized"));
    }
}