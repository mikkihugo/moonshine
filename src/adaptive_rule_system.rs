//! # Adaptive Rule System
//!
//! Integration layer that combines pattern frequency tracking with custom rule generation
//! to create an adaptive linting system that learns from your codebase and generates
//! custom rules for frequently occurring patterns.
//!
//! @category ai-integration
//! @safe program
//! @complexity high
//! @since 2.1.0

use crate::config::{AdaptiveRuleSystemConfig, MoonShineConfig, PatternTrackingConfig, RuleGenerationConfig, StarcoderConfig};
use crate::custom_rule_generator::{CustomRuleGenerator, GeneratedRule, RuleGenerationConfiguration};
use crate::error::{Error, Result};
use crate::javascript_typescript_linter::LintIssue;
use crate::pattern_frequency_tracker::{PatternCluster, PatternFrequencyTracker, PatternTrackingConfiguration};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// Configuration for the adaptive rule system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRuleSystemConfiguration {
    /// Pattern tracking configuration
    pub pattern_tracking: PatternTrackingConfiguration,
    /// Rule generation configuration
    pub rule_generation: RuleGenerationConfiguration,
    /// Minimum pattern frequency for rule generation
    pub min_pattern_frequency_for_rules: usize,
    /// Maximum number of rules to generate per analysis cycle
    pub max_rules_per_cycle: usize,
    /// Whether to auto-apply generated rules
    pub auto_apply_generated_rules: bool,
    /// Directory to save generated rules
    pub output_directory: String,
}

impl Default for AdaptiveRuleSystemConfiguration {
    fn default() -> Self {
        Self {
            pattern_tracking: PatternTrackingConfiguration::default(),
            rule_generation: RuleGenerationConfiguration::default(),
            min_pattern_frequency_for_rules: 10,
            max_rules_per_cycle: 5,
            auto_apply_generated_rules: false,
            output_directory: "generated_rules".to_string(),
        }
    }
}

impl AdaptiveRuleSystemConfiguration {
    /// Create configuration from Moon PDK extension config
    pub fn from_moon_config(config: &MoonShineConfig) -> Self {
        let mut result = Self::default();

        if let Some(adaptive_config) = config.adaptive.as_ref() {
            // Update pattern tracking configuration
            if let Some(pattern_config) = &adaptive_config.pattern_tracking {
                if let Some(freq) = pattern_config.min_pattern_frequency {
                    result.min_pattern_frequency_for_rules = freq as usize;
                    result.pattern_tracking.minimum_occurrence_threshold = freq as usize;
                }
                if let Some(age) = pattern_config.pattern_max_age_days {
                    result.pattern_tracking.pattern_max_age_days = age;
                }
                if let Some(threshold) = pattern_config.clustering_similarity_threshold {
                    result.pattern_tracking.clustering_similarity_threshold = threshold as f64;
                }
                // Note: max_clusters field not available in current PatternTrackingConfiguration
            }

            // Update rule generation configuration
            if let Some(rule_config) = &adaptive_config.rule_generation {
                if let Some(provider) = &rule_config.ai_provider {
                    result.rule_generation.ai_provider = provider.clone();
                }
                if let Some(cluster_size) = rule_config.min_cluster_size_for_rules {
                    result.rule_generation.min_cluster_size_for_rules = cluster_size as usize;
                }
                if let Some(max_rules) = rule_config.max_rules_per_cluster {
                    result.rule_generation.max_rules_per_cluster = max_rules as usize;
                    result.max_rules_per_cycle = max_rules as usize;
                }
                if let Some(auto_activate) = rule_config.enable_auto_rule_activation {
                    result.auto_apply_generated_rules = auto_activate;
                    result.rule_generation.enable_auto_rule_activation = auto_activate;
                }
                if let Some(quality) = rule_config.rule_quality_threshold {
                    let quality = quality as f64;
                    result.rule_generation.quality_threshold = quality;
                    result.rule_generation.rule_quality_threshold = quality;
                }
            }

            // Update StarCoder configuration
            if let Some(starcoder_config) = &adaptive_config.starcoder_integration {
                if let Some(threshold) = starcoder_config.training_threshold {
                    result.rule_generation.starcoder_training_threshold = threshold as usize;
                }
                if let Some(train_good) = starcoder_config.train_on_good_code {
                    result.rule_generation.train_on_good_code = train_good;
                }
                if let Some(train_bad) = starcoder_config.train_on_bad_patterns {
                    result.rule_generation.train_starcoder_on_patterns = train_bad;
                }
                if let Some(max_examples) = starcoder_config.max_training_examples {
                    result.rule_generation.max_training_examples = max_examples as usize;
                }
            }
        }

        result
    }
}

/// Statistics about the adaptive rule system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRuleSystemStats {
    /// Total patterns tracked
    pub total_patterns_tracked: usize,
    /// Patterns eligible for rule generation
    pub patterns_eligible_for_rules: usize,
    /// Total rules generated
    pub total_rules_generated: usize,
    /// Rules currently active
    pub active_rules: usize,
    /// Average rule quality score
    pub average_rule_quality: f64,
    /// Last analysis timestamp
    pub last_analysis: chrono::DateTime<chrono::Utc>,
}

