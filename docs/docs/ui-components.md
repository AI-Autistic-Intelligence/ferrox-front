---
id: ui-components
title: UI Component Library & Grid (ferrox-front-ui)
sidebar_position: 4
---

# 🎨 UI Component Library & Grid (`ferrox-front-ui`)

`ferrox-front-ui` provides a responsive 12-column layout grid, pre-styled Glassmorphism components, and a **Virtualized DataGrid** capable of rendering 1,000,000+ records.

---

## 📐 1. 12-Column Responsive Layout Grid (`layout`)

```rust
use ferrox_front_ui::layout::{container, row, col};

let layout = container()
    .child(
        row()
            .child(col("12 md-6 lg-4").child(/* Col 1 */))
            .child(col("12 md-6 lg-8").child(/* Col 2 */))
    );
```

| Function | Output Class | Description |
|---|---|---|
| `container()` | `.fx-container` | Centered layout container with max-width limits. |
| `row()` | `.fx-row` | Flexbox row container. |
| `col(spans)` | `.fx-col .fx-col-12 .fx-col-md-6` | Responsive column span calculator. |

---

## 🃏 2. Glassmorphism Components (`components`)

```rust
use ferrox_front_ui::components::{card, card_header, card_body, alert};

// Card Component
let my_card = card()
    .child(card_header("System Overview"))
    .child(card_body().text("All servers operational."));

// Alert Component (variants: "success", "danger", "warning", "info")
let my_alert = alert("Database backup completed.", "success");
```

---

## 📊 3. Virtualized DataGrid (`virtual_data_grid`)

The `virtual_data_grid` mounts only the rows visible inside the current scroll viewport into the DOM, maintaining 60fps performance even with 1,000,000 preloaded data rows.

```rust
use ferrox_front_ui::{virtual_data_grid, DataGridState};

let state = DataGridState::new(40.0, 15); // 40px row height, 15 visible rows

let columns = vec!["Transaction ID", "Customer", "Amount", "Status"];
let rows = vec![
    vec!["TX-1001", "Acme Corp", "$12,450.00", "VERIFIED"],
    vec!["TX-1002", "Stark Ind", "$98,100.00", "PROCESSING"],
];

let table_builder = virtual_data_grid(columns, rows, &state);
```
