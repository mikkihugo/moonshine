/*!
 * Database Schema for Documentation Templates
 *
 * Database-friendly structures for storing and querying documentation templates,
 * versions, and metadata. Designed for SQL databases, NoSQL stores, and in-memory
 * caching with efficient indexing and querying capabilities.
 */

use crate::templates::{tsdoc::TSDocVersion, rustdoc::RustDocVersion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database-friendly documentation template record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTemplateRecord {
    /// Primary key - template identifier
    pub id: String,
    /// Template type (tsdoc, rustdoc, etc.)
    pub template_type: DocTemplateType,
    /// Language target (typescript, rust, etc.)
    pub language: String,
    /// Template version information
    pub version: String,
    /// Template content with field markers
    pub content: String,
    /// Sed-editable version with field markers
    pub sed_content: String,
    /// Configuration template
    pub config_template: String,
    /// Moon task configuration
    pub moon_tasks: String,
    /// Template metadata
    pub metadata: DocTemplateMetadata,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
    /// Version history
    pub version_history: Vec<DocVersionEntry>,
    /// Template tags for categorization
    pub tags: Vec<String>,
    /// Search keywords
    pub keywords: Vec<String>,
}

/// Documentation template types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocTemplateType {
    TSDoc,
    RustDoc,
    JSDoc,
    PyDoc,
    GoDoc,
    Custom(String),
}

/// Template metadata for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTemplateMetadata {
    /// Change count for versioning
    pub change_count: u32,
    /// Template content checksum
    pub checksum: String,
    /// Git commit hash if available
    pub commit_hash: Option<String>,
    /// File patterns this template applies to
    pub file_patterns: Vec<String>,
    /// Exclude patterns
    pub exclude_patterns: Vec<String>,
    /// Target coverage percentage
    pub target_coverage: u8,
    /// AI model used for generation
    pub ai_model: String,
    /// COPRO optimization enabled
    pub copro_enabled: bool,
    /// Vector similarity enabled
    pub vector_similarity_enabled: bool,
    /// Embedding model
    pub embedding_model: String,
    /// Usage statistics
    pub usage_stats: UsageStatistics,
}

/// Version history entry for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocVersionEntry {
    /// Version string (e.g., "1.0.35")
    pub version: String,
    /// Changes made in this version
    pub changes: Vec<String>,
    /// Timestamp of version creation
    pub timestamp: String,
    /// User or system that made the change
    pub changed_by: Option<String>,
    /// Commit hash for this version
    pub commit_hash: Option<String>,
    /// Version type (major, minor, patch, hotfix)
    pub version_type: VersionType,
}

/// Version change types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionType {
    Major,    // Breaking changes
    Minor,    // New features
    Patch,    // Bug fixes
    Hotfix,   // Critical fixes
}

/// Usage statistics for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    /// Total times template was used
    pub usage_count: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f32,
    /// Average execution time in milliseconds
    pub avg_execution_time: u32,
    /// Last used timestamp
    pub last_used: Option<String>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics for database analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average tokens per request
    pub avg_tokens: u32,
    /// Average response size in bytes
    pub avg_response_size: u32,
    /// Error count
    pub error_count: u32,
    /// Timeout count
    pub timeout_count: u32,
    /// Cache hit rate
    pub cache_hit_rate: f32,
}

/// Sed field definition for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SedFieldDefinition {
    /// Field name (e.g., "TASK", "OUTPUT_REQUIREMENTS")
    pub field_name: String,
    /// Field type (text, multiline, list, etc.)
    pub field_type: SedFieldType,
    /// Field description
    pub description: String,
    /// Default value
    pub default_value: String,
    /// Validation rules
    pub validation_rules: Vec<String>,
    /// Example values
    pub examples: Vec<String>,
    /// Field dependencies
    pub dependencies: Vec<String>,
}

/// Types of sed-editable fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SedFieldType {
    Text,           // Single line text
    MultiLine,      // Multi-line text
    List,           // List of items
    KeyValue,       // Key-value pairs
    Code,           // Code blocks
    Config,         // Configuration values
}

/// Database query interface for documentation templates
pub trait DocTemplateDatabase {
    /// Store a documentation template
    async fn store_template(&self, template: DocTemplateRecord) -> Result<String, String>;

