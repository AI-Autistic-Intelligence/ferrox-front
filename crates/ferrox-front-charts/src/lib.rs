//! # Ferrox Front Charts (`ferrox-front-charts`)
//!
//! `ferrox-front-charts` provides a pure WebAssembly vector SVG charting engine inspired by Nivo.
//! It renders high-performance, responsive, theme-aware data visualisations directly as native SVG elements
//! without external JavaScript charting libraries.
//!
//! ## 📈 Supported Nivo-Style & Enterprise Chart Types
//! 1. `line_chart`: Smooth spline or linear vector line graphs with markers and hover tooltips.
//! 2. `area_chart`: Gradient-filled area charts with customizable baseline and stop opacities.
//! 3. `bar_chart`: Grouped/stacked vertical bar charts with rounded corners and value labels.
//! 4. `pie_chart`: Circular pie charts with calculated arc slices and legends.
//! 5. `donut_chart`: Ring donut charts with inner radius cutouts and center metrics.
//! 6. `radar_chart`: Spider-web polygonal radar charts for multi-attribute evaluation.
//! 7. `scatter_chart`: Multi-series scatter plot charts with variable radii and color mapping.
//! 8. `candlestick_chart`: Financial Japanese Candlestick (OHLC) chart with volume bars and price/time grid.
//! 9. `gauge_chart`: Graduated circular/semi-circle speedometer for KPI and telemetry.
//! 10. `heatmap_chart`: 2D intensity grid for traffic, latency, and activity matrix.
//! 11. `sparkline_chart`: Ultra-compact vector mini-chart (line, area, bar) for inline DataGrid and KPI cards.

pub mod market;
pub use market::*;

use ferrox_front_core::dom::{div, DomBuilder};
use std::f64::consts::PI;

/// Japanese Candlestick OHLC Data point with Volume and Time label.
#[derive(Debug, Clone)]
pub struct CandleDatum {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Fixed-capacity Circular Ring Buffer for zero-allocation live candle tick streaming.
#[derive(Debug, Clone)]
pub struct CandleRingBuffer {
    pub capacity: usize,
    pub candles: Vec<CandleDatum>,
}

impl CandleRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(10),
            candles: Vec::with_capacity(capacity),
        }
    }

    pub fn with_initial_candles(capacity: usize, initial: Vec<CandleDatum>) -> Self {
        let mut buf = Self::new(capacity);
        for c in initial {
            buf.push_candle(c);
        }
        buf
    }

    pub fn push_candle(&mut self, candle: CandleDatum) {
        if self.candles.len() >= self.capacity {
            self.candles.remove(0);
        }
        self.candles.push(candle);
    }

    /// Updates the latest candle or adds a new one on period close
    pub fn push_tick(&mut self, price: f64, volume: f64, time: &str, new_candle: bool) {
        if new_candle || self.candles.is_empty() {
            let candle = CandleDatum {
                time: time.to_string(),
                open: price,
                high: price,
                low: price,
                close: price,
                volume,
            };
            self.push_candle(candle);
        } else if let Some(last) = self.candles.last_mut() {
            if price > last.high {
                last.high = price;
            }
            if price < last.low {
                last.low = price;
            }
            last.close = price;
            last.volume += volume;
            last.time = time.to_string();
        }
    }

    pub fn push(&mut self, candle: CandleDatum) {
        self.push_candle(candle);
    }

    pub fn update_last(&mut self, price: f64, volume_delta: f64) {
        if let Some(last) = self.candles.last_mut() {
            if price > last.high {
                last.high = price;
            }
            if price < last.low {
                last.low = price;
            }
            last.close = price;
            last.volume += volume_delta;
        }
    }

    pub fn as_slice(&self) -> &[CandleDatum] {
        &self.candles
    }

    pub fn to_vec(&self) -> Vec<CandleDatum> {
        self.candles.clone()
    }

    pub fn len(&self) -> usize {
        self.candles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }
}

/// Configuration options for Technical Financial Charts (TradingView Parity).
#[derive(Debug, Clone)]
pub struct TechnicalChartOptions {
    pub show_sma20: bool,
    pub show_sma50: bool,
    pub show_bollinger: bool,
    pub show_rsi: bool,
    pub show_macd: bool,
    pub show_crosshair: bool,
    pub crosshair_pos: Option<(f64, f64)>,
}

impl Default for TechnicalChartOptions {
    fn default() -> Self {
        Self {
            show_sma20: true,
            show_sma50: true,
            show_bollinger: true,
            show_rsi: true,
            show_macd: true,
            show_crosshair: true,
            crosshair_pos: None,
        }
    }
}

/// Sparkline rendering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparklineType {
    Line,
    Area,
    Bar,
}

/// A 2D point data coordinate for Line and Area charts.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Data entry for Bar charts.
#[derive(Debug, Clone)]
pub struct BarDatum {
    pub label: String,
    pub value: f64,
    pub color: String,
}

/// Data entry for Pie and Donut charts.
#[derive(Debug, Clone)]
pub struct PieDatum {
    pub label: String,
    pub value: f64,
    pub color: String,
}

/// Data entry for Radar (Spider) charts.
#[derive(Debug, Clone)]
pub struct RadarDatum {
    pub attribute: String,
    pub value: f64, // Scaled 0.0 to 100.0
}

