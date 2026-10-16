---
id: ws
title: Real-Time WebSockets Engine (ferrox-front-ws)
sidebar_position: 10
---

# 🌐 Real-Time WebSockets Engine (`ferrox-front-ws`)

`ferrox-front-ws` provides an asynchronous WebAssembly WebSocket client engine built natively for Rust frontends. It bridges browser WebSocket APIs (`web_sys::WebSocket`) with Ferrox Front's reactive **Signals** graph (`Signal<T>`), enabling low-latency, real-time bi-directional streaming for dashboards, chat applications, financial tickers, and live notification systems.

---

## ⚡ 1. Overview & Architectural Role

In enterprise server-side architectures, real-time data communication is critical. Traditional HTTP short-polling or long-polling incurs massive overhead due to repeated HTTP header exchanges and connection establishment latencies.

### Why WebSockets in WebAssembly?
- 📡 **Low Latency & Persistent Connection**: Establishes a single full-duplex TCP stream between the Wasm client and the Ferrox backend (`ferrox-transports` / Axum WebSockets).
- ⚡ **Zero-Copy Memory Parsing**: Incoming WebSocket binary or text frames are processed directly inside WebAssembly linear memory without passing through heavy JavaScript parser layers.
- 🔄 **Reactive Signal Integration**: Incoming messages automatically update `Signal<T>` reactive primitives, triggering surgical DOM updates at 60fps across subscribed UI components.

```
       +-------------------------------------------------------------+
       |             Ferrox Backend / WebSocket Server               |
       +-------------------------------------------------------------+
                                     ^
                                     |  Full-Duplex WS Stream (JSON / Binary)
                                     v
       +-------------------------------------------------------------+
       |               Wasm Client (ferrox-front-ws)                 |
       |  - web_sys::WebSocket Binding                               |
       |  - Closure Callback Handlers (onmessage, onerror, onopen)   |
       +-------------------------------------------------------------+
                                     |
                                     v
       +-------------------------------------------------------------+
       |               Ferrox Signals Reactivity Graph               |
       |  - Signal<String> Status / Message Buffers                  |
       +-------------------------------------------------------------+
                                     |
                                     v
       +-------------------------------------------------------------+
       |               Surgical Wasm DOM Element Mutation            |
       +-------------------------------------------------------------+
```

---

## 🛠️ 2. Core Concepts & API Reference

### `FerroxSocket` Struct

The primary struct managing a WebSocket connection lifecycle:

```rust
pub struct FerroxSocket {
    pub url: String,
    pub status: Signal<String>,
}
```

| Field / Method | Type | Description |
|---|---|---|
| `url` | `String` | Target WebSocket endpoint URL (e.g. `wss://api.example.com/ws`). |
| `status` | `Signal<String>` | Reactive status signal tracking connection state (`"Connecting..."`, `"Connected to Ferrox WS"`, `"Error"`). |
| `FerroxSocket::new(url)` | `fn(&str) -> Result<FerroxSocket, JsValue>` | Constructor that instantiates `web_sys::WebSocket`, binds lifecycle closures, and begins connection handshake. |

---

## 🚀 3. Step-by-Step Implementation Guide

### Step 1: Add Dependency

In your project's `Cargo.toml`:

```toml
[dependencies]
ferrox-front-ws = { path = "../crates/ferrox-front-ws" }
ferrox-front-core = { path = "../crates/ferrox-front-core" }
wasm-bindgen = "0.2"
```

### Step 2: Instantiating `FerroxSocket` in a Component

```rust
use wasm_bindgen::prelude::*;
use ferrox_front_ws::FerroxSocket;
use ferrox_front_core::dom::{div, p, DomBuilder};

pub fn RealtimeDashboard() -> DomBuilder {
    // 1. Establish WebSocket Connection
    let ws_url = "wss://api.ferrox-rust.dev/ws/metrics";
    let socket = FerroxSocket::new(ws_url).expect("Failed to initialize WebSocket client");

    // 2. Render UI bound to the socket status signal
    div()
        .attr("class", "ws-card")
        .child(
            p().text(&format!("WebSocket Endpoint: {}", socket.url))
        )
        .child(
            p().attr("class", "ws-status")
                .text(&format!("Connection Status: {}", socket.status.get()))
        )
}
```

---

## 🔄 4. Internal Callback Lifecycle & Closure Management

To prevent memory leaks inside WebAssembly while maintaining event-driven callbacks, `ferrox-front-ws` wraps browser handlers using `wasm_bindgen::closure::Closure`.

### 1. `onopen` Handler
Fired when the WebSocket handshake succeeds:
```rust
let onopen_callback = Closure::<dyn FnMut()>::new(move || {
    web_sys::console::log_1(&"Connected to Ferrox WS".into());
});
ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
onopen_callback.forget(); // Keeps closure alive in Wasm memory heap
```

### 2. `onmessage` Handler
Fired when a new data frame arrives from the server:
```rust
let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
        web_sys::console::log_1(&txt);
    }
});
ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
onmessage_callback.forget();
```

### 3. `onerror` Handler
Fired on network failure or unexpected disconnection:
```rust
let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
    web_sys::console::log_1(&"WebSocket Connection Error".into());
});
ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
onerror_callback.forget();
```

---

## 🛡️ 5. Production Best Practices & Security

1. **WSS (WebSocket Secure)**: Always use `wss://` in production to enforce TLS encryption and prevent middleman payload inspection.
2. **Reconnection Strategy**: Pair `FerroxSocket` with a retry timer to handle transient network drops automatically.
3. **Authentication Handshake**: Send an initial zero-trust `SealedToken` in the query parameter or connection payload frame to validate authorization before opening server streams.
