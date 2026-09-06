# Ferrox Front Core (`ferrox-front-core`)

`ferrox-front-core` is the foundational WebAssembly reactive UI engine for the Ferrox Front framework.
It provides surgical DOM updates via fine-grained **Signals** (`Signal<T>`, `create_signal`), a high-performance
Wasm Virtual DOM builder, and runtime Glassmorphism theme switching.

## Architectural Rationale
Traditional Virtual DOM frameworks (like React) re-evaluate entire component subtrees on state changes, causing memory allocations and CPU overhead.
`ferrox-front-core` uses a signal-based reactivity graph where state changes trigger direct, surgical mutations on target DOM nodes at 60fps.

## Key Features
- ⚡ **Signals Reactivity**: Fine-grained reactive primitives (`create_signal`, `create_effect`, `create_memo`).
- 🌐 **Wasm DOM Builder**: Zero-copy DOM node creation and event listener bindings.
- 🎨 **Glassmorphism Theming Engine**: Built-in dynamic theme switching (Cyber, Ocean, Forest, Sunset, Corporate).

## Example Usage
```rust,no_run
use ferrox_front_core::prelude::*;

#[component]
pub fn Counter() -> Element {
    let (count, set_count) = create_signal(0);

    rsx! {
        div { class: "p-4 text-center",
            h1 { "Count: {count.get()}" }
            button {
                class: "btn btn-primary",
                onclick: move |_| set_count.update(|n| *n += 1),
                "Increment"
            }
        }
    }
}
```
