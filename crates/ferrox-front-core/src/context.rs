//! # Context API Submodule (`ferrox-front-core::context`)
//!
//! `context` provides thread-local dependency injection for Ferrox Front applications, allowing deeply nested components
//! to consume shared state (e.g. current user session, theme config, API clients) without prop drilling through intermediate nodes.
//!
//! ## Key Functions
//! - `provide_context<T: 'static>(value: T)`: Stores a value of type `T` in the thread-local context registry.
//! - `use_context<T: Clone + 'static>() -> Option<T>`: Retrieves a context value of type `T` from the current thread registry.
//!
//! ## Example Usage
//! ```rust
//! use ferrox_front_core::context::{provide_context, use_context};
//!
//! #[derive(Clone, Debug)]
//! struct UserConfig { pub theme: String }
//!
//! // Provide context at root
//! provide_context(UserConfig { theme: "ferrox-cyber".into() });
//!
//! // Consume context in child component
//! if let Some(config) = use_context::<UserConfig>() {
//!     println!("Current theme context: {}", config.theme);
//! }
//! ```

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static CONTEXT_REGISTRY: RefCell<HashMap<TypeId, Rc<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Provides a context value globally to the current tree thread-locally.
pub fn provide_context<T: 'static>(value: T) {
    CONTEXT_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(TypeId::of::<T>(), Rc::new(value));
    });
}

/// Consumes a context value of type T.
pub fn use_context<T: Clone + 'static>() -> Option<T> {
    CONTEXT_REGISTRY.with(|registry| {
        registry.borrow().get(&TypeId::of::<T>()).and_then(|rc_any| {
            rc_any.downcast_ref::<T>().cloned()
        })
    })
}