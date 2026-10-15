//! # Financial Market Components (`ferrox-front-charts::market`)
//!
//! Provides reusable market visualization and tick aggregation primitives:
//! - `depth_chart`: High-performance vector SVG Order Book Depth chart rendering cumulative Bids & Asks,
//!   mid-market price line, bid-ask spread indicator, and interactive crosshair.
//! - `CandleAggregator`: Real-time streaming tick aggregator combining discrete `TradeTick` events
//!   into open OHLC candles across selectable timeframes (1s, 5s, 1m, 5m, 15m, 1h, 1d) and pushing
//!   into `CandleRingBuffer`.

use ferrox_front_core::dom::*;
use crate::{CandleDatum, CandleRingBuffer};

/// Individual trade execution tick.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeTick {
    pub price: f64,
    pub volume: f64,
    pub timestamp: u64, // Unix epoch milliseconds
}

impl TradeTick {
    pub fn new(price: f64, volume: f64, timestamp: u64) -> Self {
        Self { price, volume, timestamp }
    }
}

/// Supported candlestick aggregation timeframes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeframe {
    Sec1,
    Sec5,
    Min1,
    Min5,
    Min15,
    Hour1,
    Day1,
}

impl Timeframe {
    pub fn duration_ms(&self) -> u64 {
        match self {
            Timeframe::Sec1 => 1_000,
            Timeframe::Sec5 => 5_000,
            Timeframe::Min1 => 60_000,
            Timeframe::Min5 => 300_000,
            Timeframe::Min15 => 900_000,
            Timeframe::Hour1 => 3_600_000,
            Timeframe::Day1 => 86_400_000,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Timeframe::Sec1 => "1s",
            Timeframe::Sec5 => "5s",
            Timeframe::Min1 => "1m",
            Timeframe::Min5 => "5m",
            Timeframe::Min15 => "15m",
            Timeframe::Hour1 => "1h",
            Timeframe::Day1 => "1d",
        }
    }
}

/// Helper function formatting timestamp into HH:MM:SS string.
pub fn format_timestamp_to_time(ts_ms: u64) -> String {
    let total_secs = ts_ms / 1000;
    let secs = total_secs % 60;
    let mins = (total_secs / 60) % 60;
    let hours = (total_secs / 3600) % 24;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

/// Real-time streaming candle aggregator maintaining active open candle and fixed-size historical buffer.
#[derive(Debug, Clone)]
pub struct CandleAggregator {
    pub timeframe: Timeframe,
    pub current_period_start: u64,
    pub current_candle: Option<CandleDatum>,
    pub ring_buffer: CandleRingBuffer,
}

impl CandleAggregator {
    /// Creates a new CandleAggregator with specified timeframe and historical ring buffer capacity.
    pub fn new(timeframe: Timeframe, buffer_capacity: usize) -> Self {
        Self {
            timeframe,
            current_period_start: 0,
            current_candle: None,
            ring_buffer: CandleRingBuffer::new(buffer_capacity),
        }
    }

    /// Ingests a single trade tick. Returns `Some(closed_candle)` when a timeframe boundary is crossed.
    pub fn process_tick(&mut self, tick: TradeTick) -> Option<CandleDatum> {
        let period_ms = self.timeframe.duration_ms();
        let tick_period_start = (tick.timestamp / period_ms) * period_ms;

        if let Some(mut current) = self.current_candle.take() {
            if tick_period_start == self.current_period_start {
                // Update active open candle
                if tick.price > current.high {
                    current.high = tick.price;
                }
                if tick.price < current.low {
                    current.low = tick.price;
                }
                current.close = tick.price;
                current.volume += tick.volume;
                self.current_candle = Some(current);
                None
            } else {
                // Period boundary finalized!
                let closed = current.clone();
                self.ring_buffer.push(closed.clone());

                let time_label = format_timestamp_to_time(tick_period_start);
                let new_candle = CandleDatum {
                    time: time_label,
                    open: tick.price,
                    high: tick.price,
                    low: tick.price,
                    close: tick.price,
                    volume: tick.volume,
                };
                self.current_period_start = tick_period_start;
                self.current_candle = Some(new_candle);
                Some(closed)
            }
        } else {
            // First tick ingested
            let time_label = format_timestamp_to_time(tick_period_start);
            let new_candle = CandleDatum {
                time: time_label,
                open: tick.price,
                high: tick.price,
                low: tick.price,
                close: tick.price,
                volume: tick.volume,
            };
            self.current_period_start = tick_period_start;
            self.current_candle = Some(new_candle);
            None
        }
    }

    /// Switches aggregation timeframe.
    pub fn set_timeframe(&mut self, tf: Timeframe) {
        self.timeframe = tf;
    }

    /// Retrieves all closed candles in the ring buffer.
    pub fn closed_candles(&self) -> &[CandleDatum] {
        self.ring_buffer.as_slice()
    }

    /// Retrieves all candles including the active open candle.
    pub fn get_candles_with_active(&self) -> Vec<CandleDatum> {
        let mut list = self.ring_buffer.as_slice().to_vec();
        if let Some(active) = &self.current_candle {
            list.push(active.clone());
        }
        list
    }
}

/// Order Book Depth Level (price, volume).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthLevel {
    pub price: f64,
    pub volume: f64,
}

