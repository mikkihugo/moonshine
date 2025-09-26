//! # Adaptive Pattern Analysis for AI Coder Mistakes
//!
//! Analyzes repetitive lint patterns from AI-generated code to create custom rules
//! and train local neural networks for pattern prediction and auto-fixing.

use crate::types::LintDiagnostic;
use chrono::{DateTime, Utc};
use oxc_ast::ast::Program;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Analyzer that learns from repetitive coding patterns to generate custom rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepetitivePatternLearner {
    /// Collected patterns with frequency counts
    pattern_frequencies: HashMap<String, PatternFrequency>,
    /// Pattern clustering results
    pattern_clusters: Vec<PatternCluster>,
    /// Generated custom rules from patterns
    generated_rules: Vec<GeneratedRule>,
    /// Neural network model paths
    model_paths: ModelPaths,
    /// Analysis configuration
    config: PatternLearningConfig,
}

/// Pattern frequency data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternFrequency {
    pub pattern_id: String,
    pub rule_id: String,
    pub message_template: String,
    pub count: u32,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub file_types: Vec<String>,
    pub severity_distribution: HashMap<String, u32>,
    pub code_contexts: Vec<CodeContext>,
    pub fix_patterns: Vec<FixPattern>,
}

/// Code context for pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeContext {
    pub before_lines: Vec<String>,
    pub error_line: String,
    pub after_lines: Vec<String>,
    pub ast_node_type: String,
    pub semantic_context: Option<String>,
}

/// Fix pattern extracted from successful auto-fixes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPattern {
    pub original_code: String,
    pub fixed_code: String,
    pub fix_type: FixType,
    pub confidence: f32,
    pub success_rate: f32,
}

/// Types of fixes that can be applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixType {
    /// Simple text replacement
    TextReplacement,
    /// AST-based transformation
    AstTransformation,
    /// AI-generated fix
    AiGenerated,
    /// Pattern-based template
    TemplateBasedFix,
}

/// Clustered patterns with similar characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCluster {
    pub cluster_id: String,
    pub patterns: Vec<String>,
    pub common_traits: Vec<String>,
    pub suggested_rule: Option<GeneratedRule>,
    pub confidence_score: f32,
}

/// Custom rule generated from pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub pattern_regex: Option<String>,
    pub ast_matcher: Option<String>,
    pub severity: String,
    pub auto_fix_template: Option<String>,
    pub confidence: f32,
    pub training_examples: u32,
}

/// Paths to trained neural network models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPaths {
    /// ONNX model for pattern classification
    pub pattern_classifier: Option<String>,
    /// Model for fix suggestion ranking
    pub fix_ranker: Option<String>,
    /// Code embedding model for similarity
    pub code_embedder: Option<String>,
    /// Custom pattern detector
    pub pattern_detector: Option<String>,
}

/// Configuration for repetitive pattern learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternLearningConfig {
    /// Minimum pattern frequency to consider for rule generation
    pub min_pattern_frequency: u32,
    /// Confidence threshold for auto-generated rules
    pub rule_confidence_threshold: f32,
    /// Maximum context lines to capture
    pub max_context_lines: usize,
    /// Enable neural network integration
    pub enable_neural_models: bool,
    /// Pattern clustering sensitivity
    pub clustering_threshold: f32,
    /// Auto-fix confidence threshold
    pub autofix_confidence_threshold: f32,
}

impl Default for PatternLearningConfig {
    fn default() -> Self {
        Self {
            min_pattern_frequency: 3,
            rule_confidence_threshold: 0.8,
            max_context_lines: 3,
            enable_neural_models: true,
            clustering_threshold: 0.7,
            autofix_confidence_threshold: 0.9,
        }
    }
}

impl RepetitivePatternLearner {
    /// Create new repetitive pattern learning system
    pub fn new(config: PatternLearningConfig) -> Self {
        Self {
            pattern_frequencies: HashMap::new(),
            pattern_clusters: Vec::new(),
            generated_rules: Vec::new(),
            model_paths: ModelPaths {
                pattern_classifier: None,
                fix_ranker: None,
                code_embedder: None,
                pattern_detector: None,
            },
            config,
        }
    }

