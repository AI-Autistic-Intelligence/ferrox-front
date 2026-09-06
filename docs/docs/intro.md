---
id: intro
title: Introduction to Ferrox Front
sidebar_position: 1
---

# 🛡️ Welcome to Ferrox Front

**Ferrox Front** is a native **Rust** UI framework compiled to **WebAssembly (Wasm)**, designed for building Enterprise SaaS applications, high-performance real-time dashboards, and **Zero-Trust** micro-frontend architectures.

---

## ⚡ Why a "Bootstrap Killer" in Rust Wasm?

In modern JavaScript/TypeScript frontends (React, Vue, Angular), developers face three major challenges:
1. **Virtual DOM Latency & Garbage Collection**: Traditional JS frameworks allocate large trees of JavaScript objects and re-run diffing across component subtrees on every state change, causing latencies and memory leaks on complex dashboards.
2. **XSS Vulnerabilities & Token Theft**: Session tokens (JWTs) stored in `localStorage` or JavaScript scope are exposed to Cross-Site Scripting (XSS) attacks.
3. **Fragile Dependencies**: Heavy Node.js toolchains and hundreds of `npm` packages introduce supply chain security risks.

### **Ferrox Front completely eliminates JavaScript runtime overhead.**

- 🦀 **100% Pure Rust Wasm**: Zero JS dependencies. Frontends are compiled directly to WebAssembly bytecode (`wasm32-unknown-unknown`).
- ⚡ **Signals Reactivity**: Surgical DOM updates at 60fps via fine-grained `Signal<T>` primitives without Virtual DOM overhead.
- 📐 **12-Column Grid System**: Responsive layout components (`Container`, `Row`, `Col`) natively integrated into Rust.
- 🎨 **Glassmorphism Theme Engine**: 5 stunning themes (Cyber, Ocean, Forest, Sunset, Corporate) with instant runtime switching.
- 🔒 **Zero-Trust Memory Security**: Session tokens are cryptographically sealed inside WebAssembly's private linear memory (`SealedToken`), completely unreachable by external scripts.

---

## 🏗️ Crate Architecture

The Ferrox Front ecosystem is modularized into 8 specialized crates:

- **`ferrox-front-core`**: Reactive Signals engine, Wasm DOM builder, and Glassmorphism theme manager.
- **`ferrox-front-macro`**: Procedural compiler for the `rsx!` macro supporting inline HTML-in-Rust component syntax.
- **`ferrox-front-ui`**: 12-column responsive layout grid, virtualized DataGrid (1M rows support), and UI component library.
- **`ferrox-front-security`**: Declarative RBAC `<Secure require="...">`, `SealedToken` memory protection, and WebAuthn Passkeys.
- **`ferrox-front-router`**: Single Page Application (SPA) History API router for seamless client-side navigation.
- **`ferrox-front-charts`**: Pure WebAssembly vector SVG charting engine.
- **`ferrox-front-templates`**: Pre-built enterprise SaaS page templates (Admin Dashboard, Pricing Table, Auth pages).
- **`ferrox-front-ws`**: Async WebAssembly WebSocket client for real-time data streaming.
