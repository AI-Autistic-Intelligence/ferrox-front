//! # Full-Text Documentation Search Engine (`ferrox-codex::search`)
//!
//! Provides instant, client-side fuzzy search across document titles, headings,
//! and content, tightly integrated with the `ferrox-front-ui` Command Palette (`Ctrl+K`).

use ferrox_front_router::Router;
use ferrox_front_ui::{CommandAction, CommandPaletteState};
use crate::registry::DocRegistry;

/// Constructs the CommandPaletteState populated with all documentation pages,
/// sections, headings, and quick navigation routes.
pub fn build_doc_search_palette(
    registry: &DocRegistry,
    router: &Router,
) -> CommandPaletteState {
    let mut actions = Vec::new();

    // 1. Core Quick Navigation Actions
    let r_overview = router.clone();
    actions.push(CommandAction::new(
        "nav-home",
        "Home / Overview",
        "Zero-JS WebAssembly Vision & Core Architecture",
        "Navigation",
        "home",
        Some("G H"),
        move || {
            r_overview.navigate("/overview");
        },
    ));

    let r_showcase = router.clone();
    actions.push(CommandAction::new(
        "nav-showcase",
        "Interactive Live Showcase",
        "1M-Row DataGrid, SVG Charts, and Signal Reactivity Playground",
        "Showcase",
        "activity",
        Some("G S"),
        move || {
            r_showcase.navigate("/showcase");
        },
    ));

    // 2. Index Documentation Pages and Headings
    let sidebar_items = registry.build_sidebar();
    for cat in sidebar_items {
        for child in cat.children {
            if let Some(href) = child.href {
                let href_owned = href.clone();
                let r_clone = router.clone();
                let label = child.label.clone();
                let cat_label = cat.label.clone();
                let icon_name = child.icon.unwrap_or_else(|| "file".to_string());

                actions.push(CommandAction::new(
                    format!("doc-{}", child.id),
                    label,
                    format!("Documentation • {}", cat_label),
                    cat_label.clone(),
                    icon_name,
                    None,
                    move || {
                        r_clone.navigate(&href_owned);
                    },
                ));

                // Also index headings if the doc is in the registry
                let clean_path = href.trim_start_matches('/');
                if let Some(doc) = registry.get(clean_path) {
                    for h in &doc.headings {
                        let h_slug = h.id.clone();
                        let target_url = format!("{}#{}", href, h_slug);
                        let r_head = router.clone();
                        let h_title = h.title.clone();
                        let doc_title = doc.title.clone();

                        actions.push(CommandAction::new(
                            format!("head-{}-{}", child.id, h.id),
                            h_title,
                            format!("{} • Section", doc_title),
                            cat_label.clone(),
                            "layers",
                            None,
                            move || {
                                r_head.navigate(&target_url);
                            },
                        ));
                    }
                }
            }
        }
    }

    CommandPaletteState::new(actions)
}
