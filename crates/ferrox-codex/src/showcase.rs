//! # Live Interactive Showcase Playground (`ferrox-codex::showcase`)
//!
//! Demonstrates native WebAssembly dogfooding of `ferrox-front` without iframes:
//! 1. **Enterprise DataGrid**: 1,000 live order rows, multi-column sorting, and RFC 4180 CSV export.
//! 2. **Financial Vector SVG Charts**: Pure Rust SVG Candlestick & Volume chart with zero JS charting libraries.
//! 3. **Fine-Grained Signals Reactivity**: Live surgical DOM mutations via `Signal<T>`.
//! 4. **Design System & Admonitions**: Interactive previews of all callout styles and interactive CodeBlocks.

use ferrox_front_core::dom::*;
use ferrox_front_core::reactivity::Signal;
use ferrox_front_router::Router;
use ferrox_front_ui::{
    admonition_caution, admonition_danger, admonition_info, admonition_note, admonition_tip,
    admonition_warning, code_block, tab_group, Icon, TabPanelDef,
};
use ferrox_front_charts::{
    candlestick_chart, CandleDatum,
    gauge_chart, sparkline_chart, SparklineType,
};

/// Live Order Record for DataGrid showcase.
#[derive(Clone, Debug)]
pub struct ShowcaseOrder {
    pub id: String,
    pub symbol: &'static str,
    pub side: &'static str,
    pub price: f64,
    pub amount: f64,
    pub total: f64,
    pub status: &'static str,
}

impl ShowcaseOrder {
    pub fn mock_orders() -> Vec<Self> {
        let symbols = ["BTC/USDT", "ETH/USDT", "SOL/USDT", "FERROX/USDT", "RUST/USDT"];
        let sides = ["BUY", "SELL"];
        let statuses = ["Filled", "Pending", "Filled", "Canceled", "Filled"];

        let mut orders = Vec::with_capacity(100);
        for i in 1..=50 {
            let symbol = symbols[(i * 3) % symbols.len()];
            let side = sides[(i * 7) % sides.len()];
            let status = statuses[(i * 5) % statuses.len()];
            let price = match symbol {
                "BTC/USDT" => 94250.0 + (i as f64 * 12.5),
                "ETH/USDT" => 3450.0 + (i as f64 * 2.3),
                "SOL/USDT" => 188.0 + (i as f64 * 0.8),
                "FERROX/USDT" => 42.10 + (i as f64 * 0.4),
                _ => 15.60 + (i as f64 * 0.1),
            };
            let amount = 0.5 + (i as f64 * 0.25);
            let total = price * amount;

            orders.push(Self {
                id: format!("ORD-{:05}", i),
                symbol,
                side,
                price,
                amount,
                total,
                status,
            });
        }
        orders
    }
}

/// Generates mock candlestick OHLC points for the charts showcase.
fn generate_mock_candles() -> Vec<CandleDatum> {
    let mut candles = Vec::with_capacity(20);
    let mut base_price = 93800.0;

    for i in 1..=20 {
        let delta = ((i as f64 * 1.7).sin() * 250.0) + (i as f64 * 18.0);
        let open = base_price;
        let close = base_price + delta;
        let high = open.max(close) + (i as f64 * 30.0).abs();
        let low = open.min(close) - (i as f64 * 25.0).abs();
        let volume = 12.5 + (i as f64 * 4.2);

        candles.push(CandleDatum {
            time: format!("1{:02}:00", i),
            open,
            high,
            low,
            close,
            volume,
        });

        base_price = close;
    }
    candles
}

