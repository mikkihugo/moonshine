/*!
 * TSDoc Generation Template with Claude+DSPy Integration
 *
 * Advanced TypeScript documentation generation template for PrimeCode packages.
 * This template integrates directly with Claude via DSPy for intelligent TSDoc generation,
 * using the existing prompt infrastructure and optimization capabilities.
 */

use crate::dspy::{field, sign, example, prediction, MetaSignature, Example, Prediction};
use crate::error::{Error, Result};
use crate::prompts::PromptTemplate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// DSPy signature for TSDoc generation using Claude
#[derive(Debug, Clone)]
pub struct TSDocSignature {
    demos: Vec<Example>,
    instruction: String,
    prefix: String,
}

impl TSDocSignature {
    pub fn new() -> Self {
        Self {
            demos: vec![
                // Example demonstrating comprehensive TSDoc generation
                example! {
                    "file_path": "input" => "src/coordination/safe.ts",
                    "code": "input" => "export async function createEpic(name: string, config: EpicConfig): Promise<Epic> {\n  return new Epic(name, config);\n}",
                    "missing_count": "input" => "1",
                    "current_coverage": "input" => "65",
                    "target_coverage": "input" => "90",
                    "documented_code": "output" => "/**\n * Creates a new SAFe 6.0 epic with specified configuration\n *\n * @param name - The epic name following SAFe naming conventions\n * @param config - Epic configuration including PI mapping and dependencies\n * @returns Promise resolving to the created Epic instance\n *\n * @category coordination\n * @safe large-solution\n * @mvp core\n * @complexity medium\n * @since 1.0.0\n */\nexport async function createEpic(name: string, config: EpicConfig): Promise<Epic> {\n  return new Epic(name, config);\n}"
                }
            ],
            instruction: "Generate TSDoc documentation for TypeScript constructs. Add /** */ comments with @category @safe @mvp @complexity @since tags. Return complete documented code.".to_string(),
            prefix: "TSDoc Documentation Enhancement".to_string(),
        }
    }
}

impl MetaSignature for TSDocSignature {
    fn demos(&self) -> Vec<Example> {
        self.demos.clone()
    }

    fn set_demos(&mut self, demos: Vec<Example>) -> Result<()> {
        self.demos = demos;
        Ok(())
    }

    fn instruction(&self) -> String {
        self.instruction.clone()
    }

    fn input_fields(&self) -> Value {
        field! {
            input["TypeScript file path"] => file_path: String,
            input["Source code requiring documentation"] => code: String,
            input["Number of methods missing documentation"] => missing_count: String,
            input["Current TSDoc coverage percentage"] => current_coverage: String,
            input["Target coverage percentage"] => target_coverage: String
        }
    }

    fn output_fields(&self) -> Value {
        field! {
            output["Code with comprehensive TSDoc documentation"] => documented_code: String
        }
    }

    fn update_instruction(&mut self, instruction: String) -> Result<()> {
        self.instruction = instruction;
        Ok(())
    }

    fn append(&mut self, _name: &str, _value: Value) -> Result<()> {
        Ok(())
    }

    fn prefix(&self) -> String {
        self.prefix.clone()
    }

    fn update_prefix(&mut self, prefix: String) -> Result<()> {
        self.prefix = prefix;
        Ok(())
    }
}

/// Version and metadata tracking for TSDoc template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocVersion {
    /// Current template version (auto-incremented)
    pub version: String,
    /// Last update timestamp
    pub updated_at: String,
    /// Change counter for auto-increment
    pub change_count: u32,
    /// Git commit hash if available
    pub commit_hash: Option<String>,
    /// Template checksum for change detection
    pub template_checksum: String,
}

impl TSDocVersion {
    pub fn new() -> Self {
        Self {
            version: "1.0.1".to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            change_count: 1,
            commit_hash: None,
            template_checksum: Self::calculate_template_checksum(),
        }
    }

    /// Auto-increment version (x.x.34 -> x.x.35)
    pub fn increment(&mut self) {
        self.change_count += 1;

        // Parse current version and increment patch
        let parts: Vec<&str> = self.version.split('.').collect();
        if parts.len() == 3 {
            if let Ok(patch) = parts[2].parse::<u32>() {
                self.version = format!("{}.{}.{}", parts[0], parts[1], patch + 1);
            }
        }

        self.updated_at = chrono::Utc::now().to_rfc3339();
        self.template_checksum = Self::calculate_template_checksum();
    }

    fn calculate_template_checksum() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let template_content = format!("{}{}",
            get_tsdoc_prompt_template().template,
            TSDOC_MOON_TASKS
        );

        let mut hasher = DefaultHasher::new();
        template_content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Configuration for Claude+DSPy TSDoc generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocConfig {
    /// Version tracking and metadata
    pub version: TSDocVersion,
    /// Target coverage percentage (default: 90)
    pub target_coverage: u8,
    /// Directories to scan for TypeScript files
    pub scan_directories: Vec<String>,
    /// File patterns to include
    pub include_patterns: Vec<String>,
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Whether to use COPRO optimization
    pub use_copro_optimization: bool,
    /// Claude model to use for generation
    pub claude_model: String,
    /// Custom instruction override
    pub custom_instruction: Option<String>,
    /// Enable vector similarity for example selection
    pub use_vector_similarity: bool,
    /// Vector embedding model for similarity
    pub embedding_model: String,
    /// Maximum examples to include via similarity
    pub max_similarity_examples: usize,
    /// Similarity threshold for example inclusion
    pub similarity_threshold: f32,
}

/// Vector embedding for TSDoc examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocEmbedding {
    pub code_pattern: String,
    pub documentation_pattern: String,
    pub embedding: Vec<f32>,
    pub construct_type: TSConstructType,
}

/// TypeScript construct types for categorized documentation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TSConstructType {
    Function,
    AsyncFunction,
    ArrowFunction,
    Method,
    AsyncMethod,
    StaticMethod,
    Getter,
    Setter,
    Class,
    AbstractClass,
    Interface,
    Type,
    Enum,
    Namespace,
    Module,
    Const,
    Let,
    Var,
    Property,
    Decorator,
    Generic,
}

