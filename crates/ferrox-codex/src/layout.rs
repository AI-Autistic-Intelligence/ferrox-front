//! # Documentation Layout Engine (`ferrox-codex::layout`)
//!
//! 3-column responsive layout powered by `ferrox-front-ui`:
//! - **Top Navbar**: Compact responsive Logo, Search trigger (Ctrl+K + mobile search icon), Theme Switcher, Mobile Hamburger.
//! - **Mobile Drawer**: Full off-canvas drawer with searchable DocSidebar, theme picker, language selector, and quick nav links.
//! - **Left Sidebar**: Collapsible, searchable hierarchical DocSidebar for desktop.
//! - **Center Column**: Breadcrumbs, Mobile TOC accordion, rendered Markdown content, Prev/Next pagination cards, Footer.
//! - **Right Sidebar**: Dynamic Table of Contents (TOC) with scrollspy tracking for desktop.

use web_sys::window;
use ferrox_front_core::dom::*;
use ferrox_front_core::reactivity::Signal;
use ferrox_front_core::theme::set_theme;
use ferrox_front_router::Router;
use ferrox_front_ui::{
    doc_sidebar, table_of_contents, mobile_table_of_contents, CommandPaletteState, Icon,
};
use crate::markdown::render_markdown;
use crate::registry::DocRegistry;

static THEMES: [&str; 4] = ["ferrox-cyber", "ocean-breeze", "midnight-forest", "sunset-gold"];
static CURRENT_THEME_IDX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Cycles through available design system themes on mobile tap.
pub fn cycle_theme() {
    let next = (CURRENT_THEME_IDX.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1) % THEMES.len();
    set_theme(THEMES[next]);
}

/// Opens the off-canvas mobile navigation drawer with smooth CSS slide-in.
pub fn open_mobile_drawer() {
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Some(b) = doc.get_element_by_id("ferrox-mobile-backdrop") {
            let _ = b.set_attribute("style", "display: block; opacity: 1; pointer-events: auto;");
        }
        if let Some(d) = doc.get_element_by_id("ferrox-mobile-drawer") {
            let _ = d.set_attribute("style", "transform: translateX(0);");
        }
        if let Some(body) = doc.body() {
            let _ = body.set_attribute("style", "overflow: hidden;");
        }
    }
}

/// Closes the off-canvas mobile navigation drawer.
pub fn close_mobile_drawer() {
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Some(b) = doc.get_element_by_id("ferrox-mobile-backdrop") {
            let _ = b.set_attribute("style", "display: none; opacity: 0; pointer-events: none;");
        }
        if let Some(d) = doc.get_element_by_id("ferrox-mobile-drawer") {
            let _ = d.set_attribute("style", "transform: translateX(-100%);");
        }
        if let Some(body) = doc.body() {
            let _ = body.set_attribute("style", "");
        }
    }
}

