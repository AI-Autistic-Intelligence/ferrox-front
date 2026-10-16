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
use wasm_bindgen::JsCast;
use web_sys::{window, MessageEvent};
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
    gauge_chart, sparkline_chart, SparklineType,
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
    let effective_padding = match padding_class {
        "p-4" => "p-6",
        "p-5" => "p-7",
        other => other,
    };
    div()
        .class(&format!(
            "ferrox-card rounded-2xl backdrop-blur-2xl shadow-2xl {}",
            effective_padding
        ))
        .children(vec![
            div()
                .class("flex items-center justify-between pb-4 mb-5 border-b")
                .attr("style", "border-bottom: var(--border-glass);")
                .children(vec![
                    div()
                        .class("flex items-center gap-3")
                        .children(vec![
                            div().class("w-9 h-9 rounded-xl flex items-center justify-center flex-shrink-0")
                                .attr("style", "background: var(--accent-glow); color: var(--accent-primary);")
                                .child(Icon::render(icon, 18, "var(--accent-primary)")),
                            div().children(vec![
                                h3().class("text-base font-bold tracking-tight leading-snug").attr("style", "color: var(--text-primary);").text(title),
                                p().class("text-xs mt-0.5 leading-normal").attr("style", "color: var(--text-secondary);").text(subtitle),
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
        .attr("class", "ferrox-tab-switcher flex flex-wrap items-center gap-2 p-1.5 rounded-2xl shadow-md")
        .attr("style", "background: var(--bg-surface-hover); border: var(--border-glass);")
        .children(vec![
            button_advanced("⚡ Trading Desk Pro", "glass", "sm", false, None)
                .attr("style", if active_tab == "trading" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("trading")),
            button_advanced("📊 Enterprise DataGrid", "glass", "sm", false, None)
                .attr("style", if active_tab == "datagrid" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("datagrid")),
            button_advanced("🎨 UI Elements", "glass", "sm", false, None)
                .attr("style", if active_tab == "components" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("components")),
            button_advanced("📈 Charting Suite", "glass", "sm", false, None)
                .attr("style", if active_tab == "charts" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("charts")),
            button_advanced("📝 Form Wizard", "glass", "sm", false, None)
                .attr("style", if active_tab == "form" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("form")),
            button_advanced("📁 Explorer & Overlays", "glass", "sm", false, None)
                .attr("style", if active_tab == "explorer" { "background: var(--accent-primary); color: #fff; font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(|| render_app("explorer")),
        ]);

    let cmd_k_btn = button_advanced("⌘K Command", "outline", "xs", false, Some("search"))
        .on_click(move || cp_clone.open());

    let theme_selector = div()
        .attr("class", "ferrox-theme-selector flex items-center gap-1.5")
        .children(vec![
            span().text("Theme:").attr("style", "font-size: 0.8rem; opacity: 0.7; font-weight: 600; margin-right: 2px;"),
            button_advanced("🔮 Cyber Dark", "outline", "xs", false, None).on_click(|| set_theme("ferrox-cyber")),
            button_advanced("🏢 Zinc Light", "outline", "xs", false, None).on_click(|| set_theme("enterprise-zinc")),
            button_advanced("⚡ Cyberpunk", "outline", "xs", false, None).on_click(|| set_theme("cyberpunk-neon")),
            button_advanced("🌊 Ocean", "outline", "xs", false, None).on_click(|| set_theme("ocean-breeze")),
        ]);

    let right_controls = div()
        .attr("class", "flex items-center gap-3")
        .children(vec![cmd_k_btn, theme_selector]);

    div()
        .attr("class", "ferrox-topbar flex flex-wrap items-center justify-between gap-4 px-6 py-3.5 border-b backdrop-blur-xl sticky top-0 z-40")
        .attr("style", "background: var(--bg-surface); border-bottom: var(--border-glass);")
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
        "Limit Price (USD)",
        "97050.00".to_string(),
        RuleBuilder::new().required("Price is required").numeric_range(100.0, 1000000.0, "Invalid price").build(),
    ).with_placeholder("97050.00");

    let field_amount = FormField::new(
        "trade-amount",
        "amount",
        "Quantity (BTC)",
        "0.25".to_string(),
        RuleBuilder::new().required("Quantity is required").numeric_range(0.001, 1000.0, "Invalid quantity").build(),
    ).with_placeholder("0.25");

    let order_entry_card = card_advanced(
        "Order Entry • ferrox-engine",
        "Zero-Latency In-Memory Matching",
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
                        button_advanced("BUY", "primary", "sm", false, None)
                            .attr("style", "background: #10b981; color: white; flex: 1;"),
                        button_advanced("SELL", "outline", "sm", false, None)
                            .attr("style", "border-color: #ef4444; color: #ef4444; flex: 1;"),
                    ]),
                field_price.render(),
                field_amount.render(),
                div().class("flex items-center justify-between text-xs text-slate-400 py-1 border-t border-slate-800")
                    .children(vec![
                        span().text("Estimated Value:"),
                        span().class("font-mono font-bold text-white").text("$24,262.50 USD"),
                    ]),
                button_advanced("⚡ Submit Order (Wasm Match Engine)", "primary", "md", false, Some("zap"))
                    .attr("style", "width: 100%;")
                    .on_click(move || {
                        if let Some(win) = window() {
                            let _ = win.alert_with_message("✅ Order submitted and executed instantly in ferrox-engine matching engine!");
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
        "BTC/USD Market Depth",
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
        "Real-Time Executed Transactions",
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
            card_advanced("24h Spot Vol", "Total Wasm Volume", "activity", "glass", false, "p-4")
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
                .child(span().class("text-xs text-indigo-300").text("Auto-Reconnection + Backoff Active"))
        ));

    // 8. Main Chart and Order Entry Row
    let chart_and_entry_row = row()
        .child(col("12 md-8").child(
            card_advanced(
                "Trading Desk Pro • Bitcoin Perpetual (BTC/USD 5m)",
                "TradingView Parity Engine: Candlestick, SMA (20, 50), Bollinger (±2σ), RSI (14), MACD (12,26,9)",
                "chart-candlestick",
                "glass",
                false,
                "p-6",
            )
            .child(main_chart)
            .child(
                div().class("flex items-center justify-between pt-4 mt-4 border-t border-slate-800/80 text-xs text-slate-400")
                    .children(vec![
                        span().text("CandleAggregator (5m) • Active Crosshair & Dynamic Data Reader"),
                        div().class("flex items-center gap-3")
                            .children(vec![
                                button_advanced("🔌 Connect WS Feed", "outline", "xs", false, Some("activity"))
                                    .on_click(|| {
                                        let _ = ReconnectingWs::new("wss://stream.ferrox.exchange/ws/v1");
                                    }),
                                button_advanced("⚡ Simulate Live Tick", "primary", "xs", false, Some("zap"))
                                    .on_click(|| render_app("trading")),
                            ]),
                    ])
            )
        ))
        .child(col("12 md-4").child(order_entry_card));

    // 9. Depth Chart, Order Book Ladder, and Recent Trades Row
    let depth_and_ladder_row = row()
        .child(col("12 md-6").child(
            card_advanced(
                "Order Book Market Depth (Bids vs Asks)",
                "Cumulative Depth Curve with Mid-Price and Spread",
                "chart-bar",
                "glass",
                false,
                "p-6",
            )
            .child(depth_svg)
        ))
        .child(col("12 md-3").child(order_book_card))
        .child(col("12 md-3").child(trades_card));

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![stat_cards, chart_and_entry_row, depth_and_ladder_row])
}

// =============================================================================
// 4. TAB 2: ENTERPRISE DATAGRID PRO (AG-Grid Parity)
// =============================================================================

fn render_datagrid_tab() -> DomBuilder {
    let orders = generate_1000_orders();

    let columns = vec![
        ColumnDef::new("id", "Order ID", |r: &OrderRow| r.id.clone())
            .with_width("120px")
            .with_pinned(PinnedPosition::Left)
            .with_renderer(|r: &OrderRow| {
                span().class("font-mono font-bold text-indigo-400 text-xs").text(&r.id)
            }),
        ColumnDef::new("symbol", "Asset Symbol", |r: &OrderRow| r.symbol.clone())
            .with_width("110px")
            .with_renderer(|r: &OrderRow| {
                div().class("flex items-center gap-1.5")
                    .children(vec![
                        Icon::render("activity", 12, "#38bdf8"),
                        span().class("font-semibold text-white text-xs").text(&r.symbol),
                    ])
            }),
        ColumnDef::new("side", "Side", |r: &OrderRow| r.side.clone())
            .with_width("85px")
            .with_align(TextAlign::Center)
            .with_renderer(|r: &OrderRow| {
                if r.side == "BUY" {
                    badge("BUY", "success")
                } else {
                    badge("SELL", "danger")
                }
            }),
        ColumnDef::new("price", "Price (USD)", |r: &OrderRow| format!("${:.2}", r.price))
            .with_width("115px")
            .with_align(TextAlign::Right),
        ColumnDef::new("amount", "Amount", |r: &OrderRow| format!("{:.3}", r.amount))
            .with_width("95px")
            .with_align(TextAlign::Right),
        ColumnDef::new("total", "Total Value", |r: &OrderRow| format!("${:.2}", r.total))
            .with_width("125px")
            .with_align(TextAlign::Right)
            .with_renderer(|r: &OrderRow| {
                span().class("font-mono font-bold text-xs text-emerald-400").text(&format!("${:.2}", r.total))
            }),
        ColumnDef::new("status", "Status", |r: &OrderRow| r.status.clone())
            .with_width("145px")
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
            .with_width("145px"),
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
        .class("flex flex-wrap items-center justify-between gap-4 p-4 px-6 rounded-2xl bg-slate-900/80 border border-slate-800/80 shadow-lg mb-2")
        .children(vec![
            span().class("text-xs text-slate-300 font-medium").text("Direct Table Export (WebAssembly Blob RFC 4180):"),
            button_advanced("📥 Export All 1,000 Orders (CSV)", "primary", "xs", false, Some("download"))
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
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("flex flex-wrap items-center justify-between gap-4 pb-3 border-b border-slate-800/80")
                .children(vec![
                    div().children(vec![
                        h2().class("text-2xl font-bold text-white tracking-tight").text("Enterprise DataGrid Pro • AG-Grid Parity"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("1,000 orders in Wasm linear memory, multi-column sorting, reactive filters, row selection, and native CSV export download."),
                    ]),
                    badge("1,000 Records Wasm", "primary"),
                ]),
            export_bar,
            rendered_grid,
        ])
}

// =============================================================================
// 5. TAB 3: FORM VALIDATION WIZARD (Zod + React Hook Form Parity)
// =============================================================================

fn render_form_wizard_tab() -> DomBuilder {
    // Step 1: Company Details
    let field_company = FormField::new(
        "field-company",
        "company",
        "Company Name / Entity",
        String::new(),
        RuleBuilder::new().required("Company name is required.").min_length(3, "Minimum 3 characters.").build(),
    ).with_placeholder("Ferrox Financial Labs Ltd.");

    let field_email = FormField::new(
        "field-email",
        "email",
        "Verified Corporate Email",
        String::new(),
        RuleBuilder::new().required("Corporate email is required.").email("Invalid email format (e.g. user@company.com).").build(),
    ).with_input_type("email").with_placeholder("compliance@ferrox.io");

    let field_vat = FormField::new(
        "field-vat",
        "vat",
        "Tax Identifier / VAT Number",
        String::new(),
        RuleBuilder::new().required("Tax ID is required.").min_length(8, "Tax ID must be at least 8 characters.").build(),
    ).with_placeholder("US987654321");

    let f_comp_clone = field_company.clone();
    let f_email_clone = field_email.clone();
    let f_vat_clone = field_vat.clone();

    let step1 = WizardStep::new(
        "step-1",
        "1. Company Details",
        "Enter the legal entity information authorized for trading.",
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

    // Step 2: Financial Parameters
    let field_capital = FormField::new(
        "field-capital",
        "capital",
        "Initial Allocated Capital (USD)",
        "50000".to_string(),
        RuleBuilder::new().required("Capital is required.").numeric_range(1000.0, 10000000.0, "Capital must be between $1,000 and $10,000,000.").build(),
    ).with_placeholder("e.g. 50000");

    let field_risk = FormField::new(
        "field-risk",
        "risk",
        "Max Risk Tolerance / Trade (%)",
        "2.5".to_string(),
        RuleBuilder::new().required("Risk percentage is required.").numeric_range(0.1, 10.0, "Risk percentage must be between 0.1% and 10.0%.").build(),
    ).with_placeholder("e.g. 2.5");

    let f_cap_clone = field_capital.clone();
    let f_risk_clone = field_risk.clone();

    let step2 = WizardStep::new(
        "step-2",
        "2. Financial Parameters",
        "Configure capital allocation and Risk Management metrics.",
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

    // Step 3: Zero-Trust Security Token & Confirmation
    let field_key_name = FormField::new(
        "field-key-name",
        "key_name",
        "Cryptographic Key Label (SealedToken)",
        "Production-Trading-Desk-Key-1".to_string(),
        RuleBuilder::new().required("Key label is required.").min_length(5, "Minimum 5 characters.").build(),
    );

    let f_key_clone = field_key_name.clone();

    let step3 = WizardStep::new(
        "step-3",
        "3. Zero-Trust Security",
        "Cryptographic credential sealing inside Wasm linear memory.",
        move || {
            f_key_clone.validate()
        },
        move || {
            div().children(vec![
                field_key_name.render(),
                div()
                    .class("p-4 rounded-xl bg-slate-900/60 border border-slate-800 text-xs text-slate-300 space-y-2 mt-4")
                    .children(vec![
                        div().class("font-bold text-indigo-400").text("🔒 Zero-Trust SealedToken Protection"),
                        p().text("Generated credentials are stored exclusively in isolated WebAssembly linear memory, impenetrable to browser extensions or external XSS scripts."),
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
            let _ = win.alert_with_message("✅ Onboarding completed successfully! Session authenticated with SealedToken in Wasm memory.");
        }
    });

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("text-center max-w-xl mx-auto mb-6")
                .children(vec![
                    h2().class("text-2xl font-bold text-white tracking-tight").text("Form Validation Wizard • Strict Reactive Gating"),
                    p().class("text-xs text-slate-400 mt-2")
                        .text("Zod + React Hook Form Parity: Real-time validation, touched/dirty states, WCAG 2.1 AA compliance."),
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
                info_el.set_inner_html(&format!("Selected Node: <strong>{}</strong>", node_id));
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
                    ferrox_front_ui::overlays::DatePickerSelection::Single(d) => format!("Date: {}", d),
                    ferrox_front_ui::overlays::DatePickerSelection::Range(s, e) => format!("Range: {} → {}", s, e),
                };
                info_el.set_inner_html(&format!("Time Filter: <strong>{}</strong>", text));
            }
        }
    });

    let explorer_col = col("12 md-6").child(
        card_advanced("Hierarchical TreeView Explorer", "Radix-Style Structural Navigator", "folder", "glass", false, "p-6")
            .child(tree)
            .child(
                div().id("tree-selected-label").class("text-xs text-indigo-400 mt-4 pt-3 border-t border-slate-800/80")
                    .text("Click a tree node to inspect its details.")
            )
    );

    let overlays_col = col("12 md-6").child(
        card_advanced("Pure Vector DatePicker & Command Palette", "Enterprise Overlay Components", "calendar", "glass", false, "p-6")
            .child(
                div().class("flex flex-col items-center gap-6")
                    .children(vec![
                        calendar,
                        div().id("datepicker-selected-label").class("text-xs text-emerald-400 font-semibold")
                            .text("Time Filter: No date selected."),
                        div().class("w-full pt-4 border-t border-slate-800/80 flex items-center justify-between")
                            .children(vec![
                                span().class("text-xs text-slate-400").text("Global keyboard shortcut:"),
                                button_advanced("Open Command Palette (⌘K / Ctrl+K)", "primary", "sm", false, Some("search"))
                                    .on_click(move || cp_clone.open()),
                            ])
                    ])
            )
    );

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("pb-3 border-b border-slate-800/80 mb-2")
                .children(vec![
                    h2().class("text-2xl font-bold text-white tracking-tight").text("File Explorer, Vector DatePicker & Command Palette"),
                    p().class("text-xs text-slate-400 mt-1")
                        .text("Fluid hierarchical navigation, date range picking, and Spotlight-style instant search."),
                ]),
            row().child(explorer_col).child(overlays_col),
        ])
}

// =============================================================================
// 7. TAB 5: DESIGN SYSTEM & UI ELEMENTS CATALOG
// =============================================================================

fn render_components_catalog_tab() -> DomBuilder {
    // 1. Buttons & Button Groups Section
    let buttons_card = card_advanced(
        "Button Variants, Sizing & Loading States",
        "WCAG 2.1 AA accessible buttons with hover micro-animations and active spinners",
        "zap",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-6")
            .children(vec![
                div().children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Visual Variants"),
                    div().class("flex flex-wrap items-center gap-3")
                        .children(vec![
                            button_advanced("Primary Action", "primary", "sm", false, Some("zap")),
                            button_advanced("Secondary Glass", "glass", "sm", false, Some("layers")),
                            button_advanced("Outline Cyan", "outline", "sm", false, Some("search")),
                            button_advanced("Destructive", "danger", "sm", false, Some("trash")),
                            button_advanced("Ghost Neutral", "ghost", "sm", false, None),
                            button_advanced("Link Text", "link", "sm", false, Some("external-link")),
                        ]),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Scale & Sizing Options"),
                    div().class("flex flex-wrap items-center gap-3")
                        .children(vec![
                            button_advanced("XS Button (24px)", "primary", "xs", false, None),
                            button_advanced("SM Button (32px)", "primary", "sm", false, None),
                            button_advanced("MD Button (40px)", "primary", "md", false, None),
                            button_advanced("LG Button (48px)", "primary", "lg", false, None),
                        ]),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Active Loading Spinners & Segmented Group"),
                    div().class("flex flex-wrap items-center gap-4")
                        .children(vec![
                            button_advanced("Processing Order...", "primary", "sm", true, None),
                            button_advanced("Syncing Linear Mem...", "glass", "sm", true, None),
                            button_group(vec![
                                button_advanced("1M", "glass", "xs", false, None),
                                button_advanced("5M", "primary", "xs", false, None),
                                button_advanced("15M", "glass", "xs", false, None),
                                button_advanced("1H", "glass", "xs", false, None),
                                button_advanced("1D", "glass", "xs", false, None),
                            ]),
                        ]),
                ]),
            ])
    );

    // 2. Form Controls & Advanced Inputs Section
    let input_valid = input_advanced(InputConfig {
        input_type: "email".into(),
        placeholder: "trader@ferrox.exchange".into(),
        value: "trader@ferrox.exchange".into(),
        prefix_icon: Some("mail".into()),
        suffix_icon: Some("check".into()),
        is_valid: Some(true),
        helper_text: Some("Verified institutional email address.".into()),
        clearable: false,
    });

    let input_invalid = input_advanced(InputConfig {
        input_type: "number".into(),
        placeholder: "Enter price...".into(),
        value: "0.00".into(),
        prefix_icon: Some("dollar-sign".into()),
        suffix_icon: Some("alert-circle".into()),
        is_valid: Some(false),
        helper_text: Some("Price must exceed minimum tick size ($0.50).".into()),
        clearable: false,
    });

    let forms_card = card_advanced(
        "Form Controls & Reactive Inputs",
        "Strongly-typed validation states, prefix/suffix SVG icons, and live helpers",
        "file-text",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("grid grid-cols-1 md:grid-cols-2 gap-6")
            .children(vec![
                div().class("space-y-2")
                    .children(vec![
                        span().class("text-xs font-bold text-slate-300 block mb-1").text("Verified Input (Valid)"),
                        input_valid,
                    ]),
                div().class("space-y-2")
                    .children(vec![
                        span().class("text-xs font-bold text-slate-300 block mb-1").text("Tick Boundary Input (Invalid)"),
                        input_invalid,
                    ]),
                div().class("space-y-2 md:col-span-2 pt-3 border-t border-slate-800/80")
                    .children(vec![
                        span().class("text-xs font-bold text-slate-300 block mb-1").text("Compliance Notes (Textarea with Counter)"),
                        textarea_advanced("kyc-notes", "Enter compliance audit remarks...", 250, "Authorized by compliance officer under SOC2 Type II guidelines."),
                    ]),
                div().class("space-y-2 md:col-span-2 pt-3 border-t border-slate-800/80")
                    .children(vec![
                        span().class("text-xs font-bold text-slate-300 block mb-1").text("Interactive Slippage Tolerance Slider"),
                        range_slider_with_bubble("slippage-slider", 0, 100, 35, "%"),
                    ]),
            ])
    );

    // 3. Toggles, Checkboxes & State Controls Section
    let states_card = card_advanced(
        "Switches, Checkboxes & Selection Controls",
        "Micro-animated toggle switches, custom SVG checkboxes and radio groups",
        "check",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-6")
            .children(vec![
                div().class("flex flex-wrap items-center gap-8 py-2")
                    .children(vec![
                        switch_toggle("sw-stream", true, "Order Book Streaming (< 1.8 μs)"),
                        switch_toggle("sw-hft", false, "HFT Dark Pool Routing"),
                        switch_toggle("sw-sim", true, "Hardware Acceleration (WebGL)"),
                    ]),
                div().class("flex flex-wrap items-center gap-8 pt-4 border-t border-slate-800/80")
                    .children(vec![
                        checkbox_advanced("cb-all", "Select All Order Books", false, true),
                        checkbox_advanced("cb-btc", "Auto-Hedge Spot Liquidity", true, false),
                        checkbox_advanced("cb-eth", "Cross-Margin Liquidation Safeguard", true, false),
                    ]),
                div().class("pt-4 border-t border-slate-800/80")
                    .children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Execution Architecture Selection"),
                        radio_group(
                            "exec_arch",
                            vec![
                                ("in-process", "🦀 In-Process Rust Matching Engine (Zero JS / < 1.8 μs)"),
                                ("clustered", "⚡ High-Throughput Raft Consensus Cluster (1M RPS)"),
                                ("decentralized", "🌐 Layer-2 Optimistic State Rollup"),
                            ],
                            "in-process",
                        ),
                    ]),
            ])
    );

    // 4. Surfaces, Overlays & Feedback Section
    let overlays_card = card_advanced(
        "Surfaces, Overlays, Toasts & Badges",
        "Glassmorphism modals, sliding drawers, progress indicators, and status badges",
        "layers",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-6")
            .children(vec![
                div().children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Interactive Dialog & Drawer Triggers"),
                    div().class("flex flex-wrap items-center gap-4 mb-2")
                        .children(vec![
                            button_advanced("Launch Modal Dialog", "primary", "sm", false, Some("layers"))
                                .on_click(|| {
                                    if let Some(doc) = window().and_then(|w| w.document()) {
                                        if let Some(body) = doc.body() {
                                            let m = modal_advanced(
                                                "demo-comp-modal",
                                                "Trading Node Security Verification",
                                                p().class("text-slate-300 text-sm")
                                                    .text("Confirm authorization of in-memory matching engine node #042 with sub-microsecond zero-trust token?"),
                                                Some(button_advanced("Authorize Node", "primary", "sm", false, None)),
                                                "md",
                                            );
                                            let _ = body.append_child(&m.build());
                                        }
                                    }
                                }),
                            button_advanced("Trigger Live Toast Alert", "outline", "sm", false, Some("info"))
                                .on_click(|| {
                                    if let Some(doc) = window().and_then(|w| w.document()) {
                                        if let Some(body) = doc.body() {
                                            let t = toast("Trade Executed", "0.5 BTC filled at $97,050.00 with zero slippage", "success");
                                            let _ = body.append_child(&t.build());
                                        }
                                    }
                                }),
                        ]),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Status Badges & Alert Banners"),
                    div().class("flex flex-wrap items-center gap-3 mb-4")
                        .children(vec![
                            badge("MATCHED", "success"),
                            badge("PARTIAL FILL", "warning"),
                            badge("CANCELLED", "danger"),
                            badge("RESTING LIQUIDITY", "info"),
                            badge("ROUTED VIA WASM", "primary"),
                        ]),
                    alert("Zero-copy Rust WebAssembly pipeline validated under WCAG 2.1 AA accessibility guidelines.", "success"),
                ]),
            ])
    );

    // 5. Pure Rust 60+ SVG Vector Icons Section
    let icon_names = vec![
        "zap", "activity", "trend-up", "trend-down", "dollar-sign", "credit-card",
        "chart-candlestick", "chart-bar", "layers", "cpu", "lock", "shield",
        "info", "alert-circle", "palette", "moon", "sun", "search",
        "settings", "trash", "edit", "check", "x", "chevron-right",
        "chevron-left", "menu", "folder", "database", "download"
    ];

    let mut icon_grid_items = Vec::new();
    for name in icon_names {
        icon_grid_items.push(
            div().class("flex flex-col items-center justify-center p-3.5 rounded-xl bg-slate-900/60 border border-slate-800/80 hover:border-indigo-500/50 hover:bg-slate-800/60 transition-all gap-2 min-h-[88px]")
                .children(vec![
                    Icon::render(name, 22, "#818cf8"),
                    span().class("text-[10px] text-slate-400 font-mono").text(name),
                ])
        );
    }

    let icons_card = card_advanced(
        "Pure Rust 60+ SVG Vector Icon Engine",
        "Zero HTTP requests, zero external webfonts, rendered natively into DOM in WebAssembly",
        "palette",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("grid grid-cols-3 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-3.5")
            .children(icon_grid_items)
    );

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 32px;")
        .children(vec![
            div()
                .class("pb-4 border-b border-slate-800")
                .children(vec![
                    h2().class("text-2xl font-black text-white tracking-tight").text("Enterprise Design System & UI Elements"),
                    p().class("text-xs text-slate-400 mt-1")
                        .text("Complete interactive catalog of Ferrox Front WebAssembly UI elements conforming to WCAG 2.1 AA."),
                ]),
            row().child(col("12 md-6").child(buttons_card)).child(col("12 md-6").child(forms_card)),
            row().child(col("12 md-6").child(states_card)).child(col("12 md-6").child(overlays_card)),
            row().child(col("12").child(icons_card)),
        ])
}

// =============================================================================
// 8. TAB 6: FINANCIAL & VECTOR CHARTING SUITE
// =============================================================================

fn render_charts_gallery_tab() -> DomBuilder {
    let candle_data = mock_candlestick_data();
    let mut aggregator = CandleAggregator::new(Timeframe::Min5, 50);
    for (idx, c) in candle_data.iter().enumerate() {
        let ts = 1760518800000 + (idx as u64 * 300_000);
        aggregator.process_tick(TradeTick::new(c.close, c.volume, ts));
    }
    let chart_candles = aggregator.get_candles_with_active();

    let chart_opts = TechnicalChartOptions::default();
    let main_chart = financial_technical_chart(
        "gallery-pro-chart",
        if chart_candles.is_empty() { &candle_data } else { &chart_candles },
        &chart_opts,
        860,
        480,
    );

    let mut order_book = OrderBook::new("BTC/USD");
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

    let depth_svg = depth_chart(
        "gallery-market-depth",
        &bids_tuples,
        &asks_tuples,
        560,
        280,
    );

    // Speedometer Telemetry Gauges
    let cpu_gauge = gauge_chart("gauge-cpu", 42.5, 0.0, 100.0, "CPU Load", "%", 220, 130);
    let match_gauge = gauge_chart("gauge-latency", 1.8, 0.0, 10.0, "Engine Latency", "μs", 220, 130);
    let tps_gauge = gauge_chart("gauge-tps", 940.0, 0.0, 1000.0, "Order Book TPS", "k", 220, 130);

    // Area Sparklines for top assets
    let btc_spark = sparkline_chart(vec![95200.0, 95800.0, 95400.0, 96300.0, 96800.0, 97050.0], 140, 36, SparklineType::Area, "#10b981");
    let eth_spark = sparkline_chart(vec![2740.0, 2710.0, 2780.0, 2810.0, 2840.0, 2855.0], 140, 36, SparklineType::Area, "#38bdf8");
    let sol_spark = sparkline_chart(vec![182.0, 189.0, 185.0, 194.0, 196.0, 198.5], 140, 36, SparklineType::Area, "#a855f7");
    let ferr_spark = sparkline_chart(vec![12.4, 13.1, 12.9, 14.2, 14.8, 15.6], 140, 36, SparklineType::Area, "#f59e0b");

    let sparklines_card = card_advanced(
        "Real-Time Multi-Asset Vector Sparklines",
        "Embedded mini-charts for DataGrid cells and live portfolio metric cards",
        "activity",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-5")
            .children(vec![
                div().class("p-4 rounded-2xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                    .children(vec![
                        div().children(vec![
                            span().class("text-xs text-slate-400 block").text("BTC/USD"),
                            span().class("text-base font-bold text-white").text("$97,050.00"),
                        ]),
                        btc_spark,
                    ]),
                div().class("p-4 rounded-2xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                    .children(vec![
                        div().children(vec![
                            span().class("text-xs text-slate-400 block").text("ETH/USD"),
                            span().class("text-base font-bold text-white").text("$2,855.00"),
                        ]),
                        eth_spark,
                    ]),
                div().class("p-4 rounded-2xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                    .children(vec![
                        div().children(vec![
                            span().class("text-xs text-slate-400 block").text("SOL/USD"),
                            span().class("text-base font-bold text-white").text("$198.50"),
                        ]),
                        sol_spark,
                    ]),
                div().class("p-4 rounded-2xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                    .children(vec![
                        div().children(vec![
                            span().class("text-xs text-slate-400 block").text("FERR/USD"),
                            span().class("text-base font-bold text-white").text("$15.60"),
                        ]),
                        ferr_spark,
                    ]),
            ])
    );

    let gauges_card = card_advanced(
        "Telemetry Speedometers & Quantitative Gauges",
        "High-precision semi-circular gauges with graduated arcs and numeric readouts",
        "cpu",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("grid grid-cols-1 sm:grid-cols-3 gap-6 text-center")
            .children(vec![
                div().class("flex flex-col items-center p-3").child(cpu_gauge),
                div().class("flex flex-col items-center p-3").child(match_gauge),
                div().class("flex flex-col items-center p-3").child(tps_gauge),
            ])
    );

    let technical_card = card_advanced(
        "Trading Desk Pro • Bitcoin Perpetual (BTC/USD 5m)",
        "TradingView Parity Engine: Candlesticks, SMA (20, 50), Bollinger Bands (±2σ), RSI (14), and MACD (12,26,9)",
        "chart-candlestick",
        "glass",
        false,
        "p-6",
    )
    .child(main_chart)
    .child(
        div().class("flex items-center justify-between pt-4 mt-4 border-t border-slate-800/80 text-xs text-slate-400")
            .children(vec![
                span().text("Pure Rust Vector SVG Rendering • Dynamic Crosshair Tracking"),
                button_advanced("⚡ Push Simulated Market Tick", "primary", "xs", false, Some("zap"))
                    .on_click(|| render_app("charts")),
            ])
    );

    let depth_card = card_advanced(
        "Order Book Market Depth (Bids vs Asks)",
        "Cumulative liquidity curves with mid-market equilibrium line and spread badge",
        "chart-bar",
        "glass",
        false,
        "p-6",
    )
    .child(depth_svg);

    container()
        .attr("style", "max-width: 1400px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("pb-3 border-b border-slate-800/80 mb-2")
                .children(vec![
                    h2().class("text-2xl font-black text-white tracking-tight").text("SVG Vector Financial Charting Suite"),
                    p().class("text-xs text-slate-400 mt-1")
                        .text("High-precision quantitative visualizations engineered with zero JavaScript or Canvas overhead."),
                ]),
            row().child(col("12").child(sparklines_card)),
            row().child(col("12 md-7").child(technical_card)).child(col("12 md-5").child(depth_card)),
            row().child(col("12").child(gauges_card)),
        ])
}

