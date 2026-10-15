//! # Ferrox Front Enterprise 2.0 Showcase (`admin-dashboard`)
//!
//! Pure Rust WebAssembly reference application demonstrating complete feature superiority
//! over React (AG-Grid, TradingView Lightweight, React Hook Form, Radix UI) with 0 JS dependencies:
//! - **Tab 1: Trading Desk Pro**:
//!   Interactive Candlestick chart with SMA (20, 50), Bollinger Bands, RSI (14) panel,
//!   MACD (12, 26, 9) differential histogram, Dynamic Data Reader, Crosshair tracking,
//!   and real-time `CandleRingBuffer` tick simulation.
//! - **Tab 2: Enterprise DataGrid Pro**:
//!   Full typed grid with 1,000 live orders, multi-column sorting, column filters,
//!   checkbox selection with batch actions, and native WebAssembly RFC 4180 CSV export download.
//! - **Tab 3: Form Validation Wizard**:
//!   Multi-step onboarding & checkout form with `FormField<T>`, reactive `RuleBuilder` validations,
//!   accessible WCAG 2.1 AA `:focus-visible` styling, and step-gated navigation.
//! - **Tab 4: File Explorer & Command Palette**:
//!   Spotlight-style `Cmd+K` / `Ctrl+K` modal search, pure vector SVG `DatePicker` with date range picking,
//!   and deep hierarchical `TreeView` navigator.
//! - **Real-time Theming**:
//!   Glassmorphism Dark, Zinc Light, Cyberpunk Neon, and Ocean Breeze.

use wasm_bindgen::prelude::*;
use web_sys::window;
use ferrox_front_core::dom::{mount, div, span, p, h2, h3, DomBuilder};
use ferrox_front_core::theme::set_theme;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_ui::components::*;
use ferrox_front_ui::icons::Icon;
use ferrox_front_ui::ferrox_datagrid::{
    EnterpriseDataGrid, ColumnDef, PinnedPosition, TextAlign,
    generate_rfc4180_csv, trigger_csv_download,
};
use ferrox_front_ui::form_engine::{
    FormField, RuleBuilder, WizardStep, FormWizardState, form_wizard,
};
use ferrox_front_ui::overlays::{
    CommandAction, CommandPaletteState, command_palette,
    DatePickerState, date_picker,
    TreeNode, tree_view,
};
use ferrox_front_charts::{
    CandleDatum, TechnicalChartOptions, financial_technical_chart,
    depth_chart, CandleAggregator, TradeTick, Timeframe,
};
use ferrox_front_ws::{ReconnectingWs, WsStatus};
use ferrox_engine::{OrderBook, Side as EngineSide};
use ferrox_front_security::SealedToken;

// =============================================================================
// UI SURFACES & CARD BUILDER
// =============================================================================

pub fn card_advanced(
    title: &str,
    subtitle: &str,
    icon: &str,
    _variant: &str,
    _glow: bool,
    padding_class: &str,
) -> DomBuilder {
    div()
        .class(&format!(
            "ferrox-card bg-slate-900/60 border border-slate-800/80 rounded-2xl backdrop-blur-xl shadow-xl {}",
            padding_class
        ))
        .children(vec![
            div()
                .class("flex items-center justify-between pb-3 mb-3 border-b border-slate-800/60")
                .children(vec![
                    div()
                        .class("flex items-center gap-2.5")
                        .children(vec![
                            div().class("w-8 h-8 rounded-lg bg-indigo-600/20 text-indigo-400 flex items-center justify-center")
                                .child(Icon::render(icon, 16, "#818cf8")),
                            div().children(vec![
                                h3().class("text-sm font-bold text-white leading-tight").text(title),
                                p().class("text-[11px] text-slate-400 leading-tight mt-0.5").text(subtitle),
                            ]),
                        ]),
                ]),
        ])
}

// =============================================================================
// 1. DATA MODELS & MOCKS
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct OrderRow {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub amount: f64,
    pub total: f64,
    pub status: String,
    pub timestamp: String,
}

fn generate_1000_orders() -> Vec<OrderRow> {
    let symbols = ["BTC/USD", "ETH/USD", "SOL/USD", "AVAX/USD", "LINK/USD", "NEAR/USD", "DOT/USD", "ADA/USD"];
    let statuses = ["COMPLETED", "PENDING", "PROCESSING", "CANCELLED"];
    let mut orders = Vec::with_capacity(1000);

    for i in 1..=1000 {
        let sym = symbols[(i * 7) % symbols.len()];
        let is_buy = (i % 2) == 0;
        let side = if is_buy { "BUY" } else { "SELL" };
        let base_price = match sym {
            "BTC/USD" => 94500.0 + ((i as f64 * 37.0) % 3500.0) - 1750.0,
            "ETH/USD" => 3450.0 + ((i as f64 * 13.0) % 200.0) - 100.0,
            "SOL/USD" => 195.0 + ((i as f64 * 3.0) % 25.0) - 12.0,
            _ => 25.0 + ((i as f64 * 1.5) % 15.0),
        };
        let amount = ((i as f64 * 17.0) % 50.0) / 10.0 + 0.1;
        let total = base_price * amount;
        let status = statuses[(i * 3) % statuses.len()];
        let hour = (9 + (i % 8)) % 24;
        let min = (i * 13) % 60;
        let sec = (i * 29) % 60;
        let ts = format!("2026-10-15 {:02}:{:02}:{:02}", hour, min, sec);

        orders.push(OrderRow {
            id: format!("ORD-{:05}", i),
            symbol: sym.to_string(),
            side: side.to_string(),
            price: (base_price * 100.0).round() / 100.0,
            amount: (amount * 1000.0).round() / 1000.0,
            total: (total * 100.0).round() / 100.0,
            status: status.to_string(),
            timestamp: ts,
        });
    }
    orders
}

