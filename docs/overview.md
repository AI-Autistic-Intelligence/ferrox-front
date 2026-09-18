---
id: overview
title: Overview
sidebar_position: 1
---

# Ferrox Front Overview

**Ferrox Front** is a high-performance, WebAssembly (WASM) based frontend framework written in Rust, leveraging the Yew framework under the hood. It provides a robust, type-safe environment for building complex User Interfaces while maintaining near-native performance.

## Why Rust on the Frontend?

While JavaScript/TypeScript ecosystems are mature, Rust via WebAssembly offers several distinct advantages for heavy, data-intensive web applications:

1. **Unparalleled Performance**: WASM execution speeds are significantly faster than JS parsing and JIT compilation, especially for complex state calculations or data visualizations (like trading charts).
2. **Type Safety & Correctness**: Rust's ownership model and strict type system eliminate entire classes of bugs (like `undefined is not a function`) at compile time.
3. **Shared Business Logic**: By using Rust on both the frontend (Ferrox Front) and backend (Ferrox Core), you can share models, validation rules, and business logic seamlessly without duplicating code in different languages.

## Architecture

Ferrox Front is structured around the Component architecture provided by Yew, but extended with Ferrox-specific patterns for state management and API communication.

### Key Concepts

- **Components**: UI building blocks defined using the `html!` macro (similar to JSX).
- **State Management**: A robust, context-based state management system designed to handle high-frequency updates from WebSockets.
- **WASM Interop**: Seamless bindings to browser APIs (DOM, WebGL) via `web-sys` and `js-sys`.
- **API Client**: Auto-generated clients that communicate directly with Ferrox Core backends using optimized binary protocols or standard JSON/REST.

## Quick Example

A simple Ferrox Front component looks like this:

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(MyComponent)]
pub fn my_component(props: &Props) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="ferrox-panel">
            <h1>{ &props.title }</h1>
            <p>{ "Counter: " }{ *counter }</p>
            <button {onclick}>{ "+1" }</button>
        </div>
    }
}
```

Explore the documentation to learn more about advanced state management, real-time WebSocket integration, and routing.
