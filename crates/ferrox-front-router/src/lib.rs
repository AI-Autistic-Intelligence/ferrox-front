//! # Ferrox Front Router (`ferrox-front-router`)
//!
//! Client-side History API Single Page Application (SPA) router with dynamic route parameters,
//! reactive query string parser, popstate synchronization, and Zero-Trust Auth Guard
//! backed by WebAssembly linear memory (`SealedToken`).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Event};
use ferrox_front_core::dom::{div, a, p, h2, DomBuilder};
use ferrox_front_core::reactivity::Signal;
use ferrox_front_security::SealedToken;

/// Percent-decodes a URL-encoded string slice.
pub fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.as_bytes().iter();
    while let Some(&b) = bytes.next() {
        if b == b'+' {
            result.push(' ');
        } else if b == b'%' {
            let h1 = bytes.next().copied().unwrap_or(b'0');
            let h2 = bytes.next().copied().unwrap_or(b'0');
            let hex_str = format!("{}{}", h1 as char, h2 as char);
            if let Ok(byte_val) = u8::from_str_radix(&hex_str, 16) {
                result.push(byte_val as char);
            } else {
                result.push('%');
                result.push(h1 as char);
                result.push(h2 as char);
            }
        } else {
            result.push(b as char);
        }
    }
    result
}

/// Percent-encodes a string for safe embedding into query parameters.
pub fn url_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", b));
            }
        }
    }
    result
}

/// Reactive query string parameters parser (`?page=2&filter=active&sort=desc`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    /// Parses a raw query string (with or without leading `?`).
    pub fn parse(query_str: &str) -> Self {
        let mut map = HashMap::new();
        let trimmed = query_str.trim_start_matches('?');
        if !trimmed.is_empty() {
            for pair in trimmed.split('&') {
                if pair.is_empty() {
                    continue;
                }
                let mut parts = pair.splitn(2, '=');
                let key = parts.next().unwrap_or("");
                let val = parts.next().unwrap_or("");
                map.insert(url_decode(key), url_decode(val));
            }
        }
        Self { params: map }
    }

    /// Gets a parameter value by key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    /// Parses a numeric u32 query param.
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.params.get(key).and_then(|v| v.parse::<u32>().ok())
    }

    /// Parses a boolean query param (`true`, `1`, `yes`).
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key).map(|v| {
            let lower = v.to_lowercase();
            lower == "true" || lower == "1" || lower == "yes"
        })
    }

    /// Returns the underlying parameter map.
    pub fn entries(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Converts back to a query string (e.g. `?foo=bar&num=42`).
    pub fn to_query_string(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }
        let mut pairs: Vec<String> = self
            .params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect();
        pairs.sort();
        format!("?{}", pairs.join("&"))
    }
}

/// Result of matching a route against the active URL.
#[derive(Clone, Debug, PartialEq)]
pub struct RouteMatch {
    pub path: String,
    pub pattern: String,
    pub params: HashMap<String, String>,
    pub query: QueryParams,
}

impl RouteMatch {
    /// Convenience helper to read a named dynamic route parameter (e.g. `:id`).
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    /// Convenience helper to read a query parameter.
    pub fn query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }
}

/// Route Guard protecting navigation.
#[derive(Clone)]
pub struct RouteGuard {
    pub requires_auth: bool,
    pub redirect_to: String,
    pub custom_check: Option<Rc<dyn Fn(&RouteMatch, Option<&SealedToken>) -> Result<(), String>>>,
}

impl RouteGuard {
    /// Requires an authenticated session via a valid SealedToken in Wasm memory.
    pub fn require_auth() -> Self {
        Self {
            requires_auth: true,
            redirect_to: "/login".to_string(),
            custom_check: None,
        }
    }

    /// Custom validation guard with customizable redirection.
    pub fn custom(
        redirect_to: &str,
        check: impl Fn(&RouteMatch, Option<&SealedToken>) -> Result<(), String> + 'static,
    ) -> Self {
        Self {
            requires_auth: false,
            redirect_to: redirect_to.to_string(),
            custom_check: Some(Rc::new(check)),
        }
    }

    /// Evaluates whether navigation is permitted.
    pub fn check(&self, route_match: &RouteMatch, token: Option<&SealedToken>) -> Result<(), String> {
        if self.requires_auth {
            match token {
                Some(t) if t.is_valid() => {}
                _ => {
                    return Err(format!(
                        "{}?return_to={}",
                        self.redirect_to,
                        url_encode(&route_match.path)
                    ));
                }
            }
        }
        if let Some(custom) = &self.custom_check {
            if let Err(redirect) = custom(route_match, token) {
                return Err(redirect);
            }
        }
        Ok(())
    }
}

