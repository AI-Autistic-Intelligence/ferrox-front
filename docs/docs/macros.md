---
id: macros
title: Procedural RSX Macro (ferrox-front-macro)
sidebar_position: 3
---

# 🚀 Procedural `rsx!` Macro (`ferrox-front-macro`)

`ferrox-front-macro` provides the `rsx!` procedural macro, enabling JSX/HTML-like component syntax directly inside Rust code.

---

## ⚡ 1. How `rsx!` Works

At compile time, `rsx!` parses HTML-style markup into a syntax tree (AST) and transforms it into zero-copy `DomBuilder` calls.

```rust
use ferrox_front_macro::rsx;

let view = rsx! {
    <div class="card p-4">
        <h1>"Header Title"</h1>
        <p>"Body text paragraph"</p>
    </div>
};
```

---

## 🛠️ 2. Dynamic Expressions & Signal Binding

Insert dynamic Rust expressions using curly braces `{}`:

```rust
let (username, _) = create_signal("Alice".to_string());

let view = rsx! {
    <div class="user-profile">
        <span>"Welcome back, " {username.get()} "!"</span>
    </div>
};
```

---

## 🔒 3. Compile-Time Diagnostics

Because `rsx!` is processed during `cargo check`, unclosed tags, malformed attributes, and invalid types produce instant compiler errors before your application is ever deployed to Wasm.