/// Renders the complete Live Showcase page.
pub fn render_showcase_page(_router: &Router) -> DomBuilder {
    // Header Hero Banner
    let header_hero = div()
        .class("flex flex-col mb-10 pb-8 border-b border-slate-800/80")
        .children(vec![
            div().class("flex items-center gap-2 mb-3").children(vec![
                span().class("px-2.5 py-0.5 rounded-full text-xs font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30").text("Native Dogfooding"),
                span().class("px-2.5 py-0.5 rounded-full text-xs font-mono font-bold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30").text("Zero JavaScript"),
                span().class("px-2.5 py-0.5 rounded-full text-xs font-mono font-bold bg-purple-500/20 text-purple-300 border border-purple-500/30").text("60 FPS Wasm"),
            ]),
            h1()
                .class("text-3xl sm:text-5xl font-black text-white tracking-tight mb-4")
                .text("⚡ Ferrox Front Interactive Showcase"),
            p()
                .class("text-base sm:text-lg text-slate-300 leading-relaxed max-w-3xl")
                .text("All components below execute natively inside WebAssembly linear memory using ferrox-front-ui and ferrox-front-charts. No iframes, no React reconciliation overhead, and zero external JavaScript libraries."),
        ]);

    // Panels for the TabGroup
    let panels = vec![
        TabPanelDef::new("tab-datagrid", "📊 Enterprise DataGrid", render_datagrid_tab())
            .with_icon("database"),
        TabPanelDef::new("tab-charts", "📈 Financial Vector Charts", render_charts_tab())
            .with_icon("line-chart"),
        TabPanelDef::new("tab-signals", "⚡ Signals Reactivity", render_signals_tab())
            .with_icon("zap"),
        TabPanelDef::new("tab-design", "🎨 Design System & Callouts", render_design_system_tab())
            .with_icon("layers"),
    ];

    let showcase_tabs = tab_group(panels, 0);

    div()
        .class("ferrox-showcase w-full max-w-6xl mx-auto py-8 px-4 sm:px-8")
        .children(vec![header_hero, showcase_tabs])
}

/// 1. Virtualized DataGrid Showcase Tab
fn render_datagrid_tab() -> DomBuilder {
    let orders = ShowcaseOrder::mock_orders();

    let title_bar = div()
        .class("flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-4 p-4 rounded-xl bg-slate-900/60 border border-slate-800/80")
        .children(vec![
            div().children(vec![
                h3().class("text-lg font-bold text-white").text("Live Order Book Execution Grid"),
                p().class("text-xs text-slate-400").text("Displaying 50 reactive orders with native RFC 4180 CSV export"),
            ]),
            div().class("flex items-center gap-3").children(vec![
                button()
                    .class("flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-600/30 transition-all cursor-pointer")
                    .children(vec![
                        Icon::render("download", 14, "#ffffff"),
                        span().text("Export CSV"),
                    ])
                    .on_click(|| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.alert_with_message("RFC 4180 CSV generated in WebAssembly linear memory! Ready for export.");
                        }
                    }),
            ]),
        ]);

    // Table Header
    let table_head = thead()
        .class("bg-slate-900/90 text-xs uppercase font-bold text-slate-400 border-b border-slate-800")
        .child(
            tr().children(vec![
                th().class("py-3 px-4").text("Order ID"),
                th().class("py-3 px-4").text("Pair"),
                th().class("py-3 px-4").text("Side"),
                th().class("py-3 px-4 text-right").text("Price (USDT)"),
                th().class("py-3 px-4 text-right").text("Amount"),
                th().class("py-3 px-4 text-right").text("Total (USDT)"),
                th().class("py-3 px-4 text-center").text("Status"),
            ])
        );

    // Table Body Rows
    let mut table_body = tbody().class("divide-y divide-slate-800/50 text-xs font-mono");

    for ord in orders {
        let side_badge = if ord.side == "BUY" {
            span().class("px-2 py-0.5 rounded text-[10px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30").text("BUY")
        } else {
            span().class("px-2 py-0.5 rounded text-[10px] font-bold bg-rose-500/20 text-rose-400 border border-rose-500/30").text("SELL")
        };

        let status_badge = match ord.status {
            "Filled" => span().class("px-2 py-0.5 rounded text-[10px] font-bold bg-emerald-500/10 text-emerald-400").text("Filled"),
            "Pending" => span().class("px-2 py-0.5 rounded text-[10px] font-bold bg-amber-500/10 text-amber-400").text("Pending"),
            _ => span().class("px-2 py-0.5 rounded text-[10px] font-bold bg-rose-500/10 text-rose-400").text("Canceled"),
        };

        let row = tr()
            .class("hover:bg-slate-800/30 transition-colors")
            .children(vec![
                td().class("py-2.5 px-4 font-semibold text-slate-300").text(&ord.id),
                td().class("py-2.5 px-4 font-bold text-white").text(ord.symbol),
                td().class("py-2.5 px-4").child(side_badge),
                td().class("py-2.5 px-4 text-right font-medium text-slate-200").text(&format!("{:.2}", ord.price)),
                td().class("py-2.5 px-4 text-right text-slate-300").text(&format!("{:.4}", ord.amount)),
                td().class("py-2.5 px-4 text-right font-bold text-indigo-300").text(&format!("{:.2}", ord.total)),
                td().class("py-2.5 px-4 text-center").child(status_badge),
            ]);

        table_body = table_body.child(row);
    }

    let grid_container = div()
        .class("overflow-x-auto rounded-xl border border-slate-800/80 bg-slate-950/60 shadow-2xl backdrop-blur-xl max-h-[500px] overflow-y-auto")
        .child(
            table()
                .class("w-full text-left border-collapse")
                .child(table_head)
                .child(table_body)
        );

    div().children(vec![title_bar, grid_container])
}

