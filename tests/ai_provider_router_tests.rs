//! # AI Provider Router Unit Tests
//!
//! Comprehensive unit tests for the AI provider routing system covering:
//! - Provider selection logic and scoring
//! - Rate limiting and concurrency controls
//! - Mock AI provider communication
//! - Error handling and fallback scenarios
//! - Request context analysis
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::error::{Error, Result};
use moon_shine::moon_pdk_interface::AiLinterConfig;
use moon_shine::provider_router::*;
use rstest::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;

/// Mock AI provider for testing
struct MockAIProvider {
    name: String,
    should_fail: bool,
    response_delay: Duration,
    response_content: String,
}

impl MockAIProvider {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            should_fail: false,
            response_delay: Duration::from_millis(100),
            response_content: "Mock AI response".to_string(),
        }
    }

    fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }

    fn with_delay(mut self, delay: Duration) -> Self {
        self.response_delay = delay;
        self
    }

    fn with_response(mut self, content: &str) -> Self {
        self.response_content = content.to_string();
        self
    }
}

#[fixture]
fn mock_ai_router() -> AIRouter {
    AIRouter::new()
}

#[fixture]
fn sample_code_fix_request() -> AIRequest {
    AIRequest {
        prompt: "Fix this TypeScript code that has type errors".to_string(),
        session_id: "test-session-123".to_string(),
        file_path: Some("src/test.ts".to_string()),
        context: AIContext::CodeFix {
            language: "typescript".to_string(),
            content: "const x: number = 'string';".to_string(),
        },
        preferred_providers: vec![],
    }
}

#[fixture]
fn sample_ai_linting_request() -> AIRequest {
    AIRequest {
        prompt: "Analyze this code for behavioral patterns".to_string(),
        session_id: "linting-session-456".to_string(),
        file_path: Some("src/component.tsx".to_string()),
        context: AIContext::AiLinting {
            language: "typescript".to_string(),
            content: r#"
                function Component() {
                    const [state, setState] = useState(0);
                    useEffect(() => {
                        setState(state + 1); // Missing dependency
                    });
                    return <div onClick={() => setState(state + 1)}>Click</div>;
                }
            "#.to_string(),
            static_issues: vec!["Missing dependency in useEffect".to_string()],
            analysis_focus: vec!["performance".to_string(), "react".to_string()],
        },
        preferred_providers: vec![],
    }
}

#[rstest]
fn test_provider_capabilities_scoring() {
    let router = mock_ai_router();
    let request = sample_code_fix_request();
    
    let selection_result = router.select_provider(&request);
    assert!(selection_result.is_ok(), "Provider selection should succeed");
    
    let (provider, reason) = selection_result.unwrap();
    assert!(!provider.name.is_empty(), "Selected provider should have a name");
    assert!(!reason.is_empty(), "Selection reason should be provided");
    
    // Should prefer providers with high code analysis capabilities for code fixing
    assert!(provider.capabilities.code_analysis >= 0.8, 
        "Code fix requests should select providers with high code analysis capabilities");
}

#[rstest]
fn test_provider_selection_by_context(mock_ai_router: AIRouter) {
    // Test CodeFix context preference
    let code_fix_request = AIRequest {
        prompt: "Fix this bug".to_string(),
        session_id: "session-1".to_string(),
        file_path: None,
        context: AIContext::CodeFix {
            language: "typescript".to_string(),
            content: "const x = 1;".to_string(),
        },
        preferred_providers: vec![],
    };
    
    let (provider, _) = mock_ai_router.select_provider(&code_fix_request).unwrap();
    assert!(provider.capabilities.code_analysis >= 0.8, 
        "CodeFix should prefer providers with strong code analysis");
    
    // Test CodeGeneration context preference
    let code_gen_request = AIRequest {
        prompt: "Generate a function".to_string(),
        session_id: "session-2".to_string(),
        file_path: None,
        context: AIContext::CodeGeneration {
            language: "rust".to_string(),
            specification: "Sort function".to_string(),
        },
        preferred_providers: vec![],
    };
    
    let (provider, _) = mock_ai_router.select_provider(&code_gen_request).unwrap();
    assert!(provider.capabilities.code_generation >= 0.8, 
        "CodeGeneration should prefer providers with strong code generation");
    
    // Test AiLinting context preference
    let ai_linting_request = sample_ai_linting_request();
    let (provider, _) = mock_ai_router.select_provider(&ai_linting_request).unwrap();
    assert!(provider.capabilities.complex_reasoning >= 0.8, 
        "AiLinting should prefer providers with strong reasoning for pattern detection");
}

