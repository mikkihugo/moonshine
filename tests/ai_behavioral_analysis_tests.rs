//! # AI Behavioral Analysis Tests
//!
//! Comprehensive tests for AI behavioral pattern detection including:
//! - Behavioral pattern configuration and validation
//! - Heuristic analysis algorithms
//! - AI client integration and mocking
//! - Pattern detection accuracy and confidence scoring
//! - Performance analysis of behavioral checks
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use moon_shine::oxc_adapter::ai_behavioral::*;
use moon_shine::rule_types::RuleSeverity;
use moon_shine::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use rstest::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/// Mock AI analysis client for testing
#[derive(Clone)]
struct MockAiAnalysisClient {
    responses: Arc<Mutex<Vec<AiPatternResult>>>,
    should_fail: bool,
    response_delay: Duration,
}

impl MockAiAnalysisClient {
    fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(Vec::new())),
            should_fail: false,
            response_delay: Duration::from_millis(50),
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

    fn add_response(&self, response: AiPatternResult) {
        self.responses.lock().unwrap().push(response);
    }
}

impl AiAnalysisClient for MockAiAnalysisClient {
    fn analyze_patterns(
        &self,
        _code: &str,
        patterns: &[BehavioralPattern],
        _context: &AnalysisContext,
    ) -> Result<Vec<AiPatternResult>, Box<dyn std::error::Error>> {
        if self.should_fail {
            return Err("Mock AI client failure".into());
        }

        // Simulate AI processing delay
        std::thread::sleep(self.response_delay);

        let responses = self.responses.lock().unwrap();
        if responses.is_empty() {
            // Generate mock responses for each pattern
            Ok(patterns
                .iter()
                .enumerate()
                .map(|(i, pattern)| AiPatternResult {
                    pattern_id: pattern.id.clone(),
                    confidence: 0.8 + (i as f32 * 0.05),
                    message: format!("Mock detection: {}", pattern.name),
                    suggestion: Some(format!("Mock fix for {}", pattern.name)),
                    start_offset: i * 10,
                    end_offset: (i + 1) * 10,
                    related_patterns: vec![],
                })
                .collect())
        } else {
            Ok(responses.clone())
        }
    }

    fn suggest_fix(
        &self,
        _code: &str,
        pattern: &BehavioralPattern,
        _issue_span: (usize, usize),
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if self.should_fail {
            return Err("Mock AI client failure".into());
        }

        Ok(Some(format!("Mock fix suggestion for {}", pattern.name)))
    }
}

/// Helper to create a test analysis context
fn create_test_context(file_path: &str, language: &str) -> AnalysisContext {
    AnalysisContext {
        file_path: file_path.to_string(),
        file_type: match language {
            "typescript" => SourceType::ts(),
            "javascript" => SourceType::js(),
            _ => SourceType::js(),
        },
        project_context: Some(ProjectContext {
            framework: Some("React".to_string()),
            build_tool: Some("Vite".to_string()),
            testing_framework: Some("Jest".to_string()),
            package_json_dependencies: HashMap::new(),
        }),
        dependencies: vec!["react".to_string(), "typescript".to_string()],
    }
}

/// Helper to parse code and create AST for testing
fn parse_test_code(code: &str) -> (Allocator, Program) {
    let allocator = Allocator::default();
    let source_type = SourceType::tsx(); // Support JSX for React tests
    let ParserReturn { program, .. } = Parser::new(&allocator, code, source_type).parse();
    (allocator, program)
}

#[fixture]
fn sample_react_component() -> String {
    r#"
        function MyComponent() {
            const [state, setState] = useState(0);
            
            useEffect(() => {
                // Missing dependency array - should trigger re-render pattern
                setState(state + 1);
            });
            
            return (
                <div onClick={() => setState(state + 1)}>
                    {/* Inline object creation - should trigger re-render pattern */}
                    <ChildComponent style={{margin: 10}} />
                    {state}
                </div>
            );
        }
    "#.to_string()
}

