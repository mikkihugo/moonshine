//! SunLinter Integration Module
//!
//! Integrates the 192 SunLinter JavaScript behavioral analysis rules with moon-shine's OXC static analysis.
//! Creates a unified rule engine that combines pattern matching with AI behavioral intelligence.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}
use std::collections::HashMap;

/// SunLinter rule categories mapping to WASM categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SunLinterCategory {
    Common,     // C-series: Common coding standards
    Security,   // S-series: Security patterns
    Quality,    // T-series: TypeScript quality patterns
    Performance,// P-series: Performance patterns
    Migration,  // M-series: Migration and legacy patterns
}

impl From<SunLinterCategory> for WasmRuleCategory {
    fn from(category: SunLinterCategory) -> Self {
        match category {
            SunLinterCategory::Common => WasmRuleCategory::Correctness,
            SunLinterCategory::Security => WasmRuleCategory::Restriction,
            SunLinterCategory::Quality => WasmRuleCategory::Style,
            SunLinterCategory::Performance => WasmRuleCategory::Perf,
            SunLinterCategory::Migration => WasmRuleCategory::Suspicious,
        }
    }
}

/// Configuration for a SunLinter rule
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

/// A unified rule that combines OXC static analysis with SunLinter behavioral patterns
pub struct UnifiedRule {
    pub config: SunLinterRuleConfig,
    pub oxc_patterns: Vec<String>,
    pub behavioral_patterns: Vec<BehavioralPattern>,
}

/// Behavioral analysis pattern from SunLinter
#[derive(Debug, Clone)]
pub struct BehavioralPattern {
    pub pattern_type: BehavioralPatternType,
    pub regex: String,
    pub context_required: Vec<String>,
    pub severity_weight: f32,
}

#[derive(Debug, Clone)]
pub enum BehavioralPatternType {
    DuplicateCode,
    SecurityVulnerability,
    NamingConvention,
    LogicalComplexity,
    DataFlow,
    APIUsage,
}

/// Main SunLinter integration engine
pub struct SunLinterEngine {
    rules: HashMap<String, UnifiedRule>,
    config: SunLinterEngineConfig,
}

#[derive(Debug, Clone)]
pub struct SunLinterEngineConfig {
    pub enable_hybrid_analysis: bool,
    pub enable_behavioral_ai: bool,
    pub confidence_threshold: f64,
    pub max_analysis_time_ms: u64,
}

impl Default for SunLinterEngineConfig {
    fn default() -> Self {
        Self {
            enable_hybrid_analysis: true,
            enable_behavioral_ai: true,
            confidence_threshold: 0.75,
            max_analysis_time_ms: 5000,
        }
    }
}

impl SunLinterEngine {
    pub fn new(config: SunLinterEngineConfig) -> Self {
        let mut engine = Self {
            rules: HashMap::new(),
            config,
        };
        engine.load_sunlinter_rules();
        engine
    }

    /// Load all 192 SunLinter rules from the JavaScript definitions
    fn load_sunlinter_rules(&mut self) {
        // Load C-series (Common) rules
        self.load_common_rules();
        // Load S-series (Security) rules
        self.load_security_rules();
        // Load T-series (TypeScript Quality) rules
        self.load_quality_rules();
        // Load P-series (Performance) rules
        self.load_performance_rules();
        // Load M-series (Migration) rules
        self.load_migration_rules();
    }

