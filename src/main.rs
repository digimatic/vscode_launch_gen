use clap::Parser;
use detect::detect_project_types;
use providers::{
    CppGdbConfigProvider, CppLldbConfigProvider, FastApiConfigProvider, FlaskConfigProvider,
    JavaScriptConfigProvider, NodeConfigProvider, PythonConfigProvider, PythonModuleConfigProvider,
    RustAllConfigProvider, RustConfigProvider, RustLibConfigProvider, RustTestConfigProvider,
    TypeScriptConfigProvider,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use types::ConfigProvider;

mod detect;
mod providers;
mod types;

#[derive(Parser)]
#[command(
    name = "launch-json-generator",
    version = "1.0",
    author = "Your Name",
    about = "Generates VS Code launch.json configurations"
)]
struct Cli {
    /// Output file path (default: .vscode/launch.json)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Add configurations for specific types (can be specified multiple times)
    /// Available types: python, python-module:<name>, flask, fastapi, javascript,
    /// node, typescript, rust, cpp-gdb, cpp-lldb
    #[arg(short, long, value_name = "TYPE")]
    r#type: Vec<String>,

    /// Detect project type and add appropriate configurations
    #[arg(long)]
    detect: bool,

    /// Print detected project types without generating files
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Register all available config providers
    let providers: Vec<Box<dyn ConfigProvider>> = vec![
        Box::new(PythonConfigProvider),
        Box::new(PythonModuleConfigProvider),
        Box::new(FlaskConfigProvider),
        Box::new(FastApiConfigProvider),
        Box::new(JavaScriptConfigProvider),
        Box::new(NodeConfigProvider),
        Box::new(TypeScriptConfigProvider),
        Box::new(RustConfigProvider),
        Box::new(RustLibConfigProvider),
        Box::new(RustTestConfigProvider),
        Box::new(RustAllConfigProvider),
        Box::new(CppGdbConfigProvider),
        Box::new(CppLldbConfigProvider),
    ];

    // Create a map for quick lookup by name
    let provider_map: HashMap<&str, &Box<dyn ConfigProvider>> =
        providers.iter().map(|p| (p.name(), p)).collect();

    let mut configs: Vec<Value> = Vec::new();

    // If detect flag is set, detect project types
    let mut detected_types = Vec::new();
    if args.detect || args.dry_run {
        detected_types = detect_project_types(&providers)?;

        // Print detected project types
        println!("Detected project types:");
        if detected_types.is_empty() {
            println!("  No specific project types detected");
        } else {
            for project_type in &detected_types {
                println!("  - {}", project_type);
            }
        }

        // If dry run, exit after printing detected types
        if args.dry_run {
            return Ok(());
        }
    }

    // Process manually specified types
    for type_arg in &args.r#type {
        // Split by colon to handle parameterized types (e.g., python-module:django)
        let parts: Vec<&str> = type_arg.splitn(2, ':').collect();
        let type_name = parts[0];
        let param = parts.get(1).copied();

        if let Some(provider) = provider_map.get(type_name) {
            configs.push(provider.get_config(param));
        } else {
            eprintln!("Warning: Unknown configuration type: {}", type_name);
            eprintln!(
                "Available types: {}",
                providers
                    .iter()
                    .map(|p| p.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    // Add configurations for detected types if detect flag is set
    if args.detect {
        for type_name in detected_types {
            // Skip if we already added this type manually
            if args.r#type.iter().any(|t| t.starts_with(&type_name)) {
                continue;
            }

            // Special handling for python-module which might have parameters
            if type_name.starts_with("python-module:") {
                let parts: Vec<&str> = type_name.splitn(2, ':').collect();
                if let Some(provider) = provider_map.get("python-module") {
                    configs.push(provider.get_config(parts.get(1).copied()));
                }
            } else if let Some(provider) = provider_map.get(type_name.as_str()) {
                configs.push(provider.get_config(None));
            }
        }
    }

    // If no configurations were specified through flags or detection, exit early
    if configs.is_empty() {
        println!(
            "No configurations specified. Use --detect or specify configurations with --type."
        );
        println!(
            "Available types: {}",
            providers
                .iter()
                .map(|p| p.name())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }

    // Create launch.json file
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let vscode_dir = Path::new(".vscode");
            if !vscode_dir.exists() {
                fs::create_dir(vscode_dir)?;
            }
            vscode_dir.join("launch.json")
        }
    };

    create_launch_json(&configs, &output_path)?;
    println!("Created launch.json at {}", output_path.display());

    Ok(())
}

fn create_launch_json(
    configs: &[Value],
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let launch_config = json!({
        "version": "0.2.0",
        "configurations": configs
    });

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(output_path)?;
    let formatted = serde_json::to_string_pretty(&launch_config)?;
    file.write_all(formatted.as_bytes())?;

    Ok(())
}
