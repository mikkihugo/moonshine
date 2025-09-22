//! Database Optimization Rules
//!
//! Advanced database optimization, ORM best practices, and query performance rules.
//! Focuses on SQL injection prevention, query optimization, connection management, and ORM patterns.

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

/// Require query optimization and indexing strategies
pub struct RequireQueryOptimization;

impl RequireQueryOptimization {
    pub const NAME: &'static str = "require-query-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireQueryOptimization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for N+1 query patterns
        if code.contains("forEach") && code.contains("await") && code.contains("findOne") {
            diagnostics.push(create_n_plus_one_diagnostic());
        }

        // Check for missing indexes
        if code.contains("WHERE") && code.contains("LIKE '%") {
            diagnostics.push(create_full_scan_diagnostic());
        }

        // Check for inefficient joins
        if code.contains("SELECT *") && code.contains("JOIN") {
            diagnostics.push(create_inefficient_join_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireQueryOptimization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "query_optimization".to_string(),
            confidence: 0.91,
            description: "Optimize database queries: use proper indexing, avoid N+1 queries with eager loading, select only needed columns, optimize WHERE clauses.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_n_plus_one_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireQueryOptimization::NAME.to_string(),
        message: "Potential N+1 query detected. Use eager loading or batch queries".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_full_scan_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireQueryOptimization::NAME.to_string(),
        message: "Leading wildcard LIKE causes full table scan. Consider full-text search".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_inefficient_join_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireQueryOptimization::NAME.to_string(),
        message: "SELECT * with JOIN transfers unnecessary data. Select specific columns".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require proper ORM relationship configuration
pub struct RequireOptimalOrmRelations;

impl RequireOptimalOrmRelations {
    pub const NAME: &'static str = "require-optimal-orm-relations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireOptimalOrmRelations {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing eager loading
        if code.contains("@ManyToOne") && !code.contains("fetch = FetchType.LAZY") {
            diagnostics.push(create_eager_loading_diagnostic());
        }

        // Check for missing cascade settings
        if code.contains("@OneToMany") && !code.contains("cascade") {
            diagnostics.push(create_cascade_diagnostic());
        }

        // Check for bidirectional relations without mappedBy
        if code.contains("@OneToMany") && !code.contains("mappedBy") {
            diagnostics.push(create_mapped_by_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireOptimalOrmRelations {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "orm_relations".to_string(),
            confidence: 0.89,
            description: "Configure ORM relations properly: use lazy loading by default, set appropriate cascade types, define mappedBy for bidirectional relations to avoid extra tables.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_eager_loading_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireOptimalOrmRelations::NAME.to_string(),
        message: "Consider lazy loading for @ManyToOne to avoid unnecessary data fetching".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_cascade_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireOptimalOrmRelations::NAME.to_string(),
        message: "@OneToMany relation should specify cascade behavior".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_mapped_by_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireOptimalOrmRelations::NAME.to_string(),
        message: "Bidirectional @OneToMany should use mappedBy to avoid join table".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require connection pooling and management
pub struct RequireConnectionPooling;

impl RequireConnectionPooling {
    pub const NAME: &'static str = "require-connection-pooling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireConnectionPooling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for single connection usage
        if code.contains("createConnection") && !code.contains("pool") {
            diagnostics.push(create_pooling_diagnostic());
        }

        // Check for missing connection limits
        if code.contains("createPool") && !code.contains("connectionLimit") {
            diagnostics.push(create_connection_limit_diagnostic());
        }

        // Check for missing connection timeout
        if code.contains("mysql") && !code.contains("acquireTimeout") {
            diagnostics.push(create_timeout_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireConnectionPooling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "connection_pooling".to_string(),
            confidence: 0.92,
            description: "Implement connection pooling: set appropriate pool size, configure timeouts, monitor pool usage, implement proper connection cleanup.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_pooling_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireConnectionPooling::NAME.to_string(),
        message: "Use connection pooling instead of single connections for better performance".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_connection_limit_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireConnectionPooling::NAME.to_string(),
        message: "Connection pool should specify connectionLimit to prevent resource exhaustion".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_timeout_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireConnectionPooling::NAME.to_string(),
        message: "Database connection should configure acquireTimeout to prevent hanging".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require database transaction management
pub struct RequireTransactionManagement;

impl RequireTransactionManagement {
    pub const NAME: &'static str = "require-transaction-management";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTransactionManagement {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for multiple queries without transaction
        if code.contains("INSERT") && code.contains("UPDATE") && !code.contains("transaction") {
            diagnostics.push(create_transaction_diagnostic());
        }

        // Check for missing rollback handling
        if code.contains("beginTransaction") && !code.contains("rollback") {
            diagnostics.push(create_rollback_diagnostic());
        }

        // Check for long-running transactions
        if code.contains("transaction") && code.contains("setTimeout") {
            diagnostics.push(create_long_transaction_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTransactionManagement {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "transaction_management".to_string(),
            confidence: 0.90,
            description: "Implement proper transaction management: wrap related operations in transactions, handle rollbacks on errors, keep transactions short, use appropriate isolation levels.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_transaction_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTransactionManagement::NAME.to_string(),
        message: "Multiple database operations should be wrapped in a transaction".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_rollback_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTransactionManagement::NAME.to_string(),
        message: "Transaction should include rollback handling for error cases".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_long_transaction_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTransactionManagement::NAME.to_string(),
        message: "Avoid long-running transactions that can cause lock contention".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require database schema versioning and migrations
pub struct RequireSchemaVersioning;

impl RequireSchemaVersioning {
    pub const NAME: &'static str = "require-schema-versioning";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSchemaVersioning {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct schema changes
        if code.contains("ALTER TABLE") && !code.contains("migration") {
            diagnostics.push(create_schema_change_diagnostic());
        }

        // Check for missing down migrations
        if code.contains("up:") && !code.contains("down:") {
            diagnostics.push(create_down_migration_diagnostic());
        }

        // Check for destructive operations without safety
        if code.contains("DROP") && !code.contains("if exists") {
            diagnostics.push(create_destructive_operation_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSchemaVersioning {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "schema_versioning".to_string(),
            confidence: 0.87,
            description: "Use database migrations: version all schema changes, provide rollback migrations, test migrations in staging, avoid destructive operations without safeguards.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_schema_change_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSchemaVersioning::NAME.to_string(),
        message: "Schema changes should be managed through migrations, not direct ALTER statements".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_down_migration_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSchemaVersioning::NAME.to_string(),
        message: "Migration should include down method for rollback capability".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_destructive_operation_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSchemaVersioning::NAME.to_string(),
        message: "Destructive operations should include safety checks (IF EXISTS, backup verification)".to_string(),
        line: 0,
        column: 0,
        severity: "error".to_string(),
    }
}

/// Require database performance monitoring
pub struct RequirePerformanceMonitoring;

impl RequirePerformanceMonitoring {
    pub const NAME: &'static str = "require-performance-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing query logging
        if code.contains("query") && !code.contains("log") && !code.contains("console.time") {
            diagnostics.push(create_query_logging_diagnostic());
        }

        // Check for missing slow query detection
        if code.contains("database") && !code.contains("slow_query_log") {
            diagnostics.push(create_slow_query_diagnostic());
        }

        // Check for missing connection monitoring
        if code.contains("pool") && !code.contains("on('connection'") {
            diagnostics.push(create_connection_monitoring_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceMonitoring {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "performance_monitoring".to_string(),
            confidence: 0.85,
            description: "Implement database performance monitoring: log query execution times, monitor slow queries, track connection pool usage, set up alerts for performance degradation.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_query_logging_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Database queries should include performance logging for monitoring".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_slow_query_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Enable slow query logging to identify performance bottlenecks".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_connection_monitoring_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceMonitoring::NAME.to_string(),
        message: "Connection pool should include monitoring for connection events".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

/// Require data validation and constraints
pub struct RequireDataValidation;

impl RequireDataValidation {
    pub const NAME: &'static str = "require-data-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDataValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing input validation
        if code.contains("INSERT") && !code.contains("validate") {
            diagnostics.push(create_input_validation_diagnostic());
        }

        // Check for missing foreign key constraints
        if code.contains("@Column") && code.contains("userId") && !code.contains("@ManyToOne") {
            diagnostics.push(create_foreign_key_diagnostic());
        }

        // Check for missing unique constraints
        if code.contains("email") && !code.contains("unique") {
            diagnostics.push(create_unique_constraint_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDataValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "data_validation".to_string(),
            confidence: 0.88,
            description: "Implement comprehensive data validation: validate inputs before database operations, use database constraints, implement proper foreign key relationships, add unique constraints where needed.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_input_validation_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataValidation::NAME.to_string(),
        message: "Data should be validated before INSERT operations".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_foreign_key_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataValidation::NAME.to_string(),
        message: "Foreign key relationships should use proper ORM annotations".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_unique_constraint_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataValidation::NAME.to_string(),
        message: "Email fields should have unique constraints to prevent duplicates".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

/// Require backup and recovery strategies
pub struct RequireBackupStrategy;

impl RequireBackupStrategy {
    pub const NAME: &'static str = "require-backup-strategy";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBackupStrategy {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for missing backup configuration
        if code.contains("production") && code.contains("database") && !code.contains("backup") {
            diagnostics.push(create_backup_config_diagnostic());
        }

        // Check for missing point-in-time recovery
        if code.contains("mysql") && !code.contains("binlog") {
            diagnostics.push(create_binlog_diagnostic());
        }

        // Check for missing backup testing
        if code.contains("backup") && !code.contains("restore") && !code.contains("test") {
            diagnostics.push(create_backup_testing_diagnostic());
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBackupStrategy {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "backup_strategy".to_string(),
            confidence: 0.84,
            description: "Implement comprehensive backup strategy: automated regular backups, point-in-time recovery capability, backup testing and validation, offsite storage for disaster recovery.".to_string(),
            code_example: None,
        }).collect()
    }
}

fn create_backup_config_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBackupStrategy::NAME.to_string(),
        message: "Production database should have automated backup configuration".to_string(),
        line: 0,
        column: 0,
        severity: "warning".to_string(),
    }
}

fn create_binlog_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBackupStrategy::NAME.to_string(),
        message: "MySQL should enable binary logging for point-in-time recovery".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

fn create_backup_testing_diagnostic() -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBackupStrategy::NAME.to_string(),
        message: "Backup strategy should include regular restore testing".to_string(),
        line: 0,
        column: 0,
        severity: "info".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_query_optimization() {
        let rule = RequireQueryOptimization;

        // Test N+1 query pattern
        let code_violation = r#"
            users.forEach(async user => {
                const profile = await Profile.findOne({ userId: user.id });
            });
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test optimized query
        let code_compliant = r#"
            const profiles = await Profile.find({
                userId: { $in: users.map(u => u.id) }
            });
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_optimal_orm_relations() {
        let rule = RequireOptimalOrmRelations;

        // Test missing lazy loading
        let code_violation = r#"
            @ManyToOne
            @JoinColumn(name = "user_id")
            private User user;
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper lazy loading
        let code_compliant = r#"
            @ManyToOne(fetch = FetchType.LAZY)
            @JoinColumn(name = "user_id")
            private User user;
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_connection_pooling() {
        let rule = RequireConnectionPooling;

        // Test single connection
        let code_violation = "const connection = createConnection(config);";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test connection pooling
        let code_compliant = "const pool = createPool({ connectionLimit: 10 });";
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_transaction_management() {
        let rule = RequireTransactionManagement;

        // Test multiple operations without transaction
        let code_violation = r#"
            await User.INSERT({ name: 'John' });
            await Profile.UPDATE({ userId: 1 }, { bio: 'Updated' });
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper transaction usage
        let code_compliant = r#"
            await transaction(async (trx) => {
                await User.INSERT({ name: 'John' }).transacting(trx);
                await Profile.UPDATE({ userId: 1 }, { bio: 'Updated' }).transacting(trx);
            });
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_schema_versioning() {
        let rule = RequireSchemaVersioning;

        // Test direct schema change
        let code_violation = "ALTER TABLE users ADD COLUMN email VARCHAR(255);";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test migration approach
        let code_compliant = r#"
            exports.up = function(knex) {
                return knex.schema.table('users', function(table) {
                    table.string('email');
                });
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_performance_monitoring() {
        let rule = RequirePerformanceMonitoring;

        // Test query without logging
        let code_violation = "const users = await query('SELECT * FROM users');";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test query with logging
        let code_compliant = r#"
            console.time('users_query');
            const users = await query('SELECT * FROM users');
            console.timeEnd('users_query');
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_data_validation() {
        let rule = RequireDataValidation;

        // Test INSERT without validation
        let code_violation = "INSERT INTO users (name, email) VALUES (?, ?)";
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper validation
        let code_compliant = r#"
            const validated = await validate(userData);
            INSERT INTO users (name, email) VALUES (?, ?)
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_require_backup_strategy() {
        let rule = RequireBackupStrategy;

        // Test production without backup
        let code_violation = r#"
            const config = {
                production: {
                    database: 'prod_db'
                }
            };
        "#;
        let issues = rule.run(code_violation);
        assert!(!issues.is_empty());

        // Test proper backup configuration
        let code_compliant = r#"
            const config = {
                production: {
                    database: 'prod_db',
                    backup: {
                        schedule: '0 2 * * *',
                        retention: '30 days'
                    }
                }
            };
        "#;
        let issues = rule.run(code_compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement() {
        let rule = RequireQueryOptimization;
        let diagnostics = vec![WasmRuleDiagnostic {
            rule_name: "require-query-optimization".to_string(),
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