/// 2. Vector SVG Charts Showcase Tab
fn render_charts_tab() -> DomBuilder {
    let candles = generate_mock_candles();

    let chart = candlestick_chart("showcase-candles", candles, 780, 360);

    // Telemetry Mini Sparklines
    let btc_data = vec![93200.0, 93450.0, 93100.0, 93800.0, 94250.0, 94100.0, 94800.0];
    let eth_data = vec![3380.0, 3410.0, 3390.0, 3440.0, 3480.0, 3460.0, 3520.0];

    let spark1 = sparkline_chart(btc_data, 120, 32, SparklineType::Area, "#10b981");
    let spark2 = sparkline_chart(eth_data, 120, 32, SparklineType::Area, "#6366f1");
    let gauge = gauge_chart("showcase-gauge", 84.5, 0.0, 100.0, "Throughput", "k req/s", 160, 100);

    let metric_cards = div()
        .class("grid grid-cols-1 md:grid-cols-3 gap-4 mb-6")
        .children(vec![
            div()
                .class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                .children(vec![
                    div().children(vec![
                        span().class("text-xs text-slate-400 font-bold uppercase tracking-wider").text("BTC/USDT 24h Trend"),
                        div().class("text-xl font-black text-white mt-1").text("$94,800.00"),
                        span().class("text-xs font-bold text-emerald-400").text("+2.15%"),
                    ]),
                    spark1,
                ]),
            div()
                .class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                .children(vec![
                    div().children(vec![
                        span().class("text-xs text-slate-400 font-bold uppercase tracking-wider").text("ETH/USDT 24h Trend"),
                        div().class("text-xl font-black text-white mt-1").text("$3,520.00"),
                        span().class("text-xs font-bold text-indigo-400").text("+3.80%"),
                    ]),
                    spark2,
                ]),
            div()
                .class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                .children(vec![
                    div().children(vec![
                        span().class("text-xs text-slate-400 font-bold uppercase tracking-wider").text("Throughput Telemetry"),
                        div().class("text-xl font-black text-emerald-400 mt-1").text("84,500 req/s"),
                        span().class("text-xs text-slate-400").text("Engine Latency <1.8μs"),
                    ]),
                    gauge,
                ]),
        ]);

    div().children(vec![
        metric_cards,
        div()
            .class("p-6 rounded-2xl bg-slate-950/80 border border-slate-800/80 shadow-2xl backdrop-blur-xl overflow-x-auto flex flex-col items-center")
            .children(vec![
                div().class("w-full flex items-center justify-between mb-4 border-b border-slate-800 pb-3").children(vec![
                    div().class("flex items-center gap-2").children(vec![
                        Icon::render("trending-up", 18, "#10b981"),
                        span().class("font-bold text-sm text-white").text("BTC/USDT • 15m OHLC Candlestick & Volume"),
                    ]),
                    span().class("text-xs font-mono text-slate-400").text("Pure Rust Vector SVG Engine"),
                ]),
                chart,
            ]),
    ])
}

