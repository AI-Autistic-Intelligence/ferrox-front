//! # Ferrox Front Router (`ferrox-front-router`)
//!
//! `ferrox-front-router` provides client-side SPA routing powered by the browser History API for Ferrox Front applications.
//!
//! ## Key Features
//! - 🛣️ **`Router` & `Route` Components**: Declarative route declaration mapped to component views.
//! - 🔗 **`Link` Component**: Non-reloading client-side navigation.
//! - 🔒 **Navigation Guards**: Protect routes based on authentication state or user permissions.

use wasm_bindgen::prelude::*;
use web_sys::window;
use ferrox_front_core::reactivity::Signal;

pub struct Router {
    pub current_route: Signal<String>,
}

impl Router {
    pub fn new() -> Self {
        let win = window().expect("No window found");
        let location = win.location();
        let pathname = location.pathname().unwrap_or_else(|_| "/".to_string());
        
        Self {
            current_route: Signal::new(pathname),
        }
    }

    pub fn navigate(&self, path: &str) {
        let win = window().expect("No window found");
        let history = win.history().expect("No history found");
        
        // Push state to browser history
        history.push_state_with_url(&JsValue::NULL, "", Some(path)).unwrap();
        
        // Update reactive state
        self.current_route.set(path.to_string());
    }
}