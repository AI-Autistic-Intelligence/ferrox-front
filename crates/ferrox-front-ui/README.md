# Ferrox Front UI (`ferrox-front-ui`)

Enterprise-grade Glassmorphism Design System, Advanced Typed DataGrid, Reactive Form Validation Engine, and Overlays in 100% pure Rust WebAssembly.
Completely independent of JavaScript, React, and npm.

## 1. Architectural Highlights
`ferrox-front-ui` reaches and exceeds **AG-Grid, React Hook Form, Zod, Radix UI, and Shadcn feature parity** compiled to native WebAssembly:
- **100% Rust Wasm**: Generates surgical DOM mutations through `ferrox-front-core::DomBuilder` and `Signal<T>` with zero JavaScript runtime frameworks or node_modules.
- **Enterprise DataGrid Pro (AG-Grid Parity)**: Typed `ColumnDef<T>`, multi-column sorting, column filters, checkbox row selection with batch actions, and in-memory WebAssembly RFC 4180 CSV export with native browser `Blob` download.
- **Form Reactive Validation Engine (Zod + React Hook Form Parity)**: `FormField<T>` signals (`value`, `error`, `touched`, `dirty`, `is_valid`), chainable `RuleBuilder<T>` (`required`, `email`, `min_length`, `max_length`, `numeric_range`, `regex`, `custom`), accessible WCAG 2.1 AA `:focus-visible` styling with alert icons, and multi-step `FormWizard`.
- **Advanced Enterprise Overlays**:
  - **Command Palette (`Cmd+K` / `Ctrl+K`)**: Spotlight-style modal search with global window keyboard shortcut, fuzzy matching, and keyboard navigation.
  - **Pure Vector DatePicker**: Monthly interactive calendar generated in pure Rust Wasm with single and range (`DateRange`) picking, month/year navigation, and quick shortcuts ("Oggi", "7 Giorni", "30 Giorni", "Mese").
  - **TreeView Explorer**: Deep hierarchical tree navigator for files, directories, and role permissions with smooth chevron expansion and multi-select.
  - **A11y Modal Focus Trap**: Cyclic Tab and Shift-Tab focus management conforming to WCAG 2.1 AA.
- **60+ Native Vector SVG Icons**: Module `ferrox_front_ui::icons` renders icons directly as SVG DOM elements with zero font files or external CDNs.
- **Real-time Theming Engine**: Dynamic switching between Glassmorphism Dark, Zinc Light, Cyberpunk Neon, and Ocean Breeze.

---

## 2. Component Inventory

### 📊 Enterprise DataGrid (`ferrox_front_ui::ferrox_datagrid`)
```rust
use ferrox_front_ui::ferrox_datagrid::{
    EnterpriseDataGrid, ColumnDef, PinnedPosition, TextAlign,
    generate_rfc4180_csv, trigger_csv_download,
};

let columns = vec![
    ColumnDef::new("id", "Order ID", |r: &OrderRow| r.id.clone())
        .with_width("130px")
        .with_pinned(PinnedPosition::Left),
    ColumnDef::new("price", "Price (USD)", |r: &OrderRow| format!("${:.2}", r.price))
        .with_width("120px")
        .with_align(TextAlign::Right),
    ColumnDef::new("status", "Status", |r: &OrderRow| r.status.clone())
        .with_align(TextAlign::Center),
];

let datagrid = EnterpriseDataGrid::new("orders-grid", columns, orders);
let table_element = datagrid.render();

// Native Wasm RFC 4180 CSV Download:
let csv_text = generate_rfc4180_csv(&headers, &rows);
trigger_csv_download("orders_export.csv", &csv_text);
```

### 📝 Reactive Form Validation Engine (`ferrox_front_ui::form_engine`)
```rust
use ferrox_front_ui::form_engine::{FormField, RuleBuilder, WizardStep, FormWizardState, form_wizard};

// Reactive FormField with chainable validation rules
let email_field = FormField::new(
    "user-email",
    "email",
    "Email Aziendale",
    String::new(),
    RuleBuilder::new()
        .required("L'indirizzo email è obbligatorio.")
        .email("Formato email non valido.")
        .build(),
).with_input_type("email").with_placeholder("name@company.com");

let input_dom = email_field.render();

// Multi-step FormWizard with step-gated validation
let wizard_state = FormWizardState::new(vec![step1, step2, step3]);
let wizard_dom = form_wizard(wizard_state, || {
    // Called when all steps pass validation
});
```

### ⌘ Spotlight Command Palette (`ferrox_front_ui::overlays`)
```rust
use ferrox_front_ui::overlays::{CommandAction, CommandPaletteState, command_palette};

let actions = vec![
    CommandAction::new("act-home", "Vai alla Home", "Ritorna alla dashboard principale", "Navigazione", "home", Some("G H"), || {
        // action logic
    }),
    CommandAction::new("act-csv", "Esporta Dati CSV", "Scarica report ordini in formato CSV", "Azioni Rapide", "download", Some("E C"), || {
        // export logic
    }),
];

let palette_state = CommandPaletteState::new(actions);
// Opens via Cmd+K / Ctrl+K automatically or manually:
palette_state.open();

let overlay_dom = command_palette(palette_state);
```

### 📅 Pure Vector DatePicker (`ferrox_front_ui::overlays`)
```rust
use ferrox_front_ui::overlays::{DatePickerState, date_picker, DatePickerSelection};

let date_state = DatePickerState::new(2026, 10, true /* range mode */);
let picker_dom = date_picker(date_state, |selection| {
    match selection {
        DatePickerSelection::Single(d) => println!("Selected: {}", d),
        DatePickerSelection::Range(start, end) => println!("Range: {} to {}", start, end),
    }
});
```

### 📁 Hierarchical TreeView (`ferrox_front_ui::overlays`)
```rust
use ferrox_front_ui::overlays::{TreeNode, tree_view};

let nodes = vec![
    TreeNode::new("src", "src")
        .with_icon("folder")
        .expanded(true)
        .with_children(vec![
            TreeNode::new("main", "main.rs").with_icon("code"),
            TreeNode::new("lib", "lib.rs").with_icon("code"),
        ]),
];

let tree_dom = tree_view(nodes, |node_id| {
    println!("Selected node: {}", node_id);
});
```

---

## 3. Full Storybook Reference
Consult [COMPONENT_CATALOG.md](file:///C:/Users/nn/Desktop/code/ferrox-front/crates/ferrox-front-ui/COMPONENT_CATALOG.md) for the exhaustive documentation and API specifications of every component.
