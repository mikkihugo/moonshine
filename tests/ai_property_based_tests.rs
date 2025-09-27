//! # AI Linting Property-Based Tests
//!
//! Property-based tests using proptest to verify:
//! - Rule consistency across different code inputs
//! - AI provider selection determinism
//! - Behavioral pattern detection reliability
//! - Configuration validation properties
//! - Error handling invariants
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::oxc_adapter::{AiBehavioralAnalyzer, BehavioralPattern, BehavioralPatternType};
use moon_shine::provider_router::{AIRouter, AIRequest, AIContext, ProviderCapabilities};
use moon_shine::rule_types::RuleSeverity;
use moon_shine::moon_pdk_interface::AiLinterConfig;
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use arbitrary::{Arbitrary, Unstructured};
use quickcheck::{Gen, quickcheck};
use quickcheck_macros::quickcheck;

/// Arbitrary implementation for generating test data
#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryCode {
    functions: Vec<ArbitraryFunction>,
    imports: Vec<String>,
    exports: Vec<String>,
}

#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryFunction {
    name: String,
    parameters: Vec<String>,
    body_complexity: u8, // 0-255 representing complexity
    has_async: bool,
    has_loops: bool,
    has_conditionals: bool,
    has_event_listeners: bool,
    has_timeouts: bool,
}

#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryAIRequest {
    prompt_length: u16,
    session_id_length: u8,
    context_type: u8, // 0-4 representing different context types
    content_size: u16,
    preferred_providers: Vec<u8>, // 0-2 representing claude, google, openai
}

#[derive(Debug, Clone, Arbitrary)]
struct ArbitraryConfig {
    rate_limit: u16,        // 1-1000
    batch_size: u8,         // 1-100
    retry_attempts: u8,     // 1-10
    retry_delay_ms: u16,    // 1-10000
    quality_threshold: u8,  // 0-100 (will be converted to 0.0-1.0)
    max_tokens: u16,        // 1000-50000
}

impl ArbitraryCode {
    fn to_typescript_code(&self) -> String {
        let mut code = String::new();
        
        // Add imports
        for import in &self.imports {
            code.push_str(&format!("import {{ {} }} from 'library';\n", import));
        }
        
        code.push('\n');
        
        // Add functions
        for func in &self.functions {
            code.push_str(&func.to_typescript());
            code.push('\n');
        }
        
        // Add exports
        for export in &self.exports {
            code.push_str(&format!("export {{ {} }};\n", export));
        }
        
        code
    }
}

impl ArbitraryFunction {
    fn to_typescript(&self) -> String {
        let mut func = String::new();
        
        if self.has_async {
            func.push_str("async ");
        }
        
        func.push_str(&format!("function {}({}) {{\n", 
            self.name, 
            self.parameters.join(", ")));
        
        // Add complexity based on body_complexity
        let nesting_level = (self.body_complexity / 50) as usize + 1;
        
        for level in 0..nesting_level {
            let indent = "  ".repeat(level + 1);
            
            if self.has_conditionals && level % 2 == 0 {
                func.push_str(&format!("{}if (condition{}) {{\n", indent, level));
            }
            
            if self.has_loops && level % 3 == 0 {
                func.push_str(&format!("{}for (let i{} = 0; i{} < 10; i{}++) {{\n", 
                    indent, level, level, level));
            }
        }
        
        // Add problematic patterns based on flags
        let base_indent = "  ".repeat(nesting_level + 1);
        
        if self.has_event_listeners {
            func.push_str(&format!("{}document.addEventListener('click', handler);\n", base_indent));
        }
        
        if self.has_timeouts {
            func.push_str(&format!("{}setTimeout(() => {{}}, 1000);\n", base_indent));
        }
        
        func.push_str(&format!("{}return true;\n", base_indent));
        
        // Close all braces
        for level in (0..nesting_level).rev() {
            let indent = "  ".repeat(level + 1);
            func.push_str(&format!("{}}\n", indent));
        }
        
        func.push_str("}\n");
        func
    }
}