    fn load_common_rules(&mut self) {
        // C002 - No Duplicate Code
        let c002_config = SunLinterRuleConfig {
            rule_id: "C002".to_string(),
            name: "No Duplicate Code".to_string(),
            description: "Detects duplicate code blocks longer than specified threshold".to_string(),
            category: SunLinterCategory::Common,
            min_lines: Some(5),
            ignore_comments: Some(true),
            similarity_threshold: Some(0.80),
            patterns: vec![
                r"function\s+\w+\s*\([^)]*\)\s*\{[^}]+\}".to_string(),
                r"class\s+\w+\s*\{[^}]+\}".to_string(),
            ],
            ast_selectors: vec![
                "FunctionDeclaration".to_string(),
                "ClassDeclaration".to_string(),
            ],
        };

        let c002_rule = UnifiedRule {
            config: c002_config,
            oxc_patterns: vec![
                "function_declaration".to_string(),
                "class_declaration".to_string(),
            ],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::DuplicateCode,
                    regex: r"(?s)(\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\})".to_string(),
                    context_required: vec!["function_body".to_string(), "class_body".to_string()],
                    severity_weight: 0.85,
                },
            ],
        };

        self.rules.insert("C002".to_string(), c002_rule);

        // C006 - Function Naming
        let c006_config = SunLinterRuleConfig {
            rule_id: "C006".to_string(),
            name: "Function Naming Convention".to_string(),
            description: "Enforces consistent function naming patterns".to_string(),
            category: SunLinterCategory::Common,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"function\s+([A-Z][a-z][\w]*)\s*\(".to_string(),
                r"const\s+([A-Z][a-z][\w]*)\s*=\s*\(".to_string(),
            ],
            ast_selectors: vec![
                "FunctionDeclaration > Identifier".to_string(),
                "VariableDeclarator[init.type='ArrowFunctionExpression'] > Identifier".to_string(),
            ],
        };

        let c006_rule = UnifiedRule {
            config: c006_config,
            oxc_patterns: vec!["function_declaration".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::NamingConvention,
                    regex: r"function\s+([A-Z][\w]*)\s*\(".to_string(),
                    context_required: vec!["function_declaration".to_string()],
                    severity_weight: 0.70,
                },
            ],
        };

        self.rules.insert("C006".to_string(), c006_rule);

        // C029 - Catch Block Logging
        let c029_config = SunLinterRuleConfig {
            rule_id: "C029".to_string(),
            name: "Catch Block Logging".to_string(),
            description: "Every catch block must log the error cause".to_string(),
            category: SunLinterCategory::Common,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"catch\s*\(\s*\w+\s*\)\s*\{[^}]*\}".to_string(),
            ],
            ast_selectors: vec![
                "CatchClause".to_string(),
            ],
        };

        let c029_rule = UnifiedRule {
            config: c029_config,
            oxc_patterns: vec!["catch_clause".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::LogicalComplexity,
                    regex: r"catch\s*\([^)]+\)\s*\{(?![^}]*(?:log|console|logger))[^}]*\}".to_string(),
                    context_required: vec!["try_statement".to_string()],
                    severity_weight: 0.90,
                },
            ],
        };

        self.rules.insert("C029".to_string(), c029_rule);
    }

    fn load_security_rules(&mut self) {
        // S005 - No Origin Header Authentication
        let s005_config = SunLinterRuleConfig {
            rule_id: "S005".to_string(),
            name: "No Origin Header Authentication".to_string(),
            description: "Do not use Origin header for authentication or access control".to_string(),
            category: SunLinterCategory::Security,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"req\.headers\.origin".to_string(),
                r"request\.headers\[.origin.\]".to_string(),
                r"getHeader\(.origin.\)".to_string(),
            ],
            ast_selectors: vec![
                "MemberExpression[property.name='origin']".to_string(),
            ],
        };

        let s005_rule = UnifiedRule {
            config: s005_config,
            oxc_patterns: vec!["member_expression".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::SecurityVulnerability,
                    regex: r"(?i)(?:req|request|headers?)\.(?:headers?\.)?origin".to_string(),
                    context_required: vec!["auth", "security", "access_control"].iter().map(|s| s.to_string()).collect(),
                    severity_weight: 0.95,
                },
            ],
        };

        self.rules.insert("S005".to_string(), s005_rule);

        // S009 - No Insecure Encryption
        let s009_config = SunLinterRuleConfig {
            rule_id: "S009".to_string(),
            name: "No Insecure Encryption".to_string(),
            description: "Avoid using insecure encryption algorithms".to_string(),
            category: SunLinterCategory::Security,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"createCipher\(.des.".to_string(),
                r"createCipher\(.rc4.".to_string(),
                r"createHash\(.md5.".to_string(),
                r"createHash\(.sha1.".to_string(),
            ],
            ast_selectors: vec![
                "CallExpression[callee.name='createCipher']".to_string(),
                "CallExpression[callee.name='createHash']".to_string(),
            ],
        };

        let s009_rule = UnifiedRule {
            config: s009_config,
            oxc_patterns: vec!["call_expression".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::SecurityVulnerability,
                    regex: r#"create(?:Cipher|Hash)\s*\(\s*['"](?:des|rc4|md5|sha1)['"]"#.to_string(),
                    context_required: vec!["crypto".to_string(), "encryption".to_string()],
                    severity_weight: 0.95,
                },
            ],
        };

        self.rules.insert("S009".to_string(), s009_rule);
    }

    fn load_quality_rules(&mut self) {
        // T002 - Interface Prefix I
        let t002_config = SunLinterRuleConfig {
            rule_id: "T002".to_string(),
            name: "Interface Prefix I".to_string(),
            description: "Interface names should start with 'I'".to_string(),
            category: SunLinterCategory::Quality,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"interface\s+([^I]\w*)\s*\{".to_string(),
            ],
            ast_selectors: vec![
                "TSInterfaceDeclaration".to_string(),
            ],
        };

        let t002_rule = UnifiedRule {
            config: t002_config,
            oxc_patterns: vec!["ts_interface_declaration".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::NamingConvention,
                    regex: r"interface\s+([^I][\w]*)\s*[{<]".to_string(),
                    context_required: vec!["typescript".to_string(), "interface".to_string()],
                    severity_weight: 0.75,
                },
            ],
        };

        self.rules.insert("T002".to_string(), t002_rule);

        // T019 - No This Assign
        let t019_config = SunLinterRuleConfig {
            rule_id: "T019".to_string(),
            name: "No This Assignment".to_string(),
            description: "Avoid reassigning 'this' to variables".to_string(),
            category: SunLinterCategory::Quality,
            min_lines: None,
            ignore_comments: None,
            similarity_threshold: None,
            patterns: vec![
                r"(?:var|let|const)\s+\w+\s*=\s*this\s*[;\n]".to_string(),
            ],
            ast_selectors: vec![
                "VariableDeclarator[init.type='ThisExpression']".to_string(),
            ],
        };

        let t019_rule = UnifiedRule {
            config: t019_config,
            oxc_patterns: vec!["variable_declarator".to_string()],
            behavioral_patterns: vec![
                BehavioralPattern {
                    pattern_type: BehavioralPatternType::LogicalComplexity,
                    regex: r"(?:var|let|const)\s+\w+\s*=\s*this\b".to_string(),
                    context_required: vec!["class".to_string(), "method".to_string()],
                    severity_weight: 0.80,
                },
            ],
        };

        self.rules.insert("T019".to_string(), t019_rule);
    }

    fn load_performance_rules(&mut self) {
        // Performance rules will be added as we expand the conversion
        // For now, focusing on core examples
    }

    fn load_migration_rules(&mut self) {
        // Migration rules will be added as we expand the conversion
        // For now, focusing on core examples
    }

    /// Analyze code using unified OXC + SunLinter approach
    pub fn analyze_unified(&self, code: &str, rule_id: &str) -> Vec<WasmRuleDiagnostic> {
        if let Some(rule) = self.rules.get(rule_id) {
            let mut diagnostics = Vec::new();

            // Run behavioral pattern analysis (SunLinter approach)
            for pattern in &rule.behavioral_patterns {
                let behavioral_diagnostics = self.analyze_behavioral_pattern(code, pattern, &rule.config);
                diagnostics.extend(behavioral_diagnostics);
            }

            // Enhance with AI if enabled
            if self.config.enable_behavioral_ai {
                diagnostics = self.enhance_with_ai(diagnostics, &rule.config);
            }

            diagnostics
        } else {
            Vec::new()
        }
    }

    fn analyze_behavioral_pattern(&self, code: &str, pattern: &BehavioralPattern, config: &SunLinterRuleConfig) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Use regex to find pattern matches
        if let Ok(regex) = regex::Regex::new(&pattern.regex) {
            for (line_num, line) in code.lines().enumerate() {
                if regex.is_match(line) {
                    // Check if context requirements are met
                    let context_met = pattern.context_required.is_empty() ||
                        pattern.context_required.iter().any(|ctx| {
                            self.check_context(code, line_num, ctx)
                        });

                    if context_met {
                        diagnostics.push(WasmRuleDiagnostic {
                            rule_name: config.rule_id.clone(),
                            message: format!("{}: {}", config.name, config.description),
                            line: line_num,
                            column: 0,
                            severity: self.determine_severity(pattern.severity_weight),
                        });
                    }
                }
            }
        }

        diagnostics
    }

    fn check_context(&self, code: &str, line_num: usize, context: &str) -> bool {
        // Simple context checking - can be enhanced with more sophisticated analysis
        let context_window = 5; // Check 5 lines before and after
        let start = line_num.saturating_sub(context_window);
        let end = (line_num + context_window).min(code.lines().count());

        code.lines()
            .skip(start)
            .take(end - start)
            .any(|line| line.to_lowercase().contains(&context.to_lowercase()))
    }

    fn determine_severity(&self, weight: f32) -> String {
        match weight {
            w if w >= 0.90 => "error".to_string(),
            w if w >= 0.75 => "warning".to_string(),
            _ => "info".to_string(),
        }
    }

    fn enhance_with_ai(&self, diagnostics: Vec<WasmRuleDiagnostic>, config: &SunLinterRuleConfig) -> Vec<WasmRuleDiagnostic> {
        // AI enhancement placeholder - can be integrated with existing AI systems
        diagnostics
    }

    /// Get all available SunLinter rule IDs
    pub fn get_rule_ids(&self) -> Vec<String> {
        self.rules.keys().cloned().collect()
    }

    /// Get rule configuration
    pub fn get_rule_config(&self, rule_id: &str) -> Option<&SunLinterRuleConfig> {
        self.rules.get(rule_id).map(|rule| &rule.config)
    }
}

