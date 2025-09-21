/*!
 * TypeScript Documentation Templates
 *
 * Clean separation between DSPy-optimizable instructions and data payload.
 * DSPy optimizes the instruction template, not the variable data.
 */

use crate::dspy::{field, example, MetaSignature, Example};
use crate::error::Result;
use crate::templates::languages::{ProtectedTemplate, ProtectionLevel};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// DSPy-optimizable instruction template (stable, no variable data)
pub const TSDOC_INSTRUCTION_TEMPLATE: &str = r#"
%%FIELD:TASK:START%%
Generate comprehensive TSDoc documentation for TypeScript constructs.
%%FIELD:TASK:END%%

%%FIELD:OUTPUT_REQUIREMENTS:START%%
1. Add /** */ comments above ALL exports/public methods
2. Include required tags: @category @safe @mvp @complexity @since
3. Use proper @param @returns @throws tags
4. Target coverage minimum as specified
5. Return complete code with documentation
%%FIELD:OUTPUT_REQUIREMENTS:END%%

%%FIELD:TAGS_SPECIFICATION:START%%
@category: coordination|audit|validation|security|performance|integration
@safe: team|program|large-solution|portfolio
@mvp: core|extension|future
@complexity: low|medium|high|critical
@since: version (e.g., 1.0.0)
%%FIELD:TAGS_SPECIFICATION:END%%

%%FIELD:DOCUMENTATION_PATTERNS:START%%
Functions: Purpose, parameters, return value, side effects
Classes: Purpose, usage pattern, key methods
Interfaces: Contract definition, implementation requirements
Types: Type definition, usage scenarios
Enums: Enumeration purpose, value meanings
%%FIELD:DOCUMENTATION_PATTERNS:END%%

%%FIELD:EXECUTION_COMMAND:START%%
Execute. Return documented code only.
%%FIELD:EXECUTION_COMMAND:END%%
"#;

/// Data payload structure (not optimized by DSPy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocDataPayload {
    /// File path being processed
    pub file_path: String,
    /// Source code requiring documentation
    pub code: String,
    /// Current coverage percentage
    pub current_coverage: u8,
    /// Target coverage percentage
    pub target_coverage: u8,
    /// Missing documentation count
    pub missing_count: u32,
    /// Identified construct types
    pub construct_types: Vec<String>,
    /// Lint errors (NOT included in DSPy optimization)
    pub lint_errors: Option<Vec<LintError>>,
    /// Context metadata (NOT included in DSPy optimization)
    pub context: Option<ProcessingContext>,
}

/// Lint error structure (excluded from DSPy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintError {
    pub rule: String,
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub severity: String,
}

/// Processing context (excluded from DSPy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingContext {
    pub project_name: String,
    pub package_version: String,
    pub dependencies: Vec<String>,
    pub build_environment: String,
}

/// TSDoc signature that separates instructions from data
#[derive(Debug, Clone)]
pub struct TSDocSignature {
    /// DSPy-optimizable instruction template
    instruction_template: ProtectedTemplate,
    /// Example demonstrations (optimizable)
    demos: Vec<Example>,
    /// Instruction prefix (optimizable)
    prefix: String,
}

impl TSDocSignature {
    pub fn new() -> Self {
        let instruction_template = ProtectedTemplate::new(
            "tsdoc_instructions".to_string(),
            TSDOC_INSTRUCTION_TEMPLATE.to_string(),
            ProtectionLevel::DSPyOptimization,
        );

        Self {
            instruction_template,
            demos: vec![
                example! {
                    "construct_type": "input" => "AsyncFunction",
                    "example_code": "input" => "export async function createEpic(name: string, config: EpicConfig): Promise<Epic>",
                    "documented_code": "output" => "/**\n * Creates a new SAFe 6.0 epic with specified configuration\n *\n * @param name - The epic name following SAFe naming conventions\n * @param config - Epic configuration including PI mapping and dependencies\n * @returns Promise resolving to the created Epic instance\n *\n * @category coordination\n * @safe large-solution\n * @mvp core\n * @complexity medium\n * @since 1.0.0\n */\nexport async function createEpic(name: string, config: EpicConfig): Promise<Epic>"
                }
            ],
            prefix: "TSDoc Documentation Enhancement".to_string(),
        }
    }

    /// Generate complete prompt by combining instructions with data payload
    pub fn generate_complete_prompt(&self, payload: &TSDocDataPayload) -> String {
        let instruction_content = self.instruction_template.get_content();

        format!(
            r#"{instructions}

INPUT DATA:
File: {file_path}
Current Coverage: {current_coverage}%
Target Coverage: {target_coverage}%
Missing Count: {missing_count}
Construct Types: {construct_types}

CODE:
{code}"#,
            instructions = instruction_content,
            file_path = payload.file_path,
            current_coverage = payload.current_coverage,
            target_coverage = payload.target_coverage,
            missing_count = payload.missing_count,
            construct_types = payload.construct_types.join(", "),
            code = payload.code
        )
    }

