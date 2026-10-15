---
id: overview
title: Zero-JS Vision & Architecture
sidebar_position: 1
---

# Ferrox Front: Enterprise Zero-JS WebAssembly Framework

**Ferrox Front** is a next-generation enterprise frontend framework and Design System written entirely in **Rust WebAssembly (`wasm32-unknown-unknown`)**, specifically engineered to eliminate JavaScript, React, and npm bundlers from mission-critical web applications and high-frequency financial platforms.

---

## 🚀 Why Zero-JS and Rust WebAssembly?

In modern high-performance software (trading desks, analytical dashboards, real-time data streaming), traditional JavaScript/TypeScript architectures encounter fundamental bottlenecks:

1. **Virtual DOM Overhead**: Continuous reconciliation and diffing of virtual tree nodes on every re-render cycle.
2. **Garbage Collection (GC) Stutters**: Unpredictable main-thread stalls for memory reclamation, intolerable when processing high-throughput L2/L3 market tick feeds.
3. **Dependency Fragility**: Thousands of transitive `npm` packages introducing supply-chain security risks and bundling complexity.
4. **Lack of Shared Domain Logic**: Duplicating data models, serialization, and business rules across backend (Rust/C++) and frontend (TypeScript).

Ferrox Front transforms this paradigm: **the entire graphical user interface, fine-grained reactive state, order book matching engine, and charting pipeline execute inside a single optimized, compiled WebAssembly binary**.

---

## ⚡ Performance Benchmark: Ferrox Front vs React 19

| Metric | React 19 + Shadcn / TanStack | Ferrox Front (Rust Wasm) | Ferrox Advantage |
| :--- | :--- | :--- | :--- |
| **Runtime Engine** | JavaScript V8 / Hermes | Native Compiled Wasm32 | **No JIT compilation or interpretation** |
| **Reactivity Model** | Virtual DOM Tree Diffing | **Fine-Grained Signals** (`Signal<T>`) | **O(1) Direct DOM Mutation** |
| **Memory Management** | Continuous Tracing GC | **Deterministic Rust RAII** | **Zero GC Pauses** |
| **DataGrid (100,000 Rows)** | Render lag / main thread freeze | Smooth 60 FPS Virtualization | **Zero memory leaks** |
| **Order Matching Engine** | HTTP Proxy / Slow JS Worker | In-process compiled `ferrox-engine` | **Latency &lt; 1.8 μs** |
| **Technical Charts** | Heavy Canvas / proprietary licensing | **Pure Zero-Copy Vector SVG** | **Zero external JS charting libraries** |
| **NPM Dependencies** | 1,200+ `node_modules` packages | **0 JS packages / 0 npm dependencies** | **Maximum supply chain security** |

---

## 🧩 Modular Core Crates Ecosystem

Adhering strictly to the **DRY (Don't Repeat Yourself)** architectural principle, all computation engines, visual components, and network protocols are decoupled into standalone, reusable crates:

```mermaid
graph TD
    A["ferrox-front-core<br/>(Signals & DOM Builder)"] --> B["ferrox-front-ui<br/>(Design System & DataGrid)"]
    A --> C["ferrox-front-charts<br/>(SVG Technical & Depth Charts)"]
    D["ferrox-match / ferrox-engine<br/>(Pure Rust Order Book)"] --> E["Live Trading Desk<br/>(Wasm Reference App)"]
    F["ferrox-front-ws<br/>(ReconnectingWs + Events)"] --> E
    B --> E
    C --> E
    G["ferrox-front-router<br/>(SPA Routing & Guards)"] --> E
```

### 1. `crates/ferrox-front-core`
- **Fine-Grained Signals**: Reactive primitives `Signal<T>`, `create_signal`, `create_effect`, and `create_memo`.
- **Wasm DOM Builder**: Fluent builder API (`div()`, `span()`, `button()`) for direct DOM manipulation without a Virtual DOM.
- **Dynamic Theming Engine**: Runtime switching between Enterprise Glassmorphism, Cyber Dark, Zinc Light, Cyberpunk, and Ocean Breeze themes.

### 2. `crates/ferrox-front-charts`
- **Vector SVG Engine**: Chart generation without Canvas or external graphics engines.
- **Financial Technical Charts**: Dynamic candlestick timeframes featuring SMA 20/50 overlays, Bollinger Bands (±2σ), RSI (14), and MACD (12, 26, 9) differential histograms.
- **Order Book Depth Chart**: `depth_chart(...)` rendering cumulative curves for Bids/Asks, mid-market price markers, and real-time spread calculations.
- **Streaming Aggregator**: `CandleAggregator` transforming incoming `TradeTick` streams into open and closed interval candles via `CandleRingBuffer`.

### 3. `crates/ferrox-front-ws`
- **`ReconnectingWs`**: Resilient browser WebSocket client featuring exponential backoff and randomized jitter (±20%).
- **Offline Queue**: Message dispatch buffering preventing payload loss during transient disconnections.
- **Typed Event Dispatcher**: Strongly-typed payload deserialization for `Trades`, `OrderBookDepth`, and `Ticker` events.

### 4. `ferrox-match/packages/ferrox-engine`
- **Pure Matching Engine**: In-memory `OrderBook` utilizing fixed-point `BTreeMap` precision (`1e8`).
- **Multi-Target Compilation**: Compiles both for Linux/Windows servers and directly as a pure Rust dependency on `wasm32-unknown-unknown` with zero overhead.

---

## 🛠️ Hello World with Signals and DOM Builder

```rust
use ferrox_front_core::dom::{mount, div, h1, button};
use ferrox_front_core::reactivity::create_signal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Fine-grained reactive state
    let count = create_signal(0);

    // Fluent direct DOM construction without Virtual DOM
    let root = div()
        .class("flex flex-col items-center justify-center p-8 bg-slate-900 rounded-2xl border border-slate-800")
        .child(
            h1()
                .class("text-2xl font-bold text-white mb-4")
                .text(&format!("Count: {}", count.get()))
        )
        .child(
            button()
                .class("px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg font-semibold")
                .text("Increment")
                .on_click(move || {
                    count.update(|n| *n += 1);
                })
        );

    mount("#root", root);
    Ok(())
}
```

The subsequent sections explore the **UI Components**, the **Enterprise DataGrid**, the **Vector Financial Charting Engine**, and the **Interactive Live Trading Desk** running directly in the browser.
