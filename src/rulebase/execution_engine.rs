use crate::oxc_adapter::{AiBehavioralAnalyzer, MultiEngineAnalyzer, MultiEngineConfig, OxcAdapter};
use crate::rule_types::{FixStatus, RuleMetadata, RuleSeverity};
use crate::rulebase::RuleImplementation;
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_ast::ast::Program;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub fn evaluate<'a>(&self, _rules: &[RuleMetadata], ctx: &RuleExecutionContext<'a>) -> RuleExecutionOutcome {
        let start = Instant::now();

        // Run only OXC static analysis (no AI for sync execution)
        let oxc_result = self.oxc_adapter.analyze_code(ctx.code, ctx.file_path);

        match oxc_result {
            Ok(result) => RuleExecutionOutcome {
                diagnostics: result.diagnostics,
                evaluated_rules: 1, // Simplified count
                executed_rules: 1,
                skipped_rules: 0,
                elapsed: start.elapsed(),
            },
            Err(e) => {
                log::error!("OXC analysis failed for {}: {}", ctx.file_path, e);
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
