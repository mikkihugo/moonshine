//! # Configurable Pattern Detection for Moon-Shine
//!
//! Production-ready pattern configuration system that allows dynamic loading
//! of detection rules without recompilation. Supports JSON-based rule definitions
//! with hot-reloading capabilities for development and customization.
//!
//! @category configuration
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 2.0.0

use crate::error::{Error, Result};
use crate::linter::SuggestionSeverity;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for pattern-based code analysis rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
  /// Security-related patterns with high impact
  pub security_patterns: Vec<PatternRule>,
  /// Performance-related patterns
  pub performance_patterns: Vec<PatternRule>,
  /// TypeScript/JavaScript specific patterns
  pub typescript_patterns: Vec<PatternRule>,
  /// Documentation patterns (TODO, FIXME, etc.)
  pub documentation_patterns: Vec<PatternRule>,
  /// Custom user-defined patterns
  pub custom_patterns: Vec<PatternRule>,
}

/// Individual pattern rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRule {
  /// Unique identifier for the rule
  pub id: String,
  /// Human-readable name
  pub name: String,
  /// Detailed description of what the pattern detects
  pub description: String,
  /// Regular expression pattern
  pub pattern: String,
  /// Severity level for matches
  pub severity: SuggestionSeverity,
  /// Category for grouping
  pub category: String,
  /// Whether this rule is enabled
  pub enabled: bool,
  /// Suggested fix template (optional)
  pub fix_template: Option<String>,
  /// Languages this rule applies to
  pub languages: Vec<String>,
  /// Impact score (1-10)
  pub impact_score: u8,
}

/// Compiled pattern cache for performance
#[derive(Debug)]
pub struct CompiledPatterns {
  pub security: Vec<(Regex, PatternRule)>,
  pub performance: Vec<(Regex, PatternRule)>,
  pub typescript: Vec<(Regex, PatternRule)>,
  pub documentation: Vec<(Regex, PatternRule)>,
  pub custom: Vec<(Regex, PatternRule)>,
}

impl Default for PatternConfig {
  fn default() -> Self {
    Self {
            security_patterns: vec![
                PatternRule {
                    id: "hardcoded-api-key".to_string(),
                    name: "Hardcoded API Key".to_string(),
                    description: "Detects potential hardcoded API keys or secrets".to_string(),
                    pattern: r#"(?i)(api[_-]?key|secret|token|password)\s*[:=]\s*['\"][a-zA-Z0-9]{16,}['\"]"#.to_string(),
                    severity: SuggestionSeverity::Critical,
                    category: "security".to_string(),
                    enabled: true,
                    fix_template: Some("Use environment variables or secure storage".to_string()),
                    languages: vec!["typescript".to_string(), "javascript".to_string(), "python".to_string()],
                    impact_score: 10,
                },
                PatternRule {
                    id: "sql-injection".to_string(),
                    name: "SQL Injection Risk".to_string(),
                    description: "Detects potential SQL injection vulnerabilities".to_string(),
                    pattern: r#"(?i)(query|execute)\s*\(\s*['"]\s*select.*\+.*['"]\s*\)"#.to_string(),
                    severity: SuggestionSeverity::Critical,
                    category: "security".to_string(),
                    enabled: true,
                    fix_template: Some("Use parameterized queries or prepared statements".to_string()),
                    languages: vec!["typescript".to_string(), "javascript".to_string(), "python".to_string()],
                    impact_score: 10,
                },
            ],
            performance_patterns: vec![
                PatternRule {
                    id: "inefficient-loop".to_string(),
                    name: "Inefficient Loop Pattern".to_string(),
                    description: "Detects potentially inefficient loop patterns".to_string(),
                    pattern: r"for\s*\(\s*.*;\s*.*\.length\s*;\s*.*\)\s*\{".to_string(),
                    severity: SuggestionSeverity::Warning,
                    category: "performance".to_string(),
                    enabled: true,
                    fix_template: Some("Cache array length or use for...of loop".to_string()),
                    languages: vec!["typescript".to_string(), "javascript".to_string()],
                    impact_score: 6,
                },
                PatternRule {
                    id: "synchronous-fs".to_string(),
                    name: "Synchronous File Operations".to_string(),
                    description: "Detects synchronous file system operations that block the event loop".to_string(),
                    pattern: r"fs\.(readFileSync|writeFileSync|existsSync|statSync)".to_string(),
                    severity: SuggestionSeverity::Warning,
                    category: "performance".to_string(),
                    enabled: true,
                    fix_template: Some("Use async variants: readFile, writeFile, access, stat".to_string()),
                    languages: vec!["typescript".to_string(), "javascript".to_string()],
                    impact_score: 7,
                },
            ],
            typescript_patterns: vec![
                PatternRule {
                    id: "missing-type-annotation".to_string(),
                    name: "Missing Type Annotation".to_string(),
                    description: "Detects variables without explicit type annotations".to_string(),
                    pattern: r"(let|const|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=".to_string(),
                    severity: SuggestionSeverity::Info,
                    category: "typescript".to_string(),
                    enabled: true,
                    fix_template: Some("Add explicit type annotation: const variable: Type = value".to_string()),
                    languages: vec!["typescript".to_string()],
                    impact_score: 3,
                },
                PatternRule {
                    id: "any-type-usage".to_string(),
                    name: "Any Type Usage".to_string(),
                    description: "Detects usage of 'any' type which reduces type safety".to_string(),
                    pattern: r":\s*any(\s|;|,|\)|$)".to_string(),
                    severity: SuggestionSeverity::Warning,
                    category: "typescript".to_string(),
                    enabled: true,
                    fix_template: Some("Use specific types instead of 'any' for better type safety".to_string()),
                    languages: vec!["typescript".to_string()],
                    impact_score: 5,
                },
            ],
            documentation_patterns: vec![
                PatternRule {
                    id: "todo-markers".to_string(),
                    name: "TODO/FIXME Markers".to_string(),
                    description: "Detects documentation markers indicating incomplete work".to_string(),
                    pattern: r"(?i)(TODO|FIXME|HACK|XXX|NOTE|BUG|DEPRECATED):".to_string(),
                    severity: SuggestionSeverity::Info,
                    category: "documentation".to_string(),
                    enabled: true,
                    fix_template: None,
                    languages: vec!["*".to_string()],
                    impact_score: 2,
                },
                PatternRule {
                    id: "missing-jsdoc".to_string(),
                    name: "Missing JSDoc".to_string(),
                    description: "Detects functions without JSDoc documentation".to_string(),
                    pattern: r"(export\s+)?(async\s+)?function\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(.*\)\s*(\{|:)".to_string(),
                    severity: SuggestionSeverity::Info,
                    category: "documentation".to_string(),
                    enabled: false, // Disabled by default - can be noisy
                    fix_template: Some("Add JSDoc comment above function".to_string()),
                    languages: vec!["typescript".to_string(), "javascript".to_string()],
                    impact_score: 3,
                },
            ],
            custom_patterns: vec![],
        }
  }
}

