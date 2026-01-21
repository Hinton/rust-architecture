//! Markdown document generation from parsed components.
//!
//! This module transforms a collection of [`Component`]s into a formatted
//! markdown document, grouping components by category and applying
//! configuration for titles, descriptions, and ordering.

use itertools::Itertools;

use crate::component::Component;
use crate::config::Config;
use std::collections::HashMap;
use std::fmt::Write;

/// Generates architecture documentation from a list of components.
///
/// Produces a markdown document with:
/// - A title (from config or default)
/// - An optional document description
/// - Sections for each category, containing component entries
///
/// Categories are ordered according to the config, with any unlisted
/// categories appended alphabetically. Components within each category
/// are sorted by path.
pub fn generate_document(components: &[Component], config: &Config) -> String {
    let mut doc = format!("# {}\n", config.title());

    // Add document description if present
    if let Some(desc) = &config.description {
        writeln!(doc, "\n{}", desc.trim_end()).unwrap();
    }

    if components.is_empty() {
        return doc;
    }

    let grouped = group_by_category(components);
    let ordered_categories = order_categories(&grouped, config);

    // Generate output for each category
    for category_name in ordered_categories {
        if let Some(comps) = grouped.get(category_name) {
            // Get display title from config or use raw category name
            let display_title = config.display_title_for(category_name);
            writeln!(doc, "\n## {}", display_title).unwrap();

            // Add category description if present in config
            if let Some(desc) = config
                .get_category(category_name)
                .and_then(|c| c.description.as_deref())
            {
                writeln!(doc, "\n{}", desc.trim_end()).unwrap();
            }

            doc.push('\n');
            for comp in comps {
                writeln!(doc, "- `{}`: {}", comp.path.display(), comp.description).unwrap();
            }
        }
    }

    doc
}

/// Groups components by category, sorting by path within each group.
fn group_by_category(components: &[Component]) -> HashMap<String, Vec<&Component>> {
    let mut grouped: HashMap<String, Vec<&Component>> =
        components.iter().into_group_map_by(|c| c.category.clone());

    for comps in grouped.values_mut() {
        comps.sort_by_key(|c| &c.path);
    }

    grouped
}

