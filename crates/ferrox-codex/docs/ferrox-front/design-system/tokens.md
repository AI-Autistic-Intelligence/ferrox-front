---
id: tokens
title: Design System Tokens & Aesthetics
sidebar_position: 1
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🎨 Design System Tokens & Aesthetics

Ferrox Front incorporates an enterprise-grade, high-contrast **Design System** engineered for institutional trading terminals, mission-critical dashboards, and high-frequency analytical applications.

Unlike traditional frontend frameworks that introduce megabytes of runtime CSS-in-JS overhead, Ferrox Front leverages **native CSS custom properties (variables)** mapped directly to Rust typed token structures.

---

## 💎 Design Philosophy & Core Principles

1. **Sub-Microsecond Visual Hierarchy**: Visual anchors (glow borders, semantic badges, typography) are calibrated to direct eye focus within 50ms of rendering.
2. **True Glassmorphism (`backdrop-filter`)**: Deep multi-layered translucent surfaces (`rgba(15, 23, 42, 0.72)`) backed by hardware-accelerated SVG blur filters.
3. **WCAG 2.1 AA Contrast Compliance**: All primary and secondary text elements guarantee a minimum contrast ratio of 4.5:1 against glass surfaces and dark baselines.
4. **Zero CSS-in-JS Runtime**: All styling rules are precompiled or referenced via native CSS variables—0 CPU cycles wasted on runtime style recalculation.

---

## 🎨 Color Palette & Swatches

<div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', gap: '16px', margin: '24px 0' }}>
  <div style={{ padding: '16px', borderRadius: '14px', background: '#060913', border: '1px solid rgba(148, 163, 184, 0.2)', color: '#f8fafc' }}>
    <div style={{ width: '100%', height: '44px', borderRadius: '8px', background: '#6366f1', marginBottom: '10px', boxShadow: '0 4px 14px rgba(99, 102, 241, 0.4)' }}></div>
    <div style={{ fontWeight: 700, fontSize: '0.9rem' }}>--accent-primary</div>
    <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontFamily: 'monospace' }}>#6366f1 (Indigo)</div>
    <div style={{ fontSize: '0.72rem', color: '#cbd5e1', marginTop: '4px' }}>Primary buttons, active tabs, focus rings</div>
  </div>

  <div style={{ padding: '16px', borderRadius: '14px', background: '#060913', border: '1px solid rgba(148, 163, 184, 0.2)', color: '#f8fafc' }}>
    <div style={{ width: '100%', height: '44px', borderRadius: '8px', background: '#06b6d4', marginBottom: '10px', boxShadow: '0 4px 14px rgba(6, 182, 212, 0.4)' }}></div>
    <div style={{ fontWeight: 700, fontSize: '0.9rem' }}>--accent-secondary</div>
    <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontFamily: 'monospace' }}>#06b6d4 (Cyan)</div>
    <div style={{ fontSize: '0.72rem', color: '#cbd5e1', marginTop: '4px' }}>Vector chart gradients, indicators</div>
  </div>

  <div style={{ padding: '16px', borderRadius: '14px', background: '#060913', border: '1px solid rgba(148, 163, 184, 0.2)', color: '#f8fafc' }}>
    <div style={{ width: '100%', height: '44px', borderRadius: '8px', background: '#10b981', marginBottom: '10px', boxShadow: '0 4px 14px rgba(16, 185, 129, 0.4)' }}></div>
    <div style={{ fontWeight: 700, fontSize: '0.9rem' }}>--accent-success</div>
    <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontFamily: 'monospace' }}>#10b981 (Emerald)</div>
    <div style={{ fontSize: '0.72rem', color: '#cbd5e1', marginTop: '4px' }}>Matched trades, positive PnL, valid state</div>
  </div>

  <div style={{ padding: '16px', borderRadius: '14px', background: '#060913', border: '1px solid rgba(148, 163, 184, 0.2)', color: '#f8fafc' }}>
    <div style={{ width: '100%', height: '44px', borderRadius: '8px', background: '#f43f5e', marginBottom: '10px', boxShadow: '0 4px 14px rgba(244, 63, 94, 0.4)' }}></div>
    <div style={{ fontWeight: 700, fontSize: '0.9rem' }}>--accent-danger</div>
    <div style={{ fontSize: '0.75rem', color: '#94a3b8', fontFamily: 'monospace' }}>#f43f5e (Rose)</div>
    <div style={{ fontSize: '0.72rem', color: '#cbd5e1', marginTop: '4px' }}>Sell orders, error validation, security alerts</div>
  </div>
</div>

---

## 📐 Token Specification Table

| CSS Custom Property | Default Token Value | Applied Component Role |
| :--- | :--- | :--- |
| `--bg-base` | `#060913` | Deep space viewport background canvas |
| `--bg-surface` | `rgba(15, 23, 42, 0.72)` | Translucent glassmorphism cards and tables |
| `--bg-surface-hover` | `rgba(30, 41, 59, 0.85)` | Elevated surface hover state |
| `--accent-primary` | `#6366f1` | Brand focal color, primary buttons, indicators |
| `--accent-secondary` | `#06b6d4` | Secondary charts, accents, glowing borders |
| `--accent-glow` | `rgba(99, 102, 241, 0.25)` | Radial backdrop spotlights and ambient shadows |
| `--text-primary` | `#f8fafc` | Headings, primary metrics, active labels |
| `--text-secondary` | `#94a3b8` | Subtitles, table headers, metadata captions |
| `--border-radius` | `16px` | Smooth geometric card and surface curvature |
| `--backdrop-blur` | `20px` | Hardware-accelerated frosted glass blur |

---

## ⚡ Using Design Tokens in Rust WebAssembly

In Rust components, apply tokens directly via `ferrox_front_core::dom` styles or through the `ThemeConfig` struct:

```rust
use ferrox_front_core::dom::{div, h3, p, DomBuilder};
use ferrox_front_ui::components::badge;

pub fn metric_card(title: &str, value: &str, status: &str) -> DomBuilder {
    div()
        .class("rounded-2xl backdrop-blur-xl p-6 transition-all")
        .attr("style", "background: var(--bg-surface); border: var(--border-glass);")
        .children(vec![
            div().class("flex items-center justify-between pb-3")
                .children(vec![
                    h3().class("text-xs font-bold uppercase tracking-wider")
                        .attr("style", "color: var(--text-secondary);")
                        .text(title),
                    badge(status, "success"),
                ]),
            p().class("text-2xl font-black")
                .attr("style", "color: var(--text-primary);")
                .text(value),
        ])
}
```

---

## 🎮 Live Interactive Theme Studio Preview

Customize all design tokens in real-time below using the compiled WebAssembly **Theme Studio**:

<FerroxWasmPlayground initialTab="theme-studio" height="840px" title="Ferrox Front • Design System & Theme Studio" />