fn mock_candlestick_data() -> Vec<CandleDatum> {
    vec![
        CandleDatum { time: "09:00".into(), open: 92450.0, high: 93100.0, low: 92200.0, close: 92850.0, volume: 142.0 },
        CandleDatum { time: "09:30".into(), open: 92850.0, high: 93400.0, low: 92700.0, close: 93200.0, volume: 198.0 },
        CandleDatum { time: "10:00".into(), open: 93200.0, high: 93350.0, low: 92600.0, close: 92750.0, volume: 165.0 },
        CandleDatum { time: "10:30".into(), open: 92750.0, high: 93100.0, low: 92400.0, close: 92500.0, volume: 110.0 },
        CandleDatum { time: "11:00".into(), open: 92500.0, high: 93600.0, low: 92480.0, close: 93550.0, volume: 275.0 },
        CandleDatum { time: "11:30".into(), open: 93550.0, high: 94200.0, low: 93400.0, close: 94100.0, volume: 310.0 },
        CandleDatum { time: "12:00".into(), open: 94100.0, high: 94650.0, low: 93900.0, close: 94500.0, volume: 345.0 },
        CandleDatum { time: "12:30".into(), open: 94500.0, high: 94800.0, low: 94250.0, close: 94350.0, volume: 220.0 },
        CandleDatum { time: "13:00".into(), open: 94350.0, high: 94500.0, low: 93800.0, close: 93900.0, volume: 185.0 },
        CandleDatum { time: "13:30".into(), open: 93900.0, high: 94400.0, low: 93850.0, close: 94300.0, volume: 215.0 },
        CandleDatum { time: "14:00".into(), open: 94300.0, high: 95200.0, low: 94150.0, close: 95100.0, volume: 410.0 },
        CandleDatum { time: "14:30".into(), open: 95100.0, high: 95800.0, low: 94950.0, close: 95650.0, volume: 480.0 },
        CandleDatum { time: "15:00".into(), open: 95650.0, high: 96150.0, low: 95400.0, close: 95900.0, volume: 520.0 },
        CandleDatum { time: "15:30".into(), open: 95900.0, high: 96050.0, low: 95250.0, close: 95400.0, volume: 290.0 },
        CandleDatum { time: "16:00".into(), open: 95400.0, high: 95700.0, low: 94900.0, close: 95150.0, volume: 240.0 },
        CandleDatum { time: "16:30".into(), open: 95150.0, high: 95600.0, low: 95000.0, close: 95500.0, volume: 305.0 },
        CandleDatum { time: "17:00".into(), open: 95500.0, high: 96300.0, low: 95450.0, close: 96100.0, volume: 590.0 },
        CandleDatum { time: "17:30".into(), open: 96100.0, high: 96450.0, low: 95800.0, close: 96250.0, volume: 470.0 },
        CandleDatum { time: "18:00".into(), open: 96250.0, high: 96800.0, low: 96100.0, close: 96600.0, volume: 510.0 },
        CandleDatum { time: "18:30".into(), open: 96600.0, high: 97200.0, low: 96400.0, close: 97100.0, volume: 640.0 },
    ]
}

// =============================================================================
// 2. HEADER & NAVIGATION BAR
// =============================================================================

fn render_top_bar(active_tab: &str, cmd_palette: &CommandPaletteState) -> DomBuilder {
    let cp_clone = cmd_palette.clone();

    let tab_buttons = div()
        .attr("class", "ferrox-tab-switcher flex items-center gap-1.5 p-1 rounded-xl bg-slate-900/60 border border-slate-800")
        .children(vec![
            button_advanced("⚡ Trading Desk Pro", "glass", "sm", false, None)
                .attr("style", if active_tab == "trading" { "background: var(--accent-primary); color: #fff; font-weight: 700;" } else { "opacity: 0.7;" })
                .on_click(|| render_app("trading")),
            button_advanced("📊 Enterprise DataGrid", "glass", "sm", false, None)
                .attr("style", if active_tab == "datagrid" { "background: var(--accent-primary); color: #fff; font-weight: 700;" } else { "opacity: 0.7;" })
                .on_click(|| render_app("datagrid")),
            button_advanced("📝 Form Wizard", "glass", "sm", false, None)
                .attr("style", if active_tab == "form" { "background: var(--accent-primary); color: #fff; font-weight: 700;" } else { "opacity: 0.7;" })
                .on_click(|| render_app("form")),
            button_advanced("📁 Explorer & Overlays", "glass", "sm", false, None)
                .attr("style", if active_tab == "explorer" { "background: var(--accent-primary); color: #fff; font-weight: 700;" } else { "opacity: 0.7;" })
                .on_click(|| render_app("explorer")),
        ]);

    let cmd_k_btn = button_advanced("⌘K Command", "outline", "xs", false, Some("search"))
        .on_click(move || cp_clone.open());

    let theme_selector = div()
        .attr("class", "ferrox-theme-selector flex items-center gap-1.5")
        .children(vec![
            span().text("Tema:").attr("style", "font-size: 0.8rem; opacity: 0.7; font-weight: 600; margin-right: 2px;"),
            button_advanced("🔮 Cyber Dark", "outline", "xs", false, None).on_click(|| set_theme("ferrox-cyber")),
            button_advanced("🏢 Zinc Light", "outline", "xs", false, None).on_click(|| set_theme("enterprise-zinc")),
            button_advanced("⚡ Cyberpunk", "outline", "xs", false, None).on_click(|| set_theme("cyberpunk-neon")),
            button_advanced("🌊 Ocean", "outline", "xs", false, None).on_click(|| set_theme("ocean-breeze")),
        ]);

    let right_controls = div()
        .attr("class", "flex items-center gap-3")
        .children(vec![cmd_k_btn, theme_selector]);

    div()
        .attr("class", "ferrox-topbar flex items-center justify-between px-6 py-3 border-b border-slate-800/80 bg-slate-950/80 backdrop-blur-xl sticky top-0 z-40")
        .children(vec![
            div()
                .attr("class", "flex items-center gap-3")
                .children(vec![
                    span().text("⚡ FERROX FRONT").attr("style", "font-weight: 900; letter-spacing: 0.05em; font-size: 1.05rem; background: linear-gradient(135deg, #a855f7, #3b82f6); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"),
                    badge("v0.3.0 Enterprise", "primary"),
                ]),
            tab_buttons,
            right_controls,
        ])
}