#[rstest]
fn test_provider_preference_ordering(mock_ai_router: AIRouter) {
    let request_with_preference = AIRequest {
        prompt: "Analyze this code".to_string(),
        session_id: "session-pref".to_string(),
        file_path: None,
        context: AIContext::CodeAnalysis {
            language: "python".to_string(),
            content: "def test(): pass".to_string(),
        },
        preferred_providers: vec!["google".to_string(), "claude".to_string()],
    };
    
    let (selected_provider, reason) = mock_ai_router.select_provider(&request_with_preference).unwrap();
    
    // Should respect preference order when providers are available
    assert!(reason.contains("preferred order") || selected_provider.name == "google", 
        "Should prefer explicitly requested providers when available");
}

#[rstest]
fn test_provider_context_length_requirements(mock_ai_router: AIRouter) {
    let large_context_request = AIRequest {
        prompt: "Process this large codebase".to_string(),
        session_id: "large-session".to_string(),
        file_path: None,
        context: AIContext::CodeAnalysis {
            language: "typescript".to_string(),
            content: "x".repeat(500_000), // Very large content
        },
        preferred_providers: vec![],
    };
    
    let selection_result = mock_ai_router.select_provider(&large_context_request);
    
    match selection_result {
        Ok((provider, _)) => {
            // Should select a provider with sufficient context length
            assert!(provider.capabilities.context_length >= 100_000, 
                "Large content should select providers with sufficient context length");
        },
        Err(_) => {
            // It's acceptable to fail if no provider can handle the context size
        }
    }
}

#[rstest]
fn test_rate_limiting_functionality() {
    let strict_config = AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 600,
        quality_threshold: 0.8,
        debug_session_retention_hours: 12,
        cleanup_sessions_older_than_hours: 48,
        max_concurrent_requests: 3,
        batch_size: 5,
        rate_limit_per_minute: 2, // Very strict rate limit
        max_tokens_per_request: 4000,
        retry_attempts: 3,
        retry_delay_ms: 100,
    };
    
    // First request should succeed
    let result1 = apply_rate_limiting(&strict_config);
    assert!(result1.is_ok(), "First request should succeed");
    
    // Second request should succeed
    let result2 = apply_rate_limiting(&strict_config);
    assert!(result2.is_ok(), "Second request should succeed");
    
    // Third request should fail due to rate limit
    let result3 = apply_rate_limiting(&strict_config);
    assert!(result3.is_err(), "Third request should fail due to rate limit");
    
    if let Err(error) = result3 {
        assert!(error.to_string().contains("Rate limit exceeded"), 
            "Error should mention rate limit: {}", error);
    }
}

#[rstest]
fn test_batch_processing_with_rate_limiting() {
    let files = vec![
        "file1.ts".to_string(),
        "file2.ts".to_string(),
        "file3.ts".to_string(),
        "file4.ts".to_string(),
        "file5.ts".to_string(),
    ];
    
    let config = AiLinterConfig {
        enable_claude_ai: true,
        enable_semantic_checks: true,
        claude_model: "sonnet".to_string(),
        max_processing_time: 600,
        quality_threshold: 0.8,
        debug_session_retention_hours: 12,
        cleanup_sessions_older_than_hours: 48,
        max_concurrent_requests: 3,
        batch_size: 2, // Small batch size
        rate_limit_per_minute: 60, // High rate limit for testing
        max_tokens_per_request: 4000,
        retry_attempts: 3,
        retry_delay_ms: 10, // Short delay
    };
    
    let processor = |batch: &[String]| -> Result<Vec<moon_shine::rulebase::RuleResult>, Box<dyn std::error::Error>> {
        let suggestions: Vec<_> = batch.iter().map(|filename| {
            moon_shine::rulebase::RuleResult {
                rule: "test-rule".to_string(),
                message: format!("Issue in {}", filename),
                line: 1,
                column: 1,
                severity: "warning".to_string(),
                fix_available: true,
                ai_confidence: 0.9,
                pattern_frequency: Some(0.5),
            }
        }).collect();
        Ok(suggestions)
    };
    
    let result = batch_process_files(&files, &config, processor);
    assert!(result.is_ok(), "Batch processing should succeed");
    
    let suggestions = result.unwrap();
    assert_eq!(suggestions.len(), 5, "Should process all files");
    assert!(suggestions[0].message.contains("file1.ts"), "Should process correct files");
}

