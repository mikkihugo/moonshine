//! SunLinter Rule Converter
//!
//! Systematic conversion framework for porting all 192 SunLinter JavaScript rules
//! to moon-shine WASM-compatible format with behavioral analysis preservation.

use crate::sunlinter_integration::{SunLinterRuleConfig, SunLinterCategory, BehavioralPattern, BehavioralPatternType};
use std::fs;
use std::path::Path;
use serde_json::Value;
use regex::Regex;

/// Comprehensive rule converter for all SunLinter categories
pub struct SunLinterRuleConverter {
    pub converted_rules: Vec<SunLinterRuleConfig>,
    pub conversion_stats: ConversionStats,
}

#[derive(Debug, Default)]
pub struct ConversionStats {
    pub total_rules: usize,
    pub common_rules: usize,      // C-series
    pub security_rules: usize,    // S-series
    pub quality_rules: usize,     // T-series
    pub performance_rules: usize, // P-series
    pub migration_rules: usize,   // M-series
    pub conversion_errors: Vec<String>,
}

impl SunLinterRuleConverter {
    pub fn new() -> Self {
        Self {
            converted_rules: Vec::new(),
            conversion_stats: ConversionStats::default(),
        }
    }

    /// Convert all 192 SunLinter rules from JavaScript to WASM-compatible format
    pub fn convert_all_rules(&mut self, sunlinter_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting conversion of all SunLinter rules...");

        // Convert C-series rules (Common coding standards)
        self.convert_common_rules(&format!("{}/common", sunlinter_path))?;

        // Convert S-series rules (Security patterns)
        self.convert_security_rules(&format!("{}/security", sunlinter_path))?;

        // Convert T-series rules (TypeScript quality)
        self.convert_quality_rules(&format!("{}/quality", sunlinter_path))?;

        // Convert P-series rules (Performance)
        self.convert_performance_rules(&format!("{}/performance", sunlinter_path))?;

        // Convert M-series rules (Migration)
        self.convert_migration_rules(&format!("{}/migration", sunlinter_path))?;

        self.conversion_stats.total_rules = self.converted_rules.len();

        println!("✅ Conversion complete! {} rules converted", self.conversion_stats.total_rules);
        self.print_conversion_stats();

        Ok(())
    }

    /// Convert Common (C-series) rules
    fn convert_common_rules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📁 Converting Common (C-series) rules...");