    /// Retrieve template by ID
    async fn get_template(&self, id: &str) -> Result<Option<DocTemplateRecord>, String>;

    /// List templates by type
    async fn list_templates_by_type(&self, template_type: DocTemplateType) -> Result<Vec<DocTemplateRecord>, String>;

    /// Search templates by keywords
    async fn search_templates(&self, keywords: &[String]) -> Result<Vec<DocTemplateRecord>, String>;

    /// Get template version history
    async fn get_version_history(&self, template_id: &str) -> Result<Vec<DocVersionEntry>, String>;

    /// Update template version
    async fn update_template_version(&self, template_id: &str, new_version: DocVersionEntry) -> Result<(), String>;

    /// Get usage statistics
    async fn get_usage_stats(&self, template_id: &str) -> Result<UsageStatistics, String>;

    /// Update usage statistics
    async fn update_usage_stats(&self, template_id: &str, stats: UsageStatistics) -> Result<(), String>;

    /// Query templates by file pattern
    async fn find_templates_for_file(&self, file_path: &str) -> Result<Vec<DocTemplateRecord>, String>;

    /// Get templates with performance above threshold
    async fn get_high_performance_templates(&self, min_success_rate: f32) -> Result<Vec<DocTemplateRecord>, String>;
}

/// SQL schema for documentation templates
pub const DOC_TEMPLATES_SQL_SCHEMA: &str = r#"
-- Documentation Templates Schema
CREATE TABLE IF NOT EXISTS doc_templates (
    id VARCHAR(255) PRIMARY KEY,
    template_type VARCHAR(50) NOT NULL,
    language VARCHAR(50) NOT NULL,
    version VARCHAR(20) NOT NULL,
    content TEXT NOT NULL,
    sed_content TEXT NOT NULL,
    config_template TEXT,
    moon_tasks TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    -- Metadata columns
    change_count INTEGER DEFAULT 1,
    checksum VARCHAR(64),
    commit_hash VARCHAR(40),
    target_coverage TINYINT DEFAULT 90,
    ai_model VARCHAR(100),
    copro_enabled BOOLEAN DEFAULT true,
    vector_similarity_enabled BOOLEAN DEFAULT true,
    embedding_model VARCHAR(100),

    -- Usage statistics
    usage_count BIGINT DEFAULT 0,
    success_rate FLOAT DEFAULT 1.0,
    avg_execution_time INTEGER DEFAULT 0,
    last_used TIMESTAMP NULL,

    -- Performance metrics
    avg_tokens INTEGER DEFAULT 0,
    avg_response_size INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    timeout_count INTEGER DEFAULT 0,
    cache_hit_rate FLOAT DEFAULT 0.0,

    -- Indexes
    INDEX idx_template_type (template_type),
    INDEX idx_language (language),
    INDEX idx_version (version),
    INDEX idx_success_rate (success_rate),
    INDEX idx_usage_count (usage_count),
    INDEX idx_updated_at (updated_at)
);

-- Template file patterns
CREATE TABLE IF NOT EXISTS template_file_patterns (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    template_id VARCHAR(255) NOT NULL,
    pattern VARCHAR(255) NOT NULL,
    pattern_type ENUM('include', 'exclude') NOT NULL,

    FOREIGN KEY (template_id) REFERENCES doc_templates(id) ON DELETE CASCADE,
    INDEX idx_template_pattern (template_id, pattern_type)
);

-- Template version history
CREATE TABLE IF NOT EXISTS template_version_history (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    template_id VARCHAR(255) NOT NULL,
    version VARCHAR(20) NOT NULL,
    changes JSON,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_by VARCHAR(255),
    commit_hash VARCHAR(40),
    version_type ENUM('major', 'minor', 'patch', 'hotfix') DEFAULT 'patch',

    FOREIGN KEY (template_id) REFERENCES doc_templates(id) ON DELETE CASCADE,
    INDEX idx_template_version (template_id, version),
    INDEX idx_version_timestamp (timestamp)
);

-- Template tags
CREATE TABLE IF NOT EXISTS template_tags (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    template_id VARCHAR(255) NOT NULL,
    tag VARCHAR(100) NOT NULL,

    FOREIGN KEY (template_id) REFERENCES doc_templates(id) ON DELETE CASCADE,
    UNIQUE KEY unique_template_tag (template_id, tag),
    INDEX idx_tag (tag)
);

