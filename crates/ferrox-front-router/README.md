# Ferrox Front Router (`ferrox-front-router`)

`ferrox-front-router` provides Client-Side History SPA Routing for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-router` crate implements a robust, client-side Single Page Application (SPA) routing engine. It intercepts browser URL changes and history API events (`pushState`, `popState`) to dynamically swap out active UI components without triggering a full page reload. Its architectural purpose is to provide standard URL-based navigation, deep-linking, and route parameter extraction while remaining fully integrated with the `ferrox-front-core` reactivity system.

## 2. Architectural Layering
The Router sits conceptually above the `core` reactivity engine. It exposes a global context containing the current `Location` (path, query string, hash) as a reactive Signal. The primary `<Router>` and `<Route>` components consume this signal. When the URL changes, the signal updates, triggering a localized DOM patching cycle that unmounts the old route component and mounts the new one instantly, avoiding a full VDOM diff.

## 3. How It Works Under the Hood
When the application initializes the `<Router>`, an event listener is attached to the browser's `window` object to listen for `popstate` events. Link clicks (`<a>` tags) are intercepted to prevent default browser navigation, instead updating the History API and setting the reactive `Location` signal. The `<Router>` macro matches the current path against defined `<Route>` paths using a highly optimized Radix tree algorithm to support nested routes and dynamic parameters (e.g., `/user/:id`).

## 4. Why It Was Designed This Way
Native browser navigation causes a full context tear-down and rebuild, which is unacceptably slow for complex WebAssembly applications that need to maintain state (like WebSockets or heavy data caches). By intercepting navigation and making the URL just another piece of reactive state, the framework achieves instant page transitions while maintaining the benefits of shareable, bookmarkable URLs.

## 5. Practical Usage Guide & Extended Code Examples
Define your routes inside a root `Router` component.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_router::prelude::*;

#[component]
pub fn AppRouter() -> Element {
    rsx! {
        Router {
            // Static routes
            Route { path: "/", component: Home }
            Route { path: "/dashboard", component: Dashboard }
            
            // Dynamic route with parameter extraction
            Route { path: "/user/:id", component: UserProfile }
            
            // 404 Fallback
            Route { path: "*", component: NotFound }
        }
    }
}

#[component]
pub fn UserProfile() -> Element {
    let params = use_params(); // Reactive hook for route parameters
    let user_id = params.get("id").unwrap_or_default();
    
    rsx! {
        div { "Viewing profile for user ID: {user_id}" }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Hardcoded `href` Attributes for Internal Links**
> Do not use standard HTML `<a href="/dashboard">` tags for internal navigation unless you explicitly want a full page reload. Instead, use the provided `<Link to="/dashboard">` component, which properly intercepts the click and routes it through the SPA history engine.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Route-Level Code Splitting**
> While WebAssembly doesn't support lazy-loading individual functions natively yet, you can optimize route transitions by suspending heavy data fetches at the route boundary. Use a `<Suspense>` wrapper around your `<Route>` components to show a loading skeleton while the specific route data is fetched asynchronously.
