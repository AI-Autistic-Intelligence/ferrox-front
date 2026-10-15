# Changelog - Ferrox Ecosystem

Tutte le modifiche rilevanti apportate all'ecosistema Ferrox sono documentate in questo file.
Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/) e aderisce a [Semantic Versioning](https://semver.org/).

---

## [2.0.0] - 2026-10-15

### ⚡ 1. Live Trading Desk & Disaccoppiamento Matching Engine (DRY Core)
- **`ferrox-match / ferrox-engine`**:
  - Estratta la logica core dell'Order Book in una crate Rust pura indipendente da N-API ed esportata come libreria universale (`rlib` e `cdylib`).
  - Supporto multi-target: compilazione nativa per server Linux/x86_64 e target `wasm32-unknown-unknown` per simulazioni client-side a latenza zero (`< 1.8 μs`).
  - API core standardizzata: `OrderBook::new()`, `submit_order()`, `get_depth()`, `cancel_order()`, `best_bid()`, `best_ask()`, `mid_price()`, `spread()`.
  - Gestione temporale agnostica: `js_sys::Date::now()` su WebAssembly, `SystemTime` su runtime nativo.
  - Suite di unit test completa con verifica di matching FIFO, esecuzioni parziali e cancellazione ordini.
- **`ferrox-front-charts`**:
  - Implementato `depth_chart(...)`: rendering vettoriale SVG puro della profondità di mercato (Order Book Depth) con curve cumulative Bids (emerald) e Asks (rose), linea centrale di mid-market price e pillola di spread calcolato con percentuale.
  - Implementato `CandleAggregator`: aggregatore reattivo di trade `TradeTick { price, volume, timestamp }` nella candela aperta corrente per intervalli da 1 secondo a 1 giorno, con riversamento continuo in `CandleRingBuffer`.
- **`ferrox-front-ws`**:
  - Riscritto il client WebAssembly `ReconnectingWs` con ciclo di vita esplicito `WsStatus`.
  - Exponential backoff (`delay * 1.5`, tetto 30s) combinato con **randomized jitter** (±20%) per la mitigazione dei fenomeni di Thundering Herd.
  - Coda offline in memoria (`message_queue`) per l'accodamento trasparente dei messaggi inviati durante disconnessioni transitorie.
  - Dispatcher tipizzato di eventi per canali `Trades`, `OrderBookDepth` e `Ticker`.
- **`examples/admin-dashboard`**:
  - Tab "Live Trading Desk Pro" unificato che compone esclusivamente le librerie astratte dell'ecosistema: grafico Candlestick con SMA 20/50, Bollinger Bands, RSI e MACD, Order Book Depth SVG, modulo Order Entry validato con `form_engine`, Order Book Ladder L2 e feed dei trade recenti.

---

### 📚 2. Documentazione Interattiva Wasm & Playground Docusaurus
- **Compilazione & Distribuzione Wasm**:
  - Asset pipeline con Trunk che compila l'applicazione showcase direttamente in `ferrox-docs/static/wasm/admin-dashboard/`.
- **Componente Playground React/MDX**:
  - Creato `FerroxWasmPlayground.tsx` isolato per Docusaurus SSR (`BrowserOnly`), con switch di tab (`#trading`, `#datagrid`, `#form`, `#explorer`), pulsante di ricarica istanza, modalità a schermo intero (`requestFullscreen`), indicatore live e badge di telemetria.
- **Pagine di Documentazione Approfondite**:
  - `docs/ferrox-front/overview.md`: Visione zero-JS, Signals reactivity (`Signal<T>`), confronto prestazionale dettagliato con React 19 / Virtual DOM.
  - `docs/ferrox-front/trading-desk.md`: Pagina interattiva con il componente `<FerroxWasmPlayground />` integrato per l'esecuzione diretta nel browser.
  - `docs/ferrox-front/components.md`: Catalogo di bottoni avanzati, form controls, switch toggle, modali e 60+ icone vettoriali pure Rust.
  - `docs/ferrox-front/datagrid.md`: Guida all'Enterprise DataGrid a 1M righe con ordinamento, filtri ed export CSV RFC 4180.
  - `docs/ferrox-front/charts.md`: Documentazione del motore finanziario vettoriale SVG (Candlestick, Depth Chart, indicatori tecnici e aggregatore di tick).

---

### 🚀 3. Benchmark 1 Million Requests / Second (Whitepaper Ufficiale)
- **Pubblicazione Whitepaper `1m-rps.md`**:
  - Integrazione ufficiale dei dati prestazionali del progetto `1MilionRequestPerSecond` testato su carico enterprise (8.0 vCPU limit, PostgreSQL con 100.000 record, rate-limiting Redis distribuito).
  - Grafici comparativi vettoriali SVG integrati:
    1. **Throughput (Requests/sec)**: Node.js (14.781 RPS) e Rust (13.469 RPS) al vertice, seguiti da PHP (10.627 RPS), Python (7.120 RPS) e Java (3.639 RPS).
    2. **Latenza p99 (Consistency)**: Rust trionfa con **333 ms** (più di 2x più consistente di Node a 677 ms e Python a 815 ms), confermando l'assoluta superiorità dell'assenza di Garbage Collection.
    3. **Consumo RAM**: Rust consuma appena **38 MB**, contro i 320 MB di Node e gli 890 MB della JVM.
  - Istruzioni complete di riproduzione locale tramite Docker Compose.
  - Voce di navigazione dedicata inserita nella navbar e nella sidebar di `ferrox-docs`.

---

### 📦 4. Release Validation, Packaging & CI/CD Pipeline
- **Audit Crate & Metadati**:
  - Verificato `Cargo.toml` su tutti i crate di `ferrox` e `ferrox-front`: metadati completi (description, repository, license, documentation, homepage, readme, keywords, categories).
  - Eliminata qualsiasi dipendenza a percorso locale senza specifica di versione (`path = ...` con associato `version = "..."`).
  - Script topologici di pubblicazione `publish_crates.js` creati e testati per `ferrox` e `ferrox-front`.
- **Workflow GitHub Actions (`release-pipeline.yml`)**:
  - Matrice multipiattaforma (`ubuntu-latest`, `windows-latest`, `macos-latest`).
  - Build e ottimizzazione WebAssembly automatica (`wasm-opt -Oz`).
  - Build multi-architettura per immagini Docker container (`linux/amd64`, `linux/arm64`).
  - Pubblicazione automatica sequenziale su crates.io su rilascio di tag `v*`.

---

### 🛡️ 5. Security Elevation & Hardening (OWASP WSTG)
- **Integrazione Security Sidecar**:
  - `ferrox-sentinel` e `ferrox-pentest-private` per la scansione continua di vulnerabilità (SQLi, NoSQLi, JWT/PASETO token manipulation, SSRF, XSS).
  - Token a tenuta stagna `SealedToken` in memoria WebAssembly con crittografia WebCrypto API.
  - Container Docker rinforzati con utenti non-root, file system radice in sola lettura e quote cgroups.
