//! # Ferrox Codex (`ferrox-codex`)
//!
//! Native WebAssembly documentation engine & static generator for the Ferrox ecosystem.
//! Replaces Docusaurus/Node.js with pure Rust WebAssembly dogfooding of `ferrox-front`.

pub mod highlighter;
pub mod markdown;
pub mod modules;
pub mod registry;
pub mod layout;
pub mod search;
pub mod showcase;
pub mod sync;
pub mod ssg;
pub mod i18n;
pub mod app;
pub mod cli;
pub mod sandboxes;
pub mod generated_ecosystem_docs;

pub use highlighter::*;
pub use markdown::*;
pub use modules::*;
pub use registry::*;
pub use layout::*;
pub use search::*;
pub use showcase::*;
pub use sync::*;
pub use ssg::*;
pub use i18n::*;
pub use app::*;
pub use sandboxes::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn codex_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
