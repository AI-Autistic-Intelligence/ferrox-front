# 🛡️ Ferrox Front (`WebAssembly UI Framework`)

<p align="center">
  <b>A Progressive, Enterprise-Grade WebAssembly UI Framework for Rust</b><br/>
  <i>Eliminating JavaScript / TypeScript Virtual DOM Overhead with Signals Reactivity, Zero-Trust Wasm Linear Memory Security, and a 12-Column Responsive Grid System.</i>
</p>

<p align="center">
  <a href="https://crates.io/crates/ferrox-front-core"><img src="https://img.shields.io/crates/v/ferrox-front-core.svg" alt="Crates.io" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-1.76%2B-blue.svg" alt="Rust 1.76+" /></a>
  <a href="https://webassembly.org/"><img src="https://img.shields.io/badge/WebAssembly-Wasm32-orange.svg" alt="WebAssembly" /></a>
  <a href="#-4-zero-trust-webassembly-memory-security"><img src="https://img.shields.io/badge/Security-0--Trust%20SealedToken-green.svg" alt="Security SealedToken" /></a>
  <a href="https://discord.gg/Bx3CzGec7d"><img src="https://img.shields.io/badge/Discord-Join%20Community-purple.svg" alt="Discord Community" /></a>
</p>

<p align="center">
  <a href="#-1-philosophy--architectural-rationale">Philosophy</a> •
  <a href="#-2-architectural-comparison-ferrox-front-vs-react-vs-vue-vs-yew">Comparison Matrix</a> •
  <a href="#-3-exhaustive-crate-workspace-inventory--code-examples">Crate Inventory & Code</a> •
  <a href="#-4-zero-trust-webassembly-memory-security">Zero-Trust Memory</a> •
  <a href="#-5-quickstart--deployment">Quickstart</a>
</p>

---

## ⚡ 1. Philosophy & Architectural Rationale

Traditional modern web frontend development relies heavily on complex JavaScript/TypeScript stacks (React, Vue, Angular) and heavy Virtual DOM reconciliation algorithms. While these frameworks enable rapid UI prototyping, enterprise applications frequently encounter performance bottlenecks:

1. **Virtual DOM Overhead**: Re-evaluating large component render trees on state changes leads to excessive garbage collection pauses and CPU spikes under high-frequency updates (e.g. real-time dashboards or data grids).
2. **XSS & Token Theft Vulnerabilities**: Session tokens stored in JavaScript scope or `localStorage` are vulnerable to Cross-Site Scripting (XSS) attacks.
3. **Fragile Build Toolchains**: Managing Node.js dependencies, bundlers (Webpack, Vite), and transpilers introduces supply chain security risks and fragile builds.

### **Ferrox Front solves these challenges by bringing native Rust performance and WebAssembly memory safety to the browser UI.**

`ferrox-front` provides an out-of-the-box frontend framework engineered specifically for **Enterprise SaaS applications and High-Performance Dashboards**. Written 100% in Rust and compiled directly to WebAssembly (`wasm32-unknown-unknown`), it eliminates JavaScript dependencies while providing a developer experience as clean and expressive as modern JSX.

---

## 📊 2. Architectural Comparison: Ferrox Front vs React vs Vue vs Yew

| Metric / Feature | 🛡️ Ferrox Front (Wasm/Rust) | ⚛️ React (JS/TS) | 💚 Vue (JS/TS) | 🦀 Yew (Rust/Wasm) |
|---|---|---|---|---|
| **Reactivity Model** | **Signals (`Signal<T>`) Direct DOM** | Virtual DOM Reconciliation | Proxy Reactivity VDOM | VDOM Diffing Tree |
| **Garbage Collection Pauses** | **Zero (Wasm Manual Memory)** | V8 Mark-Sweep GC Pauses | V8 Mark-Sweep GC Pauses | Zero (Wasm Memory) |
| **Token Theft XSS Risk** | **Zero (`SealedToken` in Wasm Memory)** | High (`localStorage`/JS Scope) | High (`localStorage`/JS Scope) | Medium (Raw Wasm Memory) |
| **1M Row Virtualized DataGrid** | **Native Wasm (`ferrox-front-ui`)** | Requires Heavy JS Library | Requires Heavy JS Library | Custom Implementation |
| **Passkeys / WebAuthn** | **Native Wasm FIDO2 Integration** | Custom JS Wrappers | Custom JS Wrappers | Custom Wasm Bindings |
| **Built-in 12-Column Grid** | **Native Component Grid** | Requires Bootstrap / Tailwind | Requires Vuetify / Tailwind | Custom CSS Required |
| **Vector Charting Engine** | **Pure Wasm SVG Builder** | Chart.js / Recharts (JS DOM) | Chart.js / ECharts | External JS Interop |