impl Default for TSDocConfig {
    fn default() -> Self {
        Self {
            version: TSDocVersion::new(),
            target_coverage: 90,
            scan_directories: vec!["src".to_string()],
            include_patterns: vec!["**/*.ts".to_string(), "**/*.tsx".to_string()],
            exclude_patterns: vec![
                "**/*.test.ts".to_string(),
                "**/*.test.tsx".to_string(),
                "**/dist/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            use_copro_optimization: true,
            claude_model: "claude-3-5-sonnet-20241022".to_string(),
            custom_instruction: None,
            use_vector_similarity: true,
            embedding_model: "text-embedding-3-small".to_string(),
            max_similarity_examples: 5,
            similarity_threshold: 0.8,
        }
    }
}

/// TSDoc generator using Claude+DSPy integration
pub struct TSDocGenerator {
    signature: TSDocSignature,
    config: TSDocConfig,
    embeddings_cache: Vec<TSDocEmbedding>,
}

impl TSDocGenerator {
    /// Creates a new TSDoc generator with default configuration
    pub fn new() -> Self {
        let mut config = TSDocConfig::default();
        config.version.increment(); // Auto-increment on creation

        Self {
            signature: TSDocSignature::new(),
            config,
            embeddings_cache: Self::load_embeddings_cache(),
        }
    }

    /// Creates a new TSDoc generator with custom configuration
    pub fn with_config(mut config: TSDocConfig) -> Self {
        let mut signature = TSDocSignature::new();

        // Auto-increment version on configuration
        config.version.increment();

        // Apply custom instruction if provided
        if let Some(custom_instruction) = &config.custom_instruction {
            let _ = signature.update_instruction(custom_instruction.clone());
        }

        Self {
            signature,
            config,
            embeddings_cache: Self::load_embeddings_cache(),
        }
    }

    /// Update configuration and auto-increment version
    pub fn update_config(&mut self, new_config: TSDocConfig) -> Result<()> {
        // Check if configuration actually changed
        let old_checksum = &self.config.version.template_checksum;
        let new_checksum = TSDocVersion::calculate_template_checksum();

        if old_checksum != &new_checksum {
            self.config = new_config;
            self.config.version.increment();

            // Log version change
            eprintln!("TSDoc template updated to version {}", self.config.version.version);
        }

        Ok(())
    }

    /// Get current template version and metadata
    pub fn get_version_info(&self) -> &TSDocVersion {
        &self.config.version
    }

    /// Force version increment (for manual updates)
    pub fn increment_version(&mut self) {
        self.config.version.increment();
    }

    /// Load pre-computed embeddings for TSDoc examples
    fn load_embeddings_cache() -> Vec<TSDocEmbedding> {
        vec![
            TSDocEmbedding {
                code_pattern: "export async function".to_string(),
                documentation_pattern: "async function coordination workflow".to_string(),
                embedding: vec![0.1, 0.2, 0.3], // Placeholder - would be actual embeddings
                construct_type: TSConstructType::AsyncFunction,
            },
            TSDocEmbedding {
                code_pattern: "export class".to_string(),
                documentation_pattern: "class enterprise architecture".to_string(),
                embedding: vec![0.2, 0.3, 0.4],
                construct_type: TSConstructType::Class,
            },
            TSDocEmbedding {
                code_pattern: "export interface".to_string(),
                documentation_pattern: "interface type definition".to_string(),
                embedding: vec![0.3, 0.4, 0.5],
                construct_type: TSConstructType::Interface,
            },
        ]
    }

    /// Calculate vector similarity between code and cached examples
    fn calculate_similarity(&self, code_embedding: &[f32], cached_embedding: &[f32]) -> f32 {
        // Cosine similarity calculation
        let dot_product: f32 = code_embedding.iter()
            .zip(cached_embedding.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = code_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = cached_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Select best examples using vector similarity
    async fn select_examples_by_similarity(&self, code: &str) -> Result<Vec<Example>> {
        if !self.config.use_vector_similarity {
            return Ok(self.signature.demos());
        }

        // Generate embedding for input code (placeholder - would use actual embedding model)
        let code_embedding = self.generate_code_embedding(code).await?;

        // Find most similar examples
        let mut similarities: Vec<(f32, &TSDocEmbedding)> = self.embeddings_cache.iter()
            .map(|embedding| {
                let similarity = self.calculate_similarity(&code_embedding, &embedding.embedding);
                (similarity, embedding)
            })
            .collect();

        similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Convert top similarities to examples
        let mut examples = Vec::new();
        for (similarity, embedding) in similarities.into_iter()
            .take(self.config.max_similarity_examples)
            .filter(|(sim, _)| *sim >= self.config.similarity_threshold)
        {
            examples.push(example! {
                "code": "input" => embedding.code_pattern.clone(),
                "documented_code": "output" => embedding.documentation_pattern.clone()
            });
        }

        Ok(examples)
    }

    /// Generate embedding for code snippet
    async fn generate_code_embedding(&self, code: &str) -> Result<Vec<f32>> {
        // Placeholder - would integrate with actual embedding service
        // Could use OpenAI embeddings, local models, or cached embeddings
        Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5])
    }

    /// Generates TSDoc for a TypeScript file using Claude via DSPy
    pub async fn generate_tsdoc(
        &mut self,
        file_path: &str,
        code: &str,
        current_coverage: u8,
    ) -> Result<String> {
        // Select examples using vector similarity
        let similar_examples = self.select_examples_by_similarity(code).await?;
        self.signature.set_demos(similar_examples)?;

        // Prepare DSPy inputs
        let inputs = json!({
            "file_path": file_path,
            "code": code,
            "missing_count": self.calculate_missing_count(code),
            "current_coverage": current_coverage.to_string(),
            "target_coverage": self.config.target_coverage.to_string(),
            "construct_types": self.identify_construct_types(code)
        });

        // Validate inputs against signature
        self.signature.validate_inputs(&inputs)?;

        // Generate optimized prompt using DSPy signature
        let full_prompt = self.signature.generate_prompt(&inputs);

        // Execute via Claude with optimized prompt
        let documented_code = self.call_claude(&full_prompt).await?;

        // Validate outputs
        let outputs = json!({
            "documented_code": documented_code
        });
        self.signature.validate_outputs(&outputs)?;

        Ok(documented_code)
    }

