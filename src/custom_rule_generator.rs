//! # Custom Rule Generation Pipeline
//!
//! Automated generation of custom lint rules based on detected patterns from
//! the pattern frequency tracker. Uses AI assistance and template-based generation
//! to create production-ready custom rules.
//!
//! @category ai-integration
//! @safe program
//! @complexity high
//! @since 2.1.0

use crate::dspy::{Example, MetaSignature};
use crate::error::{Error, Result};
use crate::javascript_typescript_linter::LintSeverity;
use crate::oxc_adapter::starcoder_integration::StarCoderIntegration;
use crate::pattern_frequency_tracker::{PatternCluster, PatternSignature};
use crate::provider_router::{AIContext, AIRequest, AIRouter};
use crate::rule_types::{RuleCategory, RuleSeverity};
use crate::{example, field};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for rule generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGenerationConfiguration {
    /// AI provider to use for rule generation (routed through provider_router)
    pub ai_provider: String,
    /// Whether to include test cases in generated rules
    pub generate_test_cases: bool,
    /// Whether to include documentation
    pub generate_documentation: bool,
    /// Target programming languages for rules
    pub target_languages: Vec<String>,
    /// Quality threshold for generated rules (0.0-1.0)
    pub quality_threshold: f64,
    /// Backwards-compatible alias used by configuration loaders
    pub rule_quality_threshold: f64,
    /// Minimum cluster size required before attempting generation
    pub min_cluster_size_for_rules: usize,
    /// Maximum rules to produce for a single cluster per cycle
    pub max_rules_per_cluster: usize,
    /// Cap on the number of StarCoder training examples retained between runs
    pub max_training_examples: usize,
    /// Whether generated rules should be enabled automatically
    pub enable_auto_rule_activation: bool,
    /// Whether to use vector similarity for template selection
    pub use_vector_similarity: bool,
    /// Whether to use StarCoder-1B for code pattern analysis
    pub use_starcoder_patterns: bool,
    /// Whether to train StarCoder on discovered patterns
    pub train_starcoder_on_patterns: bool,
    /// Minimum pattern frequency to trigger StarCoder training
    pub starcoder_training_threshold: usize,
    /// Whether to train StarCoder on good code examples (no violations)
    pub train_on_good_code: bool,
    /// Custom instruction override for AI generation
    pub custom_instruction: Option<String>,
}

impl Default for RuleGenerationConfiguration {
    fn default() -> Self {
        Self {
            ai_provider: "auto".to_string(), // Uses provider router for intelligent selection
            generate_test_cases: true,
            generate_documentation: true,
            target_languages: vec!["typescript".to_string(), "javascript".to_string()],
            quality_threshold: 0.85,
            rule_quality_threshold: 0.85,
            min_cluster_size_for_rules: 5,
            max_rules_per_cluster: 3,
            max_training_examples: 1000,
            enable_auto_rule_activation: false,
            use_vector_similarity: true,
            use_starcoder_patterns: true,      // Enable StarCoder-1B for pattern analysis
            train_starcoder_on_patterns: true, // Train StarCoder on discovered patterns
            starcoder_training_threshold: 10,  // Minimum 10 pattern occurrences to trigger training
            train_on_good_code: true,          // Also train on clean code examples
            custom_instruction: None,
        }
    }
}

/// Generated custom rule representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedRule {
    /// Rule identifier
    pub rule_id: String,
    /// Rule name
    pub rule_name: String,
    /// Rule description
    pub description: String,
    /// Rule category
    pub category: RuleCategory,
    /// Rule severity
    pub severity: RuleSeverity,
    /// Generated implementation code
    pub implementation_code: String,
    /// Generated test cases
    pub test_cases: Vec<GeneratedTestCase>,
    /// Generated documentation
    pub documentation: String,
    /// Source pattern cluster
    pub source_cluster: String,
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
    /// Generation metadata
    pub generation_metadata: RuleGenerationMetadata,
}

/// Test case for generated rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTestCase {
    /// Test case name
    pub name: String,
    /// Input code for testing
    pub input_code: String,
    /// Expected violations (empty if code should be valid)
    pub expected_violations: Vec<ExpectedViolation>,
    /// Test case description
    pub description: String,
}