-- Template keywords for search
CREATE TABLE IF NOT EXISTS template_keywords (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    template_id VARCHAR(255) NOT NULL,
    keyword VARCHAR(100) NOT NULL,

    FOREIGN KEY (template_id) REFERENCES doc_templates(id) ON DELETE CASCADE,
    UNIQUE KEY unique_template_keyword (template_id, keyword),
    INDEX idx_keyword (keyword)
);

-- Sed field definitions
CREATE TABLE IF NOT EXISTS sed_field_definitions (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    template_id VARCHAR(255) NOT NULL,
    field_name VARCHAR(100) NOT NULL,
    field_type ENUM('text', 'multiline', 'list', 'keyvalue', 'code', 'config') NOT NULL,
    description TEXT,
    default_value TEXT,
    validation_rules JSON,
    examples JSON,
    dependencies JSON,

    FOREIGN KEY (template_id) REFERENCES doc_templates(id) ON DELETE CASCADE,
    UNIQUE KEY unique_template_field (template_id, field_name),
    INDEX idx_field_name (field_name)
);
"#;

/// MongoDB schema for documentation templates
pub const DOC_TEMPLATES_MONGODB_SCHEMA: &str = r#"
// MongoDB Collections for Documentation Templates

// Main templates collection
db.createCollection("doc_templates", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["id", "template_type", "language", "version", "content"],
      properties: {
        _id: { bsonType: "string" },
        id: { bsonType: "string" },
        template_type: {
          enum: ["TSDoc", "RustDoc", "JSDoc", "PyDoc", "GoDoc", "Custom"]
        },
        language: { bsonType: "string" },
        version: { bsonType: "string" },
        content: { bsonType: "string" },
        sed_content: { bsonType: "string" },
        config_template: { bsonType: "string" },
        moon_tasks: { bsonType: "string" },
        metadata: {
          bsonType: "object",
          properties: {
            change_count: { bsonType: "int" },
            checksum: { bsonType: "string" },
            commit_hash: { bsonType: ["string", "null"] },
            file_patterns: { bsonType: "array" },
            exclude_patterns: { bsonType: "array" },
            target_coverage: { bsonType: "int" },
            ai_model: { bsonType: "string" },
            usage_stats: {
              bsonType: "object",
              properties: {
                usage_count: { bsonType: "long" },
                success_rate: { bsonType: "double" },
                avg_execution_time: { bsonType: "int" }
              }
            }
          }
        },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" },
        version_history: { bsonType: "array" },
        tags: { bsonType: "array" },
        keywords: { bsonType: "array" }
      }
    }
  }
});

// Create indexes for efficient querying
db.doc_templates.createIndex({ "template_type": 1 });
db.doc_templates.createIndex({ "language": 1 });
db.doc_templates.createIndex({ "version": 1 });
db.doc_templates.createIndex({ "metadata.usage_stats.success_rate": -1 });
db.doc_templates.createIndex({ "metadata.usage_stats.usage_count": -1 });
db.doc_templates.createIndex({ "updated_at": -1 });
db.doc_templates.createIndex({ "tags": 1 });
db.doc_templates.createIndex({ "keywords": "text" });

// Compound indexes for complex queries
db.doc_templates.createIndex({
  "template_type": 1,
  "language": 1,
  "metadata.usage_stats.success_rate": -1
});
"#;

/// Convert TSDoc version to database record
impl From<&super::tsdoc::TSDocConfig> for DocTemplateRecord {
    fn from(config: &super::tsdoc::TSDocConfig) -> Self {
        let tsdoc_template = super::tsdoc::get_tsdoc_prompt_template();
        let sed_template = super::tsdoc::get_tsdoc_sed_template();

        DocTemplateRecord {
            id: format!("tsdoc-{}", config.version.version),
            template_type: DocTemplateType::TSDoc,
            language: "typescript".to_string(),
            version: config.version.version.clone(),
            content: tsdoc_template.template,
            sed_content: sed_template,
            config_template: super::tsdoc::TSDOC_CONFIG_TEMPLATE.to_string(),
            moon_tasks: super::tsdoc::TSDOC_MOON_TASKS.to_string(),
            metadata: DocTemplateMetadata {
                change_count: config.version.change_count,
                checksum: config.version.template_checksum.clone(),
                commit_hash: config.version.commit_hash.clone(),
                file_patterns: config.include_patterns.clone(),
                exclude_patterns: config.exclude_patterns.clone(),
                target_coverage: config.target_coverage,
                ai_model: config.claude_model.clone(),
                copro_enabled: config.use_copro_optimization,
                vector_similarity_enabled: config.use_vector_similarity,
                embedding_model: config.embedding_model.clone(),
                usage_stats: UsageStatistics::default(),
            },
            created_at: config.version.updated_at.clone(),
            updated_at: config.version.updated_at.clone(),
            version_history: Vec::new(),
            tags: vec!["typescript".to_string(), "documentation".to_string(), "ai-generated".to_string()],
            keywords: vec!["tsdoc".to_string(), "typescript".to_string(), "documentation".to_string(), "claude".to_string()],
        }
    }
}