#[fixture]
fn sample_complex_code() -> String {
    r#"
        function complexFunction(input) {
            if (condition1) {
                for (let i = 0; i < 100; i++) {
                    if (condition2 && condition3) {
                        switch (value) {
                            case 1:
                                if (subCondition) {
                                    setTimeout(() => {
                                        // Potential memory leak - no cleanup
                                        document.addEventListener('click', handler);
                                    }, 1000);
                                }
                                break;
                            case 2:
                                if (anotherCondition || yetAnotherCondition) {
                                    // More nested complexity
                                    while (loopCondition) {
                                        // Deep nesting
                                    }
                                }
                                break;
                        }
                    }
                }
            }
        }
    "#.to_string()
}

#[fixture]
fn sample_security_vulnerable_code() -> String {
    r#"
        function handleUserInput() {
            const userInput = document.getElementById('user-input').value;
            
            // XSS vulnerability
            document.getElementById('output').innerHTML = userInput;
            
            // SQL injection pattern
            const query = "SELECT * FROM users WHERE id = " + userId;
            db.query(query);
            
            // Hardcoded secrets
            const apiKey = "sk-1234567890abcdef";
            const password = "admin123";
        }
    "#.to_string()
}

#[rstest]
fn test_behavioral_analyzer_creation() {
    let analyzer = AiBehavioralAnalyzer::new();
    let patterns = analyzer.get_patterns();
    
    assert!(!patterns.is_empty(), "Analyzer should have default patterns");
    assert!(patterns.len() >= 5, "Should have multiple behavioral patterns");
    
    // Check for essential pattern types
    let has_performance = patterns.iter().any(|p| matches!(p.pattern_type, BehavioralPatternType::PerformanceAntiPattern));
    let has_security = patterns.iter().any(|p| matches!(p.pattern_type, BehavioralPatternType::SecurityVulnerability));
    let has_complexity = patterns.iter().any(|p| matches!(p.pattern_type, BehavioralPatternType::CognitiveComplexity));
    let has_accessibility = patterns.iter().any(|p| matches!(p.pattern_type, BehavioralPatternType::AccessibilityViolation));
    
    assert!(has_performance, "Should have performance anti-pattern detection");
    assert!(has_security, "Should have security vulnerability detection");
    assert!(has_complexity, "Should have cognitive complexity detection");
    assert!(has_accessibility, "Should have accessibility violation detection");
}

#[rstest]
fn test_behavioral_analyzer_with_ai_client() {
    let mock_client = MockAiAnalysisClient::new();
    let analyzer = AiBehavioralAnalyzer::with_ai_client(Box::new(mock_client));
    
    let patterns = analyzer.get_patterns();
    assert!(!patterns.is_empty(), "Analyzer with AI client should have patterns");
}

#[rstest]
fn test_cognitive_complexity_heuristic(sample_complex_code: String) {
    let analyzer = AiBehavioralAnalyzer::new();
    
    // Test simple code
    let simple_code = "function simple() { return 1; }";
    let simple_score = analyzer.calculate_cognitive_complexity_heuristic(simple_code);
    assert!(simple_score <= 5, "Simple code should have low complexity score: {}", simple_score);
    
    // Test complex code
    let complex_score = analyzer.calculate_cognitive_complexity_heuristic(&sample_complex_code);
    assert!(complex_score > 15, "Complex code should have high complexity score: {}", complex_score);
    
    // Verify complexity is higher for more complex code
    assert!(complex_score > simple_score, "Complex code should score higher than simple code");
}

#[rstest]
fn test_heuristic_analysis_memory_leak_detection() {
    let analyzer = AiBehavioralAnalyzer::new();
    let context = create_test_context("test.js", "javascript");
    
    // Code with memory leak pattern
    let leak_code = r#"
        function setupListeners() {
            document.addEventListener('click', handleClick);
            setInterval(updateUI, 1000);
            // Missing cleanup - potential memory leak
        }
    "#;
    
    let (allocator, program) = parse_test_code(leak_code);
    let result = analyzer.run_heuristic_analysis(leak_code, &program, &context);
    
    assert!(result.is_ok(), "Heuristic analysis should succeed");
    let diagnostics = result.unwrap();
    
    // Should detect potential memory leak
    let has_memory_leak_warning = diagnostics.iter().any(|d| {
        d.rule_name.contains("memory-leak") || d.message.contains("memory leak")
    });
    assert!(has_memory_leak_warning, "Should detect potential memory leak pattern");
}