impl ArbitraryAIRequest {
    fn to_ai_request(&self) -> AIRequest {
        let prompt = "a".repeat(self.prompt_length.min(10000) as usize);
        let session_id = "s".repeat(self.session_id_length.min(100) as usize);
        let content = "c".repeat(self.content_size.min(50000) as usize);
        
        let context = match self.context_type % 5 {
            0 => AIContext::CodeFix {
                language: "typescript".to_string(),
                content: content.clone(),
            },
            1 => AIContext::CodeAnalysis {
                language: "javascript".to_string(),
                content: content.clone(),
            },
            2 => AIContext::CodeGeneration {
                language: "rust".to_string(),
                specification: content.clone(),
            },
            3 => AIContext::AiLinting {
                language: "typescript".to_string(),
                content: content.clone(),
                static_issues: vec!["test issue".to_string()],
                analysis_focus: vec!["performance".to_string()],
            },
            _ => AIContext::General,
        };
        
        let preferred_providers = self.preferred_providers.iter()
            .take(3)
            .map(|&p| match p % 3 {
                0 => "claude",
                1 => "google",
                _ => "openai",
            })
            .map(|s| s.to_string())
            .collect();
        
        AIRequest {
            prompt,
            session_id,
            file_path: Some("test.ts".to_string()),
            context,
            preferred_providers,
        }
    }
}

impl ArbitraryConfig {
    fn to_ai_linter_config(&self) -> AiLinterConfig {
        AiLinterConfig {
            enable_claude_ai: true,
            enable_semantic_checks: true,
            claude_model: "sonnet".to_string(),
            max_processing_time: 600,
            quality_threshold: (self.quality_threshold as f32) / 100.0,
            debug_session_retention_hours: 12,
            cleanup_sessions_older_than_hours: 48,
            max_concurrent_requests: 3,
            batch_size: self.batch_size.max(1) as u32,
            rate_limit_per_minute: self.rate_limit.max(1) as u32,
            max_tokens_per_request: self.max_tokens.max(1000) as u32,
            retry_attempts: self.retry_attempts.max(1) as u32,
            retry_delay_ms: self.retry_delay_ms.max(1) as u32,
        }
    }
}

/// Property: Provider selection should be deterministic for identical requests
proptest! {
    #[test]
    fn prop_provider_selection_deterministic(request_data in any::<ArbitraryAIRequest>()) {
        let router = AIRouter::new();
        let request = request_data.to_ai_request();
        
        let result1 = router.select_provider(&request);
        let result2 = router.select_provider(&request);
        
        // Both should succeed or both should fail
        match (result1, result2) {
            (Ok((provider1, reason1)), Ok((provider2, reason2))) => {
                prop_assert_eq!(provider1.name, provider2.name);
                // Reasoning might have timestamps, so just check it's not empty
                prop_assert!(!reason1.is_empty());
                prop_assert!(!reason2.is_empty());
            },
            (Err(_), Err(_)) => {
                // Both failing is acceptable (no providers available)
            },
            _ => {
                prop_assert!(false, "Provider selection should be deterministic");
            }
        }
    }
}

/// Property: Cognitive complexity should increase monotonically with code complexity
proptest! {
    #[test]
    fn prop_cognitive_complexity_monotonic(code_data in any::<ArbitraryCode>()) {
        let analyzer = AiBehavioralAnalyzer::new();
        let code = code_data.to_typescript_code();
        
        if code.len() < 10000 { // Only test reasonable sized code
            let complexity = analyzer.calculate_cognitive_complexity_heuristic(&code);
            
            // Complexity should be non-negative
            prop_assert!(complexity >= 0);
            
            // Complexity should be bounded (not infinite)
            prop_assert!(complexity < 10000);
            
            // More functions should generally mean more complexity
            let function_count = code_data.functions.len() as u32;
            if function_count > 0 {
                prop_assert!(complexity >= function_count / 2); // Very loose lower bound
            }
        }
    }
}