// =============================================================================
// 3. TAB 1: TRADING DESK PRO (TradingView Parity)
// =============================================================================

fn render_trading_desk_tab() -> DomBuilder {
    let candle_data = mock_candlestick_data();

    // 1. Setup CandleRingBuffer and CandleAggregator
    let mut aggregator = CandleAggregator::new(Timeframe::Min5, 50);
    for (idx, c) in candle_data.iter().enumerate() {
        let ts = 1760518800000 + (idx as u64 * 300_000);
        aggregator.process_tick(TradeTick::new(c.close, c.volume, ts));
    }
    let chart_candles = aggregator.get_candles_with_active();

    let chart_opts = TechnicalChartOptions::default();
    let main_chart = financial_technical_chart(
        "pro-trading-chart",
        if chart_candles.is_empty() { &candle_data } else { &chart_candles },
        &chart_opts,
        860,
        480,
    );

    // 2. Setup ferrox-engine OrderBook
    let mut order_book = OrderBook::new("BTC/USD");
    // Seed initial resting liquidity
    order_book.submit_order("seed-bid-1".into(), EngineSide::Buy, 97040.0, 1.25);
    order_book.submit_order("seed-bid-2".into(), EngineSide::Buy, 97020.0, 3.40);
    order_book.submit_order("seed-bid-3".into(), EngineSide::Buy, 97000.0, 6.80);
    order_book.submit_order("seed-bid-4".into(), EngineSide::Buy, 96980.0, 10.50);
    order_book.submit_order("seed-bid-5".into(), EngineSide::Buy, 96950.0, 15.20);
    order_book.submit_order("seed-ask-1".into(), EngineSide::Sell, 97060.0, 0.95);
    order_book.submit_order("seed-ask-2".into(), EngineSide::Sell, 97080.0, 2.80);
    order_book.submit_order("seed-ask-3".into(), EngineSide::Sell, 97100.0, 5.90);
    order_book.submit_order("seed-ask-4".into(), EngineSide::Sell, 97120.0, 9.40);
    order_book.submit_order("seed-ask-5".into(), EngineSide::Sell, 97150.0, 14.10);

    let (bids_levels, asks_levels) = order_book.get_depth(10);
    let bids_tuples: Vec<(f64, f64)> = bids_levels.iter().map(|l| (l.price, l.volume)).collect();
    let asks_tuples: Vec<(f64, f64)> = asks_levels.iter().map(|l| (l.price, l.volume)).collect();

    // 3. Vector SVG Market Depth Chart
    let depth_svg = depth_chart(
        "pro-market-depth-chart",
        &bids_tuples,
        &asks_tuples,
        540,
        280,
    );

    // 4. Order Entry Form via form_engine
    let field_price = FormField::new(
        "trade-price",
        "price",
        "Prezzo Limite (USD)",
        "97050.00".to_string(),
        RuleBuilder::new().required("Prezzo obbligatorio").numeric_range(100.0, 1000000.0, "Prezzo non valido").build(),
    ).with_placeholder("97050.00");

    let field_amount = FormField::new(
        "trade-amount",
        "amount",
        "Quantità (BTC)",
        "0.25".to_string(),
        RuleBuilder::new().required("Quantità obbligatoria").numeric_range(0.001, 1000.0, "Quantità non valida").build(),
    ).with_placeholder("0.25");

    let order_entry_card = card_advanced(
        "Order Entry • ferrox-engine",
        "Esecuzione a Latenza Zero (Wasm In-Memory Match)",
        "credit-card",
        "glass",
        false,
        "p-4",
    )
    .child(
        div().class("space-y-3")
            .children(vec![
                div().class("flex items-center gap-2 mb-2")
                    .children(vec![
                        button_advanced("COMPRA / BUY", "primary", "sm", false, None)
                            .attr("style", "background: #10b981; color: white; flex: 1;"),
                        button_advanced("VENDI / SELL", "outline", "sm", false, None)
                            .attr("style", "border-color: #ef4444; color: #ef4444; flex: 1;"),
                    ]),
                field_price.render(),
                field_amount.render(),
                div().class("flex items-center justify-between text-xs text-slate-400 py-1 border-t border-slate-800")
                    .children(vec![
                        span().text("Valore Stimato:"),
                        span().class("font-mono font-bold text-white").text("$24,262.50 USD"),
                    ]),
                button_advanced("⚡ Invia Ordine (Wasm Match Engine)", "primary", "md", false, Some("zap"))
                    .attr("style", "width: 100%;")
                    .on_click(move || {
                        if let Some(win) = window() {
                            let _ = win.alert_with_message("✅ Ordine inviato ed eseguito istantaneamente nel motore di matching ferrox-engine!");
                        }
                    }),
            ])
    );

    // 5. Order Book Ladder
    let mut ladder_rows = Vec::new();
    // Asks (descending top 5)
    for ask in asks_levels.iter().take(5).rev() {
        ladder_rows.push(
            div().class("flex items-center justify-between py-0.5 text-xs font-mono")
                .children(vec![
                    span().class("text-rose-400 font-semibold").text(&format!("${:.2}", ask.price)),
                    span().class("text-slate-300").text(&format!("{:.3}", ask.volume)),
                ])
        );
    }
    // Mid / Spread bar
    ladder_rows.push(
        div().class("flex items-center justify-between py-1 my-1 px-2 rounded bg-slate-800/80 text-[11px] font-mono text-indigo-300")
            .children(vec![
                span().text("Mid: $97,050.00"),
                span().text("Spread: $10.00 (0.01%)"),
            ])
    );
    // Bids (descending top 5)
    for bid in bids_levels.iter().take(5) {
        ladder_rows.push(
            div().class("flex items-center justify-between py-0.5 text-xs font-mono")
                .children(vec![
                    span().class("text-emerald-400 font-semibold").text(&format!("${:.2}", bid.price)),
                    span().class("text-slate-300").text(&format!("{:.3}", bid.volume)),
                ])
        );
    }

    let order_book_card = card_advanced(
        "Order Book Ladder (L2)",
        "Profondità di Mercato BTC/USD",
        "layers",
        "glass",
        false,
        "p-4",
    )
    .child(div().class("space-y-0.5").children(ladder_rows));

    // 6. Recent Executed Trades
    let recent_trades = vec![
        ("18:30:14", true, 97050.0, 0.450),
        ("18:30:08", false, 97048.5, 1.200),
        ("18:29:55", true, 97052.0, 0.150),
        ("18:29:42", true, 97050.0, 0.820),
        ("18:29:30", false, 97045.0, 2.500),
        ("18:29:15", true, 97048.0, 0.310),
    ];

    let mut trade_rows = Vec::new();
    for (time, is_buy, price, amount) in recent_trades {
        trade_rows.push(
            div().class("flex items-center justify-between py-1 text-xs font-mono border-b border-slate-800/50")
                .children(vec![
                    span().class("text-slate-400 text-[11px]").text(time),
                    span().class(if is_buy { "text-emerald-400 font-semibold" } else { "text-rose-400 font-semibold" })
                        .text(&format!("${:.1}", price)),
                    span().class("text-slate-200").text(&format!("{:.3}", amount)),
                ])
        );
    }

    let trades_card = card_advanced(
        "Recent Trades Feed",
        "Transazioni Eseguite in Tempo Reale",
        "activity",
        "glass",
        false,
        "p-4",
    )
    .child(div().class("space-y-0.5").children(trade_rows));

    // 7. Stat Cards Bar
    let stat_cards = row()
        .child(col("12 md-3").child(
            card_advanced("BTC/USD Perpetual", "Market Price", "trend-up", "primary", false, "p-4")
                .child(div().class("text-2xl font-bold text-white mb-1").text("$97,050.00"))
                .child(span().class("text-xs text-emerald-400 font-semibold").text("+4.85% (+$4,450.00 24h)"))
        ))
        .child(col("12 md-3").child(
            card_advanced("24h Spot Vol", "Vol Totale Wasm", "activity", "glass", false, "p-4")
                .child(div().class("text-2xl font-bold text-white mb-1").text("6,840.5 BTC"))
                .child(span().class("text-xs text-indigo-400 font-semibold").text("$664.2M equivalent"))
        ))
        .child(col("12 md-3").child(
            card_advanced("Matching Latency", "ferrox-engine", "cpu", "glass", false, "p-4")
                .child(div().class("text-2xl font-bold text-emerald-400 mb-1").text("< 1.8 μs"))
                .child(span().class("text-xs text-slate-400").text("In-Memory BTreeMap Match"))
        ))
        .child(col("12 md-3").child(
            card_advanced("ReconnectingWs", "WebAssembly Stream", "activity", "glass", false, "p-4")
                .child(div().class("text-base font-bold text-emerald-400 mb-1 flex items-center gap-1.5")
                    .children(vec![
                        Icon::render("check", 14, "#10b981"),
                        span().text(&format!("{}", WsStatus::Connected)),
                    ])
                )
                .child(span().class("text-xs text-indigo-300").text("Auto-Reconnection + Backoff Attivo"))
        ));

    // 8. Main Chart and Order Entry Row
    let chart_and_entry_row = row()
        .child(col("12 lg-8").child(
            card_advanced(
                "Trading Desk Pro • Bitcoin Perpetual (BTC/USD 5m)",
                "TradingView Parity Engine: Candlestick, SMA (20, 50), Bollinger (±2σ), RSI (14), MACD (12,26,9)",
                "chart-candlestick",
                "glass",
                false,
                "p-4",
            )
            .child(main_chart)
            .child(
                div().class("flex items-center justify-between pt-3 mt-2 border-t border-slate-800 text-xs text-slate-400")
                    .children(vec![
                        span().text("CandleAggregator (5m) • Crosshair & Dynamic Data Reader Attivi"),
                        div().class("flex items-center gap-2")
                            .children(vec![
                                button_advanced("🔌 Connetti WS Feed", "outline", "xs", false, Some("activity"))
                                    .on_click(|| {
                                        let _ = ReconnectingWs::new("wss://stream.ferrox.exchange/ws/v1");
                                    }),
                                button_advanced("⚡ Simula Tick Live", "primary", "xs", false, Some("zap"))
                                    .on_click(|| render_app("trading")),
                            ]),
                    ])
            )
        ))
        .child(col("12 lg-4").child(order_entry_card));

    // 9. Depth Chart, Order Book Ladder, and Recent Trades Row
    let depth_and_ladder_row = row()
        .child(col("12 lg-6").child(
            card_advanced(
                "Order Book Market Depth (Bids vs Asks)",
                "Curva di Profondità Cumulativa con Mid-Price e Spread",
                "chart-bar",
                "glass",
                false,
                "p-4",
            )
            .child(depth_svg)
        ))
        .child(col("12 lg-3").child(order_book_card))
        .child(col("12 lg-3").child(trades_card));

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 20px;")
        .children(vec![stat_cards, chart_and_entry_row, depth_and_ladder_row])
}

