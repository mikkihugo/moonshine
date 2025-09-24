//! WASM-friendly JavaScript/TypeScript linting helpers powered by the OXC toolchain.
//!
//! Moon-shine relies on these lightweight wrappers when it needs to produce lint diagnostics
//! directly inside the WASM sandbox. Under the hood we reuse the shared OXC-based linter
//! infrastructure defined in `oxc_adapter` so both native and WASM builds share identical
//! behaviour and rule coverage.

use crate::oxc_adapter::oxc_linter::{OxcConfig, OxcLinter};
use crate::types::{DiagnosticSeverity, LintDiagnostic};
use std::fs;
use std::path::{Path, PathBuf};

/// Re-exported severity type so legacy call sites keep compiling while we lean on the
/// canonical `DiagnosticSeverity` used throughout the project.
pub type LintSeverity = DiagnosticSeverity;

/// Lint issue structure consumed by other subsystems (alias of `LintDiagnostic`).
pub type LintIssue = LintDiagnostic;

/// Thin wrapper around the in-process OXC linter that works in native and WASM builds.
pub struct WasmSafeLinter {
    linter: OxcLinter,
}

impl WasmSafeLinter {
    /// Construct the linter with a sensible default configuration rooted at the current
    /// working directory. The configuration mirrors the previous out-of-process setup but
    /// stays entirely inside the WASM sandbox.
    pub fn new() -> Self {
        let mut config = OxcConfig::default();
        config.project_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            linter: OxcLinter::new(config),
        }
    }

    /// Lint a file on disk, returning the resulting diagnostics.
    pub fn lint_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<Vec<LintIssue>> {
        let path_ref = path.as_ref();
        let contents = fs::read_to_string(path_ref)?;
        Ok(self.lint_source(&contents, path_ref.to_string_lossy().as_ref()))
    }

    /// Lint an in-memory source string. When linting fails (for example because parsing fails),
    /// we surface a single diagnostic that captures the failure instead of panicking.
    pub fn lint_source(&self, source: &str, file_path: &str) -> Vec<LintIssue> {
        match self.linter.analyze_code(source, file_path) {
            Ok(result) => result.diagnostics,
            Err(error) => vec![LintIssue {
                rule_name: "moon/internal-lint-error".to_string(),
                message: format!("Failed to lint '{}': {}", file_path, error),
                file_path: file_path.to_string(),
                line: 1,
                column: 1,
                end_line: 1,
                end_column: 1,
                severity: DiagnosticSeverity::Error,
                fix_available: false,
                suggested_fix: None,
            }],
        }
    }
}

impl Default for WasmSafeLinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_console_usage() {
        let linter = WasmSafeLinter::new();
        let issues = linter.lint_source("console.log('debug');", "test.ts");
        assert!(!issues.is_empty(), "Expected console usage to produce diagnostics");
    }

    #[test]
    fn tolerant_when_no_patterns_match() {
        let linter = WasmSafeLinter::new();
        let issues = linter.lint_source("const value = 1;", "test.ts");
        assert!(issues.is_empty(), "Clean source should produce no diagnostics");
    }
}
