/*!
 * Language-specific Documentation Templates
 *
 * Organized by programming language with DSPy-aware template management
 * that preserves template integrity during optimization cycles.
 */

pub mod typescript;
pub mod rust;

use crate::dspy::{MetaSignature, Example};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Language-specific template manager with DSPy protection
#[derive(Debug, Clone)]
pub struct LanguageTemplateManager {
    /// Base templates (protected from DSPy changes)
    base_templates: HashMap<String, String>,
    /// DSPy-optimized templates (can be modified)
    optimized_templates: HashMap<String, String>,
    /// Template checksums for change detection
    template_checksums: HashMap<String, String>,
    /// DSPy optimization history
    optimization_history: Vec<OptimizationEntry>,
}

/// DSPy optimization entry with rollback capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEntry {
    /// Template ID
    pub template_id: String,
    /// Original template checksum
    pub original_checksum: String,
    /// Optimized template content
    pub optimized_content: String,
    /// Optimization timestamp
    pub timestamp: String,
    /// Performance metrics before optimization
    pub before_metrics: PerformanceMetrics,
    /// Performance metrics after optimization
    pub after_metrics: Option<PerformanceMetrics>,
    /// Whether optimization was successful
    pub success: bool,
    /// Rollback available
    pub can_rollback: bool,
}

/// Performance metrics for optimization tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Success rate (0.0 - 1.0)
    pub success_rate: f32,
    /// Average execution time in ms
    pub avg_execution_time: u32,
    /// Token efficiency (output/input ratio)
    pub token_efficiency: f32,
    /// Error rate
    pub error_rate: f32,
}

impl LanguageTemplateManager {
    /// Create new template manager with base templates
    pub fn new() -> Self {
        Self {
            base_templates: HashMap::new(),
            optimized_templates: HashMap::new(),
            template_checksums: HashMap::new(),
            optimization_history: Vec::new(),
        }
    }

    /// Register a base template (protected from DSPy changes)
    pub fn register_base_template(&mut self, id: String, content: String) -> Result<()> {
        let checksum = self.calculate_checksum(&content);
        self.base_templates.insert(id.clone(), content);
        self.template_checksums.insert(id, checksum);
        Ok(())
    }

    /// Get template for DSPy use (returns optimized if available, base otherwise)
    pub fn get_template_for_dspy(&self, id: &str) -> Option<String> {
        // Prefer optimized template if available and successful
        if let Some(optimized) = self.optimized_templates.get(id) {
            if self.is_optimization_successful(id) {
                return Some(optimized.clone());
            }
        }

        // Fallback to base template
        self.base_templates.get(id).cloned()
    }

    /// Get base template (always returns original, unmodified version)
    pub fn get_base_template(&self, id: &str) -> Option<String> {
        self.base_templates.get(id).cloned()
    }

    /// Apply DSPy optimization while preserving base template
    pub fn apply_dspy_optimization(
        &mut self,
        template_id: String,
        optimized_content: String,
        before_metrics: PerformanceMetrics,
    ) -> Result<()> {
        let original_checksum = self.template_checksums.get(&template_id)
            .ok_or_else(|| crate::error::Error::config("Template not found".to_string()))?
            .clone();

        let optimization_entry = OptimizationEntry {
            template_id: template_id.clone(),
            original_checksum,
            optimized_content: optimized_content.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            before_metrics,
            after_metrics: None,
            success: false, // Will be updated after testing
            can_rollback: true,
        };

        self.optimized_templates.insert(template_id, optimized_content);
        self.optimization_history.push(optimization_entry);

        Ok(())
    }

    /// Update optimization metrics after testing
    pub fn update_optimization_metrics(
        &mut self,
        template_id: &str,
        after_metrics: PerformanceMetrics,
        success: bool,
    ) -> Result<()> {
        if let Some(entry) = self.optimization_history.iter_mut()
            .filter(|e| e.template_id == template_id)
            .last()
        {
            entry.after_metrics = Some(after_metrics);
            entry.success = success;
        }

        Ok(())
    }

