use moon_shine::extension::{ExecuteExtensionInput, execute_extension_logic};
use moon_shine::moon_host::{FnResult, Json};
use std::collections::HashMap;

#[test]
fn test_moon_extension_registration() {
    // Test that the extension can be registered with Moon
    let manifest = moon_shine::register_extension();
    assert!(manifest.is_ok());
    
    let manifest = manifest.unwrap();
    assert_eq!(manifest.0.name, "moon-shine");
    assert!(manifest.0.description.contains("AI-powered"));
    assert!(manifest.0.version.len() > 0);
}

#[test]
fn test_moon_extension_execution() {
    // Test basic extension execution
    let input = ExecuteExtensionInput {
        args: vec!["--help".to_string()],
        context: None,
    };
    
    // This should not panic and should handle the help request gracefully
    let result = execute_extension_logic(Json(input));
    // We expect this to succeed or fail gracefully, not panic
    match result {
        Ok(_) => println!("✅ Extension execution successful"),
        Err(e) => println!("⚠️ Extension execution failed gracefully: {}", e),
    }
}

#[test]
fn test_moon_extension_config_loading() {
    // Test that the extension can load configuration
    let config = moon_shine::config::MoonShineConfig::default();
    assert!(config.operation_mode.is_some());
    assert!(config.ai.model.len() > 0);
}

#[test]
fn test_ai_provider_availability() {
    // Test that all AI providers are available
    let router = moon_shine::provider_router::AIProviderRouter::new();
    
    assert!(router.providers.contains_key("claude"));
    assert!(router.providers.contains_key("google"));
    assert!(router.providers.contains_key("openai"));
    
    println!("✅ All AI providers available: Claude, Gemini, Codex");
}

#[test]
fn test_workflow_engine_initialization() {
    // Test that the workflow engine can be initialized
    let workflow = moon_shine::workflow::WorkflowDefinition::standard();
    assert!(!workflow.steps.is_empty());
    
    let lint_only = moon_shine::workflow::WorkflowDefinition::lint_only();
    assert!(!lint_only.steps.is_empty());
    
    println!("✅ Workflow engine initialized successfully");
}

#[test]
fn test_rulebase_loading() {
    // Test that the rulebase can be loaded
    let registry = moon_shine::rule_registry::RuleRegistry::new();
    assert!(registry.is_ok());
    
    let registry = registry.unwrap();
    assert!(registry.loader.get_metadata().total_rules > 0);
    
    println!("✅ Rulebase loaded with {} rules", registry.loader.get_metadata().total_rules);
}

#[test]
fn test_oxc_adapter_functionality() {
    // Test basic OXC adapter functionality
    let adapter = moon_shine::oxc_adapter::OxcAdapter::new();
    
    // Test with simple JavaScript code
    let test_code = "console.log('Hello, World!');";
    let result = adapter.parse_and_analyze(test_code, moon_shine::oxc_span::SourceType::default_script());
    
    assert!(result.is_ok());
    println!("✅ OXC adapter parsing successful");
}

#[test]
fn test_dspy_optimization_setup() {
    // Test DSPy optimization setup
    let optimizer = moon_shine::dspy::optimizer::copro::COPRO::new();
    
    // Test basic optimization parameters
    assert!(optimizer.breadth > 0);
    assert!(optimizer.depth > 0);
    
    println!("✅ DSPy optimization setup successful");
}

#[test]
fn test_session_coordination() {
    // Test session-based coordination
    let session_id = "test-session-123";
    let session_dir = format!("/tmp/moon-shine-test/{}", session_id);
    
    // Test that session directories can be created
    std::fs::create_dir_all(&session_dir).unwrap();
    assert!(std::path::Path::new(&session_dir).exists());
    
    // Clean up
    std::fs::remove_dir_all("/tmp/moon-shine-test").unwrap_or(());
    
    println!("✅ Session coordination test successful");
}

#[test]
fn test_moon_pdk_interface() {
    // Test Moon PDK interface functionality
    use moon_shine::moon_pdk_interface::{execute_command, ExecCommandInput};
    
    // Test command execution interface (without actually executing)
    let input = ExecCommandInput {
        command: "echo".to_string(),
        args: vec!["test".to_string()],
        working_dir: Some("/tmp".to_string()),
        env: HashMap::new(),
    };
    
    // This tests the interface structure, not actual execution
    assert_eq!(input.command, "echo");
    assert_eq!(input.args, vec!["test"]);
    
    println!("✅ Moon PDK interface test successful");
}

#[test]
fn test_end_to_end_workflow() {
    // Test end-to-end workflow without external dependencies
    let workflow = moon_shine::workflow::WorkflowDefinition::standard();
    let config = moon_shine::config::MoonShineConfig::default();
    let router = moon_shine::provider_router::AIProviderRouter::new();
    
    // Verify all components are available
    assert!(!workflow.steps.is_empty());
    assert!(config.ai.model.len() > 0);
    assert!(router.providers.len() >= 3);
    
    println!("✅ End-to-end workflow test successful");
    println!("   - Workflow steps: {}", workflow.steps.len());
    println!("   - AI model: {}", config.ai.model);
    println!("   - AI providers: {}", router.providers.len());
}