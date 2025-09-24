use crate::error::{Error, Result};
use crate::tsconfig::{resolve_tsconfig, resolve_tsdoc_config};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

static FUNCTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*(?:export\s+)?(?:async\s+)?function\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\((?P<params>[^)]*)\)").expect("valid function regex")
});
static METHOD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*(?:public\s+|private\s+|protected\s+|readonly\s+|static\s+)*\s*(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\((?P<params>[^)]*)\)\s*{")
        .expect("valid method regex")
});

/// Public API for TSDoc analysis results.
#[derive(Debug, Default, Clone)]
pub struct TsdocAnalysis {
    pub total_symbols: usize,
    pub documented_symbols: usize,
    pub diagnostics: Vec<TsdocDiagnostic>,
    pub resolved_tsconfig: Option<Value>,
    pub resolved_tsdoc: Option<Value>,
}

impl TsdocAnalysis {
    pub fn coverage(&self) -> f64 {
        if self.total_symbols == 0 {
            return 100.0;
        }
        (self.documented_symbols as f64 / self.total_symbols as f64) * 100.0
    }
}

#[derive(Debug, Clone)]
pub struct TsdocDiagnostic {
    pub message: String,
    pub symbol: Option<String>,
    pub line: Option<usize>,
}

/// Represents configurable behaviour for TSDoc analysis.
#[derive(Debug, Clone)]
pub struct TsdocConfig {
    pub allowed_tags: HashSet<String>,
    pub require_param_tags: bool,
}

impl TsdocConfig {
    pub fn default() -> Self {
        let allowed_tags = ["@param", "@returns", "@remarks", "@example", "@deprecated", "@throws", "@see"]
            .into_iter()
            .map(String::from)
            .collect();

        Self {
            allowed_tags,
            require_param_tags: true,
        }
    }

    pub fn from_tsdoc_value(value: &Value) -> Self {
        let mut config = Self::default();

        if let Some(tag_defs) = value.get("tagDefinitions").and_then(Value::as_array) {
            for tag in tag_defs {
                if let Some(name) = tag.get("tagName").and_then(Value::as_str) {
                    config.allowed_tags.insert(name.to_string());
                }
            }
        }

        if let Some(require_param) = value.get("moonshine").and_then(|v| v.get("requireParamTags")).and_then(Value::as_bool) {
            config.require_param_tags = require_param;
        }

        config
    }
}

/// Analyse a TypeScript/JavaScript source file for TSDoc coverage and diagnostics.
pub fn analyze_source(content: &str, file_path: Option<&Path>) -> TsdocAnalysis {
    let workspace_root = file_path
        .and_then(|path| path.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let resolved_tsconfig = resolve_tsconfig(&workspace_root).unwrap_or(None);
    let resolved_tsdoc = resolve_tsdoc_config(&workspace_root).unwrap_or(None);

    let tsdoc_config = resolved_tsdoc.as_ref().map(TsdocConfig::from_tsdoc_value).unwrap_or_else(TsdocConfig::default);

    let mut documented_symbols = 0usize;
    let mut total_symbols = 0usize;
    let mut diagnostics = Vec::new();

    let comment_index = build_comment_index(content);

    for capture in FUNCTION_REGEX.captures_iter(content) {
        total_symbols += 1;
        let name = capture.name("name").map(|m| m.as_str().to_string()).unwrap_or_else(|| "anonymous".to_string());
        let params = capture.name("params").map(|m| m.as_str()).unwrap_or("");
        let start = capture.get(0).map(|m| m.start()).unwrap_or(0);

        match extract_doc_comment(content, start, &comment_index) {
            Some(block) => {
                documented_symbols += 1;
                let parsed = parse_doc_comment(block);
                validate_comment(&name, params, &parsed, &tsdoc_config, &mut diagnostics);
            }
            None => diagnostics.push(TsdocDiagnostic {
                message: format!("Missing TSDoc for function '{name}'"),
                symbol: Some(name.clone()),
                line: line_number_for_offset(content, start),
            }),
        }
    }

    for capture in METHOD_REGEX.captures_iter(content) {
        // Skip matches that were already accounted for by the function regex
        if capture.get(0).map(|m| m.as_str()).unwrap_or("").contains("function") {
            continue;
        }

        total_symbols += 1;
        let name = capture.name("name").map(|m| m.as_str().to_string()).unwrap_or_else(|| "anonymous".to_string());
        let params = capture.name("params").map(|m| m.as_str()).unwrap_or("");
        let start = capture.get(0).map(|m| m.start()).unwrap_or(0);

        match extract_doc_comment(content, start, &comment_index) {
            Some(block) => {
                documented_symbols += 1;
                let parsed = parse_doc_comment(block);
                validate_comment(&name, params, &parsed, &tsdoc_config, &mut diagnostics);
            }
            None => diagnostics.push(TsdocDiagnostic {
                message: format!("Missing TSDoc for method '{name}'"),
                symbol: Some(name.clone()),
                line: line_number_for_offset(content, start),
            }),
        }
    }

    TsdocAnalysis {
        total_symbols,
        documented_symbols,
        diagnostics,
        resolved_tsconfig,
        resolved_tsdoc,
    }
}

fn build_comment_index(content: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let bytes = content.as_bytes();
    let mut i = 0;

    while i + 3 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' && bytes[i + 2] == b'*' {
            indices.push(i);
        }
        i += 1;
    }
    indices
}

fn extract_doc_comment<'a>(content: &'a str, position: usize, indices: &[usize]) -> Option<&'a str> {
    let candidate = indices.iter().rev().find(|&&idx| idx < position)?;
    let comment_slice = &content[*candidate..position];
    let end_offset = comment_slice.find("*/")?;
    let after = &comment_slice[end_offset + 2..];

    if after.trim().is_empty() || after.trim_matches(|c: char| c == '\n' || c.is_whitespace()).is_empty() {
        Some(&comment_slice[..end_offset + 2])
    } else {
        None
    }
}

