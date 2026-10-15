---
id: charts
title: Motore Finanziario Vettoriale SVG
sidebar_position: 5
---

# Motore Grafico Finanziario (`ferrox-front-charts`)

`ferrox-front-charts` è una libreria vettoriale SVG ad altissima precisione progettata specificamente per cruscotti finanziari, trading desk e visualizzazioni quantitative, con **zero dipendenze JavaScript o Canvas**.

---

## 📈 Candlestick Pro con Indicatori Tecnici (`financial_technical_chart`)

Il grafico tecnico principale offre parità funzionale rispetto a strumenti di riferimento come TradingView Lightweight Charts, integrando calcoli matematici nativi in Rust per gli indicatori più utilizzati:

```rust
use ferrox_front_charts::{
    CandleDatum, TechnicalChartOptions, financial_technical_chart,
};

let candle_history: Vec<CandleDatum> = vec![
    CandleDatum { time: "18:00".into(), open: 96250.0, high: 96800.0, low: 96100.0, close: 96600.0, volume: 510.0 },
    CandleDatum { time: "18:30".into(), open: 96600.0, high: 97200.0, low: 96400.0, close: 97100.0, volume: 640.0 },
];

let mut options = TechnicalChartOptions::default();
options.show_sma_20 = true;       // Media mobile semplice 20 periodi (linea ambra)
options.show_sma_50 = true;       // Media mobile semplice 50 periodi (linea azzurra)
options.show_bollinger = true;    // Bande di Bollinger (±2 deviazioni standard)
options.show_rsi = true;          // Pannello inferiore RSI a 14 periodi con livelli 70/30
options.show_macd = true;         // Pannello inferiore MACD (12, 26, 9) con istogramma

let chart_svg = financial_technical_chart(
    "btc-pro-chart",
    &candle_history,
    &options,
    880, // Larghezza in pixel
    480, // Altezza in pixel
);
```

### Indicatori Matematici Integrati
1. **SMA (Simple Moving Average)**: Calcolo efficiente su finestra mobile O(1) per 20 e 50 periodi.
2. **Bollinger Bands**: Canale di volatilità dinamico con deviazione standard empirica calcolata in Rust a 64-bit floating point.
3. **RSI (Relative Strength Index)**: Oscillatore di momentum a 14 periodi con smoothing di Wilder ed evidenziazione zone ipercomprato (&gt;70) e ipervenduto (&lt;30).
4. **MACD (Moving Average Convergence Divergence)**: Calcolo EMA 12 ed EMA 26, linea di segnale EMA 9 e barre dell'istogramma differenziale bicolore (verde/rosso).

---

## 🌊 Order Book Market Depth Chart (`depth_chart`)

Il grafico di profondità di mercato (`depth_chart`) visualizza graficamente la liquidità dell'Order Book:

```rust
use ferrox_front_charts::depth_chart;

// Livelli cumulativi di liquidità (prezzo, volume aggregato)
let bids = vec![(97040.0, 1.25), (97020.0, 4.65), (97000.0, 11.45), (96950.0, 26.65)];
let asks = vec![(97060.0, 0.95), (97080.0, 3.75), (97100.0, 9.65), (97150.0, 23.75)];

let depth_svg = depth_chart(
    "btc-depth-chart",
    &bids,
    &asks,
    560, // Larghezza
    280, // Altezza
);
```

### Elementi del Grafico di Profondità
- **Area Verde Emerald (Bids)**: Visualizzazione cumulativa della domanda dei compratori a prezzi discendenti.
- **Area Rossa Rose (Asks)**: Visualizzazione cumulativa dell'offerta dei venditori a prezzi ascendenti.
- **Mid-Market Price Line**: Linea tratteggiata verticale che indica il prezzo medio istantaneo.
- **Spread Badge**: Pillola centrale con il valore assoluto dello spread bid-ask e la percentuale relativa (es. `Spread: $10.00 (0.01%)`).

---

## ⏱️ Streaming dei Trade & Aggregatore di Candele (`CandleAggregator`)

Nelle applicazioni finanziarie in tempo reale, i flussi WebSocket inviano singoli trade atomici. `CandleAggregator` raggruppa questi trade in tempo reale nella candela attualmente aperta, chiudendola automaticamente allo scadere del timeframe:

```rust
use ferrox_front_charts::{CandleAggregator, TradeTick, Timeframe};

// Inizializza l'aggregatore su timeframe 5 minuti con un buffer circolare di 100 candele
let mut aggregator = CandleAggregator::new(Timeframe::Min5, 100);

// Ingestione di trade ricevuti via WebSocket o ferrox-engine
aggregator.process_tick(TradeTick::new(97050.0, 0.45, 1760518800000));
aggregator.process_tick(TradeTick::new(97055.0, 1.20, 1760518815000));

// Ottieni le candele storiche chiuse più la candela attiva in tempo reale
let current_candles = aggregator.get_candles_with_active();
```

### Timeframe Supportati

| Timeframe | Costante Enum | Durata Millisecondi | Caso d'Uso |
| :--- | :--- | :--- | :--- |
| **1 Secondo** | `Timeframe::Sec1` | 1,000 ms | HFT / Scalping ultra-veloce |
| **5 Secondi** | `Timeframe::Sec5` | 5,000 ms | Monitoraggio micro-struttura |
| **1 Minuto** | `Timeframe::Min1` | 60,000 ms | Analisi intraday a breve termine |
| **5 Minuti** | `Timeframe::Min5` | 300,000 ms | Default Trading Desk standard |
| **15 Minuti** | `Timeframe::Min15` | 900,000 ms | Trend following medio termine |
| **1 Ora** | `Timeframe::Hour1` | 3,600,000 ms | Analisi ciclica oraria |
| **1 Giorno** | `Timeframe::Day1` | 86,400,000 ms | Panoramica macro-economica |

---

## 🎯 Indicatori di Telemetria: Gauges e Sparklines

Ferrox Front include anche componenti compatti per dashboard analitiche:

```rust
use ferrox_front_charts::{gauge_chart, sparkline};

// Gauge tachimetrico semicircolare per Fear & Greed o CPU usage
let cpu_gauge = gauge_chart("cpu-usage", 78.5, 0.0, 100.0, "%", 220, 140);

// Sparkline vettoriale minimalista per trend rapido nelle tabelle
let price_sparkline = sparkline("btc-trend", &[96100.0, 96400.0, 96250.0, 96900.0, 97050.0], 120, 32);
```