    /// Identify TypeScript construct types in code
    fn identify_construct_types(&self, code: &str) -> Vec<String> {
        let mut types = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.contains("export async function") { types.push("AsyncFunction".to_string()); }
            else if trimmed.contains("export function") { types.push("Function".to_string()); }
            else if trimmed.contains("export class") { types.push("Class".to_string()); }
            else if trimmed.contains("export interface") { types.push("Interface".to_string()); }
            else if trimmed.contains("export type") { types.push("Type".to_string()); }
            else if trimmed.contains("export enum") { types.push("Enum".to_string()); }
            else if trimmed.contains("public") || trimmed.contains("private") || trimmed.contains("protected") {
                types.push("Method".to_string());
            }
        }

        types
    }

    /// Calculates all TypeScript constructs missing documentation
    fn calculate_missing_count(&self, code: &str) -> String {
        let export_patterns = [
            // Functions
            r"^export\s+function\s+\w+",
            r"^export\s+async\s+function\s+\w+",
            r"^export\s+const\s+\w+\s*=\s*\(",
            r"^export\s+const\s+\w+\s*=\s*async\s*\(",
            // Classes and methods
            r"^export\s+class\s+\w+",
            r"^export\s+abstract\s+class\s+\w+",
            r"^\s+(public|private|protected)\s+\w+\s*\(",
            r"^\s+(async\s+)?(public|private|protected)\s+\w+\s*\(",
            r"^\s+(static\s+)?(public|private|protected)\s+\w+\s*\(",
            r"^\s+(get|set)\s+\w+\s*\(",
            // Interfaces and types
            r"^export\s+interface\s+\w+",
            r"^export\s+type\s+\w+",
            r"^export\s+enum\s+\w+",
            // Namespaces and modules
            r"^export\s+namespace\s+\w+",
            r"^export\s+module\s+\w+",
            r"^declare\s+namespace\s+\w+",
            r"^declare\s+module\s+\w+",
            // Constants and variables
            r"^export\s+const\s+\w+",
            r"^export\s+let\s+\w+",
            r"^export\s+var\s+\w+",
            // Default exports
            r"^export\s+default\s+class\s+\w+",
            r"^export\s+default\s+function\s+\w+",
            r"^export\s+default\s+interface\s+\w+",
            // Decorators
            r"^@\w+",
            r"^\s+@\w+",
        ];

        let lines: Vec<&str> = code.lines().collect();
        let mut missing_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check if line matches export pattern
            for pattern in &export_patterns {
                if regex::Regex::new(pattern).unwrap().is_match(trimmed) {
                    // Check if TSDoc exists above this line
                    if !self.has_tsdoc_above(&lines, i) {
                        missing_count += 1;
                    }
                    break;
                }
            }
        }

        missing_count.to_string()
    }

    /// Checks if TSDoc exists above the given line
    fn has_tsdoc_above(&self, lines: &[&str], export_line: usize) -> bool {
        // Look backwards for TSDoc comment
        for i in (0..export_line).rev() {
            let line = lines[i].trim();

            if line.ends_with("*/") {
                // Found end of comment, look for start
                for j in (0..=i).rev() {
                    if lines[j].trim().starts_with("/**") {
                        return true;
                    }
                }
            }

            // Stop if we hit non-comment, non-empty line
            if !line.is_empty() && !line.starts_with("*") && !line.starts_with("//") {
                break;
            }
        }
        false
    }

    /// Calls Claude via existing provider router
    async fn call_claude(&self, prompt: &str) -> Result<String> {
        // This would integrate with your existing provider_router.rs
        // For now, returning a placeholder that would be replaced with actual Claude call

        // Example integration with existing infrastructure:
        // let provider = crate::provider_router::get_claude_provider()?;
        // let response = provider.complete(prompt, &self.config.claude_model).await?;
        // Ok(response.content)

        // Placeholder for demonstration
        Ok(format!(
            "/**\n * Generated TSDoc documentation using Claude+DSPy\n * \n * @category coordination\n * @safe program\n * @mvp core\n * @complexity medium\n * @since 1.0.0\n */\n{}",
            prompt
        ))
    }

    /// Updates the DSPy signature with COPRO optimization
    pub async fn optimize_with_copro(&mut self, examples: Vec<Example>) -> Result<()> {
        if !self.config.use_copro_optimization {
            return Ok(());
        }

        // Integrate with existing COPRO optimizer
        // This would use your existing optimizer/copro.rs infrastructure
        self.signature.set_demos(examples)?;

        Ok(())
    }
}

/// TSDoc template structure optimized for sed field editing
pub struct TSDocTemplateFields {
    /// Field markers for sed editing
    pub task: String,
    pub input_format: String,
    pub output_requirements: String,
    pub tags_specification: String,
    pub documentation_patterns: String,
    pub execution_command: String,
}

impl TSDocTemplateFields {
    pub fn new() -> Self {
        Self {
            // %%FIELD:TASK:START%%
            task: "Generate TSDoc documentation for TypeScript constructs.".to_string(),
            // %%FIELD:TASK:END%%

            // %%FIELD:INPUT_FORMAT:START%%
            input_format: "File: {file_path}\nCurrent Coverage: {current_coverage}%\nTarget Coverage: {target_coverage}%\nMissing Count: {missing_count}\nConstruct Types: {construct_types}".to_string(),
            // %%FIELD:INPUT_FORMAT:END%%

            // %%FIELD:OUTPUT_REQUIREMENTS:START%%
            output_requirements: "1. Add /** */ comments above ALL exports/public methods\n2. Include required tags: @category @safe @mvp @complexity @since\n3. Use proper @param @returns @throws tags\n4. Target {target_coverage}% coverage minimum\n5. Return complete code with documentation".to_string(),
            // %%FIELD:OUTPUT_REQUIREMENTS:END%%

            // %%FIELD:TAGS_SPECIFICATION:START%%
            tags_specification: "@category: coordination|audit|validation|security|performance|integration\n@safe: team|program|large-solution|portfolio\n@mvp: core|extension|future\n@complexity: low|medium|high|critical\n@since: version (e.g., 1.0.0)".to_string(),
            // %%FIELD:TAGS_SPECIFICATION:END%%

            // %%FIELD:DOCUMENTATION_PATTERNS:START%%
            documentation_patterns: "Functions: Purpose, parameters, return value, side effects\nClasses: Purpose, usage pattern, key methods\nInterfaces: Contract definition, implementation requirements\nTypes: Type definition, usage scenarios\nEnums: Enumeration purpose, value meanings".to_string(),
            // %%FIELD:DOCUMENTATION_PATTERNS:END%%

            // %%FIELD:EXECUTION_COMMAND:START%%
            execution_command: "Execute. Return documented code only.".to_string(),
            // %%FIELD:EXECUTION_COMMAND:END%%
        }
    }