#[derive(Default)]
struct ParsedComment {
    summary: Vec<String>,
    tags: HashMap<String, Vec<TagEntry>>,
}

#[derive(Debug, Clone)]
struct TagEntry {
    name: String,
    value: String,
}

fn parse_doc_comment(raw: &str) -> ParsedComment {
    let mut comment = ParsedComment::default();
    let mut current_tag: Option<String> = None;

    let body = raw.trim_start_matches("/**").trim_end_matches("*/");

    for line in body.lines() {
        let line = line.trim_start();
        let line = line.strip_prefix('*').unwrap_or(line).trim_start();

        if line.starts_with('@') {
            let mut parts = line.splitn(2, ' ');
            let tag = parts.next().unwrap_or("");
            let rest = parts.next().unwrap_or("").trim().to_string();
            current_tag = Some(tag.to_string());
            comment.tags.entry(tag.to_string()).or_default().push(TagEntry {
                name: tag.to_string(),
                value: rest,
            });
            continue;
        }

        if let Some(tag) = &current_tag {
            if let Some(entries) = comment.tags.get_mut(tag) {
                if let Some(last) = entries.last_mut() {
                    if !line.is_empty() {
                        last.value.push(' ');
                        last.value.push_str(line.trim());
                    }
                    continue;
                }
            }
        }

        if !line.is_empty() {
            comment.summary.push(line.to_string());
        } else {
            current_tag = None;
        }
    }

    comment
}

fn validate_comment(symbol: &str, params: &str, comment: &ParsedComment, config: &TsdocConfig, diagnostics: &mut Vec<TsdocDiagnostic>) {
    if comment.summary.is_empty() {
        diagnostics.push(TsdocDiagnostic {
            message: format!("TSDoc summary missing for '{symbol}'"),
            symbol: Some(symbol.to_string()),
            line: None,
        });
    }

    if config.require_param_tags {
        let expected_params = parse_param_names(params);
        let documented_params: HashSet<String> = comment
            .tags
            .get("@param")
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry.value.split_whitespace().next())
                    .map(|name| name.trim_matches(|c| c == '[' || c == ']' || c == '{' || c == '}').to_string())
                    .collect()
            })
            .unwrap_or_default();

        for param in expected_params {
            if !documented_params.contains(&param) {
                diagnostics.push(TsdocDiagnostic {
                    message: format!("Missing @param documentation for '{param}' in '{symbol}'"),
                    symbol: Some(symbol.to_string()),
                    line: None,
                });
            }
        }
    }

    for tag in comment.tags.keys() {
        if !config.allowed_tags.contains(tag) {
            diagnostics.push(TsdocDiagnostic {
                message: format!("Unsupported TSDoc tag '{tag}' in '{symbol}'"),
                symbol: Some(symbol.to_string()),
                line: None,
            });
        }
    }
}

fn parse_param_names(params: &str) -> Vec<String> {
    params
        .split(',')
        .map(|param| param.trim())
        .filter(|param| !param.is_empty())
        .filter_map(|param| {
            if param.starts_with('{') || param.starts_with('[') {
                return None; // Skip destructuring for now
            }

            let name_part = param.split(':').next().unwrap_or(param).split('=').next().unwrap_or(param).trim();

            let cleaned = name_part
                .trim_start_matches("readonly ")
                .trim_start_matches("public ")
                .trim_start_matches("private ")
                .trim_start_matches("protected ")
                .trim_start_matches("async ")
                .trim_start_matches("...");

            let cleaned = cleaned.trim_end_matches('?').trim();

            if cleaned.is_empty() {
                None
            } else {
                Some(cleaned.to_string())
            }
        })
        .collect()
}

fn line_number_for_offset(content: &str, offset: usize) -> Option<usize> {
    let mut count = 1usize;
    for (idx, ch) in content.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn detects_missing_docs() {
        let src = r#"
function foo(bar: string) {
  return bar;
}
"#;
        let analysis = analyze_source(src, None);
        assert_eq!(analysis.total_symbols, 1);
        assert_eq!(analysis.documented_symbols, 0);
        assert!(analysis.coverage() - 0.0 < f64::EPSILON);
        assert_eq!(analysis.diagnostics.len(), 1);
    }

    #[test]
    fn detects_documented_function() {
        let src = r#"
/**
 * Adds two numbers.
 * @param a first number
 * @param b second number
 */
function add(a: number, b: number) {
  return a + b;
}
"#;
        let analysis = analyze_source(src, None);
        assert_eq!(analysis.total_symbols, 1);
        assert_eq!(analysis.documented_symbols, 1);
        assert!((analysis.coverage() - 100.0).abs() < f64::EPSILON);
        assert!(analysis.diagnostics.is_empty());
    }

    #[test]
    fn respects_tsdoc_extends() {
        let dir = tempdir().unwrap();
        let base = dir.path().join("tsdoc.base.json");
        std::fs::write(&base, "{ \"moonshine\": { \"requireParamTags\": false }}").unwrap();
        std::fs::write(
            dir.path().join("tsdoc.json"),
            format!("{{ \"extends\": \"{}\" }}", base.file_name().unwrap().to_string_lossy()),
        )
        .unwrap();

        let src = r#"
/**
 * Does something.
 */
function noop(param: string) {
  return param;
}
"#;
        let analysis = analyze_source(src, Some(&dir.path().join("example.ts")));
        assert!(analysis.diagnostics.iter().all(|d| !d.message.contains("@param")));
    }
}
