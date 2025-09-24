//! Lightweight telemetry sink for workflow runs.
//!
//! Records each workflow execution so future planning logic can learn from
//! historical outcomes. Uses Moon's host file APIs via the PDK—no direct
//! filesystem access from WASM.

use crate::moon_pdk_interface::{read_file_content, write_file_atomic};
use serde::Serialize;

/// Immutable snapshot describing a single workflow run.
#[derive(Debug, Serialize)]
pub struct TelemetryRecord {
    pub file_path: String,
    pub success: bool,
    pub total_steps: u32,
    pub executed_steps: Vec<String>,
    pub duration_ms: u128,
    pub issues_found: Option<u64>,
    pub ai_strategy: Option<String>,
}

const DEFAULT_MAX_RECORDS: usize = 5000;

/// Simple JSONL telemetry collector.
#[derive(Debug, Clone)]
pub struct TelemetryCollector {
    output_path: String,
    max_records: Option<usize>,
}

impl TelemetryCollector {
    /// Create a collector writing to the provided path, or the default location if `None`.
    pub fn new(path_override: Option<String>) -> Self {
        let path = path_override.unwrap_or_else(default_telemetry_path);
        let max_records = read_max_records_config().or(Some(DEFAULT_MAX_RECORDS));

        Self {
            output_path: path,
            max_records,
        }
    }

    /// Record a telemetry entry; errors are logged but do not bubble up to callers.
    pub fn record(&self, record: &TelemetryRecord) {
        match serde_json::to_string(record) {
            Ok(line) => {
                let mut lines = match read_file_content(&self.output_path) {
                    Ok(existing) if !existing.is_empty() => existing.lines().map(str::to_owned).collect::<Vec<String>>(),
                    _ => Vec::new(),
                };

                lines.push(line);

                if let Some(max) = self.max_records {
                    if lines.len() > max {
                        let excess = lines.len() - max;
                        lines.drain(0..excess);
                    }
                }

                let mut payload = lines.join("\n");
                payload.push('\n');

                if let Err(err) = write_file_atomic(&self.output_path, &payload) {
                    eprintln!("[telemetry] failed to write record to {}: {}", self.output_path, err);
                }
            }
            Err(err) => {
                eprintln!("[telemetry] failed to serialise record: {}", err);
            }
        }
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        let override_path = read_config_value("moon_shine_telemetry_path");
        Self::new(override_path)
    }
}

fn default_telemetry_path() -> String {
    ".moon/moonshine/telemetry.jsonl".to_string()
}

fn read_config_value(key: &str) -> Option<String> {
    crate::moon_pdk_interface::get_moon_config_safe(key)
        .ok()
        .flatten()
        .filter(|value| !value.trim().is_empty())
}

fn read_max_records_config() -> Option<usize> {
    read_config_value("moon_shine_telemetry_max_records")
        .and_then(|value| value.parse().ok())
        .filter(|count| *count > 0)
}

/// Utility to stringify JSON values when presenting strategy info.
pub fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Helper to decode optional numeric context values.
pub fn json_value_to_u64(value: &serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Number(num) => num.as_u64(),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    }
}