    /// Generate the complete template from fields
    pub fn to_template(&self) -> String {
        format!(
            r#"TASK: {task}

INPUT:
{input_format}

CODE:
{{code}}

OUTPUT REQUIREMENTS:
{output_requirements}

TAGS SPECIFICATION:
{tags_specification}

DOCUMENTATION PATTERNS:
{documentation_patterns}

{execution_command}"#,
            task = self.task,
            input_format = self.input_format,
            output_requirements = self.output_requirements,
            tags_specification = self.tags_specification,
            documentation_patterns = self.documentation_patterns,
            execution_command = self.execution_command
        )
    }

    /// Generate template with field markers for sed editing
    pub fn to_sed_template(&self) -> String {
        format!(
            r#"TASK: %%FIELD:TASK:START%%{task}%%FIELD:TASK:END%%

INPUT:
%%FIELD:INPUT_FORMAT:START%%{input_format}%%FIELD:INPUT_FORMAT:END%%

CODE:
{{code}}

OUTPUT REQUIREMENTS:
%%FIELD:OUTPUT_REQUIREMENTS:START%%{output_requirements}%%FIELD:OUTPUT_REQUIREMENTS:END%%

TAGS SPECIFICATION:
%%FIELD:TAGS_SPECIFICATION:START%%{tags_specification}%%FIELD:TAGS_SPECIFICATION:END%%

DOCUMENTATION PATTERNS:
%%FIELD:DOCUMENTATION_PATTERNS:START%%{documentation_patterns}%%FIELD:DOCUMENTATION_PATTERNS:END%%

%%FIELD:EXECUTION_COMMAND:START%%{execution_command}%%FIELD:EXECUTION_COMMAND:END%%"#,
            task = self.task,
            input_format = self.input_format,
            output_requirements = self.output_requirements,
            tags_specification = self.tags_specification,
            documentation_patterns = self.documentation_patterns,
            execution_command = self.execution_command
        )
    }
}

/// AI-optimized prompt template for TSDoc generation
pub fn get_tsdoc_prompt_template() -> PromptTemplate {
    let fields = TSDocTemplateFields::new();
    PromptTemplate::new("tsdoc_enhancement", &fields.to_template())
}

/// Get template with sed-friendly field markers
pub fn get_tsdoc_sed_template() -> String {
    TSDocTemplateFields::new().to_sed_template()
}

/// Sed command examples for field editing
pub const TSDOC_SED_EXAMPLES: &str = r#"
# Edit task field:
sed -i 's/%%FIELD:TASK:START%%.*%%FIELD:TASK:END%%/%%FIELD:TASK:START%%NEW_TASK_TEXT%%FIELD:TASK:END%%/' template.txt

# Edit output requirements:
sed -i '/%%FIELD:OUTPUT_REQUIREMENTS:START%%/,/%%FIELD:OUTPUT_REQUIREMENTS:END%%/c\
%%FIELD:OUTPUT_REQUIREMENTS:START%%\
1. NEW_REQUIREMENT_1\
2. NEW_REQUIREMENT_2\
%%FIELD:OUTPUT_REQUIREMENTS:END%%' template.txt

# Edit tags specification:
sed -i '/%%FIELD:TAGS_SPECIFICATION:START%%/,/%%FIELD:TAGS_SPECIFICATION:END%%/c\
%%FIELD:TAGS_SPECIFICATION:START%%\
@category: NEW_CATEGORIES\
@safe: NEW_SAFE_LEVELS\
%%FIELD:TAGS_SPECIFICATION:END%%' template.txt

# Edit documentation patterns:
sed -i '/%%FIELD:DOCUMENTATION_PATTERNS:START%%/,/%%FIELD:DOCUMENTATION_PATTERNS:END%%/c\
%%FIELD:DOCUMENTATION_PATTERNS:START%%\
Functions: NEW_PATTERN\
Classes: NEW_PATTERN\
%%FIELD:DOCUMENTATION_PATTERNS:END%%' template.txt

# Edit execution command:
sed -i 's/%%FIELD:EXECUTION_COMMAND:START%%.*%%FIELD:EXECUTION_COMMAND:END%%/%%FIELD:EXECUTION_COMMAND:START%%NEW_COMMAND%%FIELD:EXECUTION_COMMAND:END%%/' template.txt
"#;

/// Moon.yml task configuration with field markers for sed editing
pub const TSDOC_MOON_TASKS: &str = r#"
# %%FIELD:MOON_TASKS:START%%
tasks:
  tsdoc:generate:
    command: "moon ext moon-shine tsdoc --file"
    args: ["$file"]
    description: "Generate TSDoc using Claude+DSPy integration"
    inputs:
      - "src/**/*.ts"
      - "src/**/*.tsx"
    outputs:
      - "src/**/*.ts"
      - "src/**/*.tsx"

  tsdoc:optimize:
    command: "moon ext moon-shine tsdoc --optimize"
    description: "Optimize TSDoc prompts using COPRO"
    deps: ["tsdoc:generate"]

  tsdoc:check-coverage:
    command: "moon ext moon-shine tsdoc --check-coverage"
    description: "Check TSDoc coverage across all TypeScript files"
    inputs:
      - "src/**/*.ts"
      - "src/**/*.tsx"

  tsdoc:version:
    command: "moon ext moon-shine tsdoc --version"
    description: "Show TSDoc template version and metadata"

  tsdoc:sed-edit:
    command: "moon ext moon-shine tsdoc --sed-template"
    description: "Generate sed-editable template with field markers"
# %%FIELD:MOON_TASKS:END%%
"#;