    /// Analyze lint diagnostics to extract patterns
    pub fn analyze_lint_patterns(
        &mut self,
        diagnostics: &[LintDiagnostic],
        source_code: &str,
        ast: &Program<'_>,
        file_path: &str,
    ) -> Result<PatternAnalysisResult, Box<dyn std::error::Error>> {
        let mut new_patterns = 0;
        let mut updated_patterns = 0;

        for diagnostic in diagnostics {
            let pattern_id = self.generate_pattern_id(diagnostic);
            let code_context = self.extract_code_context(source_code, diagnostic.line as usize, file_path, ast)?;

            match self.pattern_frequencies.get_mut(&pattern_id) {
                Some(frequency) => {
                    // Update existing pattern
                    frequency.count += 1;
                    frequency.last_seen = Utc::now();
                    frequency.code_contexts.push(code_context);

                    // Update severity distribution
                    let severity_key = format!("{:?}", diagnostic.severity);
                    *frequency.severity_distribution.entry(severity_key).or_insert(0) += 1;

                    updated_patterns += 1;
                }
                None => {
                    // Create new pattern
                    let mut severity_distribution = HashMap::new();
                    let severity_key = format!("{:?}", diagnostic.severity);
                    severity_distribution.insert(severity_key, 1);

                    let frequency = PatternFrequency {
                        pattern_id: pattern_id.clone(),
                        rule_id: diagnostic.rule_name.clone(),
                        message_template: diagnostic.message.clone(),
                        count: 1,
                        first_seen: Utc::now(),
                        last_seen: Utc::now(),
                        file_types: vec![self.get_file_type(file_path)],
                        severity_distribution,
                        code_contexts: vec![code_context],
                        fix_patterns: Vec::new(),
                    };

                    self.pattern_frequencies.insert(pattern_id, frequency);
                    new_patterns += 1;
                }
            }
        }

        // Trigger pattern clustering if we have enough new data
        if new_patterns > 0 || updated_patterns > 5 {
            self.update_pattern_clusters()?;
        }

        // Generate new rules if patterns are frequent enough
        let new_rules = self.generate_rules_from_patterns()?;

        Ok(PatternAnalysisResult {
            new_patterns,
            updated_patterns,
            new_rules: new_rules.len(),
            total_patterns: self.pattern_frequencies.len(),
            high_frequency_patterns: self.get_high_frequency_patterns().len(),
        })
    }

    /// Generate unique pattern ID from diagnostic
    fn generate_pattern_id(&self, diagnostic: &LintDiagnostic) -> String {
        // Create pattern ID based on rule + normalized message
        let normalized_message = self.normalize_message(&diagnostic.message);
        format!("{}::{}", diagnostic.rule_name, normalized_message)
    }