/// Definition of an SPA route.
#[derive(Clone)]
pub struct RouteDef {
    pub pattern: String,
    pub guard: Option<RouteGuard>,
    pub view: Rc<dyn Fn(&RouteMatch) -> DomBuilder>,
}

/// Pattern matching function for route segments.
/// Matches patterns like `/orders/:id`, `/analytics/:tab`, `/users/:user_id/orders/:order_id`.
pub fn match_route_pattern(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let r_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();

    // Check for root route match
    if p_segs.is_empty() && r_segs.is_empty() {
        return Some(HashMap::new());
    }

    // Check for catch-all wildcards
    if pattern == "*" {
        let mut map = HashMap::new();
        map.insert("path".to_string(), path.to_string());
        return Some(map);
    }

    if p_segs.len() != r_segs.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (p, r) in p_segs.iter().zip(r_segs.iter()) {
        if let Some(stripped) = p.strip_prefix(':') {
            params.insert(stripped.to_string(), url_decode(r));
        } else if *p != *r {
            return None;
        }
    }

    Some(params)
}

/// Single Page Application Router.
#[derive(Clone)]
pub struct Router {
    pub current_path: Signal<String>,
    pub current_query: Signal<QueryParams>,
    pub active_match: Signal<Option<RouteMatch>>,
    pub auth_token: Signal<Option<SealedToken>>,
    routes: Rc<RefCell<Vec<RouteDef>>>,
    fallback: Rc<RefCell<Option<Rc<dyn Fn(&str) -> DomBuilder>>>>,
    listeners: Rc<RefCell<Vec<Rc<dyn Fn(&str)>>>>,
}

impl Router {
    /// Creates a new Router, extracting the initial pathname and query string from browser window.
    pub fn new() -> Self {
        let (initial_path, initial_query) = if let Some(win) = window() {
            let loc = win.location();
            let path = loc.pathname().unwrap_or_else(|_| "/".to_string());
            let search = loc.search().unwrap_or_default();
            (path, QueryParams::parse(&search))
        } else {
            ("/".to_string(), QueryParams::default())
        };

        let router = Self {
            current_path: Signal::new(initial_path.clone()),
            current_query: Signal::new(initial_query),
            active_match: Signal::new(None),
            auth_token: Signal::new(None),
            routes: Rc::new(RefCell::new(Vec::new())),
            fallback: Rc::new(RefCell::new(None)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        };

        // Attach popstate listener for browser Back/Forward navigation
        router.listen_popstate();

        router
    }

    /// Registers a listener callback invoked whenever the active route path changes.
    pub fn on_route_change(&self, listener: impl Fn(&str) + 'static) {
        self.listeners.borrow_mut().push(Rc::new(listener));
    }

    /// Broadcasts route updates to all registered listeners.
    fn notify_listeners(&self, path: &str) {
        let listeners = self.listeners.borrow().clone();
        for l in listeners {
            l(path);
        }
    }

    /// Registers a standard public route.
    pub fn add_route(
        &self,
        pattern: &str,
        view: impl Fn(&RouteMatch) -> DomBuilder + 'static,
    ) {
        self.routes.borrow_mut().push(RouteDef {
            pattern: pattern.to_string(),
            guard: None,
            view: Rc::new(view),
        });
        self.re_evaluate();
    }

    /// Registers a protected route requiring authentication and passing RouteGuard checks.
    pub fn add_protected_route(
        &self,
        pattern: &str,
        guard: RouteGuard,
        view: impl Fn(&RouteMatch) -> DomBuilder + 'static,
    ) {
        self.routes.borrow_mut().push(RouteDef {
            pattern: pattern.to_string(),
            guard: Some(guard),
            view: Rc::new(view),
        });
        self.re_evaluate();
    }

    /// Configures a custom 404 fallback page renderer.
    pub fn set_fallback(&self, fallback: impl Fn(&str) -> DomBuilder + 'static) {
        *self.fallback.borrow_mut() = Some(Rc::new(fallback));
    }

    /// Updates the sealed authentication token held in Wasm memory.
    pub fn set_auth_token(&self, token: Option<SealedToken>) {
        self.auth_token.set(token);
        self.re_evaluate();
    }

    /// Retrieves a cloned reference to the active sealed token.
    pub fn get_auth_token(&self) -> Option<SealedToken> {
        self.auth_token.get()
    }

    /// Navigates to a new URL, pushing a state onto the browser History API.
    pub fn navigate(&self, url: &str) {
        let (path, query_str) = split_path_and_query(url);
        let query = QueryParams::parse(query_str);

        if let Some(win) = window() {
            if let Ok(history) = win.history() {
                let _ = history.push_state_with_url(&JsValue::NULL, "", Some(url));
            }
            if let Ok(event) = web_sys::Event::new("popstate") {
                let _ = win.dispatch_event(&event);
            }
        }

        self.current_path.set(path.to_string());
        self.current_query.set(query);
        self.re_evaluate();
        self.notify_listeners(path);
    }

    /// Replaces the current state in browser History API without pushing a new entry.
    pub fn navigate_replace(&self, url: &str) {
        let (path, query_str) = split_path_and_query(url);
        let query = QueryParams::parse(query_str);

        if let Some(win) = window() {
            if let Ok(history) = win.history() {
                let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(url));
            }
            if let Ok(event) = web_sys::Event::new("popstate") {
                let _ = win.dispatch_event(&event);
            }
        }

        self.current_path.set(path.to_string());
        self.current_query.set(query);
        self.re_evaluate();
        self.notify_listeners(path);
    }

