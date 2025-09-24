//! # Mock Infrastructure for London School Testing
//!
//! Mock implementations for isolated unit testing as per London school
//! (mockist) methodology requested by user.
//!
//! @category testing
//! @safe program
//! @complexity medium
//! @since 2.0.0

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::analysis::{AnalysisResults, MoonShineResponse};
use crate::config::MoonShineConfig;
use crate::error::{Error, Result};
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::linter::SuggestionCategory;

/// Mock AI provider for isolated testing
#[derive(Debug, Clone)]
pub struct MockAiProvider {
    responses: Arc<Mutex<Vec<String>>>,
    call_count: Arc<Mutex<usize>>,
    expected_prompts: Arc<Mutex<Vec<String>>>,
    should_fail: Arc<Mutex<bool>>,
}

impl MockAiProvider {
    /// Create new mock AI provider
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(Vec::new())),
            call_count: Arc::new(Mutex::new(0)),
            expected_prompts: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    /// Add expected response for mock
    pub fn expect_response(&self, response: &str) {
        let mut responses = self.responses.lock().unwrap();
        responses.push(response.to_string());
    }

    /// Add expected prompt for verification
    pub fn expect_prompt(&self, prompt: &str) {
        let mut prompts = self.expected_prompts.lock().unwrap();
        prompts.push(prompt.to_string());
    }

    /// Configure mock to fail
    pub fn should_fail(&self, fail: bool) {
        let mut should_fail = self.should_fail.lock().unwrap();
        *should_fail = fail;
    }

    /// Get number of calls made to mock
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Verify all expected prompts were received
    pub fn verify_prompts(&self) -> bool {
        let expected = self.expected_prompts.lock().unwrap();
        let count = self.call_count.lock().unwrap();
        expected.len() == *count
    }

    /// Simulate AI response
    pub async fn generate_response(&self, _prompt: &str) -> Result<String> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;

        let should_fail = *self.should_fail.lock().unwrap();
        if should_fail {
            return Err(Error::Configuration {
                message: "Mock failure".to_string(),
            });
        }

        let mut responses = self.responses.lock().unwrap();
        if !responses.is_empty() {
            Ok(responses.remove(0))
        } else {
            Ok("Mock AI response".to_string())
        }
    }
}

/// Mock file system for isolated testing
#[derive(Debug, Clone)]
pub struct MockFileSystem {
    files: Arc<Mutex<HashMap<String, String>>>,
    directories: Arc<Mutex<Vec<String>>>,
    read_count: Arc<Mutex<usize>>,
    write_count: Arc<Mutex<usize>>,
}

impl MockFileSystem {
    /// Create new mock file system
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            directories: Arc::new(Mutex::new(Vec::new())),
            read_count: Arc::new(Mutex::new(0)),
            write_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add file to mock file system
    pub fn add_file(&self, path: &str, content: &str) {
        let mut files = self.files.lock().unwrap();
        files.insert(path.to_string(), content.to_string());
    }

    /// Add directory to mock file system
    pub fn add_directory(&self, path: &str) {
        let mut directories = self.directories.lock().unwrap();
        directories.push(path.to_string());
    }

    /// Read file from mock file system
    pub fn read_file(&self, path: &str) -> Result<String> {
        let mut count = self.read_count.lock().unwrap();
        *count += 1;

        let files = self.files.lock().unwrap();
        files.get(path).cloned().ok_or_else(|| Error::Config {
            message: format!("File not found: {}", path),
            field: None,
            value: None,
        })
    }

    /// Write file to mock file system
    pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let mut count = self.write_count.lock().unwrap();
        *count += 1;

        let mut files = self.files.lock().unwrap();
        files.insert(path.to_string(), content.to_string());
        Ok(())
    }

    /// Get read operation count
    pub fn read_count(&self) -> usize {
        *self.read_count.lock().unwrap()
    }

    /// Get write operation count
    pub fn write_count(&self) -> usize {
        *self.write_count.lock().unwrap()
    }

    /// Check if file exists in mock
    pub fn file_exists(&self, path: &str) -> bool {
        let files = self.files.lock().unwrap();
        files.contains_key(path)
    }

    /// List all files in mock
    pub fn list_files(&self) -> Vec<String> {
        let files = self.files.lock().unwrap();
        files.keys().cloned().collect()
    }
}

