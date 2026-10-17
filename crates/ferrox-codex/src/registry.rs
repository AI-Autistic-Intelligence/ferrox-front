//! # Documentation Registry & Navigation Index (`ferrox-codex::registry`)
//!
//! Central repository of all documentation pages, navigation tree structure,
//! and full-text search index compiled directly into WebAssembly.

use std::collections::HashMap;
use ferrox_front_core::dom::DomBuilder;
use ferrox_front_ui::DocNavItem;
use crate::markdown::{parse_markdown, ParsedDoc};

use crate::modules::{
    create_core_module, create_front_module, create_node_module, create_polyglot_module,
    create_python_module, CodexModule,
};

/// Search hit matching a user query across titles, headings, and body tokens.
#[derive(Debug, Clone)]
pub struct SearchHit {
    pub path: String,
    pub title: String,
    pub section: Option<String>,
    pub snippet: String,
    pub score: usize,
}

/// Central registry holding all parsed documents and microfrontend modules.
pub struct DocRegistry {
    docs: HashMap<String, ParsedDoc>,
    ordered_routes: Vec<String>,
    modules: Vec<CodexModule>,
}

impl DocRegistry {
    /// Constructs and initializes the DocRegistry with all ecosystem documentation.
    pub fn new() -> Self {
        let mut reg = Self {
            docs: HashMap::new(),
            ordered_routes: Vec::new(),
            modules: Vec::new(),
        };

        reg.load_ecosystem_docs();
        reg
    }

    /// Registers a single microfrontend module and ingest all of its documents into the search catalog.
    pub fn register_module(&mut self, module: CodexModule) {
        for (full_route, doc) in module.all_docs() {
            let clean = full_route.trim_matches('/').to_string();
            if !self.ordered_routes.contains(&clean) {
                self.ordered_routes.push(clean.clone());
            }
            self.docs.insert(clean, doc.clone());
        }
        self.modules.push(module);
    }

    /// Accessor for all registered microfrontend modules.
    pub fn modules(&self) -> &[CodexModule] {
        &self.modules
    }

    /// Registers a markdown document.
    pub fn register(&mut self, path: &str, raw_markdown: &str) {
        let parsed = parse_markdown(path, raw_markdown);
        let clean = path.trim_matches('/').to_string();
        if !self.ordered_routes.contains(&clean) {
            self.ordered_routes.push(clean.clone());
        }
        self.docs.insert(clean, parsed);
    }

    /// Retrieves a document by route path.
    pub fn get(&self, path: &str) -> Option<&ParsedDoc> {
        let clean = path.trim_start_matches('/').trim_end_matches('/');
        if clean.is_empty() {
            return self.docs.get("overview").or_else(|| self.docs.get("overview/example-projects"));
        }

        // Special convenience aliases
        if clean == "showcase" {
            return self.docs.get("ecosystem/showcase-guide").or_else(|| self.docs.get("overview"));
        }

        // 1. Direct registry lookup
        if let Some(doc) = self.docs.get(clean) {
            return Some(doc);
        }
        if let Some(doc) = self.docs.get(&format!("{}/overview", clean)) {
            return Some(doc);
        }
        if let Some(doc) = self.docs.get(&format!("{}.md", clean)) {
            return Some(doc);
        }

        // 2. Resolve via microfrontend modules (including nested recursive submodules)
        for m in &self.modules {
            if let Some(doc) = m.resolve_route(clean) {
                return Some(doc);
            }
        }

        // 3. Fallback prefix search
        let prefix = format!("{}/", clean);
        self.ordered_routes
            .iter()
            .find(|r| r.starts_with(&prefix))
            .and_then(|r| self.docs.get(r))
    }

    /// Retrieves a document by route path, cleanly falling back to "overview" if not found.
    pub fn get_or_fallback(&self, path: &str) -> &ParsedDoc {
        self.get(path)
            .or_else(|| self.get("overview"))
            .expect("Default overview document must exist in registry")
    }

