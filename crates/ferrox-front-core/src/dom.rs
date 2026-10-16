//! # DOM Builder Submodule (`ferrox-front-core::dom`)
//!
//! `dom` provides zero-copy WebAssembly DOM node construction and event listener bindings powered by `web_sys`.
//!
//! ## Core Primitives
//! - `DomBuilder`: Builder pattern struct wrapping a `web_sys::Node`.
//! - Element Helpers: `div()`, `h1()`, `p()`, `span()`, `button()`, `input()`, `svg()`, `path()`.
//! - Lifecycle: `mount(selector, builder)` attaches the Wasm DOM tree to a target CSS selector (e.g. `"#root"`).

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, Node};

/// Wraps a DOM Node to provide a Builder pattern for the UI.
pub struct DomBuilder {
    pub node: Node,
}

impl DomBuilder {
    pub fn new(tag: &str) -> Self {
        let doc = document();
        let element = doc.create_element(tag).expect("Failed to create element");
        Self { node: element.into() }
    }

    pub fn new_svg(tag: &str) -> Self {
        let doc = document();
        let element = doc.create_element_ns(Some("http://www.w3.org/2000/svg"), tag).expect("Failed to create SVG element");
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

    /// Convenience helper for setting the class attribute.
    pub fn class(self, class_name: &str) -> Self {
        self.attr("class", class_name)
    }

    /// Convenience helper for setting the id attribute.
    pub fn id(self, id_str: &str) -> Self {
        self.attr("id", id_str)
    }

    /// Convenience helper for setting inline style attribute.
    pub fn style_attr(self, style_str: &str) -> Self {
        self.attr("style", style_str)
    }

    /// Sets the inner HTML of the element directly.
    pub fn inner_html(self, html: &str) -> Self {
        if let Some(element) = self.node.dyn_ref::<Element>() {
            element.set_inner_html(html);
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
            closure.forget();
        }
        self
    }

    /// Adds a generic event listener with Event payload.
    pub fn on<F>(self, event_name: &str, mut callback: F) -> Self
    where F: FnMut(web_sys::Event) + 'static {
        use wasm_bindgen::JsCast;
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            callback(e);
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        if let Some(element) = self.node.dyn_ref::<web_sys::EventTarget>() {
            element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }
        self
    }

    /// Adds an input event listener (commonly used on text fields, sliders, etc.).
    pub fn on_input<F>(self, callback: F) -> Self
    where F: FnMut(web_sys::Event) + 'static {
        self.on("input", callback)
    }

    /// Adds a change event listener (commonly used on selects, checkboxes, radios).
    pub fn on_change<F>(self, callback: F) -> Self
    where F: FnMut(web_sys::Event) + 'static {
        self.on("change", callback)
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

    /// Appends a collection of DomBuilders as children.
    pub fn children(mut self, children: Vec<DomBuilder>) -> Self {
        for c in children {
            self = self.child(c);
        }
        self
    }

    /// Extracts the raw web_sys::Node.
    pub fn build(self) -> Node {
        self.node
    }

    /// Creates a DomBuilder from an existing Node
    pub fn from_node(node: Node) -> Self {
        Self { node }
    }
}

/// Helper to get the global document.
pub fn document() -> Document {
    window().unwrap().document().unwrap()
}

/// Helper to get an element by its ID.
pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

/// Helper functions to quickly spawn builders.
pub fn div() -> DomBuilder { DomBuilder::new("div") }
pub fn h1() -> DomBuilder { DomBuilder::new("h1") }
pub fn h2() -> DomBuilder { DomBuilder::new("h2") }
pub fn h3() -> DomBuilder { DomBuilder::new("h3") }
pub fn h4() -> DomBuilder { DomBuilder::new("h4") }
pub fn h5() -> DomBuilder { DomBuilder::new("h5") }
pub fn h6() -> DomBuilder { DomBuilder::new("h6") }
pub fn p() -> DomBuilder { DomBuilder::new("p") }
pub fn span() -> DomBuilder { DomBuilder::new("span") }
pub fn button() -> DomBuilder { DomBuilder::new("button") }
pub fn table() -> DomBuilder { DomBuilder::new("table") }
pub fn thead() -> DomBuilder { DomBuilder::new("thead") }
pub fn tbody() -> DomBuilder { DomBuilder::new("tbody") }
pub fn tr() -> DomBuilder { DomBuilder::new("tr") }
pub fn th() -> DomBuilder { DomBuilder::new("th") }
pub fn td() -> DomBuilder { DomBuilder::new("td") }
pub fn style() -> DomBuilder { DomBuilder::new("style") }
pub fn input() -> DomBuilder { DomBuilder::new("input") }
pub fn textarea() -> DomBuilder { DomBuilder::new("textarea") }
pub fn label() -> DomBuilder { DomBuilder::new("label") }
pub fn select() -> DomBuilder { DomBuilder::new("select") }
pub fn option() -> DomBuilder { DomBuilder::new("option") }
pub fn nav() -> DomBuilder { DomBuilder::new("nav") }
pub fn header() -> DomBuilder { DomBuilder::new("header") }
pub fn footer() -> DomBuilder { DomBuilder::new("footer") }
pub fn section() -> DomBuilder { DomBuilder::new("section") }
pub fn aside() -> DomBuilder { DomBuilder::new("aside") }
pub fn main_elem() -> DomBuilder { DomBuilder::new("main") }
pub fn ul() -> DomBuilder { DomBuilder::new("ul") }
pub fn ol() -> DomBuilder { DomBuilder::new("ol") }
pub fn li() -> DomBuilder { DomBuilder::new("li") }
pub fn a() -> DomBuilder { DomBuilder::new("a") }
pub fn img() -> DomBuilder { DomBuilder::new("img") }
pub fn form() -> DomBuilder { DomBuilder::new("form") }

// SVG Elements
pub fn svg() -> DomBuilder { DomBuilder::new_svg("svg") }
pub fn path() -> DomBuilder { DomBuilder::new_svg("path") }
pub fn circle() -> DomBuilder { DomBuilder::new_svg("circle") }
pub fn rect() -> DomBuilder { DomBuilder::new_svg("rect") }
pub fn line() -> DomBuilder { DomBuilder::new_svg("line") }
pub fn polyline() -> DomBuilder { DomBuilder::new_svg("polyline") }
pub fn polygon() -> DomBuilder { DomBuilder::new_svg("polygon") }
pub fn g() -> DomBuilder { DomBuilder::new_svg("g") }
pub fn text_svg() -> DomBuilder { DomBuilder::new_svg("text") }
pub fn defs() -> DomBuilder { DomBuilder::new_svg("defs") }
pub fn linear_gradient() -> DomBuilder { DomBuilder::new_svg("linearGradient") }
pub fn stop() -> DomBuilder { DomBuilder::new_svg("stop") }

/// Mounts an element to a selector (e.g. "#root"), clearing existing children first
pub fn mount(selector: &str, builder: DomBuilder) {
    let doc = document();
    let root = doc.query_selector(selector)
        .expect("Failed to parse selector")
        .expect("Element not found");
    
    root.set_inner_html("");
    root.append_child(&builder.build()).expect("Failed to mount app");
}