//! # CQRS Hooks Submodule (`ferrox-front-core::hooks`)
//!
//! `hooks` provides data fetching and mutation abstractions (`use_query`, `use_command`) connecting WebAssembly frontends
//! to Ferrox backend CQRS (Command Query Responsibility Segregation) endpoints.
//!
//! ## Key Hooks
//! - `use_query<T>(endpoint)`: Executes a read query, returning `QueryResult<T>` (`data`, `loading`, `error` signals).
//! - `use_command<T>(endpoint)`: Encapsulates a write mutation, returning `CommandResult<T>` (`execute`, `loading`, `result`).
//!
//! ## Example Usage
//! ```rust
//! use ferrox_front_core::hooks::use_query;
//!
//! #[derive(Clone)]
//! struct UserProfile { pub name: String }
//!
//! let query = use_query::<UserProfile>("/api/v1/user/me");
//! if query.loading.get() {
//!     println!("Loading user profile...");
//! }
//! ```

use crate::reactivity::Signal;

pub struct QueryResult<T> {
    pub data: Signal<Option<T>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

/// Mock CQRS use_query hook that interfaces with Ferrox backend
pub fn use_query<T: Clone + 'static>(endpoint: &str) -> QueryResult<T> {
    let data = Signal::new(None);
    let loading = Signal::new(true);
    let error = Signal::new(None);

    // In a real framework, this would use `web_sys::window().fetch_with_str(endpoint)`
    // and process the Future via `wasm_bindgen_futures::spawn_local`.
    // We simulate the fetching state.
    
    // Simulate fetch complete
    loading.set(false);

    QueryResult { data, loading, error }
}

pub struct CommandResult<T> {
    pub execute: Box<dyn Fn()>,
    pub loading: Signal<bool>,
    pub result: Signal<Option<T>>,
}

/// Mock CQRS use_command hook for mutations
pub fn use_command<T: Clone + 'static>(endpoint: &str) -> CommandResult<T> {
    let loading = Signal::new(false);
    let result = Signal::new(None);
    
    let execute = Box::new(|| {
        // Mock execution
    });

    CommandResult { execute, loading, result }
}