/// Generate changelog entry for version tracking
pub fn generate_changelog_entry(version: &TSDocVersion, changes: &[String]) -> String {
    format!(
        r#"
## Version {version} - {date}

### Changes:
{changes}

### Metadata:
- Change Count: {change_count}
- Template Checksum: {checksum}
- Commit Hash: {commit_hash}

---
"#,
        version = version.version,
        date = version.updated_at,
        changes = changes.iter().map(|c| format!("- {}", c)).collect::<Vec<_>>().join("\n"),
        change_count = version.change_count,
        checksum = version.template_checksum,
        commit_hash = version.commit_hash.as_deref().unwrap_or("N/A")
    )
}

/// Configuration file template with field markers for sed editing
pub const TSDOC_CONFIG_TEMPLATE: &str = r#"
# TSDoc Configuration Template
# %%FIELD:CONFIG_VERSION:START%%
version = "1.0.1"
updated_at = "2024-01-01T00:00:00Z"
change_count = 1
# %%FIELD:CONFIG_VERSION:END%%

# %%FIELD:CONFIG_TARGETS:START%%
target_coverage = 90
claude_model = "claude-3-5-sonnet-20241022"
# %%FIELD:CONFIG_TARGETS:END%%

# %%FIELD:CONFIG_PATTERNS:START%%
include_patterns = ["**/*.ts", "**/*.tsx"]
exclude_patterns = ["**/*.test.ts", "**/*.test.tsx", "**/dist/**", "**/node_modules/**"]
scan_directories = ["src"]
# %%FIELD:CONFIG_PATTERNS:END%%

# %%FIELD:CONFIG_OPTIMIZATION:START%%
use_copro_optimization = true
use_vector_similarity = true
embedding_model = "text-embedding-3-small"
max_similarity_examples = 5
similarity_threshold = 0.8
# %%FIELD:CONFIG_OPTIMIZATION:END%%
"#;

/// Legacy Node.js TSDoc checker (for compatibility)
pub const TSDOC_TEMPLATE: &str = r#"#!/usr/bin/env node

/**
 * @fileoverview Legacy TSDoc Coverage Checker (Compatibility Mode)
 *
 * This is maintained for compatibility. The new Claude+DSPy integration
 * provides superior TSDoc generation via moon-shine extension.
 *
 * Comprehensive TypeScript documentation coverage checker for
 * @claude-zen packages. Analyzes TypeScript files for JSDoc/TSDoc
 * coverage and provides detailed reports.
 *

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_typescript_coverage() {
        let generator = TSDocGenerator::new();
        let comprehensive_code = r#"
// All TypeScript constructs for complete coverage testing
export function syncFunction(param: string): string {}
export async function asyncFunction(data: Data): Promise<Result> {}
export const arrowFunction = (x: number) => x * 2;
export class DataProcessor {
    public process(data: string): void {}
    private validate(input: string): boolean {}
    protected transform(data: any): any {}
    static getInstance(): DataProcessor {}
    get status(): string {}
    set status(value: string) {}
}
export abstract class BaseProcessor {}
export interface ProcessorConfig { timeout: number; }
export type ResultType = Success | Error;
export enum ProcessingStatus { IDLE, PROCESSING, COMPLETE }
export namespace Validators { export function isValid(): boolean {} }
export const DEFAULT_TIMEOUT = 5000;
@Component export class DecoratedClass {}
export default class DefaultExportClass {}
        "#;

        let missing_count: u32 = generator.calculate_missing_count(comprehensive_code).parse().unwrap();
        assert!(missing_count > 15); // Detects all major TS constructs
    }

    #[test]
    fn test_ai_optimized_prompt() {
        let template = get_tsdoc_prompt_template();
        assert!(template.template.contains("TASK:"));
        assert!(template.template.contains("Execute. Return documented code only."));
        assert!(!template.template.contains("please"));
    }

    #[test]
    fn test_vector_similarity() {
        let generator = TSDocGenerator::new();
        let identical = vec![1.0, 0.0, 0.0];
        let orthogonal = vec![0.0, 1.0, 0.0];

        assert!((generator.calculate_similarity(&identical, &identical) - 1.0).abs() < 0.001);
        assert!((generator.calculate_similarity(&identical, &orthogonal) - 0.0).abs() < 0.001);
    }
}
 *
 * Features:
 * - Detects exports (functions, classes, interfaces, types, constants)
 * - Validates JSDoc presence and quality
 * - Generates coverage reports with actionable insights
 * - Supports multiple file analysis
 * - Configurable coverage thresholds
 *
 * @author Claude Code Zen Team
 * @since 1.0.0
 * @version 1.0.0
 */

import fs from 'node:fs';
import path from 'node:path';

// const __filename = fileURLToPath(import.meta.url); // Not used
// const __dirname = path.dirname(__filename); // Not used

/**
 * Configuration for TSDoc coverage checking
 */
const CONFIG = {
  /** Minimum coverage threshold for success */
  COVERAGE_THRESHOLD: 90,
  /** File patterns to include */
  INCLUDE_PATTERNS: ['**/*.ts', '**/*.tsx'],
  /** File patterns to exclude */
  EXCLUDE_PATTERNS: [
    '**/*.test.ts',
    '**/*.test.tsx',
    '**/dist/**',
    '**/node_modules/**',
  ],
  /** Directories to scan by default */
  DEFAULT_SCAN_DIRS: ['src'],
  /** Output formatting options */
  OUTPUT: {
    showUndocumented: true,
    showDocumented: false,
    colorOutput: true,
    verbose: false,
  },
};

/**
 * ANSI color codes for terminal output
 */
const COLORS = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
};

/**
 * Colorizes text for terminal output
 */
function colorize(text, color) {
  if (!CONFIG.OUTPUT.colorOutput) return text;
  return `${COLORS[color]}${text}${COLORS.reset}`;
}

/**
 * Export detection result
 */
class ExportInfo {
  constructor(name, type, line, hasJSDoc = false, jsdocQuality = 'none') {
    this.name = name;
    this.type = type;
    this.line = line;
    this.hasJSDoc = hasJSDoc;
    this.jsdocQuality = jsdocQuality;
  }
}

/**
 * File analysis result
 */
