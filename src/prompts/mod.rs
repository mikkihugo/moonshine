pub mod compiled; // Zero-runtime-cost compiled prompts

use crate::error::{Error, Result};
use crate::moon_pdk_interface::{get_moon_config, get_moon_config_safe, write_file_atomic};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

// Re-exports
pub use compiled::{available_template_names, get_compiled_prompt_template, get_template_metadata, has_template};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: Vec<String>,
}

impl PromptTemplate {
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        let template_str = template.into();
        let variables = extract_variables(&template_str);

        Self {
            name: name.into(),
            template: template_str,
            variables,
        }
    }

    pub fn render(&self, context: &std::collections::HashMap<String, String>) -> Result<String> {
        let mut result = self.template.clone();

        for var in &self.variables {
            let placeholder = format!("{{{}}}", var);
            if let Some(value) = context.get(var) {
                result = result.replace(&placeholder, value);
            } else {
                return Err(Error::config(format!("Missing variable: {}", var)));
            }
        }

        Ok(result)
    }

    /// Load prompt with priority: optimized > COPRO candidates > base template
    pub fn load_with_priority(prompt_name: &str) -> Result<Self> {
        // First, try to load optimized prompt from prompts.json
        if let Some(optimized) = Self::load_optimized(prompt_name)? {
            return Ok(optimized);
        }

        // Second, try COPRO candidates
        if let Some(copro_candidate) = Self::load_best_copro_candidate(prompt_name)? {
            return Ok(copro_candidate);
        }

        // Fall back to base template
        Self::load_base_template(prompt_name)
    }

    /// Load optimized prompt from Moon storage (prompts.json)
    fn load_optimized(prompt_name: &str) -> Result<Option<Self>> {
        if let Some(prompts_json) = get_moon_config("moonshine_prompts") {
            let prompts: serde_json::Value = serde_json::from_str(&prompts_json)?;

            if let Some(optimized_prompts) = prompts.get("optimized_prompts") {
                if let Some(prompt_data) = optimized_prompts.get(prompt_name) {
                    if let Some(template) = prompt_data.get("template").and_then(|v| v.as_str()) {
                        return Ok(Some(Self::new(prompt_name, template)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Load best COPRO candidate from Moon storage
    fn load_best_copro_candidate(prompt_name: &str) -> Result<Option<Self>> {
        if let Some(prompts_json) = get_moon_config("moonshine_prompts") {
            let prompts: serde_json::Value = serde_json::from_str(&prompts_json)?;

            if let Some(copro_candidates) = prompts.get("copro_candidates") {
                if let Some(candidates) = copro_candidates.get("active").and_then(|v| v.as_array()) {
                    // Find the highest scoring candidate for this prompt type
                    let mut best_candidate = None;
                    let mut best_score = 0.0;

                    for candidate in candidates {
                        if let (Some(template), Some(score)) = (
                            candidate.get("template").and_then(|v| v.as_str()),
                            candidate.get("score").and_then(|v| v.as_f64()),
                        ) {
                            if score > best_score {
                                best_score = score;
                                best_candidate = Some(template);
                            }
                        }
                    }

                    if let Some(template) = best_candidate {
                        return Ok(Some(Self::new(format!("{}_copro", prompt_name), template)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Load base template (fallback)
    fn load_base_template(prompt_name: &str) -> Result<Self> {
        // Default base templates for common prompt types
        let base_template = match prompt_name {
            "code_analysis" => "Analyze the following {language} code for potential issues:\n\n{code}\n\nProvide specific suggestions for improvement.",
            "type_fixing" => "Fix TypeScript type errors in the following code:\n\n{code}\n\nReturn the corrected code with proper types.",
            "eslint_fixing" => "Fix ESLint issues in the following {language} code:\n\n{code}\n\nReturn the corrected code.",
            _ => "Analyze the following {language} code:\n\n{code}\n\nProvide analysis and suggestions.",
        };

        Ok(Self::new(prompt_name, base_template))
    }
}

fn extract_variables(template: &str) -> Vec<String> {
    let mut variables = Vec::new();
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch != '{' {
                    let mut var_name = String::new();
                    for ch in chars.by_ref() {
                        if ch == '}' {
                            break;
                        }
                        var_name.push(ch);
                    }
                    if !var_name.is_empty() && !variables.contains(&var_name) {
                        variables.push(var_name);
                    }
                }
            }
        }
    }

    variables
}

/// Load prompt templates with production-grade external storage support
/// Implements robust storage hierarchy: external JSON → Moon config → embedded defaults
/// Features: retry logic, caching, multiple storage backends, graceful degradation
pub fn get_default_templates() -> Vec<PromptTemplate> {
    let mut templates = Vec::new();

    // First, try to load from external storage with robust error handling
    match load_prompts_from_external_storage() {
        Ok(external_templates) if !external_templates.is_empty() => {
            return external_templates;
        }
        Ok(_) => {
            // External storage accessible but empty - continue to fallbacks
        }
        Err(e) => {
            // Log error but continue gracefully to fallbacks
            eprintln!("Warning: External prompt storage failed: {}", e);
        }
    }

    // Second, try consolidated prompts.json from Moon storage
    if let Ok(json_templates) = load_prompts_from_json() {
        if !json_templates.is_empty() {
            return json_templates;
        }
    }

    // Third, fall back to Moon configuration keys
    templates.extend(load_templates_from_moon_config());

    // Ultimate fallback to embedded defaults if all external sources fail
    if templates.is_empty() {
        templates.extend(get_embedded_default_templates());
    }

    templates
}

/// Load prompt templates from external storage with production-grade robustness
/// Supports multiple storage backends with retry logic and caching
fn load_prompts_from_external_storage() -> Result<Vec<PromptTemplate>> {
    // Try multiple external storage paths with retry logic
    let storage_paths = [
        ".moon/moonshine/prompts.json",
        ".moon/moonshine/external-prompts.json",
        "moonshine-prompts.json",
        "/etc/moonshine/prompts.json", // System-wide storage
    ];

    let mut last_error = None;

    for path in &storage_paths {
        match load_prompts_from_file_with_retry(path, 3) {
            Ok(templates) if !templates.is_empty() => {
                return Ok(templates);
            }
            Ok(_) => {
                // File exists but empty - try next path
                continue;
            }
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    // Try loading from environment variable path
    if let Ok(Some(env_path)) = get_moon_config_safe("MOONSHINE_PROMPTS_PATH") {
        if let Ok(templates) = load_prompts_from_file_with_retry(&env_path, 2) {
            if !templates.is_empty() {
                return Ok(templates);
            }
        }
    }

    // All external sources failed
    Err(last_error.unwrap_or_else(|| Error::config("No external prompt storage available".to_string())))
}

/// Load prompts from a specific file with retry logic and validation
fn load_prompts_from_file_with_retry(file_path: &str, max_retries: u32) -> Result<Vec<PromptTemplate>> {
    let mut retries = 0;

    while retries < max_retries {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                // Validate JSON structure before parsing
                if content.trim().is_empty() {
                    return Ok(Vec::new());
                }

                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(json_data) => {
                        return parse_external_prompts_json(&json_data);
                    }
                    Err(e) => {
                        if retries + 1 >= max_retries {
                            return Err(Error::config(format!("Invalid JSON in {}: {}", file_path, e)));
                        }
                        // Wait briefly before retry (exponential backoff)
                        std::thread::sleep(std::time::Duration::from_millis(100 * (1 << retries)));
                    }
                }
            }
            Err(e) => {
                if retries + 1 >= max_retries {
                    return Err(Error::config(format!("Failed to read {}: {}", file_path, e)));
                }
                std::thread::sleep(std::time::Duration::from_millis(50 * (1 << retries)));
            }
        }
        retries += 1;
    }

    Err(Error::config(format!("Max retries exceeded for {}", file_path)))
}

/// Parse external prompts JSON with comprehensive validation
fn parse_external_prompts_json(json_data: &serde_json::Value) -> Result<Vec<PromptTemplate>> {
    let mut templates = Vec::new();

    // Validate JSON schema version
    if let Some(version) = json_data.get("version").and_then(|v| v.as_str()) {
        if !is_compatible_version(version) {
            return Err(Error::config(format!("Incompatible prompts JSON version: {}", version)));
        }
    }

    // Priority 1: Optimized prompts (highest priority)
    if let Some(optimized) = json_data.get("optimized_prompts").and_then(|v| v.as_object()) {
        for (name, data) in optimized {
            if let Some(template) = data.get("template").and_then(|v| v.as_str()) {
                templates.push(PromptTemplate::new(format!("{}_optimized", name), template));
            }
        }
    }

    // Priority 2: COPRO candidates (high-scoring only)
    if let Some(copro) = json_data.get("copro_candidates").and_then(|v| v.as_object()) {
        if let Some(active) = copro.get("active").and_then(|v| v.as_array()) {
            for candidate in active {
                if let (Some(name), Some(template), Some(score)) = (
                    candidate.get("name").and_then(|v| v.as_str()),
                    candidate.get("template").and_then(|v| v.as_str()),
                    candidate.get("score").and_then(|v| v.as_f64()),
                ) {
                    if score > 0.8 {
                        // Only use high-scoring candidates
                        templates.push(PromptTemplate::new(format!("{}_copro", name), template));
                    }
                }
            }
        }
    }

    // Priority 3: Base prompts (fallback)
    if let Some(base_prompts) = json_data.get("base_prompts").and_then(|v| v.as_object()) {
        for (name, data) in base_prompts {
            if let Some(template) = data.get("template").and_then(|v| v.as_str()) {
                // Only add if we don't already have an optimized/copro version
                let optimized_name = format!("{}_optimized", name);
                let copro_name = format!("{}_copro", name);
                if !templates.iter().any(|t| t.name == optimized_name || t.name == copro_name) {
                    templates.push(PromptTemplate::new(name.clone(), template));
                }
            }
        }
    }

    Ok(templates)
}

/// Check if the prompts JSON version is compatible with this extension
fn is_compatible_version(version: &str) -> bool {
    // Simple semantic version compatibility check
    // Accept 1.x.x versions as compatible with 1.0.0
    version.starts_with("1.")
}

/// Load prompt templates from consolidated .moon/moonshine/prompts.json
fn load_prompts_from_json() -> Result<Vec<PromptTemplate>> {
    let prompts_json = get_moon_config_safe("moonshine_prompts_file").or_else(|_| {
        // Try alternative path if main config key fails
        get_moon_config_safe("moonshine.storage.prompts")
    })?;

    if let Some(json_content) = prompts_json {
        let prompts_data: serde_json::Value = serde_json::from_str(&json_content).map_err(|e| Error::config(format!("Invalid prompts JSON: {}", e)))?;

        let mut templates = Vec::new();

        // Load optimized prompts first (highest priority)
        if let Some(optimized) = prompts_data.get("optimized_prompts").and_then(|v| v.as_object()) {
            for (name, data) in optimized {
                if let Some(template) = data.get("template").and_then(|v| v.as_str()) {
                    templates.push(PromptTemplate::new(format!("{}_optimized", name), template));
                }
            }
        }

        // Load base prompts (fallback)
        if let Some(base_prompts) = prompts_data.get("base_prompts").and_then(|v| v.as_object()) {
            for (name, data) in base_prompts {
                if let Some(template) = data.get("template").and_then(|v| v.as_str()) {
                    // Only add if we don't already have an optimized version
                    let optimized_name = format!("{}_optimized", name);
                    if !templates.iter().any(|t| t.name == optimized_name) {
                        templates.push(PromptTemplate::new(name.clone(), template));
                    }
                }
            }
        }

        // Load COPRO candidates if available
        if let Some(copro) = prompts_data.get("copro_candidates").and_then(|v| v.as_object()) {
            if let Some(active) = copro.get("active").and_then(|v| v.as_array()) {
                for candidate in active {
                    if let (Some(name), Some(template), Some(score)) = (
                        candidate.get("name").and_then(|v| v.as_str()),
                        candidate.get("template").and_then(|v| v.as_str()),
                        candidate.get("score").and_then(|v| v.as_f64()),
                    ) {
                        if score > 0.8 {
                            // Only use high-scoring candidates
                            templates.push(PromptTemplate::new(format!("{}_copro", name), template));
                        }
                    }
                }
            }
        }

        return Ok(templates);
    }

    Ok(Vec::new())
}

/// Load templates from Moon configuration keys (fallback method)
fn load_templates_from_moon_config() -> Vec<PromptTemplate> {
    let mut templates = Vec::new();

    let config_keys = [
        ("code_analysis", "moonshine.prompts.code_analysis"),
        ("code_fixing", "moonshine.prompts.code_fixing"),
        ("optimization", "moonshine.prompts.optimization"),
        ("typescript_strict", "moonshine.prompts.typescript_strict"),
        ("security_analysis", "moonshine.prompts.security_analysis"),
    ];

    for (name, config_key) in &config_keys {
        if let Ok(Some(prompt_content)) = get_moon_config_safe(config_key) {
            templates.push(PromptTemplate::new(*name, prompt_content));
        }
    }

    templates
}

/// Get embedded default templates with comprehensive ai-lint.js proven patterns
fn get_embedded_default_templates() -> Vec<PromptTemplate> {
    vec![
        // Multi-Pass AI Templates (based on ai-lint.js proven strategies)
        PromptTemplate::new("typescript_compilation_fixer",
            "🔥 **COMPILATION + CRITICAL ERRORS** (Pass 1 Focus)\n\
             - TypeScript compilation errors (TS2XXX codes) - HIGHEST PRIORITY\n\
             - Syntax errors, type errors, compilation failures\n\
             - Security issues (no-eval, no-implied-eval)\n\
             - Runtime errors (no-undef, no-unreachable)\n\
             - Promise handling (no-floating-promises)\n\n\
             **GOAL**: TypeScript compilation success + 0 critical runtime errors\n\n\
             Analyze and fix the following {language} code for compilation and critical runtime errors:\n\n\
             File: {file_path}\n\
             Current Issues ({error_count} compilation errors, {warning_count} critical warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             Return only the corrected code that compiles successfully and eliminates all critical runtime errors."
        ),
        PromptTemplate::new("method_implementation_completer",
            "⚡ **TYPE SAFETY + IMPLEMENTATION** (Pass 2 Focus)\n\
             - Remaining TypeScript strict mode violations\n\
             - Type assertion and casting issues\n\
             - **IMPLEMENT unused parameters in method bodies instead of prefixing with underscore**\n\
             - **COMPLETE async method implementations with full business logic**\n\
             - Missing return types and explicit any usage\n\
             - Interface and type definition errors\n\n\
             **GOAL**: Full parameter usage + complete async implementations + strict types\n\n\
             Improve type safety and implement complete method logic for the following {language} code:\n\n\
             File: {file_path}\n\
             Type Safety Issues ({error_count} errors, {warning_count} warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             IMPORTANT:\n\
             - Use ALL method parameters meaningfully in the method body\n\
             - Implement complete async method business logic, not placeholders\n\
             - Replace 'any' types with proper TypeScript interfaces\n\
             - Add explicit return types\n\n\
             Return the fully implemented code with strict type safety."
        ),
        PromptTemplate::new("google_style_modernizer",
            "🎨 **CODE QUALITY + GOOGLE STYLE** (Pass 3 Focus)\n\
             - **Google TypeScript Style**: Use nullish coalescing (??) over logical OR (||)\n\
             - **Google TypeScript Style**: Use optional chaining (?.) over manual null checks\n\
             - **Modern patterns**: prefer-optional-chain, prefer-nullish-coalescing ESLint rules\n\
             - Complexity reduction (complexity, max-lines, cognitive-complexity)\n\
             - Best practices (prefer-const, no-var, prefer-readonly)\n\
             - Performance optimizations and dead code elimination\n\n\
             **GOAL**: Modern TypeScript patterns + Google style compliance + reduced complexity\n\n\
             Apply Google TypeScript style and modern patterns to the following {language} code:\n\n\
             File: {file_path}\n\
             Code Quality Issues ({error_count} errors, {warning_count} warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             IMPORTANT:\n\
             - Replace || with ?? where appropriate (nullish coalescing)\n\
             - Replace manual null checks with ?. (optional chaining)\n\
             - Reduce cyclomatic complexity\n\
             - Apply modern TypeScript patterns\n\
             - Follow Google TypeScript style guide\n\n\
             Return the modernized, Google-style compliant code."
        ),
        PromptTemplate::new("import_style_organizer",
            "✨ **STYLE + CONSISTENCY** (Pass 4 Focus)\n\
             - Code formatting and style consistency\n\
             - Import organization and module structure\n\
             - Naming conventions and documentation\n\
             - Final complexity optimizations\n\n\
             **GOAL**: Consistent style + organized imports + proper naming\n\n\
             Apply consistent styling and organize the following {language} code:\n\n\
             File: {file_path}\n\
             Style Issues ({error_count} errors, {warning_count} warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             IMPORTANT:\n\
             - Organize imports by type (external, internal, relative)\n\
             - Apply consistent naming conventions\n\
             - Ensure proper code formatting\n\
             - Optimize import statements\n\n\
             Return the styled and consistently formatted code."
        ),
        PromptTemplate::new("edge_case_handler",
            "🎯 **EDGE CASES + POLISH** (Pass 5 Focus)\n\
             - Remaining edge case handling\n\
             - Final style micro-adjustments\n\
             - Error boundary improvements\n\
             - Accessibility and maintainability\n\n\
             **GOAL**: Edge case handling + final polish + maintainability\n\n\
             Apply final polish and edge case handling to the following {language} code:\n\n\
             File: {file_path}\n\
             Remaining Issues ({error_count} errors, {warning_count} warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             IMPORTANT:\n\
             - Handle edge cases and error boundaries\n\
             - Apply final micro-optimizations\n\
             - Ensure maintainability and readability\n\
             - Address any remaining lint warnings\n\n\
             Return the polished, production-ready code."
        ),
        PromptTemplate::new("production_perfectionist",
            "🏆 **ZERO TOLERANCE PERFECTIONIST** (Pass 6 Focus)\n\
             - Final compilation verification\n\
             - Any remaining micro-issues\n\
             - Absolute zero warnings and errors\n\
             - Production-ready code quality\n\n\
             **GOAL**: Absolute perfection - 0 errors, 0 warnings, production-ready\n\n\
             Achieve zero tolerance perfection for the following {language} code:\n\n\
             File: {file_path}\n\
             Final Issues ({error_count} errors, {warning_count} warnings):\n\
             {issues_list}\n\n\
             Code:\n{code}\n\n\
             IMPORTANT:\n\
             - Eliminate every single warning and error\n\
             - Ensure production-ready quality\n\
             - Verify TypeScript compilation success\n\
             - Achieve absolute code perfection\n\n\
             Return the perfectly clean, production-ready code with zero issues."
        ),
        PromptTemplate::new("tsdoc_enhancement",
            "# TSDoc Documentation Improvement Task\n\n\
             You are an expert TypeScript documentation specialist. Your task is to improve TSDoc coverage to {target_coverage}% by adding comprehensive documentation to methods and functions.\n\n\
             ## Current Status\n\
             - **Current Coverage**: {current_coverage}%\n\
             - **Target Coverage**: {target_coverage}%\n\
             - **Missing Documentation**: {missing_count} methods\n\
             - **Custom Tag Issues**: {tag_issues_count} methods\n\n\
             ## Required Custom Tags\n\
             All public methods must include these custom tags:\n\
             - @category - Method classification (coordination, audit, validation, etc.)\n\
             - @safe - SAFe 6.0 level (team, program, large-solution, portfolio)\n\
             - @mvp - MVP classification (core, extension, future)\n\
             - @complexity - Complexity level (low, medium, high, critical)\n\
             - @since - Version introduced\n\n\
             ## Instructions\n\
             1. **Add comprehensive TSDoc comments** for ALL missing methods\n\
             2. **Include all required custom tags** with appropriate values\n\
             3. **Write clear descriptions** explaining purpose, parameters, and return values\n\
             4. **Follow TSDoc standards** with proper @param and @returns tags\n\
             5. **Target 90%+ coverage** - document nearly every public method\n\n\
             File: {file_path}\n\
             Code:\n{code}\n\n\
             Return the code with comprehensive TSDoc documentation targeting {target_coverage}% coverage."
        ),
        PromptTemplate::new("complexity_analysis",
            "📊 **COMPLEXITY ANALYSIS + OPTIMIZATION**\n\n\
             Analyze and optimize complexity for the following {language} code:\n\n\
             ## Complexity Metrics\n\
             - **Cyclomatic Complexity**: {cyclomatic_complexity} (threshold: 10)\n\
             - **Halstead Difficulty**: {halstead_difficulty} (threshold: 20)\n\
             - **Lines of Code**: {lines_of_code}\n\
             - **Cognitive Complexity**: {cognitive_complexity}\n\n\
             File: {file_path}\n\
             Code:\n{code}\n\n\
             ## Optimization Goals\n\
             1. **Reduce cyclomatic complexity** below 10 per function\n\
             2. **Simplify control flow** and eliminate nested conditions\n\
             3. **Extract helper functions** for complex logic\n\
             4. **Apply SOLID principles** for better maintainability\n\
             5. **Optimize algorithms** for better performance\n\n\
             **Return the optimized code with reduced complexity while maintaining all functionality.**"
        ),
        PromptTemplate::new("security_analysis",
            "🔒 **SECURITY ANALYSIS + HARDENING**\n\n\
             Perform comprehensive security analysis for the following {language} code:\n\n\
             File: {file_path}\n\
             Security Issues Found ({issue_count} issues):\n\
             {security_issues}\n\n\
             Code:\n{code}\n\n\
             ## Security Focus Areas\n\
             1. **Input Validation**: Validate all user inputs and external data\n\
             2. **Authentication/Authorization**: Verify access controls and permissions\n\
             3. **Data Exposure**: Prevent sensitive data leakage\n\
             4. **Injection Prevention**: Protect against SQL, XSS, and other injections\n\
             5. **Cryptographic Security**: Use proper encryption and hashing\n\
             6. **Error Handling**: Avoid information disclosure in error messages\n\n\
             **Return the security-hardened code with all vulnerabilities addressed.**"
        ),
        PromptTemplate::new("performance_optimization",
            "⚡ **PERFORMANCE OPTIMIZATION**\n\n\
             Optimize performance for the following {language} code:\n\n\
             File: {file_path}\n\
             Performance Issues ({issue_count} issues):\n\
             {performance_issues}\n\n\
             Current Performance Metrics:\n\
             - **Execution Time**: {execution_time}ms\n\
             - **Memory Usage**: {memory_usage}MB\n\
             - **Algorithm Complexity**: {algorithm_complexity}\n\n\
             Code:\n{code}\n\n\
             ## Optimization Strategies\n\
             1. **Algorithm Optimization**: Replace O(n²) with O(n log n) or better\n\
             2. **Memory Efficiency**: Reduce memory allocations and garbage collection\n\
             3. **Caching**: Implement memoization for expensive computations\n\
             4. **Lazy Loading**: Defer expensive operations until needed\n\
             5. **Async Optimization**: Use Promise.all() for parallel operations\n\
             6. **Data Structure Optimization**: Use appropriate collections (Map, Set, etc.)\n\n\
             **Return the performance-optimized code maintaining all functionality.**"
        ),
        // Legacy templates for backwards compatibility
        PromptTemplate::new("code_analysis",
            "Analyze the following {language} code for potential issues and improvements:\n\n\
             File: {file_path}\nCode:\n{code}\n\n\
             Provide a detailed analysis focusing on:\n\
             1. Code quality and maintainability\n\
             2. Performance considerations\n\
             3. Security vulnerabilities\n\
             4. Best practices compliance\n\
             5. Specific improvement suggestions\n\n\
             Return structured suggestions with line numbers and severity levels."
        ),
        PromptTemplate::new("typescript_strict",
            "Apply strict TypeScript patterns to the following code:\n\n\
             File: {file_path}\nCode:\n{code}\n\n\
             Apply these improvements:\n\
             1. Add explicit return types\n\
             2. Replace 'any' with proper types\n\
             3. Add comprehensive TSDoc comments\n\
             4. Use modern TypeScript patterns\n\
             5. Implement Google TypeScript style\n\n\
             Target 90% TSDoc coverage. Return corrected code only."
        ),
    ]
}

// OptimizationConfig and WorkflowConfig moved to MoonShineConfig - all settings consolidated

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRuleConfiguration {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

pub fn load_embedded_defaults() -> Vec<PromptTemplate> {
    get_default_templates()
}

// Configuration functions removed - use MoonShineConfig::from_moon_workspace() instead
// All configuration consolidated into single MoonShineConfig struct

pub fn get_available_rules(custom_prompts: Option<&HashMap<String, String>>) -> Vec<String> {
    // Production: Load rules from configurable sources with robust fallback hierarchy
    let mut rules: BTreeSet<String> = BTreeSet::new();

    // Priority 1: Load from external rules configuration (highest priority)
    if let Ok(external_rules) = load_rules_from_external_config() {
        rules.extend(external_rules);
    }

    // Priority 2: Load from Moon configuration prompts.json
    if let Ok(json_rules) = load_rules_from_moon_config() {
        rules.extend(json_rules);
    }

    // Priority 3: Add custom prompts if provided
    if let Some(custom) = custom_prompts {
        for key in custom.keys() {
            rules.insert(key.to_string());
        }
    }

    // Priority 4: Add embedded defaults if no external sources provide rules
    if rules.is_empty() {
        rules.extend(get_embedded_default_rules());
    }

    rules.into_iter().collect()
}

/// Load available rules from external configuration with production robustness
fn load_rules_from_external_config() -> Result<Vec<String>> {
    let config_paths = [
        ".moon/moonshine/rules.json",
        ".moon/moonshine/available-rules.json",
        "moonshine-rules.json",
        "/etc/moonshine/rules.json", // System-wide configuration
    ];

    let mut last_error = None;

    for path in &config_paths {
        match load_rules_from_file_with_validation(path) {
            Ok(rules) if !rules.is_empty() => {
                return Ok(rules);
            }
            Ok(_) => continue, // Empty file, try next
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    // Try environment variable path
    if let Ok(Some(env_path)) = get_moon_config_safe("MOONSHINE_RULES_PATH") {
        if let Ok(rules) = load_rules_from_file_with_validation(&env_path) {
            if !rules.is_empty() {
                return Ok(rules);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| Error::config("No external rules configuration found".to_string())))
}

/// Load rules from a specific file with comprehensive validation
fn load_rules_from_file_with_validation(file_path: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path).map_err(|e| Error::config(format!("Failed to read rules file {}: {}", file_path, e)))?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let json_data: serde_json::Value = serde_json::from_str(&content).map_err(|e| Error::config(format!("Invalid JSON in rules file {}: {}", file_path, e)))?;

    parse_rules_json(&json_data)
}

/// Parse rules JSON with comprehensive validation and priority handling
fn parse_rules_json(json_data: &serde_json::Value) -> Result<Vec<String>> {
    let mut rules = Vec::new();

    // Validate schema version for compatibility
    if let Some(version) = json_data.get("version").and_then(|v| v.as_str()) {
        if !is_compatible_version(version) {
            return Err(Error::config(format!("Incompatible rules JSON version: {}", version)));
        }
    }

    // Priority 1: Active rules (explicitly enabled)
    if let Some(active_rules) = json_data.get("active_rules").and_then(|v| v.as_array()) {
        for rule in active_rules {
            if let Some(rule_name) = rule.as_str() {
                rules.push(rule_name.to_string());
            } else if let Some(rule_obj) = rule.as_object() {
                if let Some(name) = rule_obj.get("name").and_then(|v| v.as_str()) {
                    let enabled = rule_obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                    if enabled {
                        rules.push(name.to_string());
                    }
                }
            }
        }
    }

    // Priority 2: All available rules if no active rules specified
    if rules.is_empty() {
        if let Some(available_rules) = json_data.get("available_rules").and_then(|v| v.as_array()) {
            for rule in available_rules {
                if let Some(rule_name) = rule.as_str() {
                    rules.push(rule_name.to_string());
                }
            }
        }
    }

    // Priority 3: Extract rule names from rule definitions
    if rules.is_empty() {
        if let Some(rule_definitions) = json_data.get("rule_definitions").and_then(|v| v.as_object()) {
            for (rule_name, rule_def) in rule_definitions {
                let enabled = rule_def.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                if enabled {
                    rules.push(rule_name.to_string());
                }
            }
        }
    }

    Ok(rules)
}

/// Load rules from Moon configuration prompts.json
fn load_rules_from_moon_config() -> Result<Vec<String>> {
    if let Ok(Some(prompts_json)) = get_moon_config_safe("moonshine_prompts_file") {
        let prompts_data: serde_json::Value =
            serde_json::from_str(&prompts_json).map_err(|e| Error::config(format!("Invalid prompts JSON in Moon config: {}", e)))?;

        let mut rules = Vec::new();

        // Extract rule names from optimized prompts
        if let Some(optimized) = prompts_data.get("optimized_prompts").and_then(|v| v.as_object()) {
            for rule_name in optimized.keys() {
                rules.push(format!("{}_optimized", rule_name));
            }
        }

        // Extract rule names from base prompts
        if let Some(base_prompts) = prompts_data.get("base_prompts").and_then(|v| v.as_object()) {
            for rule_name in base_prompts.keys() {
                rules.push(rule_name.to_string());
            }
        }

        // Extract rule names from COPRO candidates
        if let Some(copro) = prompts_data.get("copro_candidates").and_then(|v| v.as_object()) {
            if let Some(active) = copro.get("active").and_then(|v| v.as_array()) {
                for candidate in active {
                    if let Some(name) = candidate.get("name").and_then(|v| v.as_str()) {
                        rules.push(format!("{}_copro", name));
                    }
                }
            }
        }

        if !rules.is_empty() {
            return Ok(rules);
        }
    }

    Ok(Vec::new())
}

/// Get embedded default rules when no external configuration is available
fn get_embedded_default_rules() -> Vec<String> {
    vec![
        // Specialized AI prompts (ai-lint.js proven strategies with descriptive names)
        "typescript_compilation_fixer".to_string(),
        "method_implementation_completer".to_string(),
        "google_style_modernizer".to_string(),
        "import_style_organizer".to_string(),
        "edge_case_handler".to_string(),
        "production_perfectionist".to_string(),
        "tsdoc_enhancement".to_string(),
        "complexity_analysis".to_string(),
        "security_analysis".to_string(),
        "performance_optimization".to_string(),
        // Legacy rules (backwards compatibility)
        "code_quality".to_string(),
        "performance".to_string(),
        "security".to_string(),
        "best_practices".to_string(),
        "no_unused_vars".to_string(),
        "missing_types".to_string(),
        "no_console".to_string(),
        "missing_jsdoc".to_string(),
        "async_best_practices".to_string(),
        "code_analysis".to_string(),
        "typescript_strict".to_string(),
    ]
}

/// Built-in fallback prompts with comprehensive ai-lint.js proven patterns
pub fn get_fallback_prompt(rule_type: &str) -> &'static str {
    match rule_type {
        // Specialized AI Prompts (ai-lint.js proven strategies with descriptive names)
        "typescript_compilation_fixer" => "🔥 **TYPESCRIPT COMPILATION FIXER**\n- TypeScript compilation errors (TS2XXX codes) - HIGHEST PRIORITY\n- Syntax errors, type errors, compilation failures\n- Security issues (no-eval, no-implied-eval)\n- Runtime errors (no-undef, no-unreachable)\n- Promise handling (no-floating-promises)\n\n**GOAL**: TypeScript compilation success + 0 critical runtime errors\n\nAnalyze and fix compilation and critical runtime errors. Return only corrected code that compiles successfully.",

        "method_implementation_completer" => "⚡ **METHOD IMPLEMENTATION COMPLETER**\n- Remaining TypeScript strict mode violations\n- Type assertion and casting issues\n- **IMPLEMENT unused parameters in method bodies instead of prefixing with underscore**\n- **COMPLETE async method implementations with full business logic**\n- Missing return types and explicit any usage\n- Interface and type definition errors\n\n**GOAL**: Full parameter usage + complete async implementations + strict types\n\nIMPORTANT:\n- Use ALL method parameters meaningfully in the method body\n- Implement complete async method business logic, not placeholders\n- Replace 'any' types with proper TypeScript interfaces\n- Add explicit return types\n\nReturn fully implemented code with strict type safety.",

        "google_style_modernizer" => "🎨 **GOOGLE STYLE MODERNIZER**\n- **Google TypeScript Style**: Use nullish coalescing (??) over logical OR (||)\n- **Google TypeScript Style**: Use optional chaining (?.) over manual null checks\n- **Modern patterns**: prefer-optional-chain, prefer-nullish-coalescing ESLint rules\n- Complexity reduction (complexity, max-lines, cognitive-complexity)\n- Best practices (prefer-const, no-var, prefer-readonly)\n- Performance optimizations and dead code elimination\n\n**GOAL**: Modern TypeScript patterns + Google style compliance + reduced complexity\n\nIMPORTANT:\n- Replace || with ?? where appropriate (nullish coalescing)\n- Replace manual null checks with ?. (optional chaining)\n- Reduce cyclomatic complexity\n- Apply modern TypeScript patterns\n- Follow Google TypeScript style guide\n\nReturn modernized, Google-style compliant code.",

        "import_style_organizer" => "✨ **IMPORT & STYLE ORGANIZER**\n- Code formatting and style consistency\n- Import organization and module structure\n- Naming conventions and documentation\n- Final complexity optimizations\n\n**GOAL**: Consistent style + organized imports + proper naming\n\nIMPORTANT:\n- Organize imports by type (external, internal, relative)\n- Apply consistent naming conventions\n- Ensure proper code formatting\n- Optimize import statements\n\nReturn styled and consistently formatted code.",

        "edge_case_handler" => "🎯 **EDGE CASE HANDLER**\n- Remaining edge case handling\n- Final style micro-adjustments\n- Error boundary improvements\n- Accessibility and maintainability\n\n**GOAL**: Edge case handling + final polish + maintainability\n\nIMPORTANT:\n- Handle edge cases and error boundaries\n- Apply final micro-optimizations\n- Ensure maintainability and readability\n- Address any remaining lint warnings\n\nReturn polished, production-ready code.",

        "production_perfectionist" => "🏆 **PRODUCTION PERFECTIONIST**\n- Final compilation verification\n- Any remaining micro-issues\n- Absolute zero warnings and errors\n- Production-ready code quality\n\n**GOAL**: Absolute perfection - 0 errors, 0 warnings, production-ready\n\nIMPORTANT:\n- Eliminate every single warning and error\n- Ensure production-ready quality\n- Verify TypeScript compilation success\n- Achieve absolute code perfection\n\nReturn perfectly clean, production-ready code with zero issues.",

        "tsdoc_enhancement" => "# TSDoc Documentation Improvement Task\n\nYou are an expert TypeScript documentation specialist. Improve TSDoc coverage to 90% by adding comprehensive documentation.\n\n## Required Custom Tags\n- @category - Method classification (coordination, audit, validation, etc.)\n- @safe - SAFe 6.0 level (team, program, large-solution, portfolio)\n- @mvp - MVP classification (core, extension, future)\n- @complexity - Complexity level (low, medium, high, critical)\n- @since - Version introduced\n\n## Instructions\n1. Add comprehensive TSDoc comments for ALL missing methods\n2. Include all required custom tags with appropriate values\n3. Write clear descriptions with @param and @returns tags\n4. Target 90%+ coverage - document nearly every public method\n\nReturn code with comprehensive TSDoc documentation.",

        "complexity_analysis" => "📊 **COMPLEXITY ANALYSIS + OPTIMIZATION**\n\nAnalyze and optimize code complexity.\n\n## Optimization Goals\n1. **Reduce cyclomatic complexity** below 10 per function\n2. **Simplify control flow** and eliminate nested conditions\n3. **Extract helper functions** for complex logic\n4. **Apply SOLID principles** for better maintainability\n5. **Optimize algorithms** for better performance\n\nReturn optimized code with reduced complexity while maintaining all functionality.",

        "security_analysis" => "🔒 **SECURITY ANALYSIS + HARDENING**\n\nPerform comprehensive security analysis.\n\n## Security Focus Areas\n1. **Input Validation**: Validate all user inputs and external data\n2. **Authentication/Authorization**: Verify access controls and permissions\n3. **Data Exposure**: Prevent sensitive data leakage\n4. **Injection Prevention**: Protect against SQL, XSS, and other injections\n5. **Cryptographic Security**: Use proper encryption and hashing\n6. **Error Handling**: Avoid information disclosure in error messages\n\nReturn security-hardened code with all vulnerabilities addressed.",

        "performance_optimization" => "⚡ **PERFORMANCE OPTIMIZATION**\n\nOptimize code performance.\n\n## Optimization Strategies\n1. **Algorithm Optimization**: Replace O(n²) with O(n log n) or better\n2. **Memory Efficiency**: Reduce memory allocations and garbage collection\n3. **Caching**: Implement memoization for expensive computations\n4. **Lazy Loading**: Defer expensive operations until needed\n5. **Async Optimization**: Use Promise.all() for parallel operations\n6. **Data Structure Optimization**: Use appropriate collections (Map, Set, etc.)\n\nReturn performance-optimized code maintaining all functionality.",

        // Legacy specific rules (maintained for backwards compatibility)
        "no_unused_vars" => "IMPLEMENT unused parameters in method bodies instead of prefixing with underscore.\nCOMPLETE async method implementations with full business logic using parameters.\nReplace empty stubs with complete, functional code using ALL method parameters.\nFocus on: meaningful parameter usage, proper async implementations, business logic.\nExample: async createResource(id: UUID, config: ResourceConfig) should use both id and config.",

        "missing_types" => "Google TypeScript Style: Use nullish coalescing (??) over logical OR (||).\nGoogle TypeScript Style: Use optional chaining (?.) over manual null checks.\nReplace explicit 'any' types with proper TypeScript interfaces.\nAdd explicit return types to all public methods and functions.\nFocus on: strict type safety, modern TypeScript patterns, Google style compliance.",

        "no_console" => "Remove console.log statements for production code.\nReplace with structured logging libraries or debug flags.\nRemove debugger statements before committing.\nFocus on: production readiness, proper logging patterns, clean deployments.",

        "missing_jsdoc" => "Add comprehensive TSDoc comments for ALL missing methods targeting 90% coverage.\nInclude required custom tags: @category, @safe, @mvp, @complexity, @since.\nWrite clear descriptions with @param, @returns, @throws tags.\nFocus on: public APIs, exported modules, factual AI-optimized documentation.\nExample: @category coordination @safe large-solution @mvp core @complexity high @since 1.0.0",

        "async_best_practices" => "Fix missing await on Promise-returning functions.\nHandle unhandled Promise rejections with proper error boundaries.\nImplement complete async method business logic, not empty placeholders.\nFocus on: proper async/await patterns, error handling, complete implementations.\nExample: async methods should use parameters meaningfully and return proper results.",

        // Default fallback
        _ => "Analyze the provided code for quality issues and return specific suggestions with production-ready improvements."
    }
}

/// Get prompt with fallback hierarchy: custom -> embedded -> built-in
pub fn get_prompt(rule_type: &str, custom_prompts: Option<&HashMap<String, String>>) -> String {
    if let Some(custom) = custom_prompts {
        if let Some(prompt) = custom.get(rule_type) {
            return prompt.clone();
        }
    }

    get_fallback_prompt(rule_type).to_string()
}

/// Save optimized prompt to JSON configuration
pub fn save_optimized_prompt(prompt_name: &str, template: &str, score: f64) -> Result<()> {
    let prompts_data = load_or_create_prompts_json()?;
    let mut prompts_obj = prompts_data.as_object().cloned().unwrap_or_default();

    // Initialize optimized_prompts section if not exists
    if !prompts_obj.contains_key("optimized_prompts") {
        prompts_obj.insert("optimized_prompts".to_string(), serde_json::json!({}));
    }

    // Add the optimized prompt
    if let Some(optimized_section) = prompts_obj.get_mut("optimized_prompts") {
        if let Some(optimized_obj) = optimized_section.as_object_mut() {
            optimized_obj.insert(
                prompt_name.to_string(),
                serde_json::json!({
                    "template": template,
                    "score": score,
                    "optimized_at": chrono::Utc::now().to_rfc3339(),
                    "version": "1.0.0"
                }),
            );
        }
    }

    // Save back to JSON
    let json_content = serde_json::to_string_pretty(&prompts_obj).map_err(|e| Error::config(format!("Failed to serialize prompts JSON: {}", e)))?;

    write_file_atomic(".moon/moonshine/prompts.json", &json_content).map_err(|e| Error::config(format!("Failed to save prompts JSON: {}", e)))?;

    Ok(())
}

/// Add COPRO candidate to JSON configuration
pub fn save_copro_candidate(prompt_name: &str, template: &str, score: f64, generation: u32) -> Result<()> {
    let prompts_data = load_or_create_prompts_json()?;
    let mut prompts_obj = prompts_data.as_object().cloned().unwrap_or_default();

    // Initialize copro_candidates section if not exists
    if !prompts_obj.contains_key("copro_candidates") {
        prompts_obj.insert(
            "copro_candidates".to_string(),
            serde_json::json!({
                "active": [],
                "archived": []
            }),
        );
    }

    // Add the candidate to active list
    if let Some(copro_section) = prompts_obj.get_mut("copro_candidates") {
        if let Some(active_array) = copro_section.get_mut("active").and_then(|v| v.as_array_mut()) {
            active_array.push(serde_json::json!({
                "name": prompt_name,
                "template": template,
                "score": score,
                "generation": generation,
                "created_at": chrono::Utc::now().to_rfc3339()
            }));

            // Keep only top 10 candidates to avoid bloat
            if active_array.len() > 10 {
                active_array.sort_by(|a, b| {
                    let score_a = a.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let score_b = b.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
                });
                active_array.truncate(10);
            }
        }
    }

    // Save back to JSON
    let json_content = serde_json::to_string_pretty(&prompts_obj).map_err(|e| Error::config(format!("Failed to serialize prompts JSON: {}", e)))?;

    write_file_atomic(".moon/moonshine/prompts.json", &json_content).map_err(|e| Error::config(format!("Failed to save prompts JSON: {}", e)))?;

    Ok(())
}

/// Load existing prompts.json or create new structure
fn load_or_create_prompts_json() -> Result<serde_json::Value> {
    if let Ok(Some(json_content)) = get_moon_config_safe("moonshine_prompts_file") {
        serde_json::from_str(&json_content).map_err(|e| Error::config(format!("Invalid existing prompts JSON: {}", e)))
    } else {
        // Create default structure
        Ok(serde_json::json!({
            "version": "1.0.0",
            "last_updated": chrono::Utc::now().to_rfc3339(),
            "base_prompts": {},
            "optimized_prompts": {},
            "copro_candidates": {
                "active": [],
                "archived": []
            },
            "metadata": {
                "extension_version": env!("CARGO_PKG_VERSION"),
                "prompt_count": 0
            }
        }))
    }
}

/// Initialize default prompts.json if it doesn't exist
pub fn initialize_prompts_storage() -> Result<()> {
    let default_prompts = serde_json::json!({
        "version": "1.0.0",
        "last_updated": chrono::Utc::now().to_rfc3339(),
        "base_prompts": {
            "code_analysis": {
                "template": get_fallback_prompt("code_analysis"),
                "variables": ["language", "file_path", "code"],
                "description": "General code analysis prompt"
            },
            "typescript_strict": {
                "template": get_fallback_prompt("typescript_strict"),
                "variables": ["file_path", "code"],
                "description": "TypeScript strict mode improvements"
            },
            "missing_jsdoc": {
                "template": get_fallback_prompt("missing_jsdoc"),
                "variables": ["file_path", "code"],
                "description": "JSDoc/TSDoc documentation improvements"
            }
        },
        "optimized_prompts": {},
        "copro_candidates": {
            "active": [],
            "archived": []
        },
        "metadata": {
            "extension_version": env!("CARGO_PKG_VERSION"),
            "prompt_count": 3,
            "initialized_at": chrono::Utc::now().to_rfc3339()
        }
    });

    let json_content = serde_json::to_string_pretty(&default_prompts).map_err(|e| Error::config(format!("Failed to serialize default prompts: {}", e)))?;

    write_file_atomic(".moon/moonshine/prompts.json", &json_content).map_err(|e| Error::config(format!("Failed to initialize prompts storage: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_prompt_template_creation() {
        let template = PromptTemplate::new("test", "Hello {name}!");
        assert_eq!(template.name, "test");
        assert_eq!(template.template, "Hello {name}!");
        assert_eq!(template.variables, vec!["name"]);
    }

    #[test]
    fn test_prompt_rendering() {
        let template = PromptTemplate::new("greeting", "Hello {name}, you are {age} years old!");
        let mut context = HashMap::new();
        context.insert("name".to_string(), "Alice".to_string());
        context.insert("age".to_string(), "25".to_string());

        let result = template.render(&context).expect("Template should render successfully");
        assert_eq!(result, "Hello Alice, you are 25 years old!");
    }

    #[test]
    fn test_missing_variable_error() {
        let template = PromptTemplate::new("test", "Hello {name}!");
        let context = HashMap::new();

        let result = template.render(&context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing variable: name"));
    }

    #[test]
    fn test_variable_extraction() {
        let variables = extract_variables("Hello {name}, welcome to {place}!");
        assert_eq!(variables, vec!["name", "place"]);
    }
}
