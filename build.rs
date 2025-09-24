use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Watch for changes in essential configuration files
    println!("cargo:rerun-if-changed=defaults/patterns.json");
    println!("cargo:rerun-if-changed=defaults/prompts.json");
    println!("cargo:rerun-if-changed=defaults/providers.json");

    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Generate default configurations for embedded resources
    generate_default_configs(&out_dir);

    // Generate compiled prompts for zero-cost runtime access
    generate_compiled_prompts(&out_dir);

    // Generate compiled providers for zero-cost runtime access
    generate_compiled_providers(&out_dir);

    // Generate compiled rulebase presets
    generate_compiled_presets(&out_dir);
}

/// Generate embedded default configurations for WASM deployment
fn generate_default_configs(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("embedded_defaults.rs");

    let mut rust_code = String::new();
    rust_code.push_str("// Auto-generated embedded defaults for WASM deployment\n");
    rust_code.push_str("// DO NOT EDIT - regenerated on each build\n\n");

    // Embed pattern configurations if available
    if let Ok(patterns_content) = fs::read_to_string("defaults/patterns.json") {
        rust_code.push_str("pub const DEFAULT_PATTERNS: &str = r#\"");
        rust_code.push_str(&patterns_content.replace("\"", "\\\""));
        rust_code.push_str("\"#;\n\n");
    } else {
        rust_code.push_str("pub const DEFAULT_PATTERNS: &str = \"{}\";\n\n");
    }

    // Embed prompt configurations if available
    if let Ok(prompts_content) = fs::read_to_string("defaults/prompts.json") {
        rust_code.push_str("pub const DEFAULT_PROMPTS: &str = r#\"");
        rust_code.push_str(&prompts_content.replace("\"", "\\\""));
        rust_code.push_str("\"#;\n\n");
    } else {
        rust_code.push_str("pub const DEFAULT_PROMPTS: &str = \"{}\";\n\n");
    }

    // Embed provider configurations if available
    if let Ok(providers_content) = fs::read_to_string("defaults/providers.json") {
        rust_code.push_str("pub const DEFAULT_PROVIDERS: &str = r#\"");
        rust_code.push_str(&providers_content.replace("\"", "\\\""));
        rust_code.push_str("\"#;\n\n");
    } else {
        rust_code.push_str("pub const DEFAULT_PROVIDERS: &str = \"{}\";\n\n");
    }

    // OXC and AI analysis statistics
    rust_code.push_str("pub const OXC_ENGINE_VERSION: &str = \"0.90.0\";\n");
    rust_code.push_str("pub const AI_ANALYSIS_ENABLED: bool = true;\n");

    fs::write(dest_path, rust_code).expect("Failed to write embedded defaults");
}

/// Generate compiled prompt templates for zero-cost runtime access
fn generate_compiled_prompts(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("compiled_prompts.rs");

    let mut rust_code = String::new();
    rust_code.push_str("// Auto-generated compiled prompt templates\n");
    rust_code.push_str("// DO NOT EDIT - regenerated on each build\n\n");

    // Basic prompt templates - embedded as constants for zero runtime cost
    rust_code.push_str("pub const DEFAULT_PROMPT_TEMPLATES: &[(&str, &str)] = &[\n");
    rust_code.push_str("    (\"code_analysis\", \"Analyze this code for potential issues:\\n\\n{code}\\n\\nProvide specific, actionable feedback.\"),\n");
    rust_code.push_str(
        "    (\"type_fixing\", \"Fix TypeScript type errors in this code:\\n\\n{code}\\n\\nErrors:\\n{errors}\\n\\nProvide the corrected code.\"),\n",
    );
    rust_code.push_str(
        "    (\"eslint_fixing\", \"Fix ESLint/OXC linting errors in this code:\\n\\n{code}\\n\\nErrors:\\n{errors}\\n\\nProvide the corrected code.\"),\n",
    );
    rust_code.push_str("    (\"performance_optimization\", \"Optimize this code for better performance:\\n\\n{code}\\n\\nFocus on algorithmic improvements and best practices.\"),\n");
    rust_code.push_str("    (\"security_review\", \"Review this code for security vulnerabilities:\\n\\n{code}\\n\\nHighlight potential security issues and provide fixes.\"),\n");
    rust_code.push_str("];\n\n");

    // Template metadata for validation and IDE support
    rust_code.push_str("pub const PROMPT_TEMPLATE_METADATA: &[(&str, &str, &[&str])] = &[\n");
    rust_code.push_str("    (\"code_analysis\", \"General Code Analysis\", &[\"code\"]),\n");
    rust_code.push_str("    (\"type_fixing\", \"TypeScript Type Fixing\", &[\"code\", \"errors\"]),\n");
    rust_code.push_str("    (\"eslint_fixing\", \"ESLint/OXC Error Fixing\", &[\"code\", \"errors\"]),\n");
    rust_code.push_str("    (\"performance_optimization\", \"Performance Optimization\", &[\"code\"]),\n");
    rust_code.push_str("    (\"security_review\", \"Security Code Review\", &[\"code\"]),\n");
    rust_code.push_str("];\n\n");

    // Helper function for template lookup
    rust_code.push_str("pub fn get_default_prompt_template(name: &str) -> Option<&'static str> {\n");
    rust_code.push_str("    DEFAULT_PROMPT_TEMPLATES.iter()\n");
    rust_code.push_str("        .find(|(template_name, _)| *template_name == name)\n");
    rust_code.push_str("        .map(|(_, template)| *template)\n");
    rust_code.push_str("}\n");

    fs::write(dest_path, rust_code).expect("Failed to write compiled prompts");
}

