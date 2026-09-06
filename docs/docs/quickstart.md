---
id: quickstart
title: Quickstart Guide
sidebar_position: 2
---

# 🚀 Quickstart: From Zero to Hero in 5 Minutes

This guide demonstrates how to install prerequisites, build your first WebAssembly application in Rust using Ferrox Front, and run it in the browser.

---

## 📦 1. Prerequisites

Ensure you have the Rust toolchain installed. Add the WebAssembly compilation target:

```bash
rustup target add wasm32-unknown-unknown
```

Install `trunk`, the fast WebAssembly build tool and local dev server for Rust:

```bash
cargo install trunk
```

---

## 🛠️ 2. Create a New Project

Create a new binary project with Cargo:

```bash
cargo new --bin my-ferrox-app
cd my-ferrox-app
```

Add Ferrox Front dependencies to your `Cargo.toml`:

```toml
[package]
name = "my-ferrox-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
ferrox-front-core = { path = "../ferrox-front/crates/ferrox-front-core" }
ferrox-front-ui = { path = "../ferrox-front/crates/ferrox-front-ui" }
ferrox-front-macro = { path = "../ferrox-front/crates/ferrox-front-macro" }
ferrox-front-security = { path = "../ferrox-front/crates/ferrox-front-security" }
wasm-bindgen = "0.2"
web-sys = "0.3"
```

---

## 🌐 3. Create the `index.html` File

Create `index.html` in your project root for Trunk:

```html
<!DOCTYPE html>
<html lang="en" data-theme="ferrox-cyber">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferrox Front App</title>
    <link rel="rust" data-trunk rel="wasm" />
</head>
<body style="margin:0; background: var(--bg-primary); color: var(--text-primary); font-family: sans-serif;">
    <div id="root"></div>
</body>
</html>
```

---

## 🦀 4. Write Application Code (`src/lib.rs`)

Replace the contents of `src/lib.rs` with:

```rust
use wasm_bindgen::prelude::*;
use ferrox_front_core::dom::{mount, div, button, h1, p};
use ferrox_front_core::reactivity::create_signal;
use ferrox_front_core::theme::set_theme;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_ui::components::{card, card_header, card_body, alert};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let (count, set_count) = create_signal(0);

    let app = container()
        .child(h1().text("⚡ Ferrox Front Wasm App"))
        .child(alert("Application booted successfully in WebAssembly!", "success"))
        .child(
            row().child(
                col("12 md-6").child(
                    card()
                        .child(card_header("Reactive Counter"))
                        .child(
                            card_body()
                                .child(p().text(&format!("Current count: {}", count.get())))
                                .child(
                                    button()
                                        .attr("class", "ferrox-btn")
                                        .text("Increment")
                                        .on_click(move || set_count.update(|n| *n += 1))
                                )
                        )
                )
            )
        );

    mount("#root", app);
    Ok(())
}
```

---

## 🖥️ 5. Run the Local Development Server

Execute the Trunk serve command:

```bash
trunk serve --open
```

Trunk will compile your Rust code into WebAssembly, launch a local web server with Hot Reloading, and open your default browser at `http://localhost:8080`.