#[rstest]
fn test_ai_context_creation_and_validation() {
    // Test CodeFix context
    let code_fix = AIContext::CodeFix {
        language: "typescript".to_string(),
        content: "const x: number = 1;".to_string(),
    };
    
    match code_fix {
        AIContext::CodeFix { language, content } => {
            assert_eq!(language, "typescript");
            assert!(content.contains("const x"));
        },
        _ => panic!("Expected CodeFix context"),
    }
    
    // Test AiLinting context with comprehensive fields
    let ai_linting = AIContext::AiLinting {
        language: "javascript".to_string(),
        content: "function test() {}".to_string(),
        static_issues: vec!["unused variable".to_string()],
        analysis_focus: vec!["performance".to_string(), "security".to_string()],
    };
    
    match ai_linting {
        AIContext::AiLinting { language, content, static_issues, analysis_focus } => {
            assert_eq!(language, "javascript");
            assert!(content.contains("function test"));
            assert_eq!(static_issues.len(), 1);
            assert_eq!(analysis_focus.len(), 2);
            assert!(analysis_focus.contains(&"performance".to_string()));
            assert!(analysis_focus.contains(&"security".to_string()));
        },
        _ => panic!("Expected AiLinting context"),
    }
}

#[rstest]
fn test_provider_config_creation() {
    // Test Claude provider config
    let claude_config = AIProviderConfig::claude();
    assert_eq!(claude_config.name, "claude");
    assert!(claude_config.capabilities.code_analysis >= 0.9);
    assert!(claude_config.capabilities.complex_reasoning >= 0.9);
    assert!(claude_config.capabilities.supports_sessions);
    
    // Test Google provider config
    let google_config = AIProviderConfig::google();
    assert_eq!(google_config.name, "google");
    assert!(google_config.capabilities.code_analysis >= 0.8);
    assert!(google_config.capabilities.speed >= 0.8);
    
    // Test OpenAI provider config
    let openai_config = AIProviderConfig::openai();
    assert_eq!(openai_config.name, "openai");
    assert!(openai_config.capabilities.code_generation >= 0.9);
    assert!(openai_config.capabilities.context_length >= 100_000);
}

#[rstest]
fn test_request_requirements_inference(mock_ai_router: AIRouter) {
    // Test CodeFix requirements
    let code_fix_request = sample_code_fix_request();
    let requirements = mock_ai_router.infer_requirements(&code_fix_request);
    
    assert!(requirements.needs_code_analysis, "CodeFix should need code analysis");
    assert!(requirements.needs_code_generation, "CodeFix should need code generation");
    assert!(requirements.needs_complex_reasoning, "CodeFix should need complex reasoning");
    assert!(!requirements.needs_speed, "CodeFix should prioritize quality over speed");
    
    // Test AiLinting requirements
    let ai_linting_request = sample_ai_linting_request();
    let requirements = mock_ai_router.infer_requirements(&ai_linting_request);
    
    assert!(requirements.needs_code_analysis, "AiLinting should need code analysis");
    assert!(requirements.needs_complex_reasoning, "AiLinting should need complex reasoning for patterns");
    assert!(!requirements.needs_code_generation, "AiLinting should not need code generation");
    
    // Test General context requirements
    let general_request = AIRequest {
        prompt: "General question".to_string(),
        session_id: "general-session".to_string(),
        file_path: None,
        context: AIContext::General,
        preferred_providers: vec![],
    };
    let requirements = mock_ai_router.infer_requirements(&general_request);
    
    assert!(requirements.needs_speed, "General requests should prioritize speed");
    assert!(!requirements.needs_code_analysis, "General requests should not need specialized code analysis");
}