---

## 📦 3. Exhaustive Crate Workspace Inventory & Code Examples

`ferrox-front` is organized into specialized, decoupled crates:

### 1. `ferrox-front-core` (Signals & Glassmorphism Theme Engine)

```rust
use ferrox_front_core::prelude::*;

#[component]
pub fn Counter() -> Element {
    // Fine-grained Signal reactivity - no VDOM re-renders
    let (count, set_count) = create_signal(0);

    rsx! {
        div { class: "glass-card p-4",
            h3 { "Current Value: {count.get()}" }
            button { 
                class: "btn btn-primary", 
                on:click: move |_| set_count.update(|n| *n += 1), 
                "Increment" 
            }
        }
    }
}
```

### 2. `ferrox-front-macro` (`rsx!` Macro Compiler)
Provides the JSX-like `rsx!` macro for compiling HTML templates into efficient WebAssembly DOM instructions.

```rust
use ferrox_front_macro::rsx;

let view = rsx! {
    ul { class: "list-group",
        li { class: "list-item", "Item 1" }
        li { class: "list-item", "Item 2" }
    }
};
```

### 3. `ferrox-front-ui` (12-Column Grid & 1M+ Row Virtualized DataGrid)

```rust
use ferrox_front_ui::prelude::*;

#[component]
pub fn UserGrid() -> Element {
    let users = vec!["Alice", "Bob", "Charlie"];

    rsx! {
        Container { fluid: true,
            Row {
                Col { md: 12,
                    DataGrid { 
                        data: users,
                        columns: vec!["Name"],
                        row_height: 40.0,
                    }
                }
            }
        }
    }
}
```

### 4. `ferrox-front-security` (Zero-Trust RBAC & SealedToken)

```rust
use ferrox_front_security::prelude::*;

#[component]
pub fn AdminPanel() -> Element {
    rsx! {
        Secure { require: "admin",
            fallback: rsx! { div { "Access Denied: Requires Admin Role" } },
            div { class: "admin-dashboard",
                h2 { "Zero-Trust Admin Dashboard" }
            }
        }
    }
}
```

### 5. `ferrox-front-router` (Client-Side History SPA Router)

```rust
use ferrox_front_router::prelude::*;

#[component]
pub fn AppRouter() -> Element {
    rsx! {
        Router {
            Route { path: "/", component: Home }
            Route { path: "/dashboard", component: Dashboard }
            Route { path: "/settings", component: Settings }
        }
    }
}
```

### 6. `ferrox-front-charts` (Pure Wasm SVG Charting Engine)

```rust
use ferrox_front_charts::BarChart;

let chart = BarChart::new()
    .data(vec![("Jan", 100), ("Feb", 250), ("Mar", 400)])
    .dimensions(600, 300)
    .render_svg();
```

### 7. `ferrox-front-templates` & `ferrox-front-ws`
Production-ready SaaS page layouts and asynchronous WebAssembly WebSocket client for real-time bi-directional streaming.

---

## 🔒 4. Zero-Trust WebAssembly Memory Security

Unlike traditional frontend frameworks where sensitive state resides in JavaScript variables or browser `localStorage` exposed to XSS scripts, `ferrox-front-security` stores auth tokens (`SealedToken`) within WebAssembly's private linear memory space.

```
       +---------------------------------------------------------+
       |               Browser JavaScript Runtime                |
       |  (Untrusted XSS Scripts CANNOT read Wasm Linear Memory)  |
       +---------------------------------------------------------+
                                    |
                    XSS Inspection Blocked (Zero Access)
                                    |
                                    v
       +---------------------------------------------------------+
       |           WebAssembly Linear Memory Space               |
       |  +---------------------------------------------------+  |
       |  | SealedToken<T> (AES-GCM-SIV Encrypted Credentials) |  |
       |  +---------------------------------------------------+  |
       +---------------------------------------------------------+
```

If an unauthorized user attempts to inspect the DOM or alter client state, unauthorized component nodes are unmounted dynamically from the Wasm DOM runtime before rendering, preventing DOM manipulation exploits.

---

## 🚀 5. Quickstart & Deployment

### 1. Install Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### 2. Run Admin Showcase
```bash
cd examples/admin-dashboard
trunk serve --open
```

---

## 📜 License

Ferrox Front is dual-licensed under either **MIT License** or **Apache License, Version 2.0**.