    /// Apply DSPy optimization to instructions only (not data)
    pub fn optimize_instructions(&mut self, optimized_instructions: String) -> Result<()> {
        self.instruction_template.apply_dspy_optimization(optimized_instructions)?;
        Ok(())
    }

    /// Reset to base instruction template
    pub fn reset_instructions(&mut self) {
        self.instruction_template.reset_to_base();
    }

    /// Check if instructions have been optimized
    pub fn is_optimized(&self) -> bool {
        self.instruction_template.is_modified()
    }

    /// Get base (unoptimized) instructions
    pub fn get_base_instructions(&self) -> &str {
        self.instruction_template.get_base_content()
    }

    /// Get current (possibly optimized) instructions
    pub fn get_current_instructions(&self) -> &str {
        self.instruction_template.get_content()
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
        self.instruction_template.get_content().to_string()
    }

    fn input_fields(&self) -> Value {
        field! {
            input["Construct type being documented"] => construct_type: String,
            input["Example code snippet"] => example_code: String
        }
    }

    fn output_fields(&self) -> Value {
        field! {
            output["Code with comprehensive TSDoc documentation"] => documented_code: String
        }
    }

    fn update_instruction(&mut self, instruction: String) -> Result<()> {
        self.instruction_template.apply_dspy_optimization(instruction)?;
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

/// TypeScript documentation generator with clean instruction/data separation
pub struct TSDocGenerator {
    signature: TSDocSignature,
    version: String,
}

impl TSDocGenerator {
    pub fn new() -> Self {
        Self {
            signature: TSDocSignature::new(),
            version: "1.0.1".to_string(),
        }
    }

    /// Generate documentation using separated instruction template and data payload
    pub async fn generate_documentation(&mut self, payload: TSDocDataPayload) -> Result<String> {
        // Generate complete prompt by combining instructions with clean data
        let full_prompt = self.signature.generate_complete_prompt(&payload);

        // Call Claude with the complete prompt (no lint errors in optimization)
        let documented_code = self.call_claude(&full_prompt).await?;

        Ok(documented_code)
    }

    /// Optimize instructions using DSPy (data payload is never included)
    pub async fn optimize_instructions_with_dspy(&mut self, examples: Vec<Example>) -> Result<()> {
        // DSPy optimization works on instruction template only
        self.signature.set_demos(examples)?;

        // Example of what DSPy would optimize (instruction template, not data):
        let optimized_instructions = r#"
TASK: Generate TSDoc documentation for TypeScript constructs.

OUTPUT REQUIREMENTS:
1. Add /** */ comments above ALL exports/public methods
2. Include required tags: @category @safe @mvp @complexity @since
3. Use proper @param @returns @throws tags
4. Target coverage minimum as specified
5. Return complete code with documentation

TAGS SPECIFICATION:
@category: coordination|audit|validation|security|performance|integration
@safe: team|program|large-solution|portfolio
@mvp: core|extension|future
@complexity: low|medium|high|critical
@since: version (e.g., 1.0.0)

DOCUMENTATION PATTERNS:
Functions: Purpose, parameters, return value, side effects
Classes: Purpose, usage pattern, key methods
Interfaces: Contract definition, implementation requirements
Types: Type definition, usage scenarios
Enums: Enumeration purpose, value meanings

Execute. Return documented code only.
        "#;

        self.signature.optimize_instructions(optimized_instructions.to_string())?;

        Ok(())
    }

    /// Reset to base instructions (rollback DSPy optimization)
    pub fn reset_to_base_instructions(&mut self) {
        self.signature.reset_instructions();
    }

    /// Get optimization status
    pub fn get_optimization_status(&self) -> OptimizationStatus {
        OptimizationStatus {
            is_optimized: self.signature.is_optimized(),
            base_instructions: self.signature.get_base_instructions().to_string(),
            current_instructions: self.signature.get_current_instructions().to_string(),
            version: self.version.clone(),
        }
    }

    /// Create data payload from file analysis (excludes lint errors from optimization)
    pub fn create_data_payload(
        &self,
        file_path: String,
        code: String,
        current_coverage: u8,
        target_coverage: u8,
    ) -> TSDocDataPayload {
        TSDocDataPayload {
            file_path,
            code: code.clone(),
            current_coverage,
            target_coverage,
            missing_count: self.calculate_missing_count(&code),
            construct_types: self.identify_construct_types(&code),
            lint_errors: None, // Intentionally excluded from DSPy optimization
            context: None,     // Intentionally excluded from DSPy optimization
        }
    }

    /// Calculate missing documentation count
    fn calculate_missing_count(&self, code: &str) -> u32 {
        // Implementation similar to existing logic
        let patterns = [
            r"^export\s+function\s+\w+",
            r"^export\s+async\s+function\s+\w+",
            r"^export\s+class\s+\w+",
            r"^export\s+interface\s+\w+",
            r"^export\s+type\s+\w+",
            r"^export\s+enum\s+\w+",
        ];

        let lines: Vec<&str> = code.lines().collect();
        let mut missing = 0;

        for (i, line) in lines.iter().enumerate() {
            for pattern in &patterns {
                if regex::Regex::new(pattern).unwrap().is_match(line.trim()) {
                    if !self.has_tsdoc_above(&lines, i) {
                        missing += 1;
                    }
                    break;
                }
            }
        }

        missing
    }

    /// Identify TypeScript construct types
    fn identify_construct_types(&self, code: &str) -> Vec<String> {
        let mut types = Vec::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.contains("export async function") { types.push("AsyncFunction".to_string()); }
            else if trimmed.contains("export function") { types.push("Function".to_string()); }
            else if trimmed.contains("export class") { types.push("Class".to_string()); }
            else if trimmed.contains("export interface") { types.push("Interface".to_string()); }
            else if trimmed.contains("export type") { types.push("Type".to_string()); }
            else if trimmed.contains("export enum") { types.push("Enum".to_string()); }
        }
        types
    }

    /// Check if TSDoc exists above line
    fn has_tsdoc_above(&self, lines: &[&str], export_line: usize) -> bool {
        for i in (0..export_line).rev() {
            let line = lines[i].trim();
            if line.starts_with("/**") || line.starts_with("///") {
                return true;
            }
            if !line.is_empty() && !line.starts_with("//") && !line.starts_with("*") {
                break;
            }
        }
        false
    }

    /// Call Claude (placeholder for actual implementation)
    async fn call_claude(&self, prompt: &str) -> Result<String> {
        // This would integrate with your existing provider router
        Ok(format!(
            "/**\n * Generated TSDoc using optimized instructions\n */\n{}",
            prompt
        ))
    }
}

/// Optimization status for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStatus {
    pub is_optimized: bool,
    pub base_instructions: String,
    pub current_instructions: String,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_data_separation() {
        let mut generator = TSDocGenerator::new();

        let payload = generator.create_data_payload(
            "src/test.ts".to_string(),
            "export function test() {}".to_string(),
            70,
            90,
        );

        // Data payload should not affect instruction optimization
        assert_eq!(payload.file_path, "src/test.ts");
        assert_eq!(payload.current_coverage, 70);
        assert_eq!(payload.target_coverage, 90);
        assert!(payload.lint_errors.is_none()); // Excluded from optimization
        assert!(payload.context.is_none());     // Excluded from optimization

        // Instructions should be separate
        let status = generator.get_optimization_status();
        assert!(!status.is_optimized);
        assert!(status.base_instructions.contains("TASK:"));
    }

