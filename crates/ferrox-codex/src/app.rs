//! # Ferrox Codex WebAssembly Application (`ferrox-codex::app`)
//!
//! Main SPA coordinator mounting into `#root`.
//! Integrates Router, DocRegistry, CommandPalette, and Layout.

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use ferrox_front_core::dom::{div, mount};
use ferrox_front_core::reactivity::Signal;
use ferrox_front_router::Router;
use ferrox_front_ui::command_palette;

use crate::layout::render_doc_layout;
use crate::registry::DocRegistry;
use crate::search::build_doc_search_palette;
use crate::showcase::render_showcase_page;
use crate::layout::render_navbar;

/// High-level function that renders the entire application for the active route.
pub fn render_codex_app(
    route: &str,
    registry: &Rc<DocRegistry>,
    router: &Router,
) {
    let clean_route = route.trim_start_matches('#');
    let path = if clean_route.is_empty() || clean_route == "/" {
        "/overview"
    } else {
        clean_route
    };

    let search_state = build_doc_search_palette(registry, router);
    let active_heading = Signal::new(None);
    let mobile_sidebar_open = Signal::new(false);

    let app_root = if path == "/showcase" {
        let navbar = render_navbar(router, &search_state, mobile_sidebar_open);
        let showcase_content = render_showcase_page(router);
        let cp_overlay = command_palette(search_state);

        div()
            .class("min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500/30 selection:text-indigo-200")
            .children(vec![navbar, showcase_content, cp_overlay])
    } else if let Some(view_fn) = registry.get_interactive_view(path) {
        let navbar = render_navbar(router, &search_state, mobile_sidebar_open);
        let live_view = view_fn();
        let cp_overlay = command_palette(search_state);

        div()
            .class("min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500/30 selection:text-indigo-200")
            .children(vec![navbar, live_view, cp_overlay])
    } else {
        let mut layout = render_doc_layout(
            registry,
            path,
            router,
            &search_state,
            active_heading,
            mobile_sidebar_open,
        );
        let cp_overlay = command_palette(search_state);
        layout = layout.child(cp_overlay);
        layout
    };

    mount("#root", app_root);

    // Trigger Mermaid.js vector SVG diagram rendering
    trigger_mermaid_render();
}

/// Triggers Mermaid.js execution in the browser to transform raw code into SVG.
#[cfg(target_arch = "wasm32")]
pub fn trigger_mermaid_render() {
    let _ = js_sys::eval("setTimeout(function() { if (window.renderFerroxMermaid) window.renderFerroxMermaid(); }, 50);");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn trigger_mermaid_render() {}

/// WebAssembly entrypoint initialized on page load.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let registry = Rc::new(DocRegistry::new());
    let router = Router::new();

    // Determine initial route from URL path or hash
    let initial_path = if let Some(win) = window() {
        let loc = win.location();
        let pathname = loc.pathname().unwrap_or_else(|_| "/".to_string());
        let hash = loc.hash().unwrap_or_default();

        if !hash.is_empty() && hash.starts_with("#/") {
            hash.trim_start_matches('#').to_string()
        } else if pathname != "/" && !pathname.is_empty() {
            pathname
        } else {
            "/overview".to_string()
        }
    } else {
        "/overview".to_string()
    };

    // Initial Render
    let reg_clone1 = registry.clone();
    let router_clone1 = router.clone();
    render_codex_app(&initial_path, &reg_clone1, &router_clone1);

    // Direct router navigation reactive subscription
    let reg_nav = registry.clone();
    let router_nav = router.clone();
    router.on_route_change(move |new_path| {
        render_codex_app(new_path, &reg_nav, &router_nav);
    });

    // Attach History and Popstate synchronization (supports both native and synthetic events)
    if let Some(win) = window() {
        let reg_pop = registry.clone();
        let router_pop = router.clone();

        let on_popstate = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(w) = window() {
                let loc = w.location();
                let path = loc.pathname().unwrap_or_else(|_| "/".to_string());
                let hash = loc.hash().unwrap_or_default();

                let effective_route = if !hash.is_empty() && hash.starts_with("#/") {
                    hash.trim_start_matches('#').to_string()
                } else if path != "/" && !path.is_empty() {
                    path
                } else {
                    "/overview".to_string()
                };

                render_codex_app(&effective_route, &reg_pop, &router_pop);
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        let _ = win.add_event_listener_with_callback(
            "popstate",
            on_popstate.as_ref().unchecked_ref(),
        );
        on_popstate.forget();

        // Also listen to hashchange for seamless client anchor and route changes
        let reg_hash = registry.clone();
        let router_hash = router.clone();

        let on_hash = Closure::wrap(Box::new(move || {
            if let Some(w) = window() {
                let loc = w.location();
                let hash = loc.hash().unwrap_or_default();
                if hash.starts_with("#/") {
                    let route = hash.trim_start_matches('#');
                    render_codex_app(route, &reg_hash, &router_hash);
                }
            }
        }) as Box<dyn FnMut()>);

        let _ = win.add_event_listener_with_callback(
            "hashchange",
            on_hash.as_ref().unchecked_ref(),
        );
        on_hash.forget();
    }

    Ok(())
}
