//! # KV Storage for Rules - Ultra-Fast Rule Access
//!
//! Uses assemblage_kv for WASM-safe high-performance rule storage.
//! Rules are distributed across KV storage for maximum speed and scalability.

use crate::error::{Error, Result};
use assemblage_kv::{storage, KvStore};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;

/// High-performance KV storage for rules using assemblage_kv
pub struct RuleKV {
    /// KV store backend
    store: Arc<RwLock<Option<KvStore>>>,
    /// Hot cache for frequently accessed rules
    hot_cache: Arc<RwLock<HashMap<String, StoredRule>>>,
    /// Rule metadata index
    metadata_index: Arc<RwLock<HashMap<String, RuleMetadata>>>,
    /// Performance metrics
    metrics: Arc<RwLock<StorageMetrics>>,
}

// KV storage slots for different rule types
const RULES_SLOT: u8 = 10;     // Rule storage slot
const METADATA_SLOT: u8 = 11;  // Rule metadata slot
const STATS_SLOT: u8 = 12;     // Rule statistics slot

/// Rule stored in KV format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRule {
    /// Rule ID
    pub id: String,
    /// Rule bytecode/implementation
    pub bytecode: Vec<u8>,
    /// Rule metadata
    pub metadata: RuleMetadata,
    /// Execution statistics
    pub stats: ExecutionStats,
    /// Last accessed timestamp
    pub last_accessed: u64,
    /// Access frequency
    pub access_count: u64,
}

/// Rule metadata for fast lookups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule category
    pub category: String,
    /// Default severity
    pub severity: String,
    /// Execution cost estimate
    pub cost: u32,
    /// Rule dependencies
    pub dependencies: Vec<String>,
    /// Whether AI-enhanced
    pub ai_enhanced: bool,
    /// Whether supports autofix
    pub autofix: bool,
    /// Rule tags for filtering
    pub tags: Vec<String>,
}

/// Execution statistics for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    /// Average execution time in nanoseconds
    pub avg_execution_ns: u64,
    /// Total executions
    pub total_executions: u64,
    /// Success rate (0-1000, fixed point)
    pub success_rate: u32,
    /// Cache hit rate (0-1000, fixed point)
    pub cache_hit_rate: u32,
    /// Memory usage estimate
    pub avg_memory_kb: u32,
}

/// Storage performance metrics
#[derive(Debug, Clone)]
pub struct StorageMetrics {
    /// Total KV operations
    pub total_operations: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Average lookup time
    pub avg_lookup_ns: u64,
    /// Storage size
    pub storage_size_kb: u64,
}

/// KV storage operations trait
#[async_trait]
pub trait RuleStorage: Send + Sync {
    /// Store a rule in KV
    async fn store_rule(&self, rule: &StoredRule) -> Result<()>;

    /// Load a rule from KV
    async fn load_rule(&self, rule_id: &str) -> Result<Option<StoredRule>>;

    /// List all rules in category
    async fn list_rules_by_category(&self, category: &str) -> Result<Vec<String>>;

    /// Get rule metadata without loading full rule
    async fn get_metadata(&self, rule_id: &str) -> Result<Option<RuleMetadata>>;

    /// Batch load multiple rules
    async fn batch_load_rules(&self, rule_ids: &[String]) -> Result<Vec<StoredRule>>;

    /// Update rule statistics
    async fn update_stats(&self, rule_id: &str, stats: &ExecutionStats) -> Result<()>;

    /// Get storage metrics
    async fn get_metrics(&self) -> StorageMetrics;
}

impl RuleKV {
    /// Create new KV storage for rules using assemblage_kv
    pub async fn new() -> Result<Self> {
        let mut rule_kv = Self {
            store: Arc::new(RwLock::new(None)),
            hot_cache: Arc::new(RwLock::new(HashMap::new())),
            metadata_index: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(StorageMetrics::new())),
        };

        // Try to initialize assemblage_kv store
        let _ = rule_kv.try_init_store().await;

