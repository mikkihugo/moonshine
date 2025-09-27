//! Minimal multi-language analyzer used by the Moon Shine workflow.
//!
//! The current extension only performs deep analysis for TypeScript and JavaScript via the
//! OXC-based [`MultiEngineAnalyzer`]. This module provides lightweight language detection and a
//! thin wrapper around the analyzer so that higher‑level code does not have to reason about
//! file extensions or the OXC configuration directly. The design leaves room for expanding into
//! additional languages without re-introducing the bulky scaffolding that previously existed.

use crate::oxc_adapter::{MultiEngineAnalyzer, MultiEngineConfig};
use crate::types::LintDiagnostic;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Languages the analyzer can recognise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedLanguage {
    TypeScript,
    JavaScript,
    Unknown,
}

/// Lightweight configuration for multi-language analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Optional explicit language override. If `None`, detection is based on the file extension.
    pub explicit_language: Option<SupportedLanguage>,
    /// Optional working directory used for resolving language-specific resources (tsconfig, etc.).
    pub workspace_root: Option<PathBuf>,
    /// OXC configuration shared across JS/TS analysis.
    #[serde(default)]
    pub oxc_config: MultiEngineConfig,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            explicit_language: None,
            workspace_root: None,
            oxc_config: MultiEngineConfig::default(),
        }
    }
}

/// Result returned after analysing a single file.
#[derive(Debug)]
pub struct LanguageAnalysisResult {
    pub language: SupportedLanguage,
    pub diagnostics: Vec<LintDiagnostic>,
    pub formatted_code: Option<String>,
}

/// Minimal multi-language analyzer responsible for routing files to the correct engine.
pub struct MultiLanguageAnalyzer {
    language_config: LanguageConfig,
    multi_engine: MultiEngineAnalyzer,
}

impl MultiLanguageAnalyzer {
    /// Create a new analyzer using the provided language configuration.
    pub fn new(language_config: LanguageConfig) -> Self {
        let multi_engine = MultiEngineAnalyzer::with_config(language_config.oxc_config.clone());
        Self { language_config, multi_engine }
    }

    /// Detect which language should be used for the given file/content pair.
    pub fn detect_language(&self, file_path: &str, _content: &str) -> SupportedLanguage {
        if let Some(explicit) = self.language_config.explicit_language {
            return explicit;
        }

        match Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref()
        {
            Some("ts") | Some("tsx") => SupportedLanguage::TypeScript,
            Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => SupportedLanguage::JavaScript,
            _ => SupportedLanguage::Unknown,
        }
    }

    /// Analyse the provided file. Unsupported languages return an empty diagnostic list.
    pub async fn analyze(
        &mut self,
        file_path: &str,
        content: &str,
    ) -> Result<LanguageAnalysisResult, Box<dyn std::error::Error>> {
        let language = self.detect_language(file_path, content);

        if language == SupportedLanguage::Unknown {
            return Ok(LanguageAnalysisResult {
                language,
                diagnostics: Vec::new(),
                formatted_code: None,
            });
        }

        let analysis = self.multi_engine.analyze_code(content, file_path).await?;
        let diagnostics = analysis.diagnostics;
        let formatted_code = analysis.formatted_code;

        Ok(LanguageAnalysisResult {
            language,
            diagnostics,
            formatted_code,
        })
    }

    /// Expose mutable access to the underlying multi-engine configuration for callers who need
    /// to tweak behaviour at runtime.
    pub fn engine_mut(&mut self) -> &mut MultiEngineAnalyzer {
        &mut self.multi_engine
    }
}
