---
id: routing
title: SPA Routing & Micro-Frontends
sidebar_position: 5
---

# 🛣️ Client-Side Router & Micro-Frontends

`ferrox-front-router` provides client-side SPA routing backed by the browser History API without page reloads.

---

## 🧭 1. SPA Router Usage

```rust
use ferrox_front_router::Router;

let router = Router::new();
let current_path = router.current_route.get();

let view = match current_path.as_str() {
    "/" => home_view(),
    "/dashboard" => dashboard_view(),
    "/analytics" => analytics_view(),
    _ => not_found_view(),
};
```

---

## 🧩 2. Micro-Frontends (Widget Embedding)

Ferrox Front allows exporting individual widgets to embed into existing websites (WordPress, static HTML, Hugo) without loading an entire Single Page Application:

```rust
use wasm_bindgen::prelude::*;
use ferrox_front_core::dom::mount;
use ferrox_front_templates::pricing_page;

#[wasm_bindgen]
pub fn mount_pricing_widget(selector: &str) {
    mount(selector, pricing_page());
}
```

In external HTML files:
```html
<script type="module">
  import init, { mount_pricing_widget } from './pkg/admin_dashboard.js';
  async function run() {
    await init();
    mount_pricing_widget('#pricing-container');
  }
  run();
</script>
<div id="pricing-container"></div>
```
