---
id: components
title: Component Library & Nivo-Style Interactive Playground
sidebar_position: 6
---

# 🎨 Component Library & Nivo-Style Interactive Playground

Explore and interact with the **Ferrox Front** component library live in your browser using our **Nivo-Style Interactive Playground**.

You can edit JSON datasets live in real-time, adjust dynamic sliders (stroke width, curve styles, row counts), toggle glassmorphism themes, simulate **Zero-Trust RBAC security actions**, and inspect auto-generated Rust component code (`rsx!`).

---

## 🎮 Nivo-Style Live Interactive Playground

import useBaseUrl from '@docusaurus/useBaseUrl';

<iframe 
    src={useBaseUrl('/playground.html')} 
    style={{width: '100%', height: '880px', border: '1px solid var(--border-glass, #333)', borderRadius: '12px', boxShadow: '0 8px 32px rgba(0,0,0,0.4)'}} 
    title="Ferrox Front Nivo-Style Interactive Component Playground"
/>

---

## 🛠️ How to Use the Live Playground

1. **📝 Live JSON Data Editing**: Edit the raw JSON data inside the right control panel textarea (e.g. `[{ "x": 10, "y": 150, "label": "Jan ($150k)" }, ...]`) and click **Apply Live JSON Data** to watch the vector SVG chart update instantly!
2. **📈 Dynamic Curve Controls**: Switch between *Smooth Spline (Bezier)*, *Linear Straight Lines*, and *Stepped Digital Waves*, or adjust the stroke width slider from 1px to 10px.
3. **📊 Virtualized DataGrid Testing**: Switch the DataGrid row count between 500, 10,000, and 1,000,000 enterprise rows to test viewport scrolling performance.
4. **🔒 Zero-Trust RBAC Simulator**: Toggle current user role (`Guest`, `User`, `Admin`) to observe how `<Secure require="admin:delete">` dynamically mounts/unmounts DOM nodes directly in WebAssembly linear memory heap.
5. **🎨 Glassmorphism Theme Switcher**: Click on any theme button (`Cyber`, `Ocean`, `Forest`, `Sunset`, `Corporate`) to switch 60fps CSS variables live.
6. **📜 Rust Code Inspector**: Copy the live-generated Rust code snippet dynamically updated based on your selected dataset and options.

---

## 📚 Component API Reference

### 📐 1. 12-Column Responsive Grid (`layout`)

```rust
use ferrox_front_ui::layout::{container, row, col};

container()
    .child(
        row()
            .child(col("12 md-6 lg-4").child(/* Column content */))
            .child(col("12 md-6 lg-8").child(/* Column content */))
    )
```

### 🃏 2. Glassmorphism Card (`components`)

```rust
use ferrox_front_ui::components::{card, card_header, card_body};

card()
    .child(card_header("Dashboard Title"))
    .child(card_body().text("Isolated card body content."))
```

### 🚨 3. Notification Alerts (`components`)

```rust
use ferrox_front_ui::components::alert;

// Available variants: "success", "danger", "warning", "info"
alert("Operation completed successfully!", "success")
```

### 📊 4. Virtualized DataGrid (`virtual_data_grid`)

`virtual_data_grid` renders tables with 1,000,000+ rows by calculating viewport scroll offsets and mounting only visible rows into the DOM.

```rust
use ferrox_front_ui::{virtual_data_grid, DataGridState};

let state = DataGridState::new(40.0, 15); // 40px row height, 15 visible rows

let columns = vec!["Transaction ID", "Enterprise Client", "Amount", "Status"];
let rows = vec![
    vec!["TX-1001", "Acme Corp", "$12,450.00", "COMPLETED"],
    vec!["TX-1002", "Stark Ind", "$98,100.00", "PROCESSING"],
];

let grid = virtual_data_grid(columns, rows, &state);
```