#[rstest]
fn test_heuristic_analysis_high_complexity(sample_complex_code: String) {
    let analyzer = AiBehavioralAnalyzer::new();
    let context = create_test_context("complex.js", "javascript");
    
    let (allocator, program) = parse_test_code(&sample_complex_code);
    let result = analyzer.run_heuristic_analysis(&sample_complex_code, &program, &context);
    
    assert!(result.is_ok(), "Heuristic analysis should succeed");
    let diagnostics = result.unwrap();
    
    // Should detect high cognitive complexity
    let has_complexity_warning = diagnostics.iter().any(|d| {
        d.rule_name.contains("cognitive-complexity") || d.message.contains("cognitive complexity")
    });
    assert!(has_complexity_warning, "Should detect high cognitive complexity");
    
    // Check that fix suggestions are provided
    let complexity_diagnostic = diagnostics.iter()
        .find(|d| d.rule_name.contains("cognitive-complexity"))
        .expect("Should have complexity diagnostic");
    
    assert!(complexity_diagnostic.fix_available, "Complex code should have fix suggestions");
    assert!(complexity_diagnostic.suggested_fix.is_some(), "Should provide fix suggestion");
}

#[tokio::test]
async fn test_ai_behavioral_analysis_with_mock_client() {
    let mock_client = MockAiAnalysisClient::new();
    
    // Add specific response for React re-render pattern
    mock_client.add_response(AiPatternResult {
        pattern_id: "react-excessive-rerenders".to_string(),
        confidence: 0.92,
        message: "Component may re-render excessively due to missing useEffect dependencies".to_string(),
        suggestion: Some("Add state to useEffect dependency array or use useCallback".to_string()),
        start_offset: 50,
        end_offset: 100,
        related_patterns: vec!["react-hooks-deps".to_string()],
    });
    
    let analyzer = AiBehavioralAnalyzer::with_ai_client(Box::new(mock_client));
    let context = create_test_context("Component.tsx", "typescript");
    
    let react_code = r#"
        function Component() {
            const [state, setState] = useState(0);
            useEffect(() => {
                setState(state + 1);
            });
            return <div>{state}</div>;
        }
    "#;
    
    let (allocator, program) = parse_test_code(react_code);
    let result = analyzer.analyze_behavioral_patterns(react_code, &program, &context).await;
    
    assert!(result.is_ok(), "AI behavioral analysis should succeed");
    let diagnostics = result.unwrap();
    
    // Should have at least heuristic results
    assert!(!diagnostics.is_empty(), "Should detect behavioral issues");
    
    // Check for AI-detected patterns
    let ai_diagnostics: Vec<_> = diagnostics.iter()
        .filter(|d| d.rule_name.starts_with("ai-behavioral:"))
        .collect();
    
    if !ai_diagnostics.is_empty() {
        let react_diagnostic = &ai_diagnostics[0];
        assert!(react_diagnostic.message.contains("re-render"), "Should detect React re-render issues");
        assert!(react_diagnostic.fix_available, "Should provide fix for detected issues");
        assert!(react_diagnostic.message.contains("92.0%"), "Should include AI confidence score");
    }
}

