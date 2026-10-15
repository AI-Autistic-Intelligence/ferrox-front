# Ferrox Front Charts (`ferrox-front-charts`)

High-performance, pure WebAssembly vector SVG charting and financial visualization engine.
Zero external JavaScript charting libraries (no D3, no Chart.js, no canvas overhead).

## 1. What It Is & Architectural Purpose
`ferrox-front-charts` generates clean, responsive, resolution-independent SVG elements directly in Rust WebAssembly.
It renders financial trading desk charts and monitoring telemetry at 60 FPS while keeping all data structures strictly inside Wasm linear memory.

---

## 2. Supported Chart Types

### 📈 Financial & Advanced Telemetry Charts
1. `candlestick_chart(id, data, width, height)`:
   - Japanese Candlestick (OHLC: Open, High, Low, Close) chart.
   - Green (bullish) and red (bearish) candle bodies with upper and lower wicks.
   - Subordinate volume histogram aligned below candles.
   - Automatic price and time grid scales.
   - Interactive crosshair with SVG `<title>` tooltip showing exact values.
2. `gauge_chart(id, value, min, max, label, unit, width, height)`:
   - Semi-circular graduated speedometer.
   - Color-coded value arc (Emerald < 60%, Amber 60-82%, Crimson > 82%).
   - Precision needle with center pivot.
   - Large numeric KPI readout with unit and subtitle.
3. `heatmap_chart(id, matrix, row_labels, col_labels, width, height)`:
   - 2D matrix with color-gradient cells for traffic, latency, and activity density.
   - Rounded interactive cells with hover transitions and `<title>` tooltips.
   - Column labels (top) and row labels (left).
4. `sparkline_chart(data, width, height, spark_type, color)`:
   - Ultra-compact vector mini-chart (`SparklineType::Line`, `SparklineType::Area`, `SparklineType::Bar`).
   - Built specifically for embedding directly in DataGrid table cells and KPI cards.

### 📊 Nivo-Style Core Charts
5. `line_chart(id, data, width, height, show_axes)`: Vector line chart with gradient stroke, grid lines, and data point circles.
6. `area_chart(data, width, height)`: Gradient-filled area chart with baseline tapering.
7. `bar_chart(id, data, width, height, show_axes)`: Rounded vertical bar chart with value labels.
8. `pie_chart(data, radius)` / `donut_chart(data, radius, inner_radius, center_title, center_val)`: Circular arc slices with center metric readout.
9. `radar_chart(data, radius)`: Concentric spider-web polygonal radar chart.
10. `scatter_chart(data, width, height)`: Multi-series scatter plot with variable radii.

---

## 3. Practical Usage Examples

### Candlestick Financial Chart
```rust
use ferrox_front_charts::{candlestick_chart, CandleDatum};

let ohlc_data = vec![
    CandleDatum { time: "09:30".into(), open: 92850.0, high: 93400.0, low: 92700.0, close: 93200.0, volume: 198.0 },
    CandleDatum { time: "10:00".into(), open: 93200.0, high: 93350.0, low: 92600.0, close: 92750.0, volume: 165.0 },
    CandleDatum { time: "10:30".into(), open: 92750.0, high: 93600.0, low: 92500.0, close: 93550.0, volume: 275.0 },
];

let widget = candlestick_chart("btc_chart", ohlc_data, 800, 350);
```

### Gauge Telemetry Speedometer
```rust
use ferrox_front_charts::gauge_chart;

let cpu_gauge = gauge_chart("cpu_gauge", 78.4, 0.0, 100.0, "CPU Load (48 Cores)", "%", 280, 160);
```

### Sparklines in DataGrid
```rust
use ferrox_front_charts::{sparkline_chart, SparklineType};

let spark = sparkline_chart(vec![12.0, 24.0, 18.0, 42.0, 68.0, 95.0], 120, 24, SparklineType::Area, "#10b981");
```
