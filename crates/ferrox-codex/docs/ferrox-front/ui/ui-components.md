---
id: ui-components
title: Glassmorphism Design System (ferrox-front-ui)
sidebar_position: 4
---

# 🎨 Glassmorphism Design System & UI Components (`ferrox-front-ui`)

`ferrox-front-ui` provides a complete, enterprise-grade **Glassmorphism Design System** for WebAssembly applications. It includes **Design Tokens**, a **12-column responsive layout grid**, a **Virtualized DataGrid** (1,000,000+ rows), and a comprehensive library of pre-styled UI components.

---

## 🎨 Design Tokens (`tokens`)

Design Tokens centralize spacing, border radius, shadows, and glassmorphism backdrop blur properties:

```rust
use ferrox_front_ui::tokens::DesignTokens;

// Spacing: SPACE_XS, SPACE_SM, SPACE_MD, SPACE_LG, SPACE_XL
// Radius: RADIUS_SM, RADIUS_MD, RADIUS_LG, RADIUS_FULL
// Glass FX: BACKDROP_BLUR, BORDER_GLASS, SHADOW_GLASS
```

---

## 🔘 UI Component Library

### 1. Buttons (`button`)

Supports 6 design variants: `"primary"`, `"secondary"`, `"outline"`, `"glass"`, `"danger"`, `"ghost"`.

```rust
use ferrox_front_ui::components::button;

let save_btn = button("Save Changes", "primary");
let cancel_btn = button("Cancel", "secondary");
let delete_btn = button("Delete Account", "danger");
```

---

### 2. Status Badges (`badge`)

Pill badges for status indication: `"primary"`, `"success"`, `"warning"`, `"danger"`, `"info"`.

```rust
use ferrox_front_ui::components::badge;

let active_badge = badge("Active", "success");
let pending_badge = badge("Pending", "warning");
```

---

### 3. Form Controls (`input`, `form_group`)

```rust
use ferrox_front_ui::components::{form_group, input};

let email_input = form_group("Corporate Email", input("email", "user@enterprise.com"));
```

---

### 4. Cards & Panels (`card`)

```rust
use ferrox_front_ui::components::{card, card_header, card_body, card_footer, button};

let my_card = card()
    .child(card_header("Security Controls"))
    .child(card_body().text("Configure Zero-Trust session permissions."))
    .child(card_footer().child(button("Save Settings", "primary")));
```

---

### 5. Notification Alerts (`alert`)

```rust
use ferrox_front_ui::components::alert;

let success_alert = alert("Database synchronization complete.", "success");
let danger_alert = alert("Connection timeout to Redis cluster.", "danger");
```

---

### 6. Progress Bars & Spinners (`progress_bar`, `spinner`)

```rust
use ferrox_front_ui::components::{progress_bar, spinner};

let progress = progress_bar(84.5); // 84.5% filled
let loading_icon = spinner();
```

---

### 7. User Pill & Avatars (`user_pill`, `avatar`)

```rust
use ferrox_front_ui::components::{user_pill, avatar};

let user = user_pill("Sarah Connor", "Security Admin");
let user_avatar = avatar("SC");
```

---

## 📊 Virtualized DataGrid (`virtual_data_grid`)

`virtual_data_grid` mounts only visible viewport rows into the DOM, maintaining 60fps scrolling performance even with 1,000,000 data rows.

```rust
use ferrox_front_ui::{virtual_data_grid, DataGridState};

let state = DataGridState::new(38.0, 15); // 38px row height, 15 visible rows

let columns = vec!["Transaction ID", "Enterprise Client", "Amount", "Status"];
let rows = vec![
    vec!["TX-1001", "Acme Corp", "$12,450.00", "VERIFIED"],
    vec!["TX-1002", "Stark Ind", "$98,100.00", "PROCESSING"],
];

let grid = virtual_data_grid(columns, rows, &state);
```