#[tokio::test]
async fn test_ai_client_failure_handling() {
    let failing_client = MockAiAnalysisClient::new().with_failure();
    let analyzer = AiBehavioralAnalyzer::with_ai_client(Box::new(failing_client));
    let context = create_test_context("test.js", "javascript");
    
    let simple_code = "function test() { return 1; }";
    let (allocator, program) = parse_test_code(simple_code);
    
    let result = analyzer.analyze_behavioral_patterns(simple_code, &program, &context).await;
    
    // Should still succeed with heuristic analysis even if AI fails
    assert!(result.is_ok(), "Should gracefully handle AI client failures");
    
    let diagnostics = result.unwrap();
    
    // Should only have heuristic results, no AI results
    let ai_diagnostics: Vec<_> = diagnostics.iter()
        .filter(|d| d.rule_name.starts_with("ai-behavioral:"))
        .collect();
    
    assert!(ai_diagnostics.is_empty(), "Should have no AI diagnostics when client fails");
}

#[rstest]
fn test_custom_pattern_addition() {
    let mut analyzer = AiBehavioralAnalyzer::new();
    let initial_count = analyzer.get_patterns().len();
    
    // Add custom pattern
    let custom_pattern = BehavioralPattern {
        id: "custom-test-pattern".to_string(),
        name: "Custom Test Pattern".to_string(),
        description: "A test pattern for unit testing".to_string(),
        category: "testing".to_string(),
        severity: RuleSeverity::Warning,
        pattern_type: BehavioralPatternType::TestingAntiPattern,
        ai_prompt: "Look for test-specific issues".to_string(),
        confidence_threshold: 0.75,
    };
    
    analyzer.add_pattern(custom_pattern.clone());
    
    let new_patterns = analyzer.get_patterns();
    assert_eq!(new_patterns.len(), initial_count + 1, "Should add one pattern");
    
    let added_pattern = new_patterns.iter()
        .find(|p| p.id == "custom-test-pattern")
        .expect("Should find added pattern");
    
    assert_eq!(added_pattern.name, "Custom Test Pattern");
    assert_eq!(added_pattern.confidence_threshold, 0.75);
    assert!(matches!(added_pattern.pattern_type, BehavioralPatternType::TestingAntiPattern));
}

#[rstest]
fn test_pattern_confidence_thresholds() {
    let analyzer = AiBehavioralAnalyzer::new();
    let patterns = analyzer.get_patterns();
    
    // Security patterns should have high confidence thresholds
    let security_patterns: Vec<_> = patterns.iter()
        .filter(|p| matches!(p.pattern_type, BehavioralPatternType::SecurityVulnerability))
        .collect();
    
    assert!(!security_patterns.is_empty(), "Should have security patterns");
    
    for pattern in security_patterns {
        assert!(pattern.confidence_threshold >= 0.8, 
            "Security pattern '{}' should have high confidence threshold: {}", 
            pattern.name, pattern.confidence_threshold);
    }
    
    // Performance patterns should have reasonable thresholds
    let performance_patterns: Vec<_> = patterns.iter()
        .filter(|p| matches!(p.pattern_type, BehavioralPatternType::PerformanceAntiPattern))
        .collect();
    
    assert!(!performance_patterns.is_empty(), "Should have performance patterns");
    
    for pattern in performance_patterns {
        assert!(pattern.confidence_threshold >= 0.6 && pattern.confidence_threshold <= 0.9, 
            "Performance pattern '{}' should have reasonable confidence threshold: {}", 
            pattern.name, pattern.confidence_threshold);
    }
}

#[rstest]
fn test_severity_conversion() {
    let test_cases = vec![
        (RuleSeverity::Error, DiagnosticSeverity::Error),
        (RuleSeverity::Warning, DiagnosticSeverity::Warning),
        (RuleSeverity::Info, DiagnosticSeverity::Info),
        (RuleSeverity::Hint, DiagnosticSeverity::Hint),
        (RuleSeverity::Custom("test".to_string()), DiagnosticSeverity::Warning),
    ];
    
    for (rule_severity, expected_diagnostic_severity) in test_cases {
        let converted = AiBehavioralAnalyzer::convert_severity(&rule_severity);
        assert_eq!(converted, expected_diagnostic_severity, 
            "Severity conversion failed for {:?}", rule_severity);
    }
}

