---
id: playbook-handbook
title: Interactive Component Playbook & High-Density Workbench
sidebar_position: 1
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 📘 Interactive Component Playbook & High-Density Workbench

`ferrox-front` provides a production-grade WebAssembly frontend ecosystem engineered in **100% pure Rust** with **zero external JavaScript dependencies**. It couples an ultra-fast vector SVG charting engine (`ferrox-front-charts`) with an enterprise Glassmorphism Design System (`ferrox-front-ui`).

The **Component Playbook** is a professional, high-density engineering workbench ("Playbook & Live Knobs Inspector") providing typed domain architectures per component, real-time reactive knobs inspection, a pervasive color engine, mathematical data generators, and live synchronized Rust code generation.

---

## 🕹️ Live Interactive Workbench 2.0

Interact directly with the WebAssembly microfrontend below to test vector charts, customize theme tokens, inspect props in real-time, and copy synchronized Rust code:

<FerroxWasmPlayground initialTab="chart-studio" height="880px" title="Ferrox Front • High-Density Component Workbench" />

---

## 1. Typed Component Data Architectures

Rather than flattening data into a single generic float vector, each component in `ferrox-front` is governed by a dedicated domain data structure tailored to its geometry and operational domain:

| Component | Dedicated Configuration Model | Core Parameters & Invariants |
| :--- | :--- | :--- |
| **Line Chart** | `LineConfig` | `points: Vec<(f64, f64)>`, `curve: LineCurve` (Linear, Monotone, Stepped), `stroke_width: f64`, `show_dots: bool`, `show_axes: bool` |
| **Area Chart** | `AreaConfig` | `points: Vec<(f64, f64)>`, `curve: LineCurve`, `stroke_width: f64`, `fill_opacity: f64` (gradient stops to 0%), `show_axes: bool` |
| **Bar Chart** | `BarConfig` | `items: Vec<BarItem>` (`label`, `value`, `color`), `padding: f64`, `rounded_corners: bool`, `show_axes: bool` |
| **Donut / Pie** | `DonutConfig` | `segments: Vec<DonutItem>` (`label`, `value`, `color`), `inner_radius_pct: f64` (0% solid pie $\to$ 85% slim ring), `pad_angle: f64` |
| **Radar Spider** | `RadarConfig` | `dimensions: Vec<RadarAxis>` (`label`, `value`), `ring_count: usize` (3 to 6 polygonal grids), `fill_opacity: f64` |
| **Gauge Meter** | `GaugeConfig` | `value: f64`, `min: f64`, `max: f64`, `target: f64`, `unit: String`, `show_ticks: bool`, `warning_thresh: f64`, `critical_thresh: f64` |
| **Sparkline** | `SparklineConfig` | `ticks: Vec<f64>`, `spark_type: SparklineType` (Area, Line, Bar), `show_trend: bool` with trend velocity calculation |

All configurations are encapsulated inside the reactive `PlaybookState`, ensuring zero-latency updates upon any knob mutation.

---

## 2. Mathematical Rigor: GaugeMeter Concentric Geometry

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
options.critical_threshold = 0.85;
options.custom_color = Some("#ec4899".into());

let gauge = gauge_chart_styled(
    "telemetry-gauge",
    73.5,
    0.0,
    100.0,
    "Telemetry Speedometer",
    "%",
    340,
    190,
    &options,
);
```

---

## 3. Real-Time Props & Knobs Inspector

The lateral inspector column gives developers interactive control over every visual and geometric parameter:

- **Line & Area Knobs:**
  - *Interpolation Curve:* Toggle dynamically between `Linear`, `Monotone` (cubic Bezier with smooth tangents), and `Stepped` (horizontal/vertical orthogonal transitions).
  - *Stroke Width:* Instant scalar selection from `1.5px` to `7.0px`.
  - *Fill Opacity:* Adjust gradient area density from `15%` to `90%`.
  - *Visual Aids:* Toggles for data point dots and coordinate grid axes.
- **Bar Knobs:**
  - *Bar Spacing / Padding:* Fine-tune inter-column gap from `10px` to `36px`.
  - *Rounded Corners:* Toggle SVG rounded caps (`rx = 6`).
- **Donut Knobs:**
  - *Cutout Radius:* Seamlessly morph from solid pie (`0%`) to standard donut (`62%`) up to ultra-slim ring (`85%`).
- **Radar Knobs:**
  - *Polygonal Rings:* Select between 3, 4, 5, or 6 concentric spider grid polygons.
  - *Area Fill Opacity:* Adjust polygon background translucency.
- **Gauge Knobs:**
  - *Live Speedometer Value:* Increment/decrement telemetry values with boundary clamping.
  - *Threshold Zones:* Adjust Warning and Critical transition limits.
- **Sparkline Knobs:**
  - *Representation Variants:* Switch instantly between `Area`, `Line`, and `Bar` sparklines.
  - *Trend Velocity:* Displays percentage rate-of-change ($\Delta\% = \frac{v_{\text{last}} - v_{\text{first}}}{v_{\text{first}}} \times 100$) with directional indicators (`▲ +` / `▼ -`).

---

## 4. Pervasive Color Engine

The workbench features a **Pervasive Color Engine** providing 6 curated harmonic palettes that cascade instantaneously across all component types:

1. **Cyber Neon:** `#ec4899` (Pink) • `#06b6d4` (Cyan) • `#a855f7` (Purple)
2. **Emerald Flow:** `#10b981` (Mint) • `#059669` (Emerald) • `#14b8a6` (Teal)
3. **Amber Solar:** `#f59e0b` (Gold) • `#f97316` (Orange) • `#ef4444` (Crimson)
4. **Deep Ocean:** `#0284c7` (Azure) • `#38bdf8` (Sky) • `#6366f1` (Indigo)
5. **Zinc Mono:** `#94a3b8` (Slate) • `#71717a` (Zinc) • `#f8fafc` (White)
6. **Synthwave Violet:** `#8b5cf6` (Violet) • `#d946ef` (Fuchsia) • `#6366f1` (Indigo)

