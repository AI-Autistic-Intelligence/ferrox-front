---
id: theme-studio
title: Theme Studio & Token Exporter
sidebar_position: 2
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🎨 Theme Studio & Token Exporter

The **Theme Studio** is an interactive design workbench compiled directly into WebAssembly. It allows engineering teams and designers to customize brand palettes, corner radiuses, and glassmorphism blurs on the fly, observe instant component previews, and export production-ready tokens in CSS and Rust formats.

---

## 🕹️ Live Interactive Theme Studio

Test theme presets, modify color tokens, tweak corner radiuses, and export generated code directly below:

<FerroxWasmPlayground initialTab="theme-studio" height="880px" title="Ferrox Front • WebAssembly Theme Studio & Exporter" />

---

## 🌟 Curated Theme Presets

Ferrox Front ships with six pre-calibrated themes designed for diverse operational contexts:

1. **🔮 Cyber Dark (`ferrox-cyber`)**: Deep space blue/purple glassmorphism with vivid indigo (`#6366f1`) and cyan (`#06b6d4`) accents. Default for high-frequency trading terminals.
2. **🏢 Zinc Light (`enterprise-zinc`)**: High-contrast, clean enterprise light palette with dark slate text (`#0f172a`) and crisp subtle borders (`#e2e8f0`). Ideal for accounting, compliance, and daylight operations.
3. **⚡ Cyberpunk Neon (`cyberpunk-neon`)**: High-energy contrast with neon hot pink (`#ec4899`) and electric cyan (`#00f0ff`). Optimized for streaming operations and telemetry war-rooms.
4. **🌊 Ocean Breeze (`ocean-breeze`)**: Deep aquatic navy baseline (`#071326`) with crystal aqua (`#00b4d8`) and sky blue highlights.
5. **🌅 Sunset Gold (`sunset-gold`)**: Warm obsidian base with radiant amber (`#f59e0b`) and crimson flame accents.
6. **🌲 Emerald Forest (`forest-emerald`)**: Deep spruce matrix with luminous mint (`#10b981`) and emerald glow.

---

## 🛠️ Modifying Tokens in Real Time

In the Theme Studio above, clicking any color pill or slider immediately applies changes to `document.documentElement.style` using zero-copy WebAssembly DOM calls:

```rust
// Internally executed by Theme Studio in WebAssembly
if let Some(win) = web_sys::window() {
    if let Some(doc) = win.document() {
        if let Some(root) = doc.document_element() {
            if let Ok(elem) = root.dyn_into::<web_sys::HtmlElement>() {
                let style = elem.style();
                let _ = style.set_property("--accent-primary", "#10b981");
                let _ = style.set_property("--border-radius", "18px");
                let _ = style.set_property("--backdrop-blur", "24px");
            }
        }
    }
}
```

---

## 📋 Exporting Custom Themes

The Theme Studio provides two instant export formats:

### 1. CSS Custom Properties (`:root`)
Drop directly into your application's `styles.css` or Tailwind theme extension:

```css
:root, [data-theme="custom-theme"] {
  --bg-base: #060913;
  --bg-surface: rgba(15, 23, 42, 0.72);
  --accent-primary: #6366f1;
  --accent-secondary: #06b6d4;
  --text-primary: #f8fafc;
  --border-radius: 16px;
  --backdrop-blur: 20px;
  --border-glass: 1px solid rgba(255, 255, 255, 0.12);
  --shadow-glass: 0 20px 40px -15px rgba(0, 0, 0, 0.55);
}
```

### 2. Rust Strong-Typed `ThemeConfig`
Consume directly in your WebAssembly frontend or backend template engines:

```rust
use ferrox_front_ui::tokens::ThemeConfig;

pub fn my_institutional_theme() -> ThemeConfig {
    ThemeConfig {
        name: "institutional_theme".into(),
        bg_base: "#060913".into(),
        bg_surface: "rgba(15, 23, 42, 0.72)".into(),
        accent_primary: "#6366f1".into(),
        accent_secondary: "#06b6d4".into(),
        text_primary: "#f8fafc".into(),
        border_radius: "16px".into(),
    }
}
```