    /// Returns all registered documents across all modules and standalone pages: Vec<(full_route, &ParsedDoc)>.
    pub fn all_docs(&self) -> Vec<(String, &ParsedDoc)> {
        let mut list = Vec::new();
        for r in &self.ordered_routes {
            if let Some(doc) = self.docs.get(r) {
                let full_route = if r.starts_with('/') { r.clone() } else { format!("/{}", r) };
                list.push((full_route, doc));
            }
        }
        list
    }

    /// Resolves an in-page interactive Wasm sandbox/playground for the active document.
    pub fn get_interactive_sandbox(&self, path: &str) -> Option<fn() -> DomBuilder> {
        let clean = path.trim_start_matches('/').trim_end_matches('/');
        match clean {
            "ferrox-front/datagrid" | "front/datagrid" | "datagrid" => {
                Some(crate::sandboxes::render_datagrid_sandbox)
            }
            "ferrox-front/charts" | "front/charts" | "charts" => {
                Some(crate::sandboxes::render_charts_sandbox)
            }
            "ferrox-front/components" | "front/components" | "components" => {
                Some(crate::sandboxes::render_form_sandbox)
            }
            _ => None,
        }
    }

    /// Resolves an interactive Wasm full-page view/playground if registered for the route.
    pub fn get_interactive_view(&self, path: &str) -> Option<crate::modules::InteractiveViewFn> {
        let clean = path.trim_start_matches('/').trim_end_matches('/');

        if clean == "sandboxes/datagrid" || clean == "ferrox-front/datagrid/playground" {
            return Some(std::rc::Rc::new(crate::sandboxes::render_datagrid_sandbox));
        }
        if clean == "sandboxes/charts" || clean == "ferrox-front/charts/live" {
            return Some(std::rc::Rc::new(crate::sandboxes::render_charts_sandbox));
        }
        if clean == "sandboxes/form" || clean == "ferrox-front/components/playground" {
            return Some(std::rc::Rc::new(crate::sandboxes::render_form_sandbox));
        }

        for m in &self.modules {
            if let Some(view) = m.resolve_interactive_view(clean) {
                return Some(view);
            }
        }
        None
    }

    /// Returns previous and next document links for pagination: (Option<(path, title)>, Option<(path, title)>)
    pub fn prev_next(&self, current_path: &str) -> (Option<(&str, &str)>, Option<(&str, &str)>) {
        let clean = current_path.trim_start_matches('/').trim_end_matches('/');
        let cur_key = if clean.is_empty() { "overview" } else { clean };

        if let Some(pos) = self.ordered_routes.iter().position(|r| r == cur_key) {
            let prev = if pos > 0 {
                let p_route = &self.ordered_routes[pos - 1];
                self.docs.get(p_route).map(|d| (p_route.as_str(), d.title.as_str()))
            } else {
                None
            };

            let next = if pos + 1 < self.ordered_routes.len() {
                let n_route = &self.ordered_routes[pos + 1];
                self.docs.get(n_route).map(|d| (n_route.as_str(), d.title.as_str()))
            } else {
                None
            };

            (prev, next)
        } else {
            (None, None)
        }
    }