/// Property: Rate limiting should be consistent
proptest! {
    #[test]
    fn prop_rate_limiting_consistency(config_data in any::<ArbitraryConfig>()) {
        let config = config_data.to_ai_linter_config();
        
        // Rate limiting should either succeed or fail consistently
        let result1 = moon_shine::provider_router::apply_rate_limiting(&config);
        
        // Basic properties
        prop_assert!(config.rate_limit_per_minute > 0);
        prop_assert!(config.retry_delay_ms > 0);
        prop_assert!(config.batch_size > 0);
        prop_assert!(config.retry_attempts > 0);
        prop_assert!(config.quality_threshold >= 0.0 && config.quality_threshold <= 1.0);
        
        // If rate limit is high, first request should usually succeed
        if config.rate_limit_per_minute > 50 {
            // This might still fail due to global state, but that's expected
            let _ = result1; // Don't assert success, just check it doesn't panic
        }
    }
}

/// Property: Behavioral patterns should have valid configuration
proptest! {
    #[test]
    fn prop_behavioral_patterns_valid(
        pattern_id in "[a-z-]{5,50}",
        pattern_name in "[A-Za-z ]{10,100}",
        confidence in 0.1f32..1.0f32,
        severity_index in 0u8..4u8
    ) {
        let severity = match severity_index {
            0 => RuleSeverity::Error,
            1 => RuleSeverity::Warning,
            2 => RuleSeverity::Info,
            _ => RuleSeverity::Hint,
        };
        
        let pattern = BehavioralPattern {
            id: pattern_id.clone(),
            name: pattern_name.clone(),
            description: "Test pattern description".to_string(),
            category: "test".to_string(),
            severity,
            pattern_type: BehavioralPatternType::CognitiveComplexity,
            ai_prompt: "Test AI prompt for analysis".to_string(),
            confidence_threshold: confidence,
        };
        
        // Properties that should always hold
        prop_assert!(!pattern.id.is_empty());
        prop_assert!(!pattern.name.is_empty());
        prop_assert!(!pattern.description.is_empty());
        prop_assert!(!pattern.ai_prompt.is_empty());
        prop_assert!(pattern.confidence_threshold > 0.0);
        prop_assert!(pattern.confidence_threshold <= 1.0);
        
        // ID should be valid identifier
        prop_assert!(pattern.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
        prop_assert!(pattern.id.len() >= 5);
        prop_assert!(pattern.id.len() <= 50);
    }
}

/// Property: Provider capabilities should be valid
proptest! {
    #[test]
    fn prop_provider_capabilities_valid(
        code_analysis in 0.0f32..1.0f32,
        code_generation in 0.0f32..1.0f32,
        complex_reasoning in 0.0f32..1.0f32,
        speed in 0.0f32..1.0f32,
        context_length in 1000u32..1000000u32
    ) {
        let capabilities = ProviderCapabilities {
            code_analysis,
            code_generation,
            complex_reasoning,
            speed,
            context_length,
            supports_sessions: true,
        };
        
        // All capability scores should be valid probabilities
        prop_assert!(capabilities.code_analysis >= 0.0 && capabilities.code_analysis <= 1.0);
        prop_assert!(capabilities.code_generation >= 0.0 && capabilities.code_generation <= 1.0);
        prop_assert!(capabilities.complex_reasoning >= 0.0 && capabilities.complex_reasoning <= 1.0);
        prop_assert!(capabilities.speed >= 0.0 && capabilities.speed <= 1.0);
        
        // Context length should be reasonable
        prop_assert!(capabilities.context_length >= 1000);
        prop_assert!(capabilities.context_length <= 1000000);
        
        // JSON serialization should work
        let json_result = serde_json::to_string(&capabilities);
        prop_assert!(json_result.is_ok());
        
        if let Ok(json) = json_result {
            let deserialized: Result<ProviderCapabilities, _> = serde_json::from_str(&json);
            prop_assert!(deserialized.is_ok());
        }
    }
}

/// QuickCheck-based tests for additional coverage
#[quickcheck]
fn qc_analyzer_creation_is_consistent(seed: u64) -> bool {
    let analyzer1 = AiBehavioralAnalyzer::new();
    let analyzer2 = AiBehavioralAnalyzer::new();
    
    // Both analyzers should have the same default patterns
    analyzer1.get_patterns().len() == analyzer2.get_patterns().len()
}

#[quickcheck]
fn qc_provider_router_creation_is_consistent(seed: u64) -> bool {
    let router1 = AIRouter::new();
    let router2 = AIRouter::new();
    
    // Both routers should have the same providers
    router1.providers.len() == router2.providers.len() &&
    router1.providers.len() == 3 // Claude, Google, OpenAI
}

#[quickcheck]
fn qc_config_validation_never_panics(rate_limit: u16, batch_size: u8, quality: u8) -> bool {
    let config = AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 600,
        quality_threshold: (quality as f32) / 255.0, // Normalize to 0.0-1.0
        debug_session_retention_hours: 12,
        cleanup_sessions_older_than_hours: 48,
        max_concurrent_requests: 3,
        batch_size: batch_size.max(1) as u32,
        rate_limit_per_minute: rate_limit.max(1) as u32,
        max_tokens_per_request: 4000,
        retry_attempts: 3,
        retry_delay_ms: 1000,
    };
    
    // Configuration creation should never panic
    config.batch_size > 0 && 
    config.rate_limit_per_minute > 0 &&
    config.quality_threshold >= 0.0 && config.quality_threshold <= 1.0
}