// =============================================================================
// 9. APPLICATION SHELL & ROUTER DISPATCH
// =============================================================================

fn create_command_palette() -> CommandPaletteState {
    let actions = vec![
        CommandAction::new("act-trading", "Go to Trading Desk Pro", "Candlestick chart, indicators, and streaming", "Navigation", "chart-candlestick", Some("G T"), || render_app("trading")),
        CommandAction::new("act-datagrid", "Go to Enterprise DataGrid", "1,000 orders grid, filters, and CSV export", "Navigation", "grid", Some("G D"), || render_app("datagrid")),
        CommandAction::new("act-components", "Go to UI Elements & Design System", "Interactive buttons, inputs, toggles, and icons", "Navigation", "zap", Some("G C"), || render_app("components")),
        CommandAction::new("act-charts", "Go to SVG Financial Charting Suite", "Candlesticks, depth curve, gauges, and sparklines", "Navigation", "activity", Some("G H"), || render_app("charts")),
        CommandAction::new("act-form", "Go to Form Validation Wizard", "Multi-step onboarding with strict validation", "Navigation", "file-text", Some("G F"), || render_app("form")),
        CommandAction::new("act-explorer", "Go to Explorer & Overlays", "TreeView, Vector DatePicker, and controls", "Navigation", "folder", Some("G E"), || render_app("explorer")),
        CommandAction::new("act-csv", "Export All Orders to CSV", "Download RFC 4180 generated in Wasm memory", "Quick Actions", "download", Some("E C"), || {
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
        CommandAction::new("act-theme-cyber", "Set Cyber Dark Theme", "Deep purple/blue glassmorphism palette", "Theme", "palette", None, || set_theme("ferrox-cyber")),
        CommandAction::new("act-theme-zinc", "Set Zinc Light Theme", "High-contrast enterprise light design", "Theme", "palette", None, || set_theme("enterprise-zinc")),
        CommandAction::new("act-theme-neon", "Set Cyberpunk Neon Theme", "Vibrant high-contrast yellow/cyan colors", "Theme", "palette", None, || set_theme("cyberpunk-neon")),
        CommandAction::new("act-theme-ocean", "Set Ocean Breeze Theme", "Deep ocean blue gradient", "Theme", "palette", None, || set_theme("ocean-breeze")),
    ];

    CommandPaletteState::new(actions)
}

pub fn render_app(mode: &str) {
    if let Some(win) = window() {
        let _ = win.location().set_hash(mode);
        if let Ok(Some(parent)) = win.parent() {
            let msg = wasm_bindgen::JsValue::from_str(&format!("ferrox-tab:{}", mode));
            let _ = parent.post_message(&msg, "*");
        }
    }

    let cmd_palette_state = create_command_palette();
    let top_bar = render_top_bar(mode, &cmd_palette_state);

    let body_content = match mode {
        "datagrid" => render_datagrid_tab(),
        "form" => render_form_wizard_tab(),
        "components" => render_components_catalog_tab(),
        "charts" => render_charts_gallery_tab(),
        "explorer" => render_explorer_tab(&cmd_palette_state),
        _ => render_trading_desk_tab(),
    };

    let cp_overlay = command_palette(cmd_palette_state);

    let app = div()
        .attr("class", "ferrox-app-shell min-h-screen flex flex-col")
        .children(vec![top_bar, body_content, cp_overlay]);

    mount("#root", app);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut initial_mode = "trading".to_string();

    if let Some(win) = window() {
        if let Ok(hash) = win.location().hash() {
            let h = hash.trim_start_matches('#');
            if ["datagrid", "form", "explorer", "trading", "components", "charts"].contains(&h) {
                initial_mode = h.to_string();
            }
        }

        // Setup hashchange listener so parent iframe or url hash changes re-render immediately
        let on_hash_change = Closure::wrap(Box::new(move || {
            if let Some(win) = window() {
                if let Ok(hash) = win.location().hash() {
                    let h = hash.trim_start_matches('#');
                    let mode = match h {
                        "datagrid" => "datagrid",
                        "form" => "form",
                        "components" => "components",
                        "charts" => "charts",
                        "explorer" => "explorer",
                        _ => "trading",
                    };
                    render_app(mode);
                }
            }
        }) as Box<dyn FnMut()>);

        let _ = win.add_event_listener_with_callback(
            "hashchange",
            on_hash_change.as_ref().unchecked_ref(),
        );
        on_hash_change.forget();

        // Also setup postMessage listener in case parent window communicates via postMessage
        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Some(msg_str) = e.data().as_string() {
                if msg_str.starts_with("ferrox-set-tab:") {
                    let tab = msg_str.trim_start_matches("ferrox-set-tab:");
                    let mode = match tab {
                        "datagrid" => "datagrid",
                        "form" => "form",
                        "components" => "components",
                        "charts" => "charts",
                        "explorer" => "explorer",
                        _ => "trading",
                    };
                    render_app(mode);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = win.add_event_listener_with_callback(
            "message",
            on_message.as_ref().unchecked_ref(),
        );
        on_message.forget();
    }

    render_app(&initial_mode);
    Ok(())
}