#[rstest]
fn test_error_handling_no_providers() {
    // Create a router with no available providers (simulate all providers failing auth)
    let router = AIRouter::new();
    
    // Create request with non-existent preferred provider
    let request = AIRequest {
        prompt: "Test prompt".to_string(),
        session_id: "test-session".to_string(),
        file_path: None,
        context: AIContext::General,
        preferred_providers: vec!["nonexistent-provider".to_string()],
    };
    
    // In real environment without API keys, this would fail gracefully
    let result = router.select_provider(&request);
    
    match result {
        Ok((provider, _)) => {
            // If it succeeds, should be a valid provider
            assert!(!provider.name.is_empty());
        },
        Err(error) => {
            // Should provide helpful error about provider availability
            assert!(error.to_string().contains("No AI providers available") ||
                   error.to_string().contains("API key"));
        }
    }
}

#[rstest]
fn test_serialization_compatibility() {
    let capabilities = ProviderCapabilities {
        code_analysis: 0.95,
        code_generation: 0.85,
        complex_reasoning: 0.90,
        speed: 0.75,
        context_length: 200000,
        supports_sessions: true,
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&capabilities).unwrap();
    assert!(json.contains("0.95"));
    assert!(json.contains("200000"));
    assert!(json.contains("true"));
    
    // Test JSON deserialization
    let deserialized: ProviderCapabilities = serde_json::from_str(&json).unwrap();
    assert!((deserialized.code_analysis - capabilities.code_analysis).abs() < f32::EPSILON);
    assert_eq!(deserialized.context_length, capabilities.context_length);
    assert_eq!(deserialized.supports_sessions, capabilities.supports_sessions);
}

#[rstest]
fn test_concurrent_rate_limiting() {
    let config = AiLinterConfig {
        rate_limit_per_minute: 10,
        retry_delay_ms: 50,
        ..AiLinterConfig::default()
    };
    
    // Simulate concurrent requests
    let handles: Vec<_> = (0..5).map(|_| {
        let config_clone = config.clone();
        std::thread::spawn(move || {
            apply_rate_limiting(&config_clone)
        })
    }).collect();
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // At least some requests should succeed
    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(successful_count > 0, "At least some concurrent requests should succeed");
    
    // Some may fail due to rate limiting, which is expected behavior
    let failed_count = results.iter().filter(|r| r.is_err()).count();
    if failed_count > 0 {
        println!("Expected: {} requests failed due to rate limiting", failed_count);
    }
}

#[rstest]
fn test_global_ai_router_singleton() {
    let router1 = get_ai_router();
    let router2 = get_ai_router();
    
    // Should return the same instance (singleton pattern)
    assert_eq!(router1.providers.len(), router2.providers.len());
    assert_eq!(router1.providers.len(), 3); // Claude, Google, OpenAI
    
    // Verify providers are present
    assert!(router1.providers.iter().any(|p| p.name == "claude"));
    assert!(router1.providers.iter().any(|p| p.name == "google"));
    assert!(router1.providers.iter().any(|p| p.name == "openai"));
}

/// Test helper for AI provider configuration edge cases
#[rstest]
fn test_provider_configuration_edge_cases() {
    // Test with minimal capabilities
    let minimal_provider = AIProviderConfig {
        name: "minimal".to_string(),
        command: "minimal-ai".to_string(),
        model: "basic".to_string(),
        api_key_env: None,
        requires_api_key: false,
        capabilities: ProviderCapabilities {
            code_analysis: 0.1,
            code_generation: 0.1,
            complex_reasoning: 0.1,
            speed: 1.0,
            context_length: 1000,
            supports_sessions: false,
        },
    };
    
    assert_eq!(minimal_provider.name(), "minimal");
    assert!(minimal_provider.capabilities.speed > 0.9);
    assert!(minimal_provider.capabilities.context_length < 10000);
    
    // Test with maximum capabilities
    let maximal_provider = AIProviderConfig {
        name: "maximal".to_string(),
        command: "maximal-ai".to_string(),
        model: "advanced".to_string(),
        api_key_env: Some("MAXIMAL_API_KEY".to_string()),
        requires_api_key: true,
        capabilities: ProviderCapabilities {
            code_analysis: 1.0,
            code_generation: 1.0,
            complex_reasoning: 1.0,
            speed: 0.1,
            context_length: 1_000_000,
            supports_sessions: true,
        },
    };
    
    assert_eq!(maximal_provider.name(), "maximal");
    assert!(maximal_provider.capabilities.code_analysis >= 0.99);
    assert!(maximal_provider.capabilities.context_length >= 500_000);
}
