use crate::oxc_adapter::ai_behavioral::AnalysisContext;
use crate::oxc_adapter::{AiBehavioralAnalyzer, MultiEngineAnalyzer, MultiEngineConfig, OxcAdapter};
use crate::rule_types::RuleMetadata;
use crate::rulebase::RuleImplementation;
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use futures::executor::block_on;
use oxc_ast::ast::Program;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Modern execution context for OXC + AI analysis
#[derive(Clone)]
pub struct RuleExecutionContext<'a> {
    pub code: &'a str,
    pub file_path: &'a str,
    pub source_type: SourceType,
    pub program: Option<&'a Program<'a>>,
}

/// Execution summary returned after running the rule engine.
#[derive(Default)]
pub struct RuleExecutionOutcome {
    pub diagnostics: Vec<LintDiagnostic>,
    pub evaluated_rules: usize,
    pub executed_rules: usize,
    pub skipped_rules: usize,
    pub elapsed: Duration,
}

/// Modern rule executor using OXC + AI multi-engine analysis
pub struct RuleExecutor {
    multi_engine_analyzer: MultiEngineAnalyzer,
    oxc_adapter: OxcAdapter,
}

impl RuleExecutor {
    pub fn new() -> Self {
        let config = MultiEngineConfig::default();
        Self {
            multi_engine_analyzer: MultiEngineAnalyzer::with_config(config),
            oxc_adapter: OxcAdapter::new(),
        }
    }