impl PatternConfig {
  /// Load pattern configuration from JSON file
  pub fn load_from_file(path: &str) -> Result<Self> {
    let content = std::fs::read_to_string(path).map_err(|e| {
      Error::config(format!("Failed to read pattern config: {}", e))
    })?;

    let config: PatternConfig =
      serde_json::from_str(&content).map_err(|e| {
        Error::config(format!("Failed to parse pattern config: {}", e))
      })?;

    Ok(config)
  }

  /// Save pattern configuration to JSON file
  pub fn save_to_file(&self, path: &str) -> Result<()> {
    let content = serde_json::to_string_pretty(self).map_err(|e| {
      Error::config(format!("Failed to serialize pattern config: {}", e))
    })?;

    std::fs::write(path, content).map_err(|e| {
      Error::config(format!("Failed to write pattern config: {}", e))
    })?;

    Ok(())
  }

  /// Load from Moon workspace config or use defaults
  pub fn from_moon_workspace() -> Result<Self> {
    let moon_config =
      crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();

    // Use pattern rules from config if available, otherwise defaults
    Ok(moon_config.pattern_rules.unwrap_or_else(|| Self::default()))
  }

  /// Compile patterns into regex objects for performance
  pub fn compile(&self) -> Result<CompiledPatterns> {
    let mut compiled = CompiledPatterns {
      security: Vec::new(),
      performance: Vec::new(),
      typescript: Vec::new(),
      documentation: Vec::new(),
      custom: Vec::new(),
    };

    // Compile security patterns
    for rule in &self.security_patterns {
      if rule.enabled {
        let regex = Regex::new(&rule.pattern).map_err(|e| {
          Error::config(format!(
            "Invalid security pattern '{}': {}",
            rule.id, e
          ))
        })?;
        compiled.security.push((regex, rule.clone()));
      }
    }

    // Compile performance patterns
    for rule in &self.performance_patterns {
      if rule.enabled {
        let regex = Regex::new(&rule.pattern).map_err(|e| {
          Error::config(format!(
            "Invalid performance pattern '{}': {}",
            rule.id, e
          ))
        })?;
        compiled.performance.push((regex, rule.clone()));
      }
    }

    // Compile TypeScript patterns
    for rule in &self.typescript_patterns {
      if rule.enabled {
        let regex = Regex::new(&rule.pattern).map_err(|e| {
          Error::config(format!(
            "Invalid TypeScript pattern '{}': {}",
            rule.id, e
          ))
        })?;
        compiled.typescript.push((regex, rule.clone()));
      }
    }

    // Compile documentation patterns
    for rule in &self.documentation_patterns {
      if rule.enabled {
        let regex = Regex::new(&rule.pattern).map_err(|e| {
          Error::config(format!(
            "Invalid documentation pattern '{}': {}",
            rule.id, e
          ))
        })?;
        compiled.documentation.push((regex, rule.clone()));
      }
    }

    // Compile custom patterns
    for rule in &self.custom_patterns {
      if rule.enabled {
        let regex = Regex::new(&rule.pattern).map_err(|e| {
          Error::config(format!("Invalid custom pattern '{}': {}", rule.id, e))
        })?;
        compiled.custom.push((regex, rule.clone()));
      }
    }

    Ok(compiled)
  }

