---
id: playbook-handbook
title: Interactive Component Playbook & Design System Handbook
sidebar_position: 1
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 📘 Interactive Component Playbook & Design System Handbook

`ferrox-front` is a production-grade WebAssembly frontend ecosystem engineered in **100% pure Rust** with **zero external JavaScript dependencies**. It couples an ultra-fast vector SVG charting engine (`ferrox-front-charts`) with an enterprise Glassmorphism Design System (`ferrox-front-ui`).

This handbook documents the design patterns, mathematical foundations, component APIs, and interactive customization capabilities available across the suite.

---

## 🕹️ Live Interactive Playbook Studio

Interact directly with the WebAssembly microfrontend below to test vector charts, customize theme tokens, and copy synchronized Rust code:

<FerroxWasmPlayground initialTab="chart-studio" height="880px" title="Ferrox Front • Interactive Component Playbook" />

---

## 1. Mathematical Rigor: GaugeMeter Concentric Geometry

The semi-circular Speedometer/GaugeMeter (`gauge_chart_styled`) visualizes telemetry and KPI metrics along a $180^\circ$ semicircle sweeping clockwise from $-\pi$ ($180^\circ$) to $0$ ($0^\circ$).

### The SVG Arc Distortion Bug & Solution
In standard SVG path syntax:
```text
A rx ry x-axis-rotation large-arc-flag sweep-flag x y
```
Because the gauge is constrained to a semicircle ($\Delta\theta \le \pi$), any value arc on this quadrant has an angular span $\le 180^\circ$. Setting `large-arc-flag` to `1` when ratio $> 0.5$ mistakenly instructed SVG renderers to draw the *major arc* ($> 180^\circ$), forcing the stroke to bulge unnaturally outside the track.

**The Fix:**
- The `large-arc` flag is strictly set to `0`:
  $$\Delta\theta \le \pi \implies \text{large-arc} \equiv 0$$
- Track, value arc, needle end, and indicator bead are co-aligned to the exact center $(c_x, c_y)$ and radius $r$:
  $$x_{\text{tip}} = c_x + r \cdot \cos(\theta), \quad y_{\text{tip}} = c_y + r \cdot \sin(\theta)$$
- The indicator bead sits exactly on $(x_{\text{tip}}, y_{\text{tip}})$, guaranteeing 100% concentricity.

```rust
use ferrox_front_charts::{gauge_chart_styled, GaugeChartOptions};

let mut options = GaugeChartOptions::default();
options.warning_threshold = 0.60;
options.critical_threshold = 0.82;
options.custom_color = Some("#6366f1".into());

let gauge = gauge_chart_styled(
    "telemetry-gauge",
    78.4,
    0.0,
    100.0,
    "CPU Utilization",
    "%",
    320,
    180,
    &options,
);
```

---

## 2. Segmented Glass Pill Bar

The component toolbar replaces legacy compressed buttons with a **Segmented Glass Pill Bar** providing unmistakable state contrast and full accessibility compliance (WCAG 2.1 AA):

- **Active State:**
  - High-contrast gradient: `linear-gradient(135deg, var(--accent-primary), var(--accent-secondary))`
  - Luminescent border: `1px solid rgba(255, 255, 255, 0.35)`
  - Accent box shadow: `0 4px 18px rgba(99, 102, 241, 0.45)`
  - Status badge: `● ACTIVE` pill indicating the currently selected mode
- **Inactive State:**
  - Vitrified glass background: `rgba(15, 23, 42, 0.6)`
  - Smooth hover transitions and clear focus rings.

---

## 3. Interactive Data & Color Engine

Developers can interactively manipulate datasets in real-time without writing boilerplate:

### Point-by-Point Inspector
Each coordinate in the dataset can be inspected and edited:
- Direct numeric increment (`+5`) and decrement (`-5`).
- Individual point removal (`×`).
- Targeted point insertion with dynamic boundary clamping.
- Array operations: Sort Ascending, Sort Descending, Reverse, and preset trajectories (Bullish Trend, Volatility Wave).

### Color Themes & Custom Tints
- **Cyber Indigo:** `#6366f1` / `#06b6d4`
- **Emerald Surge:** `#10b981` / `#34d399`
- **Amber Sunset:** `#f59e0b` / `#ef4444`
- **Neon Fuchsia:** `#d946ef` / `#8b5cf6`
- **Zinc Mono:** `#e2e8f0` / `#94a3b8`
- Plus direct hex tint selectors for custom brand styling.

---

## 4. Design System Studio & Token Inspector

In `theme_studio.rs`, developers have full control over the visual design tokens:

| Token | Range | Description |
| :--- | :--- | :--- |
| `--border-radius` | `0px` to `32px` | Controls rounding across cards, buttons, badges, and modals. |
| `--backdrop-blur` | `0px` to `40px` | Controls frosted glass blur intensity. |
| `--glass-opacity` | `15%` to `90%` | Controls surface translucency against background canvases. |
| `--accent-primary` | Hex / HSL | Primary brand accent for actions, strokes, and highlights. |
| `--accent-secondary`| Hex / HSL | Secondary gradient accent and companion fills. |

### Component Showcase with Knobs
Test UI primitives interactively with size and variant knobs:
- **Buttons:** Sizes `xs`, `sm`, `md`, `lg` in `primary`, `glass`, `outline`, and `danger` variants.
- **Badges:** Semantically colored status pills (`primary`, `success`, `warning`, `danger`, `info`).
- **Form Controls:** Text inputs with prefix/suffix icons, reactive switch toggles, and range sliders.
- **Virtual DataGrid Mini:** High-performance tabular data with custom column alignments.
- **Overlays:** Hierarchical TreeViews and vector DatePickers.

---

## 5. Synchronized Code Export

Every adjustment in the Chart Studio or Theme Studio is instantly reflected in copy-ready snippets:
1. **CSS Custom Properties (`:root`):** Instant drop-in for `styles.css`.
2. **Rust `ThemeConfig`:** Strongly-typed configuration for native Rust backends and Wasm apps.
3. **Rust `DomBuilder`:** Pure Rust WebAssembly component creation code with zero JavaScript dependencies.
