//! # Ferrox Front Microfrontend Module (`ferrox-codex::modules::front`)

use super::CodexModule;

/// Assembles the Ferrox Front WebAssembly microfrontend module.
pub fn create_front_module() -> CodexModule {
    let mut module = CodexModule::new("front", "🎨 Ferrox Front (Wasm)", "/front")
        .with_icon("layers")
        .with_badge("Zero-JS");

    // 1. Front Engine Overview & Reactive Signals
    module.register_doc(
        "overview",
        include_str!("../../../../../ferrox-docs/docs/ferrox-front/overview.md"),
    );

    // 2. UI Components Catalog
    module.register_doc(
        "components",
        include_str!("../../../../../ferrox-docs/docs/ferrox-front/components.md"),
    );

    // 3. 1,000,000-Row Virtualized DataGrid
    module.register_doc(
        "datagrid",
        include_str!("../../../../../ferrox-docs/docs/ferrox-front/datagrid.md"),
    );

    // 4. Vector SVG Charts & Sparklines
    module.register_doc(
        "charts",
        include_str!("../../../../../ferrox-docs/docs/ferrox-front/charts.md"),
    );

    // 5. Trading Desk Pro Application
    module.register_doc(
        "trading-desk",
        include_str!("../../../../../ferrox-docs/docs/ferrox-front/trading-desk.md"),
    );

    // 6. Interactive Live Wasm Sandboxes & Playgrounds
    module = module.with_interactive_view("datagrid/playground", || {
        crate::showcase::render_showcase_page(&ferrox_front_router::Router::new())
    });
    module = module.with_interactive_view("charts/live", || {
        crate::showcase::render_showcase_page(&ferrox_front_router::Router::new())
    });

    module
}
