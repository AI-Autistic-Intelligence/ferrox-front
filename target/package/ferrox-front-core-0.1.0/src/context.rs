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
