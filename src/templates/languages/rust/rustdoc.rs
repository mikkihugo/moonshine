/*!
 * RustDoc Generation Template with Claude+DSPy Integration
 *
 * Advanced Rust documentation generation template for PrimeCode packages.
 * This template integrates directly with Claude via DSPy for intelligent RustDoc generation,
 * using the existing prompt infrastructure and optimization capabilities.
 */

use crate::dspy::{field, sign, example, prediction, MetaSignature, Example, Prediction};
use crate::error::{Error, Result};
use crate::prompts::PromptTemplate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Version and metadata tracking for RustDoc template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustDocVersion {
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

impl RustDocVersion {
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
            get_rustdoc_prompt_template().template,
            RUSTDOC_MOON_TASKS
        );

        let mut hasher = DefaultHasher::new();
        template_content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Configuration for Claude+DSPy RustDoc generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustDocConfig {
    /// Version tracking and metadata
    pub version: RustDocVersion,
    /// Target coverage percentage (default: 90)
    pub target_coverage: u8,
    /// Directories to scan for Rust files
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

/// Vector embedding for RustDoc examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustDocEmbedding {
    pub code_pattern: String,
    pub documentation_pattern: String,
    pub embedding: Vec<f32>,
    pub construct_type: RustConstructType,
}

/// Rust construct types for categorized documentation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RustConstructType {
    Function,
    AsyncFunction,
    Method,
    AsyncMethod,
    AssociatedFunction,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Macro,
    Const,
    Static,
    Type,
    Union,
    Closure,
    Generic,
}

