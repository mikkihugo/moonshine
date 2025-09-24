//! # Pattern Frequency Tracking and Clustering System
//!
//! Advanced pattern analysis for detecting repeated lint violations and clustering them
//! for custom rule generation. Uses statistical analysis and machine learning techniques
//! to identify patterns that warrant new custom rules.
//!
//! @category analysis
//! @safe program
//! @complexity high
//! @since 2.1.0

use crate::error::{Error, Result};
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::types::DiagnosticSeverity;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

/// Pattern signature for grouping similar violations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PatternSignature {
    /// Normalized message pattern
    pub message_pattern: String,
    /// Severity level
    pub severity: LintSeverity,
    /// File type context
    pub file_type: String,
    /// AST node type context
    pub node_type: Option<String>,
    /// Code context hash
    pub context_hash: u64,
}

/// Frequency data for a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternFrequency {
    /// Pattern signature
    pub signature: PatternSignature,
    /// Total occurrences across codebase
    pub total_occurrences: usize,
    /// Number of unique files affected
    pub files_affected: usize,
    /// List of affected file paths
    pub affected_files: Vec<String>,
    /// Frequency trend over time
    pub trend_data: Vec<FrequencyTrendPoint>,
    /// Statistical confidence score (0.0-1.0)
    pub confidence_score: f64,
    /// First detected timestamp
    pub first_detected: chrono::DateTime<chrono::Utc>,
    /// Last detected timestamp
    pub last_detected: chrono::DateTime<chrono::Utc>,
}

/// Trend point for frequency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyTrendPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub occurrence_count: usize,
    pub files_count: usize,
}

/// Cluster of related patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCluster {
    /// Cluster identifier
    pub cluster_id: String,
    /// Primary pattern representing the cluster
    pub primary_pattern: PatternSignature,
    /// Related patterns in this cluster
    pub related_patterns: Vec<PatternSignature>,
    /// Total frequency across all patterns
    pub total_frequency: usize,
    /// Cluster cohesion score (0.0-1.0)
    pub cohesion_score: f64,
    /// Suggested rule name for this cluster
    pub suggested_rule_name: String,
    /// Suggested rule description
    pub suggested_rule_description: String,
    /// Rule generation priority (1-10)
    pub generation_priority: u8,
}

/// Configuration for pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTrackingConfiguration {
    /// Minimum occurrences to consider a pattern significant
    pub minimum_occurrence_threshold: usize,
    /// Minimum files affected to consider a pattern widespread
    pub minimum_file_threshold: usize,
    /// Time window for trend analysis (in days)
    pub trend_analysis_window_days: u32,
    /// Similarity threshold for clustering (0.0-1.0)
    pub clustering_similarity_threshold: f64,
    /// Maximum age of patterns to consider (in days)
    pub pattern_max_age_days: u32,
    /// Confidence score threshold for rule generation
    pub rule_generation_confidence_threshold: f64,
}

impl Default for PatternTrackingConfiguration {
    fn default() -> Self {
        Self {
            minimum_occurrence_threshold: 5,
            minimum_file_threshold: 3,
            trend_analysis_window_days: 30,
            clustering_similarity_threshold: 0.8,
            pattern_max_age_days: 90,
            rule_generation_confidence_threshold: 0.85,
        }
    }
}

/// Main pattern frequency tracker
#[derive(Debug)]
pub struct PatternFrequencyTracker {
    config: PatternTrackingConfiguration,
    pattern_frequencies: HashMap<PatternSignature, PatternFrequency>,
    clusters: Vec<PatternCluster>,
    analysis_history: Vec<AnalysisSnapshot>,
}

/// Snapshot of analysis state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_patterns: usize,
    pub total_occurrences: usize,
    pub clusters_formed: usize,
    pub rules_suggested: usize,
}