/// Renders the complete off-canvas Mobile Navigation Drawer.
pub fn render_mobile_drawer(
    registry: &DocRegistry,
    current_path: &str,
    router: &Router,
    search_state: &CommandPaletteState,
) -> DomBuilder {
    let r_clone_docs = router.clone();
    let r_clone_showcase = router.clone();
    let r_clone_sidebar = router.clone();
    let r_clone_lang = router.clone();
    let search_clone = search_state.clone();

    // 1. Backdrop
    let backdrop = div()
        .id("ferrox-mobile-backdrop")
        .class("ferrox-mobile-backdrop fixed inset-0 bg-slate-950/80 backdrop-blur-md z-50 transition-opacity duration-200")
        .style_attr("display: none; opacity: 0; pointer-events: none;")
        .on_click(|| close_mobile_drawer());

    // 2. Header
    let brand = div()
        .class("flex items-center gap-2.5 cursor-pointer")
        .children(vec![
            div()
                .class("w-8 h-8 rounded-xl flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 shadow-md shadow-indigo-500/25")
                .child(Icon::render("zap", 16, "#ffffff")),
            div().class("flex items-center gap-1.5").children(vec![
                span().class("font-extrabold text-base tracking-tight text-white").text("FERROX"),
                span().class("font-light text-base tracking-widest text-indigo-400").text("CODEX"),
                span().class("px-1.5 py-0.5 rounded text-[9px] font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30").text("WASM"),
            ]),
        ])
        .on_click(move || {
            close_mobile_drawer();
            r_clone_docs.navigate("/overview");
        });

    let close_btn = button()
        .class("p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800/80 transition-colors cursor-pointer")
        .attr("aria-label", "Close Menu")
        .child(Icon::render("x", 20, "#cbd5e1"))
        .on_click(|| close_mobile_drawer());

    let drawer_header = div()
        .class("flex items-center justify-between p-4 border-b border-slate-800/80 bg-slate-950")
        .children(vec![brand, close_btn]);

    // 3. Quick Actions: Search trigger button
    let search_card = button()
        .class("w-full flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-slate-900 border border-slate-800/90 text-xs text-slate-400 hover:text-white hover:border-indigo-500/40 transition-colors shadow-inner cursor-pointer")
        .children(vec![
            div().class("flex items-center gap-2.5").children(vec![
                Icon::render("search", 16, "#818cf8"),
                span().class("font-medium").text("Search docs & symbols..."),
            ]),
            span().class("px-1.5 py-0.5 rounded text-[10px] font-mono font-bold bg-slate-800 text-slate-300 border border-slate-700/60").text("Ctrl K"),
        ])
        .on_click(move || {
            close_mobile_drawer();
            search_clone.open();
        });

    // 4. Quick Nav Links: Docs & Showcase
    let nav_row = div()
        .class("grid grid-cols-2 gap-2 text-xs font-semibold")
        .children(vec![
            button()
                .class("flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg bg-indigo-600/20 text-indigo-300 border border-indigo-500/30 hover:bg-indigo-600/30 transition-colors cursor-pointer")
                .children(vec![
                    Icon::render("book-open", 14, "#818cf8"),
                    span().text("Docs"),
                ])
                .on_click(move || {
                    close_mobile_drawer();
                    r_clone_showcase.navigate("/overview");
                }),
            button()
                .class("flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg bg-slate-900 text-slate-300 border border-slate-800 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer")
                .children(vec![
                    Icon::render("zap", 14, "#10b981"),
                    span().text("Showcase"),
                ])
                .on_click({
                    let r = router.clone();
                    move || {
                        close_mobile_drawer();
                        r.navigate("/showcase");
                    }
                }),
        ]);

    // 5. Theme selection pills inside drawer
    let theme_picker = div()
        .class("flex flex-col gap-1.5")
        .children(vec![
            span().class("text-[10px] font-mono font-bold uppercase tracking-wider text-slate-400").text("Color Theme"),
            div()
                .class("grid grid-cols-4 gap-1 p-1 rounded-xl bg-slate-900 border border-slate-800/80")
                .children(vec![
                    button()
                        .class("py-1 rounded-lg text-xs font-semibold text-indigo-300 hover:bg-slate-800 transition-colors text-center")
                        .text("Cyber")
                        .on_click(|| set_theme("ferrox-cyber")),
                    button()
                        .class("py-1 rounded-lg text-xs font-semibold text-cyan-300 hover:bg-slate-800 transition-colors text-center")
                        .text("Ocean")
                        .on_click(|| set_theme("ocean-breeze")),
                    button()
                        .class("py-1 rounded-lg text-xs font-semibold text-emerald-300 hover:bg-slate-800 transition-colors text-center")
                        .text("Forest")
                        .on_click(|| set_theme("midnight-forest")),
                    button()
                        .class("py-1 rounded-lg text-xs font-semibold text-amber-300 hover:bg-slate-800 transition-colors text-center")
                        .text("Sunset")
                        .on_click(|| set_theme("sunset-gold")),
                ]),
        ]);

    // 6. Language selector
    let lang_widget = div()
        .class("flex items-center justify-between")
        .children(vec![
            span().class("text-[10px] font-mono font-bold uppercase tracking-wider text-slate-400").text("Language"),
            ferrox_front_ui::language_selector("en", move |loc| {
                close_mobile_drawer();
                if loc == "en" {
                    r_clone_lang.navigate("/overview");
                } else {
                    r_clone_lang.navigate(&format!("/{}", loc));
                }
            }),
        ]);

    let drawer_controls = div()
        .class("p-3.5 space-y-3 border-b border-slate-800/80 bg-slate-950/60")
        .children(vec![search_card, nav_row, theme_picker, lang_widget]);

    // 7. Full Hierarchical Documentation Navigation Tree
    let sidebar_items = registry.build_sidebar();
    let doc_tree = div()
        .class("flex-1 overflow-y-auto p-3 space-y-1")
        .child(
            doc_sidebar(
                sidebar_items,
                Signal::new(format!("/{}", current_path.trim_start_matches('/'))),
                move |target_url| {
                    close_mobile_drawer();
                    r_clone_sidebar.navigate(target_url);
                },
            )
        );

    // 8. Drawer Footer
    let drawer_footer = div()
        .class("p-3.5 border-t border-slate-800/80 bg-slate-950 flex items-center justify-between text-[11px] text-slate-400")
        .children(vec![
            a()
                .attr("href", "https://github.com/AI-Autistic-Intelligence/ferrox-front")
                .attr("target", "_blank")
                .attr("rel", "noopener noreferrer")
                .class("flex items-center gap-1.5 text-indigo-400 hover:text-indigo-300 transition-colors no-underline")
                .children(vec![
                    Icon::render("github", 14, "#818cf8"),
                    span().text("GitHub"),
                ]),
            span().class("font-mono text-slate-400").text("v0.3.3 Wasm"),
        ]);

    let panel = div()
        .id("ferrox-mobile-drawer")
        .class("ferrox-mobile-drawer fixed inset-y-0 left-0 w-80 max-w-[85vw] bg-slate-950 border-r border-slate-800 z-50 flex flex-col shadow-2xl overflow-hidden")
        .style_attr("transform: translateX(-100%); transition: transform 0.28s cubic-bezier(0.16, 1, 0.3, 1);")
        .children(vec![drawer_header, drawer_controls, doc_tree, drawer_footer]);

    div().children(vec![backdrop, panel])
}