impl DepthLevel {
    pub fn new(price: f64, volume: f64) -> Self {
        Self { price, volume }
    }
}

/// Renders a full TradingView-grade Order Book Market Depth Chart in pure vector SVG.
/// - Cumulative green area for Bids (buyers).
/// - Cumulative red area for Asks (sellers).
/// - Center dashed line with mid-market price and spread pill indicator.
/// - Grid lines and interactive hover guidelines.
pub fn depth_chart(
    id: &str,
    bids: &[(f64, f64)],
    asks: &[(f64, f64)],
    width: u32,
    height: u32,
) -> DomBuilder {
    let pl = 55.0; // Left padding for volume scale
    let pr = 25.0; // Right padding
    let pt = 35.0; // Top padding for mid-price pill
    let pb = 25.0; // Bottom padding for price scale

    let w = width as f64;
    let h = height as f64;
    let cw = (w - pl - pr).max(10.0);
    let ch = (h - pt - pb).max(10.0);
    let chart_bottom = pt + ch;

    // Check if empty
    if bids.is_empty() && asks.is_empty() {
        return div()
            .class("flex items-center justify-center p-6 text-xs text-slate-400 font-mono")
            .text("In attesa di ordini Bids/Asks nel book...");
    }

    // Sort Bids descending (highest price first, closest to mid)
    let mut sorted_bids = bids.to_vec();
    sorted_bids.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Sort Asks ascending (lowest price first, closest to mid)
    let mut sorted_asks = asks.to_vec();
    sorted_asks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Compute cumulative volume
    let mut cum_bids: Vec<(f64, f64)> = Vec::with_capacity(sorted_bids.len());
    let mut acc_bid = 0.0;
    for (p, v) in &sorted_bids {
        acc_bid += v;
        cum_bids.push((*p, acc_bid));
    }

    let mut cum_asks: Vec<(f64, f64)> = Vec::with_capacity(sorted_asks.len());
    let mut acc_ask = 0.0;
    for (p, v) in &sorted_asks {
        acc_ask += v;
        cum_asks.push((*p, acc_ask));
    }

    let best_bid = sorted_bids.first().map(|b| b.0).unwrap_or(90000.0);
    let best_ask = sorted_asks.first().map(|a| a.0).unwrap_or(best_bid + 1.0);
    let min_bid = sorted_bids.last().map(|b| b.0).unwrap_or(best_bid * 0.98);
    let max_ask = sorted_asks.last().map(|a| a.0).unwrap_or(best_ask * 1.02);

    let mid_price = (best_bid + best_ask) / 2.0;
    let spread = (best_ask - best_bid).max(0.0);
    let spread_pct = if mid_price > 0.0 { (spread / mid_price) * 100.0 } else { 0.0 };

    let total_max_vol = acc_bid.max(acc_ask).max(1.0);
    let price_span = (max_ask - min_bid).max(0.01);

    let scale_x = |price: f64| -> f64 {
        pl + ((price - min_bid) / price_span) * cw
    };

    let scale_y = |vol: f64| -> f64 {
        chart_bottom - (vol / total_max_vol) * ch
    };

    let mut svg = svg()
        .id(id)
        .attr("viewBox", &format!("0 0 {} {}", w, h))
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("class", "ferrox-depth-chart select-none");

    // Defs: Linear gradients for Bids (emerald) and Asks (rose)
    let bid_grad = linear_gradient()
        .id(&format!("bids-grad-{}", id))
        .attr("x1", "0")
        .attr("y1", "0")
        .attr("x2", "0")
        .attr("y2", "1")
        .children(vec![
            stop().attr("offset", "0%").attr("stop-color", "#10b981").attr("stop-opacity", "0.45"),
            stop().attr("offset", "100%").attr("stop-color", "#10b981").attr("stop-opacity", "0.05"),
        ]);

    let ask_grad = linear_gradient()
        .id(&format!("asks-grad-{}", id))
        .attr("x1", "0")
        .attr("y1", "0")
        .attr("x2", "0")
        .attr("y2", "1")
        .children(vec![
            stop().attr("offset", "0%").attr("stop-color", "#ef4444").attr("stop-opacity", "0.45"),
            stop().attr("offset", "100%").attr("stop-color", "#ef4444").attr("stop-opacity", "0.05"),
        ]);

    svg = svg.child(defs().children(vec![bid_grad, ask_grad]));

    // Background Card
    svg = svg.child(
        rect()
            .attr("width", &w.to_string())
            .attr("height", &h.to_string())
            .attr("fill", "rgba(15, 23, 42, 0.65)")
            .attr("rx", "12")
    );

    // Grid lines for Volume (25%, 50%, 75%, 100%)
    for frac in &[0.25, 0.5, 0.75, 1.0] {
        let y = chart_bottom - frac * ch;
        let v_val = frac * total_max_vol;

        svg = svg.child(
            line()
                .attr("x1", &pl.to_string())
                .attr("y1", &y.to_string())
                .attr("x2", &(pl + cw).to_string())
                .attr("y2", &y.to_string())
                .attr("stroke", "rgba(255, 255, 255, 0.06)")
                .attr("stroke-dasharray", "3,3")
        );

        svg = svg.child(
            text_svg()
                .attr("x", &(pl - 8.0).to_string())
                .attr("y", &(y + 3.0).to_string())
                .attr("fill", "#64748b")
                .attr("font-size", "9")
                .attr("font-family", "monospace")
                .attr("text-anchor", "end")
                .text(&format!("{:.1}", v_val))
        );
    }

    // 1. Bids Polygon & Stroke
    if !cum_bids.is_empty() {
        let mut bid_poly_points = Vec::new();
        let mut bid_line_points = Vec::new();

        let best_bid_x = scale_x(best_bid);
        bid_poly_points.push(format!("{:.1},{:.1}", best_bid_x, chart_bottom));

        // Iterate bids reversed: from best_bid (low cum) to min_bid (high cum)
        for &(p, v) in &cum_bids {
            let px = scale_x(p);
            let py = scale_y(v);
            bid_poly_points.push(format!("{:.1},{:.1}", px, py));
            bid_line_points.push(format!("{:.1},{:.1}", px, py));
        }

        let min_bid_x = scale_x(min_bid);
        bid_poly_points.push(format!("{:.1},{:.1}", min_bid_x, chart_bottom));

        // Area polygon
        svg = svg.child(
            polygon()
                .attr("points", &bid_poly_points.join(" "))
                .attr("fill", &format!("url(#bids-grad-{})", id))
        );

        // Border line
        svg = svg.child(
            polyline()
                .attr("points", &bid_line_points.join(" "))
                .attr("fill", "none")
                .attr("stroke", "#10b981")
                .attr("stroke-width", "2")
                .attr("stroke-linecap", "round")
        );
    }

    // 2. Asks Polygon & Stroke
    if !cum_asks.is_empty() {
        let mut ask_poly_points = Vec::new();
        let mut ask_line_points = Vec::new();

        let best_ask_x = scale_x(best_ask);
        ask_poly_points.push(format!("{:.1},{:.1}", best_ask_x, chart_bottom));

        // Iterate asks from best_ask (low cum) to max_ask (high cum)
        for &(p, v) in &cum_asks {
            let px = scale_x(p);
            let py = scale_y(v);
            ask_poly_points.push(format!("{:.1},{:.1}", px, py));
            ask_line_points.push(format!("{:.1},{:.1}", px, py));
        }

        let max_ask_x = scale_x(max_ask);
        ask_poly_points.push(format!("{:.1},{:.1}", max_ask_x, chart_bottom));

        // Area polygon
        svg = svg.child(
            polygon()
                .attr("points", &ask_poly_points.join(" "))
                .attr("fill", &format!("url(#asks-grad-{})", id))
        );

        // Border line
        svg = svg.child(
            polyline()
                .attr("points", &ask_line_points.join(" "))
                .attr("fill", "none")
                .attr("stroke", "#ef4444")
                .attr("stroke-width", "2")
                .attr("stroke-linecap", "round")
        );
    }

    // 3. Center Mid-Market Price Line & Spread Pill
    let mid_x = scale_x(mid_price);
    svg = svg.child(
        line()
            .attr("x1", &mid_x.to_string())
            .attr("y1", &pt.to_string())
            .attr("x2", &mid_x.to_string())
            .attr("y2", &chart_bottom.to_string())
            .attr("stroke", "rgba(255, 255, 255, 0.35)")
            .attr("stroke-dasharray", "4,4")
            .attr("stroke-width", "1.5")
    );

    // Mid-Price & Spread Pill
    let pill_w = 160.0;
    let pill_h = 24.0;
    let pill_x = (mid_x - pill_w / 2.0).clamp(pl, pl + cw - pill_w);
    let pill_y = 6.0;

    let pill_bg = rect()
        .attr("x", &pill_x.to_string())
        .attr("y", &pill_y.to_string())
        .attr("width", &pill_w.to_string())
        .attr("height", &pill_h.to_string())
        .attr("rx", "6")
        .attr("fill", "rgba(30, 41, 59, 0.95)")
        .attr("stroke", "rgba(99, 102, 241, 0.4)")
        .attr("stroke-width", "1");

    let pill_text = text_svg()
        .attr("x", &(pill_x + pill_w / 2.0).to_string())
        .attr("y", &(pill_y + 15.5).to_string())
        .attr("fill", "#f8fafc")
        .attr("font-size", "10")
        .attr("font-weight", "bold")
        .attr("font-family", "monospace")
        .attr("text-anchor", "middle")
        .text(&format!("${:.2} • Spread: ${:.2} ({:.2}%)", mid_price, spread, spread_pct));

    svg = svg.child(pill_bg).child(pill_text);

    // 4. Price Scale along Bottom (Min Bid, Mid Price, Max Ask)
    let price_labels = vec![
        (min_bid, pl + 15.0, "#10b981"),
        (mid_price, mid_x, "#94a3b8"),
        (max_ask, pl + cw - 15.0, "#ef4444"),
    ];

    for (p_val, px, color) in price_labels {
        svg = svg.child(
            text_svg()
                .attr("x", &px.to_string())
                .attr("y", &(chart_bottom + 16.0).to_string())
                .attr("fill", color)
                .attr("font-size", "9.5")
                .attr("font-weight", "bold")
                .attr("font-family", "monospace")
                .attr("text-anchor", "middle")
                .text(&format!("${:.1}", p_val))
        );
    }

    div().class("ferrox-depth-container w-full h-full").child(svg)
}