/// Orders categories, config-specified order first, then remaining alphabetically.
fn order_categories<'a>(
    grouped: &'a HashMap<String, Vec<&Component>>,
    config: &'a Config,
) -> Vec<&'a str> {
    let config_order = config.category_order();

    let mut result: Vec<&str> = config_order
        .iter()
        .copied()
        .filter(|name| grouped.contains_key(*name))
        .collect();

    let mut remaining: Vec<_> = grouped
        .keys()
        .map(String::as_str)
        .filter(|name| !config_order.contains(name))
        .collect();
    remaining.sort_unstable();

    result.extend(remaining);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_TITLE;
    use std::path::PathBuf;

    fn config_from_str(toml: &str) -> Config {
        toml::from_str(toml).unwrap()
    }

    #[test]
    fn test_generate_document_empty() {
        let components = vec![];
        let doc = generate_document(&components, &Config::default());
        assert_eq!(doc.trim(), format!("# {}", DEFAULT_TITLE));
    }

    #[test]
    fn test_generate_document_single_category() {
        let components = vec![Component {
            path: PathBuf::from("crates/core/README.md"),
            description: "Core utilities".to_string(),
            category: "Utilities".to_string(),
        }];

        let doc = generate_document(&components, &Config::default());
        assert!(doc.contains(&format!("# {}", DEFAULT_TITLE)));
        assert!(doc.contains("## Utilities"));
        assert!(doc.contains("crates/core"));
        assert!(doc.contains("Core utilities"));
    }

    #[test]
    fn test_generate_document_multiple_categories() {
        let components = vec![
            Component {
                path: PathBuf::from("crates/core/README.md"),
                description: "Core utilities".to_string(),
                category: "Utilities".to_string(),
            },
            Component {
                path: PathBuf::from("crates/cli/README.md"),
                description: "CLI interface".to_string(),
                category: "Interfaces".to_string(),
            },
            Component {
                path: PathBuf::from("crates/helpers/README.md"),
                description: "Helper functions".to_string(),
                category: "Utilities".to_string(),
            },
        ];

        let doc = generate_document(&components, &Config::default());
        assert!(doc.contains("## Utilities"));
        assert!(doc.contains("## Interfaces"));
        assert!(doc.contains("crates/core"));
        assert!(doc.contains("crates/cli"));
        assert!(doc.contains("crates/helpers"));
    }

    #[test]
    fn test_generate_document_sorted_categories() {
        let components = vec![
            Component {
                path: PathBuf::from("crates/cli/README.md"),
                description: "CLI interface".to_string(),
                category: "Interfaces".to_string(),
            },
            Component {
                path: PathBuf::from("crates/core/README.md"),
                description: "Core utilities".to_string(),
                category: "Utilities".to_string(),
            },
        ];

        let doc = generate_document(&components, &Config::default());
        let interfaces_pos = doc.find("## Interfaces").unwrap();
        let utilities_pos = doc.find("## Utilities").unwrap();
        // Categories should be sorted alphabetically
        assert!(interfaces_pos < utilities_pos);
    }

    #[test]
    fn test_generate_document_multiple_components_same_category() {
        let components = vec![
            Component {
                path: PathBuf::from("a/README.md"),
                description: "First".to_string(),
                category: "Test".to_string(),
            },
            Component {
                path: PathBuf::from("b/README.md"),
                description: "Second".to_string(),
                category: "Test".to_string(),
            },
        ];

        let doc = generate_document(&components, &Config::default());
        assert!(doc.contains("First"));
        assert!(doc.contains("Second"));
        let category_count = doc.matches("## Test").count();
        assert_eq!(
            category_count, 1,
            "Should only have one Test category header"
        );
    }

    #[test]
    fn test_generate_document_with_custom_title() {
        let config = config_from_str(r#"title = "Custom Title""#);
        let components = vec![];
        let doc = generate_document(&components, &config);
        assert!(doc.starts_with("# Custom Title"));
    }

    #[test]
    fn test_generate_document_with_description() {
        let config = config_from_str(
            r#"
title = "Arch Doc"
description = "This is the description."
"#,
        );
        let components = vec![];
        let doc = generate_document(&components, &config);
        assert!(doc.contains("This is the description."));
    }

    #[test]
    fn test_generate_document_category_ordering() {
        let config = config_from_str(
            r#"
[[categories]]
category = "Utilities"

[[categories]]
category = "Interfaces"
"#,
        );

        let components = vec![
            Component {
                path: PathBuf::from("cli/README.md"),
                description: "CLI".to_string(),
                category: "Interfaces".to_string(),
            },
            Component {
                path: PathBuf::from("core/README.md"),
                description: "Core".to_string(),
                category: "Utilities".to_string(),
            },
        ];

        let doc = generate_document(&components, &config);
        let utilities_pos = doc.find("## Utilities").unwrap();
        let interfaces_pos = doc.find("## Interfaces").unwrap();
        // Config order: Utilities before Interfaces
        assert!(utilities_pos < interfaces_pos);
    }

    #[test]
    fn test_generate_document_unlisted_categories_appended() {
        let config = config_from_str(
            r#"
[[categories]]
category = "First"
"#,
        );

        let components = vec![
            Component {
                path: PathBuf::from("a/README.md"),
                description: "A".to_string(),
                category: "First".to_string(),
            },
            Component {
                path: PathBuf::from("b/README.md"),
                description: "B".to_string(),
                category: "ZUnlisted".to_string(),
            },
            Component {
                path: PathBuf::from("c/README.md"),
                description: "C".to_string(),
                category: "AUnlisted".to_string(),
            },
        ];

        let doc = generate_document(&components, &config);
        let first_pos = doc.find("## First").unwrap();
        let a_unlisted_pos = doc.find("## AUnlisted").unwrap();
        let z_unlisted_pos = doc.find("## ZUnlisted").unwrap();

        // First from config, then unlisted alphabetically
        assert!(first_pos < a_unlisted_pos);
        assert!(a_unlisted_pos < z_unlisted_pos);
    }

    #[test]
    fn test_generate_document_category_display_title() {
        let config = config_from_str(
            r#"
[[categories]]
category = "utils"
title = "Utility Functions"
"#,
        );

        let components = vec![Component {
            path: PathBuf::from("utils/README.md"),
            description: "Utils".to_string(),
            category: "utils".to_string(),
        }];

        let doc = generate_document(&components, &config);
        assert!(doc.contains("## Utility Functions"));
        assert!(!doc.contains("## utils"));
    }

    #[test]
    fn test_generate_document_category_description() {
        let config = config_from_str(
            r#"
[[categories]]
category = "core"
description = "These are the core components."
"#,
        );

        let components = vec![Component {
            path: PathBuf::from("core/README.md"),
            description: "Core lib".to_string(),
            category: "core".to_string(),
        }];

        let doc = generate_document(&components, &config);
        assert!(doc.contains("These are the core components."));
    }
}
