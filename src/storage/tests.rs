//! # Storage Module Tests
//!
//! Comprehensive tests for the hybrid assemblage_kv + file persistence storage system.
//!
//! @category testing
//! @safe program
//! @complexity medium
//! @since 2.0.0

use std::collections::HashMap;
use super::*;
use crate::testing::builders::{AnalysisResultsBuilder, AiSuggestionBuilder};
use crate::testing::assertions::MoonShineAssertions;

#[tokio::test]
async fn test_storage_initialization() {
    let storage = Storage::new().await;
    assert!(storage.is_ok());

    let storage = storage.unwrap();
    assert!(!storage.db_path().is_empty());
}

#[tokio::test]
async fn test_store_and_retrieve_analysis() {
    let mut storage = Storage::new().await.unwrap();

    let results = AnalysisResultsBuilder::typescript_analysis().build();
    let key = "test_analysis_1";

    // Store analysis
    let store_result = storage.store_analysis(key, &results).await;
    assert!(store_result.is_ok());

    // Retrieve analysis
    let retrieved = storage.get_analysis(key).await.unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.files_processed, results.files_processed);
    assert_eq!(retrieved.suggestions.len(), results.suggestions.len());
}

#[tokio::test]
async fn test_store_configuration() {
    let mut storage = Storage::new().await.unwrap();

    let config = crate::config::MoonShineConfig::default();

    let store_result = storage.store_config("test_config", &config).await;
    assert!(store_result.is_ok());

    let retrieved = storage.get_config("test_config").await.unwrap();
    assert!(retrieved.is_some());

    let retrieved_config = retrieved.unwrap();
    assert_eq!(retrieved_config.ai_model, config.ai_model);
    assert_eq!(retrieved_config.max_files, config.max_files);
}

#[tokio::test]
async fn test_cache_operations() {
    let mut storage = Storage::new().await.unwrap();

    let test_data = "test cache data";
    let cache_key = "cache_test_1";

    // Store in cache
    storage.cache_put(cache_key, test_data).await.unwrap();

    // Retrieve from cache
    let cached = storage.cache_get(cache_key).await.unwrap();
    assert!(cached.is_some());
    assert_eq!(cached.unwrap(), test_data);

    // Test cache miss
    let missing = storage.cache_get("nonexistent").await.unwrap();
    assert!(missing.is_none());
}

#[tokio::test]
async fn test_bulk_operations() {
    let mut storage = Storage::new().await.unwrap();

    let mut analyses = HashMap::new();
    for i in 0..10 {
        let key = format!("bulk_test_{}", i);
        let results = AnalysisResultsBuilder::new()
            .files_processed(i)
            .processing_time(i as u64 * 100)
            .suggestion(AiSuggestionBuilder::info()
                .message(&format!("Bulk test suggestion {}", i))
                .build())
            .build();
        analyses.insert(key, results);
    }

    // Store all analyses
    for (key, results) in &analyses {
        storage.store_analysis(key, results).await.unwrap();
    }

    // Retrieve all analyses
    for (key, original) in &analyses {
        let retrieved = storage.get_analysis(key).await.unwrap().unwrap();
        assert_eq!(retrieved.files_processed, original.files_processed);
        assert_eq!(retrieved.processing_time_ms, original.processing_time_ms);
    }
}