/// Convert RustDoc version to database record
impl From<&super::rustdoc::RustDocConfig> for DocTemplateRecord {
    fn from(config: &super::rustdoc::RustDocConfig) -> Self {
        let rustdoc_template = super::rustdoc::get_rustdoc_prompt_template();
        let sed_template = super::rustdoc::get_rustdoc_sed_template();

        DocTemplateRecord {
            id: format!("rustdoc-{}", config.version.version),
            template_type: DocTemplateType::RustDoc,
            language: "rust".to_string(),
            version: config.version.version.clone(),
            content: rustdoc_template.template,
            sed_content: sed_template,
            config_template: super::rustdoc::RUSTDOC_CONFIG_TEMPLATE.to_string(),
            moon_tasks: super::rustdoc::RUSTDOC_MOON_TASKS.to_string(),
            metadata: DocTemplateMetadata {
                change_count: config.version.change_count,
                checksum: config.version.template_checksum.clone(),
                commit_hash: config.version.commit_hash.clone(),
                file_patterns: config.include_patterns.clone(),
                exclude_patterns: config.exclude_patterns.clone(),
                target_coverage: config.target_coverage,
                ai_model: config.claude_model.clone(),
                copro_enabled: config.use_copro_optimization,
                vector_similarity_enabled: config.use_vector_similarity,
                embedding_model: config.embedding_model.clone(),
                usage_stats: UsageStatistics::default(),
            },
            created_at: config.version.updated_at.clone(),
            updated_at: config.version.updated_at.clone(),
            version_history: Vec::new(),
            tags: vec!["rust".to_string(), "documentation".to_string(), "ai-generated".to_string()],
            keywords: vec!["rustdoc".to_string(), "rust".to_string(), "documentation".to_string(), "claude".to_string()],
        }
    }
}

impl Default for UsageStatistics {
    fn default() -> Self {
        Self {
            usage_count: 0,
            success_rate: 1.0,
            avg_execution_time: 0,
            last_used: None,
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_tokens: 0,
            avg_response_size: 0,
            error_count: 0,
            timeout_count: 0,
            cache_hit_rate: 0.0,
        }
    }
}

/// Helper functions for database operations
impl DocTemplateRecord {
    /// Create a new template record
    pub fn new(
        id: String,
        template_type: DocTemplateType,
        language: String,
        version: String,
        content: String,
    ) -> Self {
        Self {
            id,
            template_type,
            language,
            version,
            content: content.clone(),
            sed_content: String::new(),
            config_template: String::new(),
            moon_tasks: String::new(),
            metadata: DocTemplateMetadata {
                change_count: 1,
                checksum: format!("{:x}", content.len()), // Simple checksum
                commit_hash: None,
                file_patterns: Vec::new(),
                exclude_patterns: Vec::new(),
                target_coverage: 90,
                ai_model: "claude-3-5-sonnet-20241022".to_string(),
                copro_enabled: true,
                vector_similarity_enabled: true,
                embedding_model: "text-embedding-3-small".to_string(),
                usage_stats: UsageStatistics::default(),
            },
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            version_history: Vec::new(),
            tags: Vec::new(),
            keywords: Vec::new(),
        }
    }