    /// Rollback DSPy optimization to base template
    pub fn rollback_optimization(&mut self, template_id: &str) -> Result<()> {
        if let Some(entry) = self.optimization_history.iter_mut()
            .filter(|e| e.template_id == template_id && e.can_rollback)
            .last()
        {
            self.optimized_templates.remove(template_id);
            entry.can_rollback = false;
            entry.success = false;
        }

        Ok(())
    }

    /// Check if current optimization is successful
    fn is_optimization_successful(&self, template_id: &str) -> bool {
        self.optimization_history.iter()
            .filter(|e| e.template_id == template_id)
            .last()
            .map(|e| e.success)
            .unwrap_or(false)
    }

    /// Calculate checksum for template content
    fn calculate_checksum(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get optimization history for a template
    pub fn get_optimization_history(&self, template_id: &str) -> Vec<&OptimizationEntry> {
        self.optimization_history.iter()
            .filter(|e| e.template_id == template_id)
            .collect()
    }

    /// Reset template to base version (clears all optimizations)
    pub fn reset_to_base(&mut self, template_id: &str) -> Result<()> {
        self.optimized_templates.remove(template_id);

        // Mark all optimizations for this template as rolled back
        for entry in self.optimization_history.iter_mut() {
            if entry.template_id == template_id {
                entry.can_rollback = false;
                entry.success = false;
            }
        }

        Ok(())
    }

    /// Export template with protection markers for sed editing
    pub fn export_sed_template(&self, template_id: &str, include_protection: bool) -> Option<String> {
        let base_template = self.get_base_template(template_id)?;

        if include_protection {
            Some(format!(
                r#"# %%TEMPLATE:PROTECTED:START%%
# BASE TEMPLATE - DO NOT MODIFY DIRECTLY
# Use DSPy optimization or sed field editing only
# Template ID: {template_id}
# Checksum: {checksum}
# %%TEMPLATE:PROTECTED:END%%

{base_template}

# %%TEMPLATE:OPTIMIZED:START%%
# DSPy-optimized version will appear here
# %%TEMPLATE:OPTIMIZED:END%%"#,
                template_id = template_id,
                checksum = self.template_checksums.get(template_id).unwrap_or(&"unknown".to_string()),
                base_template = base_template
            ))
        } else {
            Some(base_template)
        }
    }
}

/// Template protection wrapper for DSPy integration
pub struct ProtectedTemplate {
    /// Template ID
    pub id: String,
    /// Protected base content
    pub base_content: String,
    /// Current active content (may be optimized)
    pub active_content: String,
    /// Protection level
    pub protection_level: ProtectionLevel,
}

/// Protection levels for templates
#[derive(Debug, Clone, PartialEq)]
pub enum ProtectionLevel {
    /// Full protection - base template cannot be changed
    Full,
    /// Field protection - only specific fields can be changed via sed
    FieldOnly,
    /// DSPy protection - DSPy can optimize but base is preserved
    DSPyOptimization,
    /// No protection - template can be freely modified
    None,
}

impl ProtectedTemplate {
    /// Create new protected template
    pub fn new(id: String, content: String, protection_level: ProtectionLevel) -> Self {
        Self {
            id,
            base_content: content.clone(),
            active_content: content,
            protection_level,
        }
    }

    /// Apply DSPy optimization if allowed
    pub fn apply_dspy_optimization(&mut self, optimized_content: String) -> Result<bool> {
        match self.protection_level {
            ProtectionLevel::DSPyOptimization | ProtectionLevel::None => {
                self.active_content = optimized_content;
                Ok(true)
            }
            _ => Ok(false) // Optimization not allowed
        }
    }

    /// Reset to base template
    pub fn reset_to_base(&mut self) {
        self.active_content = self.base_content.clone();
    }

    /// Check if template has been modified
    pub fn is_modified(&self) -> bool {
        self.base_content != self.active_content
    }

    /// Get effective template content
    pub fn get_content(&self) -> &str {
        &self.active_content
    }

