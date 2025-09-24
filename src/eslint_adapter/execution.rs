//! # ESLint Rule Execution Engine
//!
//! This module handles the execution of ESLint-compatible rules within our
//! OXC-based system. It provides both native Rust execution and fallback
//! to external ESLint when needed.

use super::context::ESLintRuleContext;
use super::rule_mapping::ESLintRuleMapper;
use super::visitor::{create_eslint_visitor, ESLintVisitorAdapter};
use super::{ESLintConfig, ESLintRuleInfo, ESLintRuleStrategy};
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// ESLint execution engine
pub struct ESLintExecutionEngine {
    mapper: ESLintRuleMapper,
}

impl ESLintExecutionEngine {
    pub fn new() -> Self {
        Self {
            mapper: ESLintRuleMapper::new(),
        }
    }

    /// Execute ESLint rules on the provided AST
    pub fn execute_rules<'a>(
        &self,
        config: &ESLintConfig,
        program: &Program<'a>,
        semantic: Option<&Semantic<'a>>,
        source_code: &'a str,
        file_path: &str,
    ) -> ESLintExecutionResult {
        let start_time = Instant::now();
        let mut diagnostics = Vec::new();
        let mut execution_stats = ESLintExecutionStats::default();

        // Convert ESLint config to rule infos
        let rule_infos = self.convert_config_to_rules(config);

        for rule_info in &rule_infos {
            let rule_start = Instant::now();

            match rule_info.strategy {
                ESLintRuleStrategy::Native => {
                    let rule_diagnostics = self.execute_native_rule(
                        rule_info,
                        program,
                        semantic,
                        source_code,
                        file_path,
                    );

                    diagnostics.extend(rule_diagnostics);
                    execution_stats.native_rules_executed += 1;
                }
                ESLintRuleStrategy::External => {
                    // TODO: Implement external ESLint execution
                    log::debug!("External ESLint execution not implemented for: {}", rule_info.eslint_id);
                    execution_stats.external_rules_skipped += 1;
                }
                ESLintRuleStrategy::Unavailable => {
                    execution_stats.unavailable_rules += 1;
                }
            }

            execution_stats.total_rule_time += rule_start.elapsed();
        }

        execution_stats.total_execution_time = start_time.elapsed();
        execution_stats.total_diagnostics = diagnostics.len();

        ESLintExecutionResult {
            diagnostics,
            stats: execution_stats,
            rules_processed: rule_infos.len(),
        }
    }

    /// Execute a native Rust implementation of an ESLint rule
    fn execute_native_rule<'a>(
        &self,
        rule_info: &ESLintRuleInfo,
        program: &Program<'a>,
        semantic: Option<&Semantic<'a>>,
        source_code: &'a str,
        file_path: &str,
    ) -> Vec<LintDiagnostic> {
        // Create ESLint-compatible context
        let context = ESLintRuleContext::new(
            source_code,
            file_path,
            program,
            semantic,
            rule_info.eslint_id.clone(),
            self.convert_severity(&rule_info.severity),
            vec![], // TODO: Pass rule options
        );

        // Get the visitor implementation for this rule
        if let Some(visitor) = create_eslint_visitor(&rule_info.eslint_id) {
            // Create adapter and execute
            let mut adapter = ESLintVisitorAdapter::new(visitor, context);
            adapter.visit_program(program);
            adapter.get_diagnostics()
        } else {
            log::warn!("No native implementation found for rule: {}", rule_info.eslint_id);
            Vec::new()
        }
    }

    /// Convert ESLint config to rule infos
    fn convert_config_to_rules(&self, config: &ESLintConfig) -> Vec<ESLintRuleInfo> {
        let mut rule_infos = Vec::new();

        for (rule_id, rule_level) in &config.rules {
            if let Some(mut rule_info) = self.mapper.map_eslint_rule(rule_id) {
                // Update severity based on config
                rule_info.severity = match rule_level {
                    super::ESLintRuleLevel::Off => continue, // Skip disabled rules
                    super::ESLintRuleLevel::Warn => crate::rule_registry::RuleSeverity::Warning,
                    super::ESLintRuleLevel::Error => crate::rule_registry::RuleSeverity::Error,
                };

                rule_infos.push(rule_info);
            } else {
                log::warn!("Unknown ESLint rule: {}", rule_id);
            }
        }

        rule_infos
    }

    /// Convert internal severity to diagnostic severity
    fn convert_severity(&self, severity: &crate::rule_registry::RuleSeverity) -> DiagnosticSeverity {
        match severity {
            crate::rule_registry::RuleSeverity::Error => DiagnosticSeverity::Error,
            crate::rule_registry::RuleSeverity::Warning => DiagnosticSeverity::Warning,
            crate::rule_registry::RuleSeverity::Info => DiagnosticSeverity::Info,
            crate::rule_registry::RuleSeverity::Hint => DiagnosticSeverity::Hint,
            crate::rule_registry::RuleSeverity::Custom(_) => DiagnosticSeverity::Warning,
        }
    }

    /// Get execution statistics
    pub fn get_supported_rules(&self) -> ESLintSupportInfo {
        let native_rules = self.mapper.get_native_rules();
        let external_rules = self.mapper.get_external_rules();
        let stats = self.mapper.get_mapping_stats();

        ESLintSupportInfo {
            total_supported_rules: stats.total_mapped_rules,
            native_rules: native_rules.clone(),
            external_rules: external_rules.clone(),
            native_count: native_rules.len(),
            external_count: external_rules.len(),
            autofix_capable_count: stats.rules_with_autofix,
        }
    }
}