    /// Add a new version to history
    pub fn add_version(&mut self, changes: Vec<String>, version_type: VersionType) {
        let version_entry = DocVersionEntry {
            version: self.version.clone(),
            changes,
            timestamp: chrono::Utc::now().to_rfc3339(),
            changed_by: None,
            commit_hash: self.metadata.commit_hash.clone(),
            version_type,
        };

        self.version_history.push(version_entry);
        self.metadata.change_count += 1;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Update usage statistics
    pub fn record_usage(&mut self, execution_time: u32, success: bool) {
        self.metadata.usage_stats.usage_count += 1;
        self.metadata.usage_stats.last_used = Some(chrono::Utc::now().to_rfc3339());

        // Update average execution time
        let total_time = self.metadata.usage_stats.avg_execution_time as u64 * (self.metadata.usage_stats.usage_count - 1);
        self.metadata.usage_stats.avg_execution_time = ((total_time + execution_time as u64) / self.metadata.usage_stats.usage_count) as u32;

        // Update success rate
        let total_successes = (self.metadata.usage_stats.success_rate * (self.metadata.usage_stats.usage_count - 1) as f32) + if success { 1.0 } else { 0.0 };
        self.metadata.usage_stats.success_rate = total_successes / self.metadata.usage_stats.usage_count as f32;

        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Check if template matches file pattern
    pub fn matches_file(&self, file_path: &str) -> bool {
        // Check include patterns
        let includes = self.metadata.file_patterns.iter().any(|pattern| {
            glob::Pattern::new(pattern).map_or(false, |p| p.matches(file_path))
        });

        // Check exclude patterns
        let excludes = self.metadata.exclude_patterns.iter().any(|pattern| {
            glob::Pattern::new(pattern).map_or(false, |p| p.matches(file_path))
        });

        includes && !excludes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_template_record_creation() {
        let record = DocTemplateRecord::new(
            "test-template".to_string(),
            DocTemplateType::TSDoc,
            "typescript".to_string(),
            "1.0.1".to_string(),
            "template content".to_string(),
        );

        assert_eq!(record.id, "test-template");
        assert_eq!(record.template_type, DocTemplateType::TSDoc);
        assert_eq!(record.language, "typescript");
        assert_eq!(record.version, "1.0.1");
        assert_eq!(record.metadata.change_count, 1);
        assert_eq!(record.metadata.usage_stats.usage_count, 0);
    }

    #[test]
    fn test_usage_recording() {
        let mut record = DocTemplateRecord::new(
            "test".to_string(),
            DocTemplateType::RustDoc,
            "rust".to_string(),
            "1.0.0".to_string(),
            "content".to_string(),
        );

        record.record_usage(1000, true);
        assert_eq!(record.metadata.usage_stats.usage_count, 1);
        assert_eq!(record.metadata.usage_stats.avg_execution_time, 1000);
        assert_eq!(record.metadata.usage_stats.success_rate, 1.0);

        record.record_usage(2000, false);
        assert_eq!(record.metadata.usage_stats.usage_count, 2);
        assert_eq!(record.metadata.usage_stats.avg_execution_time, 1500);
        assert_eq!(record.metadata.usage_stats.success_rate, 0.5);
    }

    #[test]
    fn test_file_pattern_matching() {
        let mut record = DocTemplateRecord::new(
            "test".to_string(),
            DocTemplateType::TSDoc,
            "typescript".to_string(),
            "1.0.0".to_string(),
            "content".to_string(),
        );

        record.metadata.file_patterns = vec!["**/*.ts".to_string(), "**/*.tsx".to_string()];
        record.metadata.exclude_patterns = vec!["**/*.test.ts".to_string()];

        assert!(record.matches_file("src/index.ts"));
        assert!(record.matches_file("components/Button.tsx"));
        assert!(!record.matches_file("src/index.test.ts"));
        assert!(!record.matches_file("README.md"));
    }

    #[test]
    fn test_version_history() {
        let mut record = DocTemplateRecord::new(
            "test".to_string(),
            DocTemplateType::RustDoc,
            "rust".to_string(),
            "1.0.0".to_string(),
            "content".to_string(),
        );

        record.add_version(
            vec!["Added new feature".to_string(), "Fixed bug".to_string()],
            VersionType::Minor,
        );

        assert_eq!(record.version_history.len(), 1);
        assert_eq!(record.metadata.change_count, 2);
        assert_eq!(record.version_history[0].version_type, VersionType::Minor);
        assert_eq!(record.version_history[0].changes.len(), 2);
    }
}