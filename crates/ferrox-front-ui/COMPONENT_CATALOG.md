# Ferrox Front UI: Enterprise Component Catalog & Storybook Manual

This document provides a comprehensive reference manual for all UI components in `ferrox-front-ui`.
Every component is written in 100% pure Rust WebAssembly, zero-JavaScript, and zero-npm.

---

## Table of Contents
1. [Button & Button Group](#1-button--button-group)
2. [Input & Form Controls](#2-input--form-controls)
3. [Textarea with Character Counter](#3-textarea-with-character-counter)
4. [Switch Toggle](#4-switch-toggle)
5. [Checkbox & Radio Group](#5-checkbox--radio-group)
6. [Select Dropdown & Range Slider](#6-select-dropdown--range-slider)
7. [Modal & Dialog Overlays](#7-modal--dialog-overlays)
8. [Sliding Drawer / Offcanvas](#8-sliding-drawer--offcanvas)
9. [Accordion Sections](#9-accordion-sections)
10. [Tabs Navigation](#10-tabs-navigation)
11. [Toast Notifications](#11-toast-notifications)
12. [Skeletons & Progress Rings](#12-skeletons--progress-rings)
13. [Navbar, Sidebar, Breadcrumb & Pagination](#13-navbar-sidebar-breadcrumb--pagination)
14. [60+ Native Rust SVG Icon Engine](#14-60-native-rust-svg-icon-engine)

---

### 1. Button & Button Group

#### `button(text, variant)`
Renders a standard button with medium size.
```rust
use ferrox_front_ui::components::button;

let btn = button("Save Changes", "primary");
```

#### `button_advanced(text, variant, size, loading, icon_name)`
Full-featured enterprise button with size variants and micro-spinner loading state.
```rust
use ferrox_front_ui::components::button_advanced;

// Primary with micro-spinner
let loading_btn = button_advanced("Submitting...", "primary", "md", true, None);

// Danger with trash icon
let delete_btn = button_advanced("Purge Cluster", "danger", "sm", false, Some("trash"));
```
- **Variants**: `"primary"`, `"secondary"`, `"outline"`, `"glass"`, `"danger"`, `"ghost"`, `"link"`.
- **Sizes**: `"xs"` (24px), `"sm"` (32px), `"md"` (40px), `"lg"` (48px).

#### `button_group(buttons)`
Segmented button container.
```rust
use ferrox_front_ui::components::{button_group, button_advanced};

let timeframe_group = button_group(vec![
    button_advanced("1M", "secondary", "sm", false, None),
    button_advanced("5M", "secondary", "sm", false, None),
    button_advanced("1H", "primary", "sm", false, None),
    button_advanced("1D", "secondary", "sm", false, None),
]);
```

---

### 2. Input & Form Controls

#### `input_advanced(config)`
Advanced input supporting icon prefixes/suffixes, clear button, helper text, and validity styling.
```rust
use ferrox_front_ui::components::{input_advanced, InputConfig};

let email_input = input_advanced(InputConfig {
    input_type: "email".into(),
    placeholder: "user@enterprise.dev".into(),
    value: "".into(),
    prefix_icon: Some("mail".into()),
    suffix_icon: Some("check".into()),
    is_valid: Some(true),
    helper_text: Some("Verified enterprise email domain.".into()),
    clearable: true,
});
```

---

### 3. Textarea with Character Counter

#### `textarea_advanced(id, placeholder, max_chars, value)`
Textarea with live character counter indicator.
```rust
use ferrox_front_ui::components::textarea_advanced;

let audit_box = textarea_advanced(
    "audit_notes",
    "Enter compliance notes...",
    250,
    "Cluster verified and audited under SOC2 Type II."
);
```

---

### 4. Switch Toggle

#### `switch_toggle(id, checked, label_text)`
Animated toggle switch with 60fps CSS transition.
```rust
use ferrox_front_ui::components::switch_toggle;

let live_toggle = switch_toggle("sw_live", true, "Real-time Order Book Streaming");
```

---

### 5. Checkbox & Radio Group

#### `checkbox_advanced(id, label_text, checked, indeterminate)`
Custom vector checkbox supporting checked, unchecked, and indeterminate states.
```rust
use ferrox_front_ui::components::checkbox_advanced;

let cb_all = checkbox_advanced("cb_select_all", "Select All Nodes", false, true);
let cb_item = checkbox_advanced("cb_node_1", "Node-01 US-East", true, false);
```

#### `radio_group(name, options, selected_value)`
Stylized radio options with glowing ring indicators.
```rust
use ferrox_front_ui::components::radio_group;

let radio = radio_group(
    "deployment_mode",
    vec![
        ("standalone", "Single-Node Standalone"),
        ("cluster", "High-Availability Cluster (Raft)"),
    ],
    "cluster"
);
```

---

### 6. Select Dropdown & Range Slider

#### `select_dropdown(id, options, selected_value)`
Enterprise dropdown select.
```rust
use ferrox_front_ui::components::select_dropdown;

let runtime_select = select_dropdown(
    "sel_engine",
    vec![
        ("rust", "🦀 Rust Matching Engine (LOB)"),
        ("java", "☕ Java 21 Project Loom"),
    ],
    "rust"
);
```

#### `range_slider_with_bubble(id, min, max, value, unit)`
Interactive range slider with value bubble positioned over the thumb.
```rust
use ferrox_front_ui::components::range_slider_with_bubble;

let slider = range_slider_with_bubble("rng_mem", 0, 100, 75, "%");
```

---

### 7. Modal & Dialog Overlays

#### `modal_advanced(id, title, content, footer, size)`
Modal dialog with blurred backdrop filter, size configurations, and close triggers.
```rust
use ferrox_front_core::dom::{p, div};
use ferrox_front_ui::components::{modal_advanced, button};

let modal_dialog = modal_advanced(
    "modal_node_deploy",
    "Deploy Edge Node",
    p().text("Confirm deployment of new edge node to us-west-1 region?"),
    Some(div().child(button("Cancel", "ghost")).child(button("Confirm Deploy", "primary"))),
    "md" // "sm", "md", "lg", "xl", "fullscreen"
);
```

---

### 8. Sliding Drawer / Offcanvas

#### `drawer(id, title, content, side)`
Sliding panel from the left or right of the screen.
```rust
use ferrox_front_core::dom::p;
use ferrox_front_ui::components::drawer;

let right_panel = drawer(
    "drawer_trace",
    "Distributed Trace #8401",
    p().text("Full span execution trace details across Rust and Java runtimes."),
    "right" // "left" or "right"
);
```

---

### 9. Accordion Sections

#### `accordion(items)`
Collapsible panels with rotating SVG chevrons.
```rust
use ferrox_front_core::dom::p;
use ferrox_front_ui::components::{accordion, AccordionItem};

let acc = accordion(vec![
    AccordionItem {
        title: "⚡ Zero-Trust Architecture".into(),
        content: p().text("Every inter-service communication is verified with PASETO v4."),
        is_expanded: true,
    },
    AccordionItem {
        title: "🛡️ Sentinel Threat Engine".into(),
        content: p().text("Intercepts anomalous Shannon entropy on all REST payloads."),
        is_expanded: false,
    },
]);
```

---

### 10. Tabs Navigation

#### `tabs(items, active_id, variant)`
Reactive tabs with `"line"`, `"pills"`, or `"segmented"` styles.
```rust
use ferrox_front_ui::components::{tabs, TabItem};

let nav_tabs = tabs(
    vec![
        TabItem { id: "tab_kpi".into(), label: "KPIs".into(), icon: Some("activity".into()) },
        TabItem { id: "tab_logs".into(), label: "Audit Logs".into(), icon: Some("server".into()) },
    ],
    "tab_kpi",
    "segmented" // "line", "pills", "segmented"
);
```

---

### 11. Toast Notifications

#### `toast(title, message, variant)`
Glassmorphic toast notification with auto-shrinking timed progress bar.
```rust
use ferrox_front_ui::components::toast;

let success_toast = toast("Order Matched", "LOB transaction executed in 0.04ms", "success");
let alert_toast = toast("WAF Anomaly", "High Shannon entropy detected from 198.51.100.4", "danger");
```

---

### 12. Skeletons & Progress Rings

#### `skeleton(variant)`
Shimmering pulse loading placeholders.
```rust
use ferrox_front_ui::components::skeleton;

let sk_avatar = skeleton("avatar");
let sk_title = skeleton("title");
let sk_text = skeleton("text");
let sk_card = skeleton("card");
```

#### `progress_bar_circular(percentage, size)`
Native SVG circular progress ring.
```rust
use ferrox_front_ui::components::progress_bar_circular;

let ring = progress_bar_circular(78.5, 80);
```

---

### 13. Navbar, Sidebar, Breadcrumb & Pagination

#### `navbar(brand_name, links)`
```rust
use ferrox_front_ui::components::navbar;

let top_nav = navbar("Ferrox Enterprise", vec![
    ("/dashboard", "Dashboard"),
    ("/analytics", "Analytics"),
    ("/security", "Security"),
]);
```

#### `pagination(current_page, total_pages)`
```rust
use ferrox_front_ui::components::pagination;

let pager = pagination(1, 100);
```

---

### 14. 60+ Native Rust SVG Icon Engine

Render any vector icon instantly at any size and color:
```rust
use ferrox_front_ui::icons::Icon;

// Direct invocation
let icon_home = Icon::render("home", 24, "var(--text-primary)");
let icon_shield = Icon::render("shield", 20, "#10b981");
let icon_zap = Icon::render("zap", 16, "#f59e0b");
```

#### Complete Icon Catalog:
- **Navigation**: `home`, `menu`, `x`, `chevron_down`, `chevron_up`, `chevron_left`, `chevron_right`, `search`, `filter`
- **Actions**: `plus`, `minus`, `trash`, `edit`, `copy`, `check`, `refresh`, `download`, `upload`, `share`, `external_link`
- **Security & System**: `shield`, `lock`, `unlock`, `key`, `eye`, `eye_off`, `alert_circle`, `alert_triangle`, `check_circle`, `info`
- **Data & Charts**: `bar_chart`, `line_chart`, `pie_chart`, `activity`, `trending_up`, `trending_down`, `layers`, `cpu`, `database`, `server`
- **User & Commerce**: `user`, `users`, `bell`, `mail`, `settings`, `credit_card`, `shopping_cart`, `dollar_sign`, `package`, `zap`

---

### 15. Enterprise DataGrid Pro (AG-Grid / TanStack Table Parity)

#### Typed Column Definitions (`ColumnDef<T>`)
```rust
use ferrox_front_ui::ferrox_datagrid::{
    EnterpriseDataGrid, ColumnDef, PinnedPosition, TextAlign,
    generate_rfc4180_csv, trigger_csv_download,
};

let columns = vec![
    ColumnDef::new("id", "Order ID", |r: &OrderRow| r.id.clone())
        .with_width("130px")
        .with_pinned(PinnedPosition::Left)
        .with_renderer(|r: &OrderRow| {
            span().class("font-mono font-bold text-indigo-400 text-xs").text(&r.id)
        }),
    ColumnDef::new("price", "Price (USD)", |r: &OrderRow| format!("${:.2}", r.price))
        .with_width("120px")
        .with_align(TextAlign::Right),
    ColumnDef::new("status", "Status", |r: &OrderRow| r.status.clone())
        .with_width("110px")
        .with_align(TextAlign::Center)
        .with_renderer(|r: &OrderRow| badge(&r.status, "primary")),
];

let datagrid = EnterpriseDataGrid::new("orders-grid", columns, orders);
let table_node = datagrid.render();
```

#### Native In-Memory WebAssembly RFC 4180 CSV Export
```rust
// Generates strict RFC 4180 CSV in WebAssembly linear memory and triggers browser Blob download
let csv_content = generate_rfc4180_csv(&headers, &rows);
trigger_csv_download("orders_export.csv", &csv_content);
```

---

### 16. Form Reactive Validation & State Engine (Zod + React Hook Form Parity)

#### Reactive `FormField<T>` & Chainable `RuleBuilder`
```rust
use ferrox_front_ui::form_engine::{FormField, RuleBuilder};

let email_field = FormField::new(
    "email-input",
    "email",
    "Email Aziendale Verificata",
    String::new(),
    RuleBuilder::new()
        .required("L'indirizzo email è obbligatorio.")
        .email("Formato email non conforme.")
        .build(),
).with_input_type("email").with_placeholder("finance@company.com");

// Renders with WCAG 2.1 AA :focus-visible, aria-invalid, aria-describedby, and alert icons:
let field_dom = email_field.render();
```

#### Multi-Step `FormWizard`
```rust
use ferrox_front_ui::form_engine::{WizardStep, FormWizardState, form_wizard};

let step1 = WizardStep::new("step-1", "Dati Personali", "Inserisci credenziali", || true, || div());
let step2 = WizardStep::new("step-2", "Parametri Finanziari", "Definisci capitale", || true, || div());

let wizard_state = FormWizardState::new(vec![step1, step2]);
let wizard_dom = form_wizard(wizard_state, || {
    // Execution callback upon final step completion
});
```

---

### 17. Advanced Enterprise Overlays (Spotlight Cmd+K, DatePicker & TreeView)

#### Spotlight Command Palette (`Cmd+K` / `Ctrl+K`)
```rust
use ferrox_front_ui::overlays::{CommandAction, CommandPaletteState, command_palette};

let actions = vec![
    CommandAction::new("new-order", "Nuovo Ordine", "Invia un ordine di acquisto", "Azioni Rapide", "plus", Some("N O"), || {}),
    CommandAction::new("charts", "Vai a Grafici", "Analisi tecnica e oscillatori", "Navigazione", "activity", Some("G C"), || {}),
];

let state = CommandPaletteState::new(actions);
let modal_dom = command_palette(state);
```

#### Pure Vector DatePicker (Single & DateRange)
```rust
use ferrox_front_ui::overlays::{DatePickerState, date_picker, DatePickerSelection};

let state = DatePickerState::new(2026, 10, true /* range mode */);
let picker = date_picker(state, |selection| {
    match selection {
        DatePickerSelection::Single(d) => println!("Data: {}", d),
        DatePickerSelection::Range(s, e) => println!("Intervallo: {} -> {}", s, e),
    }
});
```

#### Hierarchical TreeView Navigator
```rust
use ferrox_front_ui::overlays::{TreeNode, tree_view};

let nodes = vec![
    TreeNode::new("src", "src")
        .with_icon("folder")
        .expanded(true)
        .with_children(vec![
            TreeNode::new("lib", "lib.rs").with_icon("code"),
        ]),
];

let tree_dom = tree_view(nodes, |node_id| {
    println!("Node selected: {}", node_id);
});
```

---

### 18. Advanced Financial Technical Analysis Charts (TradingView Parity)

```rust
use ferrox_front_charts::{
    CandleDatum, CandleRingBuffer, TechnicalChartOptions, financial_technical_chart,
};

let mut ring_buffer = CandleRingBuffer::new(50);
ring_buffer.push(CandleDatum {
    time: "10:00".into(), open: 95000.0, high: 95500.0, low: 94800.0, close: 95300.0, volume: 120.0
});

let opts = TechnicalChartOptions {
    show_sma20: true,
    show_sma50: true,
    show_bollinger: true,
    show_rsi: true,
    show_macd: true,
    show_crosshair: true,
};

let chart_dom = financial_technical_chart("pro-chart", ring_buffer.as_slice(), &opts, 1100, 520);
```