/// Expected violation in test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedViolation {
    pub line: u32,
    pub column: u32,
    pub message_pattern: String,
    pub severity: LintSeverity,
}

/// Metadata about rule generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGenerationMetadata {
    /// Generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// AI provider used (via provider router)
    pub ai_provider_used: String,
    /// Template used for generation
    pub template_used: String,
    /// Pattern frequency that triggered generation
    pub source_frequency: usize,
    /// Confidence score from pattern analysis
    pub source_confidence: f64,
    /// Generation time in milliseconds
    pub generation_time_ms: u64,
}

/// DSPy signature for rule generation
#[derive(Debug, Clone)]
pub struct RuleGenerationSignature {
    demos: Vec<Example>,
    instruction: String,
    prefix: String,
}

impl RuleGenerationSignature {
    pub fn new() -> Self {
        Self {
            demos: vec![
                example! {
                    "pattern_description": "input" => "Detects unused variables with pattern 'Variable '<VAR>' is unused' occurring 15 times across 8 files",
                    "severity": "input" => "Warning",
                    "file_types": "input" => "typescript,javascript",
                    "sample_messages": "input" => "Variable 'userData' is unused\nVariable 'config' is unused\nVariable 'result' is unused",
                    "rule_implementation": "output" => "//! # No Unused Variables Rule\n//!\n//! Detects unused variable declarations that should be removed\n//! to improve code clarity and prevent confusion.\n\nuse crate::javascript_typescript_linter::{LintIssue, LintSeverity};\nuse oxc_ast::ast::*;\nuse oxc_ast_visit::Visit;\nuse oxc_semantic::Semantic;\nuse oxc_span::Span;\n\n#[derive(Debug, Default)]\npub struct NoUnusedVariables;\n\nimpl NoUnusedVariables {\n    fn create_diagnostic(&self, span: Span, var_name: &str) -> LintIssue {\n        LintIssue {\n            rule_name: \"moonshine-no-unused-variables\".to_string(),\n            message: format!(\"Variable '{}' is declared but never used\", var_name),\n            line: span.start as u32,\n            column: 1,\n            severity: LintSeverity::Warning,\n            fix_available: true,\n        }\n    }\n}\n\nimpl Visit<'_> for NoUnusedVariables {\n    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator) {\n        if let Some(id) = &decl.id {\n            // Check if variable is used in semantic analysis\n            // Implementation details would use semantic analysis\n            // to determine if variable is actually unused\n        }\n    }\n}",
                    "test_cases": "output" => "#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_unused_variable_detection() {\n        let code = \"const unused = 42; console.log('hello');\";\n        let mut rule = NoUnusedVariables::default();\n        // Test implementation\n        assert!(violations.len() == 1);\n        assert!(violations[0].message.contains(\"unused\"));\n    }\n\n    #[test]\n    fn test_used_variable_no_violation() {\n        let code = \"const used = 42; console.log(used);\";\n        let mut rule = NoUnusedVariables::default();\n        // Test implementation\n        assert!(violations.is_empty());\n    }\n}",
                    "documentation": "output" => "# moonshine-no-unused-variables\n\nDetects variables that are declared but never used in the code.\n\n## Why?\n\nUnused variables can:\n- Confuse other developers\n- Indicate incomplete code\n- Increase bundle size\n- Create maintenance burden\n\n## Examples\n\n### ❌ Incorrect\n\n```typescript\nconst data = fetchData(); // unused\nconsole.log('Processing...');\n```\n\n### ✅ Correct\n\n```typescript\nconst data = fetchData();\nconsole.log('Data:', data);\n```\n\n## Configuration\n\nThis rule has no configuration options.\n\n## When to Disable\n\n- During development when variables are temporarily unused\n- For variables that will be used by external tools\n- In generated code"
                }
            ],
            instruction: "Generate a complete custom lint rule implementation based on detected patterns. Include Rust code using OXC AST, comprehensive test cases, and clear documentation.".to_string(),
            prefix: "Custom Rule Generation".to_string(),
        }
    }
}

