//! # Microfrontend Module System (`ferrox-codex::modules`)
//!
//! Encapsulates independent ecosystem repositories into plug-and-play `CodexModule`s.
//! Supports recursive submodules (e.g. `ferrox-py` -> `auth`, `commerce`, `utils`),
//! modular navigation trees, and unified full-text search index generation.

use std::collections::HashMap;
use ferrox_front_ui::DocNavItem;
use crate::markdown::{parse_markdown, ParsedDoc};

pub mod core;
pub mod front;
pub mod node;
pub mod python;
pub mod polyglot;

pub use core::create_core_module;
pub use front::create_front_module;
pub use node::create_node_module;
pub use python::create_python_module;
pub use polyglot::create_polyglot_module;

pub type InteractiveViewFn = std::rc::Rc<dyn Fn() -> ferrox_front_core::dom::DomBuilder>;

/// Microfrontend documentation module encapsulating a specific ecosystem repository or area.
#[derive(Clone)]
pub struct CodexModule {
    pub id: String,
    pub title: String,
    pub route_prefix: String,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub docs: HashMap<String, ParsedDoc>,
    pub interactive_views: HashMap<String, InteractiveViewFn>,
    pub submodules: Vec<CodexModule>,
    pub ordered_routes: Vec<String>,
}

impl CodexModule {
    /// Creates a new microfrontend module with a unique ID, title, and base route prefix.
    pub fn new(id: impl Into<String>, title: impl Into<String>, route_prefix: impl Into<String>) -> Self {
        let p = route_prefix.into();
        Self {
            id: id.into(),
            title: title.into(),
            route_prefix: if p.starts_with('/') { p } else { format!("/{}", p) },
            icon: None,
            badge: None,
            docs: HashMap::new(),
            interactive_views: HashMap::new(),
            submodules: Vec::new(),
            ordered_routes: Vec::new(),
        }
    }

    /// Sets the icon name for this module in navigation.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the badge text for this module in navigation.
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Attaches an interactive live Wasm view/playground to this module.
    pub fn with_interactive_view(
        mut self,
        path: impl Into<String>,
        view: impl Fn() -> ferrox_front_core::dom::DomBuilder + 'static,
    ) -> Self {
        let clean = path.into().trim_matches('/').to_string();
        self.interactive_views.insert(clean, std::rc::Rc::new(view));
        self
    }

    /// Attaches a nested submodule (recursive microfrontend).
    pub fn with_submodule(mut self, sub: CodexModule) -> Self {
        self.submodules.push(sub);
        self
    }

    /// Registers a raw markdown document into this module.
    pub fn register_doc(&mut self, relative_path: &str, raw_markdown: &str) {
        let clean = relative_path.trim_matches('/').to_string();
        let full_doc_path = if clean.is_empty() || clean == "overview" {
            self.route_prefix.trim_matches('/').to_string()
        } else {
            format!("{}/{}", self.route_prefix.trim_matches('/'), clean)
        };

        let parsed = parse_markdown(&full_doc_path, raw_markdown);
        if !self.ordered_routes.contains(&clean) {
            self.ordered_routes.push(clean.clone());
        }
        self.docs.insert(clean, parsed);
    }

    /// Directly attaches a pre-parsed document into this module.
    pub fn add_parsed_doc(&mut self, relative_path: &str, doc: ParsedDoc) {
        let clean = relative_path.trim_matches('/').to_string();
        if !self.ordered_routes.contains(&clean) {
            self.ordered_routes.push(clean.clone());
        }
        self.docs.insert(clean, doc);
    }

    /// Recursively resolves a route path within this module or any nested submodule.
    pub fn resolve_route(&self, full_path: &str) -> Option<&ParsedDoc> {
        let clean = full_path.trim_matches('/');
        let my_prefix = self.route_prefix.trim_matches('/');

        // 1. Exact match with module prefix -> overview
        if clean == my_prefix {
            return self.docs.get("overview")
                .or_else(|| self.ordered_routes.first().and_then(|r| self.docs.get(r)));
        }

        // 2. Check if path starts with my_prefix/
        if let Some(rest) = clean.strip_prefix(&format!("{}/", my_prefix)) {
            // Check submodules first
            for sub in &self.submodules {
                let sub_prefix = sub.route_prefix.trim_matches('/');
                if rest == sub_prefix || rest.starts_with(&format!("{}/", sub_prefix)) {
                    if let Some(doc) = sub.resolve_route(full_path) {
                        return Some(doc);
                    }
                }
            }

            // Check submodules with relative path
            for sub in &self.submodules {
                if let Some(doc) = sub.resolve_route(rest) {
                    return Some(doc);
                }
            }

            // Check direct docs
            if let Some(doc) = self.docs.get(rest) {
                return Some(doc);
            }
            if let Some(doc) = self.docs.get(&format!("{}/overview", rest)) {
                return Some(doc);
            }
        }

        // 3. Check submodules directly
        for sub in &self.submodules {
            if let Some(doc) = sub.resolve_route(clean) {
                return Some(doc);
            }
        }

        // 4. Direct match in local docs
        self.docs.get(clean)
    }

