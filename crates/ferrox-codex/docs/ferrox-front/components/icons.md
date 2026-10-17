---
id: icons
title: Pure Rust 60+ SVG Vector Icon Engine
sidebar_position: 6
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🎨 Pure Rust 60+ SVG Vector Icon Engine

Ferrox Front embeds a library of **60+ high-precision vector SVG icons** compiled directly into Rust WebAssembly bytecode. 

Unlike font icons (FontAwesome, Material Icons) or SVG sprite packs that require external HTTP asset requests, Ferrox Front icons render natively into the DOM at **zero network cost, zero FOUT (Flash of Unstyled Text), and zero JavaScript dependencies**.

---

## 🚀 Live Interactive Icon Catalog

Browse the primary vector icons compiled into WebAssembly below:

<div style={{ padding: '24px', borderRadius: '16px', background: 'rgba(15, 23, 42, 0.72)', border: '1px solid rgba(148, 163, 184, 0.14)', margin: '24px 0', backdropFilter: 'blur(20px)' }}>
  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(110px, 1fr))', gap: '12px' }}>
    {[
      { name: 'zap', cat: 'Finance' },
      { name: 'activity', cat: 'Finance' },
      { name: 'trend-up', cat: 'Finance' },
      { name: 'trend-down', cat: 'Finance' },
      { name: 'chart-candlestick', cat: 'Finance' },
      { name: 'chart-bar', cat: 'Finance' },
      { name: 'dollar-sign', cat: 'Finance' },
      { name: 'credit-card', cat: 'Finance' },
      { name: 'shield', cat: 'Security' },
      { name: 'lock', cat: 'Security' },
      { name: 'unlock', cat: 'Security' },
      { name: 'key', cat: 'Security' },
      { name: 'database', cat: 'System' },
      { name: 'cpu', cat: 'System' },
      { name: 'server', cat: 'System' },
      { name: 'search', cat: 'UI' },
      { name: 'settings', cat: 'UI' },
      { name: 'palette', cat: 'Theme' },
      { name: 'moon', cat: 'Theme' },
      { name: 'sun', cat: 'Theme' },
      { name: 'check', cat: 'Action' },
      { name: 'alert-circle', cat: 'Action' },
      { name: 'info', cat: 'Action' },
      { name: 'trash', cat: 'Action' },
      { name: 'edit', cat: 'Action' },
      { name: 'download', cat: 'Action' },
      { name: 'folder', cat: 'UI' },
      { name: 'file-text', cat: 'UI' },
      { name: 'external-link', cat: 'UI' },
    ].map((item) => (
      <div key={item.name} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', padding: '12px', borderRadius: '10px', background: 'rgba(30, 41, 59, 0.6)', border: '1px solid rgba(148, 163, 184, 0.15)', gap: '6px' }}>
        <div style={{ width: '22px', height: '22px', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#818cf8', fontWeight: 700 }}>
          {item.name === 'zap' && '⚡'}
          {item.name === 'activity' && '📈'}
          {item.name === 'trend-up' && '↗️'}
          {item.name === 'trend-down' && '↘️'}
          {item.name === 'chart-candlestick' && '🕯️'}
          {item.name === 'chart-bar' && '📊'}
          {item.name === 'dollar-sign' && '💲'}
          {item.name === 'credit-card' && '💳'}
          {item.name === 'shield' && '🛡️'}
          {item.name === 'lock' && '🔒'}
          {item.name === 'unlock' && '🔓'}
          {item.name === 'key' && '🔑'}
          {item.name === 'database' && '🗄️'}
          {item.name === 'cpu' && '💻'}
          {item.name === 'server' && '🖥️'}
          {item.name === 'search' && '🔍'}
          {item.name === 'settings' && '⚙️'}
          {item.name === 'palette' && '🎨'}
          {item.name === 'moon' && '🌙'}
          {item.name === 'sun' && '☀️'}
          {item.name === 'check' && '✓'}
          {item.name === 'alert-circle' && '⚠️'}
          {item.name === 'info' && 'ℹ️'}
          {item.name === 'trash' && '🗑️'}
          {item.name === 'edit' && '✏️'}
          {item.name === 'download' && '📥'}
          {item.name === 'folder' && '📁'}
          {item.name === 'file-text' && '📄'}
          {item.name === 'external-link' && '🔗'}
        </div>
        <span style={{ fontSize: '0.68rem', fontFamily: 'monospace', color: '#cbd5e1' }}>{item.name}</span>
      </div>
    ))}
  </div>
</div>

---

## 🦀 Rust API Reference

Render any icon anywhere in your component tree using `Icon::render`:

```rust
use ferrox_front_ui::icons::Icon;
use ferrox_front_core::dom::{div, DomBuilder};

// 1. Render standard 18px accent icon
let icon_primary = Icon::render("zap", 18, "var(--accent-primary)");

// 2. Render semantic status icon
let icon_success = Icon::render("check", 16, "#10b981");
let icon_danger  = Icon::render("alert-circle", 16, "#f43f5e");

// 3. Render icon inside a card header
let header = div().class("flex items-center gap-2")
    .children(vec![
        Icon::render("chart-candlestick", 20, "#38bdf8"),
        div().text("Bitcoin Perpetual 5m"),
    ]);
```

### Supported Parameters

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `name` | `&str` | N/A | Identifier name from the 60+ icon registry |
| `size` | `u32` | `18` | Rendered width and height in CSS pixels (`px`) |
| `color` | `&str` | `"currentColor"` | SVG fill/stroke color or CSS custom property |

---

## 🕹️ Live Design System Playground

<FerroxWasmPlayground initialTab="components" height="740px" title="Ferrox Front • Vector Icon Engine" />
