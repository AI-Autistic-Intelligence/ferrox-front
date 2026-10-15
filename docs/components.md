---
id: components
title: Enterprise UI Component Catalog
sidebar_position: 3
---

# Enterprise Component Catalog (`ferrox-front-ui`)

The `ferrox-front-ui` crate implements an enterprise-grade, strongly-typed Design System conforming to WCAG 2.1 AA accessibility guidelines. All components support rich visual variants, micro-animations, accessible `:focus-visible` focus rings, and glassmorphism styling.

---

## 🎛️ Advanced Buttons (`button_advanced`)

The advanced button component delivers a versatile interface supporting diverse sizing variants, visual treatments, and native loading states:

```rust
use ferrox_front_ui::components::button_advanced;

// Primary Action Button with icon
let btn_submit = button_advanced("Submit Order", "primary", "md", false, Some("zap"))
    .on_click(|| println!("Order submitted!"));

// Glassmorphism Button with active loading spinner
let btn_loading = button_advanced("Processing...", "glass", "sm", true, None);

// Destructive Outline Button
let btn_delete = button_advanced("Cancel", "outline", "xs", false, Some("trash"));
```

### Supported Parameters

| Parameter | Type | Supported Values | Description |
| :--- | :--- | :--- | :--- |
| `text` | `&str` | Any string | Button label text |
| `variant` | `&str` | `"primary"`, `"outline"`, `"glass"`, `"ghost"`, `"danger"`, `"success"` | Visual presentation variant |
| `size` | `&str` | `"xs"`, `"sm"`, `"md"`, `"lg"` | Padding and typography scale |
| `is_loading` | `bool` | `true`, `false` | Renders a native SVG spinner and disables click events |
| `icon` | `Option<&str>` | Icon name (e.g., `"zap"`, `"search"`, `"download"`) | SVG icon prepended to the label |

---

## 📝 Form Controls & Reactive Validation

### 1. Advanced Input (`input_advanced`)

The enterprise input control supports prefix/suffix icons, reactive validation feedback, and instant clear triggers:

```rust
use ferrox_front_ui::components::input_advanced;

let price_input = input_advanced(
    "Limit Price (USD)",
    "price-input",
    "number",
    "97050.00",
    "Enter limit price in USD...",
    Some("dollar-sign"),
    Some("Price conforms to minimum tick size of $0.50"),
    false, // is_valid
    false, // is_invalid
);
```

### 2. Validated FormField & RuleBuilder (`form_engine`)

The `form_engine` module provides a declarative validation pipeline evaluating rules reactively on every keystroke:

```rust
use ferrox_front_ui::form_engine::{FormField, RuleBuilder};

let email_field = FormField::new(
    "user-email",
    "email",
    "Corporate Email Address",
    "".to_string(),
    RuleBuilder::new()
        .required("Email address is required.")
        .email("Please provide a valid email format (e.g., trader@ferrox.exchange)")
        .build(),
).with_placeholder("name@company.com");
```

---

## 🔄 Toggle Switches & State Controls

```rust
use ferrox_front_ui::components::switch_toggle;

// Toggle for trailing stop-loss activation or dark mode
let stop_loss_switch = switch_toggle(
    "trailing-sl",
    "Trailing Stop-Loss Active",
    true, // checked
    |is_checked| {
        println!("Trailing SL state changed to: {}", is_checked);
    },
);
```

---

## 🪟 Modals & Slide-Over Drawers

### 1. Modal Dialog

Ferrox Front modals mount directly into the DOM with glassmorphism backdrop-blur overlays and accessible keyboard focus traps:

```rust
use ferrox_front_ui::components::modal_dialog;
use ferrox_front_core::dom::{p, span};

let confirmation_modal = modal_dialog(
    "Confirm Market Order Execution",
    p().text("Are you sure you want to execute 0.5 BTC at market with an estimated fill price of $97,050.00 USD?"),
    "Execute Now",
    "Cancel",
    || println!("Order confirmed!"),
    || println!("Operation cancelled"),
);
```

### 2. Slide-Over Drawer

Drawers slide smoothly from the right or left viewport boundary with hardware-accelerated CSS transforms, ideal for configuration panels and order book filter drawers:

```rust
use ferrox_front_ui::components::drawer_panel;

let settings_drawer = drawer_panel(
    "Trading Desk Configuration",
    p().text("Tune matching engine thresholds and minimum order book volume filters."),
    true, // is_open
    || println!("Drawer closed"),
);
```

---

## 🎨 Pure Rust 60+ SVG Vector Icon Set (`Icon::render`)

Ferrox Front includes a compiled vector icon catalog embedded directly inside the Rust binary, with zero runtime HTTP requests and zero external font file dependencies:

```rust
use ferrox_front_ui::icons::Icon;

// Render icon with custom scale and hex color
let icon_zap = Icon::render("zap", 16, "#818cf8");
let icon_trend_up = Icon::render("trend-up", 20, "#10b981");
let icon_layers = Icon::render("layers", 18, "#38bdf8");
let icon_chart = Icon::render("chart-candlestick", 24, "#f59e0b");
```

### Primary Available Icon Categories
- **Finance & Trading**: `trend-up`, `trend-down`, `dollar-sign`, `credit-card`, `activity`, `chart-candlestick`, `chart-bar`, `layers`.
- **Navigation & Actions**: `search`, `settings`, `trash`, `edit`, `check`, `x`, `chevron-right`, `chevron-left`, `menu`, `external-link`.
- **Data & Files**: `folder`, `file-text`, `database`, `download`, `upload`, `filter`, `sort-asc`, `sort-desc`.
- **System & Status**: `cpu`, `zap`, `lock`, `shield`, `info`, `alert-circle`, `palette`, `moon`, `sun`.
