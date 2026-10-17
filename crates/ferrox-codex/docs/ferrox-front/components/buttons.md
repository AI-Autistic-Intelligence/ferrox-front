---
id: buttons
title: Buttons, Action Controls & Spinners
sidebar_position: 2
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🎛️ Buttons, Action Controls & Spinners

Ferrox Front delivers an accessible, high-performance button subsystem designed for rapid user interactions and sub-microsecond event dispatches in WebAssembly.

---

## 🚀 Live Interactive Component Preview (Bootstrap-Style)

Experience the visual aesthetics, micro-interactions, and hover states rendered live below:

<div style={{ padding: '24px', borderRadius: '16px', background: 'rgba(15, 23, 42, 0.72)', border: '1px solid rgba(148, 163, 184, 0.14)', margin: '24px 0', backdropFilter: 'blur(20px)' }}>
  <div style={{ marginBottom: '20px' }}>
    <div style={{ fontSize: '0.75rem', fontWeight: 700, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.05em', marginBottom: '10px' }}>
      Visual Variants:
    </div>
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px', alignItems: 'center' }}>
      <button style={{ padding: '8px 18px', borderRadius: '8px', border: 'none', background: '#6366f1', color: '#fff', fontWeight: 600, fontSize: '0.85rem', cursor: 'pointer', boxShadow: '0 4px 14px rgba(99, 102, 241, 0.4)' }}>
        ⚡ Primary Action
      </button>
      <button style={{ padding: '8px 18px', borderRadius: '8px', border: '1px solid rgba(148, 163, 184, 0.2)', background: 'rgba(30, 41, 59, 0.7)', color: '#f8fafc', fontWeight: 600, fontSize: '0.85rem', cursor: 'pointer' }}>
        🪟 Secondary Glass
      </button>
      <button style={{ padding: '8px 18px', borderRadius: '8px', border: '1px solid #38bdf8', background: 'transparent', color: '#38bdf8', fontWeight: 600, fontSize: '0.85rem', cursor: 'pointer' }}>
        🔍 Outline Cyan
      </button>
      <button style={{ padding: '8px 18px', borderRadius: '8px', border: 'none', background: '#f43f5e', color: '#fff', fontWeight: 600, fontSize: '0.85rem', cursor: 'pointer', boxShadow: '0 4px 14px rgba(244, 63, 94, 0.4)' }}>
        🗑️ Destructive
      </button>
      <button style={{ padding: '8px 18px', borderRadius: '8px', border: 'none', background: 'transparent', color: '#94a3b8', fontWeight: 600, fontSize: '0.85rem', cursor: 'pointer' }}>
        Ghost Neutral
      </button>
    </div>
  </div>

  <div style={{ paddingTop: '16px', borderTop: '1px solid rgba(148, 163, 184, 0.12)', marginBottom: '20px' }}>
    <div style={{ fontSize: '0.75rem', fontWeight: 700, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.05em', marginBottom: '10px' }}>
      Scale & Sizing Hierarchy:
    </div>
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px', alignItems: 'center' }}>
      <button style={{ padding: '4px 10px', borderRadius: '6px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.72rem', fontWeight: 600 }}>
        XS (24px)
      </button>
      <button style={{ padding: '6px 14px', borderRadius: '7px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.8rem', fontWeight: 600 }}>
        SM (32px)
      </button>
      <button style={{ padding: '10px 20px', borderRadius: '8px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.9rem', fontWeight: 600 }}>
        MD (40px)
      </button>
      <button style={{ padding: '14px 28px', borderRadius: '10px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '1rem', fontWeight: 700 }}>
        LG (48px)
      </button>
    </div>
  </div>

  <div style={{ paddingTop: '16px', borderTop: '1px solid rgba(148, 163, 184, 0.12)' }}>
    <div style={{ fontSize: '0.75rem', fontWeight: 700, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.05em', marginBottom: '10px' }}>
      Segmented Button Group & Loading Spinners:
    </div>
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '16px', alignItems: 'center' }}>
      <div style={{ display: 'inline-flex', borderRadius: '8px', overflow: 'hidden', border: '1px solid rgba(148, 163, 184, 0.2)' }}>
        <button style={{ padding: '6px 12px', border: 'none', background: 'rgba(30, 41, 59, 0.7)', color: '#94a3b8', fontSize: '0.75rem', cursor: 'pointer' }}>1M</button>
        <button style={{ padding: '6px 12px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.75rem', fontWeight: 700, cursor: 'pointer' }}>5M</button>
        <button style={{ padding: '6px 12px', border: 'none', background: 'rgba(30, 41, 59, 0.7)', color: '#94a3b8', fontSize: '0.75rem', cursor: 'pointer' }}>15M</button>
        <button style={{ padding: '6px 12px', border: 'none', background: 'rgba(30, 41, 59, 0.7)', color: '#94a3b8', fontSize: '0.75rem', cursor: 'pointer' }}>1H</button>
      </div>

      <button style={{ display: 'inline-flex', alignItems: 'center', gap: '8px', padding: '6px 16px', borderRadius: '8px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.8rem', fontWeight: 600 }}>
        <span style={{ width: '12px', height: '12px', border: '2px solid rgba(255,255,255,0.3)', borderTopColor: '#fff', borderRadius: '50%', display: 'inline-block', animation: 'ferrox-spin 0.8s linear infinite' }}></span>
        Processing Order...
      </button>
    </div>
  </div>
</div>

---

## 🦀 Rust API Reference

The primary button construction primitive is `button_advanced`:

```rust
use ferrox_front_ui::components::{button_advanced, button_group};

// 1. Primary Action with icon and click callback
let buy_btn = button_advanced("Submit Market Order", "primary", "md", false, Some("zap"))
    .on_click(|| {
        web_sys::console::log_1(&"Order dispatched to in-memory matching engine".into());
    });

// 2. Loading State (disables clicks and renders SVG spinner)
let loading_btn = button_advanced("Synchronizing Ledger...", "primary", "sm", true, None);

// 3. Destructive Action
let cancel_btn = button_advanced("Cancel All Resting Orders", "danger", "sm", false, Some("trash"));

// 4. Segmented Timeframe Switcher
let timeframe_selector = button_group(vec![
    button_advanced("1M", "glass", "xs", false, None),
    button_advanced("5M", "primary", "xs", false, None),
    button_advanced("15M", "glass", "xs", false, None),
    button_advanced("1H", "glass", "xs", false, None),
]);
```

### Supported Parameters

| Parameter | Type | Options | Purpose |
| :--- | :--- | :--- | :--- |
| `label` | `&str` | String text | User-facing button caption |
| `variant` | `&str` | `"primary"`, `"glass"`, `"outline"`, `"danger"`, `"ghost"`, `"link"` | Semantic visual style |
| `size` | `&str` | `"xs"`, `"sm"`, `"md"`, `"lg"` | Elevation & padding scale |
| `loading` | `bool` | `true`, `false` | Displays spinning SVG indicator & disables pointer |
| `icon` | `Option<&str>` | `"zap"`, `"search"`, `"trash"`, `"download"`, etc. | Leading vector SVG icon |

---

## 🕹️ Live Full Application Playground

Test button interactions within the complete compiled WebAssembly environment:

<FerroxWasmPlayground initialTab="components" height="740px" title="Ferrox Front • UI Elements Showcase" />