/// Renders the Top Navigation Bar.
pub fn render_navbar(
    router: &Router,
    search_state: &CommandPaletteState,
    _mobile_sidebar_open: Signal<bool>,
) -> DomBuilder {
    let r_clone1 = router.clone();
    let r_clone2 = router.clone();
    let r_clone3 = router.clone();
    let search_clone_desktop = search_state.clone();
    let search_clone_mobile = search_state.clone();

    // Brand Logo
    let brand = div()
        .class("flex items-center gap-2.5 sm:gap-3 cursor-pointer group min-w-0")
        .children(vec![
            div()
                .class("w-8 h-8 sm:w-10 sm:h-10 rounded-xl flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 shadow-lg shadow-indigo-500/25 group-hover:scale-105 transition-transform duration-200 shrink-0")
                .child(Icon::render("zap", 18, "#ffffff")),
            div()
                .class("flex flex-col min-w-0")
                .children(vec![
                    div()
                        .class("flex items-center gap-1.5 sm:gap-2")
                        .children(vec![
                            span().class("font-extrabold text-base sm:text-lg tracking-tight text-white").text("FERROX"),
                            span().class("font-light text-base sm:text-lg tracking-widest text-indigo-400").text("CODEX"),
                            span().class("px-1.5 py-0.5 rounded text-[9px] sm:text-[10px] font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30").text("WASM"),
                        ]),
                    span().class("hidden md:inline-block text-[11px] sm:text-xs text-slate-400 font-medium truncate").text("Native WebAssembly Documentation Engine"),
                ]),
        ])
        .on_click(move || {
            r_clone1.navigate("/overview");
        });

    // Search Trigger (Desktop: Ctrl+K input style)
    let search_button = button()
        .class("hidden sm:flex items-center gap-3 px-3.5 py-1.5 lg:py-2 rounded-xl bg-slate-900/80 hover:bg-slate-800/90 text-slate-400 hover:text-white border border-slate-800/80 shadow-inner transition-all duration-150 cursor-pointer w-48 md:w-64 lg:w-80 justify-between shrink-0")
        .children(vec![
            div().class("flex items-center gap-2").children(vec![
                Icon::render("search", 16, "#94a3b8"),
                span().class("text-xs sm:text-sm font-medium").text("Search docs..."),
            ]),
            span().class("flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[10px] sm:text-xs font-mono font-bold bg-slate-800 text-slate-300 border border-slate-700/60 shadow-sm").text("Ctrl K"),
        ])
        .on_click(move || {
            search_clone_desktop.open();
        });

    // Mobile Search Icon Button
    let mobile_search_btn = button()
        .class("sm:hidden p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer shrink-0")
        .attr("aria-label", "Search Documentation")
        .child(Icon::render("search", 18, "#94a3b8"))
        .on_click(move || {
            search_clone_mobile.open();
        });

    // Desktop Nav Links
    let nav_links = div()
        .class("hidden lg:flex items-center gap-5 text-sm font-medium text-slate-300 shrink-0")
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

    // Desktop Theme Switcher Selector
    let theme_selector = div()
        .class("hidden md:flex items-center gap-1 p-1 rounded-xl bg-slate-900/80 border border-slate-800/80 shrink-0")
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

    // Mobile Theme Cycle Button
    let mobile_theme_btn = button()
        .class("md:hidden p-2 rounded-lg text-amber-400 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer shrink-0")
        .attr("aria-label", "Cycle Theme")
        .attr("title", "Toggle Color Theme")
        .child(Icon::render("sun", 18, "#f59e0b"))
        .on_click(|| cycle_theme());

    // Language Selector (i18n) - desktop/tablet
    let r_clone_lang = router.clone();
    let lang_widget = div()
        .class("hidden sm:flex items-center shrink-0")
        .child(
            ferrox_front_ui::language_selector("en", move |loc| {
                if loc == "en" {
                    r_clone_lang.navigate("/overview");
                } else {
                    r_clone_lang.navigate(&format!("/{}", loc));
                }
            })
        );

    // Mobile Hamburger
    let mobile_btn = button()
        .class("md:hidden p-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer shrink-0")
        .attr("aria-label", "Open Navigation Menu")
        .child(Icon::render("menu", 20, "#94a3b8"))
        .on_click(|| {
            open_mobile_drawer();
        });

    div()
        .class("ferrox-navbar sticky top-0 z-40 w-full h-14 sm:h-16 px-3 sm:px-6 lg:px-8 flex items-center justify-between bg-slate-950/85 backdrop-blur-xl border-b border-slate-800/80")
        .children(vec![
            div().class("flex items-center gap-3 sm:gap-6 min-w-0").children(vec![brand, search_button]),
            div().class("flex items-center gap-1 sm:gap-2.5 shrink-0").children(vec![
                nav_links,
                lang_widget,
                theme_selector,
                mobile_search_btn,
                mobile_theme_btn,
                mobile_btn,
            ]),
        ])
}

