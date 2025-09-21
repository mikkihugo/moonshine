//! Cost-Aware AI Orchestration for Moon Shine
//!
//! Intelligent AI usage that balances quality with cost efficiency:
//! - Tool evaluation to determine AI necessity
//! - Smart pass planning based on code complexity
//! - Targeted AI usage where maximum value is achieved
//! - Cost budgets and ROI analysis for each AI call

use crate::error::{Error, Result};
use crate::config::MoonShineConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cost-aware AI orchestration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAwareConfig {
    /// Maximum AI spend per file (in credits/tokens)
    pub max_ai_budget_per_file: f64,
    /// Minimum issue severity to trigger AI analysis
    pub min_severity_for_ai: AISeverityThreshold,
    /// Performance budget for quick assessment
    pub quick_assessment_budget_ms: u64,
    /// ROI threshold for AI intervention
    pub min_roi_threshold: f64,
    /// Enable adaptive pass planning
    pub enable_adaptive_passes: bool,
}

impl Default for CostAwareConfig {
    fn default() -> Self {
        Self {
            max_ai_budget_per_file: 0.10, // 10 cents per file
            min_severity_for_ai: AISeverityThreshold::Warning,
            quick_assessment_budget_ms: 50, // 50ms for quick evaluation
            min_roi_threshold: 2.0, // 2x return on AI investment
            enable_adaptive_passes: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AISeverityThreshold {
    Info,
    Warning,
    Error,
    Critical,
}

/// Quick assessment result to guide AI strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAssessment {
    /// Code complexity score (0.0-1.0)
    pub complexity_score: f64,
    /// Number of issues found by static analysis
    pub static_issues_count: usize,
    /// Estimated fix difficulty (0.0-1.0)
    pub fix_difficulty: f64,
    /// Code quality score (0.0-1.0)
    pub quality_score: f64,
    /// Recommended AI strategy
    pub recommended_strategy: AIStrategy,
    /// Assessment execution time
    pub assessment_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIStrategy {
    /// Skip AI - issues are simple enough for automated fixes
    SkipAI { reason: String },
    /// Light AI - quick suggestions for specific issues
    LightAI {
        target_issues: Vec<String>,
        budget_estimate: f64,
    },
    /// Standard AI - comprehensive analysis and fixes
    StandardAI {
        passes: usize,
        budget_estimate: f64,
    },
    /// Heavy AI - complex refactoring and architectural improvements
    HeavyAI {
        passes: usize,
        specialized_models: Vec<String>,
        budget_estimate: f64,
    },
}

/// AI usage tracking and cost management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICostTracker {
    pub total_tokens_used: u64,
    pub total_cost: f64,
    pub calls_by_provider: HashMap<String, ProviderUsage>,
    pub roi_metrics: ROIMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub calls_count: u32,
    pub tokens_used: u64,
    pub cost: f64,
    pub average_response_time_ms: u64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROIMetrics {
    pub issues_fixed_by_ai: u32,
    pub manual_fix_time_saved_hours: f64,
    pub estimated_value_generated: f64,
    pub roi_ratio: f64, // value_generated / cost
}

/// Main cost-aware AI orchestrator
pub struct CostAwareAIOrchestrator {
    config: CostAwareConfig,
    cost_tracker: AICostTracker,
    assessment_cache: HashMap<String, QuickAssessment>,
}

impl CostAwareAIOrchestrator {
    pub fn new(config: CostAwareConfig) -> Self {
        Self {
            config,
            cost_tracker: AICostTracker {
                total_tokens_used: 0,
                total_cost: 0.0,
                calls_by_provider: HashMap::new(),
                roi_metrics: ROIMetrics {
                    issues_fixed_by_ai: 0,
                    manual_fix_time_saved_hours: 0.0,
                    estimated_value_generated: 0.0,
                    roi_ratio: 0.0,
                },
            },
            assessment_cache: HashMap::new(),
        }
    }

    /// Perform quick assessment to determine AI strategy
    pub async fn quick_assessment(&mut self,
        file_path: &str,
        source_code: &str
    ) -> Result<QuickAssessment> {
        let start_time = Instant::now();

        // Check cache first
        let cache_key = format!("{}:{}", file_path,
            format!("{:x}", md5::compute(source_code.as_bytes())));

        if let Some(cached) = self.assessment_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Quick static analysis (no AI)
        let complexity_score = self.calculate_complexity_score(source_code);
        let static_issues = self.run_quick_static_analysis(source_code);
        let fix_difficulty = self.estimate_fix_difficulty(&static_issues);
        let quality_score = self.calculate_quality_score(source_code, &static_issues);

        // Determine AI strategy based on assessment
        let recommended_strategy = self.determine_ai_strategy(
            complexity_score,
            static_issues.len(),
            fix_difficulty,
            quality_score,
        );

        let assessment = QuickAssessment {
            complexity_score,
            static_issues_count: static_issues.len(),
            fix_difficulty,
            quality_score,
            recommended_strategy,
            assessment_time_ms: start_time.elapsed().as_millis() as u64,
        };

        // Cache the assessment
        self.assessment_cache.insert(cache_key, assessment.clone());

        Ok(assessment)
    }


    /// Calculate code complexity score
    fn calculate_complexity_score(&self, source_code: &str) -> f64 {
        let lines = source_code.lines().count();
        let cyclomatic_complexity = self.estimate_cyclomatic_complexity(source_code);
        let nesting_depth = self.calculate_max_nesting_depth(source_code);

        // Normalize to 0.0-1.0 scale
        let line_score = (lines as f64 / 1000.0).min(1.0);
        let complexity_score = (cyclomatic_complexity as f64 / 20.0).min(1.0);
        let nesting_score = (nesting_depth as f64 / 10.0).min(1.0);

        (line_score + complexity_score + nesting_score) / 3.0
    }

    /// Run quick static analysis (regex-based for speed)
    fn run_quick_static_analysis(&self, source_code: &str) -> Vec<StaticIssue> {
        let mut issues = Vec::new();

        // Quick regex-based checks for common issues
        if source_code.contains("console.log") {
            issues.push(StaticIssue {
                rule_id: "no-console".to_string(),
                severity: IssueSeverity::Warning,
                line: 0, // Simplified for quick assessment
                message: "Console statement found".to_string(),
                fix_complexity: FixComplexity::Easy,
            });
        }

        if source_code.contains("any") && source_code.contains(":") {
            issues.push(StaticIssue {
                rule_id: "no-any".to_string(),
                severity: IssueSeverity::Warning,
                line: 0,
                message: "Any type usage detected".to_string(),
                fix_complexity: FixComplexity::Medium,
            });
        }

        if source_code.contains("eval(") {
            issues.push(StaticIssue {
                rule_id: "no-eval".to_string(),
                severity: IssueSeverity::Error,
                line: 0,
                message: "Eval usage detected".to_string(),
                fix_complexity: FixComplexity::Hard,
            });
        }

        issues
    }

    /// Estimate fix difficulty based on issues
    fn estimate_fix_difficulty(&self, issues: &[StaticIssue]) -> f64 {
        if issues.is_empty() {
            return 0.0;
        }

        let difficulty_sum: f64 = issues.iter().map(|issue| {
            match issue.fix_complexity {
                FixComplexity::Easy => 0.1,
                FixComplexity::Medium => 0.5,
                FixComplexity::Hard => 1.0,
            }
        }).sum();

        (difficulty_sum / issues.len() as f64).min(1.0)
    }

    /// Calculate quality score
    fn calculate_quality_score(&self, source_code: &str, issues: &[StaticIssue]) -> f64 {
        let lines = source_code.lines().count() as f64;
        let issue_density = issues.len() as f64 / lines.max(1.0);

        // Higher issues per line = lower quality
        (1.0 - (issue_density * 10.0)).max(0.0)
    }

    /// Determine AI strategy based on assessment metrics
    fn determine_ai_strategy(&self,
        complexity: f64,
        issue_count: usize,
        fix_difficulty: f64,
        quality: f64
    ) -> AIStrategy {
        // Skip AI for high-quality, simple code
        if quality > 0.9 && complexity < 0.3 && issue_count < 3 {
            return AIStrategy::SkipAI {
                reason: "High quality code with minimal issues".to_string(),
            };
        }

        // Light AI for simple fixes
        if fix_difficulty < 0.3 && issue_count < 10 {
            return AIStrategy::LightAI {
                target_issues: vec!["style".to_string(), "simple-fixes".to_string()],
                budget_estimate: 0.02, // 2 cents
            };
        }

        // Heavy AI for complex, low-quality code
        if complexity > 0.7 || quality < 0.4 || fix_difficulty > 0.7 {
            return AIStrategy::HeavyAI {
                passes: 3,
                specialized_models: vec![
                    "claude".to_string(),
                    "codex".to_string(),
                    "gemini".to_string(),
                ],
                budget_estimate: 0.08, // 8 cents
            };
        }

        // Standard AI for everything else
        AIStrategy::StandardAI {
            passes: 2,
            budget_estimate: 0.05, // 5 cents
        }
    }

    /// Estimate cyclomatic complexity (simplified)
    fn estimate_cyclomatic_complexity(&self, source_code: &str) -> usize {
        let control_flow_keywords = ["if", "else", "while", "for", "switch", "case", "catch", "&&", "||"];
        let mut complexity = 1; // Base complexity

        for keyword in &control_flow_keywords {
            complexity += source_code.matches(keyword).count();
        }

        complexity
    }

    /// Calculate maximum nesting depth
    fn calculate_max_nesting_depth(&self, source_code: &str) -> usize {
        let mut max_depth = 0;
        let mut current_depth = 0;

        for char in source_code.chars() {
            match char {
                '{' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                },
                '}' => {
                    current_depth = current_depth.saturating_sub(1);
                },
                _ => {}
            }
        }

        max_depth
    }

    /// Track AI usage and cost
    pub fn track_ai_usage(&mut self, provider: &str, tokens: u64, cost: f64, success: bool) {
        self.cost_tracker.total_tokens_used += tokens;
        self.cost_tracker.total_cost += cost;

        let usage = self.cost_tracker.calls_by_provider
            .entry(provider.to_string())
            .or_insert(ProviderUsage {
                calls_count: 0,
                tokens_used: 0,
                cost: 0.0,
                average_response_time_ms: 0,
                success_rate: 0.0,
            });

        usage.calls_count += 1;
        usage.tokens_used += tokens;
        usage.cost += cost;

        // Update success rate
        let old_success_count = (usage.success_rate * (usage.calls_count - 1) as f64) as u32;
        let new_success_count = old_success_count + if success { 1 } else { 0 };
        usage.success_rate = new_success_count as f64 / usage.calls_count as f64;
    }

    /// Get cost summary
    pub fn get_cost_summary(&self) -> String {
        format!(
            "AI Cost Summary: ${:.3} total, {} tokens, ROI: {:.2}x",
            self.cost_tracker.total_cost,
            self.cost_tracker.total_tokens_used,
            self.cost_tracker.roi_metrics.roi_ratio
        )
    }
}

#[derive(Debug, Clone)]
struct StaticIssue {
    rule_id: String,
    severity: IssueSeverity,
    line: usize,
    message: String,
    fix_complexity: FixComplexity,
}

#[derive(Debug, Clone)]
enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
enum FixComplexity {
    Easy,    // Automated fix available
    Medium,  // Simple AI intervention needed
    Hard,    // Complex refactoring required
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_skip_ai_for_high_quality_code() {
        let mut orchestrator = CostAwareAIOrchestrator::new(CostAwareConfig::default());

        let high_quality_code = r#"
        export interface User {
            readonly id: string;
            readonly name: string;
            readonly email: string;
        }

        export function validateUser(user: User): boolean {
            return user.id.length > 0 &&
                   user.name.length > 0 &&
                   user.email.includes('@');
        }
        "#;

        let assessment = orchestrator.quick_assessment("test.ts", high_quality_code).await.unwrap();

        assert!(matches!(assessment.recommended_strategy, AIStrategy::SkipAI { .. }));
        assert!(assessment.quality_score > 0.8);
    }

    #[tokio::test]
    async fn test_heavy_ai_for_complex_code() {
        let mut orchestrator = CostAwareAIOrchestrator::new(CostAwareConfig::default());

        let complex_code = r#"
        function processData(data: any): any {
            if (data) {
                if (data.items) {
                    for (let i = 0; i < data.items.length; i++) {
                        if (data.items[i]) {
                            if (data.items[i].active) {
                                console.log(data.items[i]);
                                eval(data.items[i].script);
                                if (data.items[i].children) {
                                    for (let j = 0; j < data.items[i].children.length; j++) {
                                        // Deep nesting continues...
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "#;

        let assessment = orchestrator.quick_assessment("complex.ts", complex_code).await.unwrap();

        assert!(matches!(assessment.recommended_strategy, AIStrategy::HeavyAI { .. }));
        assert!(assessment.complexity_score > 0.6);
        assert!(assessment.fix_difficulty > 0.5);
    }
}