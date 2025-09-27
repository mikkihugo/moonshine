//! Core storage functionality for Moon Shine
//!
//! Self-documenting storage abstraction with WASM-compatible implementation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage interface for persisting analysis data
#[derive(Debug, Clone)]
pub struct MoonShineStorage {
    pub config: StorageConfig,
    pub in_memory_cache: HashMap<String, String>,
    pub persistent_data: HashMap<String, StorageEntry>,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub enable_persistence: bool,
    pub cache_size_limit: usize,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

/// Storage entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: String,
    pub value: String,
    pub timestamp: u64,
    pub access_count: u64,
    pub size_bytes: usize,
}

impl MoonShineStorage {
    /// Create new storage with default configuration
    pub fn new() -> Self {
        Self {
            config: StorageConfig::default(),
            in_memory_cache: HashMap::new(),
            persistent_data: HashMap::new(),
        }
    }

    /// Store data with key
    pub fn store(&mut self, key: &str, value: &str) -> Result<(), String> {
        let entry = StorageEntry {
            key: key.to_string(),
            value: value.to_string(),
            timestamp: self.current_timestamp(),
            access_count: 0,
            size_bytes: value.len(),
        };

        // Store in cache
        self.in_memory_cache.insert(key.to_string(), value.to_string());

        // Store persistently if enabled
        if self.config.enable_persistence {
            self.persistent_data.insert(key.to_string(), entry);
        }

        Ok(())
    }

    /// Retrieve data by key
    pub fn retrieve(&mut self, key: &str) -> Option<String> {
        // Check cache first
        if let Some(value) = self.in_memory_cache.get(key) {
            return Some(value.clone());
        }

        // Check persistent storage
        if let Some(entry) = self.persistent_data.get_mut(key) {
            entry.access_count += 1;
            // Update cache
            self.in_memory_cache.insert(key.to_string(), entry.value.clone());
            return Some(entry.value.clone());
        }

        None
    }

    /// Delete data by key
    pub fn delete(&mut self, key: &str) -> bool {
        let cache_removed = self.in_memory_cache.remove(key).is_some();
        let persistent_removed = self.persistent_data.remove(key).is_some();
        cache_removed || persistent_removed
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.in_memory_cache.clear();
        self.persistent_data.clear();
    }

    /// Get storage statistics
    pub fn get_stats(&self) -> StorageStats {
        StorageStats {
            cache_entries: self.in_memory_cache.len(),
            persistent_entries: self.persistent_data.len(),
            total_size_bytes: self.calculate_total_size(),
            cache_hit_ratio: 0.0, // Would be calculated from access patterns
        }
    }

    /// Check if key exists
    pub fn exists(&self, key: &str) -> bool {
        self.in_memory_cache.contains_key(key) || self.persistent_data.contains_key(key)
    }

    /// List all keys
    pub fn list_keys(&self) -> Vec<String> {
        let mut keys = self.in_memory_cache.keys().cloned().collect::<std::collections::HashSet<_>>();
        keys.extend(self.persistent_data.keys().cloned());
        keys.into_iter().collect()
    }

    /// Get current timestamp (simplified)
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    /// Calculate total storage size
    fn calculate_total_size(&self) -> usize {
        self.in_memory_cache.values().map(|v| v.len()).sum::<usize>() + self.persistent_data.values().map(|e| e.size_bytes).sum::<usize>()
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub cache_entries: usize,
    pub persistent_entries: usize,
    pub total_size_bytes: usize,
    pub cache_hit_ratio: f32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            enable_persistence: true,
            cache_size_limit: 1024 * 1024, // 1MB
            compression_enabled: false,
            encryption_enabled: false,
        }
    }
}

impl Default for MoonShineStorage {
    fn default() -> Self {
        Self::new()
    }
}