impl MetaSignature for RuleGenerationSignature {
    fn demos(&self) -> Vec<Example> {
        self.demos.clone()
    }

    fn set_demos(&mut self, demos: Vec<Example>) -> anyhow::Result<()> {
        self.demos = demos;
        Ok(())
    }

    fn instruction(&self) -> String {
        self.instruction.clone()
    }

    fn input_fields(&self) -> Value {
        field! {
            input["Pattern description from frequency analysis"] => pattern_description: String,
            input["Rule severity level"] => severity: String,
            input["Target file types"] => file_types: String,
            input["Sample violation messages"] => sample_messages: String
        }
    }

    fn output_fields(&self) -> Value {
        field! {
            output["Complete Rust rule implementation using OXC AST"] => rule_implementation: String,
            output["Comprehensive test cases"] => test_cases: String,
            output["Rule documentation in Markdown"] => documentation: String
        }
    }

    fn update_instruction(&mut self, instruction: String) -> anyhow::Result<()> {
        self.instruction = instruction;
        Ok(())
    }

    fn append(&mut self, _name: &str, _value: Value) -> anyhow::Result<()> {
        Ok(())
    }

    fn prefix(&self) -> String {
        self.prefix.clone()
    }

    fn update_prefix(&mut self, prefix: String) -> anyhow::Result<()> {
        self.prefix = prefix;
        Ok(())
    }
}

/// Custom rule generation pipeline
pub struct CustomRuleGenerator {
    config: RuleGenerationConfiguration,
    signature: RuleGenerationSignature,
    generated_rules: Vec<GeneratedRule>,
    template_library: HashMap<String, RuleTemplate>,
    ai_router: AIRouter,
    starcoder_integration: StarCoderIntegration,
}

/// Rule template for different pattern types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub applicable_patterns: Vec<String>,
    pub implementation_template: String,
    pub test_template: String,
    pub documentation_template: String,
}