/// Data entry for Scatter Plot charts.
#[derive(Debug, Clone)]
pub struct ScatterDatum {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

/// Renders a Nivo-style Vector SVG Line Chart with gradient strokes.
pub fn line_chart(id: &str, data: Vec<Point>, width: u32, height: u32, show_axes: bool) -> DomBuilder {
    let mut path_d = String::new();
    
    let pl = if show_axes { 60.0 } else { 20.0 };
    let pr = 20.0;
    let pt = 20.0;
    let pb = if show_axes { 40.0 } else { 20.0 };

    let mut min_x = 0.0;
    let mut max_x = 1.0;
    let mut min_y = 0.0;
    let mut max_y = 1.0;

    if !data.is_empty() {
        max_x = data.iter().map(|p| p.x).fold(f64::MIN, f64::max);
        min_x = data.iter().map(|p| p.x).fold(f64::MAX, f64::min);
        max_y = data.iter().map(|p| p.y).fold(f64::MIN, f64::max);
        min_y = data.iter().map(|p| p.y).fold(f64::MAX, f64::min);
    }
    
    let range_x = (max_x - min_x).max(0.1);
    let range_y = (max_y - min_y).max(0.1);

    let inner_w = width as f64 - pl - pr;
    let inner_h = height as f64 - pt - pb;

    let scale_x = |x: f64| -> f64 { pl + ((x - min_x) / range_x) * inner_w };
    let scale_y = |y: f64| -> f64 { (height as f64 - pb) - ((y - min_y) / range_y) * inner_h };

    if !data.is_empty() {
        path_d.push_str(&format!("M {} {}", scale_x(data[0].x), scale_y(data[0].y)));
        for point in data.iter().skip(1) {
            path_d.push_str(&format!(" L {} {}", scale_x(point.x), scale_y(point.y)));
        }
    }

    let grad_id = format!("lineChartGrad-{}", id);
    
    let mut svg = DomBuilder::new_svg("svg")
        .attr("class", "ferrox-svg-chart")
        .attr("style", "max-width: 100%; height: auto; overflow: visible;")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .child(
            DomBuilder::new_svg("defs").child(
                DomBuilder::new_svg("linearGradient")
                    .attr("id", &grad_id)
                    .attr("x1", "0%").attr("y1", "0%").attr("x2", "100%").attr("y2", "0%")
                    .child(DomBuilder::new_svg("stop").attr("offset", "0%").attr("stop-color", "var(--accent-glow, #9333ea)"))
                    .child(DomBuilder::new_svg("stop").attr("offset", "100%").attr("stop-color", "var(--accent-cyan, #06b6d4)"))
            )
        );

    // Draw Grid and Axes if enabled
    if show_axes {
        let grid_color = "rgba(255,255,255,0.1)";
        let text_color = "rgba(255,255,255,0.5)";
        
        for i in 0..=5 {
            let val = min_y + (range_y * (i as f64 / 5.0));
            let y_pos = scale_y(val);
            
            svg = svg.child(
                DomBuilder::new_svg("line")
                    .attr("x1", &pl.to_string())
                    .attr("x2", &(width as f64 - pr).to_string())
                    .attr("y1", &y_pos.to_string())
                    .attr("y2", &y_pos.to_string())
                    .attr("stroke", grid_color)
                    .attr("stroke-width", "1")
            ).child(
                DomBuilder::new_svg("text")
                    .attr("x", &(pl - 10.0).to_string())
                    .attr("y", &(y_pos + 4.0).to_string())
                    .attr("fill", text_color)
                    .attr("font-size", "12")
                    .attr("font-family", "sans-serif")
                    .attr("text-anchor", "end")
                    .text(&format!("{:.2}", val))
            );
        }

        for i in 0..=5 {
            let val = min_x + (range_x * (i as f64 / 5.0));
            let x_pos = scale_x(val);
            
            svg = svg.child(
                DomBuilder::new_svg("line")
                    .attr("x1", &x_pos.to_string())
                    .attr("x2", &x_pos.to_string())
                    .attr("y1", &pt.to_string())
                    .attr("y2", &(height as f64 - pb).to_string())
                    .attr("stroke", grid_color)
                    .attr("stroke-width", "1")
            ).child(
                DomBuilder::new_svg("text")
                    .attr("x", &x_pos.to_string())
                    .attr("y", &(height as f64 - pb + 20.0).to_string())
                    .attr("fill", text_color)
                    .attr("font-size", "12")
                    .attr("font-family", "sans-serif")
                    .attr("text-anchor", "middle")
                    .text(&format!("{:.1}", val))
            );
        }
    }

    svg = svg.child(
        DomBuilder::new_svg("path")
            .attr("d", &path_d)
            .attr("fill", "none")
            .attr("stroke", &format!("url(#{})", grad_id))
            .attr("stroke-width", "4")
            .attr("stroke-linecap", "round")
            .attr("stroke-linejoin", "round")
    );

    if !data.is_empty() {
        for point in &data {
            let cx = scale_x(point.x);
            let cy = scale_y(point.y);
            let circle = DomBuilder::new_svg("circle")
                .attr("cx", &cx.to_string())
                .attr("cy", &cy.to_string())
                .attr("r", "5")
                .attr("fill", "var(--accent-cyan, #06b6d4)")
                .attr("cursor", "crosshair")
                .child(DomBuilder::new_svg("title").text(&format!("Valore: {:.3}", point.y)));
            
            svg = svg.child(circle);
        }
    }

    div().child(svg)
}

/// Renders a Nivo-style Gradient Area Chart with filled baseline opacity.
pub fn area_chart(data: Vec<Point>, width: u32, height: u32) -> DomBuilder {
    let mut line_d = String::new();
    if !data.is_empty() {
        line_d.push_str(&format!("M {} {}", data[0].x, data[0].y));
        for pt in data.iter().skip(1) {
            line_d.push_str(&format!(" L {} {}", pt.x, pt.y));
        }
    }

    let area_d = if !data.is_empty() {
        let last = data.last().unwrap();
        let first = data.first().unwrap();
        format!("{} L {} {} L {} {} Z", line_d, last.x, height, first.x, height)
    } else {
        String::new()
    };

    div().child(
        DomBuilder::new_svg("svg")
            .attr("class", "ferrox-svg-area-chart")
            .attr("style", "max-width: 100%; height: auto;")
            .attr("width", &width.to_string())
            .attr("height", &height.to_string())
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .child(
                DomBuilder::new_svg("defs").child(
                    DomBuilder::new_svg("linearGradient")
                        .attr("id", "areaFillGrad")
                        .attr("x1", "0%").attr("y1", "0%").attr("x2", "0%").attr("y2", "100%")
                        .child(DomBuilder::new_svg("stop").attr("offset", "0%").attr("stop-color", "var(--accent-cyan, #06b6d4)").attr("stop-opacity", "0.4"))
                        .child(DomBuilder::new_svg("stop").attr("offset", "100%").attr("stop-color", "var(--accent-cyan, #06b6d4)").attr("stop-opacity", "0.0"))
                )
            )
            .child(DomBuilder::new_svg("path").attr("d", &area_d).attr("fill", "url(#areaFillGrad)"))
            .child(DomBuilder::new_svg("path").attr("d", &line_d).attr("fill", "none").attr("stroke", "var(--accent-cyan, #06b6d4)").attr("stroke-width", "3"))
    )
}

pub fn bar_chart(id: &str, data: Vec<BarDatum>, width: u32, height: u32, show_axes: bool) -> DomBuilder {
    let mut svg = DomBuilder::new_svg("svg")
        .attr("id", id)
        .attr("class", "ferrox-svg-bar-chart")
        .attr("style", "max-width: 100%; height: auto; overflow: visible;")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    let max_val = data.iter().map(|d| d.value).fold(0.001, f64::max);
    let bar_count = data.len();
    
    let pl = if show_axes { 60.0 } else { 0.0 };
    let pr = 20.0;
    let pt = 20.0;
    let pb = if show_axes { 40.0 } else { 30.0 };
    let inner_h = height as f64 - pt - pb;

    if show_axes {
        let grid_color = "rgba(255,255,255,0.1)";
        let text_color = "rgba(255,255,255,0.5)";
        
        for i in 0..=5 {
            let val = max_val * (i as f64 / 5.0);
            let y_pos = (height as f64 - pb) - (val / max_val) * inner_h;
            
            svg = svg.child(
                DomBuilder::new_svg("line")
                    .attr("x1", &pl.to_string())
                    .attr("x2", &(width as f64 - pr).to_string())
                    .attr("y1", &y_pos.to_string())
                    .attr("y2", &y_pos.to_string())
                    .attr("stroke", grid_color)
                    .attr("stroke-width", "1")
            ).child(
                DomBuilder::new_svg("text")
                    .attr("x", &(pl - 10.0).to_string())
                    .attr("y", &(y_pos + 4.0).to_string())
                    .attr("fill", text_color)
                    .attr("font-size", "12")
                    .attr("font-family", "sans-serif")
                    .attr("text-anchor", "end")
                    .text(&format!("{:.2}", val))
            );
        }
    }

    let padding = 20.0;
    let available_w = (width as f64 - pl - pr) - (padding * (bar_count as f64 + 1.0));
    let bar_w = (available_w / bar_count as f64).clamp(10.0, 60.0);
    
    let actual_total_w = (bar_w * bar_count as f64) + (padding * (bar_count as f64 - 1.0));
    let start_x = pl + ((width as f64 - pl - pr) - actual_total_w) / 2.0;

    for (i, datum) in data.iter().enumerate() {
        let bar_h = (datum.value / max_val) * inner_h;
        let x = start_x + i as f64 * (bar_w + padding);
        let y = (height as f64 - pb) - bar_h;

        let rect = DomBuilder::new_svg("rect")
            .attr("x", &x.to_string())
            .attr("y", &y.to_string())
            .attr("width", &bar_w.to_string())
            .attr("height", &bar_h.to_string())
            .attr("rx", "6")
            .attr("fill", &datum.color)
            .attr("cursor", "crosshair")
            .child(DomBuilder::new_svg("title").text(&format!("{}: {:.3}", datum.label, datum.value)));

        let label = DomBuilder::new_svg("text")
            .attr("x", &(x + bar_w / 2.0).to_string())
            .attr("y", &(height as f64 - 10.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-primary, #fff)")
            .attr("font-size", "12")
            .text(&datum.label);

        svg = svg.child(rect).child(label);
    }

    div().child(svg)
}

/// Renders a Nivo-style Donut Chart with inner radius cutout and center text.
pub fn donut_chart(data: Vec<PieDatum>, radius: f64, inner_radius: f64, center_title: &str, center_val: &str) -> DomBuilder {
    let size = (radius * 2.0) + 40.0;
    let cx = size / 2.0;
    let cy = size / 2.0;
    let total: f64 = data.iter().map(|d| d.value).sum();

    let mut svg = DomBuilder::new_svg("svg")
        .attr("class", "ferrox-svg-donut-chart")
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("viewBox", &format!("0 0 {} {}", size, size));

    let mut current_angle = -PI / 2.0; // Start at top

    for datum in &data {
        let slice_angle = (datum.value / total) * 2.0 * PI;
        let end_angle = current_angle + slice_angle;

        let x1_outer = cx + radius * current_angle.cos();
        let y1_outer = cy + radius * current_angle.sin();
        let x2_outer = cx + radius * end_angle.cos();
        let y2_outer = cy + radius * end_angle.sin();

        let x1_inner = cx + inner_radius * end_angle.cos();
        let y1_inner = cy + inner_radius * end_angle.sin();
        let x2_inner = cx + inner_radius * current_angle.cos();
        let y2_inner = cy + inner_radius * current_angle.sin();

        let large_arc = if slice_angle > PI { "1" } else { "0" };

        let d = format!(
            "M {} {} A {} {} 0 {} 1 {} {} L {} {} A {} {} 0 {} 0 {} {} Z",
            x1_outer, y1_outer, radius, radius, large_arc, x2_outer, y2_outer,
            x1_inner, y1_inner, inner_radius, inner_radius, large_arc, x2_inner, y2_inner
        );

        let path = DomBuilder::new_svg("path")
            .attr("d", &d)
            .attr("fill", &datum.color)
            .attr("stroke", "var(--bg-primary, #0a0a16)")
            .attr("stroke-width", "2");

        svg = svg.child(path);
        current_angle = end_angle;
    }

    // Center Text Info
    let title_text = DomBuilder::new_svg("text")
        .attr("x", &cx.to_string())
        .attr("y", &(cy - 6.0).to_string())
        .attr("text-anchor", "middle")
        .attr("fill", "var(--accent-cyan, #06b6d4)")
        .attr("font-size", "14")
        .attr("font-weight", "600")
        .text(center_title);

    let val_text = DomBuilder::new_svg("text")
        .attr("x", &cx.to_string())
        .attr("y", &(cy + 16.0).to_string())
        .attr("text-anchor", "middle")
        .attr("fill", "var(--text-primary, #fff)")
        .attr("font-size", "18")
        .attr("font-weight", "700")
        .text(center_val);

    div().child(svg.child(title_text).child(val_text))
}

/// Renders a Nivo-style Polygonal Radar (Spider) Chart for multi-attribute evaluation.
pub fn radar_chart(data: Vec<RadarDatum>, radius: f64) -> DomBuilder {
    let size = (radius * 2.0) + 80.0;
    let cx = size / 2.0;
    let cy = size / 2.0;
    let count = data.len();

    let mut svg = DomBuilder::new_svg("svg")
        .attr("class", "ferrox-svg-radar-chart")
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("viewBox", &format!("0 0 {} {}", size, size));

    // Concentric grid rings
    for level in [0.25, 0.50, 0.75, 1.00] {
        let r = radius * level;
        let mut grid_pts = String::new();
        for i in 0..count {
            let angle = (i as f64 / count as f64) * 2.0 * PI - (PI / 2.0);
            let x = cx + r * angle.cos();
            let y = cy + r * angle.sin();
            grid_pts.push_str(&format!("{},{} ", x, y));
        }
        svg = svg.child(
            DomBuilder::new_svg("polygon")
                .attr("points", grid_pts.trim())
                .attr("fill", "none")
                .attr("stroke", "rgba(255,255,255,0.15)")
                .attr("stroke-width", "1")
        );
    }

    // Dataset Polygon
    let mut data_pts = String::new();
    for (i, datum) in data.iter().enumerate() {
        let r = radius * (datum.value / 100.0).clamp(0.0, 1.0);
        let angle = (i as f64 / count as f64) * 2.0 * PI - (PI / 2.0);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        data_pts.push_str(&format!("{},{} ", x, y));

        // Attribute Label
        let label_r = radius + 20.0;
        let lx = cx + label_r * angle.cos();
        let ly = cy + label_r * angle.sin();
        let label = DomBuilder::new_svg("text")
            .attr("x", &lx.to_string())
            .attr("y", &ly.to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-primary, #fff)")
            .attr("font-size", "11")
            .text(&datum.attribute);
        svg = svg.child(label);
    }

    let polygon = DomBuilder::new_svg("polygon")
        .attr("points", data_pts.trim())
        .attr("fill", "rgba(147, 51, 234, 0.35)")
        .attr("stroke", "var(--accent-glow, #9333ea)")
        .attr("stroke-width", "3");

    div().child(svg.child(polygon))
}

/// Renders a Nivo-style Scatter Plot Chart with variable node radii and color mapping.
pub fn scatter_chart(data: Vec<ScatterDatum>, width: u32, height: u32) -> DomBuilder {
    let mut svg = DomBuilder::new_svg("svg")
        .attr("class", "ferrox-svg-scatter-chart")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    for pt in data {
        let circle = DomBuilder::new_svg("circle")
            .attr("cx", &pt.x.to_string())
            .attr("cy", &pt.y.to_string())
            .attr("r", &pt.radius.to_string())
            .attr("fill", &pt.color)
            .attr("opacity", "0.85")
            .attr("stroke", "var(--bg-primary, #0a0a16)")
            .attr("stroke-width", "1.5");
        svg = svg.child(circle);
    }

    div().child(svg)
}

// =============================================================================
// TECHNICAL ANALYSIS CALCULATION HELPERS
// =============================================================================

/// Calculates Simple Moving Average (SMA)
pub fn calculate_sma(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    let n = prices.len();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        if i + 1 < period {
            out.push(None);
        } else {
            let sum: f64 = prices[i + 1 - period..=i].iter().sum();
            out.push(Some(sum / period as f64));
        }
    }
    out
}

/// Calculates Bollinger Bands (Upper, Middle SMA, Lower)
pub fn calculate_bollinger(
    prices: &[f64],
    period: usize,
    multiplier: f64,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let sma = calculate_sma(prices, period);
    let n = prices.len();
    let mut upper = Vec::with_capacity(n);
    let mut lower = Vec::with_capacity(n);

    for i in 0..n {
        if let Some(m) = sma[i] {
            let variance: f64 = prices[i + 1 - period..=i]
                .iter()
                .map(|&p| (p - m).powi(2))
                .sum::<f64>()
                / period as f64;
            let std_dev = variance.sqrt();
            upper.push(Some(m + multiplier * std_dev));
            lower.push(Some(m - multiplier * std_dev));
        } else {
            upper.push(None);
            lower.push(None);
        }
    }
    (upper, sma, lower)
}

/// Calculates Relative Strength Index (RSI) for given period (typically 14)
pub fn calculate_rsi(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    let n = prices.len();
    let mut out = Vec::with_capacity(n);
    if n < period + 1 {
        out.resize(n, None);
        return out;
    }

    out.push(None);
    let mut gains = Vec::with_capacity(n);
    let mut losses = Vec::with_capacity(n);

    for i in 1..n {
        let diff = prices[i] - prices[i - 1];
        if diff >= 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    for _ in 1..period {
        out.push(None);
    }

    let mut avg_gain: f64 = gains[0..period].iter().sum::<f64>() / period as f64;
    let mut avg_loss: f64 = losses[0..period].iter().sum::<f64>() / period as f64;
    let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
    out.push(Some((100.0 - (100.0 / (1.0 + rs))).clamp(0.0, 100.0)));

    for i in period..gains.len() {
        avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i]) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i]) / period as f64;
        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        out.push(Some((100.0 - (100.0 / (1.0 + rs))).clamp(0.0, 100.0)));
    }

