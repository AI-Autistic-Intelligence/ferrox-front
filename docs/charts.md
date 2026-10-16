---
id: charts
title: SVG Vector Financial Charting Engine
sidebar_position: 5
---

# Financial Charting Engine (`ferrox-front-charts`)

`ferrox-front-charts` is an ultra-high-precision SVG vector graphics library engineered specifically for financial dashboards, trading desks, and quantitative telemetry, featuring **zero JavaScript or Canvas dependencies**.

---

## 📈 Pro Candlestick with Technical Indicators (`financial_technical_chart`)

The primary technical chart provides functional parity with industry benchmarks such as TradingView Lightweight Charts, integrating native Rust mathematical algorithms for essential quantitative indicators:

```rust
use ferrox_front_charts::{
    CandleDatum, TechnicalChartOptions, financial_technical_chart,
};

let candle_history: Vec<CandleDatum> = vec![
    CandleDatum { time: "18:00".into(), open: 96250.0, high: 96800.0, low: 96100.0, close: 96600.0, volume: 510.0 },
    CandleDatum { time: "18:30".into(), open: 96600.0, high: 97200.0, low: 96400.0, close: 97100.0, volume: 640.0 },
];

let mut options = TechnicalChartOptions::default();
options.show_sma20 = true;       // 20-period Simple Moving Average (amber overlay)
options.show_sma50 = true;       // 50-period Simple Moving Average (cyan overlay)
options.show_bollinger = true;    // Bollinger Bands (±2 standard deviations)
options.show_rsi = true;          // 14-period RSI sub-panel with 70/30 threshold markers
options.show_macd = true;         // MACD (12, 26, 9) sub-panel with dual-color differential histogram

let chart_svg = financial_technical_chart(
    "btc-pro-chart",
    &candle_history,
    &options,
    880, // Width in pixels
    480, // Height in pixels
);
```

### Integrated Mathematical Indicators
1. **SMA (Simple Moving Average)**: Efficient O(1) rolling-window calculation for 20 and 50 period overlays.
2. **Bollinger Bands**: Dynamic volatility channel computed via empirical standard deviation in native 64-bit floating-point Rust.
3. **RSI (Relative Strength Index)**: 14-period momentum oscillator with Wilder smoothing and clear overbought (&gt;70) and oversold (&lt;30) zones.
4. **MACD (Moving Average Convergence Divergence)**: Exponential Moving Averages (EMA 12 and EMA 26), signal line (EMA 9), and dynamic dual-color (green/red) differential histogram bars.

---

## 🌊 Order Book Market Depth Chart (`depth_chart`)

The market depth chart (`depth_chart`) visualizes live order book liquidity:

```rust
use ferrox_front_charts::depth_chart;

// Cumulative liquidity levels (price, aggregated volume)
let bids = vec![(97040.0, 1.25), (97020.0, 4.65), (97000.0, 11.45), (96950.0, 26.65)];
let asks = vec![(97060.0, 0.95), (97080.0, 3.75), (97100.0, 9.65), (97150.0, 23.75)];

let depth_svg = depth_chart(
    "btc-depth-chart",
    &bids,
    &asks,
    560, // Width
    280, // Height
);
```

### Depth Chart Components
- **Emerald Green Area (Bids)**: Cumulative buyer demand plotted across descending price levels.
- **Rose Red Area (Asks)**: Cumulative seller supply plotted across ascending price levels.
- **Mid-Market Price Line**: Vertical dashed indicator demarcating the instantaneous fair market equilibrium.
- **Spread Badge**: Centered badge detailing absolute bid-ask spread and relative basis points (e.g., `Spread: $10.00 (0.01%)`).

---

## ⏱️ Trade Tick Streaming & Candle Aggregator (`CandleAggregator`)

In real-time trading environments, WebSocket feeds push discrete, atomic trade executions. `CandleAggregator` dynamically compiles trade ticks into the current open candle interval, finalizing candles automatically when the interval elapses:

```rust
use ferrox_front_charts::{CandleAggregator, TradeTick, Timeframe};

// Initialize aggregator for 5-minute interval with a 100-candle circular ring buffer
let mut aggregator = CandleAggregator::new(Timeframe::Min5, 100);

// Ingest incoming trade ticks from WebSocket or ferrox-engine
aggregator.process_tick(TradeTick::new(97050.0, 0.45, 1760518800000));
aggregator.process_tick(TradeTick::new(97055.0, 1.20, 1760518815000));

// Retrieve finalized historical candles combined with the current active open candle
let current_candles = aggregator.get_candles_with_active();
```

### Supported Timeframe Resolutions

| Timeframe | Enum Constant | Duration (ms) | Target Use Case |
| :--- | :--- | :--- | :--- |
| **1 Second** | `Timeframe::Sec1` | 1,000 ms | Ultra-low-latency HFT / Scalping |
| **5 Seconds** | `Timeframe::Sec5` | 5,000 ms | Order book microstructure monitoring |
| **1 Minute** | `Timeframe::Min1` | 60,000 ms | Short-term intraday momentum |
| **5 Minutes** | `Timeframe::Min5` | 300,000 ms | Standard Trading Desk baseline |
| **15 Minutes** | `Timeframe::Min15` | 900,000 ms | Mid-term trend following |
| **1 Hour** | `Timeframe::Hour1` | 3,600,000 ms | Multi-session cyclical analysis |
| **1 Day** | `Timeframe::Day1` | 86,400,000 ms | Macro trend overview |

---

## 🎯 Telemetry Indicators: Gauges & Sparklines

Ferrox Front also delivers lightweight, embeddable vector telemetry widgets:

```rust
use ferrox_front_charts::{gauge_chart, sparkline_chart, SparklineType};

// Semi-circular speedometer gauge for Fear & Greed index or CPU load
let cpu_gauge = gauge_chart("cpu-usage", 78.5, 0.0, 100.0, "CPU Load", "%", 220, 140);

// Minimalist vector sparkline for compact table trend visualization
let price_sparkline = sparkline_chart(
    vec![96100.0, 96400.0, 96250.0, 96900.0, 97050.0],
    120,
    32,
    SparklineType::Line,
    "#10b981",
);
```
