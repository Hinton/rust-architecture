use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FrontMatter {
    pub description: Option<String>,
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

/// Extract the first paragraph after the title from markdown content.
/// Skips the front matter (if present) and the first heading, then returns
/// the first non-empty paragraph.
pub fn extract_first_paragraph(content: &str) -> Option<String> {
    let mut lines = content.lines().peekable();

    // Skip front matter if present
    if lines.peek().map(|l| l.trim()) == Some("---") {
        lines.next();
        for line in lines.by_ref() {
            if line.trim() == "---" {
                break;
            }
        }
    }

    // Skip any blank lines and the first heading
    let mut found_heading = false;
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') {
            found_heading = true;
            break;
        }
        // If we hit non-heading content before a heading, use it
        if !trimmed.is_empty() {
            // Collect this paragraph
            let mut paragraph = String::from(trimmed);
            for next_line in lines {
                let next_trimmed = next_line.trim();
                if next_trimmed.is_empty() {
                    break;
                }
                paragraph.push(' ');
                paragraph.push_str(next_trimmed);
            }
            return Some(paragraph);
        }
    }

    if !found_heading {
        return None;
    }

    // Now find the first paragraph after the heading
    let mut paragraph_lines: Vec<&str> = Vec::new();
    let mut in_paragraph = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if in_paragraph {
                break;
            }
            continue;
        }

        // Skip other headings
        if trimmed.starts_with('#') {
            if in_paragraph {
                break;
            }
            continue;
        }

        in_paragraph = true;
        paragraph_lines.push(trimmed);
    }

    if paragraph_lines.is_empty() {
        None
    } else {
        Some(paragraph_lines.join(" "))
    }
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
        assert_eq!(front_matter.description, Some("Core utilities for the project".to_string()));
        assert_eq!(front_matter.category, "Utilities");
    }

    #[test]
    fn test_parse_front_matter_missing_description() {
        let yaml = r#"category: "Utilities""#;

        let result = parse_front_matter(yaml);
        assert!(result.is_ok());
        let front_matter = result.unwrap();
        assert_eq!(front_matter.description, None);
        assert_eq!(front_matter.category, "Utilities");
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
        assert_eq!(
            front_matter.description,
            Some("Parser with **markdown** and `code` formatting".to_string())
        );
    }

    #[test]
    fn test_extract_first_paragraph_simple() {
        let content = r#"# Title

This is the first paragraph.

This is the second paragraph."#;

        let result = extract_first_paragraph(content);
        assert_eq!(result, Some("This is the first paragraph.".to_string()));
    }

    #[test]
    fn test_extract_first_paragraph_with_front_matter() {
        let content = r#"---
category: "Test"
---

# Title

This is the description from the content.

More content here."#;

        let result = extract_first_paragraph(content);
        assert_eq!(result, Some("This is the description from the content.".to_string()));
    }

    #[test]
    fn test_extract_first_paragraph_multiline() {
        let content = r#"# Title

This is a paragraph that spans
multiple lines without
a blank line break.

This is the second paragraph."#;

        let result = extract_first_paragraph(content);
        assert_eq!(
            result,
            Some("This is a paragraph that spans multiple lines without a blank line break.".to_string())
        );
    }

    #[test]
    fn test_extract_first_paragraph_no_content() {
        let content = r#"# Title"#;

        let result = extract_first_paragraph(content);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_first_paragraph_only_headings() {
        let content = r#"# Title

## Section 1

## Section 2"#;

        let result = extract_first_paragraph(content);
        assert_eq!(result, None);
    }
}
