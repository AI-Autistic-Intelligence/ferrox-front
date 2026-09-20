# Ferrox Front Charts (`ferrox-front-charts`)

`ferrox-front-charts` is a Pure Wasm SVG Charting Engine for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-charts` crate provides a native, WebAssembly-driven data visualization library. It is capable of rendering high-frequency, complex charts (Bar, Line, Pie, Scatter) directly into scalable SVG elements. Its architectural purpose is to eliminate the need for heavy, garbage-collected JavaScript charting libraries (like Chart.js or D3.js) by building the DOM trees for vector graphics directly in Rust, ensuring maximum performance for dashboards and observability tools.

## 2. Architectural Layering
This crate sits at the application layer, utilizing both `ferrox-front-core` (for reactive data binding) and `ferrox-front-macro` (for SVG node generation). It generates `<svg>`, `<rect>`, `<path>`, and `<circle>` tags that are seamlessly injected into the component tree. It operates entirely independently of HTML Canvas, ensuring that charts are resolution-independent and perfectly crisp on high-DPI displays.

## 3. How It Works Under the Hood
When charting data is provided (e.g., via a `Vec<Datum>`), the engine calculates the required pixel scales, axes, grid lines, and data paths mathematically inside Wasm. It then constructs a localized SVG DOM tree using the `rsx!` macro. When the underlying data signal updates (e.g., live streaming telemetry), the reactive graph patches only the specific SVG attributes (like the `d` attribute of a `<path>` or the `height` of a `<rect>`), resulting in buttery-smooth 60 FPS animations without rebuilding the entire chart.

## 4. Why It Was Designed This Way
Integrating JS charting libraries into a Wasm SPA is notoriously difficult and slow, requiring constant serialization of large datasets back and forth across the JS/Wasm boundary via `serde`. By generating pure SVGs natively in Rust, `ferrox-front-charts` keeps all large datasets securely and efficiently inside the Wasm linear memory. SVG also provides native accessibility and CSS styling hooks that Canvas implementations lack.

## 5. Practical Usage Guide & Extended Code Examples
Creating a simple reactive bar chart.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_charts::prelude::*;

#[component]
pub fn ServerLoadChart() -> Element {
    // Reactive signal containing chart data
    let (data, _) = create_signal(vec![
        BarDatum { label: "CPU".into(), value: 45.0 },
        BarDatum { label: "RAM".into(), value: 80.0 },
    ]);

    rsx! {
        div { class: "chart-container",
            BarChart {
                data: data.get(),
                width: 600,
                height: 300,
                color: "#8b5cf6", // Purple theme
                show_axes: true,
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Rendering 100,000+ Data Points in SVG**
> While SVG is highly performant for UI graphs, do not attempt to render raw scatter plots or line charts with hundreds of thousands of individual points. The browser DOM will choke on creating 100,000 `<circle>` tags. For massive high-density datasets, you must downsample the data in Rust before rendering, or switch to a WebGL Canvas implementation.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: CSS Styling for SVG Elements**
> Leverage external CSS to style your charts rather than hardcoding colors in Rust. `ferrox-front-charts` assigns semantic class names (e.g., `.fx-chart-bar`, `.fx-chart-axis`) to its generated SVG nodes. Using CSS allows you to easily implement light/dark mode switching and hover effects without triggering Wasm reactivity cycles.
