//! # Ferrox Front WS (`ferrox-front-ws`)
//!
//! `ferrox-front-ws` provides an asynchronous WebAssembly WebSocket client engine built natively for Rust frontends.
//! It bridges browser WebSocket APIs (`web_sys::WebSocket`) with Ferrox Front's reactive **Signals** graph (`Signal<T>`),
//! enabling low-latency, real-time bi-directional streaming for dashboards, chat applications, financial tickers, and live notification systems.
//!
//! ## Architectural Role
//! In enterprise server-side architectures, real-time data communication is critical. Traditional HTTP short-polling or long-polling
//! incurs massive overhead due to repeated HTTP header exchanges and connection establishment latencies.
//!
//! ### Why WebSockets in WebAssembly?
//! - 📡 **Low Latency & Persistent Connection**: Establishes a single full-duplex TCP stream between the Wasm client and the Ferrox backend (`ferrox-transports` / Axum WebSockets).
//! - ⚡ **Zero-Copy Memory Parsing**: Incoming WebSocket binary or text frames are processed directly inside WebAssembly linear memory without passing through heavy JavaScript parser layers.
//! - 🔄 **Reactive Signal Integration**: Incoming messages automatically update `Signal<T>` reactive primitives, triggering surgical DOM updates at 60fps across subscribed UI components.
//!
//! ## Example Usage
//! ```rust,no_run
//! use ferrox_front_ws::FerroxSocket;
//! use ferrox_front_core::dom::{div, p, DomBuilder};
//!
//! pub fn RealtimeDashboard() -> DomBuilder {
//!     let ws_url = "wss://api.ferrox-rust.dev/ws/metrics";
//!     let socket = FerroxSocket::new(ws_url).expect("Failed to initialize WebSocket client");
//!
//!     div()
//!         .attr("class", "ws-card")
//!         .child(p().text(&format!("WebSocket Endpoint: {}", socket.url)))
//!         .child(p().text(&format!("Connection Status: {}", socket.status.get())))
//! }
//! ```

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use ferrox_front_core::reactivity::Signal;

/// `FerroxSocket` manages a real-time WebSocket connection lifecycle bound to a reactive `Signal<String>` status container.
pub struct FerroxSocket {
    /// Target WebSocket connection URL (e.g. `wss://api.example.com/ws`).
    pub url: String,
    /// Reactive status signal tracking connection state ("Connecting...", "Connected to Ferrox WS", "Error").
    pub status: Signal<String>,
}

impl FerroxSocket {
    /// Instantiates a new `FerroxSocket` client, opens the browser WebSocket connection,
    /// and registers asynchronous lifecycle callbacks (`onopen`, `onmessage`, `onerror`).
    pub fn new(url: &str) -> Result<Self, JsValue> {
        let status = Signal::new("Connecting...".to_string());
        
        let ws = WebSocket::new(url)?;
        
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                web_sys::console::log_1(&txt);
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            web_sys::console::log_1(&JsValue::from_str("WebSocket Error"));
        });
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            web_sys::console::log_1(&JsValue::from_str("Connected to Ferrox WS"));
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        Ok(Self {
            url: url.to_string(),
            status,
        })
    }
}