class FileAnalysis {
  constructor(filePath) {
    this.filePath = filePath;
    this.exports = [];
    this.documented = [];
    this.undocumented = [];
    this.coverage = 0;
    this.quality = 'unknown';
  }

  /**
   * Calculates coverage percentage
   */
  calculateCoverage() {
    if (this.exports.length === 0) {
      this.coverage = 100;
      return this.coverage;
    }
    this.coverage = Math.round(
        (this.documented.length / this.exports.length) * 100,
    );
    return this.coverage;
  }

  /**
   * Determines documentation quality rating
   */
  assessQuality() {
    const coverage = this.calculateCoverage();
    if (coverage === 100) this.quality = 'excellent';
    else if (coverage >= 90) this.quality = 'good';
    else if (coverage >= 75) this.quality = 'fair';
    else if (coverage >= 50) this.quality = 'poor';
    else this.quality = 'critical';
    return this.quality;
  }
}

/**
 * Checks if a line contains JSDoc documentation
 */
function hasJSDocAbove(lines, exportLineIndex) {
  let hasJSDoc = false;
  let jsdocStart = -1;
  let jsdocEnd = -1;

  // Look backwards from export line to find JSDoc
  for (
    let j = exportLineIndex - 1;
    j >= Math.max(0, exportLineIndex - 100);
    j--
  ) {
    const line = lines[j].trim();

    if (line.endsWith('*/') && jsdocEnd === -1) {
      jsdocEnd = j;
    }

    if (line.startsWith('/**') && jsdocEnd !== -1) {
      jsdocStart = j;
      hasJSDoc = true;
      break;
    }

    // Stop if we hit non-comment, non-empty line
    if (
      line !== '' &&
      !line.startsWith('*') &&
      !line.startsWith('//') &&
      !line.startsWith('/**') &&
      !line.endsWith('*/')
    ) {
      break;
    }
  }

  let quality = 'none';
  if (hasJSDoc && jsdocStart !== -1 && jsdocEnd !== -1) {
    const jsdocLines = lines.slice(jsdocStart, jsdocEnd + 1);
    const jsdocContent = jsdocLines.join('\n');

    // Assess JSDoc quality
    if (
      jsdocContent.includes('@param') ||
      jsdocContent.includes('@returns') ||
      jsdocContent.includes('@example')
    ) {
      quality = 'comprehensive';
    } else if (jsdocContent.length > 100) {
      quality = 'detailed';
    } else {
      quality = 'basic';
    }
  }

  return {hasJSDoc, quality};
}

/**
 * Analyzes a TypeScript file for documentation coverage
 */
function analyzeFile(filePath) {
  const analysis = new FileAnalysis(filePath);

  if (!fs.existsSync(filePath)) {
    console.warn(colorize(`⚠️  File not found: ${filePath}`, 'yellow'));
    return analysis;
  }

  const content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');

  // Regex patterns for different export types
  const exportPatterns = [
    {type: 'interface', regex: /^export\s+interface\s+(?<name>\w+)/u},
    {type: 'type', regex: /^export\s+type\s+(?<name>\w+)/u},
    {type: 'class', regex: /^export\s+class\s+(?<name>\w+)/u},
    {type: 'function', regex: /^export\s+function\s+(?<name>\w+)/u},
    {type: 'const', regex: /^export\s+const\s+(?<name>\w+)/u},
    {type: 'let', regex: /^export\s+let\s+(?<name>\w+)/u},
    {type: 'var', regex: /^export\s+var\s+(?<name>\w+)/u},
    {type: 'enum', regex: /^export\s+enum\s+(?<name>\w+)/u},
    {type: 'namespace', regex: /^export\s+namespace\s+(?<name>\w+)/u},
  ];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();

    for (const pattern of exportPatterns) {
      const match = pattern.regex.exec(line);
      if (match) {
        const exportName = match.groups?.name ?? match[1];
        const {hasJSDoc, quality} = hasJSDocAbove(lines, i);

        const exportInfo = new ExportInfo(
            exportName,
            pattern.type,
            i + 1,
            hasJSDoc,
            quality,
        );
        analysis.exports.push(exportInfo);

        if (hasJSDoc) {
          analysis.documented.push(exportInfo);
        } else {
          analysis.undocumented.push(exportInfo);
        }
        break;
      }
    }
  }

  analysis.calculateCoverage();
  analysis.assessQuality();

  return analysis;
}

/**
 * Generates a detailed report for a single file
 */
function generateFileReport(analysis) {
  const fileName = path.basename(analysis.filePath);
  const {coverage} = analysis;
  const {quality} = analysis;


  console.log(`\n📄 ${colorize(fileName, 'cyan')}`);

  console.log('─'.repeat(50));

  console.log(`Total exports: ${analysis.exports.length}`);

  console.log(`Documented: ${colorize(analysis.documented.length, 'green')}`);

  console.log(
      `Coverage: ${colorize(`${coverage}%`, coverage >= CONFIG.COVERAGE_THRESHOLD ? 'green' : 'red')}`,
  );

  console.log(
      `Quality: ${colorize(quality, quality === 'excellent' ? 'green' : quality === 'good' ? 'blue' : 'yellow')}`,
  );

  // Show undocumented exports
  if (CONFIG.OUTPUT.showUndocumented && analysis.undocumented.length > 0) {
    console.log(
        `\n${colorize(`❌ Missing TSDoc (${analysis.undocumented.length}):`, 'red')}`,
    );
    analysis.undocumented.forEach((exp) => {
      console.log(
          `   • ${colorize(exp.name, 'yellow')} (${exp.type}, line ${exp.line})`,
      );
    });
  }

  // Show documented exports if requested
  if (CONFIG.OUTPUT.showDocumented && analysis.documented.length > 0) {
    console.log(
        `\n${colorize(`✅ Documented exports (${analysis.documented.length}):`, 'green')}`,
    );
    analysis.documented.forEach((exp) => {
      const qualityColor =
        exp.jsdocQuality === 'comprehensive' ?
          'green' :
          exp.jsdocQuality === 'detailed' ?
            'blue' :
            'yellow';

      console.log(
          `   • ${exp.name} (${colorize(exp.jsdocQuality, qualityColor)})`,
      );
    });
  }

  return analysis;
}