    /// Recursively collects all documents in this module and submodules as (full_route, &ParsedDoc).
    pub fn all_docs(&self) -> Vec<(String, &ParsedDoc)> {
        let mut list = Vec::new();
        let my_prefix = self.route_prefix.trim_matches('/');

        for (rel, doc) in &self.docs {
            let full_route = if rel == "overview" {
                format!("/{}", my_prefix)
            } else {
                format!("/{}/{}", my_prefix, rel)
            };
            list.push((full_route, doc));
        }

        for sub in &self.submodules {
            list.extend(sub.all_docs());
        }

        list
    }

    /// Builds a hierarchical DocNavItem for the sidebar navigation tree.
    pub fn build_sidebar(&self) -> DocNavItem {
        let mut children = Vec::new();
        let my_prefix = self.route_prefix.trim_matches('/');

        // 1. Direct documents in this module
        for r in &self.ordered_routes {
            if let Some(d) = self.docs.get(r) {
                let full_href = if r == "overview" {
                    format!("/{}", my_prefix)
                } else {
                    format!("/{}/{}", my_prefix, r)
                };

                let item = DocNavItem::new(
                    format!("{}-{}", self.id, r.replace('/', "-")),
                    d.title.clone(),
                )
                .with_href(full_href);

                children.push(item);
            }
        }

        // 2. Submodules
        for sub in &self.submodules {
            children.push(sub.build_sidebar());
        }

        let mut nav = DocNavItem::new(format!("mod-{}", self.id), self.title.clone())
            .with_children(children);

        if let Some(ref ico) = self.icon {
            nav = nav.with_icon(ico);
        }
        if let Some(ref bg) = self.badge {
            nav = nav.with_badge(bg);
        }

        nav
    }

    /// Recursively searches for an interactive view matching the given route.
    pub fn resolve_interactive_view(&self, full_path: &str) -> Option<InteractiveViewFn> {
        let clean = full_path.trim_matches('/');
        let my_prefix = self.route_prefix.trim_matches('/');

        // 1. Direct match in local interactive views
        if let Some(view) = self.interactive_views.get(clean) {
            return Some(view.clone());
        }

        // 2. Check if path starts with my_prefix/
        if let Some(rest) = clean.strip_prefix(&format!("{}/", my_prefix)) {
            if let Some(view) = self.interactive_views.get(rest) {
                return Some(view.clone());
            }
            for sub in &self.submodules {
                if let Some(view) = sub.resolve_interactive_view(rest) {
                    return Some(view);
                }
                if let Some(view) = sub.resolve_interactive_view(full_path) {
                    return Some(view);
                }
            }
        }

        // 3. Check submodules directly
        for sub in &self.submodules {
            if let Some(view) = sub.resolve_interactive_view(clean) {
                return Some(view);
            }
        }

        None
    }
}

/// Abstract Microfrontend Module interface.
pub trait CodexModuleTrait {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn route_prefix(&self) -> &str;
    fn docs(&self) -> Vec<(String, &ParsedDoc)>;
    fn sidebar(&self) -> DocNavItem;
    fn interactive_views(&self) -> &HashMap<String, InteractiveViewFn>;
    fn submodules(&self) -> &[CodexModule];
}

impl CodexModuleTrait for CodexModule {
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn route_prefix(&self) -> &str {
        &self.route_prefix
    }
    fn docs(&self) -> Vec<(String, &ParsedDoc)> {
        self.all_docs()
    }
    fn sidebar(&self) -> DocNavItem {
        self.build_sidebar()
    }
    fn interactive_views(&self) -> &HashMap<String, InteractiveViewFn> {
        &self.interactive_views
    }
    fn submodules(&self) -> &[CodexModule] {
        &self.submodules
    }
}