/// Renders Breadcrumbs trail at the top of content.
pub fn render_breadcrumbs(path: &str, title: &str, router: &Router) -> DomBuilder {
    let r_clone = router.clone();
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    let mut bc = div()
        .class("flex items-center flex-wrap gap-1.5 text-xs sm:text-sm text-slate-400 font-medium mb-4 sm:mb-6")
        .children(vec![
            span()
                .class("hover:text-indigo-400 transition-colors cursor-pointer")
                .text("Docs")
                .on_click(move || r_clone.navigate("/overview")),
            Icon::render("chevron-right", 12, "#64748b"),
        ]);

    if parts.len() > 1 {
        let section = parts[0].replace('-', " ");
        bc = bc.child(span().class("capitalize text-slate-300").text(&section));
        bc = bc.child(Icon::render("chevron-right", 12, "#64748b"));
    }

    bc.child(span().class("text-slate-200 font-semibold truncate max-w-[200px] sm:max-w-none").text(title))
}

/// Renders bottom previous/next page pagination cards.
pub fn render_pagination(
    prev: Option<(&str, &str)>,
    next: Option<(&str, &str)>,
    router: &Router,
) -> DomBuilder {
    let mut grid = div().class("grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4 my-8 sm:my-12 pt-6 sm:pt-8 border-t border-slate-800/60");

    if let Some((p_path, p_title)) = prev {
        let p_path_owned = format!("/{}", p_path.trim_start_matches('/'));
        let r_clone = router.clone();

        let card = div()
            .class("flex flex-col p-4 sm:p-5 rounded-xl sm:rounded-2xl border border-slate-800/80 hover:border-indigo-500/50 bg-slate-900/40 hover:bg-slate-900/80 transition-all duration-200 cursor-pointer group")
            .children(vec![
                span().class("text-xs uppercase font-bold tracking-wider text-slate-400 mb-1.5 flex items-center gap-1.5").children(vec![
                    Icon::render("arrow-left", 12, "#94a3b8"),
                    span().text("Previous"),
                ]),
                span().class("text-sm sm:text-base font-bold text-white group-hover:text-indigo-300 transition-colors").text(p_title),
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
            .class("flex flex-col items-end text-right p-4 sm:p-5 rounded-xl sm:rounded-2xl border border-slate-800/80 hover:border-indigo-500/50 bg-slate-900/40 hover:bg-slate-900/80 transition-all duration-200 cursor-pointer group")
            .children(vec![
                span().class("text-xs uppercase font-bold tracking-wider text-slate-400 mb-1.5 flex items-center gap-1.5").children(vec![
                    span().text("Next"),
                    Icon::render("arrow-right", 12, "#94a3b8"),
                ]),
                span().class("text-sm sm:text-base font-bold text-white group-hover:text-indigo-300 transition-colors").text(n_title),
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
        .class("mt-12 sm:mt-16 py-6 sm:py-8 border-t border-slate-800/80 flex flex-col sm:flex-row items-center justify-between text-xs text-slate-500 gap-3 text-center sm:text-left")
        .children(vec![
            div().class("flex items-center gap-2").children(vec![
                Icon::render("zap", 14, "#6366f1"),
                span().class("font-semibold text-slate-400").text("Ferrox Codex"),
                span().class("hidden sm:inline").text("• Pure Rust WebAssembly Dogfooding of ferrox-front"),
            ]),
            span().class("text-[11px] sm:text-xs").text("Zero JavaScript Runtime Overhead • 60 FPS"),
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
    let mobile_drawer = render_mobile_drawer(registry, current_path, router, search_state);
    let breadcrumbs = render_breadcrumbs(&doc.path, &doc.title, router);
    let mobile_toc = mobile_table_of_contents(doc.headings.clone(), active_heading.clone());
    let content = render_markdown(doc, router);
    let pagination = render_pagination(prev, next, router);
    let footer = render_footer();

    // Center Column Assembly (embeds live Wasm interactive sandboxes)
    let mut center_children = vec![breadcrumbs, mobile_toc, content];
    if let Some(sandbox_fn) = registry.get_interactive_sandbox(current_path) {
        center_children.push(sandbox_fn());
    }
    center_children.push(pagination);
    center_children.push(footer);

    let main_content = div()
        .class("flex-1 min-w-0 max-w-4xl xl:max-w-5xl 2xl:max-w-6xl py-6 sm:py-8 px-3.5 sm:px-8 lg:px-12")
        .children(center_children);

    // Left Sidebar Assembly (Desktop)
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

    // Right Sidebar Assembly (Table of Contents - Desktop)
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
            mobile_drawer,
            div()
                .class("flex-1 w-full max-w-[1536px] xl:max-w-[1680px] 2xl:max-w-[1840px] mx-auto flex justify-center min-w-0")
                .children(vec![left_sidebar, main_content, right_sidebar]),
        ])
}