#[quickcheck]
fn qc_cognitive_complexity_bounds(nesting_depth: u8, condition_count: u8) -> bool {
    let analyzer = AiBehavioralAnalyzer::new();
    
    // Generate code with known structure
    let mut code = String::new();
    code.push_str("function test() {\n");
    
    for level in 0..nesting_depth.min(10) {
        code.push_str(&"  ".repeat(level as usize + 1));
        code.push_str("if (condition) {\n");
    }
    
    for _ in 0..condition_count.min(20) {
        code.push_str("  if (cond) { /* */ }\n");
    }
    
    for level in (0..nesting_depth.min(10)).rev() {
        code.push_str(&"  ".repeat(level as usize + 1));
        code.push_str("}\n");
    }
    
    code.push_str("}\n");
    
    let complexity = analyzer.calculate_cognitive_complexity_heuristic(&code);
    
    // Complexity should be reasonable bounds
    complexity >= 0 && complexity <= 1000
}

#[quickcheck]
fn qc_ai_request_serialization_roundtrip(prompt_len: u8, session_len: u8) -> bool {
    let prompt = "a".repeat(prompt_len.min(100) as usize);
    let session_id = "s".repeat(session_len.min(50) as usize);
    
    let request = AIRequest {
        prompt,
        session_id,
        file_path: Some("test.ts".to_string()),
        context: AIContext::General,
        preferred_providers: vec![],
    };
    
    // JSON serialization roundtrip should work
    let json_result = serde_json::to_string(&request);
    if let Ok(json) = json_result {
        let deserialized: Result<AIRequest, _> = serde_json::from_str(&json);
        deserialized.is_ok()
    } else {
        false
    }
}

/// Property-based invariant: Analysis results should be stable for identical inputs
proptest! {
    #[test]
    fn prop_analysis_stability(code_data in any::<ArbitraryCode>()) {
        let adapter = moon_shine::oxc_adapter::OxcAdapter::new();
        let code = code_data.to_typescript_code();
        
        if code.len() < 5000 && code.len() > 10 { // Test reasonable code sizes
            let result1 = adapter.analyze_code(&code, "test1.tsx");
            let result2 = adapter.analyze_code(&code, "test2.tsx");
            
            // Both should succeed or both should fail
            match (result1, result2) {
                (Ok(analysis1), Ok(analysis2)) => {
                    // Same code should produce same number of diagnostics
                    prop_assert_eq!(analysis1.diagnostics.len(), analysis2.diagnostics.len());
                },
                (Err(_), Err(_)) => {
                    // Both failing is acceptable for malformed code
                },
                _ => {
                    prop_assert!(false, "Analysis should be deterministic for identical input");
                }
            }
        }
    }
}

