use std::{collections::HashMap, fs};

use serde_json::Value;
use walkdir::WalkDir;

use crate::types::ConfigProvider;

pub fn detect_project_types(
    providers: &[Box<dyn ConfigProvider>],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut detected_types = Vec::new();
    let mut detected_files = HashMap::new();
    let mut has_python_files = false;
    let mut has_js_files = false;
    let mut has_ts_files = false;
    let mut has_rust_files = false;
    let mut has_cpp_files = false;

    // Scan files for detection
    for entry in WalkDir::new(".")
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Check file extensions for quick language detection
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                match ext_str.as_ref() {
                    "py" => has_python_files = true,
                    "js" => has_js_files = true,
                    "ts" => has_ts_files = true,
                    "rs" => has_rust_files = true,
                    "cpp" | "cc" | "cxx" | "h" | "hpp" => has_cpp_files = true,
                    _ => {}
                }
            }

            // Store special files for content-based detection
            if file_name == "requirements.txt"
                || file_name == "package.json"
                || file_name == "Cargo.toml"
                || file_name == "CMakeLists.txt"
                || file_name == "Makefile"
            {
                detected_files.insert(file_name.clone(), path.to_path_buf());
            }

            // Check each provider for file-based detection
            for provider in providers.iter() {
                if provider.can_detect_from_file(path) {
                    detected_types.push(provider.name().to_string());
                }
            }
        }
    }

    // Add language-specific types based on file extensions
    if has_python_files {
        detected_types.push("python".to_string());
    }
    if has_js_files {
        detected_types.push("javascript".to_string());
    }
    if has_ts_files {
        detected_types.push("typescript".to_string());
    }
    if has_cpp_files {
        detected_types.push("cpp-gdb".to_string());
    }

    // Special handling for npm/node
    if detected_files.contains_key("package.json") {
        detected_types.push("node".to_string());

        // Check package.json for specific frameworks
        if let Ok(content) = fs::read_to_string(detected_files.get("package.json").unwrap()) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(deps) = json.get("dependencies") {
                    if deps.get("react").is_some() {
                        detected_types.push("react".to_string());
                    }
                    if deps.get("vue").is_some() {
                        detected_types.push("vue".to_string());
                    }
                    if deps.get("express").is_some() {
                        detected_types.push("express".to_string());
                    }
                }
            }
        }
    }

    // Special handling for Python frameworks
    if detected_files.contains_key("requirements.txt") {
        if let Ok(content) = fs::read_to_string(detected_files.get("requirements.txt").unwrap()) {
            if content.contains("flask") {
                detected_types.push("flask".to_string());
            }
            if content.contains("fastapi") {
                detected_types.push("fastapi".to_string());
            }
            if content.contains("django") {
                detected_types.push("python-module:django".to_string());
            }
            if content.contains("pytest") {
                detected_types.push("python-module:pytest".to_string());
            }
        }
    }

    // Special handling for Rust projects
    if has_rust_files {
        detected_types.push("rust".to_string());

        // Check if this is a library project
        if let Some(cargo_toml) = detected_files.get("Cargo.toml") {
            if let Ok(content) = fs::read_to_string(cargo_toml) {
                if content.contains("[lib]") || !content.contains("[[bin]]") {
                    detected_types.push("rust-lib".to_string());
                }
            }
        }

        // Check for test files
        let mut has_tests = false;
        for entry in WalkDir::new(".")
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains("#[test]") || content.contains("mod test") {
                        has_tests = true;
                        break;
                    }
                }
            }
        }

        if has_tests {
            detected_types.push("rust-test".to_string());
        }

        // If we have at least 2 of the 3 types, suggest the all-in-one config
        let rust_types = [
            "rust".to_string(),
            "rust-lib".to_string(),
            "rust-test".to_string(),
        ];
        let count = rust_types
            .iter()
            .filter(|t| detected_types.contains(t))
            .count();
        if count >= 2 {
            detected_types.push("rust-all".to_string());
        }
    }

    // Perform content-based detection for special files
    for (filename, path) in &detected_files {
        if let Ok(content) = fs::read_to_string(path) {
            for provider in providers.iter() {
                if provider.can_detect_from_content(filename, &content) {
                    // Skip if we already added the type through file detection
                    if !detected_types.contains(&provider.name().to_string()) {
                        detected_types.push(provider.name().to_string());
                    }
                }
            }
        }
    }

    // Remove duplicates
    detected_types.sort();
    detected_types.dedup();

    Ok(detected_types)
}
