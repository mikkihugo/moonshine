//! Compile-time prompt templates - zero runtime parsing!
//!
//! Prompt templates are converted from JSON to native Rust const data at build time.
//! This provides instant access with zero parsing overhead.

// Include the auto-generated prompt templates
include!(concat!(env!("OUT_DIR"), "/compiled_prompts.rs"));

/// Get default prompt template by name with zero runtime cost
pub fn get_compiled_prompt_template(name: &str) -> Option<&'static str> {
    get_default_prompt_template(name)
}

/// Get all available template names
pub fn available_template_names() -> Vec<&'static str> {
    DEFAULT_PROMPT_TEMPLATES.iter().map(|(name, _)| *name).collect()
}

/// Get template metadata (name, display_name, variables)
pub fn get_template_metadata(name: &str) -> Option<(&'static str, &'static [&'static str])> {
    PROMPT_TEMPLATE_METADATA
        .iter()
        .find(|(template_name, _, _)| *template_name == name)
        .map(|(_, display_name, variables)| (*display_name, *variables))
}

/// Check if a template exists
pub fn has_template(name: &str) -> bool {
    get_compiled_prompt_template(name).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_templates_loaded() {
        assert!(!DEFAULT_PROMPT_TEMPLATES.is_empty());
        assert!(!PROMPT_TEMPLATE_METADATA.is_empty());
    }

    #[test]
    fn test_template_access() {
        let available_names = available_template_names();
        assert!(!available_names.is_empty());

        // Test accessing a specific template
        if let Some(first_name) = available_names.first() {
            assert!(has_template(first_name));
            let template = get_compiled_prompt_template(first_name);
            assert!(template.is_some());
            assert!(!template.unwrap().is_empty());
        }
    }

    #[test]
    fn test_template_metadata() {
        let available_names = available_template_names();

        for name in available_names {
            let metadata = get_template_metadata(name);
            assert!(metadata.is_some());

            let (display_name, variables) = metadata.unwrap();
            assert!(!display_name.is_empty());
            // Variables array can be empty for simple templates
        }
    }

    #[test]
    fn test_basic_templates_exist() {
        // Test that basic templates are compiled
        assert!(has_template("code_analysis"));
        assert!(has_template("type_fixing"));
        assert!(has_template("eslint_fixing"));
    }
}