/// Main adaptive rule system that orchestrates pattern tracking and rule generation
pub struct AdaptiveRuleSystem {
    config: AdaptiveRuleSystemConfiguration,
    pattern_tracker: PatternFrequencyTracker,
    rule_generator: CustomRuleGenerator,
    generated_rules: Vec<GeneratedRule>,
    analysis_history: Vec<AnalysisCycle>,
}

/// Represents one complete analysis and rule generation cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCycle {
    pub cycle_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub patterns_analyzed: usize,
    pub clusters_formed: usize,
    pub rules_generated: usize,
    pub execution_time_ms: u64,
}

impl AdaptiveRuleSystem {
    /// Create new adaptive rule system
    pub fn new(config: AdaptiveRuleSystemConfiguration) -> Self {
        let pattern_tracker = PatternFrequencyTracker::new(config.pattern_tracking.clone());
        let rule_generator = CustomRuleGenerator::new(config.rule_generation.clone());

        Self {
            config,
            pattern_tracker,
            rule_generator,
            generated_rules: Vec::new(),
            analysis_history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(AdaptiveRuleSystemConfiguration::default())
    }

    /// Process lint issues and update pattern tracking
    pub fn process_lint_issues(&mut self, issues: &[LintIssue], file_path: &str) -> Result<()> {
        self.pattern_tracker.process_lint_issues(issues, file_path)
    }

    /// Run complete analysis cycle: track patterns, cluster them, and generate rules
    pub async fn run_analysis_cycle(&mut self) -> Result<AnalysisCycle> {
        let start_time = std::time::Instant::now();
        let cycle_id = uuid::Uuid::new_v4().to_string();

        // Step 1: Perform clustering analysis on tracked patterns
        let clusters = self.pattern_tracker.perform_clustering_analysis()?;

        // Step 2: Filter clusters that are ready for rule generation
        let eligible_clusters = self.get_eligible_clusters_for_rule_generation(&clusters);

        // Step 3: Generate rules for top clusters (limited by max_rules_per_cycle)
        let clusters_to_process: Vec<&PatternCluster> = eligible_clusters.into_iter().take(self.config.max_rules_per_cycle).collect();

        let mut rules_generated = 0;
        for cluster in &clusters_to_process {
            match self.rule_generator.generate_rule_from_cluster(cluster).await {
                Ok(rule) => {
                    self.generated_rules.push(rule.clone());

                    // Export rule to file if configured
                    if !self.config.output_directory.is_empty() {
                        let _ = self.rule_generator.export_rule_to_file(&rule, &self.config.output_directory);
                    }

                    rules_generated += 1;

                    println!("✅ Generated rule '{}' from cluster '{}'", rule.rule_name, cluster.cluster_id);
                }
                Err(e) => {
                    eprintln!("❌ Failed to generate rule for cluster '{}': {}", cluster.cluster_id, e);
                }
            }
        }

        // Step 4: Clean up old patterns
        let cleaned_patterns = self.pattern_tracker.cleanup_old_patterns()?;
        if cleaned_patterns > 0 {
            println!("🧹 Cleaned up {} old patterns", cleaned_patterns);
        }

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        let cycle = AnalysisCycle {
            cycle_id,
            timestamp: chrono::Utc::now(),
            patterns_analyzed: self.pattern_tracker.get_analysis_summary().total_patterns,
            clusters_formed: clusters.len(),
            rules_generated,
            execution_time_ms,
        };

        self.analysis_history.push(cycle.clone());

        println!(
            "🎯 Analysis cycle complete: {} patterns → {} clusters → {} rules ({}ms)",
            cycle.patterns_analyzed, cycle.clusters_formed, cycle.rules_generated, cycle.execution_time_ms
        );

        Ok(cycle)
    }

    /// Get clusters eligible for rule generation
    fn get_eligible_clusters_for_rule_generation<'a>(&self, clusters: &'a [PatternCluster]) -> Vec<&'a PatternCluster> {
        clusters
            .iter()
            .filter(|cluster| {
                cluster.total_frequency >= self.config.min_pattern_frequency_for_rules
                && cluster.generation_priority >= 7 // High priority only
                && cluster.cohesion_score >= 0.8 // High cohesion
            })
            .collect()
    }

    /// Get current system statistics
    pub fn get_statistics(&self) -> AdaptiveRuleSystemStats {
        let summary = self.pattern_tracker.get_analysis_summary();
        let eligible_patterns = self.pattern_tracker.get_patterns_for_rule_generation().len();

        let average_quality = if self.generated_rules.is_empty() {
            0.0
        } else {
            self.generated_rules.iter().map(|r| r.quality_score).sum::<f64>() / self.generated_rules.len() as f64
        };

        AdaptiveRuleSystemStats {
            total_patterns_tracked: summary.total_patterns,
            patterns_eligible_for_rules: eligible_patterns,
            total_rules_generated: self.generated_rules.len(),
            active_rules: self.generated_rules.len(), // For now, all generated rules are active
            average_rule_quality: average_quality,
            last_analysis: summary.timestamp,
        }
    }

    /// Get all generated rules
    pub fn get_generated_rules(&self) -> &[GeneratedRule] {
        &self.generated_rules
    }

    /// Get analysis history
    pub fn get_analysis_history(&self) -> &[AnalysisCycle] {
        &self.analysis_history
    }

    /// Export analysis report
    pub fn export_analysis_report(&self, file_path: &str) -> Result<()> {
        let stats = self.get_statistics();
        let report = serde_json::to_string_pretty(&json!({
            "system_stats": stats,
            "generated_rules": self.generated_rules.iter().map(|r| json!({
                "rule_id": r.rule_id,
                "rule_name": r.rule_name,
                "description": r.description,
                "quality_score": r.quality_score,
                "source_cluster": r.source_cluster,
                "generation_metadata": r.generation_metadata
            })).collect::<Vec<_>>(),
            "analysis_history": self.analysis_history
        }))?;

        std::fs::write(file_path, report)?;
        Ok(())
    }

    /// Reset the system (clear all tracked patterns and generated rules)
    pub fn reset(&mut self) {
        self.pattern_tracker = PatternFrequencyTracker::new(self.config.pattern_tracking.clone());
        self.rule_generator = CustomRuleGenerator::new(self.config.rule_generation.clone());
        self.generated_rules.clear();
        self.analysis_history.clear();
    }

    /// Update configuration
    pub fn update_configuration(&mut self, config: AdaptiveRuleSystemConfiguration) {
        self.config = config;
        // Note: This doesn't update existing tracker/generator instances
        // In a production system, you might want to recreate them
    }
}

