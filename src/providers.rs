use std::path::Path;

use crate::types::ConfigProvider;
use serde_json::{Value, json};

pub struct PythonConfigProvider;
impl ConfigProvider for PythonConfigProvider {
    fn name(&self) -> &'static str {
        "python"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Python: Current File",
            "type": "debugpy",
            "request": "launch",
            "program": "${file}",
            "console": "integratedTerminal",
            "justMyCode": true,
            "args": []
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        path.extension().map_or(false, |ext| ext == "py")
    }
}

pub struct PythonModuleConfigProvider;
impl ConfigProvider for PythonModuleConfigProvider {
    fn name(&self) -> &'static str {
        "python-module"
    }

    fn get_config(&self, params: Option<&str>) -> Value {
        let module_name = params.unwrap_or("app");
        json!({
            "name": format!("Python: Module {}", module_name),
            "type": "debugpy",
            "request": "launch",
            "module": module_name,
            "console": "integratedTerminal",
            "justMyCode": true,
            "args": []
        })
    }

    fn can_detect_from_file(&self, _path: &Path) -> bool {
        false // Not auto-detected from file extension
    }

    fn can_detect_from_content(&self, filename: &str, content: &str) -> bool {
        if filename == "requirements.txt" {
            if content.contains("django") {
                return true;
            }
            if content.contains("pytest") {
                return true;
            }
        }
        false
    }
}

pub struct FlaskConfigProvider;
impl ConfigProvider for FlaskConfigProvider {
    fn name(&self) -> &'static str {
        "flask"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Python: Flask",
            "type": "python",
            "request": "launch",
            "module": "flask",
            "env": {
                "FLASK_APP": "app.py",
                "FLASK_DEBUG": "1"
            },
            "args": [
                "run",
                "--no-debugger",
                "--no-reload"
            ],
            "jinja": true,
            "justMyCode": true
        })
    }

    fn can_detect_from_file(&self, _path: &Path) -> bool {
        false // Not detected from file extension
    }

    fn can_detect_from_content(&self, filename: &str, content: &str) -> bool {
        filename == "requirements.txt" && content.contains("flask")
    }
}

pub struct FastApiConfigProvider;
impl ConfigProvider for FastApiConfigProvider {
    fn name(&self) -> &'static str {
        "fastapi"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Python: FastAPI",
            "type": "python",
            "request": "launch",
            "module": "uvicorn",
            "args": [
                "app.main:app",
                "--reload"
            ],
            "justMyCode": true
        })
    }

    fn can_detect_from_file(&self, _path: &Path) -> bool {
        false // Not detected from file extension
    }

    fn can_detect_from_content(&self, filename: &str, content: &str) -> bool {
        filename == "requirements.txt" && content.contains("fastapi")
    }
}

pub struct JavaScriptConfigProvider;
impl ConfigProvider for JavaScriptConfigProvider {
    fn name(&self) -> &'static str {
        "javascript"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "JavaScript: Launch Chrome",
            "type": "chrome",
            "request": "launch",
            "url": "http://localhost:3000",
            "webRoot": "${workspaceFolder}"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        path.extension().map_or(false, |ext| ext == "js")
    }
}

pub struct NodeConfigProvider;
impl ConfigProvider for NodeConfigProvider {
    fn name(&self) -> &'static str {
        "node"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Node.js: Current File",
            "type": "node",
            "request": "launch",
            "program": "${file}",
            "console": "integratedTerminal"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        path.file_name()
            .map_or(false, |name| name == "package.json")
    }
}

pub struct TypeScriptConfigProvider;
impl ConfigProvider for TypeScriptConfigProvider {
    fn name(&self) -> &'static str {
        "typescript"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "TypeScript: Current File",
            "type": "node",
            "request": "launch",
            "program": "${file}",
            "preLaunchTask": "tsc: build - tsconfig.json",
            "outFiles": ["${workspaceFolder}/dist/**/*.js"]
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if ext == "ts" {
                return true;
            }
        }
        path.file_name()
            .map_or(false, |name| name == "tsconfig.json")
    }
}

pub struct RustConfigProvider;
impl ConfigProvider for RustConfigProvider {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Rust: Debug Binary",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
            "args": [],
            "cwd": "${workspaceFolder}",
            "preLaunchTask": "cargo build"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                return true;
            }
        }
        path.file_name().map_or(false, |name| name == "Cargo.toml")
    }
}

