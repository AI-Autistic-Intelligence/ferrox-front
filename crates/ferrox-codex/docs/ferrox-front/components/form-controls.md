---
id: form-controls
title: Form Controls, Reactive Inputs & Validation
sidebar_position: 3
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 📝 Form Controls, Reactive Inputs & Validation

The Ferrox Front form subsystem achieves **Zod and React Hook Form feature parity** directly in Rust WebAssembly, eliminating the high CPU cost and memory allocations of JavaScript validation libraries.

---

## 🚀 Live Interactive Component Preview (Bootstrap-Style)

Test the reactive inputs, validation states, and selection controls rendered live below:

<div style={{ padding: '24px', borderRadius: '16px', background: 'rgba(15, 23, 42, 0.72)', border: '1px solid rgba(148, 163, 184, 0.14)', margin: '24px 0', backdropFilter: 'blur(20px)' }}>
  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '20px', marginBottom: '24px' }}>
    {/* Valid Input */}
    <div>
      <label style={{ fontSize: '0.75rem', fontWeight: 700, color: '#f8fafc', display: 'block', marginBottom: '6px' }}>
        Institutional Trader Email (Valid State):
      </label>
      <div style={{ display: 'flex', alignItems: 'center', padding: '8px 12px', borderRadius: '8px', background: 'rgba(30, 41, 59, 0.7)', border: '1px solid #10b981', boxShadow: '0 0 0 2px rgba(16, 185, 129, 0.2)' }}>
        <span style={{ marginRight: '8px', color: '#10b981' }}>✉️</span>
        <input style={{ background: 'transparent', border: 'none', color: '#f8fafc', width: '100%', fontSize: '0.85rem', outline: 'none' }} defaultValue="alex.vance@institutional-trade.io" />
        <span style={{ color: '#10b981', fontWeight: 700 }}>✓</span>
      </div>
      <div style={{ fontSize: '0.72rem', color: '#10b981', marginTop: '4px' }}>
        ✓ Authorized institutional domain verified via WebCrypto.
      </div>
    </div>

    {/* Invalid Input */}
    <div>
      <label style={{ fontSize: '0.75rem', fontWeight: 700, color: '#f8fafc', display: 'block', marginBottom: '6px' }}>
        Execution Price Limit (Invalid Boundary):
      </label>
      <div style={{ display: 'flex', alignItems: 'center', padding: '8px 12px', borderRadius: '8px', background: 'rgba(30, 41, 59, 0.7)', border: '1px solid #f43f5e', boxShadow: '0 0 0 2px rgba(244, 63, 94, 0.2)' }}>
        <span style={{ marginRight: '8px', color: '#f43f5e' }}>💲</span>
        <input style={{ background: 'transparent', border: 'none', color: '#f8fafc', width: '100%', fontSize: '0.85rem', outline: 'none' }} defaultValue="0.00" />
        <span style={{ color: '#f43f5e', fontWeight: 700 }}>⚠️</span>
      </div>
      <div style={{ fontSize: '0.72rem', color: '#f43f5e', marginTop: '4px' }}>
        ⚠️ Price must exceed minimum tick size boundary ($0.50).
      </div>
    </div>
  </div>

  {/* Slider and Switches */}
  <div style={{ paddingTop: '20px', borderTop: '1px solid rgba(148, 163, 184, 0.12)', display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '24px' }}>
    <div>
      <label style={{ fontSize: '0.75rem', fontWeight: 700, color: '#f8fafc', display: 'block', marginBottom: '8px' }}>
        Slippage Tolerance Slider:
      </label>
      <input type="range" min="0" max="100" defaultValue="35" style={{ width: '100%', accentColor: '#6366f1', cursor: 'pointer' }} />
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.72rem', color: '#94a3b8', marginTop: '4px' }}>
        <span>0.1% Strict</span>
        <span style={{ fontWeight: 700, color: '#6366f1' }}>35% Auto-Hedge</span>
        <span>5.0% Max</span>
      </div>
    </div>

    <div>
      <label style={{ fontSize: '0.75rem', fontWeight: 700, color: '#f8fafc', display: 'block', marginBottom: '8px' }}>
        Execution Switches:
      </label>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <label style={{ display: 'flex', alignItems: 'center', gap: '10px', fontSize: '0.8rem', color: '#cbd5e1', cursor: 'pointer' }}>
          <input type="checkbox" defaultChecked style={{ width: '16px', height: '16px', accentColor: '#6366f1' }} />
          <span>Real-time In-Memory Order Book Streaming (&lt; 1.8 μs)</span>
        </label>
        <label style={{ display: 'flex', alignItems: 'center', gap: '10px', fontSize: '0.8rem', color: '#cbd5e1', cursor: 'pointer' }}>
          <input type="checkbox" defaultChecked style={{ width: '16px', height: '16px', accentColor: '#10b981' }} />
          <span>Hardware Accelerated WebGL Charting</span>
        </label>
      </div>
    </div>
  </div>
</div>

---

## 🦀 Reactive Form Engine & RuleBuilder

The `form_engine` module enforces declarative validation rules that evaluate reactively on every input mutation:

```rust
use ferrox_front_ui::form_engine::{FormField, RuleBuilder};

// 1. Define reactive institutional email field
let email_field = FormField::new("admin@ferrox.exchange")
    .with_rules(
        RuleBuilder::new()
            .required("Institutional email is mandatory.")
            .email("Must be a valid RFC 5322 email syntax.")
            .build()
    );

// 2. Define minimum order quantity field
let qty_field = FormField::new(10.0)
    .with_rules(
        RuleBuilder::new()
            .min_val(1.0, "Minimum order lot size is 1.0 unit.")
            .max_val(500.0, "Single order risk cap is 500.0 units.")
            .build()
    );

// 3. Evaluate validation state
if email_field.validate() && qty_field.validate() {
    web_sys::console::log_1(&"Form validated: dispatching to Wasm linear memory".into());
}
```

---

## 🕹️ Interactive Multi-Step Form Wizard

Test the full multi-step reactive validation onboarding wizard compiled in WebAssembly below:

<FerroxWasmPlayground initialTab="form" height="760px" title="Ferrox Front • Form Validation Wizard" />