    /// Performs fast in-memory full-text search across all documents.
    pub fn search(&self, query: &str) -> Vec<SearchHit> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return Vec::new();
        }

        let mut hits = Vec::new();

        for (path, doc) in &self.docs {
            let mut score = 0;
            let title_lower = doc.title.to_lowercase();
            let mut matched_section: Option<String> = None;

            // 1. Exact or partial title match (highest score)
            if title_lower == q {
                score += 100;
            } else if title_lower.contains(&q) {
                score += 50;
            }

            // 2. Heading matches
            for h in &doc.headings {
                let h_lower = h.title.to_lowercase();
                if h_lower.contains(&q) {
                    score += 30;
                    if matched_section.is_none() {
                        matched_section = Some(h.title.clone());
                    }
                }
            }

            // 3. Keywords / tags matches
            for kw in &doc.metadata.keywords {
                if kw.to_lowercase().contains(&q) {
                    score += 20;
                }
            }

            // 4. Content matches
            let text_lower = doc.search_text.to_lowercase();
            if let Some(pos) = text_lower.find(&q) {
                score += 10;
                let char_indices: Vec<(usize, char)> = doc.search_text.char_indices().collect();
                let char_pos = char_indices.iter().position(|&(idx, _)| idx >= pos).unwrap_or(0);
                let start_char = char_pos.saturating_sub(15);
                let end_char = (char_pos + q.chars().count() + 25).min(char_indices.len());
                let snippet: String = char_indices[start_char..end_char].iter().map(|&(_, c)| c).collect();
                let snippet_formatted = format!("...{}...", snippet.trim());

                hits.push(SearchHit {
                    path: path.clone(),
                    title: doc.title.clone(),
                    section: matched_section,
                    snippet: snippet_formatted,
                    score,
                });
            } else if score > 0 {
                let snippet = doc.metadata.description.clone()
                    .unwrap_or_else(|| doc.search_text.chars().take(100).collect());

                hits.push(SearchHit {
                    path: path.clone(),
                    title: doc.title.clone(),
                    section: matched_section,
                    snippet,
                    score,
                });
            }
        }

        hits.sort_by(|a, b| b.score.cmp(&a.score));
        hits.truncate(8); // Top 8 relevant results
        hits
    }

    /// Constructs the hierarchical documentation sidebar navigation tree.
    pub fn build_sidebar(&self) -> Vec<DocNavItem> {
        let mut items = vec![
            // 🚀 Getting Started & Interactive Showcase
            DocNavItem::new("cat-getting-started", "🚀 Getting Started")
                .with_icon("zap")
                .with_children(vec![
                    DocNavItem::new("nav-overview", "Zero-JS Vision & Architecture")
                        .with_href("/overview")
                        .with_icon("zap"),
                    DocNavItem::new("nav-showcase", "Interactive Live Showcases")
                        .with_href("/showcase")
                        .with_icon("activity")
                        .with_badge("Live Wasm"),
                ]),
        ];

        // Complete 315-file ecosystem sidebar tree
        items.extend(crate::generated_ecosystem_docs::build_complete_ecosystem_sidebar());

        items
    }

    /// Loads built-in ecosystem documentation markdown files.
    fn load_ecosystem_docs(&mut self) {
        // 1. Overview & Vision
        self.register(
            "overview",
            include_str!("../docs/ferrox-front/overview.md"),
        );

        // 2. Example Projects
        self.register(
            "overview/example-projects",
            include_str!("../docs/overview/example-projects.md"),
        );

        // 3. Ferrox Front Overview
        self.register(
            "ferrox-front/overview",
            include_str!("../docs/ferrox-front/overview.md"),
        );

        // 4. UI Components Catalog
        self.register(
            "ferrox-front/components",
            include_str!("../docs/ferrox-front/components.md"),
        );

        // 5. Enterprise DataGrid
        self.register(
            "ferrox-front/datagrid",
            include_str!("../docs/ferrox-front/datagrid.md"),
        );

        // 6. Vector SVG Charts
        self.register(
            "ferrox-front/charts",
            include_str!("../docs/ferrox-front/charts.md"),
        );

        // 7. Trading Desk Pro
        self.register(
            "ferrox-front/trading-desk",
            include_str!("../docs/ferrox-front/trading-desk.md"),
        );

        // 8. Security Audit Report
        self.register(
            "security/audit",
            include_str!("../docs/SECURITY_AUDIT_REPORT.md"),
        );

        // 9. Ecosystem Showcase Guide
        self.register(
            "ecosystem/showcase-guide",
            include_str!("../docs/ECOSYSTEM_SHOWCASE_GUIDE.md"),
        );

        // 10. 1 Million RPS Benchmark
        self.register(
            "benchmarks/1m-rps",
            include_str!("../docs/benchmarks/1m-rps.md"),
        );

        // 11. Ingest all 315 ecosystem documentation files from ferrox-docs/docs!
        crate::generated_ecosystem_docs::register_all_ecosystem_docs(self);

        // 12. Register plug-and-play Microfrontend modules
        self.register_module(create_core_module());
        self.register_module(create_front_module());
        self.register_module(create_node_module());
        self.register_module(create_python_module());
        self.register_module(create_polyglot_module());
    }
}

impl Default for DocRegistry {
    fn default() -> Self {
        Self::new()
    }
}