        Ok(rule_kv)
    }

    /// Try to initialize the assemblage_kv store
    async fn try_init_store(&self) -> Result<()> {
        let storage_backend = storage::open(".moon/moonshine/rules")
            .await
            .map_err(|e| Error::Storage(format!("Failed to open rule storage: {:?}", e)))?;

        let kv_store = KvStore::open(storage_backend)
            .await
            .map_err(|e| Error::Storage(format!("Failed to open rule KV store: {:?}", e)))?;

        let mut store = self.store.write().unwrap();
        *store = Some(kv_store);

        Ok(())
    }

    /// Store rule with optimizations
    pub async fn store_rule_optimized(&self, rule: &StoredRule) -> Result<()> {
        let start = std::time::Instant::now();

        // Serialize rule
        let serialized = bincode::serialize(rule)
            .map_err(|e| Error::Storage(format!("Failed to serialize rule: {}", e)))?;

        // Store in assemblage_kv if available
        if let Some(store) = self.store.read().unwrap().as_ref() {
            let mut current = store.current().await;
            current.insert(RULES_SLOT, &rule.id, &serialized)
                .map_err(|e| Error::Storage(format!("Failed to insert rule: {:?}", e)))?;
            current.commit().await
                .map_err(|e| Error::Storage(format!("Failed to commit rule: {:?}", e)))?;
        }

        // Update hot cache if frequently accessed
        if rule.access_count > 100 {
            let mut cache = self.hot_cache.write().unwrap();
            cache.insert(rule.id.clone(), rule.clone());
        }

        // Update metadata index
        {
            let mut index = self.metadata_index.write().unwrap();
            index.insert(rule.id.clone(), rule.metadata.clone());
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.total_operations += 1;
            metrics.avg_lookup_ns = (metrics.avg_lookup_ns + start.elapsed().as_nanos() as u64) / 2;
        }

        Ok(())
    }

    /// Load rule with caching
    pub async fn load_rule_cached(&self, rule_id: &str) -> Result<Option<StoredRule>> {
        let start = std::time::Instant::now();

        // Check hot cache first
        {
            let cache = self.hot_cache.read().unwrap();
            if let Some(rule) = cache.get(rule_id) {
                // Update metrics for cache hit
                let mut metrics = self.metrics.write().unwrap();
                metrics.cache_hits += 1;
                metrics.total_operations += 1;
                return Ok(Some(rule.clone()));
            }
        }

        // Load from assemblage_kv storage
        let data = if let Some(store) = self.store.read().unwrap().as_ref() {
            let current = store.current().await;
            current.get::<&str, Vec<u8>>(RULES_SLOT, rule_id).await
                .map_err(|e| Error::Storage(format!("Failed to load rule: {:?}", e)))?
        } else {
            None
        };

        let rule = if let Some(data) = data {
            let mut rule: StoredRule = bincode::deserialize(&data)
                .map_err(|e| Error::Serialization(format!("Failed to deserialize rule: {}", e)))?;

            // Update access statistics
            rule.last_accessed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            rule.access_count += 1;

            // Store back with updated stats
            self.store_rule_optimized(&rule).await?;

            Some(rule)
        } else {
            None
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            if rule.is_some() {
                metrics.cache_misses += 1;
            }
            metrics.total_operations += 1;
            metrics.avg_lookup_ns = (metrics.avg_lookup_ns + start.elapsed().as_nanos() as u64) / 2;
        }

        Ok(rule)
    }

    /// Batch load rules for parallel execution
    pub async fn batch_load_optimized(&self, rule_ids: &[String]) -> Result<Vec<StoredRule>> {
        let mut rules = Vec::with_capacity(rule_ids.len());
        let mut cache_hits = 0;
        let mut cache_misses = 0;

        // First pass: check hot cache
        let mut remaining_ids = Vec::new();
        {
            let cache = self.hot_cache.read().unwrap();
            for rule_id in rule_ids {
                if let Some(rule) = cache.get(rule_id) {
                    rules.push(rule.clone());
                    cache_hits += 1;
                } else {
                    remaining_ids.push(rule_id.clone());
                }
            }
        }

        // Second pass: load from KV in parallel
        let kv_futures: Vec<_> = remaining_ids.iter().map(|id| {
            let id = id.clone();
            let storage = self.storage.clone();
            async move {
                let key = format!("rule:{}", id);
                storage.get(&key)
            }
        }).collect();

        // Execute all KV loads in parallel
        let kv_results = futures::future::join_all(kv_futures).await;

        for (i, result) in kv_results.into_iter().enumerate() {
            match result {
                Ok(Some(data)) => {
                    let rule: StoredRule = bincode::deserialize(&data)
                        .map_err(|e| Error::Serialization(format!("Failed to deserialize rule: {}", e)))?;
                    rules.push(rule);
                    cache_misses += 1;
                }
                Ok(None) => {
                    // Rule not found - continue
                    cache_misses += 1;
                }
                Err(e) => {
                    return Err(Error::Storage(format!("Batch load failed: {}", e)));
                }
            }
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.cache_hits += cache_hits;
            metrics.cache_misses += cache_misses;
            metrics.total_operations += rule_ids.len() as u64;
        }

        Ok(rules)
    }

    /// Get rules by category with filtering
    pub async fn get_rules_by_category(&self, category: &str, filters: &RuleFilters) -> Result<Vec<RuleMetadata>> {
        let mut matching_rules = Vec::new();

        // Scan metadata index
        let index = self.metadata_index.read().unwrap();
        for (rule_id, metadata) in index.iter() {
            if metadata.category == category && self.matches_filters(metadata, filters) {
                matching_rules.push(metadata.clone());
            }
        }

        // Sort by execution cost for optimization
        matching_rules.sort_by_key(|r| r.cost);

        Ok(matching_rules)
    }

    /// Get hot rules for prefetching
    pub async fn get_hot_rules(&self, limit: usize) -> Vec<String> {
        let cache = self.hot_cache.read().unwrap();
        let mut hot_rules: Vec<_> = cache.iter().collect();

        // Sort by access count
        hot_rules.sort_by_key(|(_, rule)| std::cmp::Reverse(rule.access_count));

        hot_rules.into_iter()
            .take(limit)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Optimize storage based on usage patterns
    pub async fn optimize_storage(&self) -> Result<()> {
        // Analyze access patterns
        let hot_rules = self.get_hot_rules(100).await;

        // Preload hot rules into cache
        for rule_id in hot_rules {
            if let Ok(Some(rule)) = self.load_rule_cached(&rule_id).await {
                let mut cache = self.hot_cache.write().unwrap();
                cache.insert(rule_id, rule);
            }
        }

        // Clean up cold entries from cache
        let mut cache = self.hot_cache.write().unwrap();
        cache.retain(|_, rule| rule.access_count > 10);

        Ok(())
    }

    fn matches_filters(&self, metadata: &RuleMetadata, filters: &RuleFilters) -> bool {
        if let Some(ref tags) = filters.tags {
            if !tags.iter().any(|tag| metadata.tags.contains(tag)) {
                return false;
            }
        }

        if let Some(max_cost) = filters.max_cost {
            if metadata.cost > max_cost {
                return false;
            }
        }

        if let Some(ai_enhanced) = filters.ai_enhanced {
            if metadata.ai_enhanced != ai_enhanced {
                return false;
            }
        }

        true
    }
}