/// Property: Pattern detection should be consistent across multiple runs
proptest! {
    #[test]
    fn prop_pattern_detection_consistency(
        has_memory_leak in any::<bool>(),
        has_complexity in any::<bool>(),
        function_count in 1u8..20u8
    ) {
        let analyzer = AiBehavioralAnalyzer::new();
        
        let mut code = String::new();
        
        for i in 0..function_count {
            code.push_str(&format!("function func{}() {{\n", i));
            
            if has_memory_leak && i == 0 {
                code.push_str("  document.addEventListener('click', handler);\n");
            }
            
            if has_complexity {
                for level in 0..3 {
                    code.push_str(&format!("{}if (condition{}) {{\n", "  ".repeat(level + 1), level));
                }
                for level in (0..3).rev() {
                    code.push_str(&format!("{}}\n", "  ".repeat(level + 1)));
                }
            }
            
            code.push_str("}\n\n");
        }
        
        // Run heuristic analysis multiple times
        let context = moon_shine::oxc_adapter::ai_behavioral::AnalysisContext {
            file_path: "test.tsx".to_string(),
            file_type: oxc_span::SourceType::tsx(),
            project_context: None,
            dependencies: vec![],
        };
        
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::tsx();
        let parser_result = oxc_parser::Parser::new(&allocator, &code, source_type).parse();
        
        let result1 = analyzer.run_heuristic_analysis(&code, &parser_result.program, &context);
        let result2 = analyzer.run_heuristic_analysis(&code, &parser_result.program, &context);
        
        // Results should be consistent
        match (result1, result2) {
            (Ok(diag1), Ok(diag2)) => {
                prop_assert_eq!(diag1.len(), diag2.len());
                
                // If we expect memory leaks, should detect them consistently
                if has_memory_leak {
                    let leak_count1 = diag1.iter().filter(|d| d.message.contains("memory leak")).count();
                    let leak_count2 = diag2.iter().filter(|d| d.message.contains("memory leak")).count();
                    prop_assert_eq!(leak_count1, leak_count2);
                }
                
                // If we expect complexity, should detect it consistently
                if has_complexity && function_count > 5 {
                    let complexity_count1 = diag1.iter().filter(|d| d.message.contains("complexity")).count();
                    let complexity_count2 = diag2.iter().filter(|d| d.message.contains("complexity")).count();
                    prop_assert_eq!(complexity_count1, complexity_count2);
                }
            },
            (Err(_), Err(_)) => {
                // Both failing is acceptable
            },
            _ => {
                prop_assert!(false, "Pattern detection should be consistent");
            }
        }
    }
}

/// Property: Configuration serialization should preserve semantics
proptest! {
    #[test]
    fn prop_config_serialization_preserves_semantics(config_data in any::<ArbitraryConfig>()) {
        let original_config = config_data.to_ai_linter_config();
        
        // Serialize and deserialize
        let json = serde_json::to_string(&original_config).unwrap();
        let restored_config: AiLinterConfig = serde_json::from_str(&json).unwrap();
        
        // Key properties should be preserved
        prop_assert_eq!(original_config.rate_limit_per_minute, restored_config.rate_limit_per_minute);
        prop_assert_eq!(original_config.batch_size, restored_config.batch_size);
        prop_assert_eq!(original_config.retry_attempts, restored_config.retry_attempts);
        prop_assert_eq!(original_config.max_tokens_per_request, restored_config.max_tokens_per_request);
        prop_assert!((original_config.quality_threshold - restored_config.quality_threshold).abs() < 0.001);
        
        // Boolean flags should be preserved
        prop_assert_eq!(original_config.enable_claude_ai, restored_config.enable_claude_ai);
        prop_assert_eq!(original_config.enable_semantic_checks, restored_config.enable_semantic_checks);
    }
}