/**
 * Generates overall summary report
 */
function generateSummaryReport(analyses) {
  const totalFiles = analyses.length;
  const totalExports = analyses.reduce((sum, a) => sum + a.exports.length, 0);
  const totalDocumented = analyses.reduce(
      (sum, a) => sum + a.documented.length,
      0,
  );
  const overallCoverage =
    totalExports > 0 ? Math.round((totalDocumented / totalExports) * 100) : 100;


  console.log(`\n${'═'.repeat(60)}`);

  console.log(colorize('📊 TSDOC COVERAGE SUMMARY', 'bright'));

  console.log('═'.repeat(60));

  console.log(`Files analyzed: ${totalFiles}`);

  console.log(`Total exports: ${totalExports}`);

  console.log(`Total documented: ${colorize(totalDocumented, 'green')}`);

  console.log(
      `Overall coverage: ${colorize(`${overallCoverage}%`, overallCoverage >= CONFIG.COVERAGE_THRESHOLD ? 'green' : 'red')}`,
  );

  // Quality assessment
  let qualityRating;
  let emoji;
  if (overallCoverage === 100) {
    qualityRating = 'PERFECT';
    emoji = '🏆';
  } else if (overallCoverage >= 95) {
    qualityRating = 'EXCELLENT';
    emoji = '🥇';
  } else if (overallCoverage >= 90) {
    qualityRating = 'VERY GOOD';
    emoji = '🥈';
  } else if (overallCoverage >= 75) {
    qualityRating = 'GOOD';
    emoji = '🥉';
  } else if (overallCoverage >= 50) {
    qualityRating = 'NEEDS IMPROVEMENT';
    emoji = '📝';
  } else {
    qualityRating = 'CRITICAL';
    emoji = '🚨';
  }


  console.log(
      `\n${emoji} ${colorize(qualityRating, 'bright')} DOCUMENTATION COVERAGE! ${emoji}`,
  );

  // File breakdown by quality
  const qualityBreakdown = {};
  analyses.forEach((analysis) => {
    qualityBreakdown[analysis.quality] =
      (qualityBreakdown[analysis.quality] || 0) + 1;
  });

  if (Object.keys(qualityBreakdown).length > 1) {
    console.log(`\n📈 File Quality Breakdown:`);
    Object.entries(qualityBreakdown).forEach(([quality, count]) => {
      const color =
        quality === 'excellent' ?
          'green' :
          quality === 'good' ?
            'blue' :
            quality === 'fair' ?
              'yellow' :
              'red';

      console.log(`   ${colorize(quality, color)}: ${count} files`);
    });
  }

  return {
    totalFiles,
    totalExports,
    totalDocumented,
    overallCoverage,
    qualityRating,
    meetsThreshold: overallCoverage >= CONFIG.COVERAGE_THRESHOLD,
  };
}

/**
 * Gets TypeScript files to analyze
 */
function getFilesToAnalyze(
    directories = CONFIG.DEFAULT_SCAN_DIRS,
    excludeDtsFiles = false,
) {
  const files = [];

  for (const dir of directories) {
    if (fs.existsSync(dir)) {
      const entries = fs.readdirSync(dir, {withFileTypes: true});

      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);

        if (
          entry.isDirectory() &&
          !entry.name.startsWith('.') &&
          entry.name !== 'node_modules'
        ) {
          // Recursively scan subdirectories
          files.push(...getFilesToAnalyze([fullPath], excludeDtsFiles));
        } else if (
          entry.isFile() &&
          entry.name.endsWith('.ts') &&
          !entry.name.endsWith('.test.ts')
        ) {
          // Exclude .d.ts files if requested
          if (excludeDtsFiles && entry.name.endsWith('.d.ts')) {
            // Skip .d.ts files when exclude flag is set
            continue;
          }
          files.push(fullPath);
        }
      }
    }
  }

  return files;
}

/**
 * Main execution function
 */
