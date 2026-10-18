//! # Ferrox Front WS (`ferrox-front-ws`)
//!
//! Enterprise WebAssembly WebSocket client for real-time bi-directional market data streaming.
//! Features:
//! - `ReconnectingWs`: Automatic reconnection with exponential backoff and randomized jitter.
//! - Outgoing message queue buffer during transient disconnects.
//! - Typed market event dispatcher for channels: `Trades`, `OrderBookDepth`, and `Ticker`.
//! - Reactive connection lifecycle signals (`Signal<WsStatus>`).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CloseEvent, ErrorEvent, MessageEvent, WebSocket};
use ferrox_front_core::reactivity::Signal;

/// Connection status lifecycle of `ReconnectingWs`.
#[derive(Debug, Clone, PartialEq)]
pub enum WsStatus {
    Connecting,
    Connected,
    Reconnecting { attempt: u32, next_delay_ms: u32 },
    Disconnected,
    Error(String),
}

impl std::fmt::Display for WsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WsStatus::Connecting => write!(f, "Connecting..."),
            WsStatus::Connected => write!(f, "Connected to Ferrox WS"),
            WsStatus::Reconnecting { attempt, next_delay_ms } => {
                write!(f, "Reconnecting (attempt #{}, {}ms)...", attempt, next_delay_ms)
            }
            WsStatus::Disconnected => write!(f, "Disconnected"),
            WsStatus::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

/// A trade execution event payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WsTrade {
    pub price: f64,
    pub amount: f64,
    pub is_buy: bool,
    pub timestamp: u64,
}

/// Typed market events streamed over WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum WsMarketEvent {
    Trades {
        pair: String,
        trades: Vec<WsTrade>,
    },
    OrderBookDepth {
        pair: String,
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
    },
    Ticker {
        pair: String,
        last_price: f64,
        volume_24h: f64,
        change_24h: f64,
        high_24h: f64,
        low_24h: f64,
    },
    MeshTelemetry {
        rust_matching_latency_ns: u64,
        python_feed_latency_ms: f64,
        java_loom_settled_per_sec: u64,
        java_loom_latency_micros: u64,
        php_saga_active_count: u32,
        php_saga_success_rate: f64,
        redpanda_throughput_eps: u64,
        timestamp: u64,
    },
    Raw(String),
}

/// Robust Reconnecting WebSocket Client for WebAssembly applications.
#[derive(Clone)]
pub struct ReconnectingWs {
    pub url: String,
    pub status: Signal<WsStatus>,
    ws_instance: Rc<RefCell<Option<WebSocket>>>,
    message_queue: Rc<RefCell<VecDeque<String>>>,
    retry_count: Rc<RefCell<u32>>,
    base_delay_ms: u32,
    max_delay_ms: u32,
    listeners: Rc<RefCell<Vec<Rc<dyn Fn(&WsMarketEvent)>>>>,
}

impl ReconnectingWs {
    /// Creates a new ReconnectingWs client with default exponential backoff parameters.
    pub fn new(url: &str) -> Self {
        Self::with_backoff(url, 500, 15000)
    }

    /// Creates a ReconnectingWs client with customized backoff parameters.
    pub fn with_backoff(url: &str, base_delay_ms: u32, max_delay_ms: u32) -> Self {
        let ws = Self {
            url: url.to_string(),
            status: Signal::new(WsStatus::Disconnected),
            ws_instance: Rc::new(RefCell::new(None)),
            message_queue: Rc::new(RefCell::new(VecDeque::new())),
            retry_count: Rc::new(RefCell::new(0)),
            base_delay_ms,
            max_delay_ms,
            listeners: Rc::new(RefCell::new(Vec::new())),
        };

        ws.connect();
        ws
    }

    /// Subscribes a listener callback to incoming market events.
    pub fn on_event<F>(&self, callback: F)
    where
        F: Fn(&WsMarketEvent) + 'static,
    {
        self.listeners.borrow_mut().push(Rc::new(callback));
    }

    /// Sends a raw text message. If offline, the message is queued in memory until reconnected.
    pub fn send(&self, msg: &str) {
        let ws_opt = self.ws_instance.borrow();
        if let Some(ws) = ws_opt.as_ref() {
            if ws.ready_state() == WebSocket::OPEN {
                let _ = ws.send_with_str(msg);
                return;
            }
        }
        // Queue message for later transmission
        self.message_queue.borrow_mut().push_back(msg.to_string());
    }

    /// Serializes and sends a typed JSON payload.
    pub fn send_json<T: Serialize>(&self, payload: &T) {
        if let Ok(json_str) = serde_json::to_string(payload) {
            self.send(&json_str);
        }
    }

