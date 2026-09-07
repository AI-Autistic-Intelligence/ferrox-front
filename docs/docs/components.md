---
id: components
title: Component Library & Interactive Playground
sidebar_position: 6
---

# 🎨 Component Library & Interactive Playground

Explore and interact with the **Ferrox Front** component library live in your browser using our **Nivo-Style Interactive Playground**.

You can edit JSON datasets live in real-time, adjust dynamic sliders (stroke width, curve styles, row counts), toggle glassmorphism themes, simulate **Zero-Trust RBAC security actions**, and inspect auto-generated Rust component code (`rsx!`).

---

## 🎮 Live Interactive Playground

import useBaseUrl from '@docusaurus/useBaseUrl';

<iframe 
    src={useBaseUrl('/playground.html')} 
    style={{width: '100%', height: '880px', border: '1px solid var(--border-glass, #333)', borderRadius: '12px', boxShadow: '0 8px 32px rgba(0,0,0,0.4)'}} 
    title="Ferrox Front Interactive Component Playground"
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

The `ferrox-front-ui` crate provides dozens of enterprise-grade components. Below are examples of how to instantiate them directly in your Rust Wasm front-end.

### 📐 1. Responsive Grid Layout

```rust
use ferrox_front_ui::layout::{container, row, col};

container()
    .child(
        row()
            .child(col("12 md-6 lg-4").child(/* Column content */))
            .child(col("12 md-6 lg-8").child(/* Column content */))
    )
```

### 🃏 2. Glassmorphism Card

```rust
use ferrox_front_ui::components::{card, card_header, card_body, card_footer};

card()
    .child(card_header("Account Settings"))
    .child(card_body().text("Manage your account preferences here."))
    .child(card_footer().child(/* Add buttons here */))
```

### 🚨 3. Notification Alerts

```rust
use ferrox_front_ui::components::alert;

// Available variants: "success", "danger", "warning", "info"
alert("Operation completed successfully!", "success");
alert("Network connection lost. Retrying...", "warning");
```

### 🎯 4. Interactive Buttons

```rust
use ferrox_front_ui::components::button;

// Available variants: "primary", "secondary", "outline", "glass", "danger", "ghost"
button("Submit Form", "primary");
button("Cancel", "ghost");
button("Delete Account", "danger");
```

### 🏷️ 5. Status Badges

```rust
use ferrox_front_ui::components::badge;

// Available variants: "primary", "success", "warning", "danger", "info"
badge("ACTIVE", "success");
badge("PENDING", "warning");
badge("ERROR", "danger");
```

### 📝 6. Form Inputs & Groups

```rust
use ferrox_front_ui::components::{form_group, input, textarea, select};

form_group("Email Address", input("email", "john@example.com"));
form_group("Biography", textarea("Tell us about yourself...", 4));

// Select Dropdown
form_group("Country", select(vec![
    ("US", "United States"),
    ("UK", "United Kingdom"),
    ("IT", "Italy")
]));
```

### 🔘 7. Toggles, Checkboxes & Radios

```rust
use ferrox_front_ui::components::{toggle_switch, checkbox, radio};

toggle_switch("Enable Dark Mode", true);
checkbox("I accept the Terms and Conditions", false);
radio("payment_method", "credit_card", "Credit Card", true);
```

### 🎚️ 8. Range Sliders & Progress Bars

```rust
use ferrox_front_ui::components::{progress_bar, range_slider};

// Progress bar (0.0 to 100.0)
progress_bar(75.5);

// Range slider (min, max, value)
range_slider(0, 100, 42);
```

### 👤 9. Avatars & User Pills

```rust
use ferrox_front_ui::components::{avatar, user_pill};

avatar("JD");
user_pill("Jane Doe", "Administrator");
```

### 📁 10. File Upload Dropzone

```rust
use ferrox_front_ui::components::file_upload;

file_upload("image/png, image/jpeg, application/pdf");
```

### ⏳ 11. Loading Spinners

```rust
use ferrox_front_ui::components::spinner;

spinner();
```

### 📊 12. Virtualized DataGrid

`virtual_data_grid` renders tables with 1,000,000+ rows instantly by calculating viewport scroll offsets and mounting only visible rows into the DOM.

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
