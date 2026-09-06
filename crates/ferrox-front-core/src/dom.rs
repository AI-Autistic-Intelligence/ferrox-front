//! # DOM Builder Submodule (`ferrox-front-core::dom`)
//!
//! `dom` provides zero-copy WebAssembly DOM node construction and event listener bindings powered by `web_sys`.
//!
//! ## Core Primitives
//! - `DomBuilder`: Builder pattern struct wrapping a `web_sys::Node`.
//! - Element Helpers: `div()`, `h1()`, `h2()`, `h3()`, `p()`, `span()`, `button()`, `table()`.
//! - Lifecycle: `mount(selector, builder)` attaches the Wasm DOM tree to a target CSS selector (e.g. `"#root"`).
//!
//! ## Example Usage
//! ```rust,no_run
//! use ferrox_front_core::dom::{div, button, mount};
//!
//! let app = div()
//!     .attr("class", "app-wrapper")
//!     .child(
//!         button()
//!             .text("Submit")
//!             .on_click(|| web_sys::console::log_1(&"Clicked!".into()))
//!     );
//!
//! mount("#root", app);
//! ```

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, Node, Text};

/// Wraps a DOM Node to provide a Builder pattern for the UI.
pub struct DomBuilder {
    node: Node,
}

impl DomBuilder {
    pub fn new(tag: &str) -> Self {
        let doc = document();
        let element = doc.create_element(tag).expect("Failed to create element");
        Self { node: element.into() }
    }

    pub fn text_node(text: &str) -> Self {
        let doc = document();
        let text_node = doc.create_text_node(text);
        Self { node: text_node.into() }
    }

    /// Sets an attribute on the element (only if it is an Element).
    pub fn attr(self, name: &str, value: &str) -> Self {
        if let Some(element) = self.node.dyn_ref::<Element>() {
            element.set_attribute(name, value).unwrap();
        }
        self
    }

    /// Adds a click event listener.
    pub fn on_click<F>(self, mut callback: F) -> Self 
    where F: FnMut() + 'static {
        use wasm_bindgen::JsCast;
        let closure = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        
        if let Some(element) = self.node.dyn_ref::<web_sys::EventTarget>() {
            element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget(); // Leak per semplicità nel mock
        }
        self
    }

    /// Adds a text node child.
    pub fn text(self, text: &str) -> Self {
        let text_node = document().create_text_node(text);
        self.node.append_child(text_node.as_ref()).unwrap();
        self
    }

    /// Appends another DomBuilder as a child.
    pub fn child(self, child: DomBuilder) -> Self {
        self.node.append_child(&child.node).unwrap();
        self
    }

    /// Extracts the raw web_sys::Node.
    pub fn build(self) -> Node {
        self.node
    }
}

/// Helper to get the global document.
pub fn document() -> Document {
    window().unwrap().document().unwrap()
}

/// Helper functions to quickly spawn builders.
pub fn div() -> DomBuilder { DomBuilder::new("div") }
pub fn h1() -> DomBuilder { DomBuilder::new("h1") }
pub fn h2() -> DomBuilder { DomBuilder::new("h2") }
pub fn h3() -> DomBuilder { DomBuilder::new("h3") }
pub fn p() -> DomBuilder { DomBuilder::new("p") }
pub fn span() -> DomBuilder { DomBuilder::new("span") }
pub fn button() -> DomBuilder { DomBuilder::new("button") }
pub fn table() -> DomBuilder { DomBuilder::new("table") }

/// Mounts an element to a selector (e.g. "#root")
pub fn mount(selector: &str, builder: DomBuilder) {
    let doc = document();
    let root = doc.query_selector(selector)
        .expect("Failed to parse selector")
        .expect("Element not found");
    
    root.append_child(&builder.build()).expect("Failed to mount app");
}