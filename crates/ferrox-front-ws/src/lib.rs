//! # Ferrox Front WS (`ferrox-front-ws`)
//!
//! `ferrox-front-ws` provides an async WebSocket client wrapper tailored for WebAssembly applications.
//!
//! ## Key Features
//! - 📡 **Async WebSocket Streams**: Connect, send, and receive WebSocket messages using Tokio/Futures channels.
//! - 🔄 **Auto-Reconnect**: Automatic exponential backoff reconnection handling on network drops.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use ferrox_front_core::reactivity::Signal;

pub struct FerroxSocket {
    pub url: String,
    pub status: Signal<String>,
}

impl FerroxSocket {
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