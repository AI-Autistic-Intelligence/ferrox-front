//! # Documentation Layout Engine (`ferrox-codex::layout`)
//!
//! 3-column responsive layout powered by `ferrox-front-ui`:
//! - **Top Navbar**: Logo, Search trigger (Ctrl+K), Theme Switcher, GitHub link.
//! - **Left Sidebar**: Collapsible, searchable hierarchical DocSidebar.
//! - **Center Column**: Breadcrumbs, rendered Markdown content, Prev/Next pagination cards, Footer.
//! - **Right Sidebar**: Dynamic Table of Contents (TOC) with scrollspy tracking.

use ferrox_front_core::dom::*;
use ferrox_front_core::reactivity::Signal;
use ferrox_front_core::theme::set_theme;
use ferrox_front_router::Router;
use ferrox_front_ui::{
    doc_sidebar, table_of_contents, CommandPaletteState, Icon,
};
use crate::markdown::render_markdown;
use crate::registry::DocRegistry;

/// Renders the Top Navigation Bar.
pub fn render_navbar(
    router: &Router,
    search_state: &CommandPaletteState,
    mobile_sidebar_open: Signal<bool>,
) -> DomBuilder {
    let r_clone1 = router.clone();
    let r_clone2 = router.clone();
    let r_clone3 = router.clone();
    let search_clone = search_state.clone();
    let mobile_toggle_clone = mobile_sidebar_open.clone();

    // Brand Logo
    let brand = div()
        .class("flex items-center gap-3 cursor-pointer group")
        .children(vec![
            div()
                .class("w-10 h-10 rounded-xl flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 shadow-lg shadow-indigo-500/25 group-hover:scale-105 transition-transform duration-200")
                .child(Icon::render("zap", 22, "#ffffff")),
            div()
                .class("flex flex-col")
                .children(vec![
                    div()
                        .class("flex items-center gap-2")
                        .children(vec![
                            span().class("font-extrabold text-lg tracking-tight text-white").text("FERROX"),
                            span().class("font-light text-lg tracking-widest text-indigo-400").text("CODEX"),
                            span().class("px-1.5 py-0.5 rounded text-[10px] font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30").text("WASM"),
                        ]),
                    span().class("text-xs text-slate-400 font-medium").text("Native WebAssembly Documentation Engine"),
                ]),
        ])
        .on_click(move || {
            r_clone1.navigate("/overview");
        });

    // Search Trigger (Ctrl+K)
    let search_button = button()
        .class("hidden sm:flex items-center gap-3 px-4 py-2 rounded-xl bg-slate-900/80 hover:bg-slate-800/90 text-slate-400 hover:text-white border border-slate-800/80 shadow-inner transition-all duration-150 cursor-pointer w-80 lg:w-96 justify-between")
        .children(vec![
            div().class("flex items-center gap-2.5").children(vec![
                Icon::render("search", 18, "#94a3b8"),
                span().class("text-sm font-medium").text("Search documentation..."),
            ]),
            span().class("flex items-center gap-0.5 px-2 py-0.5 rounded text-xs font-mono font-bold bg-slate-800 text-slate-300 border border-slate-700/60 shadow-sm").text("Ctrl K"),
        ])
        .on_click(move || {
            search_clone.open();
        });

    // Nav Links
    let nav_links = div()
        .class("hidden lg:flex items-center gap-6 text-sm font-medium text-slate-300")
        .children(vec![
            span()
                .class("hover:text-indigo-400 transition-colors cursor-pointer")
                .text("Docs")
                .on_click(move || r_clone2.navigate("/overview")),
            span()
                .class("hover:text-indigo-400 transition-colors cursor-pointer")
                .text("Live Showcase")
                .on_click(move || r_clone3.navigate("/showcase")),
            a()
                .attr("href", "https://github.com/AI-Autistic-Intelligence/ferrox-front")
                .attr("target", "_blank")
                .attr("rel", "noopener noreferrer")
                .class("hover:text-indigo-400 transition-colors flex items-center gap-1.5 no-underline")
                .children(vec![
                    span().text("GitHub"),
                    Icon::render("external-link", 13, "#94a3b8"),
                ]),
        ]);

    // Theme Switcher Selector
    let theme_selector = div()
        .class("flex items-center gap-1 p-1 rounded-xl bg-slate-900/80 border border-slate-800/80")
        .children(vec![
            button()
                .class("px-2.5 py-1 rounded-lg text-xs font-semibold text-indigo-300 hover:text-white hover:bg-slate-800 transition-colors")
                .text("Cyber")
                .on_click(|| set_theme("ferrox-cyber")),
            button()
                .class("px-2.5 py-1 rounded-lg text-xs font-semibold text-cyan-300 hover:text-white hover:bg-slate-800 transition-colors")
                .text("Ocean")
                .on_click(|| set_theme("ocean-breeze")),
            button()
                .class("px-2.5 py-1 rounded-lg text-xs font-semibold text-emerald-300 hover:text-white hover:bg-slate-800 transition-colors")
                .text("Forest")
                .on_click(|| set_theme("midnight-forest")),
            button()
                .class("px-2.5 py-1 rounded-lg text-xs font-semibold text-amber-300 hover:text-white hover:bg-slate-800 transition-colors")
                .text("Sunset")
                .on_click(|| set_theme("sunset-gold")),
        ]);

    // Language Selector (i18n)
    let r_clone_lang = router.clone();
    let lang_widget = ferrox_front_ui::language_selector("en", move |loc| {
        if loc == "en" {
            r_clone_lang.navigate("/overview");
        } else {
            r_clone_lang.navigate(&format!("/{}", loc));
        }
    });

    // Mobile Hamburger
    let mobile_btn = button()
        .class("md:hidden p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors")
        .child(Icon::render("menu", 20, "#94a3b8"))
        .on_click(move || {
            let cur = mobile_toggle_clone.get();
            mobile_toggle_clone.set(!cur);
        });

    div()
        .class("ferrox-navbar sticky top-0 z-40 w-full h-16 px-4 sm:px-8 flex items-center justify-between bg-slate-950/80 backdrop-blur-xl border-b border-slate-800/80")
        .children(vec![
            div().class("flex items-center gap-8").children(vec![brand, search_button]),
            div().class("flex items-center gap-3 sm:gap-4").children(vec![nav_links, lang_widget, theme_selector, mobile_btn]),
        ])
}

