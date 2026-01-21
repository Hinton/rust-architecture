use anyhow::{Context, Result};
use argh::FromArgs;
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

use rust_architecture::{generate_document, parse_component, Config};

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

    #[argh(option, short = 'c')]
    /// path to config file (default: architecture.toml in current directory)
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();

    match cli.command {
        Commands::Generate(args) => {
            generate_architecture(&args.pattern, &args.output, args.config.as_deref())?;
            println!(
                "Architecture documentation generated at: {}",
                args.output.display()
            );
        }
    }

    Ok(())
}

fn generate_architecture(pattern: &str, output: &Path, config_path: Option<&Path>) -> Result<()> {
    // Load config (use default if not specified or doesn't exist)
    let config_file = config_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("architecture.toml"));

    let config = Config::load(&config_file)?;

    let files = find_markdown_files(pattern)?;
    let base_dir = get_base_dir_from_pattern(pattern);

    let mut components = Vec::new();
    for file in files {
        if let Ok(component) = parse_component(file, &base_dir) {
            components.push(component);
        }
    }

    let doc = generate_document(&components, &config);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

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