    /// Re-evaluates routes against current path, enforcing guards and updating `active_match`.
    pub fn re_evaluate(&self) {
        let cur_path = self.current_path.get();
        let cur_query = self.current_query.get();
        let token = self.auth_token.get();

        let routes = self.routes.borrow();
        for route in routes.iter() {
            if let Some(params) = match_route_pattern(&route.pattern, &cur_path) {
                let rm = RouteMatch {
                    path: cur_path.clone(),
                    pattern: route.pattern.clone(),
                    params,
                    query: cur_query.clone(),
                };

                // Check guard
                if let Some(guard) = &route.guard {
                    if let Err(redirect_url) = guard.check(&rm, token.as_ref()) {
                        // Redirect triggered by guard
                        drop(routes);
                        self.navigate_replace(&redirect_url);
                        return;
                    }
                }

                self.active_match.set(Some(rm));
                return;
            }
        }

        // No match found
        self.active_match.set(None);
    }

    /// Subscribes to the native or synthetic `popstate` window event for back/forward and SPA navigation.
    fn listen_popstate(&self) {
        if let Some(win) = window() {
            let self_clone = self.clone();
            let closure = Closure::wrap(Box::new(move |_: Event| {
                if let Some(w) = window() {
                    let loc = w.location();
                    let path = loc.pathname().unwrap_or_else(|_| "/".to_string());
                    let search = loc.search().unwrap_or_default();
                    self_clone.current_path.set(path.clone());
                    self_clone.current_query.set(QueryParams::parse(&search));
                    self_clone.re_evaluate();
                    self_clone.notify_listeners(&path);
                }
            }) as Box<dyn FnMut(Event)>);

            let _ = win.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }

    /// Renders the currently active route view or fallback 404 view.
    pub fn render_active(&self) -> DomBuilder {
        let cur_match = self.active_match.get();
        let routes = self.routes.borrow();

        if let Some(rm) = cur_match {
            for route in routes.iter() {
                if route.pattern == rm.pattern {
                    return (route.view)(&rm);
                }
            }
        }

        // Render fallback if configured
        let cur_path = self.current_path.get();
        if let Some(fb) = self.fallback.borrow().as_ref() {
            return fb(&cur_path);
        }

        // Default 404 view
        default_404_view(&cur_path)
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function splitting URL into path and query string.
fn split_path_and_query(url: &str) -> (&str, &str) {
    if let Some(pos) = url.find('?') {
        (&url[..pos], &url[pos..])
    } else {
        (url, "")
    }
}

/// Default 404 Not Found view with Glassmorphic styling.
fn default_404_view(path: &str) -> DomBuilder {
    div()
        .class("flex flex-col items-center justify-center min-h-[400px] p-8 text-center")
        .children(vec![
            h2().class("text-4xl font-extrabold text-rose-500 mb-2").text("404"),
            p().class("text-lg font-semibold text-slate-200 mb-1").text("Pagina Non Trovata"),
            p().class("text-sm text-slate-400 font-mono mb-4").text(&format!("Rotta richiesta: {}", path)),
            a()
                .attr("href", "#")
                .class("px-4 py-2 rounded-xl bg-indigo-600 text-white text-xs font-semibold shadow-lg hover:bg-indigo-500 transition-all")
                .text("← Torna alla Home"),
        ])
}

/// High-level component rendering the conditional active route of the Router.
pub fn router_view(router: &Router) -> DomBuilder {
    div()
        .class("ferrox-router-view w-full h-full transition-opacity duration-200")
        .child(router.render_active())
}

/// Accessible SPA client navigation link component preventing page reloads.
pub fn link(router: &Router, to: &str, label_text: &str, class_name: &str) -> DomBuilder {
    let to_owned = to.to_string();
    let router_clone = router.clone();

    a()
        .attr("href", to)
        .class(class_name)
        .text(label_text)
        .on("click", move |e: Event| {
            e.prevent_default();
            router_clone.navigate(&to_owned);
        })
}