/// Generate compiled provider configurations for zero-cost runtime access
fn generate_compiled_providers(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("compiled_providers.rs");

    let mut rust_code = String::new();
    rust_code.push_str("// Auto-generated compiled provider configurations\n");
    rust_code.push_str("// DO NOT EDIT - regenerated on each build\n\n");

    // Provider configurations - embedded as constants for zero runtime cost
    rust_code.push_str("pub const DEFAULT_PROVIDER_CONFIGS: &[(&str, &str, &str)] = &[\n");
    rust_code.push_str("    (\"claude\", \"Anthropic Claude\", r#\"{\"max_tokens\": 4096, \"temperature\": 0.1}\"#),\n");
    rust_code.push_str("    (\"openai\", \"OpenAI GPT\", r#\"{\"max_tokens\": 4096, \"temperature\": 0.1}\"#),\n");
    rust_code.push_str("    (\"gemini\", \"Google Gemini\", r#\"{\"max_tokens\": 4096, \"temperature\": 0.1}\"#),\n");
    rust_code.push_str("];\n\n");

    // Provider capabilities mapping
    rust_code.push_str("pub const PROVIDER_CAPABILITIES: &[(&str, &[&str])] = &[\n");
    rust_code.push_str("    (\"claude\", &[\"code_analysis\", \"type_fixing\", \"refactoring\", \"security_review\"]),\n");
    rust_code.push_str("    (\"openai\", &[\"code_analysis\", \"type_fixing\", \"documentation\"]),\n");
    rust_code.push_str("    (\"gemini\", &[\"code_analysis\", \"performance_optimization\"]),\n");
    rust_code.push_str("];\n\n");

    // Helper function for provider lookup
    rust_code.push_str("pub fn get_default_provider_config(name: &str) -> Option<(&'static str, &'static str)> {\n");
    rust_code.push_str("    DEFAULT_PROVIDER_CONFIGS.iter()\n");
    rust_code.push_str("        .find(|(provider_name, _, _)| *provider_name == name)\n");
    rust_code.push_str("        .map(|(_, display_name, config)| (*display_name, *config))\n");
    rust_code.push_str("}\n\n");

    rust_code.push_str("pub fn get_provider_capabilities(name: &str) -> Option<&'static [&'static str]> {\n");
    rust_code.push_str("    PROVIDER_CAPABILITIES.iter()\n");
    rust_code.push_str("        .find(|(provider_name, _)| *provider_name == name)\n");
    rust_code.push_str("        .map(|(_, capabilities)| *capabilities)\n");
    rust_code.push_str("}\n\n");

    // Add the missing functions that compiled.rs expects
    rust_code.push_str("pub const DEFAULT_PROVIDER_CAPABILITIES: &[(&str, ProviderCapabilities)] = &[\n");
    rust_code.push_str("    (\"claude\", ProviderCapabilities {\n");
    rust_code.push_str("        code_analysis: 0.95,\n");
    rust_code.push_str("        code_generation: 0.90,\n");
    rust_code.push_str("        complex_reasoning: 0.95,\n");
    rust_code.push_str("        speed: 0.80,\n");
    rust_code.push_str("        context_length: 200000,\n");
    rust_code.push_str("        supports_sessions: true,\n");
    rust_code.push_str("    }),\n");
    rust_code.push_str("    (\"openai\", ProviderCapabilities {\n");
    rust_code.push_str("        code_analysis: 0.85,\n");
    rust_code.push_str("        code_generation: 0.88,\n");
    rust_code.push_str("        complex_reasoning: 0.80,\n");
    rust_code.push_str("        speed: 0.85,\n");
    rust_code.push_str("        context_length: 128000,\n");
    rust_code.push_str("        supports_sessions: true,\n");
    rust_code.push_str("    }),\n");
    rust_code.push_str("    (\"google\", ProviderCapabilities {\n");
    rust_code.push_str("        code_analysis: 0.82,\n");
    rust_code.push_str("        code_generation: 0.85,\n");
    rust_code.push_str("        complex_reasoning: 0.88,\n");
    rust_code.push_str("        speed: 0.90,\n");
    rust_code.push_str("        context_length: 1000000,\n");
    rust_code.push_str("        supports_sessions: false,\n");
    rust_code.push_str("    }),\n");
    rust_code.push_str("];\n\n");

    rust_code.push_str("pub fn get_default_provider_capabilities(name: &str) -> Option<ProviderCapabilities> {\n");
    rust_code.push_str("    DEFAULT_PROVIDER_CAPABILITIES.iter()\n");
    rust_code.push_str("        .find(|(provider_name, _)| *provider_name == name)\n");
    rust_code.push_str("        .map(|(_, capabilities)| capabilities.clone())\n");
    rust_code.push_str("}\n\n");

    rust_code.push_str("pub fn available_provider_names() -> Vec<&'static str> {\n");
    rust_code.push_str("    DEFAULT_PROVIDER_CAPABILITIES.iter()\n");
    rust_code.push_str("        .map(|(name, _)| *name)\n");
    rust_code.push_str("        .collect()\n");
    rust_code.push_str("}\n");

    fs::write(dest_path, rust_code).expect("Failed to write compiled providers");
}

