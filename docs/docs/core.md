---
id: core
title: Ferrox Front Core Engine (ferrox-front-core)
sidebar_position: 2
---

# 🧠 Ferrox Front Core Engine (`ferrox-front-core`)

`ferrox-front-core` is the foundational engine of Ferrox Front. It houses the fine-grained **Signals Reactivity Graph**, the zero-copy **Wasm DOM Builder**, and the **Glassmorphism Theme System**.

---

## ⚡ 1. The Signals Reactivity Engine

The reactive system is built around `Signal<T>`, an atomic, thread-safe, cheaply-clonable state container (`Arc<RwLock<T>>`).

### API Summary

```rust
use ferrox_front_core::reactivity::{create_signal, Signal};

// 1. Create a reactive signal pair
let (read_signal, write_signal) = create_signal(100);

// 2. Read current value (.get())
let current_val = read_signal.get();

// 3. Set new value (.set(new_val))
write_signal.set(200);

// 4. In-place closure mutation (.update(closure))
write_signal.update(|val| *val += 50);
```

---

## 🧱 2. DOM Builder Engine (`DomBuilder`)

Ferrox Front does not use string concatenations or runtime HTML templates. Components build node hierarchies using `DomBuilder`, which invokes browser Web APIs (`web_sys::Document`) directly.

### Constructor & Helper Functions

```rust
use ferrox_front_core::dom::{div, span, button, h1, p, DomBuilder, mount};

// Create elements
let container = div()
    .attr("class", "my-container")
    .attr("style", "padding: 1rem;")
    .child(h1().text("Welcome"))
    .child(
        button()
            .attr("class", "btn-primary")
            .text("Click Me")
            .on_click(|| println!("Clicked!"))
    );

// Mount to browser DOM
mount("#root", container);
```

---

## 🎨 3. Runtime Glassmorphism Theme Engine (`theme`)

Ferrox Front supports dynamic runtime theme switching at 60fps by mutating the `data-theme` attribute on the root HTML element.

### Available Themes
- `ferrox-cyber`: Dark background with neon purple/cyan accents and frosted glass blur.
- `ocean-breeze`: Oceanic blue and Atlantic cyan gradients.
- `midnight-forest`: Deep emerald green background.
- `sunset-gold`: Warm amber and golden orange tones.
- `corporate-slate`: Minimalist slate grey corporate styling.

### API Reference

```rust
use ferrox_front_core::theme::{set_theme, get_theme};

// Switch active theme
set_theme("ferrox-cyber");

// Query current theme
let active = get_theme();
assert_eq!(active, "ferrox-cyber");
```
