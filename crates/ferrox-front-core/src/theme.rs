//! # Glassmorphism Theme Submodule (`ferrox-front-core::theme`)
//!
//! `theme` manages runtime theme switching at 60fps by mutating the `data-theme` attribute on `document.documentElement`.
//!
//! ## Functions
//! - `set_theme(name)`: Switches active theme (`"ferrox-cyber"`, `"ocean-breeze"`, `"midnight-forest"`, `"sunset-gold"`, `"corporate-slate"`).
//! - `get_theme()`: Retrieves currently active theme string.

use wasm_bindgen::prelude::*;
use web_sys::window;

/// Sets the application theme by updating the data-theme attribute on the document element (<html>)
pub fn set_theme(theme_name: &str) {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(doc_element) = doc.document_element() {
                doc_element.set_attribute("data-theme", theme_name).unwrap();
            }
        }
    }
}

/// Returns the currently active theme name, defaulting to "ferrox-cyber"
pub fn get_theme() -> String {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(doc_element) = doc.document_element() {
                return doc_element.get_attribute("data-theme").unwrap_or_else(|| "ferrox-cyber".to_string());
            }
        }
    }
    "ferrox-cyber".to_string()
}