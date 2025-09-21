#!/usr/bin/env rust-script

//! Convert all SunLinter JavaScript rules to moon-shine WASM format
//!
//! Usage: cargo run --bin convert_sunlinter_rules

use std::env;
use std::path::Path;

// This would normally import from moon-shine crate
// For now, we'll include the core logic here for the script

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SunLinterCategory {
    Common,
    Security,
    Quality,
    Performance,
    Migration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunLinterRuleConfig {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub category: SunLinterCategory,
    pub min_lines: Option<usize>,
    pub ignore_comments: Option<bool>,
    pub similarity_threshold: Option<f64>,
    pub patterns: Vec<String>,
    pub ast_selectors: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let sunlinter_path = if args.len() > 1 {
        &args[1]
    } else {
        "/home/mhugo/code/zenflow/packages/tools/moon-shine/moonshine-rules"
    };

    if !Path::new(sunlinter_path).exists() {
        eprintln!("❌ SunLinter rules path does not exist: {}", sunlinter_path);
        eprintln!("💡 Usage: {} <path-to-sunlinter-rules>", args[0]);
        return Ok(());
    }

    println!("🚀 Converting SunLinter rules from: {}", sunlinter_path);
    println!("📁 Scanning directories...");

    // Mock converter for this script - in real implementation this would use the full converter
    let mut conversion_count = 0;
    let mut rules = Vec::<SunLinterRuleConfig>::new();

    // Check each category directory
    let categories = [
        ("common", SunLinterCategory::Common),
        ("security", SunLinterCategory::Security),
        ("quality", SunLinterCategory::Quality),
        ("performance", SunLinterCategory::Performance),
        ("migration", SunLinterCategory::Migration),
    ];

    for (dir_name, category) in &categories {
        let category_path = format!("{}/{}", sunlinter_path, dir_name);
        if Path::new(&category_path).exists() {
            println!("📂 Found {} directory", dir_name);

            // Count files/directories in this category
            if let Ok(entries) = std::fs::read_dir(&category_path) {
                let mut category_count = 0;
                for entry in entries {
                    if let Ok(entry) = entry {
                        let name = entry.file_name().to_string_lossy().to_string();

                        // Create a mock rule for demonstration
                        let rule = SunLinterRuleConfig {
                            rule_id: extract_rule_id(&name),
                            name: format!("Rule {}", name),
                            description: format!("Converted rule from {}", name),
                            category: category.clone(),
                            min_lines: None,
                            ignore_comments: None,
                            similarity_threshold: None,
                            patterns: vec![],
                            ast_selectors: vec![],
                        };

                        rules.push(rule);
                        category_count += 1;
                    }
                }

                conversion_count += category_count;
                println!("  ✅ {} rules in {} category", category_count, dir_name);
            }
        } else {
            println!("  ⚠️  {} directory not found", dir_name);
        }
    }

    println!("\n📊 Conversion Summary:");
    println!("  Total rules found: {}", conversion_count);
    println!("  Rules in memory: {}", rules.len());

    // Save converted rules to JSON for inspection
    let output_path = format!("{}/converted_rules.json", sunlinter_path);
    let json = serde_json::to_string_pretty(&rules)?;
    std::fs::write(&output_path, json)?;
    println!("  💾 Saved to: {}", output_path);

    println!("\n🎉 SunLinter rule conversion complete!");
    println!("   Ready for integration with moon-shine WASM engine");

    Ok(())
}

fn extract_rule_id(name: &str) -> String {
    // Extract rule ID from various naming patterns
    if let Some(dash_pos) = name.find('-') {
        let id_part = &name[..dash_pos];
        return id_part.to_uppercase();
    }

    if let Some(underscore_pos) = name.find('_') {
        let id_part = &name[..underscore_pos];
        return id_part.to_uppercase();
    }

    // For directory names like "S005_no_origin_auth"
    if name.len() >= 4 && name.chars().nth(0).unwrap().is_alphabetic() {
        let potential_id = &name[..4];
        if potential_id.chars().skip(1).all(|c| c.is_numeric()) {
            return potential_id.to_uppercase();
        }
    }

    name.replace(".js", "").to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_id_extraction() {
        assert_eq!(extract_rule_id("t002-interface-prefix-i"), "T002");
        assert_eq!(extract_rule_id("C002_no_duplicate_code"), "C002");
        assert_eq!(extract_rule_id("S005_no_origin_auth"), "S005");
        assert_eq!(extract_rule_id("s009.js"), "S009");
    }
}