  /// Add a custom pattern rule
  pub fn add_custom_pattern(&mut self, rule: PatternRule) {
    self.custom_patterns.push(rule);
  }

  /// Remove a pattern rule by ID
  pub fn remove_pattern(&mut self, id: &str) -> bool {
    let mut removed = false;

    self.security_patterns.retain(|r| {
      r.id != id || {
        removed = true;
        false
      }
    });
    self.performance_patterns.retain(|r| {
      r.id != id || {
        removed = true;
        false
      }
    });
    self.typescript_patterns.retain(|r| {
      r.id != id || {
        removed = true;
        false
      }
    });
    self.documentation_patterns.retain(|r| {
      r.id != id || {
        removed = true;
        false
      }
    });
    self.custom_patterns.retain(|r| {
      r.id != id || {
        removed = true;
        false
      }
    });

    removed
  }

  /// Enable or disable a pattern rule by ID
  pub fn set_pattern_enabled(&mut self, id: &str, enabled: bool) -> bool {
    let mut found = false;

    for rule in &mut self.security_patterns {
      if rule.id == id {
        rule.enabled = enabled;
        found = true;
        break;
      }
    }

    if !found {
      for rule in &mut self.performance_patterns {
        if rule.id == id {
          rule.enabled = enabled;
          found = true;
          break;
        }
      }
    }

    if !found {
      for rule in &mut self.typescript_patterns {
        if rule.id == id {
          rule.enabled = enabled;
          found = true;
          break;
        }
      }
    }

    if !found {
      for rule in &mut self.documentation_patterns {
        if rule.id == id {
          rule.enabled = enabled;
          found = true;
          break;
        }
      }
    }

    if !found {
      for rule in &mut self.custom_patterns {
        if rule.id == id {
          rule.enabled = enabled;
          found = true;
          break;
        }
      }
    }

    found
  }

  /// Get all pattern rules as a flat list
  pub fn get_all_rules(&self) -> Vec<&PatternRule> {
    let mut all_rules = Vec::new();
    all_rules.extend(&self.security_patterns);
    all_rules.extend(&self.performance_patterns);
    all_rules.extend(&self.typescript_patterns);
    all_rules.extend(&self.documentation_patterns);
    all_rules.extend(&self.custom_patterns);
    all_rules
  }

  /// Get enabled rules count by category
  pub fn get_enabled_counts(&self) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    counts.insert(
      "security".to_string(),
      self.security_patterns.iter().filter(|r| r.enabled).count(),
    );
    counts.insert(
      "performance".to_string(),
      self
        .performance_patterns
        .iter()
        .filter(|r| r.enabled)
        .count(),
    );
    counts.insert(
      "typescript".to_string(),
      self
        .typescript_patterns
        .iter()
        .filter(|r| r.enabled)
        .count(),
    );
    counts.insert(
      "documentation".to_string(),
      self
        .documentation_patterns
        .iter()
        .filter(|r| r.enabled)
        .count(),
    );
    counts.insert(
      "custom".to_string(),
      self.custom_patterns.iter().filter(|r| r.enabled).count(),
    );
    counts
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_pattern_config() {
    let config = PatternConfig::default();
    assert!(!config.security_patterns.is_empty());
    assert!(!config.performance_patterns.is_empty());
    assert!(!config.typescript_patterns.is_empty());
    assert!(!config.documentation_patterns.is_empty());
  }

  #[test]
  fn test_pattern_compilation() {
    let config = PatternConfig::default();
    let compiled = config.compile().expect("Should compile successfully");

    assert!(!compiled.security.is_empty());
    assert!(!compiled.performance.is_empty());
    assert!(!compiled.typescript.is_empty());
    assert!(!compiled.documentation.is_empty());
  }

  #[test]
  fn test_pattern_management() {
    let mut config = PatternConfig::default();

    // Test adding custom pattern
    let custom_rule = PatternRule {
      id: "test-rule".to_string(),
      name: "Test Rule".to_string(),
      description: "Test description".to_string(),
      pattern: r"test_pattern".to_string(),
      severity: SuggestionSeverity::Warning,
      category: "custom".to_string(),
      enabled: true,
      fix_template: None,
      languages: vec!["rust".to_string()],
      impact_score: 5,
    };

    config.add_custom_pattern(custom_rule);
    assert_eq!(config.custom_patterns.len(), 1);

    // Test disabling pattern
    assert!(config.set_pattern_enabled("test-rule", false));
    assert!(!config.custom_patterns[0].enabled);

    // Test removing pattern
    assert!(config.remove_pattern("test-rule"));
    assert!(config.custom_patterns.is_empty());
  }

  #[test]
  fn test_json_serialization() {
    let config = PatternConfig::default();
    let json = serde_json::to_string(&config).expect("Should serialize");
    let deserialized: PatternConfig =
      serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(
      config.security_patterns.len(),
      deserialized.security_patterns.len()
    );
    assert_eq!(
      config.performance_patterns.len(),
      deserialized.performance_patterns.len()
    );
  }
}