function main() {
  const args = process.argv.slice(2);

  // Parse command line arguments
  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
${colorize('TSDoc Coverage Checker', 'bright')}
${colorize('Usage:', 'blue')} node check-tsdoc.mjs [options] [files...]

${colorize('Options:', 'blue')}
  --threshold <number>    Set coverage threshold (default: ${CONFIG.COVERAGE_THRESHOLD})
  --show-documented      Show documented exports in report
  --verbose              Enable verbose output
  --no-color             Disable colored output
  --help, -h             Show this help message

${colorize('Examples:', 'blue')}
  node check-tsdoc.mjs                    # Check all TypeScript files
  node check-tsdoc.mjs src/index.ts       # Check specific file
  node check-tsdoc.mjs --threshold 95     # Set 95% threshold
    `);
    process.exit(0);
  }

  // Parse options
  if (args.includes('--show-documented')) {
    CONFIG.OUTPUT.showDocumented = true;
  }
  if (args.includes('--verbose')) {
    CONFIG.OUTPUT.verbose = true;
  }
  if (args.includes('--no-color')) {
    CONFIG.OUTPUT.colorOutput = false;
  }

  // Exclude .d.ts files option
  let excludeDtsFiles = false;
  if (args.includes('--exclude-dts')) {
    excludeDtsFiles = true;
  }

  const thresholdIndex = args.indexOf('--threshold');
  if (thresholdIndex !== -1 && args[thresholdIndex + 1]) {
    CONFIG.COVERAGE_THRESHOLD = parseInt(args[thresholdIndex + 1], 10);
  }

  // Get files to analyze
  const fileArgs = args.filter(
      (arg) => !arg.startsWith('--') && !arg.match(/^\d+$/),
  );
  const filesToAnalyze =
    fileArgs.length > 0 ?
      fileArgs :
      getFilesToAnalyze(CONFIG.DEFAULT_SCAN_DIRS, excludeDtsFiles);

  if (filesToAnalyze.length === 0) {
    console.warn(
      colorize('⚠️  No TypeScript files found to analyze', 'yellow'),
    );
    process.exit(1);
  }


  console.log(colorize('🔍 TSDoc Coverage Analysis', 'bright'));

  console.log(colorize(`Analyzing ${filesToAnalyze.length} files...`, 'blue'));

  // Analyze all files
  const analyses = filesToAnalyze.map(analyzeFile);

  // Check if this is a basic check (default threshold of 90)
  const isBasicCheck =
    CONFIG.COVERAGE_THRESHOLD === 90 && !args.includes('--threshold');

  // Generate reports
  analyses.forEach(generateFileReport);
  const summary = generateSummaryReport(analyses);

  // Add helpful message for basic check
  if (isBasicCheck && summary.overallCoverage < 100) {
    console.log(`\n${'─'.repeat(60)}`);

    console.log(
        colorize('💡 TIP: For stricter documentation requirements', 'cyan'),
    );

    console.log(
        colorize('Run: pnpm docs:check-strict', 'bright') +
      colorize(' (requires 100% coverage)', 'cyan'),
    );

    console.log(colorize('Or:  pnpm docs:check --threshold 100', 'bright'));

    if (summary.overallCoverage >= 90) {
      console.log(
          `\n${colorize('🎯 Current coverage is good!', 'green')} Consider aiming for 100% with strict mode.`,
      );
    } else {
      console.log(
          `\n${colorize('📈 Improve coverage first', 'yellow')}, then try strict mode for perfection.`,
      );
    }
  }

  // Exit with appropriate code
  process.exit(summary.meetsThreshold ? 0 : 1);
}

// Run the script
if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}

export {analyzeFile, CONFIG, generateFileReport, generateSummaryReport};
"#;

/// Package.json scripts configuration for TSDoc integration
pub const TSDOC_PACKAGE_SCRIPTS: &str = r#"{
  "scripts": {
    "docs:check": "node scripts/check-tsdoc.mjs",
    "docs:check-strict": "node scripts/check-tsdoc.mjs --threshold 100",
    "docs:check-verbose": "node scripts/check-tsdoc.mjs --verbose --show-documented"
  }
}"#;

/// Moon.yml task configuration for TSDoc integration
pub const TSDOC_MOON_TASKS: &str = r#"tasks:
  docs:check:
    command: "node scripts/check-tsdoc.mjs"
    description: "Check TypeScript documentation coverage (90% threshold)"

  docs:check-strict:
    command: "node scripts/check-tsdoc.mjs --threshold 100"
    description: "Check TypeScript documentation coverage (100% threshold)"

  docs:check-verbose:
    command: "node scripts/check-tsdoc.mjs --verbose --show-documented"
    description: "Verbose TypeScript documentation coverage check"
"#;

/// Configuration struct for TSDoc template customization
#[derive(Debug, Clone)]
pub struct TsDocConfig {
    /// Coverage threshold percentage (default: 90)
    pub coverage_threshold: u8,
    /// Directories to scan for TypeScript files
    pub scan_directories: Vec<String>,
    /// File patterns to include
    pub include_patterns: Vec<String>,
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Whether to show documented exports in output
    pub show_documented: bool,
    /// Whether to enable verbose output
    pub verbose: bool,
    /// Whether to enable colored output
    pub color_output: bool,
}

impl Default for TsDocConfig {
    fn default() -> Self {
        Self {
            coverage_threshold: 90,
            scan_directories: vec!["src".to_string()],
            include_patterns: vec!["**/*.ts".to_string(), "**/*.tsx".to_string()],
            exclude_patterns: vec![
                "**/*.test.ts".to_string(),
                "**/*.test.tsx".to_string(),
                "**/dist/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            show_documented: false,
            verbose: false,
            color_output: true,
        }
    }
}

impl TsDocConfig {
    /// Creates a new TSDoc configuration with custom settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the coverage threshold
    pub fn with_threshold(mut self, threshold: u8) -> Self {
        self.coverage_threshold = threshold;
        self
    }

    /// Adds a scan directory
    pub fn with_scan_dir(mut self, dir: &str) -> Self {
        self.scan_directories.push(dir.to_string());
        self
    }

    /// Enables strict mode (100% coverage threshold)
    pub fn strict_mode(mut self) -> Self {
        self.coverage_threshold = 100;
        self
    }

    /// Enables verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self.show_documented = true;
        self
    }
}

/// Generates a customized TSDoc script based on configuration
pub fn generate_tsdoc_script(config: &TsDocConfig) -> String {
    let mut script = TSDOC_TEMPLATE.to_string();

    // Replace default configuration values
    script = script.replace(
        "COVERAGE_THRESHOLD: 90,",
        &format!("COVERAGE_THRESHOLD: {},", config.coverage_threshold)
    );

    script = script.replace(
        "DEFAULT_SCAN_DIRS: ['src'],",
        &format!("DEFAULT_SCAN_DIRS: [{}],",
            config.scan_directories
                .iter()
                .map(|d| format!("'{}'", d))
                .collect::<Vec<_>>()
                .join(", ")
        )
    );

    script = script.replace(
        "showDocumented: false,",
        &format!("showDocumented: {},", config.show_documented)
    );

    script = script.replace(
        "verbose: false,",
        &format!("verbose: {},", config.verbose)
    );

    script = script.replace(
        "colorOutput: true,",
        &format!("colorOutput: {},", config.color_output)
    );

    script
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TsDocConfig::default();
        assert_eq!(config.coverage_threshold, 90);
        assert_eq!(config.scan_directories, vec!["src"]);
        assert!(!config.show_documented);
        assert!(!config.verbose);
        assert!(config.color_output);
    }

    #[test]
    fn test_config_builder() {
        let config = TsDocConfig::new()
            .with_threshold(95)
            .with_scan_dir("lib")
            .verbose();

        assert_eq!(config.coverage_threshold, 95);
        assert!(config.scan_directories.contains(&"lib".to_string()));
        assert!(config.verbose);
        assert!(config.show_documented);
    }

    #[test]
    fn test_strict_mode() {
        let config = TsDocConfig::new().strict_mode();
        assert_eq!(config.coverage_threshold, 100);
    }

    #[test]
    fn test_generate_script_customization() {
        let config = TsDocConfig::new()
            .with_threshold(95)
            .verbose();

        let script = generate_tsdoc_script(&config);
        assert!(script.contains("COVERAGE_THRESHOLD: 95,"));
        assert!(script.contains("showDocumented: true,"));
        assert!(script.contains("verbose: true,"));
    }
}