    /// Get base template content (always original)
    pub fn get_base_content(&self) -> &str {
        &self.base_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_manager_registration() {
        let mut manager = LanguageTemplateManager::new();

        let template_content = "TASK: Test template\n{code}".to_string();
        manager.register_base_template("test".to_string(), template_content.clone()).unwrap();

        assert_eq!(manager.get_base_template("test"), Some(template_content));
        assert_eq!(manager.get_template_for_dspy("test"), Some(template_content));
    }

    #[test]
    fn test_dspy_optimization() {
        let mut manager = LanguageTemplateManager::new();

        let base_content = "Original template".to_string();
        let optimized_content = "Optimized template".to_string();

        manager.register_base_template("test".to_string(), base_content.clone()).unwrap();

        let metrics = PerformanceMetrics {
            success_rate: 0.8,
            avg_execution_time: 1000,
            token_efficiency: 0.75,
            error_rate: 0.2,
        };

        manager.apply_dspy_optimization("test".to_string(), optimized_content.clone(), metrics).unwrap();

        // Base template should remain unchanged
        assert_eq!(manager.get_base_template("test"), Some(base_content));

        // DSPy should get optimized version (but success=false initially)
        assert_eq!(manager.get_template_for_dspy("test"), Some(base_content));

        // Update metrics to successful
        let after_metrics = PerformanceMetrics {
            success_rate: 0.9,
            avg_execution_time: 800,
            token_efficiency: 0.85,
            error_rate: 0.1,
        };

        manager.update_optimization_metrics("test", after_metrics, true).unwrap();

        // Now DSPy should get optimized version
        assert_eq!(manager.get_template_for_dspy("test"), Some(optimized_content));
    }

    #[test]
    fn test_optimization_rollback() {
        let mut manager = LanguageTemplateManager::new();

        let base_content = "Base template".to_string();
        let optimized_content = "Optimized template".to_string();

        manager.register_base_template("test".to_string(), base_content.clone()).unwrap();

        let metrics = PerformanceMetrics {
            success_rate: 0.8,
            avg_execution_time: 1000,
            token_efficiency: 0.75,
            error_rate: 0.2,
        };

        manager.apply_dspy_optimization("test".to_string(), optimized_content, metrics).unwrap();
        manager.update_optimization_metrics("test", metrics, true).unwrap();

        // Rollback optimization
        manager.rollback_optimization("test").unwrap();

        // Should return base template
        assert_eq!(manager.get_template_for_dspy("test"), Some(base_content));
    }

    #[test]
    fn test_protected_template() {
        let mut template = ProtectedTemplate::new(
            "test".to_string(),
            "Original content".to_string(),
            ProtectionLevel::DSPyOptimization,
        );

        assert!(!template.is_modified());
        assert_eq!(template.get_content(), "Original content");

        // Apply optimization
        let success = template.apply_dspy_optimization("Optimized content".to_string()).unwrap();
        assert!(success);
        assert!(template.is_modified());
        assert_eq!(template.get_content(), "Optimized content");
        assert_eq!(template.get_base_content(), "Original content");

        // Reset to base
        template.reset_to_base();
        assert!(!template.is_modified());
        assert_eq!(template.get_content(), "Original content");
    }

    #[test]
    fn test_protection_levels() {
        let mut full_protected = ProtectedTemplate::new(
            "full".to_string(),
            "Content".to_string(),
            ProtectionLevel::Full,
        );

        let success = full_protected.apply_dspy_optimization("Modified".to_string()).unwrap();
        assert!(!success); // Should not allow modification
        assert_eq!(full_protected.get_content(), "Content");

        let mut dspy_protected = ProtectedTemplate::new(
            "dspy".to_string(),
            "Content".to_string(),
            ProtectionLevel::DSPyOptimization,
        );

        let success = dspy_protected.apply_dspy_optimization("Modified".to_string()).unwrap();
        assert!(success); // Should allow DSPy optimization
        assert_eq!(dspy_protected.get_content(), "Modified");
    }
}