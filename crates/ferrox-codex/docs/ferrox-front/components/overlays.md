---
id: overlays
title: Overlays, Modals, DatePicker & Command Palette (⌘K)
sidebar_position: 5
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🪟 Overlays, Modals, DatePicker & Command Palette (⌘K)

Ferrox Front includes a suite of enterprise overlay components engineered directly in Rust WebAssembly, featuring **Spotlight-style Command Palette (`Cmd+K`)**, pure vector **SVG DatePicker**, slide-over drawers, and WCAG 2.1 AA accessible focus-trapped modal dialogs.

---

## 🚀 Live Interactive Component Preview (Bootstrap-Style)

<div style={{ padding: '24px', borderRadius: '16px', background: 'rgba(15, 23, 42, 0.72)', border: '1px solid rgba(148, 163, 184, 0.14)', margin: '24px 0', backdropFilter: 'blur(20px)' }}>
  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '20px', marginBottom: '24px' }}>
    {/* Command Palette Card */}
    <div style={{ padding: '18px', borderRadius: '14px', background: 'rgba(30, 41, 59, 0.7)', border: '1px solid rgba(148, 163, 184, 0.15)' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '10px' }}>
        <span style={{ fontSize: '0.82rem', fontWeight: 700, color: '#f8fafc' }}>Spotlight Command Search</span>
        <span style={{ fontSize: '0.7rem', padding: '2px 8px', borderRadius: '4px', background: 'rgba(99, 102, 241, 0.2)', color: '#a5b4fc', fontFamily: 'monospace' }}>⌘K / Ctrl+K</span>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', padding: '8px 12px', borderRadius: '8px', background: 'rgba(15, 23, 42, 0.8)', border: '1px solid rgba(148, 163, 184, 0.2)', marginBottom: '10px' }}>
        <span style={{ marginRight: '8px', color: '#94a3b8' }}>🔍</span>
        <input style={{ background: 'transparent', border: 'none', color: '#f8fafc', fontSize: '0.8rem', width: '100%', outline: 'none' }} placeholder="Type a command or asset symbol..." defaultValue="BTC/USD Perp" />
      </div>
      <div style={{ fontSize: '0.72rem', color: '#94a3b8' }}>Fuzzy index search across routes, order books, and theme presets.</div>
    </div>

    {/* Vector DatePicker Card */}
    <div style={{ padding: '18px', borderRadius: '14px', background: 'rgba(30, 41, 59, 0.7)', border: '1px solid rgba(148, 163, 184, 0.15)' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '10px' }}>
        <span style={{ fontSize: '0.82rem', fontWeight: 700, color: '#f8fafc' }}>Pure Vector DatePicker</span>
        <span style={{ fontSize: '0.7rem', color: '#38bdf8', fontWeight: 600 }}>Zero Moment/DayJS</span>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '6px 12px', borderRadius: '8px', background: 'rgba(15, 23, 42, 0.8)', border: '1px solid rgba(148, 163, 184, 0.2)', fontSize: '0.78rem', color: '#cbd5e1' }}>
        <span>📅 Range: 2026-10-01 → 2026-10-15</span>
        <span style={{ color: '#10b981', fontWeight: 700 }}>✓ Applied</span>
      </div>
      <div style={{ fontSize: '0.72rem', color: '#94a3b8', marginTop: '10px' }}>Generated natively in Rust with single or date-range selection.</div>
    </div>
  </div>

  {/* Modal Trigger Demonstration */}
  <div style={{ paddingTop: '16px', borderTop: '1px solid rgba(148, 163, 184, 0.12)', display: 'flex', flexWrap: 'wrap', gap: '14px', alignItems: 'center' }}>
    <button style={{ padding: '8px 18px', borderRadius: '8px', border: 'none', background: '#6366f1', color: '#fff', fontSize: '0.82rem', fontWeight: 600, cursor: 'pointer', boxShadow: '0 4px 14px rgba(99, 102, 241, 0.4)' }}>
      🪟 Trigger Modal Dialog
    </button>
    <button style={{ padding: '8px 18px', borderRadius: '8px', border: '1px solid rgba(148, 163, 184, 0.2)', background: 'rgba(30, 41, 59, 0.7)', color: '#f8fafc', fontSize: '0.82rem', fontWeight: 600, cursor: 'pointer' }}>
      📂 Open Slide-Over Drawer
    </button>
    <button style={{ padding: '8px 18px', borderRadius: '8px', border: '1px solid #10b981', background: 'rgba(16, 185, 129, 0.15)', color: '#10b981', fontSize: '0.82rem', fontWeight: 600, cursor: 'pointer' }}>
      🔔 Fire Live Toast Alert
    </button>
  </div>
</div>

---

## 🦀 Rust API Implementation

### 1. Command Palette (`command_palette`)
Spotlight-style global modal search with keyboard navigation (`ArrowUp`, `ArrowDown`, `Enter`, `Escape`):

```rust
use ferrox_front_ui::overlays::{CommandAction, CommandPaletteState, command_palette};

let actions = vec![
    CommandAction::new("act-trading", "Go to Trading Desk", "Live Candlestick charts", "Navigation", "chart-candlestick", Some("G T"), || {
        // Route handler
    }),
    CommandAction::new("act-csv", "Export Orders to CSV", "Download RFC 4180 in Wasm", "Actions", "download", Some("E C"), || {
        // Trigger CSV download
    }),
];

let state = CommandPaletteState::new(actions);
let overlay_dom = command_palette(state);
```

### 2. Modal Dialog (`modal_advanced`)
Focus-trapped accessible dialog conforming to WCAG 2.1 AA:

```rust
use ferrox_front_ui::overlays::modal_advanced;
use ferrox_front_ui::components::button_advanced;
use ferrox_front_core::dom::p;

let modal = modal_advanced(
    "auth-modal",
    "Trading Node Security Verification",
    p().class("text-slate-300 text-sm")
        .text("Authorize in-memory matching engine node with sub-microsecond zero-trust token?"),
    Some(button_advanced("Authorize Node", "primary", "sm", false, None)),
    "md",
);
```

---

## 🕹️ Live Explorer & Overlays Playground

Test the hierarchical TreeView, pure vector DatePicker, and Command Palette live below:

<FerroxWasmPlayground initialTab="explorer" height="780px" title="Ferrox Front • Explorer & Overlays" />