impl Default for RustDocConfig {
    fn default() -> Self {
        Self {
            version: RustDocVersion::new(),
            target_coverage: 90,
            scan_directories: vec!["src".to_string()],
            include_patterns: vec!["**/*.rs".to_string()],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/*test*.rs".to_string(),
                "**/tests/**".to_string(),
                "**/benches/**".to_string(),
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

/// DSPy signature for RustDoc generation using Claude
#[derive(Debug, Clone)]
pub struct RustDocSignature {
    demos: Vec<Example>,
    instruction: String,
    prefix: String,
}

impl RustDocSignature {
    pub fn new() -> Self {
        Self {
            demos: vec![
                // Example demonstrating comprehensive RustDoc generation
                example! {
                    "file_path": "input" => "src/javascript_typescript_linter.rs",
                    "code": "input" => "pub async fn process_lint_results(results: Vec<LintResult>) -> Result<ProcessedResults> {\n    optimize_results(results).await\n}",
                    "missing_count": "input" => "1",
                    "current_coverage": "input" => "65",
                    "target_coverage": "input" => "90",
                    "documented_code": "output" => "/// Processes lint results with optimization for WASM runtime\n///\n/// This function takes raw lint results and applies optimization strategies\n/// suitable for WebAssembly execution environments, including memory\n/// management and async coordination.\n///\n/// # Arguments\n///\n/// * `results` - Vector of lint results from analysis phase\n///\n/// # Returns\n///\n/// * `Result<ProcessedResults>` - Optimized results or error\n///\n/// # Errors\n///\n/// Returns error if optimization fails or memory constraints exceeded\n///\n/// # Examples\n///\n/// ```rust\n/// let results = vec![LintResult::new(\"warning\")];\n/// let processed = process_lint_results(results).await?;\n/// ```\npub async fn process_lint_results(results: Vec<LintResult>) -> Result<ProcessedResults> {\n    optimize_results(results).await\n}"
                }
            ],
            instruction: "Generate RustDoc documentation for Rust constructs. Add /// comments with comprehensive documentation including examples, errors, and safety notes.".to_string(),
            prefix: "RustDoc Documentation Enhancement".to_string(),
        }
    }
}

impl MetaSignature for RustDocSignature {
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
            input["Rust file path"] => file_path: String,
            input["Source code requiring documentation"] => code: String,
            input["Number of items missing documentation"] => missing_count: String,
            input["Current RustDoc coverage percentage"] => current_coverage: String,
            input["Target coverage percentage"] => target_coverage: String
        }
    }

    fn output_fields(&self) -> Value {
        field! {
            output["Code with comprehensive RustDoc documentation"] => documented_code: String
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

/// RustDoc generator using Claude+DSPy integration
pub struct RustDocGenerator {
    signature: RustDocSignature,
    config: RustDocConfig,
    embeddings_cache: Vec<RustDocEmbedding>,
}

impl RustDocGenerator {
    /// Creates a new RustDoc generator with default configuration
    pub fn new() -> Self {
        let mut config = RustDocConfig::default();
        config.version.increment(); // Auto-increment on creation

        Self {
            signature: RustDocSignature::new(),
            config,
            embeddings_cache: Self::load_embeddings_cache(),
        }
    }

    /// Creates a new RustDoc generator with custom configuration
    pub fn with_config(mut config: RustDocConfig) -> Self {
        let mut signature = RustDocSignature::new();

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
    pub fn update_config(&mut self, new_config: RustDocConfig) -> Result<()> {
        // Check if configuration actually changed
        let old_checksum = &self.config.version.template_checksum;
        let new_checksum = RustDocVersion::calculate_template_checksum();

        if old_checksum != &new_checksum {
            self.config = new_config;
            self.config.version.increment();

            // Log version change
            eprintln!("RustDoc template updated to version {}", self.config.version.version);
        }

        Ok(())
    }

    /// Get current template version and metadata
    pub fn get_version_info(&self) -> &RustDocVersion {
        &self.config.version
    }

    /// Force version increment (for manual updates)
    pub fn increment_version(&mut self) {
        self.config.version.increment();
    }

    /// Load pre-computed embeddings for RustDoc examples
    fn load_embeddings_cache() -> Vec<RustDocEmbedding> {
        vec![
            RustDocEmbedding {
                code_pattern: "pub async fn".to_string(),
                documentation_pattern: "async function with examples and errors".to_string(),
                embedding: vec![0.1, 0.2, 0.3],
                construct_type: RustConstructType::AsyncFunction,
            },
            RustDocEmbedding {
                code_pattern: "pub struct".to_string(),
                documentation_pattern: "struct with field documentation".to_string(),
                embedding: vec![0.2, 0.3, 0.4],
                construct_type: RustConstructType::Struct,
            },
            RustDocEmbedding {
                code_pattern: "pub trait".to_string(),
                documentation_pattern: "trait with implementation notes".to_string(),
                embedding: vec![0.3, 0.4, 0.5],
                construct_type: RustConstructType::Trait,
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
        let mut similarities: Vec<(f32, &RustDocEmbedding)> = self.embeddings_cache.iter()
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
        Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5])
    }

    /// Generates RustDoc for a Rust file using Claude via DSPy
    pub async fn generate_rustdoc(
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

    /// Identify Rust construct types in code
    fn identify_construct_types(&self, code: &str) -> Vec<String> {
        let mut types = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.contains("pub async fn") { types.push("AsyncFunction".to_string()); }
            else if trimmed.contains("pub fn") { types.push("Function".to_string()); }
            else if trimmed.contains("pub struct") { types.push("Struct".to_string()); }
            else if trimmed.contains("pub enum") { types.push("Enum".to_string()); }
            else if trimmed.contains("pub trait") { types.push("Trait".to_string()); }
            else if trimmed.contains("impl") { types.push("Impl".to_string()); }
            else if trimmed.contains("pub mod") { types.push("Module".to_string()); }
            else if trimmed.contains("macro_rules!") { types.push("Macro".to_string()); }
            else if trimmed.contains("pub const") { types.push("Const".to_string()); }
            else if trimmed.contains("pub static") { types.push("Static".to_string()); }
            else if trimmed.contains("pub type") { types.push("Type".to_string()); }
        }

        types
    }

    /// Calculates all Rust constructs missing documentation
    fn calculate_missing_count(&self, code: &str) -> String {
        let export_patterns = [
            // Functions
            r"^pub\s+fn\s+\w+",
            r"^pub\s+async\s+fn\s+\w+",
            r"^pub\s*\(\s*crate\s*\)\s+fn\s+\w+",
            // Structs and enums
            r"^pub\s+struct\s+\w+",
            r"^pub\s+enum\s+\w+",
            r"^pub\s+union\s+\w+",
            // Traits and implementations
            r"^pub\s+trait\s+\w+",
            r"^impl\s+\w+",
            r"^impl\s*<.*>\s+\w+",
            // Modules and macros
            r"^pub\s+mod\s+\w+",
            r"^macro_rules!\s+\w+",
            r"^pub\s+use\s+",
            // Constants and statics
            r"^pub\s+const\s+\w+",
            r"^pub\s+static\s+\w+",
            r"^pub\s+type\s+\w+",
            // Associated items
            r"^\s+pub\s+fn\s+\w+",
            r"^\s+pub\s+async\s+fn\s+\w+",
            r"^\s+fn\s+\w+",
            r"^\s+async\s+fn\s+\w+",
        ];

        let lines: Vec<&str> = code.lines().collect();
        let mut missing_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check if line matches export pattern
            for pattern in &export_patterns {
                if regex::Regex::new(pattern).unwrap().is_match(trimmed) {
                    // Check if RustDoc exists above this line
                    if !self.has_rustdoc_above(&lines, i) {
                        missing_count += 1;
                    }
                    break;
                }
            }
        }

        missing_count.to_string()
    }

    /// Checks if RustDoc exists above the given line
    fn has_rustdoc_above(&self, lines: &[&str], export_line: usize) -> bool {
        // Look backwards for RustDoc comment
        for i in (0..export_line).rev() {
            let line = lines[i].trim();

            if line.starts_with("///") || line.starts_with("/**") {
                return true;
            }

            // Stop if we hit non-comment, non-empty line
            if !line.is_empty() && !line.starts_with("//") && !line.starts_with("#[") {
                break;
            }
        }
        false
    }

    /// Calls Claude via existing provider router
    async fn call_claude(&self, prompt: &str) -> Result<String> {
        // This would integrate with your existing provider_router.rs
        // Placeholder for demonstration
        Ok(format!(
            "/// Generated RustDoc documentation using Claude+DSPy\n/// \n/// # Examples\n/// \n/// ```rust\n/// // Example usage\n/// ```\n{}",
            prompt
        ))
    }

    /// Updates the DSPy signature with COPRO optimization
    pub async fn optimize_with_copro(&mut self, examples: Vec<Example>) -> Result<()> {
        if !self.config.use_copro_optimization {
            return Ok(());
        }

        // Integrate with existing COPRO optimizer
        self.signature.set_demos(examples)?;

        Ok(())
    }
}

/// RustDoc template structure optimized for sed field editing
pub struct RustDocTemplateFields {
    /// Field markers for sed editing
    pub task: String,
    pub input_format: String,
    pub output_requirements: String,
    pub documentation_patterns: String,
    pub examples_format: String,
    pub safety_notes: String,
    pub execution_command: String,
}

impl RustDocTemplateFields {
    pub fn new() -> Self {
        Self {
            // %%FIELD:TASK:START%%
            task: "Generate RustDoc documentation for Rust constructs.".to_string(),
            // %%FIELD:TASK:END%%

            // %%FIELD:INPUT_FORMAT:START%%
            input_format: "File: {file_path}\nCurrent Coverage: {current_coverage}%\nTarget Coverage: {target_coverage}%\nMissing Count: {missing_count}\nConstruct Types: {construct_types}".to_string(),
            // %%FIELD:INPUT_FORMAT:END%%

            // %%FIELD:OUTPUT_REQUIREMENTS:START%%
            output_requirements: "1. Add /// comments above ALL pub items\n2. Include # Arguments, # Returns, # Errors sections\n3. Add # Examples with working code\n4. Include # Safety notes for unsafe code\n5. Target {target_coverage}% coverage minimum\n6. Return complete code with documentation".to_string(),
            // %%FIELD:OUTPUT_REQUIREMENTS:END%%

            // %%FIELD:DOCUMENTATION_PATTERNS:START%%
            documentation_patterns: "Functions: Purpose, arguments, returns, errors, examples\nStructs: Purpose, fields, usage patterns, examples\nEnums: Purpose, variants, when to use\nTraits: Contract definition, implementation notes, examples\nModules: Purpose, main components, usage guide\nMacros: Purpose, syntax, examples, limitations".to_string(),
            // %%FIELD:DOCUMENTATION_PATTERNS:END%%

            // %%FIELD:EXAMPLES_FORMAT:START%%
            examples_format: "```rust\n// Brief description\nlet example = function_call();\nassert_eq!(example.result, expected);\n```".to_string(),
            // %%FIELD:EXAMPLES_FORMAT:END%%

            // %%FIELD:SAFETY_NOTES:START%%
            safety_notes: "# Safety\n\nThis function is unsafe because [specific reason].\nCaller must ensure [specific requirements].".to_string(),
            // %%FIELD:SAFETY_NOTES:END%%

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

DOCUMENTATION PATTERNS:
{documentation_patterns}

EXAMPLES FORMAT:
{examples_format}

SAFETY NOTES FORMAT:
{safety_notes}

{execution_command}"#,
            task = self.task,
            input_format = self.input_format,
            output_requirements = self.output_requirements,
            documentation_patterns = self.documentation_patterns,
            examples_format = self.examples_format,
            safety_notes = self.safety_notes,
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

DOCUMENTATION PATTERNS:
%%FIELD:DOCUMENTATION_PATTERNS:START%%{documentation_patterns}%%FIELD:DOCUMENTATION_PATTERNS:END%%

EXAMPLES FORMAT:
%%FIELD:EXAMPLES_FORMAT:START%%{examples_format}%%FIELD:EXAMPLES_FORMAT:END%%

SAFETY NOTES FORMAT:
%%FIELD:SAFETY_NOTES:START%%{safety_notes}%%FIELD:SAFETY_NOTES:END%%

%%FIELD:EXECUTION_COMMAND:START%%{execution_command}%%FIELD:EXECUTION_COMMAND:END%%"#,
            task = self.task,
            input_format = self.input_format,
            output_requirements = self.output_requirements,
            documentation_patterns = self.documentation_patterns,
            examples_format = self.examples_format,
            safety_notes = self.safety_notes,
            execution_command = self.execution_command
        )
    }
}

/// AI-optimized prompt template for RustDoc generation
pub fn get_rustdoc_prompt_template() -> PromptTemplate {
    let fields = RustDocTemplateFields::new();
    PromptTemplate::new("rustdoc_enhancement", &fields.to_template())
}

/// Get template with sed-friendly field markers
pub fn get_rustdoc_sed_template() -> String {
    RustDocTemplateFields::new().to_sed_template()
}

/// Sed command examples for field editing
pub const RUSTDOC_SED_EXAMPLES: &str = r#"
# Edit task field:
sed -i 's/%%FIELD:TASK:START%%.*%%FIELD:TASK:END%%/%%FIELD:TASK:START%%NEW_TASK_TEXT%%FIELD:TASK:END%%/' template.txt

# Edit output requirements:
sed -i '/%%FIELD:OUTPUT_REQUIREMENTS:START%%/,/%%FIELD:OUTPUT_REQUIREMENTS:END%%/c\
%%FIELD:OUTPUT_REQUIREMENTS:START%%\
1. NEW_REQUIREMENT_1\
2. NEW_REQUIREMENT_2\
%%FIELD:OUTPUT_REQUIREMENTS:END%%' template.txt

# Edit documentation patterns:
sed -i '/%%FIELD:DOCUMENTATION_PATTERNS:START%%/,/%%FIELD:DOCUMENTATION_PATTERNS:END%%/c\
%%FIELD:DOCUMENTATION_PATTERNS:START%%\
Functions: NEW_PATTERN\
Structs: NEW_PATTERN\
%%FIELD:DOCUMENTATION_PATTERNS:END%%' template.txt

# Edit examples format:
sed -i '/%%FIELD:EXAMPLES_FORMAT:START%%/,/%%FIELD:EXAMPLES_FORMAT:END%%/c\
%%FIELD:EXAMPLES_FORMAT:START%%\
```rust\
// NEW_EXAMPLE_FORMAT\
```\
%%FIELD:EXAMPLES_FORMAT:END%%' template.txt
"#;

/// Moon.yml task configuration with field markers for sed editing
pub const RUSTDOC_MOON_TASKS: &str = r#"
# %%FIELD:MOON_TASKS:START%%
tasks:
  rustdoc:generate:
    command: "moon ext moon-shine rustdoc --file"
    args: ["$file"]
    description: "Generate RustDoc using Claude+DSPy integration"
    inputs:
      - "src/**/*.rs"
    outputs:
      - "src/**/*.rs"

  rustdoc:optimize:
    command: "moon ext moon-shine rustdoc --optimize"
    description: "Optimize RustDoc prompts using COPRO"
    deps: ["rustdoc:generate"]

  rustdoc:check-coverage:
    command: "moon ext moon-shine rustdoc --check-coverage"
    description: "Check RustDoc coverage across all Rust files"
    inputs:
      - "src/**/*.rs"

  rustdoc:version:
    command: "moon ext moon-shine rustdoc --version"
    description: "Show RustDoc template version and metadata"

  rustdoc:sed-edit:
    command: "moon ext moon-shine rustdoc --sed-template"
    description: "Generate sed-editable template with field markers"
# %%FIELD:MOON_TASKS:END%%
"#;

/// Generate changelog entry for version tracking
pub fn generate_rustdoc_changelog_entry(version: &RustDocVersion, changes: &[String]) -> String {
    format!(
        r#"
## RustDoc Version {version} - {date}

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
pub const RUSTDOC_CONFIG_TEMPLATE: &str = r#"
# RustDoc Configuration Template
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
include_patterns = ["**/*.rs"]
exclude_patterns = ["**/target/**", "**/*test*.rs", "**/tests/**", "**/benches/**"]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustdoc_config_default() {
        let config = RustDocConfig::default();
        assert_eq!(config.target_coverage, 90);
        assert!(config.use_vector_similarity);
        assert_eq!(config.max_similarity_examples, 5);
        assert_eq!(config.similarity_threshold, 0.8);
    }

    #[test]
    fn test_rust_construct_identification() {
        let generator = RustDocGenerator::new();
        let code = r#"
pub async fn process_data(data: &[u8]) -> Result<ProcessedData> {}
pub struct DataProcessor { field: String }
pub trait ProcessorTrait { fn process(&self); }
pub enum Status { Pending, Complete }
pub mod processor_module {}
macro_rules! generate_processor {}
pub const DEFAULT_SIZE: usize = 1024;
        "#;

        let types = generator.identify_construct_types(code);
        assert!(types.contains(&"AsyncFunction".to_string()));
        assert!(types.contains(&"Struct".to_string()));
        assert!(types.contains(&"Trait".to_string()));
        assert!(types.contains(&"Enum".to_string()));
        assert!(types.contains(&"Module".to_string()));
        assert!(types.contains(&"Macro".to_string()));
        assert!(types.contains(&"Const".to_string()));
    }

    #[test]
    fn test_rustdoc_missing_count_calculation() {
        let generator = RustDocGenerator::new();
        let code = r#"
pub fn undocumented() {}
/// Documented function
pub fn documented() {}
pub struct UndocumentedStruct {}
        "#;

        let missing = generator.calculate_missing_count(code);
        // Should detect undocumented function and struct
        assert_eq!(missing, "2");
    }

    #[test]
    fn test_comprehensive_rust_patterns() {
        let generator = RustDocGenerator::new();
        let comprehensive_code = r#"
pub fn sync_function(param: &str) -> String {}
pub async fn async_function(data: &[u8]) -> Result<Vec<u8>> {}
pub struct DataProcessor { field: String }
pub enum ProcessingStatus { Idle, Processing, Complete }
pub trait Processor { fn process(&self) -> Result<()>; }
impl Processor for DataProcessor { fn process(&self) -> Result<()> {} }
pub mod submodule { pub fn helper() {} }
macro_rules! generate_code { () => {}; }
pub const BUFFER_SIZE: usize = 4096;
pub static GLOBAL_STATE: AtomicBool = AtomicBool::new(false);
pub type ProcessResult = Result<String, ProcessError>;
        "#;

        let missing_count: u32 = generator.calculate_missing_count(comprehensive_code).parse().unwrap();
        assert!(missing_count > 10); // Should detect most Rust constructs
    }

    #[test]
    fn test_ai_optimized_rustdoc_prompt() {
        let template = get_rustdoc_prompt_template();
        assert!(template.template.contains("TASK:"));
        assert!(template.template.contains("Execute. Return documented code only."));
        assert!(!template.template.contains("please"));
    }

    #[tokio::test]
    async fn test_rustdoc_generation_workflow() {
        let config = RustDocConfig {
            target_coverage: 95,
            use_vector_similarity: true,
            max_similarity_examples: 3,
            similarity_threshold: 0.7,
            ..Default::default()
        };

        let mut generator = RustDocGenerator::with_config(config);

        let test_code = r#"
pub async fn process_wasm_lint(config: LintConfig) -> Result<LintResults> {
    execute_lint_analysis(config).await
}
        "#;

        let result = generator.generate_rustdoc(
            "src/javascript_typescript_linter.rs",
            test_code,
            75
        ).await;

        assert!(result.is_ok());
    }
}