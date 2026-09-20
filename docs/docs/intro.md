---
id: intro
title: Introduction & Ferrox-Front Architecture
sidebar_position: 1
---

# Introduction & Ferrox-Front Architecture

Welcome to **Ferrox-Front**, the high-performance WebAssembly (Wasm) frontend framework written in pure Rust. It combines fine-grained reactive signals, compile-time JSX HTML templates, zero-trust cryptographic security, real-time WebSocket state synchronization, and GPU-accelerated WebGL charts into a unified application suite.

---

## 1. What It Is & Architectural Purpose
Modern web applications require rendering complex, real-time user interfaces with high responsiveness. However, JavaScript frontend frameworks (React, Vue, Angular) struggle with heavy client-side computations (real-time telemetry charts, client-side encryption, large data grid rendering) due to single-threaded event-loop bottlenecks and garbage collection pauses.

**Ferrox-Front** leverages Rust and WebAssembly to eliminate runtime garbage collection entirely. It delivers native 60 FPS rendering performance, compile-time type safety, and fine-grained reactive state updates without Virtual DOM overhead.

## 2. Architectural Layering
| Crate | Category & Purpose | Key Capabilities |
| :--- | :--- | :--- |
| **`ferrox-front-core`** | Framework Foundation | Signal runtime, effect scheduler, resource context primitives. |
| **`ferrox-front-templates`**| HTML / JSX Engine | Compile-time `view!` JSX macro, static node cloning, SSR. |
| **`ferrox-front-macro`** | Procedural Macros | Component procedural macros (`#[component]`), memo derives. |
| **`ferrox-front-router`**| Client Navigation | Declarative Wasm client-side routing & route guard matching. |
| **`ferrox-front-ui`** | Component Library | Accessible UI primitives (buttons, modals, data tables). |
| **`ferrox-front-charts`**| Data Visualization | GPU-accelerated WebGL charts (candlestick, line, bar). |
| **`ferrox-front-ws`** | Real-Time Transport | Reconnecting WebSocket manager with binary MsgPack support. |
| **`ferrox-front-security`**| Zero-Trust Wasm | WebCrypto API integration, AES-256-GCM, XSS sanitizers. |

## 3. How It Works Under the Hood
Unlike VDOM frameworks that diff full object trees on state mutations, Ferrox-Front binds signals directly to individual DOM node pointers. The execution flows seamlessly from the browser event loop to reactive signal state, which then notifies the effect scheduler. The scheduler batches pending updates and executes direct DOM node mutations via WebAssembly and `web_sys`.

## 4. Why It Was Designed This Way
Ferrox-Front was designed to enforce native memory performance and end-to-end type integrity. Rust's affine type system and ownership model guarantee memory safety and zero garbage collection pauses during intensive rendering. Furthermore, it allows sharing identical Rust data models (structs, enums, validation rules) across your backend Ferrox microservices and WebAssembly frontend app.

## 5. Practical Usage Guide & Extended Code Examples
To start using Ferrox-Front, proceed to the quickstart guide. A typical application structure is heavily reliant on the macros and core components:
```rust
use ferrox_front_core::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div { "Welcome to Ferrox Front!" }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Mixing JavaScript Frameworks**
> Do not attempt to run Ferrox-Front components inside an existing heavy JavaScript framework (like React or Angular) container. Wasm memory spaces and JS garbage collection loops conflict severely and negate the performance benefits of Rust.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Embrace Rust's Type System**
> Share your `types` crate between your backend Ferrox API and your frontend Wasm application. This guarantees that API contracts are strictly enforced at compile time, eliminating runtime undefined errors.
