use crate::config::MoonShineConfig;
use crate::error::Result;
use crate::rule_registry::{RuleRegistry, RuleRegistryStats};
use crate::rulebase::{ExecutionPlan, RuleExecutionContext, RuleExecutionOutcome, RuleExecutor};
use crate::types::{DiagnosticSeverity, FixableLintIssue, LintDiagnostic};
use oxc_allocator::Allocator;
use oxc_ast::ast::{Program, Statement};
use oxc_diagnostics::{OxcDiagnostic, Severity};
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeScriptCompilationResult {
    pub success: bool,
    pub syntax_errors: Vec<CompilationDiagnostic>,
    pub type_errors: Vec<CompilationDiagnostic>,
    pub warnings: Vec<CompilationDiagnostic>,
    pub generated_js: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodeLintingResult {
    pub errors: Vec<LintDiagnostic>,
    pub warnings: Vec<LintDiagnostic>,
    pub fixable_issues: Vec<FixableLintIssue>,
    pub auto_fixed_code: Option<String>,
    pub rules_applied: Vec<String>,
    pub execution: RuleExecutionReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodeFormattingResult {
    pub formatted_code: String,
    pub changed: bool,
    pub source_map: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentationAnalysisResult {
    pub coverage_percentage: f32,
    pub documented_items: Vec<DocumentedItem>,
    pub missing_documentation: Vec<MissingDocumentation>,
    pub documentation_errors: Vec<DocumentationError>,
    pub generated_docs: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationDiagnostic {
    pub message: String,
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub severity: DiagnosticSeverity,
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedItem {
    pub name: String,
    pub item_type: String,
    pub documentation: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentation {
    pub item_name: String,
    pub item_type: String,
    pub line: u32,
    pub column: u32,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationError {
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub severity: DiagnosticSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickAssessment {
    pub complexity_score: f64,
    pub estimated_issues: u32,
    pub ai_recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleExecutionReport {
    pub evaluated_rules: usize,
    pub executed_rules: usize,
    pub skipped_rules: usize,
    pub diagnostics_emitted: usize,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPipelineResult {
    pub compilation: TypeScriptCompilationResult,
    pub linting: CodeLintingResult,
    pub documentation: DocumentationAnalysisResult,
    pub formatting: CodeFormattingResult,
    pub rule_stats: RuleRegistryStats,
    pub execution_plan: ExecutionPlan,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct LintConfig {
    pub enable_fix: bool,
    pub import_plugin: bool,
    pub react_plugin: bool,
    pub jsx_a11y_plugin: bool,
    pub typescript_plugin: bool,
}

pub struct AnalysisPipeline {
    rule_registry: RuleRegistry,
    rule_executor: RuleExecutor,
}

impl AnalysisPipeline {
    pub fn new() -> Self {
        let registry = RuleRegistry::new().expect("embedded rulebase must load");
        let executor = RuleExecutor::new();
        Self {
            rule_registry: registry,
            rule_executor: executor,
        }
    }

    pub fn registry(&self) -> &RuleRegistry {
        &self.rule_registry
    }

    pub async fn assess_code_quickly(
        &self,
        code: &str,
        file_path: &str,
        budget: Duration,
        complexity_threshold: f64,
        enable_quick_static_analysis: bool,
    ) -> Result<QuickAssessment> {
        let start = Instant::now();
        let stats = self.rule_registry.get_statistics();

        let mut complexity = estimate_complexity_from_text(code);
        let mut estimated_branches = 0u32;

        if enable_quick_static_analysis && budget.as_millis() > 0 {
            let (branch_count, function_count) = quick_ast_metrics(code, file_path, budget.saturating_sub(start.elapsed()));
            estimated_branches = branch_count;
            complexity += (function_count as f64 * 0.02) + (branch_count as f64 * 0.03);
        }

        complexity = complexity.clamp(0.0, 1.0);
        let estimated_issues = ((stats.total_rules as f64) * (complexity.max(0.1) * 0.03)) as u32 + estimated_branches.min(20);
        let ai_recommended = complexity >= complexity_threshold || estimated_issues >= 12;

        Ok(QuickAssessment {
            complexity_score: complexity,
            estimated_issues,
            ai_recommended,
        })
    }

    pub async fn run(&self, code: &str, file_path: &str, _config: &MoonShineConfig) -> Result<AnalysisPipelineResult> {
        let start = Instant::now();
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(file_path).unwrap_or_else(|_| SourceType::ts());
        let parser_result = Parser::new(&allocator, code, source_type).with_options(ParseOptions::default()).parse();

        let syntax_errors = parser_result
            .errors
            .iter()
            .map(|err| convert_diagnostic(err, file_path, code))
            .collect::<Vec<_>>();

        let program: Program<'_> = parser_result.program;

        let semantic_return = SemanticBuilder::new().with_check_syntax_error(true).with_build_jsdoc(true).build(&program);

        let type_errors = semantic_return
            .errors
            .iter()
            .map(|err| convert_diagnostic(err, file_path, code))
            .collect::<Vec<_>>();

        let compilation = TypeScriptCompilationResult {
            success: syntax_errors.is_empty() && type_errors.is_empty(),
            syntax_errors,
            type_errors,
            warnings: Vec::new(),
            generated_js: None,
        };

        let enabled_rules = self.rule_registry.get_enabled_rules();
        let rule_context = RuleExecutionContext {
            code,
            file_path,
            program: &program,
            semantic: Some(&semantic_return.semantic),
        };
        let outcome = self.rule_executor.evaluate(&enabled_rules, &rule_context);

        let linting = build_linting_result(code, outcome);
        let documentation = analyze_documentation(code, &program);

        let formatting = CodeFormattingResult {
            formatted_code: code.to_string(),
            changed: false,
            source_map: None,
        };

        Ok(AnalysisPipelineResult {
            compilation,
            linting,
            documentation,
            formatting,
            rule_stats: self.rule_registry.get_statistics(),
            execution_plan: ExecutionPlan::new(enabled_rules.len()),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

fn build_linting_result(code: &str, outcome: RuleExecutionOutcome) -> CodeLintingResult {
    let mut rule_names = BTreeSet::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut fixable = Vec::new();

    for diag in &outcome.diagnostics {
        rule_names.insert(diag.rule_name.clone());
        if matches!(diag.severity, DiagnosticSeverity::Error) {
            errors.push(diag.clone());
        } else {
            warnings.push(diag.clone());
        }

        if diag.fix_available {
            fixable.push(FixableLintIssue {
                rule_name: diag.rule_name.clone(),
                description: diag.message.clone(),
                original_text: snippet_at_line(code, diag.line).unwrap_or_default(),
                fixed_text: String::new(),
                line: diag.line,
                column: diag.column,
            });
        }
    }

    CodeLintingResult {
        errors,
        warnings,
        fixable_issues: fixable,
        auto_fixed_code: None,
        rules_applied: rule_names.into_iter().collect(),
        execution: RuleExecutionReport {
            evaluated_rules: outcome.evaluated_rules,
            executed_rules: outcome.executed_rules,
            skipped_rules: outcome.skipped_rules,
            diagnostics_emitted: outcome.diagnostics.len(),
            elapsed_ms: outcome.elapsed.as_millis() as u64,
        },
    }
}

fn analyze_documentation(code: &str, program: &Program<'_>) -> DocumentationAnalysisResult {
    let mut documented = Vec::new();
    let mut missing = Vec::new();
    let mut total_items = 0u32;

    for stmt in &program.body {
        if let Statement::FunctionDeclaration(func) = stmt {
            total_items += 1;
            let name = func.id.as_ref().map(|ident| ident.name.to_string()).unwrap_or_else(|| "anonymous".to_string());
            if has_leading_docs(code, func.span) {
                documented.push(DocumentedItem {
                    name,
                    item_type: "function".to_string(),
                    documentation: extract_comment_block(code, func.span),
                    line: span_line(code, func.span),
                    column: span_column(code, func.span),
                });
            } else {
                missing.push(MissingDocumentation {
                    item_name: name,
                    item_type: "function".to_string(),
                    line: span_line(code, func.span),
                    column: span_column(code, func.span),
                    suggestion: "Add JSDoc describing parameters and return value".to_string(),
                });
            }
        }
    }

    let coverage_percentage = if total_items == 0 {
        100.0
    } else {
        (documented.len() as f32 / total_items as f32) * 100.0
    };

    DocumentationAnalysisResult {
        coverage_percentage,
        documented_items: documented,
        missing_documentation: missing,
        documentation_errors: Vec::new(),
        generated_docs: None,
    }
}

fn convert_diagnostic(error: &OxcDiagnostic, file_path: &str, code: &str) -> CompilationDiagnostic {
    let span = span_from_diagnostic(error);
    let inner = error.as_ref();
    let severity = match inner.severity {
        Severity::Error => DiagnosticSeverity::Error,
        Severity::Warning => DiagnosticSeverity::Warning,
        Severity::Advice => DiagnosticSeverity::Hint,
    };

    CompilationDiagnostic {
        message: error.to_string(),
        file_path: file_path.to_string(),
        line: span_line(code, span),
        column: span_column(code, span),
        severity,
        error_code: None,
    }
}

fn span_from_diagnostic(error: &OxcDiagnostic) -> Span {
    let inner = error.as_ref();
    if let Some(labels) = inner.labels.as_ref() {
        if let Some(label) = labels.first() {
            let start = label.offset() as u32;
            let len = label.len() as u32;
            return if len == 0 { Span::new(start, start) } else { Span::new(start, start + len) };
        }
    }
    Span::empty(0)
}

fn quick_ast_metrics(code: &str, file_path: &str, remaining_budget: Duration) -> (u32, u32) {
    if remaining_budget.is_zero() {
        return (0, 0);
    }

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(file_path).unwrap_or_else(|_| SourceType::ts());
    let parser = Parser::new(&allocator, code, source_type).with_options(ParseOptions::default()).parse();
    let program = parser.program;

    let mut branch_count = 0u32;
    let mut function_count = 0u32;

    for stmt in &program.body {
        match stmt {
            Statement::IfStatement(_)
            | Statement::SwitchStatement(_)
            | Statement::ForStatement(_)
            | Statement::WhileStatement(_)
            | Statement::ForOfStatement(_)
            | Statement::ForInStatement(_) => branch_count += 1,
            Statement::FunctionDeclaration(_) => function_count += 1,
            _ => {}
        }
    }

    (branch_count, function_count)
}

fn estimate_complexity_from_text(code: &str) -> f64 {
    let lines = code.lines().count() as f64;
    let keywords = ["if", "for", "while", "switch", "catch", "class", "return"];
    let keyword_hits = keywords.iter().map(|kw| code.matches(kw).count() as f64).sum::<f64>();
    ((lines / 200.0) + (keyword_hits / 500.0)).clamp(0.0, 1.0)
}

fn snippet_at_line(code: &str, line: u32) -> Option<String> {
    code.lines().nth(line.saturating_sub(1) as usize).map(|line_text| line_text.trim().to_string())
}

fn has_leading_docs(code: &str, span: Span) -> bool {
    let start = span.start.min(code.len() as u32) as usize;
    let prefix = &code[..start];
    let snippet = prefix.rsplit_once('\n').map(|(_, tail)| tail).unwrap_or(prefix);
    snippet.trim_end().ends_with("*/") || snippet.trim_start().starts_with("///")
}

fn extract_comment_block(code: &str, span: Span) -> String {
    let start = span.start.min(code.len() as u32) as usize;
    let prefix = &code[..start];
    let mut docs = Vec::new();
    for line in prefix.lines().rev().take(10) {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/**") || trimmed.starts_with("*") {
            docs.push(trimmed.trim_start_matches(['/', '*'].as_ref()).trim());
        } else if !docs.is_empty() {
            break;
        }
    }
    docs.reverse();
    docs.join("\n")
}

fn span_line(code: &str, span: Span) -> u32 {
    let start = span.start.min(code.len() as u32) as usize;
    code[..start].lines().count() as u32 + 1
}

fn span_column(code: &str, span: Span) -> u32 {
    let start = span.start.min(code.len() as u32) as usize;
    let prefix = &code[..start];
    prefix
        .rsplit_once('\n')
        .map(|(_, tail)| tail.chars().count() as u32 + 1)
        .unwrap_or(prefix.chars().count() as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn quick_assessment_works() {
        let pipeline = AnalysisPipeline::new();
        let code = "function demo(x) { if (x) { return x; } return 0; }";
        let assessment = pipeline
            .assess_code_quickly(code, "demo.ts", Duration::from_millis(20), 0.4, true)
            .await
            .unwrap();
        assert!(assessment.estimated_issues >= 1);
    }
}
