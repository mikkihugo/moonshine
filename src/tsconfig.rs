use crate::error::{Error, Result};
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolve the effective TypeScript configuration (tsconfig.json/tsconfig.jsonc)
/// by following the standard `extends` chain and merging compiler options.
#[cfg(not(feature = "wasm"))]
pub fn resolve_tsconfig(start_dir: impl AsRef<Path>) -> Result<Option<Value>> {
    let start_dir = start_dir.as_ref();
    let tsconfig_path = find_config_file(start_dir, &TS_CONFIG_CANDIDATES);

    match tsconfig_path {
        Some(path) => {
            let mut visited = HashSet::new();
            let resolved = load_and_merge(&path, &mut visited)?;
            Ok(Some(resolved))
        }
        None => Ok(None),
    }
}

/// WASM builds do not have direct filesystem access, so we return `None` and
/// allow the host to inject configuration through other channels.
#[cfg(feature = "wasm")]
pub fn resolve_tsconfig(_start_dir: impl AsRef<Path>) -> Result<Option<Value>> {
    Ok(None)
}

/// Resolve the effective TSDoc configuration (tsdoc.json/tsdoc.jsonc) including
/// support for the TSDoc `extends` chain.
#[cfg(not(feature = "wasm"))]
pub fn resolve_tsdoc_config(start_dir: impl AsRef<Path>) -> Result<Option<Value>> {
    let start_dir = start_dir.as_ref();
    let tsdoc_path = find_config_file(start_dir, &TSDOC_CONFIG_CANDIDATES);

    match tsdoc_path {
        Some(path) => {
            let mut visited = HashSet::new();
            let resolved = load_and_merge(&path, &mut visited)?;
            Ok(Some(resolved))
        }
        None => Ok(None),
    }
}

#[cfg(feature = "wasm")]
pub fn resolve_tsdoc_config(_start_dir: impl AsRef<Path>) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(not(feature = "wasm"))]
const TS_CONFIG_CANDIDATES: [&str; 4] = ["tsconfig.json", "tsconfig.jsonc", "tsconfig.base.json", "tsconfig.app.json"];

#[cfg(not(feature = "wasm"))]
const TSDOC_CONFIG_CANDIDATES: [&str; 2] = ["tsdoc.json", "tsdoc.jsonc"];

#[cfg(not(feature = "wasm"))]
fn find_config_file(start_dir: &Path, candidates: &[&str]) -> Option<PathBuf> {
    let mut current = Some(start_dir.to_path_buf());

    while let Some(dir) = current {
        for candidate in candidates {
            let candidate_path = dir.join(candidate);
            if candidate_path.exists() {
                return Some(candidate_path);
            }
        }

        current = dir.parent().map(|parent| parent.to_path_buf());
    }

    None
}

#[cfg(not(feature = "wasm"))]
fn load_and_merge(path: &Path, visited: &mut HashSet<PathBuf>) -> Result<Value> {
    if !visited.insert(path.to_path_buf()) {
        return Err(Error::Config {
            message: format!("Circular configuration extends detected at {}", path.display()),
            field: Some("extends".into()),
            value: None,
        });
    }

    let mut current = load_json_with_comments(path)?;

    let extends = current.get("extends").cloned();
    let mut merged_parent: Option<Value> = None;

    if let Some(extends_value) = extends {
        match extends_value {
            Value::String(reference) => {
                let parent_path = resolve_extends_reference(path, &reference)?;
                merged_parent = Some(load_and_merge(&parent_path, visited)?);
            }
            Value::Array(entries) => {
                for entry in entries {
                    if let Value::String(reference) = entry {
                        let parent_path = resolve_extends_reference(path, reference)?;
                        let parent_value = load_and_merge(&parent_path, visited)?;
                        merged_parent = Some(match merged_parent {
                            Some(existing) => merge_values(existing, parent_value),
                            None => parent_value,
                        });
                    }
                }
            }
            _ => {
                return Err(Error::Config {
                    message: "Unsupported extends format".into(),
                    field: Some("extends".into()),
                    value: Some(extends_value.to_string()),
                })
            }
        }
    }

    // Remove extends from child to avoid duplication during merge
    if let Some(obj) = current.as_object_mut() {
        obj.remove("extends");
    }

    let resolved = match merged_parent {
        Some(parent) => merge_values(parent, current),
        None => current,
    };

    visited.remove(path);
    Ok(resolved)
}