    /// Execute rules using modern OXC + AI analysis
    pub async fn evaluate_async<'a>(&mut self, _rules: &[RuleMetadata], ctx: &RuleExecutionContext<'a>) -> RuleExecutionOutcome {
        let start = Instant::now();

        // Run multi-engine OXC + AI analysis
        let analysis_result = self.multi_engine_analyzer.analyze_code(ctx.code, ctx.file_path).await;

        match analysis_result {
            Ok(result) => RuleExecutionOutcome {
                diagnostics: result.diagnostics,
                evaluated_rules: result.stats.oxc_rules_executed + result.stats.ai_patterns_checked,
                executed_rules: result.stats.oxc_rules_executed + result.stats.ai_patterns_checked,
                skipped_rules: 0,
                elapsed: start.elapsed(),
            },
            Err(e) => {
                log::error!("Analysis failed for {}: {}", ctx.file_path, e);
                RuleExecutionOutcome {
                    diagnostics: vec![],
                    evaluated_rules: 0,
                    executed_rules: 0,
                    skipped_rules: 0,
                    elapsed: start.elapsed(),
                }
            }
        }
    }

    /// Synchronous fallback for compatibility
    pub fn evaluate<'a>(&self, rules: &[RuleMetadata], ctx: &RuleExecutionContext<'a>) -> RuleExecutionOutcome {
        let start = Instant::now();

        // Run OXC static analysis
        let mut diagnostics = Vec::new();
        let mut executed_rules = 0;

        for rule in rules {
            match &rule.implementation {
                RuleImplementation::OxcStatic { rule_name } => {
                    // Execute OXC static rule
                    if let Ok(rule_diagnostics) = self.execute_oxc_rule(rule_name, ctx.code, ctx.file_path) {
                        diagnostics.extend(rule_diagnostics);
                        executed_rules += 1;
                    }
                }
                RuleImplementation::AiBehavioral { pattern_type } => {
                    // Execute AI behavioral rule
                    match self.execute_ai_rule(pattern_type, ctx.code, ctx.file_path, ctx.source_type) {
                        Ok(ai_diagnostics) => {
                            diagnostics.extend(ai_diagnostics);
                            executed_rules += 1;
                        }
                        Err(error) => {
                            log::warn!("AI behavioral rule '{}' failed for {}: {}", pattern_type, ctx.file_path, error);
                        }
                    }
                }
                RuleImplementation::Hybrid { oxc_rule, ai_pattern } => {
                    // Execute hybrid rule
                    match self.execute_hybrid_rule(oxc_rule, ai_pattern, ctx.code, ctx.file_path, ctx.source_type) {
                        Ok(hybrid_diagnostics) => {
                            diagnostics.extend(hybrid_diagnostics);
                            executed_rules += 1;
                        }
                        Err(error) => {
                            log::warn!("Hybrid rule '{}' failed for {}: {}", ai_pattern, ctx.file_path, error);
                        }
                    }
                }
                _ => {
                    // Skip unsupported rule types
                    continue;
                }
            }
        }

        RuleExecutionOutcome {
            diagnostics,
            evaluated_rules: rules.len(),
            executed_rules,
            skipped_rules: rules.len() - executed_rules,
            elapsed: start.elapsed(),
        }
    }

    /// Execute a single OXC static rule
    fn execute_oxc_rule(&self, rule_name: &str, code: &str, file_path: &str) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        // Parse the code using OXC
        let allocator = oxc_allocator::Allocator::default();
        let source_type = SourceType::from_path(file_path)?;
        let ret = oxc_parser::Parser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            // Convert parse errors to diagnostics
            return Ok(ret
                .errors
                .iter()
                .map(|err| {
                    let (line, column, end_line, end_column) = if let Some(labels) = &err.labels {
                        if let Some(label) = labels.first() {
                            // Extract line/column info from the span
                            (1, 1, 1, 1) // Default values for now
                        } else {
                            (1, 1, 1, 1)
                        }
                    } else {
                        (1, 1, 1, 1)
                    };

                    LintDiagnostic {
                        rule_name: "parse-error".to_string(),
                        message: err.message.to_string(),
                        file_path: file_path.to_string(),
                        line,
                        column,
                        end_line,
                        end_column,
                        severity: DiagnosticSeverity::Error,
                        fix_available: false,
                        suggested_fix: None,
                    }
                })
                .collect());
        }

        // Apply specific OXC rule
        let mut diagnostics = Vec::new();
        match rule_name {
            "no-unused-vars" => {
                diagnostics.extend(self.check_unused_variables(&ret.program));
            }
            "no-console" => {
                diagnostics.extend(self.check_console_usage(&ret.program));
            }
            "prefer-const" => {
                diagnostics.extend(self.check_prefer_const(&ret.program));
            }
            _ => {
                // Generic rule execution
                diagnostics.extend(self.execute_generic_rule(rule_name, &ret.program));
            }
        }

        Ok(diagnostics)
    }

    /// Execute AI behavioral rule
    fn execute_ai_rule(
        &self,
        pattern_type: &str,
        code: &str,
        file_path: &str,
        source_type: SourceType,
    ) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        let allocator = oxc_allocator::Allocator::default();
        let parse_result = oxc_parser::Parser::new(&allocator, code, source_type).parse();

        if !parse_result.errors.is_empty() {
            return Ok(parse_result
                .errors
                .iter()
                .map(|err| LintDiagnostic {
                    rule_name: format!("ai-behavioral-{}", pattern_type),
                    message: err.message.to_string(),
                    file_path: file_path.to_string(),
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 1,
                    severity: DiagnosticSeverity::Error,
                    fix_available: false,
                    suggested_fix: None,
                })
                .collect());
        }

        let analyzer = AiBehavioralAnalyzer::new();
        let context = AnalysisContext {
            file_path: file_path.to_string(),
            file_type: source_type,
            project_context: None,
            dependencies: Vec::new(),
        };

        block_on(analyzer.analyze_behavioral_patterns(code, &parse_result.program, &context))
    }

    /// Execute hybrid rule (OXC + AI)
    fn execute_hybrid_rule(
        &self,
        oxc_rule: &str,
        ai_pattern: &str,
        code: &str,
        file_path: &str,
        source_type: SourceType,
    ) -> Result<Vec<LintDiagnostic>, Box<dyn std::error::Error>> {
        let mut diagnostics = Vec::new();

        // Run OXC analysis first
        if let Ok(oxc_diagnostics) = self.execute_oxc_rule(oxc_rule, code, file_path) {
            diagnostics.extend(oxc_diagnostics);
        }

        // Then run AI analysis
        if let Ok(ai_diagnostics) = self.execute_ai_rule(ai_pattern, code, file_path, source_type) {
            diagnostics.extend(ai_diagnostics);
        }

        Ok(diagnostics)
    }

    /// Check for unused variables
    fn check_unused_variables(&self, program: &Program) -> Vec<LintDiagnostic> {
        let diagnostics = Vec::new();
        // Implementation would traverse AST and find unused variables
        // This is a simplified version
        diagnostics
    }

    /// Check for console usage
    fn check_console_usage(&self, program: &Program) -> Vec<LintDiagnostic> {
        let diagnostics = Vec::new();
        // Implementation would traverse AST and find console.* calls
        // This is a simplified version
        diagnostics
    }

    /// Check for prefer const
    fn check_prefer_const(&self, _program: &Program) -> Vec<LintDiagnostic> {
        let diagnostics = Vec::new();
        // Implementation would traverse AST and find let declarations that could be const
        // This is a simplified version
        diagnostics
    }

    /// Execute generic rule
    fn execute_generic_rule(&self, _rule_name: &str, _program: &Program) -> Vec<LintDiagnostic> {
        let diagnostics = Vec::new();
        // Generic rule execution logic
        diagnostics
    }

    /// Update multi-engine analyzer configuration
    pub fn update_config(&mut self, config: MultiEngineConfig) {
        self.multi_engine_analyzer.update_config(config);
    }
}

/// Lightweight execution plan used by higher-level schedulers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub total_rules: usize,
    pub estimated_duration: Duration,
}

impl ExecutionPlan {
    pub fn new(rule_count: usize) -> Self {
        Self {
            total_rules: rule_count,
            estimated_duration: Duration::from_millis(rule_count as u64 * 2),
        }
    }
}

// Note: Legacy OXC visitor-based rule implementations removed
// Modern rule execution is handled by BiomeAdapter and UnifiedAnalyzer
// which provide both static analysis (via Biome) and AI behavioral patterns