pub struct RustLibConfigProvider;
impl ConfigProvider for RustLibConfigProvider {
    fn name(&self) -> &'static str {
        "rust-lib"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Rust: Debug Library",
            "type": "lldb",
            "request": "launch",
            "cargo": {
                "args": [
                    "build",
                    "--lib"
                ]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        // Detect if this is likely a library project
        if path.file_name().map_or(false, |name| name == "Cargo.toml") {
            if let Ok(content) = std::fs::read_to_string(path) {
                return content.contains("[lib]") || !content.contains("[[bin]]");
            }
        }
        path.file_name().map_or(false, |name| name == "lib.rs")
    }

    fn can_detect_from_content(&self, filename: &str, content: &str) -> bool {
        filename == "Cargo.toml" && (content.contains("[lib]") || !content.contains("[[bin]]"))
    }
}

pub struct RustTestConfigProvider;
impl ConfigProvider for RustTestConfigProvider {
    fn name(&self) -> &'static str {
        "rust-test"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "Rust: Debug Tests",
            "type": "lldb",
            "request": "launch",
            "cargo": {
                "args": [
                    "test",
                    "--no-run"
                ]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                // Check if the file contains test modules or functions
                if let Ok(content) = std::fs::read_to_string(path) {
                    return content.contains("#[test]") || content.contains("mod test");
                }
            }
        }
        false
    }

    fn can_detect_from_content(&self, filename: &str, content: &str) -> bool {
        filename.ends_with(".rs") && (content.contains("#[test]") || content.contains("mod test"))
    }
}

pub struct RustAllConfigProvider;
impl ConfigProvider for RustAllConfigProvider {
    fn name(&self) -> &'static str {
        "rust-all"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "configurations": [
                {
                    "name": "Rust: Debug Binary",
                    "type": "lldb",
                    "request": "launch",
                    "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                    "args": [],
                    "cwd": "${workspaceFolder}",
                    "preLaunchTask": "cargo build"
                },
                {
                    "name": "Rust: Debug Library",
                    "type": "lldb",
                    "request": "launch",
                    "cargo": {
                        "args": [
                            "build",
                            "--lib"
                        ]
                    },
                    "args": [],
                    "cwd": "${workspaceFolder}"
                },
                {
                    "name": "Rust: Debug Tests",
                    "type": "lldb",
                    "request": "launch",
                    "cargo": {
                        "args": [
                            "test",
                            "--no-run"
                        ]
                    },
                    "args": [],
                    "cwd": "${workspaceFolder}"
                }
            ]
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                return true;
            }
        }
        path.file_name().map_or(false, |name| name == "Cargo.toml")
    }
}

fn detect_cpp_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy();
        if ext_str == "cpp"
            || ext_str == "cc"
            || ext_str == "cxx"
            || ext_str == "h"
            || ext_str == "hpp"
        {
            return true;
        }
    }

    if let Some(name) = path.file_name() {
        let name_str = name.to_string_lossy();
        if name_str == "CMakeLists.txt" || name_str == "Makefile" {
            return true;
        }
    }

    false
}

pub struct CppGdbConfigProvider;
impl ConfigProvider for CppGdbConfigProvider {
    fn name(&self) -> &'static str {
        "cpp-gdb"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "C++: GDB",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/build/${fileBasenameNoExtension}",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": false,
            "MIMode": "gdb",
            "setupCommands": [
                {
                    "description": "Enable pretty-printing for gdb",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                }
            ],
            "preLaunchTask": "C/C++: Build active file"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        detect_cpp_file(path)
    }
}

pub struct CppLldbConfigProvider;
impl ConfigProvider for CppLldbConfigProvider {
    fn name(&self) -> &'static str {
        "cpp-lldb"
    }

    fn get_config(&self, _params: Option<&str>) -> Value {
        json!({
            "name": "C++: LLDB",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/build/${fileBasenameNoExtension}",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": false,
            "preLaunchTask": "C/C++: Build active file"
        })
    }

    fn can_detect_from_file(&self, path: &Path) -> bool {
        detect_cpp_file(path)
    }
}
