# Ferrox Front WS (`ferrox-front-ws`)

`ferrox-front-ws` provides an asynchronous WebAssembly WebSocket client for real-time streaming.

## 1. What It Is & Architectural Purpose
The `ferrox-front-ws` crate is the dedicated networking and transport layer for real-time Ferrox Front applications. It provides a robust, auto-reconnecting WebSocket client wrapper designed to run efficiently inside the browser. Its primary architectural purpose is to manage bi-directional streaming of binary data (e.g., MsgPack, Protobuf) between the Wasm frontend and Rust backend servers, bypassing the overhead of standard HTTP polling or JSON serialization.

## 2. Architectural Layering
This crate acts as the networking abstraction layer. It interfaces directly with the browser's `WebSocket` API via `web_sys` and exposes reactive streams to the `ferrox-front-core` layer. When messages are received over the socket, the transport layer decodes them and updates `Signal` values, which subsequently triggers the reactivity graph to patch the DOM without requiring manual DOM manipulation.

## 3. How It Works Under the Hood
The `WebSocketClient` is initialized with a URL and spawns a background asynchronous task using `wasm_bindgen_futures::spawn_local`. It registers event callbacks (onopen, onmessage, onerror, onclose) via JavaScript closures. Received `ArrayBuffer` payloads are parsed directly in Wasm linear memory. To prevent UI freezing, high-frequency inbound messages (e.g., live telemetry) can be buffered or throttled within this layer before being pushed to the reactive signals.

## 4. Why It Was Designed This Way
High-performance dashboards (like gaming server telemetry or trading charts) require constant, low-latency updates. Standard JSON over HTTP polling introduces unacceptable latency, header overhead, and parsing delays. By utilizing WebSockets with binary serialization formats natively in Rust, `ferrox-front-ws` minimizes serialization/deserialization CPU costs and drastically reduces bandwidth, keeping the application fast and responsive.

## 5. Practical Usage Guide & Extended Code Examples
Connecting to a server and binding incoming data to a signal.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_ws::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct ServerStats { cpu: f32, ram: f32 }

#[component]
pub fn LiveStats() -> Element {
    let (stats, set_stats) = create_signal(ServerStats { cpu: 0.0, ram: 0.0 });
    
    // Connect and subscribe to binary messages
    use_websocket("ws://localhost:8080/stream", move |msg_bytes| {
        if let Ok(new_stats) = rmp_serde::from_slice::<ServerStats>(&msg_bytes) {
            set_stats.set(new_stats);
        }
    });

    rsx! {
        div { class: "stats-panel",
            p { "CPU: {stats.get().cpu}%" }
            p { "RAM: {stats.get().ram}%" }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Triggering Heavy Renders on Every Message**
> If your WebSocket receives 100 messages per second, do not bind the raw socket event directly to a signal that triggers a massive DOM reconciliation (like re-rendering a 10,000 row table). This will overwhelm the browser's rendering engine. Instead, use a throttle or debounce mechanism in the WebSocket callback to update the signal only at visually perceptible intervals (e.g., 30fps).

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Auto-Reconnection & Backoff**
> The network is unreliable. Always enable the built-in auto-reconnection feature when instantiating the client. It uses an exponential backoff algorithm with jitter to prevent server thundering herds when the backend restarts. Furthermore, bind a reactive signal to the socket's connection state to display a visually obvious "Reconnecting..." indicator to the user.
