# Changelog - Ferrox Ecosystem

All notable changes to the Ferrox ecosystem are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [2.0.0] - 2026-10-15

### ⚡ 1. Live Trading Desk & Matching Engine Decoupling (DRY Core)
- **`ferrox-match / ferrox-engine`**:
  - Extracted Order Book core logic into a pure, N-API-independent Rust crate exported as a universal library (`rlib` and `cdylib`).
  - Multi-target compilation support: native build for Linux/x86_64 servers and `wasm32-unknown-unknown` target for client-side zero-latency simulations (`< 1.8 μs`).
  - Standardized core API: `OrderBook::new()`, `submit_order()`, `get_depth()`, `cancel_order()`, `best_bid()`, `best_ask()`, `mid_price()`, `spread()`.
  - Agnostic time handling: `js_sys::Date::now()` on WebAssembly, `SystemTime` on native runtime.
  - Comprehensive unit test suite verifying FIFO matching, partial executions, and order cancellation.
- **`ferrox-front-charts`**:
  - Implemented `depth_chart(...)`: pure SVG vector rendering of market depth (Order Book Depth) with cumulative Bids (emerald) and Asks (rose) curves, central mid-market price line, and calculated spread pill with percentage.
  - Implemented `CandleAggregator`: reactive trade aggregator (`TradeTick { price, volume, timestamp }`) updating the current open candle across intervals from 1 second to 1 day, flushing continuously into `CandleRingBuffer`.
- **`ferrox-front-ws`**:
  - Re-engineered the WebAssembly `ReconnectingWs` client with an explicit `WsStatus` lifecycle state machine.
  - Exponential backoff (`delay * 1.5`, 30s cap) combined with **randomized jitter** (±20%) for Thundering Herd mitigation.
  - In-memory offline queue (`message_queue`) for transparent buffering of messages dispatched during transient disconnects.
  - Strongly-typed event dispatcher for `Trades`, `OrderBookDepth`, and `Ticker` channels.
- **`examples/admin-dashboard`**:
  - Unified "Live Trading Desk Pro" tab composed exclusively of abstract ecosystem libraries: Candlestick chart with SMA 20/50, Bollinger Bands, RSI, and MACD; SVG Order Book Depth; Order Entry form validated with `form_engine`; Level 2 Order Book Ladder; and real-time recent trades feed.

---

### 📚 2. Interactive Wasm Documentation & Docusaurus Playground
- **Wasm Compilation & Distribution**:
  - Trunk asset pipeline compiling the showcase application directly into `ferrox-docs/static/wasm/admin-dashboard/`.
- **Playground React/MDX Component**:
  - Implemented `FerroxWasmPlayground.tsx` isolated for Docusaurus SSR (`BrowserOnly`), featuring tab navigation (`#trading`, `#datagrid`, `#form`, `#explorer`), instance reload action, fullscreen mode (`requestFullscreen`), live connection indicator, and telemetry badge.
- **In-Depth Documentation Pages**:
  - `docs/ferrox-front/overview.md`: Zero-JS vision, Signals reactivity (`Signal<T>`), detailed performance comparison against React 19 / Virtual DOM.
  - `docs/ferrox-front/trading-desk.md`: Interactive page featuring the embedded `<FerroxWasmPlayground />` component for in-browser execution.
  - `docs/ferrox-front/components.md`: Catalog of advanced buttons, form controls, switch toggles, modals, and 60+ pure Rust vector icons.
  - `docs/ferrox-front/datagrid.md`: Guide to the Enterprise 1M-Row DataGrid with multi-column sorting, column filters, and RFC 4180 CSV export.
  - `docs/ferrox-front/charts.md`: Documentation for the financial SVG vector engine (Candlestick, Depth Chart, technical indicators, and tick aggregation).

---

### 🚀 3. 1 Million Requests / Second Benchmark (Official Whitepaper)
- **Whitepaper Publication `1m-rps.md`**:
  - Official release of empirical performance data from `1MilionRequestPerSecond` benchmarked against real enterprise workloads (8.0 vCPU limit, PostgreSQL with 100,000 records, distributed Redis rate limiting).
  - Integrated comparative SVG vector charts:
    1. **Throughput (Requests/sec)**: Node.js (14,781 RPS) and Rust (13,469 RPS) leading, followed by PHP (10,627 RPS), Python (7,120 RPS), and Java (3,639 RPS).
    2. **p99 Latency (Consistency)**: Rust delivers **333 ms** (more than 2x more consistent than Node at 677 ms and Python at 815 ms), proving the advantage of zero Garbage Collection pauses.
    3. **RAM Consumption**: Rust consumes just **38 MB**, compared to 320 MB for Node and 890 MB for the Java JVM.
  - Complete local reproduction instructions using Docker Compose.
  - Dedicated navigation item in the `ferrox-docs` top navbar and sidebar.

---

### 📦 4. Release Validation, Packaging & CI/CD Pipeline
- **Crate Audit & Metadata**:
  - Validated `Cargo.toml` across all crates in `ferrox` and `ferrox-front`: complete metadata (description, repository, license, documentation, homepage, readme, keywords, categories).
  - Removed local-only path dependencies without version constraints (`path = ...` with associated `version = "..."`).
  - Topological publishing scripts `publish_crates.js` created and validated for `ferrox` and `ferrox-front`.
- **GitHub Actions Workflow (`release-pipeline.yml`)**:
  - Cross-platform build matrix (`ubuntu-latest`, `windows-latest`, `macos-latest`).
  - Automated WebAssembly compilation and optimization (`wasm-opt -Oz`).
  - Multi-architecture container builds for Docker images (`linux/amd64`, `linux/arm64`).
  - Automated sequential publication to crates.io upon `v*` tag release.

---

### 🛡️ 5. Security Elevation & Hardening (OWASP WSTG)
- **Security Sidecar Integration**:
  - `ferrox-sentinel` and `ferrox-pentest-private` for continuous vulnerability scanning (SQLi, NoSQLi, JWT/PASETO token manipulation, SSRF, XSS).
  - Watertight `SealedToken` in WebAssembly memory encrypted via WebCrypto API.
  - Hardened Docker containers with non-root users, read-only root filesystems, and strict cgroup quotas.