Direct tint selectors and custom hex overrides provide instant brand harmonization.

---

## 5. Algorithmic Data Generators

To eliminate static mock arrays, `data_generators.rs` implements mathematical stochastic generators using a zero-dependency deterministic `FastRng` (SplitMix64 / Box-Muller transform) executing identically in native Rust and WebAssembly:

- 🎲 **Brownian Motion:** Simulates financial random walk trajectories with volatility and directional drift:
  $$S_t = S_{t-1} + \mu + \sigma Z_t, \quad Z_t \sim \mathcal{N}(0, 1)$$
- 🌊 **Harmonic Wave:** Generates multi-frequency sinusoidal waveforms combined with Gaussian noise to emulate periodic sensor telemetry:
  $$y(t) = y_0 + A_1 \sin\left(\frac{2\pi t}{T_1}\right) + A_2 \cos\left(\frac{2\pi t}{T_2}\right) + \epsilon$$
- ⚡ **Traffic Spikes:** Simulates stochastic server ingress with exponential burst spikes and gradual recovery.
- 📊 **Normal Bell:** Generates Gaussian bell distributions centered symmetrically across the discrete steps:
  $$f(x) = H \cdot \exp\left(-\frac{(x - \mu)^2}{2\sigma^2}\right)$$

---

## 6. Interactive Data Table Editor

Located in the center column beneath the SVG preview canvas, the **Data Table Editor** allows granular control over active datasets:
- **Direct Point Modification:** Quick numeric increment (`+5`) and decrement (`-5`) buttons per item.
- **Individual Point Deletion:** Deletion (`×`) with minimum record safeguarding.
- **Dynamic Point Addition:** `[➕ Add Next Point]` button that calculates appropriate next steps according to component domain rules.

---

## 7. Segmented Glass Pill Bar & 3-Zone Layout

The layout follows a professional **3-Zone High-Density Workbench** architecture:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       SEGMENTED GLASS PILL BAR                              │
│ [Line] [Area] [Bar] [Donut / Pie] [Radar Spider] [Gauge Meter] [Sparkline]  │
└─────────────────────────────────────────────────────────────────────────────┘
┌───────────────────┬───────────────────────────────┬─────────────────────────┐
│  ZONE 1: KNOBS    │     ZONE 2: CANVAS & TABLE    │   ZONE 3: RUST CODE     │
│  (Col 3 / 12)     │         (Col 6 / 12)          │      (Col 3 / 12)       │
├───────────────────┼───────────────────────────────┼─────────────────────────┤
│ • Props Inspector │ • Interactive Vector SVG      │ • Strongly-typed        │
│   (Curve, Stroke, │   Preview Canvas (60 FPS)     │   Rust DomBuilder       │
│   Cutout, Opacity)│ • Algorithmic Generators      │   Code Generator        │
│ • Color Engine    │   (Brownian, Wave, Spikes)    │ • 1-Click Clipboard     │
│   (6 Palettes &   │ • Interactive Data Table      │   Copy with Toast       │
│   Direct Tints)   │   Point Editor (+5, -5, ×)    │                         │
└───────────────────┴───────────────────────────────┴─────────────────────────┘
```

The **Segmented Glass Pill Bar** uses:
- Vitrified dark-glass container (`rgba(15, 23, 42, 0.8)` with `backdrop-filter: blur(20px)`).
- Luminescent active state (`linear-gradient(135deg, #6366f1, #a855f7)` with glowing border `rgba(255, 255, 255, 0.4)` and a distinct `● ACTIVE` badge).
- Inactive pills with soft hover states.

---

## 8. Synchronized Pure Rust Code Generation

Every adjustment in the inspector is dynamically reflected in copy-ready Rust code using `ferrox-front-charts` and `DomBuilder`. A single click on **"📋 Copy Snippet"** delivers an idiomatic, strongly-typed code snippet directly to the clipboard, completely ready for production deployment.