/// Generate compiled rulebase presets for zero-cost runtime access
fn generate_compiled_presets(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("compiled_presets.rs");

    let mut rust_code = String::new();
    rust_code.push_str("// Auto-generated compiled rulebase presets\n");
    rust_code.push_str("// DO NOT EDIT - regenerated on each build\n\n");
    rust_code.push_str("use lazy_static::lazy_static;\n\n");

    // Generate the specific preset constants that the code expects

    rust_code.push_str("lazy_static! {\n");
    rust_code.push_str("    pub static ref PERFORMANCE_OPTIMIZED_PRESET: HashMap<String, Value> = {\n");
    rust_code.push_str("        let mut preset = HashMap::new();\n");
    rust_code.push_str("        preset.insert(\"rules\".to_string(), serde_json::json!({\n");
    rust_code.push_str("            \"no-empty-array\": \"warn\",\n");
    rust_code.push_str("            \"no-unused-vars\": \"error\",\n");
    rust_code.push_str("            \"prefer-const\": \"error\",\n");
    rust_code.push_str("            \"no-var\": \"error\"\n");
    rust_code.push_str("        }));\n");
    rust_code.push_str("        preset.insert(\"ai_enhanced\".to_string(), Value::Bool(true));\n");
    rust_code.push_str("        preset.insert(\"category\".to_string(), Value::String(\"performance\".to_string()));\n");
    rust_code.push_str("        preset\n");
    rust_code.push_str("    };\n\n");

    rust_code.push_str("    pub static ref DEVELOPMENT_FRIENDLY_PRESET: HashMap<String, Value> = {\n");
    rust_code.push_str("        let mut preset = HashMap::new();\n");
    rust_code.push_str("        preset.insert(\"rules\".to_string(), serde_json::json!({\n");
    rust_code.push_str("            \"no-debugger\": \"warn\",\n");
    rust_code.push_str("            \"no-console\": \"warn\",\n");
    rust_code.push_str("            \"no-empty-array\": \"info\"\n");
    rust_code.push_str("        }));\n");
    rust_code.push_str("        preset.insert(\"ai_enhanced\".to_string(), Value::Bool(true));\n");
    rust_code.push_str("        preset.insert(\"category\".to_string(), Value::String(\"development\".to_string()));\n");
    rust_code.push_str("        preset\n");
    rust_code.push_str("    };\n\n");

    rust_code.push_str("    pub static ref TYPESCRIPT_STRICT_PRESET: HashMap<String, Value> = {\n");
    rust_code.push_str("        let mut preset = HashMap::new();\n");
    rust_code.push_str("        preset.insert(\"rules\".to_string(), serde_json::json!({\n");
    rust_code.push_str("            \"no-any\": \"error\",\n");
    rust_code.push_str("            \"strict-type-checks\": \"error\",\n");
    rust_code.push_str("            \"no-implicit-any\": \"error\"\n");
    rust_code.push_str("        }));\n");
    rust_code.push_str("        preset.insert(\"ai_enhanced\".to_string(), Value::Bool(true));\n");
    rust_code.push_str("        preset.insert(\"category\".to_string(), Value::String(\"typescript\".to_string()));\n");
    rust_code.push_str("        preset\n");
    rust_code.push_str("    };\n\n");

    rust_code.push_str("    pub static ref ENTERPRISE_STRICT_PRESET: HashMap<String, Value> = {\n");
    rust_code.push_str("        let mut preset = HashMap::new();\n");
    rust_code.push_str("        preset.insert(\"rules\".to_string(), serde_json::json!({\n");
    rust_code.push_str("            \"no-debugger\": \"error\",\n");
    rust_code.push_str("            \"no-console\": \"error\",\n");
    rust_code.push_str("            \"no-any\": \"error\",\n");
    rust_code.push_str("            \"strict-mode\": \"error\"\n");
    rust_code.push_str("        }));\n");
    rust_code.push_str("        preset.insert(\"ai_enhanced\".to_string(), Value::Bool(true));\n");
    rust_code.push_str("        preset.insert(\"category\".to_string(), Value::String(\"enterprise\".to_string()));\n");
    rust_code.push_str("        preset\n");
    rust_code.push_str("    };\n\n");

    rust_code.push_str("    pub static ref SECURITY_CRITICAL_PRESET: HashMap<String, Value> = {\n");
    rust_code.push_str("        let mut preset = HashMap::new();\n");
    rust_code.push_str("        preset.insert(\"rules\".to_string(), serde_json::json!({\n");
    rust_code.push_str("            \"no-eval\": \"error\",\n");
    rust_code.push_str("            \"no-dangerous-html\": \"error\",\n");
    rust_code.push_str("            \"no-unsafe-innerHTML\": \"error\"\n");
    rust_code.push_str("        }));\n");
    rust_code.push_str("        preset.insert(\"ai_enhanced\".to_string(), Value::Bool(true));\n");
    rust_code.push_str("        preset.insert(\"category\".to_string(), Value::String(\"security\".to_string()));\n");
    rust_code.push_str("        preset\n");
    rust_code.push_str("    };\n");
    rust_code.push_str("}\n\n");

    // Rulebase presets - embedded as constants for zero runtime cost
    rust_code.push_str("pub const DEFAULT_RULEBASE_PRESETS: &[(&str, &str)] = &[\n");
    rust_code.push_str("    (\"performance-optimized\", \"{\\\"rules\\\": {\\\"no-empty-array\\\": \\\"warn\\\", \\\"no-unused-vars\\\": \\\"error\\\"}, \\\"ai_enhanced\\\": true}\"),\n");
    rust_code.push_str("    (\"development-friendly\", \"{\\\"rules\\\": {\\\"no-debugger\\\": \\\"warn\\\", \\\"no-console\\\": \\\"warn\\\"}, \\\"ai_enhanced\\\": true}\"),\n");
    rust_code.push_str("    (\"typescript-strict\", \"{\\\"rules\\\": {\\\"no-any\\\": \\\"error\\\", \\\"strict-type-checks\\\": \\\"error\\\"}, \\\"ai_enhanced\\\": true}\"),\n");
    rust_code.push_str("    (\"security-focused\", \"{\\\"rules\\\": {\\\"no-eval\\\": \\\"error\\\", \\\"no-dangerous-html\\\": \\\"error\\\"}, \\\"ai_enhanced\\\": true}\"),\n");
    rust_code.push_str("];\n\n");

    // Helper function for preset lookup
    rust_code.push_str("pub fn get_rulebase_preset(name: &str) -> Option<&'static str> {\n");
    rust_code.push_str("    DEFAULT_RULEBASE_PRESETS.iter()\n");
    rust_code.push_str("        .find(|(preset_name, _)| *preset_name == name)\n");
    rust_code.push_str("        .map(|(_, config)| *config)\n");
    rust_code.push_str("}\n\n");

    // Note: available_presets() is implemented in presets.rs, not generated here

    fs::write(dest_path, rust_code).expect("Failed to write compiled presets");
}