        // Get all C-series rule directories
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let rule_path = entry.path();
                if let Some(rule_name) = rule_path.file_name().and_then(|n| n.to_str()) {
                    if rule_name.starts_with('C') {
                        match self.convert_single_rule(&rule_path, SunLinterCategory::Common) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.common_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", rule_name, e)
                                );
                            }
                        }
                    }
                }
            }
        }

        // Also convert standalone .js files in quality/ that are C-series
        if let Ok(entries) = fs::read_dir(&format!("{}/quality", path.replace("/common", ""))) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with("c0") && file_name.ends_with(".js") {
                        match self.convert_js_file(&entry.path(), SunLinterCategory::Common) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.common_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", file_name, e)
                                );
                            }
                        }
                    }
                }
            }
        }

        println!("  ✅ Converted {} C-series rules", self.conversion_stats.common_rules);
        Ok(())
    }

    /// Convert Security (S-series) rules
    fn convert_security_rules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔒 Converting Security (S-series) rules...");

        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let rule_path = entry.path();
                if let Some(rule_name) = rule_path.file_name().and_then(|n| n.to_str()) {
                    if rule_name.starts_with('S') {
                        match self.convert_single_rule(&rule_path, SunLinterCategory::Security) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.security_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", rule_name, e)
                                );
                            }
                        }
                    }
                }
            }
        }

        println!("  ✅ Converted {} S-series rules", self.conversion_stats.security_rules);
        Ok(())
    }

    /// Convert Quality (T-series) rules
    fn convert_quality_rules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚡ Converting Quality (T-series) rules...");

        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() && entry.file_name().to_string_lossy().ends_with(".js") {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with('t') || file_name.starts_with("T") {
                    match self.convert_js_file(&entry.path(), SunLinterCategory::Quality) {
                        Ok(rule) => {
                            self.converted_rules.push(rule);
                            self.conversion_stats.quality_rules += 1;
                        }
                        Err(e) => {
                            self.conversion_stats.conversion_errors.push(
                                format!("Failed to convert {}: {}", file_name, e)
                            );
                        }
                    }
                }
            }
        }

        println!("  ✅ Converted {} T-series rules", self.conversion_stats.quality_rules);
        Ok(())
    }

    /// Convert Performance (P-series) rules
    fn convert_performance_rules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Converting Performance (P-series) rules...");

        if Path::new(path).exists() {
            let entries = fs::read_dir(path)?;

            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name().to_string_lossy().to_string();

                if file_name.starts_with('P') || file_name.starts_with('p') {
                    if entry.file_type()?.is_dir() {
                        match self.convert_single_rule(&entry.path(), SunLinterCategory::Performance) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.performance_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", file_name, e)
                                );
                            }
                        }
                    } else if file_name.ends_with(".js") {
                        match self.convert_js_file(&entry.path(), SunLinterCategory::Performance) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.performance_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", file_name, e)
                                );
                            }
                        }
                    }
                }
            }
        }

        println!("  ✅ Converted {} P-series rules", self.conversion_stats.performance_rules);
        Ok(())
    }

    /// Convert Migration (M-series) rules
    fn convert_migration_rules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Converting Migration (M-series) rules...");

        if Path::new(path).exists() {
            let entries = fs::read_dir(path)?;

            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name().to_string_lossy().to_string();

                if file_name.starts_with('M') || file_name.starts_with('m') {
                    if entry.file_type()?.is_dir() {
                        match self.convert_single_rule(&entry.path(), SunLinterCategory::Migration) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.migration_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", file_name, e)
                                );
                            }
                        }
                    } else if file_name.ends_with(".js") {
                        match self.convert_js_file(&entry.path(), SunLinterCategory::Migration) {
                            Ok(rule) => {
                                self.converted_rules.push(rule);
                                self.conversion_stats.migration_rules += 1;
                            }
                            Err(e) => {
                                self.conversion_stats.conversion_errors.push(
                                    format!("Failed to convert {}: {}", file_name, e)
                                );
                            }
                        }
                    }
                }
            }
        }

        println!("  ✅ Converted {} M-series rules", self.conversion_stats.migration_rules);
        Ok(())
    }

    /// Convert a single rule from directory structure (analyzer.js + config.json)
    fn convert_single_rule(&self, rule_path: &Path, category: SunLinterCategory) -> Result<SunLinterRuleConfig, Box<dyn std::error::Error>> {
        let rule_name = rule_path.file_name()
            .ok_or("Invalid rule path")?
            .to_str()
            .ok_or("Invalid rule name")?;

        // Extract rule ID from name (e.g., "C002_no_duplicate_code" -> "C002")
        let rule_id = if let Some(underscore_pos) = rule_name.find('_') {
            rule_name[..underscore_pos].to_string()
        } else {
            rule_name.to_string()
        };

        // Read analyzer.js file
        let analyzer_path = rule_path.join("analyzer.js");
        let analyzer_content = if analyzer_path.exists() {
            fs::read_to_string(&analyzer_path)?
        } else {
            return Err(format!("analyzer.js not found for rule {}", rule_name).into());
        };

        // Read config.json if it exists
        let config_path = rule_path.join("config.json");
        let config_content = if config_path.exists() {
            fs::read_to_string(&config_path)?
        } else {
            "{}".to_string()
        };

        // Parse configuration
        let config_json: Value = serde_json::from_str(&config_content)
            .unwrap_or_else(|_| serde_json::json!({}));

        // Extract patterns from analyzer content
        let patterns = self.extract_patterns_from_analyzer(&analyzer_content);
        let ast_selectors = self.extract_ast_selectors(&analyzer_content);

        // Build rule configuration
        let rule_config = SunLinterRuleConfig {
            rule_id: rule_id.clone(),
            name: self.extract_rule_name(&analyzer_content, &rule_id),
            description: self.extract_description(&analyzer_content, &rule_id),
            category,
            min_lines: config_json.get("minLines").and_then(|v| v.as_u64()).map(|v| v as usize),
            ignore_comments: config_json.get("ignoreComments").and_then(|v| v.as_bool()),
            similarity_threshold: config_json.get("similarityThreshold").and_then(|v| v.as_f64()),
            patterns,
            ast_selectors,
        };

        Ok(rule_config)
    }

    /// Convert a standalone JavaScript file
    fn convert_js_file(&self, file_path: &Path, category: SunLinterCategory) -> Result<SunLinterRuleConfig, Box<dyn std::error::Error>> {
        let file_name = file_path.file_name()
            .ok_or("Invalid file path")?
            .to_str()
            .ok_or("Invalid file name")?;

        // Extract rule ID from filename
        let rule_id = self.extract_rule_id_from_filename(file_name);

        // Read file content
        let content = fs::read_to_string(file_path)?;

        // Extract information from content
        let patterns = self.extract_patterns_from_content(&content);
        let ast_selectors = self.extract_ast_selectors(&content);

        let rule_config = SunLinterRuleConfig {
            rule_id: rule_id.clone(),
            name: self.extract_rule_name(&content, &rule_id),
            description: self.extract_description(&content, &rule_id),
            category,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns,
            ast_selectors,
        };

        Ok(rule_config)
    }

    /// Extract rule patterns from analyzer content
    fn extract_patterns_from_analyzer(&self, content: &str) -> Vec<String> {
        let mut patterns = Vec::new();

        // Look for regex patterns in the code
        let regex_pattern = Regex::new(r#"(?:regex|pattern|match)\s*[=:]\s*[`"'/]([^`"'/]+)[`"'/]"#).unwrap();
        for caps in regex_pattern.captures_iter(content) {
            if let Some(pattern) = caps.get(1) {
                patterns.push(pattern.as_str().to_string());
            }
        }

        // Look for specific patterns in different formats
        let specific_patterns = [
            r#"\.includes\(['"`]([^'"`]+)['"`]\)"#,
            r#"\.match\(/([^/]+)/g?\)"#,
            r#"new RegExp\(['"`]([^'"`]+)['"`]"#,
            r#"/([^/]+)/g?\.test"#,
        ];

        for pattern_regex in &specific_patterns {
            if let Ok(regex) = Regex::new(pattern_regex) {
                for caps in regex.captures_iter(content) {
                    if let Some(pattern) = caps.get(1) {
                        patterns.push(pattern.as_str().to_string());
                    }
                }
            }
        }

        patterns
    }

    /// Extract patterns from content using multiple strategies
    fn extract_patterns_from_content(&self, content: &str) -> Vec<String> {
        self.extract_patterns_from_analyzer(content)
    }

    /// Extract AST selectors from content
    fn extract_ast_selectors(&self, content: &str) -> Vec<String> {
        let mut selectors = Vec::new();

        // Common AST node types to look for
        let ast_patterns = [
            "FunctionDeclaration",
            "VariableDeclaration",
            "ClassDeclaration",
            "InterfaceDeclaration",
            "TSInterfaceDeclaration",
            "CatchClause",
            "CallExpression",
            "MemberExpression",
            "BinaryExpression",
            "AssignmentExpression",
            "Identifier",
            "Literal",
        ];

        for pattern in &ast_patterns {
            if content.contains(pattern) {
                selectors.push(pattern.to_string());
            }
        }

        selectors
    }

    /// Extract rule name from content
    fn extract_rule_name(&self, content: &str, rule_id: &str) -> String {
        // Look for rule name in comments or meta.docs.description
        if let Some(caps) = Regex::new(r#"description:\s*['"`]([^'"`]+)['"`]"#).unwrap().captures(content) {
            if let Some(name) = caps.get(1) {
                return name.as_str().to_string();
            }
        }

        // Look in comments
        if let Some(caps) = Regex::new(r#"/\*\*\s*\n\s*\*\s*([^\n]+)"#).unwrap().captures(content) {
            if let Some(name) = caps.get(1) {
                return name.as_str().trim().to_string();
            }
        }

        // Fallback to rule ID
        rule_id.to_string()
    }

    /// Extract description from content
    fn extract_description(&self, content: &str, rule_id: &str) -> String {
        // Look for Purpose or description
        if let Some(caps) = Regex::new(r#"Purpose:\s*([^\n]+)"#).unwrap().captures(content) {
            if let Some(desc) = caps.get(1) {
                return desc.as_str().trim().to_string();
            }
        }

        if let Some(caps) = Regex::new(r#"description:\s*['"`]([^'"`]+)['"`]"#).unwrap().captures(content) {
            if let Some(desc) = caps.get(1) {
                return desc.as_str().to_string();
            }
        }

        format!("SunLinter rule {}", rule_id)
    }

    /// Extract rule ID from filename
    fn extract_rule_id_from_filename(&self, filename: &str) -> String {
        // Handle formats like "t002-interface-prefix-i.js" -> "T002"
        if let Some(dash_pos) = filename.find('-') {
            let id_part = &filename[..dash_pos];
            return id_part.to_uppercase();
        }

        // Handle formats like "C002_no_duplicate_code.js" -> "C002"
        if let Some(underscore_pos) = filename.find('_') {
            let id_part = &filename[..underscore_pos];
            return id_part.to_uppercase();
        }

        // Remove .js extension and return as-is
        filename.replace(".js", "").to_uppercase()
    }

    /// Print detailed conversion statistics
    fn print_conversion_stats(&self) {
        println!("\n📊 Conversion Statistics:");
        println!("  Total Rules Converted: {}", self.conversion_stats.total_rules);
        println!("  Common (C-series): {}", self.conversion_stats.common_rules);
        println!("  Security (S-series): {}", self.conversion_stats.security_rules);
        println!("  Quality (T-series): {}", self.conversion_stats.quality_rules);
        println!("  Performance (P-series): {}", self.conversion_stats.performance_rules);
        println!("  Migration (M-series): {}", self.conversion_stats.migration_rules);

        if !self.conversion_stats.conversion_errors.is_empty() {
            println!("\n⚠️  Conversion Errors ({}):", self.conversion_stats.conversion_errors.len());
            for error in &self.conversion_stats.conversion_errors {
                println!("  - {}", error);
            }
        }

        println!("\n✅ Conversion complete! SunLinter rules ready for WASM integration.");
    }

    /// Get all converted rules
    pub fn get_converted_rules(&self) -> &[SunLinterRuleConfig] {
        &self.converted_rules
    }

    /// Save converted rules to JSON file for inspection
    pub fn save_converted_rules(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.converted_rules)?;
        fs::write(output_path, json)?;
        println!("💾 Converted rules saved to: {}", output_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_rule_id_extraction() {
        let converter = SunLinterRuleConverter::new();

        assert_eq!(converter.extract_rule_id_from_filename("t002-interface-prefix-i.js"), "T002");
        assert_eq!(converter.extract_rule_id_from_filename("C002_no_duplicate_code.js"), "C002");
        assert_eq!(converter.extract_rule_id_from_filename("s005.js"), "S005");
    }

    #[test]
    fn test_pattern_extraction() {
        let converter = SunLinterRuleConverter::new();
        let content = r#"
            const pattern = /function\s+\w+\s*\(/;
            if (line.includes("interface ")) {
                return true;
            }
        "#;

        let patterns = converter.extract_patterns_from_content(content);
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_ast_selector_extraction() {
        let converter = SunLinterRuleConverter::new();
        let content = r#"
            TSInterfaceDeclaration(node) {
                const interfaceName = node.id.name;
                if (!interfaceName.startsWith("I")) {
                    context.report({
                        node: node.id,
                        messageId: "interfacePrefix"
                    });
                }
            }
        "#;

        let selectors = converter.extract_ast_selectors(content);
        assert!(selectors.contains(&"TSInterfaceDeclaration".to_string()));
    }
}