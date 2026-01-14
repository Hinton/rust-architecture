use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FrontMatter {
    pub description: String,
    pub category: String,
}

/// Extract YAML front matter from markdown content
/// Returns the front matter content between the --- delimiters
pub fn extract_front_matter(content: &str) -> Option<&str> {
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() || !lines[0].trim().starts_with("---") {
        return None;
    }
    
    // Find the closing ---
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            // Extract content between the two ---
            return Some(&content[lines[0].len() + 1..lines[0..=i].join("\n").len() - 3]);
        }
    }
    
    None
}

/// Parse YAML front matter into FrontMatter struct
pub fn parse_front_matter(yaml: &str) -> anyhow::Result<FrontMatter> {
    Ok(serde_yaml::from_str(yaml)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_front_matter_with_valid_yaml() {
        let content = r#"---
description: "Core utilities"
category: "Utilities"
---

# Header
Some content"#;
        
        let front_matter = extract_front_matter(content);
        assert!(front_matter.is_some());
        let fm = front_matter.unwrap();
        assert!(fm.contains("description"));
        assert!(fm.contains("Core utilities"));
    }

    #[test]
    fn test_extract_front_matter_without_delimiters() {
        let content = r#"# Header
Some content without front matter"#;
        
        let front_matter = extract_front_matter(content);
        assert!(front_matter.is_none());
    }

    #[test]
    fn test_extract_front_matter_empty_file() {
        let content = "";
        let front_matter = extract_front_matter(content);
        assert!(front_matter.is_none());
    }

    #[test]
    fn test_extract_front_matter_only_opening_delimiter() {
        let content = r#"---
description: "Test"
No closing delimiter"#;
        
        let front_matter = extract_front_matter(content);
        assert!(front_matter.is_none());
    }

    #[test]
    fn test_parse_front_matter_valid() {
        let yaml = r#"description: "Core utilities for the project"
category: "Utilities""#;
        
        let result = parse_front_matter(yaml);
        assert!(result.is_ok());
        
        let front_matter = result.unwrap();
        assert_eq!(front_matter.description, "Core utilities for the project");
        assert_eq!(front_matter.category, "Utilities");
    }

    #[test]
    fn test_parse_front_matter_missing_description() {
        let yaml = r#"category: "Utilities""#;
        
        let result = parse_front_matter(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_front_matter_missing_category() {
        let yaml = r#"description: "Core utilities""#;
        
        let result = parse_front_matter(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_front_matter_invalid_yaml() {
        let yaml = r#"this is not valid yaml: ["#;
        
        let result = parse_front_matter(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_front_matter_with_special_chars() {
        let yaml = r#"description: "Parser with **markdown** and `code` formatting"
category: "Utilities""#;
        
        let result = parse_front_matter(yaml);
        assert!(result.is_ok());
        
        let front_matter = result.unwrap();
        assert_eq!(front_matter.description, "Parser with **markdown** and `code` formatting");
    }
}