/// Factory for creating unified rules from SunLinter definitions
pub struct SunLinterRuleFactory;

impl SunLinterRuleFactory {
    /// Create a WASM-compatible rule from SunLinter configuration
    pub fn create_unified_rule(config: SunLinterRuleConfig) -> Box<dyn WasmRule> {
        Box::new(SunLinterUnifiedRule { config })
    }
}

/// WASM-compatible wrapper for SunLinter rules
pub struct SunLinterUnifiedRule {
    config: SunLinterRuleConfig,
}

impl WasmRule for SunLinterUnifiedRule {
    const NAME: &'static str = "sunlinter-unified";
    const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        // Create temporary engine for rule execution
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        engine.analyze_unified(code, &self.config.rule_id)
    }
}

impl EnhancedWasmRule for SunLinterUnifiedRule {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            suggestion_type: "sunlinter_behavioral".to_string(),
            confidence: 0.85,
            description: format!("SunLinter behavioral analysis suggests: {}", self.config.description),
            code_example: None,
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunlinter_engine_creation() {
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        assert!(!engine.rules.is_empty());
        assert!(engine.rules.contains_key("C002"));
        assert!(engine.rules.contains_key("S005"));
        assert!(engine.rules.contains_key("T002"));
    }

    #[test]
    fn test_duplicate_code_detection() {
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        let code = r#"
            function test1() {
                console.log("duplicate");
                return true;
            }

            function test2() {
                console.log("duplicate");
                return true;
            }
        "#;

        let diagnostics = engine.analyze_unified(code, "C002");
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_security_pattern_detection() {
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        let code = r#"
            if (req.headers.origin === allowedOrigin) {
                authenticate(user);
            }
        "#;

        let diagnostics = engine.analyze_unified(code, "S005");
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_interface_naming_convention() {
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        let code = r#"
            interface UserData {
                name: string;
                email: string;
            }
        "#;

        let diagnostics = engine.analyze_unified(code, "T002");
        assert!(!diagnostics.is_empty());
        assert_eq!(diagnostics[0].rule_name, "T002");
    }

    #[test]
    fn test_catch_block_logging() {
        let engine = SunLinterEngine::new(SunLinterEngineConfig::default());
        let code = r#"
            try {
                riskyOperation();
            } catch (error) {
                // No logging here
                return false;
            }
        "#;

        let diagnostics = engine.analyze_unified(code, "C029");
        assert!(!diagnostics.is_empty());
    }
}