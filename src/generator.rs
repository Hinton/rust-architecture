use crate::component::Component;
use std::collections::BTreeMap;

/// Generate architecture documentation from a list of components
pub fn generate_document(components: &[Component]) -> String {
    let mut doc = String::from("# Architecture Documentation\n");

    if components.is_empty() {
        return doc;
    }

    // Group components by category
    let mut categories: BTreeMap<String, Vec<&Component>> = BTreeMap::new();
    for component in components {
        categories
            .entry(component.category.clone())
            .or_default()
            .push(component);
    }

    // Generate output for each category
    for (category, comps) in categories {
        doc.push_str(&format!("\n## {}\n\n", category));

        for comp in comps {
            doc.push_str(&format!(
                "- `{}`: {}\n",
                comp.path.display(),
                comp.description
            ));
        }
    }

    doc
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_document_empty() {
        let components = vec![];
        let doc = generate_document(&components);
        assert_eq!(doc.trim(), "# Architecture Documentation");
    }

    #[test]
    fn test_generate_document_single_category() {
        let components = vec![Component {
            path: PathBuf::from("crates/core/README.md"),
            description: "Core utilities".to_string(),
            category: "Utilities".to_string(),
        }];

        let doc = generate_document(&components);
        assert!(doc.contains("# Architecture Documentation"));
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

        let doc = generate_document(&components);
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

        let doc = generate_document(&components);
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

        let doc = generate_document(&components);
        assert!(doc.contains("First"));
        assert!(doc.contains("Second"));
        let category_count = doc.matches("## Test").count();
        assert_eq!(
            category_count, 1,
            "Should only have one Test category header"
        );
    }
}
