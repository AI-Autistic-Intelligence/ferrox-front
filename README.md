<div align="center">
  <h1>🛡️ Ferrox Front</h1>
  <p><strong>A Progressive, Enterprise-Grade WebAssembly UI Framework for Rust</strong></p>
  <p><i>Eliminating JavaScript/TypeScript overhead with Signals Reactivity, Zero-Trust Memory Security, and a 12-Column Responsive Grid.</i></p>
  
  [![Crates.io](https://img.shields.io/crates/v/ferrox-front-core.svg)](https://crates.io/crates/ferrox-front-core)
  [![Rust](https://img.shields.io/badge/rust-1.76%2B-blue.svg)](https://www.rust-lang.org)
  [![WebAssembly](https://img.shields.io/badge/WebAssembly-Enabled-orange.svg)](https://webassembly.org/)
  [![Security](https://img.shields.io/badge/Security-0--Trust%20RBAC-green.svg)]()
</div>

---

## ⚡ Philosophy & Architectural Rationale

Traditional modern web frontend development relies heavily on complex JavaScript/TypeScript stacks (React, Vue, Angular) and heavy Virtual DOM reconciliation algorithms. While these frameworks enable rapid UI prototyping, enterprise applications frequently encounter performance bottlenecks:
1. **Virtual DOM Overhead**: Re-evaluating large component render trees on state changes leads to excessive garbage collection pauses and CPU spikes under high-frequency updates (e.g. real-time dashboards or data grids).
2. **XSS & Token Theft Vulnerabilities**: Session tokens stored in JavaScript scope or `localStorage` are vulnerable to Cross-Site Scripting (XSS) attacks.
3. **Fragile Build Toolchains**: Managing Node.js dependencies, bundlers (Webpack, Vite), and transpilers introduces supply chain security risks and fragile builds.

### **Ferrox Front solves these challenges by bringing native Rust performance and WebAssembly memory safety to the browser UI.**

`ferrox-front` provides an out-of-the-box frontend framework engineered specifically for **Enterprise SaaS applications and High-Performance Dashboards**. Written 100% in Rust and compiled directly to WebAssembly (`wasm32-unknown-unknown`), it eliminates JavaScript dependencies while providing a developer experience as clean and expressive as modern JSX.

### 🔑 Key Framework Highlights
- 🦀 **Signals Reactivity & Wasm Virtual DOM**: Surgical DOM updates powered by fine-grained `Signal<T>` primitives without the memory overhead of traditional Virtual DOM trees.
- 📐 **Native 12-Column Grid System**: Built-in responsive grid system (`Container`, `Row`, `Col`) integrated into Rust components. No CSS framework setup required.
- 🎨 **Glassmorphism Theme Engine**: 5 dynamic themes (Cyber, Ocean, Forest, Sunset, Corporate) with instant 60fps runtime theme switching.
- 🔒 **Zero-Trust Memory Security**: Cryptographically seals session tokens in WebAssembly linear memory (`SealedToken`), completely isolating credentials from JS XSS exploits.
- 📊 **Virtualized DataGrid & Wasm Charts**: Smoothly renders 1,000,000+ data rows with viewport virtualization and vector SVG charts.
- 🔑 **WebAuthn Passkeys**: Native WebAssembly FIDO2 biometrics integration.

---

## 🛠️ Crate Workspace Inventory

`ferrox-front` is organized into modular, decoupled crates:

| Crate Name | Description | README |
|---|---|---|
| `ferrox-front-core` | Reactive Signals engine (`Signal<T>`), Wasm DOM builder, and runtime Glassmorphism theming | [Read README](crates/ferrox-front-core/README.md) |
| `ferrox-front-macro` | Procedural macro compiler (`rsx!`) for inline JSX-style HTML-in-Rust component syntax | [Read README](crates/ferrox-front-macro/README.md) |
| `ferrox-front-ui` | 12-column responsive Grid system, virtualized DataGrid (1M rows), and UI controls | [Read README](crates/ferrox-front-ui/README.md) |
| `ferrox-front-security` | Zero-Trust RBAC guards (`<Secure require="admin">`), `SealedToken` protection, and WebAuthn Passkeys | [Read README](crates/ferrox-front-security/README.md) |
| `ferrox-front-router` | Client-side History API SPA router with route params and navigation guards | [Read README](crates/ferrox-front-router/README.md) |
| `ferrox-front-charts` | Pure WebAssembly vector SVG charting engine (Bar, Line, Pie graphs) | [Read README](crates/ferrox-front-charts/README.md) |
| `ferrox-front-templates` | Production-ready enterprise SaaS page templates (Auth, Pricing, Admin Dashboard) | [Read README](crates/ferrox-front-templates/README.md) |
| `ferrox-front-ws` | Async WebAssembly WebSocket client for real-time bi-directional data streaming | [Read README](crates/ferrox-front-ws/README.md) |
| `admin-dashboard` | Full SaaS admin dashboard showcase application | [Read README](examples/admin-dashboard/README.md) |

---

## 🚀 Quickstart

### 1. Install Prerequisites
Add the WebAssembly compilation target and install the `trunk` Wasm bundler:
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### 2. Run the SaaS Admin Showcase App
Launch the pre-built admin dashboard showcase locally:
```bash
cd examples/admin-dashboard
trunk serve --open
```

### 3. Example Component Code

```rust
use ferrox_front_core::prelude::*;
use ferrox_front_ui::prelude::*;
use ferrox_front_security::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let (metrics, set_metrics) = create_signal(1420);

    rsx! {
        Container { fluid: true,
            Row {
                Col { md: 4,
                    Card { title: "Total Conversions",
                        h2 { class: "text-primary", "{metrics.get()}" }
                    }
                }
                Col { md: 8,
                    Secure { require: "admin",
                        div { class: "alert alert-success",
                            "Welcome Admin! Accessing Zero-Trust Wasm linear memory."
                        }
                    }
                }
            }
        }
    }
}
```

---

## 🔒 Focus: Zero-Trust WebAssembly Security

Unlike traditional frontend frameworks where sensitive state resides in JavaScript variables or browser storage exposed to XSS scripts, `ferrox-front-security` stores auth tokens (`SealedToken`) within WebAssembly's private linear memory space.

If an unauthorized user attempts to inspect the DOM or alter client state, unauthorized component nodes are unmounted dynamically from the Wasm DOM runtime before rendering, preventing DOM manipulation exploits.

---

## 💬 Community & Support

Join the official global **Ferrox Community**:
- 💬 **Discord Server:** [https://discord.gg/Bx3CzGec7d](https://discord.gg/Bx3CzGec7d)
- 🤖 **Reddit Subreddit:** [r/Ferrox](https://www.reddit.com/r/Ferrox/)

---

## 📜 License

Ferrox Front is dual-licensed under either **MIT License** or **Apache License, Version 2.0**.