    #[test]
    fn test_dspy_optimization_instructions_only() {
        let mut generator = TSDocGenerator::new();

        // Before optimization
        let status_before = generator.get_optimization_status();
        assert!(!status_before.is_optimized);

        // Apply optimization (only affects instructions, not data)
        let examples = vec![
            example! {
                "construct_type": "input" => "Function",
                "example_code": "input" => "export function example() {}",
                "documented_code": "output" => "/** Example function */\nexport function example() {}"
            }
        ];

        tokio_test::block_on(async {
            generator.optimize_instructions_with_dspy(examples).await.unwrap();
        });

        // After optimization
        let status_after = generator.get_optimization_status();
        assert!(status_after.is_optimized);
        assert_ne!(status_after.base_instructions, status_after.current_instructions);

        // Data payload creation should be unaffected
        let payload = generator.create_data_payload(
            "test.ts".to_string(),
            "export function test() {}".to_string(),
            75,
            95,
        );

        assert_eq!(payload.target_coverage, 95); // Data unchanged by optimization
    }

    #[test]
    fn test_prompt_generation() {
        let generator = TSDocGenerator::new();

        let payload = TSDocDataPayload {
            file_path: "src/example.ts".to_string(),
            code: "export function example() {}".to_string(),
            current_coverage: 80,
            target_coverage: 90,
            missing_count: 1,
            construct_types: vec!["Function".to_string()],
            lint_errors: Some(vec![LintError {
                rule: "missing-docs".to_string(),
                message: "Missing documentation".to_string(),
                line: 1,
                column: 1,
                severity: "warning".to_string(),
            }]),
            context: None,
        };

        let prompt = generator.signature.generate_complete_prompt(&payload);

        // Prompt should contain instructions and clean data
        assert!(prompt.contains("Generate comprehensive TSDoc"));
        assert!(prompt.contains("src/example.ts"));
        assert!(prompt.contains("export function example()"));
        assert!(prompt.contains("Target Coverage: 90"));

        // Lint errors should NOT be in the prompt (excluded from optimization)
        assert!(!prompt.contains("missing-docs"));
        assert!(!prompt.contains("Missing documentation"));
    }

    #[test]
    fn test_instruction_reset() {
        let mut generator = TSDocGenerator::new();

        // Optimize instructions
        tokio_test::block_on(async {
            generator.optimize_instructions_with_dspy(vec![]).await.unwrap();
        });

        assert!(generator.get_optimization_status().is_optimized);

        // Reset to base
        generator.reset_to_base_instructions();

        let status = generator.get_optimization_status();
        assert!(!status.is_optimized);
        assert_eq!(status.base_instructions, status.current_instructions);
    }
}