impl PatternFrequencyTracker {
    /// Create new pattern frequency tracker
    pub fn new(config: PatternTrackingConfiguration) -> Self {
        Self {
            config,
            pattern_frequencies: HashMap::new(),
            clusters: Vec::new(),
            analysis_history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(PatternTrackingConfiguration::default())
    }

    /// Process lint issues and update pattern frequencies
    pub fn process_lint_issues(&mut self, issues: &[LintIssue], file_path: &str) -> Result<()> {
        let file_type = Self::extract_file_type(file_path);
        let now = chrono::Utc::now();

        for issue in issues {
            let signature = self.create_pattern_signature(issue, &file_type)?;

            // Update or create pattern frequency
            let frequency = self.pattern_frequencies.entry(signature.clone()).or_insert_with(|| PatternFrequency {
                signature: signature.clone(),
                total_occurrences: 0,
                files_affected: 0,
                affected_files: Vec::new(),
                trend_data: Vec::new(),
                confidence_score: 0.0,
                first_detected: now,
                last_detected: now,
            });

            // Update frequency data
            frequency.total_occurrences += 1;
            frequency.last_detected = now;

            if !frequency.affected_files.contains(&file_path.to_string()) {
                frequency.affected_files.push(file_path.to_string());
                frequency.files_affected += 1;
            }

            // Add trend point
            frequency.trend_data.push(FrequencyTrendPoint {
                timestamp: now,
                occurrence_count: frequency.total_occurrences,
                files_count: frequency.files_affected,
            });

            // Update confidence score based on frequency and spread
            // Note: We calculate this after updating the frequency data
        }

        // Update confidence scores for all modified patterns
        // We need to calculate scores first, then update to avoid borrowing conflicts
        let mut score_updates: Vec<(PatternSignature, f64)> = Vec::new();

        for (key, frequency) in &self.pattern_frequencies {
            let confidence_score = self.calculate_confidence_score_from_data(frequency.total_occurrences, frequency.files_affected, frequency.first_detected);
            score_updates.push((key.clone(), confidence_score));
        }

        // Now apply the updates
        for (key, score) in score_updates {
            if let Some(frequency) = self.pattern_frequencies.get_mut(&key) {
                frequency.confidence_score = score;
            }
        }

        Ok(())
    }

    /// Create pattern signature from lint issue
    fn create_pattern_signature(&self, issue: &LintIssue, file_type: &str) -> Result<PatternSignature> {
        // Normalize message to pattern
        let message_pattern = self.normalize_message_to_pattern(&issue.message);

        // Create context from surrounding code (placeholder)
        let context_hash = self.calculate_context_hash(&issue.message, file_type);

        Ok(PatternSignature {
            message_pattern,
            severity: issue.severity.clone(),
            file_type: file_type.to_string(),
            node_type: self.extract_node_type_from_message(&issue.message),
            context_hash,
        })
    }

    /// Normalize lint message to pattern
    fn normalize_message_to_pattern(&self, message: &str) -> String {
        // Replace specific identifiers with placeholders
        let mut pattern = message.to_string();

        // Replace quoted strings with placeholder
        pattern = regex::Regex::new(r#"'[^']*'"#).unwrap().replace_all(&pattern, "'<IDENTIFIER>'").to_string();
        pattern = regex::Regex::new(r#""[^"]*""#).unwrap().replace_all(&pattern, "\"<IDENTIFIER>\"").to_string();

        // Replace numbers with placeholder
        pattern = regex::Regex::new(r"\b\d+\b").unwrap().replace_all(&pattern, "<NUMBER>").to_string();

        // Replace variable names (camelCase, snake_case)
        pattern = regex::Regex::new(r"\b[a-z][a-zA-Z0-9_]*\b").unwrap().replace_all(&pattern, "<VAR>").to_string();

        // Replace type names (PascalCase)
        pattern = regex::Regex::new(r"\b[A-Z][a-zA-Z0-9]*\b").unwrap().replace_all(&pattern, "<TYPE>").to_string();

        pattern
    }

    /// Extract node type from message if possible
    fn extract_node_type_from_message(&self, message: &str) -> Option<String> {
        // Look for common AST node indicators in messages
        if message.contains("function") {
            Some("Function".to_string())
        } else if message.contains("variable") {
            Some("Variable".to_string())
        } else if message.contains("class") {
            Some("Class".to_string())
        } else if message.contains("import") {
            Some("Import".to_string())
        } else if message.contains("export") {
            Some("Export".to_string())
        } else if message.contains("type") {
            Some("Type".to_string())
        } else if message.contains("interface") {
            Some("Interface".to_string())
        } else {
            None
        }
    }

    /// Calculate context hash for grouping
    fn calculate_context_hash(&self, message: &str, file_type: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        file_type.hash(&mut hasher);
        hasher.finish()
    }

    /// Calculate confidence score for a pattern
    fn calculate_confidence_score(&self, frequency: &PatternFrequency) -> f64 {
        self.calculate_confidence_score_from_data(frequency.total_occurrences, frequency.files_affected, frequency.first_detected)
    }

    /// Calculate confidence score from raw data
    fn calculate_confidence_score_from_data(&self, total_occurrences: usize, files_affected: usize, first_detected: chrono::DateTime<chrono::Utc>) -> f64 {
        let occurrence_factor = (total_occurrences as f64 / 100.0).min(1.0);
        let spread_factor = (files_affected as f64 / 50.0).min(1.0);
        let age_factor = {
            let age_days = (chrono::Utc::now() - first_detected).num_days() as f64;
            (age_days / 30.0).min(1.0) // More confidence with age
        };

        (occurrence_factor * 0.4 + spread_factor * 0.4 + age_factor * 0.2).min(1.0)
    }

    /// Extract file type from path
    fn extract_file_type(file_path: &str) -> String {
        std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Perform clustering analysis on patterns
    pub fn perform_clustering_analysis(&mut self) -> Result<Vec<PatternCluster>> {
        let significant_patterns: Vec<&PatternFrequency> = self
            .pattern_frequencies
            .values()
            .filter(|freq| {
                freq.total_occurrences >= self.config.minimum_occurrence_threshold
                    && freq.files_affected >= self.config.minimum_file_threshold
                    && freq.confidence_score >= self.config.rule_generation_confidence_threshold
            })
            .collect();

        let mut clusters = Vec::new();
        let mut clustered_patterns = std::collections::HashSet::new();

        for pattern in &significant_patterns {
            if clustered_patterns.contains(&pattern.signature) {
                continue;
            }

            // Find similar patterns for clustering
            let mut cluster_patterns = vec![pattern.signature.clone()];
            clustered_patterns.insert(pattern.signature.clone());

            for other_pattern in &significant_patterns {
                if clustered_patterns.contains(&other_pattern.signature) {
                    continue;
                }

                let similarity = self.calculate_pattern_similarity(&pattern.signature, &other_pattern.signature);
                if similarity >= self.config.clustering_similarity_threshold {
                    cluster_patterns.push(other_pattern.signature.clone());
                    clustered_patterns.insert(other_pattern.signature.clone());
                }
            }

            if cluster_patterns.len() >= 2 {
                // Only create clusters with multiple patterns
                let cluster = self.create_cluster(cluster_patterns, &significant_patterns)?;
                clusters.push(cluster);
            }
        }

        self.clusters = clusters.clone();
        Ok(clusters)
    }

    /// Calculate similarity between two patterns
    fn calculate_pattern_similarity(&self, pattern1: &PatternSignature, pattern2: &PatternSignature) -> f64 {
        let mut similarity = 0.0;
        let mut factors = 0.0;

        // Message pattern similarity (weighted heavily)
        if pattern1.message_pattern == pattern2.message_pattern {
            similarity += 0.5;
        } else {
            // Calculate string similarity
            let string_sim = self.calculate_string_similarity(&pattern1.message_pattern, &pattern2.message_pattern);
            similarity += string_sim * 0.5;
        }
        factors += 0.5;

        // Severity similarity
        if pattern1.severity == pattern2.severity {
            similarity += 0.2;
        }
        factors += 0.2;

        // File type similarity
        if pattern1.file_type == pattern2.file_type {
            similarity += 0.15;
        }
        factors += 0.15;

        // Node type similarity
        match (&pattern1.node_type, &pattern2.node_type) {
            (Some(n1), Some(n2)) if n1 == n2 => similarity += 0.15,
            (None, None) => similarity += 0.10, // Both unknown is somewhat similar
            _ => {}                             // Different or one unknown
        }
        factors += 0.15;

        similarity / factors
    }

    /// Calculate string similarity using Levenshtein distance
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let distance = self.levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len());
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }

    /// Calculate Levenshtein distance
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1).min(matrix[i][j - 1] + 1).min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Create cluster from patterns
    fn create_cluster(&self, patterns: Vec<PatternSignature>, all_patterns: &[&PatternFrequency]) -> Result<PatternCluster> {
        // Find primary pattern (most frequent)
        let primary_pattern = patterns
            .iter()
            .max_by_key(|sig| {
                all_patterns
                    .iter()
                    .find(|freq| &freq.signature == *sig)
                    .map(|freq| freq.total_occurrences)
                    .unwrap_or(0)
            })
            .unwrap()
            .clone();

        // Calculate total frequency
        let total_frequency: usize = patterns
            .iter()
            .filter_map(|sig| all_patterns.iter().find(|freq| freq.signature == *sig).map(|freq| freq.total_occurrences))
            .sum();

        // Calculate cohesion score
        let cohesion_score = self.calculate_cluster_cohesion(&patterns);

        // Generate rule suggestions
        let (rule_name, rule_description) = self.generate_rule_suggestion(&primary_pattern, &patterns);

        // Calculate priority based on frequency and confidence
        let priority = self.calculate_generation_priority(total_frequency, cohesion_score);

        Ok(PatternCluster {
            cluster_id: self.generate_cluster_id(&primary_pattern),
            primary_pattern,
            related_patterns: patterns,
            total_frequency,
            cohesion_score,
            suggested_rule_name: rule_name,
            suggested_rule_description: rule_description,
            generation_priority: priority,
        })
    }

    /// Calculate cluster cohesion score
    fn calculate_cluster_cohesion(&self, patterns: &[PatternSignature]) -> f64 {
        if patterns.len() < 2 {
            return 1.0;
        }

        let mut total_similarity = 0.0;
        let mut comparisons = 0;

        for i in 0..patterns.len() {
            for j in i + 1..patterns.len() {
                total_similarity += self.calculate_pattern_similarity(&patterns[i], &patterns[j]);
                comparisons += 1;
            }
        }

        if comparisons == 0 {
            1.0
        } else {
            total_similarity / comparisons as f64
        }
    }

    /// Generate rule suggestion for cluster
    fn generate_rule_suggestion(&self, primary: &PatternSignature, patterns: &[PatternSignature]) -> (String, String) {
        // Extract common theme from message patterns
        let common_theme = self.extract_common_theme(&primary.message_pattern);

        let rule_name = format!("moonshine-{}", common_theme.to_lowercase().replace(' ', "-").replace("<", "").replace(">", ""));

        let rule_description = format!(
            "Detects {} patterns occurring {} times across {} variations. Primary pattern: '{}'",
            common_theme,
            patterns.len(),
            patterns.len(),
            primary.message_pattern
        );

        (rule_name, rule_description)
    }

    /// Extract common theme from message pattern
    fn extract_common_theme(&self, message: &str) -> String {
        // Look for key themes in the message
        if message.contains("unused") {
            "unused-code".to_string()
        } else if message.contains("type") {
            "type-issues".to_string()
        } else if message.contains("import") {
            "import-issues".to_string()
        } else if message.contains("export") {
            "export-issues".to_string()
        } else if message.contains("function") {
            "function-issues".to_string()
        } else if message.contains("variable") {
            "variable-issues".to_string()
        } else if message.contains("const") {
            "const-issues".to_string()
        } else if message.contains("class") {
            "class-issues".to_string()
        } else if message.contains("async") {
            "async-issues".to_string()
        } else if message.contains("promise") {
            "promise-issues".to_string()
        } else {
            "code-quality".to_string()
        }
    }

    /// Generate cluster ID
    fn generate_cluster_id(&self, primary: &PatternSignature) -> String {
        let mut hasher = DefaultHasher::new();
        primary.hash(&mut hasher);
        format!("cluster-{:x}", hasher.finish())
    }

    /// Calculate generation priority
    fn calculate_generation_priority(&self, frequency: usize, cohesion: f64) -> u8 {
        let freq_score = (frequency as f64 / 100.0).min(1.0);
        let combined_score = (freq_score * 0.6 + cohesion * 0.4) * 10.0;
        combined_score.round() as u8
    }

    /// Get patterns ready for rule generation
    pub fn get_patterns_for_rule_generation(&self) -> Vec<&PatternCluster> {
        self.clusters
            .iter()
            .filter(|cluster| {
                cluster.generation_priority >= 7 // High priority clusters only
                && cluster.cohesion_score >= self.config.clustering_similarity_threshold
                && cluster.total_frequency >= self.config.minimum_occurrence_threshold * 2
            })
            .collect()
    }

    /// Get analysis summary
    pub fn get_analysis_summary(&self) -> AnalysisSnapshot {
        AnalysisSnapshot {
            timestamp: chrono::Utc::now(),
            total_patterns: self.pattern_frequencies.len(),
            total_occurrences: self.pattern_frequencies.values().map(|freq| freq.total_occurrences).sum(),
            clusters_formed: self.clusters.len(),
            rules_suggested: self.get_patterns_for_rule_generation().len(),
        }
    }

    /// Clean up old patterns
    pub fn cleanup_old_patterns(&mut self) -> Result<usize> {
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(self.config.pattern_max_age_days as i64);
        let initial_count = self.pattern_frequencies.len();

        self.pattern_frequencies.retain(|_, freq| freq.last_detected > cutoff_date);

        Ok(initial_count - self.pattern_frequencies.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_issue(message: &str, severity: LintSeverity) -> LintIssue {
        LintIssue {
            rule_name: "test-rule".to_string(),
            message: message.to_string(),
            line: 1,
            column: 1,
            severity,
            fix_available: false,
        }
    }

    #[test]
    fn test_pattern_signature_creation() {
        let mut tracker = PatternFrequencyTracker::with_defaults();
        let issue = create_test_issue("Variable 'userData' is unused", LintSeverity::Warning);

        let signature = tracker.create_pattern_signature(&issue, "ts").unwrap();
        assert_eq!(signature.message_pattern, "Variable '<VAR>' is unused");
        assert_eq!(signature.file_type, "ts");
        assert_eq!(signature.node_type, Some("Variable".to_string()));
    }

    #[test]
    fn test_message_normalization() {
        let tracker = PatternFrequencyTracker::with_defaults();

        let normalized = tracker.normalize_message_to_pattern("Function 'calculateTotal' has unused parameter 'tax'");
        assert_eq!(normalized, "Function '<VAR>' has unused parameter '<VAR>'");

        let normalized2 = tracker.normalize_message_to_pattern("Type 'UserData' is not exported");
        assert_eq!(normalized2, "<TYPE> '<TYPE>' is not exported");
    }

    #[test]
    fn test_pattern_frequency_tracking() {
        let mut tracker = PatternFrequencyTracker::with_defaults();
        let issues = vec![
            create_test_issue("Variable 'x' is unused", LintSeverity::Warning),
            create_test_issue("Variable 'y' is unused", LintSeverity::Warning),
            create_test_issue("Function 'test' is unused", LintSeverity::Warning),
        ];

        tracker.process_lint_issues(&issues, "test.ts").unwrap();

        assert_eq!(tracker.pattern_frequencies.len(), 2); // 2 distinct patterns

        // Check variable pattern frequency
        let var_pattern = tracker
            .pattern_frequencies
            .values()
            .find(|freq| freq.signature.message_pattern.contains("Variable"))
            .unwrap();
        assert_eq!(var_pattern.total_occurrences, 2);
    }

    #[test]
    fn test_pattern_similarity_calculation() {
        let tracker = PatternFrequencyTracker::with_defaults();

        let sig1 = PatternSignature {
            message_pattern: "Variable '<VAR>' is unused".to_string(),
            severity: LintSeverity::Warning,
            file_type: "ts".to_string(),
            node_type: Some("Variable".to_string()),
            context_hash: 123,
        };

        let sig2 = PatternSignature {
            message_pattern: "Variable '<VAR>' is unused".to_string(),
            severity: LintSeverity::Warning,
            file_type: "ts".to_string(),
            node_type: Some("Variable".to_string()),
            context_hash: 456,
        };

        let similarity = tracker.calculate_pattern_similarity(&sig1, &sig2);
        assert!(similarity > 0.8); // Should be very similar except context
    }

    #[test]
    fn test_clustering_analysis() {
        let mut tracker = PatternFrequencyTracker::with_defaults();
        tracker.config.minimum_occurrence_threshold = 1;
        tracker.config.minimum_file_threshold = 1;
        tracker.config.rule_generation_confidence_threshold = 0.0;

        // Add multiple similar patterns
        for i in 0..5 {
            let issue = create_test_issue(&format!("Variable 'var{}' is unused", i), LintSeverity::Warning);
            tracker.process_lint_issues(&[issue], &format!("test{}.ts", i)).unwrap();
        }

        let clusters = tracker.perform_clustering_analysis().unwrap();
        assert!(!clusters.is_empty());

        let cluster = &clusters[0];
        assert!(cluster.suggested_rule_name.contains("unused"));
        assert!(cluster.generation_priority > 0);
    }

    #[test]
    fn test_confidence_score_calculation() {
        let tracker = PatternFrequencyTracker::with_defaults();

        let mut frequency = PatternFrequency {
            signature: PatternSignature {
                message_pattern: "test".to_string(),
                severity: LintSeverity::Warning,
                file_type: "ts".to_string(),
                node_type: None,
                context_hash: 123,
            },
            total_occurrences: 50,
            files_affected: 25,
            affected_files: (0..25).map(|i| format!("file{}.ts", i)).collect(),
            trend_data: Vec::new(),
            confidence_score: 0.0,
            first_detected: chrono::Utc::now() - chrono::Duration::days(15),
            last_detected: chrono::Utc::now(),
        };

        frequency.confidence_score = tracker.calculate_confidence_score(&frequency);
        assert!(frequency.confidence_score > 0.5);
        assert!(frequency.confidence_score <= 1.0);
    }
}