/// Mock configuration provider for isolated testing
#[derive(Debug, Clone)]
pub struct MockConfigProvider {
    config: Arc<Mutex<MoonShineConfig>>,
    load_count: Arc<Mutex<usize>>,
    should_fail_load: Arc<Mutex<bool>>,
}

impl MockConfigProvider {
    /// Create new mock configuration provider
    pub fn new(config: MoonShineConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            load_count: Arc::new(Mutex::new(0)),
            should_fail_load: Arc::new(Mutex::new(false)),
        }
    }

    /// Update mock configuration
    pub fn update_config(&self, config: MoonShineConfig) {
        let mut cfg = self.config.lock().unwrap();
        *cfg = config;
    }

    /// Configure mock to fail on load
    pub fn should_fail_load(&self, fail: bool) {
        let mut should_fail = self.should_fail_load.lock().unwrap();
        *should_fail = fail;
    }

    /// Load configuration from mock
    pub fn load_config(&self) -> Result<MoonShineConfig> {
        let mut count = self.load_count.lock().unwrap();
        *count += 1;

        let should_fail = *self.should_fail_load.lock().unwrap();
        if should_fail {
            return Err(Error::Config {
                message: "Mock config load failure".to_string(),
                field: None,
                value: None,
            });
        }

        let config = self.config.lock().unwrap();
        Ok(config.clone())
    }

    /// Get load operation count
    pub fn load_count(&self) -> usize {
        *self.load_count.lock().unwrap()
    }
}

/// Mock workflow engine for isolated testing
#[derive(Debug, Clone)]
pub struct MockWorkflowEngine {
    analysis_results: Arc<Mutex<Option<AnalysisResults>>>,
    execution_count: Arc<Mutex<usize>>,
    should_fail: Arc<Mutex<bool>>,
    execution_time_ms: Arc<Mutex<u64>>,
}

impl MockWorkflowEngine {
    /// Create new mock workflow engine
    pub fn new() -> Self {
        Self {
            analysis_results: Arc::new(Mutex::new(None)),
            execution_count: Arc::new(Mutex::new(0)),
            should_fail: Arc::new(Mutex::new(false)),
            execution_time_ms: Arc::new(Mutex::new(100)),
        }
    }

    /// Set expected analysis results
    pub fn expect_results(&self, results: AnalysisResults) {
        let mut analysis_results = self.analysis_results.lock().unwrap();
        *analysis_results = Some(results);
    }

    /// Configure mock to fail
    pub fn should_fail(&self, fail: bool) {
        let mut should_fail = self.should_fail.lock().unwrap();
        *should_fail = fail;
    }

    /// Set mock execution time
    pub fn set_execution_time(&self, time_ms: u64) {
        let mut execution_time = self.execution_time_ms.lock().unwrap();
        *execution_time = time_ms;
    }

    /// Execute mock workflow
    pub async fn execute(&self, _config: &MoonShineConfig, _files: Vec<String>) -> Result<AnalysisResults> {
        let mut count = self.execution_count.lock().unwrap();
        *count += 1;

        let should_fail = *self.should_fail.lock().unwrap();
        if should_fail {
            return Err(Error::Config {
                message: "Mock workflow failure".to_string(),
                field: None,
            });
        }

        // Simulate execution time
        let execution_time = *self.execution_time_ms.lock().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(execution_time)).await;

        let results = self.analysis_results.lock().unwrap();
        if let Some(ref results) = *results {
            Ok(results.clone())
        } else {
            // Return default mock results
            Ok(AnalysisResults {
                suggestions: vec![LintIssue {
                    message: "Mock suggestion".to_string(),
                    severity: LintSeverity::Info,
                    category: SuggestionCategory::BestPractices,
                    line: 1,
                    column: 1,
                    rule_id: Some("mock-rule".to_string()),
                    suggested_fix: Some("Mock fix".to_string()),
                    confidence: 0.9,
                    auto_fixable: false,
                    impact_score: 0,
                    related_suggestions: vec![],
                }],
                // files_processed, processing_time_ms, metadata fields removed
                ..Default::default()
            })
        }
    }

    /// Get execution count
    pub fn execution_count(&self) -> usize {
        *self.execution_count.lock().unwrap()
    }
}

/// Mock cache provider for isolated testing
#[derive(Debug, Clone)]
pub struct MockCache {
    cache: Arc<Mutex<HashMap<String, String>>>,
    hit_count: Arc<Mutex<usize>>,
    miss_count: Arc<Mutex<usize>>,
    should_fail: Arc<Mutex<bool>>,
}