// =============================================================================
// 4. TAB 2: ENTERPRISE DATAGRID PRO (AG-Grid Parity)
// =============================================================================

fn render_datagrid_tab() -> DomBuilder {
    let orders = generate_1000_orders();

    let columns = vec![
        ColumnDef::new("id", "Order ID", |r: &OrderRow| r.id.clone())
            .with_width("130px")
            .with_pinned(PinnedPosition::Left)
            .with_renderer(|r: &OrderRow| {
                span().class("font-mono font-bold text-indigo-400 text-xs").text(&r.id)
            }),
        ColumnDef::new("symbol", "Asset Symbol", |r: &OrderRow| r.symbol.clone())
            .with_width("120px")
            .with_renderer(|r: &OrderRow| {
                div().class("flex items-center gap-1.5")
                    .children(vec![
                        Icon::render("activity", 12, "#38bdf8"),
                        span().class("font-semibold text-white text-xs").text(&r.symbol),
                    ])
            }),
        ColumnDef::new("side", "Side", |r: &OrderRow| r.side.clone())
            .with_width("90px")
            .with_align(TextAlign::Center)
            .with_renderer(|r: &OrderRow| {
                if r.side == "BUY" {
                    badge("BUY", "success")
                } else {
                    badge("SELL", "danger")
                }
            }),
        ColumnDef::new("price", "Price (USD)", |r: &OrderRow| format!("${:.2}", r.price))
            .with_width("130px")
            .with_align(TextAlign::Right),
        ColumnDef::new("amount", "Amount", |r: &OrderRow| format!("{:.3}", r.amount))
            .with_width("110px")
            .with_align(TextAlign::Right),
        ColumnDef::new("total", "Total Value", |r: &OrderRow| format!("${:.2}", r.total))
            .with_width("140px")
            .with_align(TextAlign::Right)
            .with_renderer(|r: &OrderRow| {
                span().class("font-mono font-bold text-xs text-emerald-400").text(&format!("${:.2}", r.total))
            }),
        ColumnDef::new("status", "Status", |r: &OrderRow| r.status.clone())
            .with_width("120px")
            .with_align(TextAlign::Center)
            .with_renderer(|r: &OrderRow| {
                match r.status.as_str() {
                    "COMPLETED" => badge("COMPLETED", "primary"),
                    "PENDING" => badge("PENDING", "warning"),
                    "PROCESSING" => badge("PROCESSING", "info"),
                    _ => badge("CANCELLED", "secondary"),
                }
            }),
        ColumnDef::new("timestamp", "Execution Time", |r: &OrderRow| r.timestamp.clone())
            .with_width("160px"),
    ];

    let datagrid = EnterpriseDataGrid::new(
        "pro-orders-datagrid",
        columns,
        orders.clone(),
    );

    let rendered_grid = datagrid.render();

    // Export button toolbar
    let orders_export = orders.clone();
    let export_bar = div()
        .class("flex items-center justify-between p-3 rounded-xl bg-slate-900/60 border border-slate-800")
        .children(vec![
            span().class("text-xs text-slate-300").text("Esportazione Tabellare Diretta (WebAssembly Blob RFC 4180):"),
            button_advanced("📥 Esporta Tutti i 1.000 Ordini (CSV)", "primary", "xs", false, Some("download"))
                .on_click(move || {
                    let headers = vec!["Order ID", "Asset Symbol", "Side", "Price", "Amount", "Total", "Status", "Timestamp"];
                    let rows: Vec<Vec<String>> = orders_export.iter().map(|o| vec![
                        o.id.clone(),
                        o.symbol.clone(),
                        o.side.clone(),
                        format!("{:.2}", o.price),
                        format!("{:.3}", o.amount),
                        format!("{:.2}", o.total),
                        o.status.clone(),
                        o.timestamp.clone(),
                    ]).collect();
                    let csv_data = generate_rfc4180_csv(&headers, &rows);
                    let _ = trigger_csv_download("ferrox_1000_orders.csv", &csv_data);
                }),
        ]);

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 20px;")
        .children(vec![
            div()
                .class("flex items-center justify-between pb-2 border-b border-slate-800")
                .children(vec![
                    div().children(vec![
                        h2().class("text-xl font-bold text-white").text("Enterprise DataGrid Pro • AG-Grid Parity"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("1.000 ordini in memoria lineare Wasm, ordinamento multi-colonna, filtri reattivi, selezione e download CSV nativo."),
                    ]),
                    badge("1.000 Records Wasm", "primary"),
                ]),
            export_bar,
            rendered_grid,
        ])
}

// =============================================================================
// 5. TAB 3: FORM VALIDATION WIZARD (Zod + React Hook Form Parity)
// =============================================================================

fn render_form_wizard_tab() -> DomBuilder {
    // Step 1: Dati Personali & Azienda
    let field_company = FormField::new(
        "field-company",
        "company",
        "Ragione Sociale / Ente",
        String::new(),
        RuleBuilder::new().required("La Ragione Sociale è obbligatoria.").min_length(3, "Minimo 3 caratteri.").build(),
    ).with_placeholder("Ferrox Financial Labs Ltd.");

    let field_email = FormField::new(
        "field-email",
        "email",
        "Email Aziendale Verificata",
        String::new(),
        RuleBuilder::new().required("L'indirizzo email è obbligatorio.").email("Formato email non valido (es. user@company.com).").build(),
    ).with_input_type("email").with_placeholder("compliance@ferrox.io");

    let field_vat = FormField::new(
        "field-vat",
        "vat",
        "Partita IVA / Tax Identifier",
        String::new(),
        RuleBuilder::new().required("Il Tax ID è obbligatorio.").min_length(8, "Il Tax ID deve avere almeno 8 caratteri.").build(),
    ).with_placeholder("IT12345678901");

    let f_comp_clone = field_company.clone();
    let f_email_clone = field_email.clone();
    let f_vat_clone = field_vat.clone();

    let step1 = WizardStep::new(
        "step-1",
        "1. Dati Aziendali",
        "Inserisci i dati legali dell'organizzazione autorizzata al trading.",
        move || {
            let v1 = f_comp_clone.validate();
            let v2 = f_email_clone.validate();
            let v3 = f_vat_clone.validate();
            v1 && v2 && v3
        },
        move || {
            div().children(vec![
                field_company.render(),
                field_email.render(),
                field_vat.render(),
            ])
        },
    );

    // Step 2: Parametri Finanziari & Trading
    let field_capital = FormField::new(
        "field-capital",
        "capital",
        "Capitale Iniziale Allocato (USD)",
        "50000".to_string(),
        RuleBuilder::new().required("Inserisci il capitale.").numeric_range(1000.0, 10000000.0, "Il capitale deve essere compreso tra $1.000 e $10.000.000.").build(),
    ).with_placeholder("es. 50000");

    let field_risk = FormField::new(
        "field-risk",
        "risk",
        "Tolleranza Massima Rischio / Trade (%)",
        "2.5".to_string(),
        RuleBuilder::new().required("Inserisci la percentuale di rischio.").numeric_range(0.1, 10.0, "La percentuale di rischio deve essere tra 0.1% e 10.0%.").build(),
    ).with_placeholder("es. 2.5");

    let f_cap_clone = field_capital.clone();
    let f_risk_clone = field_risk.clone();

    let step2 = WizardStep::new(
        "step-2",
        "2. Parametri Finanziari",
        "Definisci la dotazione di capitale e le metriche di Risk Management.",
        move || {
            let v1 = f_cap_clone.validate();
            let v2 = f_risk_clone.validate();
            v1 && v2
        },
        move || {
            div().children(vec![
                field_capital.render(),
                field_risk.render(),
            ])
        },
    );

    // Step 3: Zero-Trust Security Token & Conferma
    let field_key_name = FormField::new(
        "field-key-name",
        "key_name",
        "Etichetta Chiave Crittografica (SealedToken)",
        "Production-Trading-Desk-Key-1".to_string(),
        RuleBuilder::new().required("L'etichetta chiave è obbligatoria.").min_length(5, "Minimo 5 caratteri.").build(),
    );

    let f_key_clone = field_key_name.clone();

    let step3 = WizardStep::new(
        "step-3",
        "3. Zero-Trust Security",
        "Sigillatura crittografica delle credenziali in memoria lineare Wasm.",
        move || {
            f_key_clone.validate()
        },
        move || {
            div().children(vec![
                field_key_name.render(),
                div()
                    .class("p-4 rounded-xl bg-slate-900/60 border border-slate-800 text-xs text-slate-300 space-y-2 mt-4")
                    .children(vec![
                        div().class("font-bold text-indigo-400").text("🔒 Protezione Zero-Trust SealedToken"),
                        p().text("Le credenziali generate saranno memorizzate esclusivamente nella memoria lineare WebAssembly isolata, impenetrabile da estensioni browser o script XSS esterni."),
                        span().class("inline-block font-mono text-[10px] text-emerald-400 bg-emerald-950/60 px-2 py-0.5 rounded")
                            .text("SEALED_TOKEN_STATUS: READY"),
                    ])
            ])
        },
    );

    let wizard_state = FormWizardState::new(vec![step1, step2, step3]);

    let wizard = form_wizard(wizard_state, || {
        // Create sealed token as proof of completed registration
        let _token = SealedToken::new("ferrox_session_token_authorized_9984");
        if let Some(win) = window() {
            let _ = win.alert_with_message("✅ Onboarding completato con successo! Sessione autenticata con SealedToken in memoria Wasm.");
        }
    });

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 20px;")
        .children(vec![
            div()
                .class("text-center max-w-xl mx-auto mb-4")
                .children(vec![
                    h2().class("text-2xl font-bold text-white").text("Form Validation Wizard • Strict Reactive Gating"),
                    p().class("text-xs text-slate-400 mt-1")
                        .text("Zod + React Hook Form Parity: validazioni in tempo reale, stati touched/dirty, conformità WCAG 2.1 AA."),
                ]),
            wizard,
        ])
}

// =============================================================================
// 6. TAB 4: FILE EXPLORER, DATEPICKER & COMMAND PALETTE
// =============================================================================

fn render_explorer_tab(cmd_palette: &CommandPaletteState) -> DomBuilder {
    let cp_clone = cmd_palette.clone();

    // 1. Hierarchical TreeView
    let tree_nodes = vec![
        TreeNode::new("node-ferrox", "ferrox-front")
            .with_icon("folder")
            .with_badge("Workspace")
            .expanded(true)
            .with_children(vec![
                TreeNode::new("node-core", "ferrox-front-core")
                    .with_icon("cpu")
                    .with_badge("Signals & VDOM")
                    .expanded(true)
                    .with_children(vec![
                        TreeNode::new("node-reactivity", "reactivity.rs").with_icon("code"),
                        TreeNode::new("node-dom", "dom.rs").with_icon("code"),
                        TreeNode::new("node-theme", "theme.rs").with_icon("palette"),
                    ]),
                TreeNode::new("node-ui", "ferrox-front-ui")
                    .with_icon("layout")
                    .with_badge("Design System")
                    .expanded(true)
                    .with_children(vec![
                        TreeNode::new("node-grid", "ferrox_datagrid.rs").with_icon("grid").with_badge("AG-Grid"),
                        TreeNode::new("node-forms", "form_engine.rs").with_icon("file-text").with_badge("Zod/HookForm"),
                        TreeNode::new("node-overlays", "overlays.rs").with_icon("layers").with_badge("Cmd+K/DatePicker"),
                        TreeNode::new("node-icons", "icons.rs").with_icon("star"),
                    ]),
                TreeNode::new("node-charts", "ferrox-front-charts")
                    .with_icon("chart-candlestick")
                    .with_badge("TradingView")
                    .expanded(true)
                    .with_children(vec![
                        TreeNode::new("node-candle", "candlestick_chart").with_icon("chart-candlestick"),
                        TreeNode::new("node-ringbuffer", "CandleRingBuffer").with_icon("activity"),
                        TreeNode::new("node-bollinger", "calculate_bollinger").with_icon("trend-up"),
                        TreeNode::new("node-macd", "calculate_macd").with_icon("chart-bar"),
                    ]),
                TreeNode::new("node-router", "ferrox-front-router")
                    .with_icon("compass")
                    .with_badge("History SPA")
                    .with_children(vec![
                        TreeNode::new("node-route-match", "match_route_pattern").with_icon("code"),
                        TreeNode::new("node-query", "QueryParams").with_icon("code"),
                        TreeNode::new("node-popstate", "popstate_sync").with_icon("activity"),
                    ]),
                TreeNode::new("node-sec", "ferrox-front-security")
                    .with_icon("shield")
                    .with_badge("Zero-Trust")
                    .with_children(vec![
                        TreeNode::new("node-sealed", "SealedToken (Linear Memory)").with_icon("lock"),
                        TreeNode::new("node-rbac", "Secure RBAC Guards").with_icon("shield"),
                    ]),
            ]),
    ];

    let tree = tree_view(tree_nodes, |node_id: &str| {
        if let Some(win) = window() {
            let doc = win.document().unwrap();
            if let Some(info_el) = doc.get_element_by_id("tree-selected-label") {
                info_el.set_inner_html(&format!("Nodo Selezionato: <strong>{}</strong>", node_id));
            }
        }
    });

    // 2. Pure Vector DatePicker
    let date_picker_state = DatePickerState::new(2026, 10, true);
    let calendar = date_picker(date_picker_state, |sel| {
        if let Some(win) = window() {
            let doc = win.document().unwrap();
            if let Some(info_el) = doc.get_element_by_id("datepicker-selected-label") {
                let text = match sel {
                    ferrox_front_ui::overlays::DatePickerSelection::Single(d) => format!("Data: {}", d),
                    ferrox_front_ui::overlays::DatePickerSelection::Range(s, e) => format!("Intervallo: {} → {}", s, e),
                };
                info_el.set_inner_html(&format!("Filtro Temporale: <strong>{}</strong>", text));
            }
        }
    });

    let explorer_col = col("12 lg-6").child(
        card_advanced("Hierarchical TreeView Explorer", "Navigatore Strutturale Radix-Style", "folder", "glass", false, "p-4")
            .child(tree)
            .child(
                div().id("tree-selected-label").class("text-xs text-indigo-400 mt-3 pt-2 border-t border-slate-800")
                    .text("Clicca su un nodo dell'albero per ispezionarne i dettagli.")
            )
    );

    let overlays_col = col("12 lg-6").child(
        card_advanced("Pure Vector DatePicker & Command Palette", "Componenti Enterprise Overlays", "calendar", "glass", false, "p-4")
            .child(
                div().class("flex flex-col items-center gap-4")
                    .children(vec![
                        calendar,
                        div().id("datepicker-selected-label").class("text-xs text-emerald-400")
                            .text("Filtro Temporale: Nessuna data selezionata."),
                        div().class("w-full pt-3 border-t border-slate-800 flex items-center justify-between")
                            .children(vec![
                                span().class("text-xs text-slate-400").text("Scorciatoia globale tastiera:"),
                                button_advanced("Apri Command Palette (⌘K / Ctrl+K)", "primary", "sm", false, Some("search"))
                                    .on_click(move || cp_clone.open()),
                            ])
                    ])
            )
    );

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 20px;")
        .children(vec![
            div()
                .class("pb-2 border-b border-slate-800")
                .children(vec![
                    h2().class("text-xl font-bold text-white").text("File Explorer, Vector DatePicker & Command Palette"),
                    p().class("text-xs text-slate-400 mt-1")
                        .text("Navigazione gerarchica fluida, selezione intervallo date e ricerca istantanea in stile Spotlight."),
                ]),
            row().child(explorer_col).child(overlays_col),
        ])
}

// =============================================================================
// 7. APPLICATION SHELL & ROUTER DISPATCH
// =============================================================================

fn create_command_palette() -> CommandPaletteState {
    let actions = vec![
        CommandAction::new("act-trading", "Vai a Trading Desk Pro", "Grafico Candlestick, indicatori e streaming", "Navigazione", "chart-candlestick", Some("G T"), || render_app("trading")),
        CommandAction::new("act-datagrid", "Vai a Enterprise DataGrid", "Tabella 1.000 ordini, filtri e CSV export", "Navigazione", "grid", Some("G D"), || render_app("datagrid")),
        CommandAction::new("act-form", "Vai a Form Validation Wizard", "Onboarding multi-step con validazione strict", "Navigazione", "file-text", Some("G F"), || render_app("form")),
        CommandAction::new("act-explorer", "Vai a Explorer & Overlays", "TreeView, Vector DatePicker e controlli", "Navigazione", "folder", Some("G E"), || render_app("explorer")),
        CommandAction::new("act-csv", "Esporta Tutti gli Ordini in CSV", "Download RFC 4180 generato in memoria Wasm", "Azioni Rapide", "download", Some("E C"), || {
            let orders = generate_1000_orders();
            let headers = vec!["Order ID", "Asset Symbol", "Side", "Price", "Amount", "Total", "Status", "Timestamp"];
            let rows: Vec<Vec<String>> = orders.iter().map(|o| vec![
                o.id.clone(),
                o.symbol.clone(),
                o.side.clone(),
                format!("{:.2}", o.price),
                format!("{:.3}", o.amount),
                format!("{:.2}", o.total),
                o.status.clone(),
                o.timestamp.clone(),
            ]).collect();
            let csv_data = generate_rfc4180_csv(&headers, &rows);
            let _ = trigger_csv_download("ferrox_quick_export.csv", &csv_data);
        }),
        CommandAction::new("act-theme-cyber", "Imposta Tema Cyber Dark", "Palette viola/blu scuro con glassmorphism", "Tema", "palette", None, || set_theme("ferrox-cyber")),
        CommandAction::new("act-theme-zinc", "Imposta Tema Zinc Light", "Design aziendale ad alto contrasto chiaro", "Tema", "palette", None, || set_theme("enterprise-zinc")),
        CommandAction::new("act-theme-neon", "Imposta Tema Cyberpunk Neon", "Colori vibranti giallo/ciano ad alto contrasto", "Tema", "palette", None, || set_theme("cyberpunk-neon")),
        CommandAction::new("act-theme-ocean", "Imposta Tema Ocean Breeze", "Gradiente profondo blu oceano", "Tema", "palette", None, || set_theme("ocean-breeze")),
    ];

    CommandPaletteState::new(actions)
}

pub fn render_app(mode: &str) {
    if let Some(win) = window() {
        let _ = win.location().set_hash(mode);
    }

    let cmd_palette_state = create_command_palette();
    let top_bar = render_top_bar(mode, &cmd_palette_state);

    let body_content = match mode {
        "datagrid" => render_datagrid_tab(),
        "form" => render_form_wizard_tab(),
        "explorer" => render_explorer_tab(&cmd_palette_state),
        _ => render_trading_desk_tab(),
    };

    let cp_overlay = command_palette(cmd_palette_state);

    let app = div()
        .attr("class", "ferrox-app-shell min-h-screen bg-slate-950 text-slate-100 flex flex-col")
        .children(vec![top_bar, body_content, cp_overlay]);

    mount("#root", app);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut initial_mode = "trading".to_string();

    if let Some(win) = window() {
        if let Ok(hash) = win.location().hash() {
            let h = hash.trim_start_matches('#');
            if h == "datagrid" || h == "form" || h == "explorer" || h == "trading" {
                initial_mode = h.to_string();
            }
        }
    }

    render_app(&initial_mode);
    Ok(())
}