    out
}

/// Calculates MACD (MACD Line, Signal Line, Histogram)
pub fn calculate_macd(
    prices: &[f64],
    fast: usize,
    slow: usize,
    signal: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let n = prices.len();
    if n < slow {
        return (vec![None; n], vec![None; n], vec![None; n]);
    }

    // Helper for EMA
    let calc_ema = |period: usize| -> Vec<Option<f64>> {
        let mut ema = Vec::with_capacity(n);
        let k = 2.0 / (period as f64 + 1.0);
        let mut current_ema = 0.0;
        for i in 0..n {
            if i + 1 < period {
                ema.push(None);
            } else if i + 1 == period {
                current_ema = prices[0..period].iter().sum::<f64>() / period as f64;
                ema.push(Some(current_ema));
            } else {
                current_ema = (prices[i] * k) + (current_ema * (1.0 - k));
                ema.push(Some(current_ema));
            }
        }
        ema
    };

    let ema_fast = calc_ema(fast);
    let ema_slow = calc_ema(slow);

    let mut macd_line = Vec::with_capacity(n);
    for i in 0..n {
        match (ema_fast[i], ema_slow[i]) {
            (Some(f), Some(s)) => macd_line.push(Some(f - s)),
            _ => macd_line.push(None),
        }
    }

    // Signal line: EMA of MACD line
    let mut signal_line = Vec::with_capacity(n);
    let valid_macd: Vec<f64> = macd_line.iter().filter_map(|&v| v).collect();
    let k_sig = 2.0 / (signal as f64 + 1.0);

    let mut sig_values = Vec::new();
    if valid_macd.len() >= signal {
        let mut cur_sig = valid_macd[0..signal].iter().sum::<f64>() / signal as f64;
        sig_values.push(cur_sig);
        for v in &valid_macd[signal..] {
            cur_sig = (*v * k_sig) + (cur_sig * (1.0 - k_sig));
            sig_values.push(cur_sig);
        }
    }

    let mut sig_idx = 0;
    let sig_start = n - valid_macd.len() + signal - 1;
    for i in 0..n {
        if i < sig_start || sig_idx >= sig_values.len() {
            signal_line.push(None);
        } else {
            signal_line.push(Some(sig_values[sig_idx]));
            sig_idx += 1;
        }
    }

    let mut hist = Vec::with_capacity(n);
    for i in 0..n {
        match (macd_line[i], signal_line[i]) {
            (Some(m), Some(s)) => hist.push(Some(m - s)),
            _ => hist.push(None),
        }
    }

    (macd_line, signal_line, hist)
}

