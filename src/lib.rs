pub mod component;
pub mod front_matter;
pub mod generator;

pub use component::{Component, parse_component};
pub use front_matter::{extract_front_matter, parse_front_matter, FrontMatter};
pub use generator::generate_document;
