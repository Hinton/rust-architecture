use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Default document title when none is specified in config
pub(crate) const DEFAULT_TITLE: &str = "Architecture Documentation";

/// Configuration for the architecture documentation generator
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Config {
    /// Document title (default: "Architecture Documentation")
    pub title: Option<String>,

    /// Document description, rendered after the title
    pub description: Option<String>,

    /// Ordered list of category configurations
    pub categories: Vec<CategoryConfig>,
}

/// Configuration for a single category
#[derive(Debug, Deserialize, Clone)]
pub struct CategoryConfig {
    /// Category name as it appears in front matter (required)
    pub category: String,

    /// Display title for the category heading (defaults to `category`)
    pub title: Option<String>,

    /// Description rendered under the category heading
    pub description: Option<String>,
}

impl Config {
    /// Load config from a TOML file, returns default config if file doesn't exist
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))
    }

    /// Get the document title, with fallback to default
    pub(crate) fn title(&self) -> &str {
        self.title.as_deref().unwrap_or(DEFAULT_TITLE)
    }

    /// Get category config by name
    pub(crate) fn get_category(&self, name: &str) -> Option<&CategoryConfig> {
        self.categories.iter().find(|c| c.category == name)
    }

    /// Get display title for a category, falling back to the raw category name
    pub(crate) fn display_title_for<'a>(&'a self, category_name: &'a str) -> &'a str {
        self.get_category(category_name)
            .and_then(|c| c.title.as_deref())
            .unwrap_or(category_name)
    }

    /// Get ordered list of category names from config
    pub(crate) fn category_order(&self) -> Vec<&str> {
        self.categories
            .iter()
            .map(|c| c.category.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_from_str(toml: &str) -> Result<Config> {
        Ok(toml::from_str(toml)?)
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.title(), DEFAULT_TITLE);
        assert!(config.description.is_none());
        assert!(config.categories.is_empty());
    }

    #[test]
    fn test_config_from_toml_full() {
        let toml = r#"
title = "My Architecture"
description = "Overview of the system"

[[categories]]
category = "core"
title = "Core Systems"
description = "The foundation"

[[categories]]
category = "api"
"#;
        let config = config_from_str(toml).unwrap();
        assert_eq!(config.title(), "My Architecture");
        assert_eq!(
            config.description.as_deref(),
            Some("Overview of the system")
        );
        assert_eq!(config.categories.len(), 2);
        assert_eq!(config.categories[0].title.as_deref(), Some("Core Systems"));
        assert_eq!(config.categories[1].title, None);
    }

    #[test]
    fn test_config_from_toml_minimal() {
        let toml = r#"title = "Simple Title""#;
        let config = config_from_str(toml).unwrap();
        assert_eq!(config.title(), "Simple Title");
        assert!(config.categories.is_empty());
    }

    #[test]
    fn test_config_empty_toml() {
        let toml = "";
        let config = config_from_str(toml).unwrap();
        assert_eq!(config.title(), DEFAULT_TITLE);
    }

    #[test]
    fn test_config_invalid_toml() {
        let toml = "invalid = [";
        let result = config_from_str(toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_display_title_for_known_category() {
        let toml = r#"
[[categories]]
category = "core"
title = "Core Module"
"#;
        let config = config_from_str(toml).unwrap();
        assert_eq!(config.display_title_for("core"), "Core Module");
    }

    #[test]
    fn test_display_title_for_unknown_category() {
        let config = Config::default();
        assert_eq!(config.display_title_for("unknown"), "unknown");
    }

    #[test]
    fn test_category_order() {
        let toml = r#"
[[categories]]
category = "z-last"

[[categories]]
category = "a-first"
"#;
        let config = config_from_str(toml).unwrap();
        let order = config.category_order();
        assert_eq!(order, vec!["z-last", "a-first"]);
    }

    #[test]
    fn test_load_nonexistent_file_returns_default() {
        let config = Config::load(Path::new("/nonexistent/path/config.toml")).unwrap();
        assert_eq!(config.title(), DEFAULT_TITLE);
    }
}