// =============================================================================
// TRADINGVIEW PARITY FINANCIAL & TECHNICAL ANALYSIS CHART
// =============================================================================

/// Renders a full TradingView-grade Financial Technical Analysis Chart with
/// Candlesticks (OHLC), Volume, Bollinger Bands, SMA 20/50, RSI (14) panel,
/// MACD (12,26,9) panel, dynamic Data Reader, and interactive crosshair.
pub fn financial_technical_chart(
    id: &str,
    data: &[CandleDatum],
    options: &TechnicalChartOptions,
    width: u32,
    height: u32,
) -> DomBuilder {
    let pl = 65.0; // Left padding for price scale
    let pr = 20.0; // Right padding
    let pt = 35.0; // Top padding for header data reader
    let pb = 25.0; // Bottom padding for time labels

    let w = width as f64;
    let h = height as f64;
    let plot_w = (w - pl - pr).max(50.0);
    let total_plot_h = (h - pt - pb).max(100.0);

    let n = data.len();
    if n == 0 {
        return div().child(
            DomBuilder::new_svg("svg")
                .attr("width", &width.to_string())
                .attr("height", &height.to_string())
        );
    }

    // Allocate vertical space for sub-panels
    let has_rsi = options.show_rsi;
    let has_macd = options.show_macd;
    let osc_count = (if has_rsi { 1 } else { 0 }) + (if has_macd { 1 } else { 0 });

    let (main_h, rsi_h, macd_h, rsi_top, macd_top) = match osc_count {
        2 => {
            let m = total_plot_h * 0.58;
            let r = total_plot_h * 0.18;
            let c = total_plot_h * 0.18;
            let gap = total_plot_h * 0.03;
            (m, r, c, pt + m + gap, pt + m + r + gap * 2.0)
        }
        1 => {
            let m = total_plot_h * 0.72;
            let sub = total_plot_h * 0.24;
            let gap = total_plot_h * 0.04;
            let sub_top = pt + m + gap;
            if has_rsi {
                (m, sub, 0.0, sub_top, 0.0)
            } else {
                (m, 0.0, sub, 0.0, sub_top)
            }
        }
        _ => (total_plot_h, 0.0, 0.0, 0.0, 0.0),
    };

    let vol_h = main_h * 0.20;
    let price_h = main_h * 0.74;
    let vol_top = pt + price_h + (main_h * 0.06);

    let close_prices: Vec<f64> = data.iter().map(|c| c.close).collect();
    let sma20 = calculate_sma(&close_prices, 20);
    let sma50 = calculate_sma(&close_prices, 50);
    let (bb_upper, _, bb_lower) = calculate_bollinger(&close_prices, 20, 2.0);
    let rsi_vals = calculate_rsi(&close_prices, 14);
    let (macd_line, macd_sig, macd_hist) = calculate_macd(&close_prices, 12, 26, 9);

    // Min & Max calculations for price
    let mut min_price = f64::MAX;
    let mut max_price = f64::MIN;
    let mut max_volume = 1.0f64;

    for c in data {
        if c.low < min_price { min_price = c.low; }
        if c.high > max_price { max_price = c.high; }
        if c.volume > max_volume { max_volume = c.volume; }
    }

    if options.show_bollinger {
        for u in &bb_upper {
            if let Some(v) = *u { if v > max_price { max_price = v; } }
        }
        for l in &bb_lower {
            if let Some(v) = *l { if v < min_price { min_price = v; } }
        }
    }

    let p_span = (max_price - min_price).max(1.0);
    let p_min = min_price - p_span * 0.03;
    let p_max = max_price + p_span * 0.03;
    let p_range = (p_max - p_min).max(1.0);

    let scale_y_price = |p: f64| -> f64 { (pt + price_h) - ((p - p_min) / p_range) * price_h };
    let scale_y_vol = |v: f64| -> f64 { (vol_top + vol_h) - (v / max_volume) * vol_h };

    let slot_w = plot_w / n as f64;
    let candle_w = (slot_w * 0.65).clamp(3.0, 32.0);

    let mut svg = DomBuilder::new_svg("svg")
        .attr("id", id)
        .attr("class", "ferrox-tradingview-chart")
        .attr("style", "max-width: 100%; height: auto; overflow: visible; font-family: -apple-system, BlinkMacSystemFont, sans-serif;")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    // Dynamic Header Data Reader
    let latest = &data[n - 1];
    let latest_sma20 = sma20.last().and_then(|&v| v).unwrap_or(latest.close);
    let latest_rsi = rsi_vals.last().and_then(|&v| v).unwrap_or(50.0);
    let latest_macd = macd_hist.last().and_then(|&v| v).unwrap_or(0.0);

    let reader_text = format!(
        "BTC/USD • O: {:.2}  H: {:.2}  L: {:.2}  C: {:.2}  Vol: {:.0}  SMA20: {:.2}  RSI(14): {:.1}  MACD: {:+.2}",
        latest.open, latest.high, latest.low, latest.close, latest.volume, latest_sma20, latest_rsi, latest_macd
    );

    svg = svg.child(
        DomBuilder::new_svg("text")
            .attr("x", &pl.to_string())
            .attr("y", &(pt - 14.0).to_string())
            .attr("fill", "var(--text-primary, #ffffff)")
            .attr("font-size", "11")
            .attr("font-family", "monospace")
            .attr("font-weight", "600")
            .text(&reader_text)
    );

    // 1. Horizontal Price Grid (5 ticks)
    for i in 0..=4 {
        let val = p_min + (p_range * (i as f64 / 4.0));
        let y = scale_y_price(val);
        svg = svg.child(
            DomBuilder::new_svg("line")
                .attr("x1", &pl.to_string())
                .attr("x2", &(w - pr).to_string())
                .attr("y1", &y.to_string())
                .attr("y2", &y.to_string())
                .attr("stroke", "rgba(255,255,255,0.06)")
                .attr("stroke-dasharray", "4 4")
        ).child(
            DomBuilder::new_svg("text")
                .attr("x", &(pl - 8.0).to_string())
                .attr("y", &(y + 4.0).to_string())
                .attr("fill", "rgba(255,255,255,0.4)")
                .attr("font-size", "10")
                .attr("text-anchor", "end")
                .text(&format!("{:.2}", val))
        );
    }

    // 2. Bollinger Bands (Semi-transparent channel + lines)
    if options.show_bollinger {
        let mut upper_pts = Vec::new();
        let mut lower_pts = Vec::new();

        for i in 0..n {
            let cx = pl + (i as f64 + 0.5) * slot_w;
            if let (Some(u), Some(l)) = (bb_upper[i], bb_lower[i]) {
                upper_pts.push((cx, scale_y_price(u)));
                lower_pts.push((cx, scale_y_price(l)));
            }
        }

        if !upper_pts.is_empty() {
            let mut channel_d = format!("M {} {}", upper_pts[0].0, upper_pts[0].1);
            for pt in upper_pts.iter().skip(1) {
                channel_d.push_str(&format!(" L {} {}", pt.0, pt.1));
            }
            for pt in lower_pts.iter().rev() {
                channel_d.push_str(&format!(" L {} {}", pt.0, pt.1));
            }
            channel_d.push_str(" Z");

            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &channel_d)
                    .attr("fill", "rgba(56, 189, 248, 0.08)")
            );

            // Upper band line
            let mut u_d = format!("M {} {}", upper_pts[0].0, upper_pts[0].1);
            for pt in upper_pts.iter().skip(1) {
                u_d.push_str(&format!(" L {} {}", pt.0, pt.1));
            }
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &u_d)
                    .attr("fill", "none")
                    .attr("stroke", "rgba(56, 189, 248, 0.4)")
                    .attr("stroke-width", "1")
                    .attr("stroke-dasharray", "3 3")
            );

            // Lower band line
            let mut l_d = format!("M {} {}", lower_pts[0].0, lower_pts[0].1);
            for pt in lower_pts.iter().skip(1) {
                l_d.push_str(&format!(" L {} {}", pt.0, pt.1));
            }
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &l_d)
                    .attr("fill", "none")
                    .attr("stroke", "rgba(56, 189, 248, 0.4)")
                    .attr("stroke-width", "1")
                    .attr("stroke-dasharray", "3 3")
            );
        }
    }

    // 3. SMA 20 (Yellow) and SMA 50 (Cyan) Overlays
    if options.show_sma20 {
        let mut sma_d = String::new();
        for (i, opt) in sma20.iter().enumerate() {
            if let Some(v) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_price(v);
                if sma_d.is_empty() {
                    sma_d = format!("M {} {}", cx, cy);
                } else {
                    sma_d.push_str(&format!(" L {} {}", cx, cy));
                }
            }
        }
        if !sma_d.is_empty() {
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &sma_d)
                    .attr("fill", "none")
                    .attr("stroke", "#facc15")
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
            );
        }
    }

    if options.show_sma50 {
        let mut sma50_d = String::new();
        for (i, opt) in sma50.iter().enumerate() {
            if let Some(v) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_price(v);
                if sma50_d.is_empty() {
                    sma50_d = format!("M {} {}", cx, cy);
                } else {
                    sma50_d.push_str(&format!(" L {} {}", cx, cy));
                }
            }
        }
        if !sma50_d.is_empty() {
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &sma50_d)
                    .attr("fill", "none")
                    .attr("stroke", "#38bdf8")
                    .attr("stroke-width", "1.5")
                    .attr("stroke-linecap", "round")
            );
        }
    }

    // 4. Candlesticks and Underlying Volume Bars
    for (i, c) in data.iter().enumerate() {
        let cx = pl + (i as f64 + 0.5) * slot_w;
        let is_bullish = c.close >= c.open;
        let color = if is_bullish { "#10b981" } else { "#ef4444" };

        let y_high = scale_y_price(c.high);
        let y_low = scale_y_price(c.low);
        let y_open = scale_y_price(c.open);
        let y_close = scale_y_price(c.close);

        let body_top = y_open.min(y_close);
        let body_bottom = y_open.max(y_close);
        let body_h = (body_bottom - body_top).max(2.0);

        let mut candle_group = DomBuilder::new_svg("g")
            .attr("class", "ferrox-candle-node")
            .attr("cursor", "crosshair");

        // Wick
        candle_group = candle_group.child(
            DomBuilder::new_svg("line")
                .attr("x1", &cx.to_string())
                .attr("x2", &cx.to_string())
                .attr("y1", &y_high.to_string())
                .attr("y2", &y_low.to_string())
                .attr("stroke", color)
                .attr("stroke-width", "1.5")
        );

        // Body
        candle_group = candle_group.child(
            DomBuilder::new_svg("rect")
                .attr("x", &(cx - candle_w / 2.0).to_string())
                .attr("y", &body_top.to_string())
                .attr("width", &candle_w.to_string())
                .attr("height", &body_h.to_string())
                .attr("rx", "1.5")
                .attr("fill", color)
        );

        // Volume Bar
        let vol_y = scale_y_vol(c.volume);
        let vol_bar_h = ((vol_top + vol_h) - vol_y).max(1.0);
        candle_group = candle_group.child(
            DomBuilder::new_svg("rect")
                .attr("x", &(cx - candle_w / 2.0).to_string())
                .attr("y", &vol_y.to_string())
                .attr("width", &candle_w.to_string())
                .attr("height", &vol_bar_h.to_string())
                .attr("rx", "1")
                .attr("fill", color)
                .attr("opacity", "0.45")
        );

        let tooltip_text = format!(
            "Time: {}\nO: {:.2}  H: {:.2}\nL: {:.2}  C: {:.2}\nVol: {:.0}",
            c.time, c.open, c.high, c.low, c.close, c.volume
        );
        candle_group = candle_group.child(DomBuilder::new_svg("title").text(&tooltip_text));
        svg = svg.child(candle_group);
    }

    // 5. RSI (14) Sub-Panel
    if has_rsi {
        let rsi_bottom = rsi_top + rsi_h;
        let scale_y_rsi = |val: f64| -> f64 { rsi_bottom - (val / 100.0) * rsi_h };

        svg = svg.child(
            DomBuilder::new_svg("rect")
                .attr("x", &pl.to_string())
                .attr("y", &rsi_top.to_string())
                .attr("width", &plot_w.to_string())
                .attr("height", &rsi_h.to_string())
                .attr("fill", "rgba(0,0,0,0.2)")
                .attr("stroke", "rgba(255,255,255,0.08)")
        ).child(
            DomBuilder::new_svg("text")
                .attr("x", &(pl - 8.0).to_string())
                .attr("y", &(rsi_top + 12.0).to_string())
                .attr("fill", "#c084fc")
                .attr("font-size", "9")
                .attr("font-weight", "700")
                .attr("text-anchor", "end")
                .text("RSI 14")
        );

        // 70 (Overbought) and 30 (Oversold) lines
        let y_70 = scale_y_rsi(70.0);
        let y_30 = scale_y_rsi(30.0);
        svg = svg.child(
            DomBuilder::new_svg("line")
                .attr("x1", &pl.to_string()).attr("x2", &(w - pr).to_string())
                .attr("y1", &y_70.to_string()).attr("y2", &y_70.to_string())
                .attr("stroke", "#f59e0b").attr("stroke-dasharray", "3 3").attr("opacity", "0.5")
        ).child(
            DomBuilder::new_svg("line")
                .attr("x1", &pl.to_string()).attr("x2", &(w - pr).to_string())
                .attr("y1", &y_30.to_string()).attr("y2", &y_30.to_string())
                .attr("stroke", "#10b981").attr("stroke-dasharray", "3 3").attr("opacity", "0.5")
        );

        // RSI Line
        let mut rsi_d = String::new();
        for (i, opt) in rsi_vals.iter().enumerate() {
            if let Some(val) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_rsi(val);
                if rsi_d.is_empty() {
                    rsi_d = format!("M {} {}", cx, cy);
                } else {
                    rsi_d.push_str(&format!(" L {} {}", cx, cy));
                }
            }
        }
        if !rsi_d.is_empty() {
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &rsi_d)
                    .attr("fill", "none")
                    .attr("stroke", "#c084fc")
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
            );
        }
    }

    // 6. MACD (12, 26, 9) Sub-Panel
    if has_macd {
        let _macd_bottom = macd_top + macd_h;
        let mut max_hist = 1.0f64;
        for opt in &macd_hist {
            if let Some(v) = *opt {
                if v.abs() > max_hist { max_hist = v.abs(); }
            }
        }
        let macd_mid = macd_top + (macd_h / 2.0);
        let scale_y_macd = |val: f64| -> f64 { macd_mid - (val / max_hist) * (macd_h / 2.0) };

        svg = svg.child(
            DomBuilder::new_svg("rect")
                .attr("x", &pl.to_string())
                .attr("y", &macd_top.to_string())
                .attr("width", &plot_w.to_string())
                .attr("height", &macd_h.to_string())
                .attr("fill", "rgba(0,0,0,0.2)")
                .attr("stroke", "rgba(255,255,255,0.08)")
        ).child(
            DomBuilder::new_svg("text")
                .attr("x", &(pl - 8.0).to_string())
                .attr("y", &(macd_top + 12.0).to_string())
                .attr("fill", "#38bdf8")
                .attr("font-size", "9")
                .attr("font-weight", "700")
                .attr("text-anchor", "end")
                .text("MACD")
        ).child(
            // Baseline 0
            DomBuilder::new_svg("line")
                .attr("x1", &pl.to_string()).attr("x2", &(w - pr).to_string())
                .attr("y1", &macd_mid.to_string()).attr("y2", &macd_mid.to_string())
                .attr("stroke", "rgba(255,255,255,0.2)")
        );

        // MACD Histogram Bars
        for (i, opt) in macd_hist.iter().enumerate() {
            if let Some(val) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_macd(val);
                let bar_top = cy.min(macd_mid);
                let bar_h = (cy - macd_mid).abs().max(1.0);
                let col = if val >= 0.0 { "#10b981" } else { "#ef4444" };

                svg = svg.child(
                    DomBuilder::new_svg("rect")
                        .attr("x", &(cx - candle_w / 3.0).to_string())
                        .attr("y", &bar_top.to_string())
                        .attr("width", &(candle_w * 0.66).to_string())
                        .attr("height", &bar_h.to_string())
                        .attr("fill", col)
                        .attr("opacity", "0.7")
                );
            }
        }

        // MACD Line
        let mut line_d = String::new();
        for (i, opt) in macd_line.iter().enumerate() {
            if let Some(val) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_macd(val);
                if line_d.is_empty() {
                    line_d = format!("M {} {}", cx, cy);
                } else {
                    line_d.push_str(&format!(" L {} {}", cx, cy));
                }
            }
        }
        if !line_d.is_empty() {
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &line_d)
                    .attr("fill", "none")
                    .attr("stroke", "#3b82f6")
                    .attr("stroke-width", "1.5")
            );
        }

        // Signal Line
        let mut sig_d = String::new();
        for (i, opt) in macd_sig.iter().enumerate() {
            if let Some(val) = *opt {
                let cx = pl + (i as f64 + 0.5) * slot_w;
                let cy = scale_y_macd(val);
                if sig_d.is_empty() {
                    sig_d = format!("M {} {}", cx, cy);
                } else {
                    sig_d.push_str(&format!(" L {} {}", cx, cy));
                }
            }
        }
        if !sig_d.is_empty() {
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &sig_d)
                    .attr("fill", "none")
                    .attr("stroke", "#f59e0b")
                    .attr("stroke-width", "1.5")
            );
        }
    }

    // 7. Time Axis Labels (Every few ticks)
    let step = (n / 6).max(1);
    for (i, c) in data.iter().enumerate() {
        if i % step == 0 || i == n - 1 {
            let cx = pl + (i as f64 + 0.5) * slot_w;
            svg = svg.child(
                DomBuilder::new_svg("text")
                    .attr("x", &cx.to_string())
                    .attr("y", &(h - 8.0).to_string())
                    .attr("fill", "rgba(255,255,255,0.45)")
                    .attr("font-size", "10")
                    .attr("text-anchor", "middle")
                    .text(&c.time)
            );
        }
    }

    // 8. Interactive Crosshair Overlay
    if options.show_crosshair {
        let hover_x = options.crosshair_pos.map(|(x, _)| x).unwrap_or(pl + (n as f64 - 0.5) * slot_w);
        let hover_y = options.crosshair_pos.map(|(_, y)| y).unwrap_or(scale_y_price(latest.close));

        svg = svg.child(
            // Vertical crosshair line
            DomBuilder::new_svg("line")
                .attr("x1", &hover_x.to_string())
                .attr("x2", &hover_x.to_string())
                .attr("y1", &pt.to_string())
                .attr("y2", &(h - pb).to_string())
                .attr("stroke", "rgba(255,255,255,0.25)")
                .attr("stroke-dasharray", "3 3")
        ).child(
            // Horizontal crosshair line on price chart
            DomBuilder::new_svg("line")
                .attr("x1", &pl.to_string())
                .attr("x2", &(w - pr).to_string())
                .attr("y1", &hover_y.to_string())
                .attr("y2", &hover_y.to_string())
                .attr("stroke", "rgba(255,255,255,0.25)")
                .attr("stroke-dasharray", "3 3")
        );
    }

    div().child(svg)
}