#[cfg(not(feature = "wasm"))]
fn resolve_extends_reference(base_file: &Path, reference: &str) -> Result<PathBuf> {
    let base_dir = base_file.parent().unwrap_or_else(|| Path::new("."));

    // Relative path reference
    let candidate = base_dir.join(reference);
    if candidate.exists() {
        return Ok(candidate);
    }

    // If reference omits extension, try appending .json or .jsonc
    let candidate_json = candidate.with_extension("json");
    if candidate_json.exists() {
        return Ok(candidate_json);
    }

    let candidate_jsonc = candidate.with_extension("jsonc");
    if candidate_jsonc.exists() {
        return Ok(candidate_jsonc);
    }

    // Support package references (e.g. "@microsoft/api-extractor/extends/base.json")
    if reference.contains('/') {
        if let Some(node_modules) = find_upwards(base_dir, "node_modules") {
            let package_path = node_modules.join(reference);
            if package_path.exists() {
                return Ok(package_path);
            }

            let package_json_path = package_path.with_extension("json");
            if package_json_path.exists() {
                return Ok(package_json_path);
            }

            let package_jsonc_path = package_path.with_extension("jsonc");
            if package_jsonc_path.exists() {
                return Ok(package_jsonc_path);
            }
        }
    }

    Err(Error::Config {
        message: format!("Unable to resolve extends reference '{}' from {}", reference, base_file.display()),
        field: Some("extends".into()),
        value: Some(reference.to_string()),
    })
}

#[cfg(not(feature = "wasm"))]
fn find_upwards(start: &Path, needle: &str) -> Option<PathBuf> {
    let mut current = Some(start.to_path_buf());

    while let Some(dir) = current {
        let candidate = dir.join(needle);
        if candidate.exists() {
            return Some(candidate);
        }
        current = dir.parent().map(|parent| parent.to_path_buf());
    }

    None
}

#[cfg(not(feature = "wasm"))]
fn merge_values(mut base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Object(mut base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                match base_map.get_mut(&key) {
                    Some(existing) => {
                        let merged = merge_values(existing.clone(), value);
                        base_map.insert(key, merged);
                    }
                    None => {
                        base_map.insert(key, value);
                    }
                }
            }
            Value::Object(base_map)
        }
        (_, overlay_value) => overlay_value,
    }
}

#[cfg(not(feature = "wasm"))]
fn load_json_with_comments(path: &Path) -> Result<Value> {
    let content = fs::read_to_string(path).map_err(|err| Error::Io {
        path: path.display().to_string(),
        source: err,
    })?;

    // Use json5 to tolerate comments and trailing commas when parsing configuration files.
    json5::from_str(&content).map_err(|err| Error::DataParsing {
        message: format!("Failed to parse JSON from {}: {}", path.display(), err),
        line_number: err.position().map(|pos| pos.line() as usize),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn merges_simple_extends_chain() {
        let dir = tempdir().unwrap();
        let base_path = dir.path().join("tsconfig.base.json");
        let child_path = dir.path().join("tsconfig.json");

        File::create(&base_path)
            .unwrap()
            .write_all(br#"{ "compilerOptions": { "strict": true, "target": "ES2022" } }"#)
            .unwrap();

        File::create(&child_path)
            .unwrap()
            .write_all(br#"{ "extends": "./tsconfig.base.json", "compilerOptions": { "module": "ESNext" } }"#)
            .unwrap();

        let resolved = resolve_tsconfig(dir.path()).unwrap().unwrap();
        let compiler_options = resolved.get("compilerOptions").and_then(Value::as_object).expect("compiler options present");

        assert_eq!(compiler_options.get("strict"), Some(&json!(true)));
        assert_eq!(compiler_options.get("module"), Some(&json!("ESNext")));
        assert_eq!(compiler_options.get("target"), Some(&json!("ES2022")));
    }

    #[test]
    fn returns_none_when_missing() {
        let dir = tempdir().unwrap();
        let resolved = resolve_tsconfig(dir.path()).unwrap();
        assert!(resolved.is_none());
    }
}
