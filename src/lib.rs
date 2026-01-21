mod component;
mod config;
mod front_matter;
mod generator;

pub use component::{parse_component, Component};
pub use config::{CategoryConfig, Config};
pub use generator::generate_document;
