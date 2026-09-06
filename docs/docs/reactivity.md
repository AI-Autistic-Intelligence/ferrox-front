---
id: reactivity
title: Reactivity & Signals API
sidebar_position: 4
---

# ⚡ Signals Guide & State Management

**Signals** represent the fundamental building block for state management in Ferrox Front.

---

## 📦 1. `Signal<T>` Primitive

A `Signal<T>` encapsulates a reactive state value. It is cheaply clonable (`Arc<RwLock<T>>`) and thread-safe within the WebAssembly event loop.

```rust
use ferrox_front_core::reactivity::create_signal;

// Create a signal initialized to 0
let (count, set_count) = create_signal(0);

// Read current value
println!("Current value: {}", count.get());

// Set new value
set_count.set(42);

// Mutate via closure
set_count.update(|n| *n += 1);
```

---

## 🔄 2. Reactivity in UI Components

When a `Signal` is read within a UI component, the DOM node automatically subscribes to updates emitted by that Signal.

```rust
use ferrox_front_core::dom::{div, p, button};
use ferrox_front_core::reactivity::create_signal;

pub fn CounterExample() -> DomBuilder {
    let (score, set_score) = create_signal(100);

    div()
        .child(p().text(&format!("Current Score: {}", score.get())))
        .child(
            button()
                .text("+10 Points")
                .on_click(move || set_score.update(|s| *s += 10))
        )
}
```