impl CustomRuleGenerator {
    /// Create new rule generator
    pub fn new(config: RuleGenerationConfiguration) -> Self {
        Self {
            config,
            signature: RuleGenerationSignature::new(),
            generated_rules: Vec::new(),
            template_library: Self::load_rule_templates(),
            ai_router: AIRouter::new(),
            starcoder_integration: StarCoderIntegration::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(RuleGenerationConfiguration::default())
    }

    /// Load rule templates for different pattern types
    fn load_rule_templates() -> HashMap<String, RuleTemplate> {
        let mut templates = HashMap::new();

        // Unused code template
        templates.insert(
            "unused-code".to_string(),
            RuleTemplate {
                template_id: "unused-code".to_string(),
                name: "Unused Code Detection".to_string(),
                description: "Template for detecting unused variables, functions, imports, etc.".to_string(),
                applicable_patterns: vec!["unused".to_string(), "never used".to_string()],
                implementation_template: include_str!("templates/unused_code_template.rs").to_string(),
                test_template: include_str!("templates/unused_code_test_template.rs").to_string(),
                documentation_template: include_str!("templates/unused_code_docs_template.md").to_string(),
            },
        );

        // Type safety template
        templates.insert(
            "type-safety".to_string(),
            RuleTemplate {
                template_id: "type-safety".to_string(),
                name: "Type Safety Enforcement".to_string(),
                description: "Template for TypeScript type safety rules".to_string(),
                applicable_patterns: vec!["type".to_string(), "any".to_string(), "assertion".to_string()],
                implementation_template: "// Type safety rule template".to_string(),
                test_template: "// Type safety test template".to_string(),
                documentation_template: "# Type Safety Rule\n\nEnforces TypeScript type safety best practices.".to_string(),
            },
        );

        // Best practices template
        templates.insert(
            "best-practices".to_string(),
            RuleTemplate {
                template_id: "best-practices".to_string(),
                name: "Best Practices Enforcement".to_string(),
                description: "Template for general code quality and best practices rules".to_string(),
                applicable_patterns: vec!["prefer".to_string(), "avoid".to_string(), "use".to_string()],
                implementation_template: "// Best practices rule template".to_string(),
                test_template: "// Best practices test template".to_string(),
                documentation_template: "# Best Practices Rule\n\nEnforces coding best practices.".to_string(),
            },
        );

        templates
    }

    /// Generate custom rule from pattern cluster
    pub async fn generate_rule_from_cluster(&mut self, cluster: &PatternCluster) -> Result<GeneratedRule> {
        let start_time = std::time::Instant::now();

        let cluster_size = cluster.related_patterns.len() + 1;
        if cluster_size < self.config.min_cluster_size_for_rules {
            return Err(Error::Validation {
                field: "min_cluster_size_for_rules".to_string(),
                expected: format!(">= {}", self.config.min_cluster_size_for_rules),
                actual: cluster_size.to_string(),
            });
        }

        // Select appropriate template
        let template = self.select_template_for_cluster(cluster)?;

        // Prepare AI inputs
        let pattern_description = format!(
            "Detects {} patterns with message pattern '{}' occurring {} times across {} files",
            self.extract_pattern_theme(&cluster.primary_pattern),
            cluster.primary_pattern.message_pattern,
            cluster.total_frequency,
            cluster.related_patterns.len()
        );

        let sample_messages = cluster
            .related_patterns
            .iter()
            .take(3)
            .map(|p| p.message_pattern.clone())
            .collect::<Vec<_>>()
            .join("\n");

        let inputs = json!({
            "pattern_description": pattern_description,
            "severity": self.map_severity_to_string(&cluster.primary_pattern.severity),
            "file_types": cluster.primary_pattern.file_type.clone(),
            "sample_messages": sample_messages
        });

        // Validate inputs
        self.signature.validate_inputs(&inputs)?;

        // Generate rule using AI
        let full_prompt = self.signature.generate_prompt(&inputs);
        let ai_response = self.call_ai_for_rule_generation(&full_prompt).await?;

        // Parse AI response
        let (implementation, test_cases_code, documentation) = self.parse_ai_response(&ai_response)?;

        // Generate test cases
        let test_cases = self.generate_test_cases_from_cluster(cluster, &test_cases_code)?;

        // Calculate quality score
        let quality_score = self.calculate_rule_quality(&implementation, &test_cases, cluster);

        // Create rule metadata
        let rule_id = format!("moonshine-{}", cluster.suggested_rule_name.replace("moonshine-", ""));
        let generation_metadata = RuleGenerationMetadata {
            generated_at: chrono::Utc::now(),
            ai_provider_used: self.config.ai_provider.clone(),
            template_used: template.template_id.clone(),
            source_frequency: cluster.total_frequency,
            source_confidence: cluster.cohesion_score,
            generation_time_ms: start_time.elapsed().as_millis() as u64,
        };

        let generated_rule = GeneratedRule {
            rule_id: rule_id.clone(),
            rule_name: cluster.suggested_rule_name.clone(),
            description: cluster.suggested_rule_description.clone(),
            category: self.map_pattern_to_category(&cluster.primary_pattern),
            severity: self.map_lint_severity_to_rule_severity(&cluster.primary_pattern.severity),
            implementation_code: implementation,
            test_cases,
            documentation,
            source_cluster: cluster.cluster_id.clone(),
            quality_score,
            generation_metadata,
        };

        // Validate rule quality
        let quality_threshold = self.config.rule_quality_threshold;
        if quality_score >= quality_threshold {
            self.generated_rules.push(generated_rule.clone());
            Ok(generated_rule)
        } else {
            Err(Error::Config {
                message: format!("Generated rule quality score {} below threshold {}", quality_score, quality_threshold),
                field: Some("quality_threshold".to_string()),
                value: Some(quality_score.to_string()),
            })
        }
    }

    /// Select appropriate template for cluster
    fn select_template_for_cluster(&self, cluster: &PatternCluster) -> Result<&RuleTemplate> {
        let pattern_theme = self.extract_pattern_theme(&cluster.primary_pattern);

        // Find best matching template
        for (_, template) in &self.template_library {
            for applicable_pattern in &template.applicable_patterns {
                if pattern_theme.to_lowercase().contains(&applicable_pattern.to_lowercase()) {
                    return Ok(template);
                }
            }
        }

        // Default to best practices template
        self.template_library.get("best-practices").ok_or_else(|| Error::Config {
            message: "No suitable template found for pattern cluster".to_string(),
            field: Some("template_library".to_string()),
            value: Some("best-practices".to_string()),
        })
    }

    /// Extract theme from pattern signature
    fn extract_pattern_theme(&self, pattern: &PatternSignature) -> String {
        let message = &pattern.message_pattern.to_lowercase();

        if message.contains("unused") {
            "unused-code".to_string()
        } else if message.contains("type") || message.contains("any") {
            "type-safety".to_string()
        } else if message.contains("import") || message.contains("export") {
            "import-export".to_string()
        } else if message.contains("async") || message.contains("promise") {
            "async-patterns".to_string()
        } else if message.contains("function") {
            "function-patterns".to_string()
        } else if message.contains("class") {
            "class-patterns".to_string()
        } else {
            "best-practices".to_string()
        }
    }

    /// Call AI provider for rule generation using provider router
    async fn call_ai_for_rule_generation(&self, prompt: &str) -> Result<String> {
        let context = AIContext::CodeGeneration {
            language: "rust".to_string(),
            specification: "Custom lint rule generation".to_string(),
        };

        let mut preferred_providers = Vec::new();
        if self.config.ai_provider != "auto" && !self.config.ai_provider.is_empty() {
            preferred_providers.push(self.config.ai_provider.clone());
        }

        let request = AIRequest {
            prompt: prompt.to_string(),
            session_id: format!("rulegen-{}", Uuid::new_v4()),
            file_path: Some("generated_rule.rs".to_string()),
            context,
            preferred_providers,
        };

        match self.ai_router.execute(request).await {
            Ok(response) => Ok(response.content),
            Err(e) => {
                moon_warn!("AI provider routing failed during rule generation: {}. Falling back to template output.", e);
                Ok(self.generate_fallback_rule(prompt))
            }
        }
    }

    /// Generate fallback rule when AI provider fails
    fn generate_fallback_rule(&self, prompt: &str) -> String {
        format!(
            "IMPLEMENTATION:\n// Template-based rule implementation\n// Pattern: {}\n\nTEST_CASES:\n#[test]\nfn test_rule() {{\n    // Generated test case\n}}\n\nDOCUMENTATION:\n# Custom Rule\n\nGenerated from pattern analysis.\n\nFallback mode used due to AI provider unavailability.",
            prompt.lines().take(1).collect::<Vec<_>>().join("")
        )
    }

    /// Parse AI response into components
    fn parse_ai_response(&self, response: &str) -> Result<(String, String, String)> {
        // Simple parsing - in production would be more sophisticated
        let mut implementation = String::new();
        let mut test_cases = String::new();
        let mut documentation = String::new();

        let mut current_section = "";

        for line in response.lines() {
            if line.starts_with("IMPLEMENTATION:") {
                current_section = "impl";
                continue;
            } else if line.starts_with("TEST_CASES:") {
                current_section = "tests";
                continue;
            } else if line.starts_with("DOCUMENTATION:") {
                current_section = "docs";
                continue;
            }

            match current_section {
                "impl" => implementation.push_str(&format!("{}\n", line)),
                "tests" => test_cases.push_str(&format!("{}\n", line)),
                "docs" => documentation.push_str(&format!("{}\n", line)),
                _ => {}
            }
        }

        Ok((implementation, test_cases, documentation))
    }

    /// Generate test cases from cluster
    fn generate_test_cases_from_cluster(&self, cluster: &PatternCluster, _test_code: &str) -> Result<Vec<GeneratedTestCase>> {
        let mut test_cases = Vec::new();

        // Positive test case (should trigger rule)
        test_cases.push(GeneratedTestCase {
            name: "detects_violation".to_string(),
            input_code: "const unused = 42; console.log('hello');".to_string(),
            expected_violations: vec![ExpectedViolation {
                line: 1,
                column: 7,
                message_pattern: cluster.primary_pattern.message_pattern.clone(),
                severity: cluster.primary_pattern.severity.clone(),
            }],
            description: "Should detect the pattern violation".to_string(),
        });

        // Negative test case (should not trigger rule)
        test_cases.push(GeneratedTestCase {
            name: "no_violation_when_used".to_string(),
            input_code: "const used = 42; console.log(used);".to_string(),
            expected_violations: vec![],
            description: "Should not trigger when code is correct".to_string(),
        });

        Ok(test_cases)
    }

    /// Calculate rule quality score
    fn calculate_rule_quality(&self, implementation: &str, test_cases: &[GeneratedTestCase], cluster: &PatternCluster) -> f64 {
        let mut score: f64 = 0.0;

        // Implementation quality (40%)
        let impl_score = self.assess_implementation_quality(implementation);
        score += impl_score * 0.4;

        // Test coverage quality (30%)
        let test_score = self.assess_test_quality(test_cases);
        score += test_score * 0.3;

        // Pattern match quality (30%)
        let pattern_score = cluster.cohesion_score;
        score += pattern_score * 0.3;

        score.min(1.0)
    }

    /// Assess implementation code quality
    fn assess_implementation_quality(&self, implementation: &str) -> f64 {
        let mut score: f64 = 0.0;

        // Check for required imports
        if implementation.contains("use crate::javascript_typescript_linter") {
            score += 0.2;
        }
        if implementation.contains("use oxc_ast") {
            score += 0.2;
        }
        if implementation.contains("Visit") {
            score += 0.2;
        }

        // Check for proper structure
        if implementation.contains("impl Visit") {
            score += 0.2;
        }
        if implementation.contains("LintIssue") {
            score += 0.2;
        }

        score.min(1.0)
    }

    /// Assess test quality
    fn assess_test_quality(&self, test_cases: &[GeneratedTestCase]) -> f64 {
        if test_cases.is_empty() {
            return 0.0;
        }

        let mut score: f64 = 0.0;

        // Check for both positive and negative test cases
        let has_positive = test_cases.iter().any(|tc| !tc.expected_violations.is_empty());
        let has_negative = test_cases.iter().any(|tc| tc.expected_violations.is_empty());

        if has_positive {
            score += 0.5;
        }
        if has_negative {
            score += 0.5;
        }

        score
    }

    /// Map pattern to rule category
    fn map_pattern_to_category(&self, pattern: &PatternSignature) -> RuleCategory {
        let theme = self.extract_pattern_theme(pattern);
        match theme.as_str() {
            "unused-code" => RuleCategory::Correctness,
            "type-safety" => RuleCategory::Style,
            "async-patterns" => RuleCategory::Performance,
            _ => RuleCategory::Style,
        }
    }

    /// Map lint severity to rule severity
    fn map_lint_severity_to_rule_severity(&self, severity: &LintSeverity) -> RuleSeverity {
        match severity {
            LintSeverity::Error => RuleSeverity::Error,
            LintSeverity::Warning => RuleSeverity::Warning,
            LintSeverity::Info | LintSeverity::Hint => RuleSeverity::Info,
        }
    }

    /// Map severity to string
    fn map_severity_to_string(&self, severity: &LintSeverity) -> String {
        match severity {
            LintSeverity::Error => "Error".to_string(),
            LintSeverity::Warning => "Warning".to_string(),
            LintSeverity::Info | LintSeverity::Hint => "Info".to_string(),
        }
    }

    /// Generate rules for multiple clusters
    pub async fn generate_rules_for_clusters(&mut self, clusters: &[PatternCluster]) -> Result<Vec<GeneratedRule>> {
        let mut generated_rules = Vec::new();

        for cluster in clusters {
            match self.generate_rule_from_cluster(cluster).await {
                Ok(rule) => generated_rules.push(rule),
                Err(e) => {
                    eprintln!("Failed to generate rule for cluster {}: {}", cluster.cluster_id, e);
                    continue;
                }
            }
        }

        // Train StarCoder on discovered patterns if enabled
        if self.config.train_starcoder_on_patterns {
            self.train_starcoder_on_patterns(clusters).await?;
        }

        Ok(generated_rules)
    }

    /// Train StarCoder on discovered patterns and good code examples
    async fn train_starcoder_on_patterns(&mut self, clusters: &[PatternCluster]) -> Result<()> {
        let mut training_examples = Vec::new();

        // Filter clusters that meet the training threshold
        let eligible_clusters: Vec<&PatternCluster> = clusters
            .iter()
            .filter(|cluster| cluster.total_frequency >= self.config.starcoder_training_threshold)
            .collect();

        for cluster in &eligible_clusters {
            // Create training examples from bad patterns (violations)
            let bad_pattern_example = format!(
                "// BAD PATTERN (avoid this):\n// Pattern: {}\n// Frequency: {} violations\n// Files affected: {}\n// Examples of what NOT to do:\n{}",
                cluster.primary_pattern.message_pattern,
                cluster.total_frequency,
                cluster.related_patterns.len(),
                cluster
                    .related_patterns
                    .iter()
                    .take(3)
                    .map(|p| format!("// ❌ {}", p.message_pattern))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            training_examples.push(bad_pattern_example);

            // Generate good code examples if enabled
            if self.config.train_on_good_code {
                let good_pattern_example = self.generate_good_code_example(cluster);
                training_examples.push(good_pattern_example);
            }
        }

        // Only train if we have enough examples and meet threshold
        if !training_examples.is_empty() && eligible_clusters.len() >= self.config.starcoder_training_threshold {
            if training_examples.len() > self.config.max_training_examples {
                training_examples.truncate(self.config.max_training_examples);
            }

            println!(
                "🧠 Training StarCoder on {} pattern examples ({} violations, {} good examples)...",
                training_examples.len(),
                eligible_clusters.len(),
                if self.config.train_on_good_code { eligible_clusters.len() } else { 0 }
            );

            // Send training data to StarCoder integration
            if let Err(e) = self.starcoder_integration.add_training_examples(training_examples).await {
                return Err(Error::Processing {
                    message: format!("Failed to add training examples to StarCoder: {}", e),
                });
            }

            // Trigger incremental training
            if let Err(e) = self.starcoder_integration.train_on_patterns().await {
                return Err(Error::Processing {
                    message: format!("Failed to trigger StarCoder training: {}", e),
                });
            }
            println!("✅ StarCoder training complete - model updated with your codebase patterns");
        } else {
            println!(
                "⏳ Not enough patterns for StarCoder training (need {} occurrences, found {})",
                self.config.starcoder_training_threshold,
                eligible_clusters.len()
            );
        }

        Ok(())
    }

    /// Generate good code example from bad pattern
    fn generate_good_code_example(&self, cluster: &PatternCluster) -> String {
        let pattern_theme = self.extract_pattern_theme(&cluster.primary_pattern);

        let good_example = match pattern_theme.as_str() {
            "unused-code" => {
                "// GOOD PATTERN (follow this):\n// Variables are properly used\nconst userData = fetchUserData();\nconsole.log('Processing:', userData);\nreturn processData(userData);"
            },
            "type-safety" => {
                "// GOOD PATTERN (follow this):\n// Proper TypeScript typing\ninterface User {\n  id: number;\n  name: string;\n}\nconst user: User = { id: 1, name: 'John' };"
            },
            "async-patterns" => {
                "// GOOD PATTERN (follow this):\n// Proper async/await usage\ntry {\n  const result = await processAsync();\n  return result;\n} catch (error) {\n  handleError(error);\n}"
            },
            _ => {
                "// GOOD PATTERN (follow this):\n// Clean, readable code\nconst result = performOperation();\nif (result.success) {\n  handleSuccess(result.data);\n}"
            }
        };

        format!(
            "{}\n// ✅ This pattern avoids: {}\n// Frequency prevented: {} potential violations",
            good_example, cluster.primary_pattern.message_pattern, cluster.total_frequency
        )
    }

    /// Export generated rule to file
    pub fn export_rule_to_file(&self, rule: &GeneratedRule, output_dir: &str) -> Result<String> {
        let rule_file_path = format!("{}/{}.rs", output_dir, rule.rule_id.replace("-", "_"));

        let complete_rule = format!(
            "//! # {}\n//!\n//! {}\n//!\n//! Generated automatically from pattern analysis\n//! Source cluster: {}\n//! Quality score: {:.2}\n\n{}",
            rule.rule_name, rule.description, rule.source_cluster, rule.quality_score, rule.implementation_code
        );

        std::fs::write(&rule_file_path, complete_rule)?;
        Ok(rule_file_path)
    }

    /// Get generation statistics
    pub fn get_generation_statistics(&self) -> HashMap<String, Value> {
        let mut stats = HashMap::new();

        stats.insert("total_rules_generated".to_string(), json!(self.generated_rules.len()));
        stats.insert(
            "average_quality_score".to_string(),
            json!(self.generated_rules.iter().map(|r| r.quality_score).sum::<f64>() / self.generated_rules.len() as f64),
        );
        stats.insert(
            "rules_by_category".to_string(),
            json!(self.generated_rules.iter().fold(HashMap::new(), |mut acc, rule| {
                *acc.entry(format!("{:?}", rule.category)).or_insert(0) += 1;
                acc
            })),
        );

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern_frequency_tracker::PatternFrequency;

    fn create_test_cluster() -> PatternCluster {
        PatternCluster {
            cluster_id: "test-cluster".to_string(),
            primary_pattern: PatternSignature {
                message_pattern: "Variable '<VAR>' is unused".to_string(),
                severity: LintSeverity::Warning,
                file_type: "ts".to_string(),
                node_type: Some("Variable".to_string()),
                context_hash: 123,
            },
            related_patterns: vec![],
            total_frequency: 10,
            cohesion_score: 0.9,
            suggested_rule_name: "moonshine-no-unused-variables".to_string(),
            suggested_rule_description: "Detects unused variables".to_string(),
            generation_priority: 8,
        }
    }

    #[test]
    fn test_template_selection() {
        let generator = CustomRuleGenerator::with_defaults();
        let cluster = create_test_cluster();

        let template = generator.select_template_for_cluster(&cluster).unwrap();
        assert_eq!(template.template_id, "unused-code");
    }

    #[test]
    fn test_pattern_theme_extraction() {
        let generator = CustomRuleGenerator::with_defaults();
        let cluster = create_test_cluster();

        let theme = generator.extract_pattern_theme(&cluster.primary_pattern);
        assert_eq!(theme, "unused-code");
    }

    #[test]
    fn test_ai_response_parsing() {
        let generator = CustomRuleGenerator::with_defaults();
        let response = "IMPLEMENTATION:\nconst rule = {};\n\nTEST_CASES:\ntest('works', () => {});\n\nDOCUMENTATION:\n# Rule Docs\n";

        let (impl_code, test_code, docs) = generator.parse_ai_response(response).unwrap();
        assert!(impl_code.contains("const rule"));
        assert!(test_code.contains("test('works'"));
        assert!(docs.contains("# Rule Docs"));
    }

    #[test]
    fn test_implementation_quality_assessment() {
        let generator = CustomRuleGenerator::with_defaults();

        let good_impl = r#"
            use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
            use oxc_ast::ast::*;
            use oxc_ast_visit::Visit;

            impl Visit<'_> for MyRule {
                // implementation
            }
        "#;

        let quality = generator.assess_implementation_quality(good_impl);
        assert!(quality > 0.8);
    }

    #[tokio::test]
    async fn test_rule_generation_workflow() {
        let mut generator = CustomRuleGenerator::with_defaults();
        let cluster = create_test_cluster();

        // This would normally call AI, but we're testing the workflow
        // In a real test environment, you might mock the AI call
        let result = generator.generate_rule_from_cluster(&cluster).await;

        // Check that we either get a rule or a quality threshold error
        match result {
            Ok(rule) => {
                assert_eq!(rule.source_cluster, cluster.cluster_id);
                assert!(!rule.implementation_code.is_empty());
            }
            Err(Error::Config { message, .. }) if message.contains("quality score") => {
                // Expected if quality is below threshold
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}
