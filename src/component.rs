//! Component parsing and representation.
//!
//! This module handles parsing markdown files with YAML front matter
//! into structured `Component` data used for architecture documentation.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::front_matter::{extract_first_paragraph, extract_front_matter, parse_front_matter};

/// A parsed component from a markdown README file.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// Path to the component's README, relative to the base directory.
    pub path: PathBuf,
    /// Description extracted from front matter or the first paragraph.
    pub description: String,
    /// Category for grouping components in the output.
    pub category: String,
}

/// Parses a markdown file and extracts component information.
///
/// The file must contain YAML front matter with at least a `category` field.
/// The description is taken from the front matter `description` field if present,
/// otherwise falls back to the first paragraph after the front matter.
///
/// # Arguments
///
/// * `path` - Absolute path to the markdown file
/// * `base_dir` - Base directory used to compute the relative path
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - No front matter is found
/// - Front matter is invalid YAML
/// - No description is found in front matter or content
pub fn parse_component(path: PathBuf, base_dir: &Path) -> Result<Component> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let front_matter_str = extract_front_matter(&content)
        .with_context(|| format!("No front matter found in: {}", path.display()))?;

    let front_matter = parse_front_matter(front_matter_str)
        .with_context(|| format!("Failed to parse front matter in: {}", path.display()))?;

    // Use front matter description, or fall back to first paragraph
    let description = front_matter
        .description
        .or_else(|| extract_first_paragraph(&content))
        .with_context(|| {
            format!(
                "No description found in front matter or content: {}",
                path.display()
            )
        })?;

    // Make path relative to base_dir
    let relative_path = path
        .strip_prefix(base_dir)
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|_| path.clone());

    Ok(Component {
        path: relative_path,
        description,
        category: front_matter.category,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_parse_component_valid_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_component.md");

        let content = r#"---
description: "Test component"
category: "Testing"
---

# Test Component"#;

        fs::write(&test_file, content).unwrap();

        let result = parse_component(test_file.clone(), &temp_dir);
        assert!(result.is_ok());

        let component = result.unwrap();
        assert_eq!(component.description, "Test component");
        assert_eq!(component.category, "Testing");
        assert_eq!(component.path, PathBuf::from("test_component.md"));

        fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_parse_component_missing_front_matter() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_no_front_matter.md");

        let content = "# Just a header\nNo front matter here";
        fs::write(&test_file, content).unwrap();

        let result = parse_component(test_file.clone(), &temp_dir);
        assert!(result.is_err());

        fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_parse_component_preserves_relative_path() {
        let temp_dir = env::temp_dir();
        let nested_dir = temp_dir.join("nested").join("path");
        fs::create_dir_all(&nested_dir).ok();
        let test_file = nested_dir.join("test.md");

        let content = r#"---
description: "Nested component"
category: "Test"
---

# Nested"#;

        fs::write(&test_file, content).unwrap();

        let result = parse_component(test_file.clone(), &temp_dir);
        assert!(result.is_ok());

        let component = result.unwrap();
        assert_eq!(component.path, PathBuf::from("nested/path/test.md"));

        fs::remove_dir_all(temp_dir.join("nested")).ok();
    }

    #[test]
    fn test_parse_component_nonexistent_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("nonexistent.md");

        let result = parse_component(test_file, &temp_dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_component_description_from_first_paragraph() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_paragraph_fallback.md");

        let content = r#"---
category: "Testing"
---

# Test Component

This description comes from the first paragraph.

More content here."#;

        fs::write(&test_file, content).unwrap();

        let result = parse_component(test_file.clone(), &temp_dir);
        assert!(result.is_ok());

        let component = result.unwrap();
        assert_eq!(
            component.description,
            "This description comes from the first paragraph."
        );
        assert_eq!(component.category, "Testing");

        fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_parse_component_prefers_front_matter_description() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_prefer_front_matter.md");

        let content = r#"---
description: "From front matter"
category: "Testing"
---

# Test Component

This paragraph should be ignored."#;

        fs::write(&test_file, content).unwrap();

        let result = parse_component(test_file.clone(), &temp_dir);
        assert!(result.is_ok());

        let component = result.unwrap();
        assert_eq!(component.description, "From front matter");

        fs::remove_file(&test_file).ok();
    }
}