/// Standard Japanese Candlestick Chart (Backward-Compatible API)
pub fn candlestick_chart(id: &str, data: Vec<CandleDatum>, width: u32, height: u32) -> DomBuilder {
    financial_technical_chart(id, &data, &TechnicalChartOptions::default(), width, height)
}

/// Renders a high-precision semi-circular Speedometer/Gauge Chart with graduated arc, needle, and KPI readout.
pub fn gauge_chart(id: &str, value: f64, min: f64, max: f64, label: &str, unit: &str, width: u32, height: u32) -> DomBuilder {
    let w = width as f64;
    let h = height as f64;
    let cx = w / 2.0;
    let cy = (h * 0.54).round();
    let radius = ((w / 2.0) - 24.0).min(cy - 12.0).max(40.0);

    let span = (max - min).max(0.001);
    let ratio = ((value - min) / span).clamp(0.0, 1.0);

    // Speedometer arch from 180° (-PI) to 0° (0.0) strictly in the upper half (y <= cy)
    let start_angle = -PI;
    let current_angle = start_angle + ratio * PI;

    // Outer Background Track Arc
    let bg_x1 = cx - radius;
    let bg_y1 = cy;
    let bg_x2 = cx + radius;
    let bg_y2 = cy;
    let bg_d = format!("M {} {} A {} {} 0 0 1 {} {}", bg_x1, bg_y1, radius, radius, bg_x2, bg_y2);

    // Value Arc
    let val_x2 = cx + radius * current_angle.cos();
    let val_y2 = cy + radius * current_angle.sin();
    let large_arc = if ratio > 0.5 { "1" } else { "0" };
    let val_d = format!("M {} {} A {} {} 0 {} 1 {} {}", bg_x1, bg_y1, radius, radius, large_arc, val_x2, val_y2);

    let val_color = if ratio < 0.60 {
        "#10b981" // Emerald
    } else if ratio < 0.82 {
        "#f59e0b" // Amber
    } else {
        "#ef4444" // Crimson
    };

    let mut svg = DomBuilder::new_svg("svg")
        .attr("id", id)
        .attr("class", "ferrox-svg-gauge-chart")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    // Background Track Arc
    svg = svg.child(
        DomBuilder::new_svg("path")
            .attr("d", &bg_d)
            .attr("fill", "none")
            .attr("stroke", "rgba(148, 163, 184, 0.2)")
            .attr("stroke-width", "13")
            .attr("stroke-linecap", "round")
    );

    // Value Fill Arc
    if ratio > 0.001 {
        svg = svg.child(
            DomBuilder::new_svg("path")
                .attr("d", &val_d)
                .attr("fill", "none")
                .attr("stroke", val_color)
                .attr("stroke-width", "13")
                .attr("stroke-linecap", "round")
        );
    }

    // Graduated Ticks at 0%, 25%, 50%, 75%, 100%
    for step in [0.0, 0.25, 0.50, 0.75, 1.0] {
        let tick_angle = start_angle + step * PI;
        let r_inner = radius - 14.0;
        let r_outer = radius + 9.0;

        let x_in = cx + r_inner * tick_angle.cos();
        let y_in = cy + r_inner * tick_angle.sin();
        let x_out = cx + r_outer * tick_angle.cos();
        let y_out = cy + r_outer * tick_angle.sin();

        svg = svg.child(
            DomBuilder::new_svg("line")
                .attr("x1", &x_in.to_string())
                .attr("y1", &y_in.to_string())
                .attr("x2", &x_out.to_string())
                .attr("y2", &y_out.to_string())
                .attr("stroke", "rgba(148, 163, 184, 0.35)")
                .attr("stroke-width", "1.5")
        );
    }

    // Needle Indicator (sweeps strictly in upper semicircle y <= cy)
    let needle_len = radius - 6.0;
    let needle_x = cx + needle_len * current_angle.cos();
    let needle_y = cy + needle_len * current_angle.sin();

    svg = svg.child(
        DomBuilder::new_svg("line")
            .attr("x1", &cx.to_string())
            .attr("y1", &cy.to_string())
            .attr("x2", &needle_x.to_string())
            .attr("y2", &needle_y.to_string())
            .attr("stroke", "var(--text-primary, #ffffff)")
            .attr("stroke-width", "2.5")
            .attr("stroke-linecap", "round")
    ).child(
        // Pivot Circle centered at (cx, cy)
        DomBuilder::new_svg("circle")
            .attr("cx", &cx.to_string())
            .attr("cy", &cy.to_string())
            .attr("r", "5.5")
            .attr("fill", val_color)
            .attr("stroke", "var(--bg-surface, #0f172a)")
            .attr("stroke-width", "2")
    );

    // Readout Text placed safely below the pivot (no possible needle collision)
    let readout_str = format!("{:.1} {}", value, unit);
    svg = svg.child(
        DomBuilder::new_svg("text")
            .attr("x", &cx.to_string())
            .attr("y", &(cy + 25.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-primary, #ffffff)")
            .attr("font-size", "18")
            .attr("font-weight", "800")
            .text(&readout_str)
    ).child(
        DomBuilder::new_svg("text")
            .attr("x", &cx.to_string())
            .attr("y", &(cy + 42.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-secondary, #94a3b8)")
            .attr("font-size", "11")
            .attr("font-weight", "600")
            .text(label)
    ).child(
        // Min Label at bottom left of arc
        DomBuilder::new_svg("text")
            .attr("x", &(cx - radius).to_string())
            .attr("y", &(cy + 13.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-secondary, #94a3b8)")
            .attr("font-size", "10")
            .attr("font-weight", "500")
            .text(&format!("{:.0}", min))
    ).child(
        // Max Label at bottom right of arc
        DomBuilder::new_svg("text")
            .attr("x", &(cx + radius).to_string())
            .attr("y", &(cy + 13.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-secondary, #94a3b8)")
            .attr("font-size", "10")
            .attr("font-weight", "500")
            .text(&format!("{:.0}", max))
    );

    div()
        .attr("style", "display: flex; justify-content: center; align-items: center; width: 100%;")
        .child(svg)
}

/// Renders a 2D Heatmap Matrix Chart with color gradient cells for traffic intensity,
/// latency, or commit activity.
pub fn heatmap_chart(
    id: &str,
    matrix: Vec<Vec<f64>>,
    row_labels: Vec<&str>,
    col_labels: Vec<&str>,
    width: u32,
    height: u32,
) -> DomBuilder {
    let pl = 55.0; // Left padding for row labels
    let pt = 30.0; // Top padding for col labels
    let pr = 20.0;
    let pb = 25.0;

    let n_rows = matrix.len();
    let n_cols = if n_rows > 0 { matrix[0].len() } else { 0 };

    let mut min_val = f64::MAX;
    let mut max_val = f64::MIN;

    for row in &matrix {
        for &v in row {
            if v < min_val { min_val = v; }
            if v > max_val { max_val = v; }
        }
    }

    if min_val >= max_val {
        min_val = 0.0;
        max_val = 100.0;
    }
    let span = (max_val - min_val).max(0.001);

    let plot_w = (width as f64 - pl - pr).max(50.0);
    let plot_h = (height as f64 - pt - pb).max(50.0);

    let cell_w = if n_cols > 0 { (plot_w / n_cols as f64) - 2.0 } else { 10.0 };
    let cell_h = if n_rows > 0 { (plot_h / n_rows as f64) - 2.0 } else { 10.0 };

    let mut svg = DomBuilder::new_svg("svg")
        .attr("id", id)
        .attr("class", "ferrox-svg-heatmap-chart")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    // Column Labels
    for (j, label) in col_labels.iter().enumerate() {
        let x = pl + j as f64 * (cell_w + 2.0) + (cell_w / 2.0);
        svg = svg.child(
            DomBuilder::new_svg("text")
                .attr("x", &x.to_string())
                .attr("y", &(pt - 10.0).to_string())
                .attr("text-anchor", "middle")
                .attr("fill", "rgba(255,255,255,0.6)")
                .attr("font-size", "10")
                .text(label)
        );
    }

    // Rows and Cells
    for (i, row) in matrix.iter().enumerate() {
        let y = pt + i as f64 * (cell_h + 2.0);

        // Row Label
        if i < row_labels.len() {
            svg = svg.child(
                DomBuilder::new_svg("text")
                    .attr("x", &(pl - 10.0).to_string())
                    .attr("y", &(y + cell_h / 2.0 + 4.0).to_string())
                    .attr("text-anchor", "end")
                    .attr("fill", "rgba(255,255,255,0.6)")
                    .attr("font-size", "10")
                    .text(row_labels[i])
            );
        }

        for (j, &val) in row.iter().enumerate() {
            let x = pl + j as f64 * (cell_w + 2.0);
            let t = ((val - min_val) / span).clamp(0.0, 1.0);

            // Interpolate color from dark navy -> purple -> cyan/emerald
            let color = if t < 0.25 {
                format!("rgba(30, 58, 138, {:.2})", 0.3 + t * 2.0) // Navy
            } else if t < 0.55 {
                format!("rgba(99, 102, 241, {:.2})", 0.4 + t * 0.8) // Indigo
            } else if t < 0.80 {
                format!("rgba(168, 85, 247, {:.2})", 0.6 + t * 0.4) // Purple
            } else {
                format!("rgba(6, 182, 212, {:.2})", 0.8 + t * 0.2) // Cyan Neon
            };

            let title_text = format!(
                "{}, {}: {:.2}",
                if i < row_labels.len() { row_labels[i] } else { "Row" },
                if j < col_labels.len() { col_labels[j] } else { "Col" },
                val
            );

            let cell = DomBuilder::new_svg("rect")
                .attr("x", &x.to_string())
                .attr("y", &y.to_string())
                .attr("width", &cell_w.to_string())
                .attr("height", &cell_h.to_string())
                .attr("rx", "3")
                .attr("fill", &color)
                .attr("cursor", "pointer")
                .child(DomBuilder::new_svg("title").text(&title_text));

            svg = svg.child(cell);
        }
    }

    div().child(svg)
}

/// Renders an ultra-compact vector Sparkline Chart (Line, Area, or Bar)
/// designed for embedding directly in DataGrid cells and metric cards.
pub fn sparkline_chart(data: Vec<f64>, width: u32, height: u32, spark_type: SparklineType, color: &str) -> DomBuilder {
    let pad = 2.0;
    let w = (width as f64 - pad * 2.0).max(10.0);
    let h = (height as f64 - pad * 2.0).max(10.0);

    let n = data.len();
    if n == 0 {
        return div().child(
            DomBuilder::new_svg("svg")
                .attr("width", &width.to_string())
                .attr("height", &height.to_string())
        );
    }

    let min_val = data.iter().copied().fold(f64::MAX, f64::min);
    let max_val = data.iter().copied().fold(f64::MIN, f64::max);
    let span = (max_val - min_val).max(0.001);

    let scale_x = |i: usize| -> f64 { pad + (i as f64 / (n - 1).max(1) as f64) * w };
    let scale_y = |val: f64| -> f64 { (pad + h) - ((val - min_val) / span) * h };

    let mut svg = DomBuilder::new_svg("svg")
        .attr("class", "ferrox-svg-sparkline")
        .attr("style", "display: inline-block; vertical-align: middle; overflow: visible;")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    match spark_type {
        SparklineType::Line => {
            let mut d = format!("M {} {}", scale_x(0), scale_y(data[0]));
            for i in 1..n {
                d.push_str(&format!(" L {} {}", scale_x(i), scale_y(data[i])));
            }
            svg = svg.child(
                DomBuilder::new_svg("path")
                    .attr("d", &d)
                    .attr("fill", "none")
                    .attr("stroke", color)
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
            );
            // End-point circle
            let last_x = scale_x(n - 1);
            let last_y = scale_y(data[n - 1]);
            svg = svg.child(
                DomBuilder::new_svg("circle")
                    .attr("cx", &last_x.to_string())
                    .attr("cy", &last_y.to_string())
                    .attr("r", "2.5")
                    .attr("fill", color)
            );
        }
        SparklineType::Area => {
            let mut line_d = format!("M {} {}", scale_x(0), scale_y(data[0]));
            for i in 1..n {
                line_d.push_str(&format!(" L {} {}", scale_x(i), scale_y(data[i])));
            }
            let area_d = format!(
                "{} L {} {} L {} {} Z",
                line_d,
                scale_x(n - 1),
                pad + h,
                scale_x(0),
                pad + h
            );
            let grad_id = format!("sparkAreaGrad-{}", (scale_x(0) * 1000.0) as u32);
            svg = svg.child(
                DomBuilder::new_svg("defs").child(
                    DomBuilder::new_svg("linearGradient")
                        .attr("id", &grad_id)
                        .attr("x1", "0%").attr("y1", "0%").attr("x2", "0%").attr("y2", "100%")
                        .child(DomBuilder::new_svg("stop").attr("offset", "0%").attr("stop-color", color).attr("stop-opacity", "0.45"))
                        .child(DomBuilder::new_svg("stop").attr("offset", "100%").attr("stop-color", color).attr("stop-opacity", "0.0"))
                )
            ).child(
                DomBuilder::new_svg("path")
                    .attr("d", &area_d)
                    .attr("fill", &format!("url(#{})", grad_id))
            ).child(
                DomBuilder::new_svg("path")
                    .attr("d", &line_d)
                    .attr("fill", "none")
                    .attr("stroke", color)
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
            );
        }
        SparklineType::Bar => {
            let bar_w = (w / n as f64 - 1.5).max(1.5);
            for (i, &val) in data.iter().enumerate() {
                let x = pad + i as f64 * (w / n as f64);
                let y = scale_y(val);
                let bar_h = ((pad + h) - y).max(1.0);
                svg = svg.child(
                    DomBuilder::new_svg("rect")
                        .attr("x", &x.to_string())
                        .attr("y", &y.to_string())
                        .attr("width", &bar_w.to_string())
                        .attr("height", &bar_h.to_string())
                        .attr("rx", "1")
                        .attr("fill", color)
                );
            }
        }
    }

    div().child(svg)
}