/// Rule filtering options
#[derive(Debug, Clone)]
pub struct RuleFilters {
    /// Filter by tags
    pub tags: Option<Vec<String>>,
    /// Maximum execution cost
    pub max_cost: Option<u32>,
    /// Whether to include AI-enhanced rules
    pub ai_enhanced: Option<bool>,
    /// Whether to include autofix rules
    pub autofix: Option<bool>,
}

#[async_trait]
impl RuleStorage for RuleKV {
    async fn store_rule(&self, rule: &StoredRule) -> Result<()> {
        self.store_rule_optimized(rule).await
    }

    async fn load_rule(&self, rule_id: &str) -> Result<Option<StoredRule>> {
        self.load_rule_cached(rule_id).await
    }

    async fn list_rules_by_category(&self, category: &str) -> Result<Vec<String>> {
        let filters = RuleFilters {
            tags: None,
            max_cost: None,
            ai_enhanced: None,
            autofix: None,
        };

        let metadata = self.get_rules_by_category(category, &filters).await?;
        Ok(metadata.into_iter().map(|m| m.name).collect())
    }

    async fn get_metadata(&self, rule_id: &str) -> Result<Option<RuleMetadata>> {
        let index = self.metadata_index.read().unwrap();
        Ok(index.get(rule_id).cloned())
    }

    async fn batch_load_rules(&self, rule_ids: &[String]) -> Result<Vec<StoredRule>> {
        self.batch_load_optimized(rule_ids).await
    }

    async fn update_stats(&self, rule_id: &str, stats: &ExecutionStats) -> Result<()> {
        if let Some(mut rule) = self.load_rule_cached(rule_id).await? {
            rule.stats = stats.clone();
            self.store_rule_optimized(&rule).await?;
        }
        Ok(())
    }

    async fn get_metrics(&self) -> StorageMetrics {
        self.metrics.read().unwrap().clone()
    }
}

impl StorageMetrics {
    fn new() -> Self {
        Self {
            total_operations: 0,
            cache_hits: 0,
            cache_misses: 0,
            avg_lookup_ns: 0,
            storage_size_kb: 0,
        }
    }

    /// Calculate cache hit rate as percentage
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / self.total_operations as f64) * 100.0
        }
    }
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            avg_execution_ns: 0,
            total_executions: 0,
            success_rate: 1000, // 100.0% in fixed point
            cache_hit_rate: 0,
            avg_memory_kb: 0,
        }
    }
}

impl Default for RuleFilters {
    fn default() -> Self {
        Self {
            tags: None,
            max_cost: None,
            ai_enhanced: None,
            autofix: None,
        }
    }
}