#[rstest]
fn test_position_calculation() {
    let analyzer = AiBehavioralAnalyzer::new();
    
    let test_code = "line 1\nline 2\nline 3";
    
    // Test position at start
    let (line, col) = analyzer.calculate_position(test_code, 0);
    assert_eq!((line, col), (1, 1), "Position at start should be (1, 1)");
    
    // Test position at beginning of second line
    let (line, col) = analyzer.calculate_position(test_code, 7); // After "line 1\n"
    assert_eq!((line, col), (2, 1), "Position at second line should be (2, 1)");
    
    // Test position in middle of second line
    let (line, col) = analyzer.calculate_position(test_code, 10); // "i" in "line 2"
    assert_eq!((line, col), (2, 4), "Position in second line should be (2, 4)");
}

#[rstest]
fn test_pattern_threshold_lookup() {
    let analyzer = AiBehavioralAnalyzer::new();
    
    // Test existing pattern
    let threshold = analyzer.get_pattern_threshold("react-excessive-rerenders");
    assert!(threshold > 0.0 && threshold <= 1.0, "Threshold should be valid probability");
    
    // Test non-existent pattern (should return default)
    let default_threshold = analyzer.get_pattern_threshold("non-existent-pattern");
    assert_eq!(default_threshold, 0.8, "Non-existent pattern should return default threshold");
}

#[rstest]
fn test_analysis_context_creation() {
    let context = create_test_context("src/components/Button.tsx", "typescript");
    
    assert_eq!(context.file_path, "src/components/Button.tsx");
    assert!(context.file_type.is_typescript());
    assert!(context.project_context.is_some());
    
    let project_context = context.project_context.unwrap();
    assert_eq!(project_context.framework, Some("React".to_string()));
    assert_eq!(project_context.build_tool, Some("Vite".to_string()));
    assert_eq!(project_context.testing_framework, Some("Jest".to_string()));
    
    assert!(context.dependencies.contains(&"react".to_string()));
    assert!(context.dependencies.contains(&"typescript".to_string()));
}

#[tokio::test]
async fn test_performance_of_behavioral_analysis() {
    let analyzer = AiBehavioralAnalyzer::new();
    let context = create_test_context("large-file.ts", "typescript");
    
    // Create a moderately large code sample
    let large_code = (0..100)
        .map(|i| format!("function func{}() {{ return {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    
    let (allocator, program) = parse_test_code(&large_code);
    
    let start_time = std::time::Instant::now();
    let result = analyzer.run_heuristic_analysis(&large_code, &program, &context);
    let analysis_time = start_time.elapsed();
    
    assert!(result.is_ok(), "Analysis of large code should succeed");
    assert!(analysis_time.as_millis() < 1000, "Analysis should complete quickly: {}ms", analysis_time.as_millis());
    
    let diagnostics = result.unwrap();
    // Large code might not have issues, so we just check it doesn't crash
    assert!(diagnostics.len() < 100, "Should not generate excessive diagnostics");
}

#[rstest]
fn test_default_behavioral_patterns_completeness() {
    let analyzer = AiBehavioralAnalyzer::new();
    let patterns = analyzer.get_patterns();
    
    // Check that we have patterns for major categories
    let pattern_types: std::collections::HashSet<_> = patterns.iter()
        .map(|p| std::mem::discriminant(&p.pattern_type))
        .collect();
    
    assert!(pattern_types.len() >= 4, "Should have patterns for multiple categories");
    
    // Verify each pattern has required fields
    for pattern in patterns {
        assert!(!pattern.id.is_empty(), "Pattern should have non-empty ID");
        assert!(!pattern.name.is_empty(), "Pattern should have non-empty name");
        assert!(!pattern.description.is_empty(), "Pattern should have non-empty description");
        assert!(!pattern.ai_prompt.is_empty(), "Pattern should have non-empty AI prompt");
        assert!(pattern.confidence_threshold > 0.0 && pattern.confidence_threshold <= 1.0, 
            "Pattern confidence threshold should be valid: {}", pattern.confidence_threshold);
    }
}
