//! # OXC + Moon PDK Integration (in-process)
//!
//! Provides an incremental analysis harness that mirrors how the extension runs inside Moon while
//! staying entirely in-process. We reuse the `WasmSafeLinter` wrapper so both native and WASM builds
//! execute the same OXC-based linting pipeline without invoking external CLIs.

use crate::javascript_typescript_linter::WasmSafeLinter;
use crate::types::LintDiagnostic;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// In-process analyzer that tracks file modification times and only re-lints changed files.
pub struct OxcMoonAnalyzer {
    project_root: PathBuf,
    file_cache: HashMap<PathBuf, u64>,
    linter: WasmSafeLinter,
}

/// Result of running the analyzer.
#[derive(Debug)]
pub struct MoonAnalysisResult {
    pub diagnostics: Vec<LintDiagnostic>,
    pub analyzed_files: Vec<PathBuf>,
    pub analysis_time_ms: u64,
    pub incremental: bool,
    pub cached_hits: usize,
}

impl OxcMoonAnalyzer {
    /// Create a new analyzer rooted at the given project directory.
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            file_cache: HashMap::new(),
            linter: WasmSafeLinter::new(),
        }
    }

    /// Run analysis, only linting files that have changed since the last invocation.
    pub async fn analyze_incremental(&mut self) -> Result<MoonAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let changed_files = self.discover_changed_files()?;
        let incremental = !self.file_cache.is_empty();

        if changed_files.is_empty() {
            return Ok(MoonAnalysisResult {
                diagnostics: Vec::new(),
                analyzed_files: Vec::new(),
                analysis_time_ms: start_time.elapsed().as_millis() as u64,
                incremental,
                cached_hits: 0,
            });
        }

        let mut diagnostics = Vec::new();
        for file_path in &changed_files {
            let content = fs::read_to_string(file_path)?;
            diagnostics.extend(self.linter.lint_source(&content, &file_path.to_string_lossy()));
        }

        self.update_file_cache(&changed_files)?;

        Ok(MoonAnalysisResult {
            diagnostics,
            analyzed_files: changed_files,
            analysis_time_ms: start_time.elapsed().as_millis() as u64,
            incremental,
            cached_hits: 0,
        })
    }

    /// Discover files that have changed since the previous analysis run.
    fn discover_changed_files(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut changed_files = Vec::new();

        for entry in walkdir::WalkDir::new(&self.project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_supported_file(e.path()))
        {
            let file_path = entry.path().to_path_buf();

            if let Ok(metadata) = fs::metadata(&file_path) {
                if let Ok(modified) = metadata.modified() {
                    let timestamp = modified.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

                    match self.file_cache.get(&file_path) {
                        Some(&last_timestamp) if timestamp <= last_timestamp => {}
                        _ => changed_files.push(file_path.clone()),
                    }
                }
            }
        }

        Ok(changed_files)
    }

    /// Determine whether a path is supported by the analyzer.
    fn is_supported_file(&self, path: &Path) -> bool {
        matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" | "json" | "jsonc")
        )
    }

    /// Update internal timestamp cache after a run.
    fn update_file_cache(&mut self, files: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
        for file_path in files {
            if let Ok(metadata) = fs::metadata(file_path) {
                if let Ok(modified) = metadata.modified() {
                    let timestamp = modified.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
                    self.file_cache.insert(file_path.clone(), timestamp);
                }
            }
        }
        Ok(())
    }
}