/// Renders Breadcrumbs trail at the top of content.
pub fn render_breadcrumbs(path: &str, title: &str, router: &Router) -> DomBuilder {
    let r_clone = router.clone();
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    let mut bc = div()
        .class("flex items-center gap-2 text-sm text-slate-400 font-medium mb-6")
        .children(vec![
            span()
                .class("hover:text-indigo-400 transition-colors cursor-pointer")
                .text("Docs")
                .on_click(move || r_clone.navigate("/overview")),
            Icon::render("chevron-right", 13, "#64748b"),
        ]);

    if parts.len() > 1 {
        let section = parts[0].replace('-', " ");
        bc = bc.child(span().class("capitalize").text(&section));
        bc = bc.child(Icon::render("chevron-right", 13, "#64748b"));
    }

    bc.child(span().class("text-slate-200 font-semibold truncate").text(title))
}

/// Renders bottom previous/next page pagination cards.
pub fn render_pagination(
    prev: Option<(&str, &str)>,
    next: Option<(&str, &str)>,
    router: &Router,
) -> DomBuilder {
    let mut grid = div().class("grid grid-cols-1 sm:grid-cols-2 gap-4 my-12 pt-8 border-t border-slate-800/60");

    if let Some((p_path, p_title)) = prev {
        let p_path_owned = format!("/{}", p_path.trim_start_matches('/'));
        let r_clone = router.clone();

        let card = div()
            .class("flex flex-col p-5 rounded-2xl border border-slate-800/80 hover:border-indigo-500/50 bg-slate-900/40 hover:bg-slate-900/80 transition-all duration-200 cursor-pointer group")
            .children(vec![
                span().class("text-xs uppercase font-bold tracking-wider text-slate-400 mb-1.5 flex items-center gap-1.5").children(vec![
                    Icon::render("arrow-left", 12, "#94a3b8"),
                    span().text("Previous"),
                ]),
                span().class("text-base font-bold text-white group-hover:text-indigo-300 transition-colors").text(p_title),
            ])
            .on_click(move || {
                r_clone.navigate(&p_path_owned);
            });

        grid = grid.child(card);
    } else {
        grid = grid.child(div()); // Empty spacer
    }

    if let Some((n_path, n_title)) = next {
        let n_path_owned = format!("/{}", n_path.trim_start_matches('/'));
        let r_clone = router.clone();

        let card = div()
            .class("flex flex-col items-end text-right p-5 rounded-2xl border border-slate-800/80 hover:border-indigo-500/50 bg-slate-900/40 hover:bg-slate-900/80 transition-all duration-200 cursor-pointer group")
            .children(vec![
                span().class("text-xs uppercase font-bold tracking-wider text-slate-400 mb-1.5 flex items-center gap-1.5").children(vec![
                    span().text("Next"),
                    Icon::render("arrow-right", 12, "#94a3b8"),
                ]),
                span().class("text-base font-bold text-white group-hover:text-indigo-300 transition-colors").text(n_title),
            ])
            .on_click(move || {
                r_clone.navigate(&n_path_owned);
            });

        grid = grid.child(card);
    }

    grid
}

