use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::front_matter::{extract_front_matter, parse_front_matter};

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub path: PathBuf,
    pub description: String,
    pub category: String,
}

/// Parse a markdown file and extract component information
pub fn parse_component(path: PathBuf, base_dir: &Path) -> Result<Component> {
    let content = fs::read_to_string(&path)
        .context(format!("Failed to read file: {}", path.display()))?;
    
    let front_matter_str = extract_front_matter(&content)
        .context(format!("No front matter found in: {}", path.display()))?;
    
    let front_matter = parse_front_matter(front_matter_str)
        .context(format!("Failed to parse front matter in: {}", path.display()))?;
    
    // Make path relative to base_dir
    let relative_path = if let Ok(rel) = path.strip_prefix(base_dir) {
        rel.to_path_buf()
    } else {
        path.clone()
    };
    
    Ok(Component {
        path: relative_path,
        description: front_matter.description,
        category: front_matter.category,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

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
}
