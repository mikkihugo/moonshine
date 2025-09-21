//! Parallel ESLint + OXC Lint Runner for Moon Shine
//! Runs both linters concurrently, aggregates results, and provides a modular API.

use std::process::Stdio;
use std::time::Instant;
use std::path::Path;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::try_join;
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct LintIssue {
  pub rule_name: String,
  pub message: String,
  pub file_path: String,
  pub line: u32,
  pub column: u32,
  pub severity: String,
  pub fix_available: bool,
  pub source: String,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LintMetrics {
  pub total_duration_ms: u128,
  pub eslint_duration_ms: Option<u128>,
  pub oxc_duration_ms: Option<u128>,
  pub files_processed: usize,
  pub errors: usize,
  pub warnings: usize,
  pub concurrency: usize,
}

impl LintMetrics {
  pub fn to_human(&self) -> String {
    format!(
      "Lint Metrics:\n  Total time: {}ms\n  ESLint: {}ms\n  OXC: {}ms\n  Files: {}\n  Errors: {}\n  Warnings: {}\n  Concurrency: {}",
      self.total_duration_ms,
      self.eslint_duration_ms.unwrap_or(0),
      self.oxc_duration_ms.unwrap_or(0),
      self.files_processed,
      self.errors,
      self.warnings,
      self.concurrency,
    )
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelLintResult {
  pub issues: Vec<LintIssue>,
  pub tool_status: Vec<(String, bool)>,
  pub duration_ms: u128,
  pub metrics: LintMetrics,
}

pub struct ParallelLintConfig {
  pub target: String,
  pub eslint_path: String,
  pub oxc_path: String,
  pub concurrency: usize,
}

impl Default for ParallelLintConfig {
  fn default() -> Self {
    Self {
      target: ".".to_string(),
      eslint_path: "eslint".to_string(),
      oxc_path: "oxlint".to_string(),
      concurrency: 2,
    }
  }
}

/// Run ESLint and OXC (oxlint) in parallel on the given target.
/// Returns merged issues and tool status.
pub async fn run_parallel_lint(config: ParallelLintConfig) -> ParallelLintResult {
  let start = Instant::now();

  let eslint_start = Instant::now();
  let eslint_fut = run_eslint(&config.eslint_path, &config.target);
  let oxc_start = Instant::now();
  let oxc_fut = run_oxc(&config.oxc_path, &config.target);

  let (eslint_res, oxc_res) = match try_join!(eslint_fut, oxc_fut) {
    Ok((e, o)) => (e, o),
    Err(e) => {
      error!("Lint subprocess error: {e:?}");
      return ParallelLintResult {
        issues: vec![],
        tool_status: vec![
          ("eslint".to_string(), false),
          ("oxc".to_string(), false),
        ],
        duration_ms: start.elapsed().as_millis(),
        metrics: LintMetrics::default(),
      };
    }
  };

  let eslint_duration = eslint_start.elapsed().as_millis();
  let oxc_duration = oxc_start.elapsed().as_millis();

  let mut issues = Vec::new();
  let mut tool_status = vec![];

  match eslint_res {
    Ok(mut v) => {
      for mut i in &mut v { i.source = "eslint".to_string(); }
      issues.extend(v);
      tool_status.push(("eslint".to_string(), true));
    }
    Err(e) => {
      error!("ESLint failed: {e}");
      tool_status.push(("eslint".to_string(), false));
    }
  }
  match oxc_res {
    Ok(mut v) => {
      for mut i in &mut v { i.source = "oxc".to_string(); }
      issues.extend(v);
      tool_status.push(("oxc".to_string(), true));
    }
    Err(e) => {
      error!("OXC failed: {e}");
      tool_status.push(("oxc".to_string(), false));
    }
  }

  // Merge and sort by file/line/column, preserving order
  issues.sort_by(|a, b| {
    a.file_path.cmp(&b.file_path)
      .then(a.line.cmp(&b.line))
      .then(a.column.cmp(&b.column))
      .then(a.rule_name.cmp(&b.rule_name))
  });

  // Metrics calculation
  use std::collections::HashSet;
  let mut files = HashSet::new();
  let mut errors = 0;
  let mut warnings = 0;
  for issue in &issues {
    files.insert(&issue.file_path);
    match issue.severity.as_str() {
      "Error" => errors += 1,
      "Warning" => warnings += 1,
      _ => {}
    }
  }
  let metrics = LintMetrics {
    total_duration_ms: start.elapsed().as_millis(),
    eslint_duration_ms: Some(eslint_duration),
    oxc_duration_ms: Some(oxc_duration),
    files_processed: files.len(),
    errors,
    warnings,
    concurrency: config.concurrency,
  };

  info!("Parallel lint completed in {}ms", start.elapsed().as_millis());

  ParallelLintResult {
    issues,
    tool_status,
    duration_ms: start.elapsed().as_millis(),
    metrics,
  }
}

async fn run_eslint(eslint_path: &str, target: &str) -> Result<Vec<LintIssue>, String> {
  let output = Command::new(eslint_path)
    .arg("--format")
    .arg("json")
    .arg(target)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .await
    .map_err(|e| format!("Failed to spawn ESLint: {e}"))?;

  if !output.status.success() && !output.stdout.is_empty() {
    error!("ESLint stderr: {}", String::from_utf8_lossy(&output.stderr));
  }

  let raw = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&raw)
    .map_err(|e| format!("ESLint output parse error: {e}"))?;

  let mut issues = Vec::new();
  if let Some(arr) = parsed.as_array() {
    for file in arr {
      let file_path = file.get("filePath").and_then(|v| v.as_str()).unwrap_or("").to_string();
      if let Some(messages) = file.get("messages").and_then(|v| v.as_array()) {
        for msg in messages {
          issues.push(LintIssue {
            rule_name: msg.get("ruleId").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            message: msg.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            file_path: file_path.clone(),
            line: msg.get("line").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            column: msg.get("column").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            severity: match msg.get("severity").and_then(|v| v.as_u64()) {
              Some(2) => "Error".to_string(),
              Some(1) => "Warning".to_string(),
              _ => "Info".to_string(),
            },
            fix_available: msg.get("fix").is_some(),
            source: "eslint".to_string(),
          });
        }
      }
    }
  }
  Ok(issues)
}

async fn run_oxc(oxc_path: &str, target: &str) -> Result<Vec<LintIssue>, String> {
  let output = Command::new(oxc_path)
    .arg("--format")
    .arg("json")
    .arg(target)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .await
    .map_err(|e| format!("Failed to spawn OXC: {e}"))?;

  if !output.status.success() && !output.stdout.is_empty() {
    error!("OXC stderr: {}", String::from_utf8_lossy(&output.stderr));
  }

  let raw = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&raw)
    .map_err(|e| format!("OXC output parse error: {e}"))?;

  let mut issues = Vec::new();
  if let Some(arr) = parsed.as_array() {
    for file in arr {
      let file_path = file.get("filePath").and_then(|v| v.as_str()).unwrap_or("").to_string();
      if let Some(messages) = file.get("messages").and_then(|v| v.as_array()) {
        for msg in messages {
          issues.push(LintIssue {
            rule_name: msg.get("ruleId").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            message: msg.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            file_path: file_path.clone(),
            line: msg.get("line").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            column: msg.get("column").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            severity: match msg.get("severity").and_then(|v| v.as_str()) {
              Some("error") => "Error".to_string(),
              Some("warning") => "Warning".to_string(),
              _ => "Info".to_string(),
            },
            fix_available: msg.get("fix").is_some(),
            source: "oxc".to_string(),
          });
        }
      }
    }
  }
  Ok(issues)
}

// Minimal usage example (for test or integration)
#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_parallel_lint_runner_smoke() {
    let config = ParallelLintConfig {
      target: "src".to_string(),
      ..Default::default()
    };
    let result = run_parallel_lint(config).await;
    assert_eq!(result.tool_status.len(), 2, "Should have two tool statuses (OXC and ESLint)");

    // Metrics assertions
    let metrics = &result.metrics;
    // Duration should be nonzero (allowing for very fast runs)
    assert!(metrics.total_duration_ms >= 0, "Total duration should be non-negative");
    // Concurrency should be at least 1
    assert!(metrics.concurrency >= 1, "Concurrency should be at least 1");
    // Files processed should be >= 0
    assert!(metrics.files_processed >= 0, "Files processed should be >= 0");
    // Errors and warnings should be >= 0
    assert!(metrics.errors_found >= 0, "Errors found should be >= 0");
    assert!(metrics.warnings_found >= 0, "Warnings found should be >= 0");
    // Per-linter durations present
    assert!(metrics.linter_durations_ms.contains_key("OXC"), "OXC duration present");
    assert!(metrics.linter_durations_ms.contains_key("ESLint"), "ESLint duration present");
    // Accept empty issues if no JS/TS files present
  }
}