/// Renders the page footer.
pub fn render_footer() -> DomBuilder {
    div()
        .class("mt-16 py-8 border-t border-slate-800/80 flex flex-col sm:flex-row items-center justify-between text-xs text-slate-500 gap-4")
        .children(vec![
            div().class("flex items-center gap-2").children(vec![
                Icon::render("zap", 14, "#6366f1"),
                span().class("font-semibold text-slate-400").text("Ferrox Codex"),
                span().text("• Pure Rust WebAssembly Dogfooding of ferrox-front"),
            ]),
            span().text("Zero JavaScript Runtime Overhead • Zero React • 60 FPS"),
        ])
}

/// Complete responsive 3-column document view layout.
pub fn render_doc_layout(
    registry: &DocRegistry,
    current_path: &str,
    router: &Router,
    search_state: &CommandPaletteState,
    active_heading: Signal<Option<String>>,
    mobile_sidebar_open: Signal<bool>,
) -> DomBuilder {
    let doc = registry.get(current_path).or_else(|| registry.get("overview")).unwrap();
    let (prev, next) = registry.prev_next(current_path);

    let navbar = render_navbar(router, search_state, mobile_sidebar_open.clone());
    let breadcrumbs = render_breadcrumbs(&doc.path, &doc.title, router);
    let content = render_markdown(doc, router);
    let pagination = render_pagination(prev, next, router);
    let footer = render_footer();

    // Center Column Assembly
    let main_content = div()
        .class("flex-1 min-w-0 max-w-4xl xl:max-w-5xl 2xl:max-w-6xl py-8 px-6 sm:px-10 lg:px-14")
        .children(vec![breadcrumbs, content, pagination, footer]);

    // Left Sidebar Assembly
    let sidebar_items = registry.build_sidebar();
    let r_clone = router.clone();
    let left_sidebar = div()
        .class("hidden md:block w-72 lg:w-80 xl:w-84 shrink-0 h-[calc(100vh-4rem)] sticky top-16 overflow-y-auto border-r border-slate-800/80 bg-slate-950/40 p-4 lg:p-5")
        .child(
            doc_sidebar(
                sidebar_items,
                Signal::new(format!("/{}", current_path.trim_start_matches('/'))),
                move |target_url| {
                    r_clone.navigate(target_url);
                },
            )
        );

    // Right Sidebar Assembly (Table of Contents)
    let right_sidebar = if !doc.headings.is_empty() {
        div()
            .class("hidden xl:block w-64 lg:w-72 xl:w-80 shrink-0 h-[calc(100vh-4rem)] sticky top-16 overflow-y-auto py-8 pr-4 pl-4 border-l border-slate-800/80")
            .child(table_of_contents(doc.headings.clone(), active_heading))
    } else {
        div().class("hidden xl:block w-6")
    };

    div()
        .class("min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500/30 selection:text-indigo-200")
        .children(vec![
            navbar,
            div()
                .class("flex-1 w-full max-w-[1536px] xl:max-w-[1680px] 2xl:max-w-[1840px] mx-auto flex justify-center")
                .children(vec![left_sidebar, main_content, right_sidebar]),
        ])
}