    /// Normalize diagnostic message to extract pattern
    fn normalize_message(&self, message: &str) -> String {
        // Replace variable names, numbers, and strings with placeholders
        let mut normalized = message.to_string();

        // Replace quoted strings
        normalized = regex::Regex::new(r#"'[^']*'"#).unwrap().replace_all(&normalized, "'<STRING>'").to_string();
        normalized = regex::Regex::new(r#""[^"]*""#).unwrap().replace_all(&normalized, "\"<STRING>\"").to_string();

        // Replace numbers
        normalized = regex::Regex::new(r"\b\d+\b").unwrap().replace_all(&normalized, "<NUMBER>").to_string();

        // Replace common variable patterns
        normalized = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b")
            .unwrap()
            .replace_all(&normalized, "<IDENTIFIER>")
            .to_string();

        normalized
    }

    /// Extract code context around the error
    fn extract_code_context(
        &self,
        source_code: &str,
        error_line: usize,
        file_path: &str,
        ast: &Program<'_>,
    ) -> Result<CodeContext, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = source_code.lines().collect();
        let max_context = self.config.max_context_lines;

        let start_line = if error_line > max_context { error_line - max_context } else { 0 };
        let end_line = std::cmp::min(error_line + max_context + 1, lines.len());

        let before_lines = lines[start_line..error_line].iter().map(|s| s.to_string()).collect();

        let error_line_content = lines.get(error_line).unwrap_or(&"").to_string();

        let after_lines = lines[error_line + 1..end_line].iter().map(|s| s.to_string()).collect();

        // Extract AST node type at error location (simplified)
        let ast_node_type = self.get_ast_node_type_at_line(ast, error_line);

        Ok(CodeContext {
            before_lines,
            error_line: error_line_content,
            after_lines,
            ast_node_type,
            semantic_context: None, // TODO: Add semantic analysis
        })
    }

    /// Get AST node type at specific line (simplified implementation)
    fn get_ast_node_type_at_line(&self, _ast: &Program, _line: usize) -> String {
        // TODO: Implement proper AST traversal to find node at line
        "Unknown".to_string()
    }

    /// Get file type from extension
    fn get_file_type(&self, file_path: &str) -> String {
        std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Update pattern clusters using similarity analysis
    fn update_pattern_clusters(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement clustering algorithm
        // For now, group by rule_id as a simple clustering
        let mut clusters: HashMap<String, Vec<String>> = HashMap::new();

        for (pattern_id, frequency) in &self.pattern_frequencies {
            clusters.entry(frequency.rule_id.clone()).or_insert_with(Vec::new).push(pattern_id.clone());
        }

        self.pattern_clusters = clusters
            .into_iter()
            .map(|(rule_id, patterns)| PatternCluster {
                cluster_id: format!("cluster_{}", rule_id),
                patterns,
                common_traits: vec![], // TODO: Extract common traits
                suggested_rule: None,
                confidence_score: 0.0,
            })
            .collect();

        Ok(())
    }

    /// Generate custom rules from high-frequency patterns
    fn generate_rules_from_patterns(&mut self) -> Result<Vec<GeneratedRule>, Box<dyn std::error::Error>> {
        let mut new_rules = Vec::new();

        for (pattern_id, frequency) in &self.pattern_frequencies {
            if frequency.count >= self.config.min_pattern_frequency {
                // Check if we already have a rule for this pattern
                if !self.generated_rules.iter().any(|r| r.rule_id == *pattern_id) {
                    let rule = self.create_rule_from_pattern(pattern_id, frequency)?;
                    if rule.confidence >= self.config.rule_confidence_threshold {
                        new_rules.push(rule);
                    }
                }
            }
        }

        self.generated_rules.extend(new_rules.clone());
        Ok(new_rules)
    }

    /// Create a custom rule from a pattern
    fn create_rule_from_pattern(&self, pattern_id: &str, frequency: &PatternFrequency) -> Result<GeneratedRule, Box<dyn std::error::Error>> {
        // Calculate confidence based on frequency and consistency
        let confidence = self.calculate_rule_confidence(frequency);

        // Generate rule description
        let description = format!(
            "AI-generated rule for pattern detected {} times: {}",
            frequency.count, frequency.message_template
        );

        // TODO: Generate regex pattern from code contexts
        let pattern_regex = self.generate_regex_from_contexts(&frequency.code_contexts);

        Ok(GeneratedRule {
            rule_id: format!("adaptive_{}", pattern_id),
            name: format!("Adaptive Rule: {}", frequency.rule_id),
            description,
            pattern_regex,
            ast_matcher: None,               // TODO: Generate AST matcher
            severity: "warning".to_string(), // Default severity
            auto_fix_template: None,         // TODO: Generate from fix patterns
            confidence,
            training_examples: frequency.count,
        })
    }

    /// Calculate confidence score for rule generation
    fn calculate_rule_confidence(&self, frequency: &PatternFrequency) -> f32 {
        let frequency_score = (frequency.count as f32 / 10.0).min(1.0);
        let consistency_score = if frequency.code_contexts.len() > 1 {
            // TODO: Calculate consistency based on context similarity
            0.8
        } else {
            0.5
        };

        (frequency_score + consistency_score) / 2.0
    }

    /// Generate regex pattern from code contexts
    fn generate_regex_from_contexts(&self, _contexts: &[CodeContext]) -> Option<String> {
        // TODO: Implement pattern extraction from contexts
        None
    }

    /// Get patterns with high frequency
    fn get_high_frequency_patterns(&self) -> Vec<&PatternFrequency> {
        self.pattern_frequencies
            .values()
            .filter(|f| f.count >= self.config.min_pattern_frequency)
            .collect()
    }

    /// Train neural models on collected patterns
    pub async fn train_models(&mut self) -> Result<ModelTrainingResult, Box<dyn std::error::Error>> {
        if !self.config.enable_neural_models {
            return Ok(ModelTrainingResult {
                models_trained: 0,
                training_samples: 0,
                accuracy_metrics: HashMap::new(),
            });
        }

        // TODO: Integrate with your neural crates for:
        // 1. Pattern classification model
        // 2. Fix suggestion ranking model
        // 3. Code embedding model for similarity
        // 4. Custom pattern detector neural network

        log::info!("Training neural models on {} patterns", self.pattern_frequencies.len());

        Ok(ModelTrainingResult {
            models_trained: 0, // TODO: Implement
            training_samples: self.pattern_frequencies.len(),
            accuracy_metrics: HashMap::new(),
        })
    }

    /// Predict patterns using trained models
    pub async fn predict_patterns(&self, source_code: &str, ast: &Program<'_>) -> Result<Vec<PredictedPattern>, Box<dyn std::error::Error>> {
        // Use SourceType to determine file type for better pattern analysis
        let file_type = if source_code.contains("import ") || source_code.contains("export ") {
            "module"
        } else if source_code.contains("interface ") || source_code.contains("type ") {
            "typescript"
        } else {
            "javascript"
        };
        let mut predictions = Vec::new();

        // Use BTreeMap for frequency tracking of patterns
        let mut pattern_frequencies: BTreeMap<String, u32> = BTreeMap::new();

        // Analyze AST patterns
        for stmt in &ast.body {
            match stmt {
                oxc_ast::ast::Statement::VariableDeclaration(decl) => {
                    // Track pattern frequency
                    *pattern_frequencies.entry("variable-declaration".to_string()).or_insert(0) += 1;

                    // Check for common variable declaration patterns
                    if decl.declarations.len() > 3 {
                        predictions.push(PredictedPattern {
                            pattern_type: "multiple-variable-declaration".to_string(),
                            confidence: 0.8,
                            line_number: decl.span.start,
                            suggested_fix: Some("Split into separate declarations for better readability".to_string()),
                        });
                    }
                }
                oxc_ast::ast::Statement::IfStatement(if_stmt) => {
                    // Track pattern frequency
                    *pattern_frequencies.entry("if-statement".to_string()).or_insert(0) += 1;

                    // Check for nested if patterns
                    if self.count_nested_ifs(if_stmt) > 3 {
                        predictions.push(PredictedPattern {
                            pattern_type: "deep-nesting".to_string(),
                            confidence: 0.9,
                            line_number: if_stmt.span.start,
                            suggested_fix: Some("Extract nested conditions into separate functions".to_string()),
                        });
                    }
                }
                oxc_ast::ast::Statement::ForStatement(for_stmt) => {
                    // Track pattern frequency
                    *pattern_frequencies.entry("for-loop".to_string()).or_insert(0) += 1;

                    // Check for complex for loops
                    if self.is_complex_for_loop(for_stmt) {
                        predictions.push(PredictedPattern {
                            pattern_type: "complex-loop".to_string(),
                            confidence: 0.7,
                            line_number: for_stmt.span.start,
                            suggested_fix: Some("Consider using array methods like map, filter, or reduce".to_string()),
                        });
                    }
                }
                _ => {}
            }
        }

        // Analyze source code patterns
        let lines: Vec<&str> = source_code.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            if line.len() > 120 {
                predictions.push(PredictedPattern {
                    pattern_type: "long-line".to_string(),
                    confidence: 1.0,
                    line_number: line_num as u32 + 1,
                    suggested_fix: Some("Break long line into multiple lines".to_string()),
                });
            }

            if line.matches(';').count() > 3 {
                predictions.push(PredictedPattern {
                    pattern_type: "multiple-statements".to_string(),
                    confidence: 0.8,
                    line_number: line_num as u32 + 1,
                    suggested_fix: Some("Split into separate lines for better readability".to_string()),
                });
            }
        }

        Ok(predictions)
    }

    /// Count nested if statements
    fn count_nested_ifs(&self, if_stmt: &oxc_ast::ast::IfStatement) -> u32 {
        let mut count = 1;
        if let Some(consequent) = &if_stmt.consequent {
            if let oxc_ast::ast::Statement::IfStatement(nested) = consequent.as_ref() {
                count += self.count_nested_ifs(nested);
            }
        }
        if let Some(alternate) = &if_stmt.alternate {
            if let oxc_ast::ast::Statement::IfStatement(nested) = alternate.as_ref() {
                count += self.count_nested_ifs(nested);
            }
        }
        count
    }

    /// Check if for loop is complex
    fn is_complex_for_loop(&self, for_stmt: &oxc_ast::ast::ForStatement) -> bool {
        // Simple heuristic: check if loop has multiple statements in body
        if let Some(body) = &for_stmt.body {
            match body.as_ref() {
                oxc_ast::ast::Statement::BlockStatement(block) => block.statement.len() > 5,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Suggest fixes using pattern analysis and ML models
    pub async fn suggest_fixes(&self, diagnostic: &LintDiagnostic, code_context: &CodeContext) -> Result<Vec<FixSuggestion>, Box<dyn std::error::Error>> {
        let pattern_id = self.generate_pattern_id(diagnostic);

        if let Some(frequency) = self.pattern_frequencies.get(&pattern_id) {
            // Use historical fix patterns to suggest fixes
            let mut suggestions = Vec::new();

            for fix_pattern in &frequency.fix_patterns {
                if fix_pattern.confidence >= self.config.autofix_confidence_threshold {
                    suggestions.push(FixSuggestion {
                        description: format!("Apply pattern-based fix (confidence: {:.2})", fix_pattern.confidence),
                        fix_code: fix_pattern.fixed_code.clone(),
                        fix_type: fix_pattern.fix_type.clone(),
                        confidence: fix_pattern.confidence,
                    });
                }
            }

            // TODO: Use neural models to generate additional suggestions

            Ok(suggestions)
        } else {
            Ok(Vec::new())
        }
    }

    /// Export pattern data for external analysis
    pub fn export_patterns(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(&self.pattern_frequencies)?)
    }

    /// Load neural network models
    pub fn load_models(&mut self, model_paths: ModelPaths) -> Result<(), Box<dyn std::error::Error>> {
        self.model_paths = model_paths;
        // TODO: Load ONNX models using your neural crates
        Ok(())
    }
}

/// Result of pattern analysis
#[derive(Debug)]
pub struct PatternAnalysisResult {
    pub new_patterns: usize,
    pub updated_patterns: usize,
    pub new_rules: usize,
    pub total_patterns: usize,
    pub high_frequency_patterns: usize,
}

/// Result of model training
#[derive(Debug)]
pub struct ModelTrainingResult {
    pub models_trained: usize,
    pub training_samples: usize,
    pub accuracy_metrics: HashMap<String, f32>,
}

/// Predicted pattern from ML models
#[derive(Debug, Clone)]
pub struct PredictedPattern {
    pub pattern_type: String,
    pub confidence: f32,
    pub suggested_fix: Option<String>,
    pub line_number: u32,
}

/// Fix suggestion from pattern analysis
#[derive(Debug)]
pub struct FixSuggestion {
    pub description: String,
    pub fix_code: String,
    pub fix_type: FixType,
    pub confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_analyzer_creation() {
        let config = AdaptiveAnalysisConfig::default();
        let analyzer = AdaptivePatternAnalyzer::new(config);
        assert_eq!(analyzer.pattern_frequencies.len(), 0);
    }

    #[test]
    fn test_message_normalization() {
        let config = AdaptiveAnalysisConfig::default();
        let analyzer = AdaptivePatternAnalyzer::new(config);

        let message = "Variable 'userName' is unused";
        let normalized = analyzer.normalize_message(message);
        assert_eq!(normalized, "Variable '<STRING>' is unused");
    }

    #[test]
    fn test_pattern_id_generation() {
        let config = AdaptiveAnalysisConfig::default();
        let analyzer = AdaptivePatternAnalyzer::new(config);

        let diagnostic = LintDiagnostic {
            rule_id: "no-unused-vars".to_string(),
            message: "Variable 'x' is unused".to_string(),
            severity: crate::types::DiagnosticSeverity::Warning,
            line: 1,
            column: 1,
            end_line: Some(1),
            end_column: Some(10),
            file_path: "test.js".to_string(),
            suggested_fix: None,
        };

        let pattern_id = analyzer.generate_pattern_id(&diagnostic);
        assert!(pattern_id.starts_with("no-unused-vars::"));
    }
}
