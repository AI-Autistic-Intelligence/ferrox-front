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