    /// Subscribes to a market data channel (e.g. "trades", "depth", "ticker") for a pair.
    pub fn subscribe(&self, channel: &str, pair: &str) {
        let payload = serde_json::json!({
            "action": "subscribe",
            "channel": channel,
            "pair": pair
        });
        self.send_json(&payload);
    }

    /// Initiates connection lifecycle.
    pub fn connect(&self) {
        self.status.set(WsStatus::Connecting);

        let ws = match WebSocket::new(&self.url) {
            Ok(socket) => socket,
            Err(e) => {
                let err_str = format!("{:?}", e);
                self.status.set(WsStatus::Error(err_str));
                self.schedule_reconnect();
                return;
            }
        };

        let self_open = self.clone();
        let onopen_cb = Closure::<dyn FnMut()>::new(move || {
            self_open.status.set(WsStatus::Connected);
            *self_open.retry_count.borrow_mut() = 0;

            // Flush message queue
            let mut queue = self_open.message_queue.borrow_mut();
            if let Some(ws) = self_open.ws_instance.borrow().as_ref() {
                while let Some(msg) = queue.pop_front() {
                    let _ = ws.send_with_str(&msg);
                }
            }
        });
        ws.set_onopen(Some(onopen_cb.as_ref().unchecked_ref()));
        onopen_cb.forget();

        let self_msg = self.clone();
        let onmessage_cb = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let s: String = txt.into();
                let event = match serde_json::from_str::<WsMarketEvent>(&s) {
                    Ok(ev) => ev,
                    Err(_) => WsMarketEvent::Raw(s),
                };

                let listeners = self_msg.listeners.borrow();
                for l in listeners.iter() {
                    l(&event);
                }
            }
        });
        ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
        onmessage_cb.forget();

        let self_err = self.clone();
        let onerror_cb = Closure::<dyn FnMut(_)>::new(move |_e: ErrorEvent| {
            self_err.status.set(WsStatus::Error("Errore WebSocket di rete".to_string()));
        });
        ws.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
        onerror_cb.forget();

        let self_close = self.clone();
        let onclose_cb = Closure::<dyn FnMut(_)>::new(move |_e: CloseEvent| {
            if self_close.status.get() != WsStatus::Disconnected {
                self_close.schedule_reconnect();
            }
        });
        ws.set_onclose(Some(onclose_cb.as_ref().unchecked_ref()));
        onclose_cb.forget();

        *self.ws_instance.borrow_mut() = Some(ws);
    }

    /// Schedules an automatic reconnection attempt with exponential backoff and jitter.
    fn schedule_reconnect(&self) {
        let mut attempt = *self.retry_count.borrow();
        attempt += 1;
        *self.retry_count.borrow_mut() = attempt;

        // Exponential backoff: base * 2^(attempt - 1) capped at max_delay
        let exp = (2u64.saturating_pow(attempt.min(8) - 1)) as f64;
        let mut delay = (self.base_delay_ms as f64 * exp).min(self.max_delay_ms as f64);

        // Add randomized jitter ±20%
        let jitter = 0.8 + (js_sys::Math::random() * 0.4);
        delay *= jitter;
        let delay_ms = delay as u32;

        self.status.set(WsStatus::Reconnecting {
            attempt,
            next_delay_ms: delay_ms,
        });

        let self_clone = self.clone();
        if let Some(win) = window() {
            let cb = Closure::<dyn FnMut()>::new(move || {
                self_clone.connect();
            });
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                delay_ms as i32,
            );
            cb.forget();
        }
    }

    /// Closes the connection and stops automatic reconnection.
    pub fn disconnect(&self) {
        self.status.set(WsStatus::Disconnected);
        if let Some(ws) = self.ws_instance.borrow_mut().take() {
            let _ = ws.close();
        }
    }
}

/// Backwards compatibility struct wrapping `ReconnectingWs`.
pub struct FerroxSocket {
    pub url: String,
    pub status: Signal<String>,
    inner: ReconnectingWs,
}

impl FerroxSocket {
    pub fn new(url: &str) -> Result<Self, JsValue> {
        let inner = ReconnectingWs::new(url);
        let status_str = Signal::new("Connecting...".to_string());

        let status_str_clone = status_str.clone();
        let inner_status = inner.status.clone();

        // Update status string on change
        let current = inner_status.get().to_string();
        status_str_clone.set(current);

        Ok(Self {
            url: url.to_string(),
            status: status_str,
            inner,
        })
    }

    pub fn send(&self, msg: &str) {
        self.inner.send(msg);
    }
}