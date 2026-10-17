---
id: surfaces
title: Surfaces, Cards, Badges & Alerts
sidebar_position: 4
---

import FerroxWasmPlayground from '@site/src/components/FerroxWasmPlayground';

# 🪟 Surfaces, Cards, Badges & Alerts

Ferrox Front surfaces are built using hardware-accelerated frosted glass (`backdrop-filter`) and high-contrast semantic indicators that provide clear feedback during high-velocity trading and data analytics.

---

## 🚀 Live Interactive Component Preview (Bootstrap-Style)

<div style={{ padding: '24px', borderRadius: '16px', background: 'rgba(15, 23, 42, 0.72)', border: '1px solid rgba(148, 163, 184, 0.14)', margin: '24px 0', backdropFilter: 'blur(20px)' }}>
  {/* Semantic Badges */}
  <div style={{ marginBottom: '24px' }}>
    <div style={{ fontSize: '0.75rem', fontWeight: 700, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.05em', marginBottom: '12px' }}>
      Semantic Status Badges:
    </div>
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '10px', alignItems: 'center' }}>
      <span style={{ padding: '4px 10px', borderRadius: '999px', fontSize: '0.75rem', fontWeight: 700, background: 'rgba(16, 185, 129, 0.15)', color: '#10b981', border: '1px solid rgba(16, 185, 129, 0.3)' }}>
        ● MATCHED
      </span>
      <span style={{ padding: '4px 10px', borderRadius: '999px', fontSize: '0.75rem', fontWeight: 700, background: 'rgba(245, 158, 11, 0.15)', color: '#f59e0b', border: '1px solid rgba(245, 158, 11, 0.3)' }}>
        ● PARTIAL FILL
      </span>
      <span style={{ padding: '4px 10px', borderRadius: '999px', fontSize: '0.75rem', fontWeight: 700, background: 'rgba(244, 63, 94, 0.15)', color: '#f43f5e', border: '1px solid rgba(244, 63, 94, 0.3)' }}>
        ● CANCELLED
      </span>
      <span style={{ padding: '4px 10px', borderRadius: '999px', fontSize: '0.75rem', fontWeight: 700, background: 'rgba(6, 182, 212, 0.15)', color: '#06b6d4', border: '1px solid rgba(6, 182, 212, 0.3)' }}>
        ● RESTING LIQUIDITY
      </span>
      <span style={{ padding: '4px 10px', borderRadius: '999px', fontSize: '0.75rem', fontWeight: 700, background: 'rgba(99, 102, 241, 0.15)', color: '#a5b4fc', border: '1px solid rgba(99, 102, 241, 0.3)' }}>
        ⚡ ROUTED VIA WASM
      </span>
    </div>
  </div>

  {/* Card Surface Sample */}
  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '16px', marginBottom: '24px' }}>
    <div style={{ padding: '20px', borderRadius: '14px', background: 'rgba(30, 41, 59, 0.6)', border: '1px solid rgba(148, 163, 184, 0.15)', boxShadow: '0 10px 25px -5px rgba(0,0,0,0.4)' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '12px' }}>
        <span style={{ fontSize: '0.8rem', fontWeight: 700, color: '#f8fafc' }}>Order Book Liquidity</span>
        <span style={{ fontSize: '0.7rem', color: '#10b981', fontWeight: 600 }}>+14.8% 24h</span>
      </div>
      <div style={{ fontSize: '1.6rem', fontWeight: 900, color: '#f8fafc', letterSpacing: '-0.02em' }}>$148,920,400</div>
      <div style={{ fontSize: '0.75rem', color: '#94a3b8', marginTop: '4px' }}>Deep resting liquidity across 12 institutional tiers.</div>
    </div>

    <div style={{ padding: '20px', borderRadius: '14px', background: 'rgba(30, 41, 59, 0.6)', border: '1px solid rgba(148, 163, 184, 0.15)', boxShadow: '0 10px 25px -5px rgba(0,0,0,0.4)' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '12px' }}>
        <span style={{ fontSize: '0.8rem', fontWeight: 700, color: '#f8fafc' }}>Matching Engine Speed</span>
        <span style={{ fontSize: '0.7rem', color: '#38bdf8', fontWeight: 600 }}>Ultra Low Latency</span>
      </div>
      <div style={{ fontSize: '1.6rem', fontWeight: 900, color: '#38bdf8', letterSpacing: '-0.02em' }}>1.8 μs</div>
      <div style={{ fontSize: '0.75rem', color: '#94a3b8', marginTop: '4px' }}>Deterministic tick processing in Rust WebAssembly.</div>
    </div>
  </div>

  {/* Alert Banner */}
  <div style={{ padding: '14px 18px', borderRadius: '10px', background: 'rgba(16, 185, 129, 0.12)', border: '1px solid rgba(16, 185, 129, 0.3)', color: '#6ee7b7', display: 'flex', alignItems: 'center', gap: '12px', fontSize: '0.82rem' }}>
    <span style={{ fontSize: '1.2rem' }}>🛡️</span>
    <div>
      <strong>Zero-Copy WebAssembly Pipeline:</strong> All UI surfaces and data grids render without intermediate JavaScript memory cloning or garbage collection pauses.
    </div>
  </div>
</div>

---

## 🦀 Rust API Implementation

Construct glass cards and metric surfaces in pure Rust:

```rust
use ferrox_front_ui::components::{badge, alert};
use ferrox_front_core::dom::{div, h3, p, DomBuilder};

pub fn sample_surface() -> DomBuilder {
    div()
        .class("ferrox-card rounded-2xl backdrop-blur-2xl p-6 shadow-2xl")
        .children(vec![
            div().class("flex items-center justify-between pb-4 border-b border-slate-800")
                .children(vec![
                    h3().class("text-base font-bold text-white").text("Institutional Security Node"),
                    badge("ACTIVE WASM", "primary"),
                ]),
            p().class("text-xs text-slate-400 mt-3")
                .text("Session credentials sealed inside WebAssembly isolated linear memory."),
            alert("Cryptographic integrity verified under FIPS 140-3 guidelines.", "success"),
        ])
}
```

---

## 🕹️ Interactive Design System Catalog

<FerroxWasmPlayground initialTab="components" height="740px" title="Ferrox Front • UI Surfaces Catalog" />
