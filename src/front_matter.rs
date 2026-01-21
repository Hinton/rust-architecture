//! YAML front matter extraction and parsing.
//!
//! This module provides utilities for extracting and parsing YAML front matter
//! from markdown files, as well as fallback extraction of the first paragraph.

use serde::Deserialize;

/// Parsed YAML front matter from a markdown file.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) struct FrontMatter {
    /// Optional description of the component.
    pub description: Option<String>,
    /// Required category for grouping in the output.
    pub category: String,
}

/// Extracts YAML front matter from markdown content.
///
/// Looks for content between `---` delimiters at the start of the file.
/// Returns `None` if no valid front matter block is found.
///
/// # Example
///
/// ```text
/// ---
/// category: "Utils"
/// ---
/// # Content here
/// ```
pub(crate) fn extract_front_matter(content: &str) -> Option<&str> {
    let content = content.strip_prefix("---")?;
    let content = content.strip_prefix(['\n', '\r'])?;
    let end = content.find("\n---").or_else(|| content.find("\r\n---"))?;
    Some(&content[..end])
}

/// Parses a YAML string into a [`FrontMatter`] struct.
///
/// # Errors
///
/// Returns an error if the YAML is invalid or missing required fields.
pub(crate) fn parse_front_matter(yaml: &str) -> anyhow::Result<FrontMatter> {
    Ok(serde_yaml::from_str(yaml)?)
}

/// Extracts the first paragraph after the title from markdown content.
///
/// Skips front matter (if present) and any headings, then returns the first
/// non-empty paragraph. Multi-line paragraphs are joined with spaces.
///
/// Returns `None` if no paragraph content is found.
pub(crate) fn extract_first_paragraph(content: &str) -> Option<String> {
    let mut lines = content.lines().peekable();

    // Skip front matter if present
    if lines.peek().is_some_and(|l| l.trim() == "---") {
        lines.next();
        lines.find(|line| line.trim() == "---");
    }

    // Skip blank lines and headings until we find paragraph content
    let first_para_line = lines
        .by_ref()
        .find(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))?;

    // Collect contiguous non-empty lines into a paragraph
    let mut paragraph = String::from(first_para_line.trim());
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            break;
        }
        paragraph.push(' ');
        paragraph.push_str(trimmed);
    }

    Some(paragraph)
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
        assert_eq!(
            front_matter.description,
            Some("Core utilities for the project".to_string())
        );
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
        assert_eq!(
            result,
            Some("This is the description from the content.".to_string())
        );
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
            Some(
                "This is a paragraph that spans multiple lines without a blank line break."
                    .to_string()
            )
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