/// Create adaptive rule system with intelligent pattern learning
/// Uses configuration from Moon PDK extension config
pub fn create_adaptive_rule_system() -> AdaptiveRuleSystem {
    // Load configuration from Moon PDK
    let moon_config = match MoonShineConfig::from_moon_workspace() {
        Ok(config) => config,
        Err(_) => {
            // Fallback to defaults if Moon config loading fails
            MoonShineConfig::default()
        }
    };

    // Create configuration from Moon PDK settings
    let config = AdaptiveRuleSystemConfiguration::from_moon_config(&moon_config);

    AdaptiveRuleSystem::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::javascript_typescript_linter::LintSeverity;

    fn create_test_issue(message: &str) -> LintIssue {
        LintIssue {
            rule_name: "test-rule".to_string(),
            message: message.to_string(),
            line: 1,
            column: 1,
            severity: LintSeverity::Warning,
            fix_available: false,
        }
    }

    #[test]
    fn test_adaptive_system_creation() {
        let system = AdaptiveRuleSystem::with_defaults();
        let stats = system.get_statistics();

        assert_eq!(stats.total_patterns_tracked, 0);
        assert_eq!(stats.total_rules_generated, 0);
    }

    #[test]
    fn test_pattern_processing() {
        let mut system = AdaptiveRuleSystem::with_defaults();

        let issues = vec![
            create_test_issue("Variable 'x' is unused"),
            create_test_issue("Variable 'y' is unused"),
            create_test_issue("Function 'test' is unused"),
        ];

        system.process_lint_issues(&issues, "test.ts").unwrap();

        let stats = system.get_statistics();
        assert!(stats.total_patterns_tracked > 0);
    }

    #[tokio::test]
    async fn test_basic_analysis_cycle() {
        let mut system = create_adaptive_rule_system();

        // Add some patterns first
        for i in 0..10 {
            let issues = vec![create_test_issue(&format!("Variable 'var{}' is unused", i))];
            system.process_lint_issues(&issues, &format!("test{}.ts", i)).unwrap();
        }

        // Run analysis cycle
        let cycle = system.run_analysis_cycle().await.unwrap();

        assert!(cycle.patterns_analyzed > 0);
        assert!(cycle.execution_time_ms > 0);

        let stats = system.get_statistics();
        println!("Analysis complete - tracked {} patterns", stats.total_patterns_tracked);
    }

    #[test]
    fn test_configuration_update() {
        let mut system = AdaptiveRuleSystem::with_defaults();

        let mut new_config = AdaptiveRuleSystemConfiguration::default();
        new_config.max_rules_per_cycle = 10;

        system.update_configuration(new_config);
        assert_eq!(system.config.max_rules_per_cycle, 10);
    }

    #[test]
    fn test_system_reset() {
        let mut system = AdaptiveRuleSystem::with_defaults();

        // Add some data
        let issues = vec![create_test_issue("test message")];
        system.process_lint_issues(&issues, "test.ts").unwrap();

        // Reset system
        system.reset();

        let stats = system.get_statistics();
        assert_eq!(stats.total_patterns_tracked, 0);
        assert_eq!(stats.total_rules_generated, 0);
    }
}