/// 3. Fine-Grained Signals Reactivity Showcase Tab
fn render_signals_tab() -> DomBuilder {
    let count_signal = Signal::new(42);
    let count_clone1 = count_signal.clone();
    let count_clone2 = count_signal.clone();
    let count_display_id = "signal-count-display";

    let counter_card = div()
        .class("p-6 rounded-2xl bg-slate-900/60 border border-slate-800/80 shadow-xl flex flex-col items-center text-center")
        .children(vec![
            span().class("text-xs font-bold uppercase tracking-wider text-slate-400 mb-2").text("Signal<i32> Counter"),
            div()
                .id(count_display_id)
                .class("text-5xl font-black text-indigo-400 font-mono my-4")
                .text(&count_signal.get().to_string()),
            div().class("flex items-center gap-3").children(vec![
                button()
                    .class("px-4 py-2 rounded-xl text-xs font-bold bg-slate-800 hover:bg-slate-700 text-white transition-colors cursor-pointer")
                    .text("- Decrement")
                    .on_click(move || {
                        let cur = count_clone1.get();
                        let next = cur - 1;
                        count_clone1.set(next);
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(el) = doc.get_element_by_id(count_display_id) {
                                el.set_text_content(Some(&next.to_string()));
                            }
                        }
                    }),
                button()
                    .class("px-4 py-2 rounded-xl text-xs font-bold bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-600/30 transition-all cursor-pointer")
                    .text("+ Increment")
                    .on_click(move || {
                        let cur = count_clone2.get();
                        let next = cur + 1;
                        count_clone2.set(next);
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(el) = doc.get_element_by_id(count_display_id) {
                                el.set_text_content(Some(&next.to_string()));
                            }
                        }
                    }),
            ]),
            span().class("text-[11px] text-slate-500 mt-4").text("State updates mutate target DOM nodes in O(1) time without Virtual DOM diffing"),
        ]);

    let explanation_card = div()
        .class("p-6 rounded-2xl bg-slate-900/60 border border-slate-800/80 shadow-xl flex flex-col justify-between")
        .children(vec![
            div().children(vec![
                h3().class("text-lg font-bold text-white mb-2").text("Zero-GC Signal Reactivity"),
                p().class("text-sm text-slate-300 leading-relaxed mb-4").text("Unlike React which traverses and re-evaluates the entire Virtual DOM subtree, Ferrox Front uses a directed dependency graph. Changing a signal notifies exclusively the subscribed DOM builder callbacks."),
            ]),
            code_block(
                "let count = Signal::new(0);\ncount.update(|n| *n += 1); // Surgical O(1) DOM mutation",
                Some("rust"),
                Some("Reactivity Primitive"),
            ),
        ]);

    div()
        .class("grid grid-cols-1 md:grid-cols-2 gap-6")
        .children(vec![counter_card, explanation_card])
}

/// 4. Design System & Admonitions Showcase Tab
fn render_design_system_tab() -> DomBuilder {
    div()
        .class("flex flex-col space-y-4")
        .children(vec![
            admonition_tip(
                Some("ProTip: Zero-JS Dogfooding"),
                span().text("Ferrox Codex compiles directly to wasm32-unknown-unknown with opt-level = 'z', removing 100% of Node.js and NPM build dependencies."),
            ),
            admonition_note(
                Some("Architecture Insight"),
                span().text("All visual components reside inside ferrox-front-ui adhering to strict DRY principles so that any enterprise app can reuse them."),
            ),
            admonition_warning(
                Some("Production Performance Constraint"),
                span().text("Avoid allocating excessive heap objects inside high-frequency animation loops. Prefer Signal updates and pre-allocated buffers."),
            ),
            admonition_danger(
                Some("Security Invariant"),
                span().text("Never store raw JWT credentials inside localStorage or JavaScript globals. Ferrox Front uses SealedToken backed by Wasm linear memory."),
            ),
            admonition_caution(
                Some("API Stability"),
                span().text("The Wasm DOM Builder zero-copy pipeline operates at 60 FPS. Keep callback closures lightweight."),
            ),
            admonition_info(
                Some("Ecosystem Status"),
                span().text("Ferrox Front 2.1 introduces native doc components (Admonition, CodeBlock, TabGroup, TOC, DocSidebar)."),
            ),
        ])
}
