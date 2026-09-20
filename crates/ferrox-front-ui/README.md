# Ferrox Front UI (`ferrox-front-ui`)

`ferrox-front-ui` provides high-performance enterprise UI components for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-ui` crate is the primary user interface component library for the Ferrox Front framework. It provides a robust, responsive 12-column Grid system, layout primitives (`Container`, `Row`, `Col`), and complex high-performance widgets such as the virtualized `DataGrid`. Its architectural purpose is to supply developers with pre-built, aesthetically pleasing, and highly optimized components out-of-the-box, accelerating enterprise application development without sacrificing Wasm performance.

## 2. Architectural Layering
This crate sits above `ferrox-front-core` and `ferrox-front-macro` in the stack. It consumes the reactive signals and the `rsx!` macro to build its internal component tree. By decoupling these high-level layout elements from the core, developers can choose to use the `ferrox-front-ui` standard library or bypass it entirely to build custom DOM structures using just the core primitives. 

## 3. How It Works Under the Hood
Components in `ferrox-front-ui` are implemented as standard Rust functions returning `Element`. The `DataGrid` component, for instance, utilizes viewport intersection observation and scroll event tracking to implement row virtualization. Instead of rendering 1,000,000 DOM rows, it only mounts the ~20 rows currently visible on screen, updating their underlying text nodes as the user scrolls. This logic is tightly coupled with `ferrox-front-core` signals for maximum rendering speed.

## 4. Why It Was Designed This Way
Enterprise dashboards often require rendering massive datasets or complex nested layouts. Using standard HTML tables for large datasets in the browser causes severe DOM bloat, leading to crashes or massive memory spikes. This crate was designed to solve this by providing virtualization by default. The Grid system was designed to mimic the familiar Bootstrap/Tailwind mental model (12 columns) but implemented purely in Rust/Wasm for seamless integration with the `rsx!` macro.

## 5. Practical Usage Guide & Extended Code Examples
Use the UI components to construct responsive layouts and data tables.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_ui::prelude::*;

#[component]
pub fn DashboardView() -> Element {
    let massive_dataset = vec!["Row 1", "Row 2", "Row 3"]; // Imagine 1M rows

    rsx! {
        Container { fluid: true,
            Row {
                Col { md: 4, sm: 12,
                    div { class: "glass-card", "Sidebar" }
                }
                Col { md: 8, sm: 12,
                    DataGrid { 
                        data: massive_dataset,
                        columns: vec!["Data"],
                        row_height: 40.0,
                    }
                }
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Deeply Nested Grids without Bounds**
> Do not nest `Container` -> `Row` -> `Col` excessively (e.g., 10 levels deep) to achieve minor pixel padding. While Wasm is fast, excessive DOM node depth still forces the browser's layout engine to work harder during reflows. Use CSS grid or flexbox utility classes for micro-layouts inside a `Col`.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Fluid Containers for Dashboards**
> For admin panels and observability dashboards, always use `<Container fluid=true>` instead of a fixed-width container. This ensures the 12-column grid utilizes 100% of the available screen real estate, which is critical for displaying wide data tables and side-by-side charts effectively.