impl MockCache {
    /// Create new mock cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            hit_count: Arc::new(Mutex::new(0)),
            miss_count: Arc::new(Mutex::new(0)),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    /// Put value in mock cache
    pub fn put(&self, key: &str, value: &str) -> Result<()> {
        let should_fail = *self.should_fail.lock().unwrap();
        if should_fail {
            return Err(Error::Config {
                message: "Mock cache failure".to_string(),
                field: None,
            });
        }

        let mut cache = self.cache.lock().unwrap();
        cache.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Get value from mock cache
    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        if let Some(value) = cache.get(key) {
            let mut hit_count = self.hit_count.lock().unwrap();
            *hit_count += 1;
            Some(value.clone())
        } else {
            let mut miss_count = self.miss_count.lock().unwrap();
            *miss_count += 1;
            None
        }
    }

    /// Configure mock to fail
    pub fn should_fail(&self, fail: bool) {
        let mut should_fail = self.should_fail.lock().unwrap();
        *should_fail = fail;
    }

    /// Get cache hit count
    pub fn hit_count(&self) -> usize {
        *self.hit_count.lock().unwrap()
    }

    /// Get cache miss count
    pub fn miss_count(&self) -> usize {
        *self.miss_count.lock().unwrap()
    }

    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let hits = self.hit_count();
        let misses = self.miss_count();
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }
}

/// Mock factory for creating all mocks
pub struct MockFactory;

impl MockFactory {
    /// Create complete mock environment for London school testing
    pub fn london_environment() -> LondonMockEnvironment {
        LondonMockEnvironment {
            ai_provider: MockAiProvider::new(),
            file_system: MockFileSystem::new(),
            config_provider: MockConfigProvider::new(MoonShineConfig::london_test()),
            workflow_engine: MockWorkflowEngine::new(),
            cache: MockCache::new(),
        }
    }
}

/// Complete mock environment for London school testing
#[derive(Debug, Clone)]
pub struct LondonMockEnvironment {
    pub ai_provider: MockAiProvider,
    pub file_system: MockFileSystem,
    pub config_provider: MockConfigProvider,
    pub workflow_engine: MockWorkflowEngine,
    pub cache: MockCache,
}

impl LondonMockEnvironment {
    /// Setup realistic mock data for testing
    pub fn setup_realistic_scenario(&self) {
        // Setup mock files
        self.file_system.add_file(
            "src/component.tsx",
            r#"
import React from 'react';

const Component: React.FC = () => {
    console.log('debug');
    return <div>Hello</div>;
};

export default Component;
"#,
        );

        // Setup expected AI responses
        self.ai_provider
            .expect_response(r#"{"suggestions": [{"message": "Use proper logging", "severity": "warning"}]}"#);

        // Setup cache data
        let _ = self.cache.put("analysis:src/component.tsx", "cached_result");
    }

    /// Verify all mocks were called as expected
    pub fn verify_all(&self) -> bool {
        self.ai_provider.verify_prompts() && self.file_system.read_count() > 0 && self.config_provider.load_count() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_ai_provider() {
        let mock = MockAiProvider::new();
        mock.expect_response("test response");

        let response = mock.generate_response("test prompt").await.unwrap();
        assert_eq!(response, "test response");
        assert_eq!(mock.call_count(), 1);
    }

    #[test]
    fn test_mock_file_system() {
        let mock = MockFileSystem::new();
        mock.add_file("test.ts", "const x = 1;");

        let content = mock.read_file("test.ts").unwrap();
        assert_eq!(content, "const x = 1;");
        assert_eq!(mock.read_count(), 1);
        assert!(mock.file_exists("test.ts"));
    }

    #[test]
    fn test_mock_cache() {
        let mock = MockCache::new();
        let _ = mock.put("key1", "value1");

        let value = mock.get("key1");
        assert_eq!(value, Some("value1".to_string()));
        assert_eq!(mock.hit_count(), 1);

        let missing = mock.get("key2");
        assert_eq!(missing, None);
        assert_eq!(mock.miss_count(), 1);
    }

    #[test]
    fn test_london_mock_environment() {
        let env = MockFactory::london_environment();
        env.setup_realistic_scenario();

        assert!(env.file_system.file_exists("src/component.tsx"));
        assert_eq!(env.cache.get("analysis:src/component.tsx"), Some("cached_result".to_string()));
    }
}
