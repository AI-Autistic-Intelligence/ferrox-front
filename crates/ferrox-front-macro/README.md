# Ferrox Front Macro (`ferrox-front-macro`)

`ferrox-front-macro` provides the JSX-like `rsx!` procedural macro for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-macro` crate contains the procedural macros necessary for authoring component templates in the Ferrox Front framework. Its primary architectural purpose is to provide an ergonomic, HTML-like syntax (`rsx!`) inside Rust code, which is then parsed at compile-time and expanded into highly optimized, zero-allocation WebAssembly DOM instructions via `ferrox-front-core`.

## 2. Architectural Layering
This crate acts as a compile-time transformation layer. It bridges the gap between the developer's declarative UI code and the low-level imperative DOM builder API of `ferrox-front-core`. Since it is a procedural macro crate, it operates exclusively during the Rust compilation phase (`rustc`) and has no runtime overhead in the final compiled WebAssembly binary.

## 3. How It Works Under the Hood
When the Rust compiler encounters an `rsx!` block, `ferrox-front-macro` uses the `syn` and `quote` crates to parse the custom pseudo-HTML syntax into an Abstract Syntax Tree (AST). It analyzes static HTML nodes versus dynamic signal-bound variables. Static subtrees are hoisted into Wasm-compatible HTML template elements that are cloned instantly (`Node.cloneNode`), while dynamic bindings (e.g., `{count.get()}`) generate targeted `create_effect` closures that patch only the changing text nodes or attributes.

## 4. Why It Was Designed This Way
Writing raw DOM manipulation code using `web_sys` (e.g., `document.create_element("div")`, `element.set_attribute()`) is extremely verbose and error-prone. Conversely, using a Virtual DOM string-parsing approach at runtime wastes CPU cycles. This macro was designed to offer the best of both worlds: the developer experience of JSX combined with the runtime performance of raw DOM manipulation, moving all parsing overhead to compile-time.

## 5. Practical Usage Guide & Extended Code Examples
The `rsx!` macro is used inside components to return an `Element`.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_macro::rsx;

#[component]
pub fn UserProfile(name: String, age: u32) -> Element {
    rsx! {
        div { class: "user-profile card",
            h2 { "User: {name}" }
            p {
                class: "details",
                "Age: ", strong { "{age}" }
            }
            // Event listeners
            button {
                on:click: move |_| log::info!("Clicked!"),
                "Say Hello"
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Complex Logic Inside `rsx!`**
> Do not write complex `match` statements, heavy loops, or extensive business logic directly within the `{}` braces of the `rsx!` macro. This makes the template difficult to read and forces the macro to parse large chunks of arbitrary Rust code. Calculate your state outside the macro and pass the resulting signals or variables in.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Component Composition**
> Keep your `rsx!` blocks small and compose larger UIs by calling other components as tags. For example, instead of a massive 500-line `rsx!` block, break it down into `<Header />`, `<Sidebar />`, and `<MainContent />` components. The macro optimizes these nested calls efficiently without runtime penalty.
