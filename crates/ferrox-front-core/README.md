# Ferrox Front Core (`ferrox-front-core`)

`ferrox-front-core` is the foundational WebAssembly reactive UI engine for the Ferrox Front framework.

## 1. What It Is & Architectural Purpose
The `ferrox-front-core` crate serves as the reactive kernel and foundational engine for the entire Ferrox Front ecosystem. It provides surgical DOM updates via fine-grained **Signals** (`Signal<T>`, `create_signal`), a high-performance Wasm Virtual DOM builder, and runtime Glassmorphism theme switching. Its primary architectural purpose is to eliminate the need for heavy Virtual DOM diffing algorithms found in traditional JavaScript frameworks, replacing them with a direct, compile-time optimized dependency graph that runs natively in the browser via WebAssembly.

## 2. Architectural Layering
This crate sits at the absolute bottom of the Ferrox Front architectural stack. It interfaces directly with the browser's DOM Engine (`web_sys`) and WebAssembly linear memory. Higher-level crates like `ferrox-front-ui` (components) and `ferrox-front-router` rely entirely on the reactive primitives (`create_signal`, `create_effect`) exposed by this core layer. It abstracts away raw DOM pointers into safe, ergonomic Rust types while maintaining zero-cost abstraction principles.

## 3. How It Works Under the Hood
At its core, `ferrox-front-core` utilizes a signal-based reactivity graph. When a state change occurs (e.g., via `signal.set(value)`), the system does not re-evaluate the entire component tree. Instead, it marks only the specifically dependent `Effect` closures as stale. A micro-task batch scheduler then deduplicates redundant updates within the same animation frame (60fps) and triggers direct, surgical mutations on the exact target DOM nodes. This creates a highly efficient, push-based reactivity model entirely contained within Wasm linear memory.

## 4. Why It Was Designed This Way
Traditional Virtual DOM frameworks (like React or Vue) re-evaluate entire component subtrees on state changes, causing continuous memory allocations, V8 Mark-Sweep GC pauses, and CPU overhead. `ferrox-front-core` was designed to bypass this entirely. By managing the stack and arena linear memory manually in Rust, it guarantees zero Garbage Collection pauses, ensuring buttery-smooth animations and eliminating the micro-stuttering typically seen in complex single-page applications.

## 5. Practical Usage Guide & Extended Code Examples
The core primitives are typically used alongside the `rsx!` macro to build reactive components. 

```rust,no_run
use ferrox_front_core::prelude::*;

#[component]
pub fn Counter() -> Element {
    let (count, set_count) = create_signal(0);
    let double_count = create_memo(move |_| count.get() * 2);

    rsx! {
        div { class: "glass-card p-4 text-center",
            h1 { "Count: {count.get()}" }
            h2 { "Double: {double_count.get()}" }
            button {
                class: "btn btn-primary",
                on:click: move |_| set_count.update(|n| *n += 1),
                "Increment"
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Heavy Computation in Effects**
> Do not perform heavy synchronous CPU computations (e.g., large data sorting or cryptographic hashing) directly inside a `create_effect` or `on:click` handler. Since the reactive graph runs on the main browser UI thread, this will block DOM rendering and drop frames. 

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Use Derived Memos for Expensive State**
> Always use `create_memo` for state that is derived from other signals if the derivation logic is even slightly expensive. Memos cache their output and only re-evaluate when their specific dependencies change, preventing unnecessary recalculations during unrelated DOM updates.
