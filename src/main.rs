use anyhow::{Context, Result};
use argh::FromArgs;
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

use rust_architecture::{generate_document, parse_component};

#[derive(FromArgs)]
/// Generate architecture documentation from markdown files
struct Cli {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Commands {
    Generate(GenerateArgs),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "generate")]
/// Generate architecture documentation
struct GenerateArgs {
    #[argh(positional)]
    /// glob pattern to match markdown files (e.g., **/README.md)
    pattern: String,
    
    #[argh(positional)]
    /// output file path for the generated documentation
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();

    match cli.command {
        Commands::Generate(args) => {
            generate_architecture(&args.pattern, &args.output)?;
            println!("Architecture documentation generated at: {}", args.output.display());
        }
    }

    Ok(())
}

fn generate_architecture(pattern: &str, output: &Path) -> Result<()> {
    // Find all markdown files matching the pattern
    let files = find_markdown_files(pattern)?;
    
    // Determine the base directory from the pattern
    let base_dir = get_base_dir_from_pattern(pattern);
    
    // Parse each file and extract components
    let mut components = Vec::new();
    for file in files {
        if let Ok(component) = parse_component(file, &base_dir) {
            components.push(component);
        }
    }
    
    // Generate the documentation
    let doc = generate_document(&components);
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to output file
    fs::write(output, doc).context("Failed to write output file")?;
    
    Ok(())
}

fn find_markdown_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in glob(pattern).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => eprintln!("Error reading path: {}", e),
        }
    }
    
    Ok(files)
}

fn get_base_dir_from_pattern(pattern: &str) -> PathBuf {
    // Extract the base directory from the glob pattern
    // e.g., "/path/to/fixtures/**/README.md" -> "/path/to/fixtures/"
    let path = PathBuf::from(pattern);
    
    // Find the first component with wildcards
    let mut base = PathBuf::new();
    for component in path.components() {
        let comp_str = component.as_os_str().to_string_lossy();
        if comp_str.contains('*') || comp_str.contains('?') || comp_str.contains('[') {
            break;
        }
        base.push(component);
    }
    
    base
}

