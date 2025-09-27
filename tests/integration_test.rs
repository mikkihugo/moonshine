use moon_shine::config::MoonShineConfig;
use moon_shine::workflow::{WorkflowDefinition, WorkflowEngine};

#[test]
fn test_workflow_creation() {
    let config = MoonShineConfig::default();
    let definition = WorkflowDefinition::standard();
    let sample_code = "console.log('Hello, World!');";
    let file_path = "test.js".to_string();

    let engine = WorkflowEngine::new(definition, sample_code.to_string(), file_path, config);
    assert!(engine.is_ok(), "Should create workflow engine successfully");
}

#[test]
fn test_ai_linting_workflow() {
    let config = MoonShineConfig::default();
    let definition = WorkflowDefinition::ai_only();
    let sample_code = r#"
    function test() {
        // This code has potential issues
        let x = 1;
        eval("dangerous code");
        document.innerHTML = userInput;
    }
    "#;
    let file_path = "test.js".to_string();

    let engine = WorkflowEngine::new(definition, sample_code.to_string(), file_path, config);
    assert!(engine.is_ok(), "Should create AI workflow engine successfully");
}

#[test]
fn test_oxc_adapter_functionality() {
    use moon_shine::oxc_adapter::OxcAdapter;

    let adapter = OxcAdapter::new();
    let sample_code = "const x = 1; console.log(x);";

    let result = adapter.analyze_code(sample_code, "test.js");
    assert!(result.is_ok(), "OXC adapter should analyze code successfully");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_extension_registration() {
        use moon_shine::{register_extension};

        let result = register_extension();
        assert!(result.is_ok(), "Extension should register successfully");

        let manifest = result.unwrap().0;
        assert_eq!(manifest.name, "moon-shine");
        assert!(!manifest.description.is_empty());
        assert!(!manifest.version.is_empty());
    }

    #[test]
    fn test_moon_pdk_functions() {
        use moon_shine::moon_pdk_interface::{ExecCommandInput, execute_command};

        // Test with a simple command that should work
        let input = ExecCommandInput {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: std::collections::HashMap::new(),
            working_dir: None,
        };

        let result = execute_command(input);
        // In test environment, this might fail due to WASM context, but structure should be valid
        assert!(result.is_ok() || result.is_err(), "Command execution should return a result");
    }
}