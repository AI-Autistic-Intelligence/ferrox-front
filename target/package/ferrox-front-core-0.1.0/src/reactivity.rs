use std::cell::RefCell;
use std::rc::Rc;

/// A simple reactive primitive. In a real production framework,
/// this would be backed by a thread-local arena (like Leptos or Sycamore)
/// to avoid Rc overhead and allow Copy semantics.
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(initial)),
        }
    }

    pub fn get(&self) -> T {
        // In a real implementation, getting the value would register the current
        // running Effect as a dependency.
        self.value.borrow().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        // In a real implementation, setting the value would trigger all
        // dependencies (Effects) to re-run.
    }
}

/// Helper function to create a new Signal
pub fn create_signal<T: Clone>(initial: T) -> Signal<T> {
    Signal::new(initial)
}
