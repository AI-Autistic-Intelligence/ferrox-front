---
id: overview
title: Visione Zero-JS & Architettura
sidebar_position: 1
---

# Ferrox Front: Framework WebAssembly Enterprise Zero-JS

**Ferrox Front** è un framework frontend e Design System enterprise all'avanguardia scritto interamente in **Rust WebAssembly (`wasm32-unknown-unknown`)**, progettato specificamente per eliminare JavaScript, React e i bundler npm dalle applicazioni web mission-critical e dai sistemi finanziari ad alta frequenza.

---

## 🚀 Perché Zero-JS e Rust WebAssembly?

Nel software moderno ad alte prestazioni (trading desks, cruscotti analitici, streaming di dati in tempo reale), lo stack JavaScript/TypeScript tradizionale presenta colli di bottiglia insormontabili:

1. **Virtual DOM Overhead**: Diffing continuo di alberi di nodi virtuali ad ogni re-render.
2. **Garbage Collection (GC) Stutters**: Pausa imprevista del thread principale per il recupero della memoria, inaccettabile durante il processing di flussi di quotazioni L2/L3.
3. **Fragilità delle Dipendenze**: Migliaia di pacchetti `npm`, vulnerabilità nella supply chain e complessità di bundling.
4. **Assenza di Condivisione Reale della Logica**: Duplicazione delle strutture dati tra backend (Rust/C++) e frontend (TypeScript).

Ferrox Front ribalta questo paradigma: **l'intera interfaccia grafica, la reattività a segnali, il motore di matching dell'order book e la renderizzazione dei grafici risiedono in un singolo binario Wasm compilato e ottimizzato**.

---

## ⚡ Confronto Prestazionale: Ferrox Front vs React 19

| Metrica | React 19 + Shadcn / TanStack | Ferrox Front (Rust Wasm) | Vantaggio Ferrox |
| :--- | :--- | :--- | :--- |
| **Runtime Engine** | JavaScript V8 / Hermes | Compilato Nativo Wasm32 | **Nessuna interpretazione JIT** |
| **Modello Reattivo** | Virtual DOM Tree Diffing | **Signals Fine-Grained** (`Signal<T>`) | **Mutazione O(1) diretta del DOM** |
| **Garbage Collector** | Tracing GC continuo | **RAII Rust deterministico** | **Zero GC Pauses** |
| **DataGrid (100.000 righe)** | Lag di renderizzazione / freeze | Virtualizzazione fluida a 60 FPS | **Nessun memory leak** |
| **Motore Finanziario Matching** | Proxy HTTP / Web Worker JS lento | `ferrox-engine` compilato in-process | **Latenza &lt; 1.8 μs** |
| **Grafici Tecnici (TradingView)** | Canvas pesante / licenza proprietaria | **SVG Vettoriale Puro Zero-Copy** | **Nessuna libreria JS esterna** |
| **Dipendenze NPM Esterne** | 1,200+ pacchetti `node_modules` | **0 pacchetti JS / 0 npm** | **Massima sicurezza supply chain** |

---

## 🧩 Ecosistema Modulare dei Crate Core

In ottemperanza alla regola architetturale **DRY (Don't Repeat Yourself)**, nessuna logica di calcolo, componente visivo o protocollo è vincolato a showcase app o esempi. L'ecosistema è ripartito in crate indipendenti e riutilizzabili:

```mermaid
graph TD
    A["ferrox-front-core<br/>(Signals & DOM Builder)"] --> B["ferrox-front-ui<br/>(Design System & DataGrid)"]
    A --> C["ferrox-front-charts<br/>(SVG Technical & Depth Charts)"]
    D["ferrox-match / ferrox-engine<br/>(Pure Rust Order Book)"] --> E["Live Trading Desk<br/>(Wasm Reference App)"]
    F["ferrox-front-ws<br/>(ReconnectingWs + Events)"] --> E
    B --> E
    C --> E
    G["ferrox-front-router<br/>(SPA Routing & Guards)"] --> E
```

### 1. `crates/ferrox-front-core`
- **Signals Fine-Grained**: Primitive reattive `Signal<T>`, `create_signal`, `create_effect`, `create_memo`.
- **Wasm DOM Builder**: Costruttore fluido (`div()`, `span()`, `button()`) per la manipolazione diretta del DOM senza Virtual DOM.
- **Dynamic Theming Engine**: Switch dinamico a runtime tra temi Enterprise Glassmorphism, Cyber Dark, Zinc Light, Cyberpunk e Ocean Breeze.

### 2. `crates/ferrox-front-charts`
- **Motore Vettoriale SVG**: Generazione di grafici senza Canvas o librerie esterne.
- **Financial Technical Charts**: Candlestick a timeframe dinamico con SMA 20/50, Bollinger Bands (±2σ), RSI (14) e MACD (12, 26, 9) differential histogram.
- **Order Book Depth Chart**: `depth_chart(...)` con curve cumulative per Bids/Asks, mid-market price line e calcolo automatico dello spread.
- **Streaming Aggregator**: `CandleAggregator` che converte i singoli tick `TradeTick` in candele aperte e chiuse con `CandleRingBuffer`.

### 3. `crates/ferrox-front-ws`
- **`ReconnectingWs`**: Client WebSocket resiliente per browser WebAssembly con riconnessione esponenziale e jitter casuale (±20%).
- **Offline Queue**: Buffer di accodamento messaggi che previene la perdita di payload durante disconnessioni transitorie.
- **Typed Event Dispatcher**: Parsing strongly-typed per eventi `Trades`, `OrderBookDepth` e `Ticker`.

### 4. `ferrox-match/packages/ferrox-engine`
- **Motore di Matching Puro**: `OrderBook` in memoria con `BTreeMap` a virgola fissa moltiplicata (`1e8`).
- **Supporto Multi-Target**: Compila sia per server Linux/Windows che come dipendenza Rust pura per target `wasm32-unknown-unknown` a latenza zero.

---

## 🛠️ Hello World con Signals e DOM Builder

```rust
use ferrox_front_core::dom::{mount, div, h1, button};
use ferrox_front_core::reactivity::create_signal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Stato reattivo a grana fine
    let count = create_signal(0);

    // Costruzione fluida del DOM senza Virtual DOM
    let root = div()
        .class("flex flex-col items-center justify-center p-8 bg-slate-900 rounded-2xl border border-slate-800")
        .child(
            h1()
                .class("text-2xl font-bold text-white mb-4")
                .text(&format!("Conteggio: {}", count.get()))
        )
        .child(
            button()
                .class("px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg font-semibold")
                .text("Incrementa")
                .on_click(move || {
                    count.update(|n| *n += 1);
                })
        );

    mount("#root", root);
    Ok(())
}
```

Nelle sezioni successive della documentazione esploreremo nel dettaglio i **Componenti UI**, l'**Enterprise DataGrid**, il **Motore Finanziario Vettoriale** e il **Trading Desk Interattivo** eseguibile direttamente nel browser.
