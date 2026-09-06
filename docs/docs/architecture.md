---
id: architecture
title: Architecture Deep Dive
sidebar_position: 3
---

# 🧠 Internal Architecture of Ferrox Front

Ferrox Front is engineered using system design principles and zero-cost abstractions to guarantee peak execution performance inside WebAssembly.

---

## ⚡ 1. Reactive Signals Dependency Graph

Unlike Virtual DOM frameworks that calculate component tree diffs on every state change, Ferrox Front uses a directed dependency graph of fine-grained reactive primitives called **Signals**.

```
[ Signal<T> (State) ] ---> (Automatic Notification) ---> [ Subscriber / DOM Node ]
```

### Key Advantages of Wasm Signals:
- **Zero Allocations**: Updates directly mutate the target DOM node attribute or text via `web_sys`.
- **Surgical Execution**: If only a single `<span>` depends on a Signal, only that `<span>` is updated. No parent or sibling component re-renders.

---

## 🎨 2. Glassmorphism Theme Engine

The dynamic theming system leverages native CSS custom properties controlled by the `data-theme` attribute on the root `<html>` element.

| Theme Name | Description |
|---|---|
| `ferrox-cyber` | Neon Purple/Cyan with Dark Background and Frosted Glass blur |
| `ocean-breeze` | Deep Oceanic Blue and Atlantic Cyan with translucent accents |
| `midnight-forest` | Dark Emerald background with vivid green highlights |
| `sunset-gold` | Warm Amber tones and golden orange gradients |
| `corporate-slate` | Minimalist Slate Grey layout for corporate enterprise dashboards |

### 60fps Theme Switching in Rust:
```rust
use ferrox_front_core::theme::set_theme;

// Switch theme dynamically at runtime
set_theme("ferrox-cyber");
```

---

## 🧱 3. Zero-Copy DOM Builder

Every Ferrox component returns a `DomBuilder`. DOM nodes are constructed inside WebAssembly linear memory and mounted into the browser DOM using direct calls to `web_sys::Document`.