impl Default for ESLintExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of ESLint rule execution
#[derive(Debug, Clone)]
pub struct ESLintExecutionResult {
    pub diagnostics: Vec<LintDiagnostic>,
    pub stats: ESLintExecutionStats,
    pub rules_processed: usize,
}

/// Statistics from ESLint rule execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ESLintExecutionStats {
    pub total_execution_time: std::time::Duration,
    pub total_rule_time: std::time::Duration,
    pub native_rules_executed: usize,
    pub external_rules_executed: usize,
    pub external_rules_skipped: usize,
    pub unavailable_rules: usize,
    pub total_diagnostics: usize,
}

/// Information about ESLint rule support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintSupportInfo {
    pub total_supported_rules: usize,
    pub native_rules: Vec<String>,
    pub external_rules: Vec<String>,
    pub native_count: usize,
    pub external_count: usize,
    pub autofix_capable_count: usize,
}

/// ESLint configuration builder for common presets
pub struct ESLintConfigBuilder {
    rules: HashMap<String, super::ESLintRuleLevel>,
    extends: Vec<String>,
    env: HashMap<String, bool>,
}

impl ESLintConfigBuilder {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            extends: Vec::new(),
            env: HashMap::new(),
        }
    }

    /// Add recommended ESLint rules
    pub fn with_recommended(mut self) -> Self {
        let recommended_rules = vec![
            ("no-console", super::ESLintRuleLevel::Warn),
            ("no-debugger", super::ESLintRuleLevel::Error),
            ("no-alert", super::ESLintRuleLevel::Warn),
            ("no-eval", super::ESLintRuleLevel::Error),
            ("no-empty", super::ESLintRuleLevel::Warn),
            ("no-unreachable", super::ESLintRuleLevel::Error),
            ("no-constant-condition", super::ESLintRuleLevel::Warn),
            ("no-dupe-keys", super::ESLintRuleLevel::Error),
            ("no-duplicate-case", super::ESLintRuleLevel::Error),
            ("curly", super::ESLintRuleLevel::Warn),
            ("eqeqeq", super::ESLintRuleLevel::Warn),
        ];

        for (rule_id, level) in recommended_rules {
            self.rules.insert(rule_id.to_string(), level);
        }

        self.extends.push("eslint:recommended".to_string());
        self
    }

    /// Add strict rules for production
    pub fn with_strict(mut self) -> Self {
        let strict_rules = vec![
            ("no-implied-eval", super::ESLintRuleLevel::Error),
            ("no-new-func", super::ESLintRuleLevel::Error),
            ("no-script-url", super::ESLintRuleLevel::Error),
            ("no-empty-function", super::ESLintRuleLevel::Warn),
        ];

        for (rule_id, level) in strict_rules {
            self.rules.insert(rule_id.to_string(), level);
        }

        self
    }

    /// Add a specific rule
    pub fn with_rule(mut self, rule_id: &str, level: super::ESLintRuleLevel) -> Self {
        self.rules.insert(rule_id.to_string(), level);
        self
    }

    /// Set an environment
    pub fn with_env(mut self, env: &str, enabled: bool) -> Self {
        self.env.insert(env.to_string(), enabled);
        self
    }

    /// Build the ESLint configuration
    pub fn build(self) -> ESLintConfig {
        ESLintConfig {
            rules: self.rules,
            env: if self.env.is_empty() { None } else { Some(self.env) },
            extends: if self.extends.is_empty() { None } else { Some(self.extends) },
            parser_options: None,
        }
    }
}

impl Default for ESLintConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for ESLint execution

/// Create a basic ESLint configuration for testing
pub fn create_test_config() -> ESLintConfig {
    ESLintConfigBuilder::new()
        .with_recommended()
        .with_env("browser", true)
        .with_env("node", true)
        .build()
}

/// Create a production-ready ESLint configuration
pub fn create_production_config() -> ESLintConfig {
    ESLintConfigBuilder::new()
        .with_recommended()
        .with_strict()
        .with_env("browser", true)
        .with_env("es2022", true)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{ParseOptions, Parser};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    #[test]
    fn test_execution_engine_creation() {
        let engine = ESLintExecutionEngine::new();
        let support_info = engine.get_supported_rules();

        assert!(support_info.total_supported_rules > 0);
        assert!(support_info.native_count > 0);
        assert!(!support_info.native_rules.is_empty());
    }

    #[test]
    fn test_rule_execution() {
        let engine = ESLintExecutionEngine::new();
        let config = create_test_config();

        let source = "console.log('test'); debugger; eval('code');";
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let semantic_return = SemanticBuilder::new()
            .build(&parser_result.program);

        let result = engine.execute_rules(
            &config,
            &parser_result.program,
            Some(&semantic_return.semantic),
            source,
            "test.js",
        );

        assert!(result.diagnostics.len() > 0);
        assert!(result.stats.native_rules_executed > 0);
        assert_eq!(result.stats.total_diagnostics, result.diagnostics.len());
    }

    #[test]
    fn test_config_builder() {
        let config = ESLintConfigBuilder::new()
            .with_recommended()
            .with_rule("no-console", super::super::ESLintRuleLevel::Error)
            .with_env("browser", true)
            .build();

        assert!(!config.rules.is_empty());
        assert!(config.env.is_some());
        assert_eq!(config.rules["no-console"], super::super::ESLintRuleLevel::Error);
    }

    #[test]
    fn test_preset_configs() {
        let test_config = create_test_config();
        let prod_config = create_production_config();

        assert!(!test_config.rules.is_empty());
        assert!(!prod_config.rules.is_empty());
        assert!(prod_config.rules.len() >= test_config.rules.len());
    }
}