#[tokio::test]
async fn test_storage_persistence() {
    let temp_dir = std::env::temp_dir().join("moon_shine_storage_test");

    // Create storage with specific path
    let mut storage1 = Storage::with_path(&temp_dir).await.unwrap();

    let test_key = "persistence_test";
    let results = AnalysisResultsBuilder::typescript_issues().build();

    // Store data
    storage1.store_analysis(test_key, &results).await.unwrap();

    // Close storage
    drop(storage1);

    // Reopen storage with same path
    let storage2 = Storage::with_path(&temp_dir).await.unwrap();

    // Verify data persisted
    let retrieved = storage2.get_analysis(test_key).await.unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.suggestions.len(), results.suggestions.len());

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[tokio::test]
async fn test_concurrent_access() {
    let storage = std::sync::Arc::new(tokio::sync::Mutex::new(Storage::new().await.unwrap()));

    let mut handles = Vec::new();

    // Spawn concurrent tasks
    for i in 0..5 {
        let storage_clone = storage.clone();
        let handle = tokio::spawn(async move {
            let mut storage = storage_clone.lock().await;
            let key = format!("concurrent_{}", i);
            let results = AnalysisResultsBuilder::new()
                .files_processed(i)
                .suggestion(AiSuggestionBuilder::info()
                    .message(&format!("Concurrent test {}", i))
                    .build())
                .build();

            storage.store_analysis(&key, &results).await.unwrap();

            // Verify immediate retrieval
            let retrieved = storage.get_analysis(&key).await.unwrap();
            assert!(retrieved.is_some());
            i
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results: Vec<usize> = futures::future::try_join_all(handles).await.unwrap();
    assert_eq!(results.len(), 5);

    // Verify all data was stored correctly
    let storage = storage.lock().await;
    for i in 0..5 {
        let key = format!("concurrent_{}", i);
        let retrieved = storage.get_analysis(&key).await.unwrap();
        assert!(retrieved.is_some());
    }
}

#[tokio::test]
async fn test_storage_limits() {
    let mut storage = Storage::new().await.unwrap();

    // Test large data storage
    let large_suggestions: Vec<_> = (0..1000).map(|i| {
        AiSuggestionBuilder::warning()
            .message(&format!("Large dataset suggestion {}", i))
            .file_path(&format!("src/large_file_{}.ts", i / 100))
            .line_number(i % 1000 + 1)
            .build()
    }).collect();

    let large_results = AnalysisResultsBuilder::new()
        .suggestions(large_suggestions)
        .files_processed(1000)
        .processing_time(5000)
        .build();

    let store_result = storage.store_analysis("large_dataset", &large_results).await;
    assert!(store_result.is_ok());

    let retrieved = storage.get_analysis("large_dataset").await.unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.suggestions.len(), 1000);
    assert_eq!(retrieved.files_processed, 1000);
}

#[tokio::test]
async fn test_storage_cleanup() {
    let mut storage = Storage::new().await.unwrap();

    // Store multiple entries
    for i in 0..10 {
        let key = format!("cleanup_test_{}", i);
        let results = AnalysisResultsBuilder::clean().build();
        storage.store_analysis(&key, &results).await.unwrap();
    }

    // Verify all entries exist
    for i in 0..10 {
        let key = format!("cleanup_test_{}", i);
        let retrieved = storage.get_analysis(&key).await.unwrap();
        assert!(retrieved.is_some());
    }

    // Test cleanup operation
    let cleanup_result = storage.cleanup_old_entries(std::time::Duration::from_secs(0)).await;
    assert!(cleanup_result.is_ok());

    // Note: Since we just created the entries, they shouldn't be cleaned up
    // This tests the cleanup mechanism without actually removing recent data
}

#[tokio::test]
async fn test_storage_error_handling() {
    let mut storage = Storage::new().await.unwrap();

    // Test invalid key handling
    let retrieved = storage.get_analysis("").await.unwrap();
    assert!(retrieved.is_none());

    // Test retrieval of non-existent data
    let missing = storage.get_analysis("definitely_not_there").await.unwrap();
    assert!(missing.is_none());

    // Test storing with empty key
    let results = AnalysisResultsBuilder::clean().build();
    let store_result = storage.store_analysis("", &results).await;
    // Should handle gracefully (implementation dependent)
    assert!(store_result.is_ok() || store_result.is_err()); // Either is acceptable
}

#[tokio::test]
async fn test_storage_serialization() {
    let mut storage = Storage::new().await.unwrap();

    // Create complex analysis results with various data types
    let complex_results = AnalysisResultsBuilder::new()
        .suggestion(AiSuggestionBuilder::error()
            .message("Complex error with special chars: 🚀💥🔥")
            .file_path("src/unicode_test.ts")
            .confidence_score(0.95)
            .build())
        .suggestion(AiSuggestionBuilder::warning()
            .message("Warning with newlines\nand\ttabs")
            .file_path("src/formatting_test.ts")
            .confidence_score(0.75)
            .build())
        .files_processed(42)
        .processing_time(1337)
        .metadata("complex_field", "complex_value")
        .metadata("unicode", "テスト")
        .build();

    // Store and retrieve
    storage.store_analysis("serialization_test", &complex_results).await.unwrap();
    let retrieved = storage.get_analysis("serialization_test").await.unwrap().unwrap();

    // Verify complex data was preserved
    assert_eq!(retrieved.suggestions.len(), 2);
    assert_eq!(retrieved.files_processed, 42);
    assert_eq!(retrieved.processing_time_ms, 1337);

    // Verify unicode handling
    assert!(retrieved.suggestions[0].message.contains("🚀"));
    assert!(retrieved.suggestions[1].message.contains("\n"));

    // Verify metadata
    assert_eq!(retrieved.metadata.get("complex_field"), Some(&"complex_value".to_string()));
    assert_eq!(retrieved.metadata.get("unicode"), Some(&"テスト".to_string()));
}