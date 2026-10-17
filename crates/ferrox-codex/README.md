# ⚡ Ferrox Codex (`ferrox-codex`)

> **Native WebAssembly Documentation Engine & Microfrontend Platform for the Ferrox Ecosystem**  
> *Zero Node.js • Zero Virtual DOM • Vector SVG Mermaid • Plug-and-Play Microfrontends • AI-Native (`llms.txt`)*

[![Crates.io](https://img.shields.io/crates/v/ferrox-codex.svg)](https://crates.io/crates/ferrox-codex)
[![Documentation](https://docs.rs/ferrox-codex/badge.svg)](https://docs.rs/ferrox-codex)
[![Ecosystem](https://img.shields.io/badge/docs-ferrox--rust.dev-indigo.svg)](https://ferrox-rust.dev)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

---

## 🌟 Overview

**Ferrox Codex** replaces heavy Node.js/React-based documentation frameworks (such as Docusaurus) with a blazing-fast, memory-safe, pure Rust and WebAssembly documentation platform.

It is dogfooded across the entire [Ferrox ecosystem](https://ferrox-rust.dev) to document backend architectures (1,000,000 req/sec), frontend Wasm UI components (DataGrid, SVG Charts, Trading Desk), and polyglot microservices (Node.js, Python, PHP, Java).

---

## 🚀 Key Features

- **🎨 Docusaurus Visual Parity**:
  - **Vector SVG Mermaid**: Native SVG rendering of Sequence Diagrams, Flowcharts, State Diagrams, and Class Diagrams styled with the dark glassmorphic `ferrox-cyber` palette.
  - **Interactive TabGroup**: Reactive multi-panel switcher for multi-language installation guides (Cargo / npm / pip / composer).
  - **Glassmorphic Admonitions**: Modern callouts for `[!NOTE]`, `[!TIP]`, `[!WARNING]`, `[!DANGER]`, `[!CAUTION]`, and `[!INFO]`.
- **🧪 Live WebAssembly Sandboxes & Playgrounds**:
  - **1,000,000-Row Virtualized DataGrid**: 60 FPS real-time rendering, instant column sorting, and full-text filter.
  - **Streaming Financial Charts**: Vector SVG Candlestick & Order Book Depth chart fed by a high-frequency Wasm tick stream simulator (up to 1 kHz).
  - **Reactive Form Validation Engine**: Real-time email RFC validation, JSON schema linting, and password entropy meter.
- **🧩 Plug-and-Play Microfrontend Architecture (`CodexModule`)**:
  - Independent, encapsulated documentation modules for every repository (`core`, `front`, `node`, `py`, `polyglot`).
  - Recursive nested submodules (e.g. `ferrox-py` -> `auth`, `commerce`, `utils`).
  - Embedded live Wasm sandboxes and playgrounds (`interactive_views`).
- **🤖 AI-Native Documentation Layer**:
  - Automated generation of [`llms.txt`](https://ferrox-rust.dev/llms.txt) complying with the [llmstxt.org](https://llmstxt.org) standard for AI agents (Claude, GPT, Gemini, DeepSeek).
  - Full context corpus export via [`llms-full.txt`](https://ferrox-rust.dev/llms-full.txt).
- **⚡ Static Site Generation (SSG) & SEO**:
  - Pre-rendered static HTML routes with rich OpenGraph meta tags (`og:title`, `og:description`, `og:image`, `og:url`, `twitter:card`) for rich Discord, Twitter/X, and LinkedIn social previews.
- **🔍 Instant In-Memory Full-Text Search**:
  - Keyboard-driven `CommandPalette` (`Ctrl+K`) searching titles, headings, tags, and document content with weighted scoring and snippet extraction.
- **🌐 Multi-Language (i18n)**:
  - Built-in support for 8 languages: English, Italian, Spanish, French, German, Chinese, Japanese, and Korean.

---

## 📦 Installation & CLI Usage

Install the native documentation CLI from crates.io:

```bash
cargo install ferrox-codex
```

### CLI Commands

```bash
# Initialize a new documentation starter project
ferrox-codex init [project-name]

# Build release WebAssembly bundle and AI SSG artifacts
ferrox-codex build [--out-dir <dir>]

# Generate llms.txt, llms-full.txt, and static SEO HTML pages
ferrox-codex ssg [--out-dir <dir>]

# Ingest all documentation sources across repositories into a bundle
ferrox-codex sync --sources <path-to-sources> [--out <bundle.json>]

# Compile and export directly to GitHub Pages deployment root
ferrox-codex export-gh-pages

# Launch local development server with hot-reloading
ferrox-codex serve
```

---

## 🧩 Defining a Custom `CodexModule`

```rust
use ferrox_codex::modules::CodexModule;

pub fn create_custom_module() -> CodexModule {
    let mut module = CodexModule::new("analytics", "📊 Ferrox Analytics", "/analytics")
        .with_icon("activity")
        .with_badge("Real-Time");

    module.register_doc("overview", include_str!("../docs/overview.md"));
    module.register_doc("metrics", include_str!("../docs/metrics.md"));

    // Attach recursive submodule
    let mut alerts_sub = CodexModule::new("alerts", "🔔 Real-time Alerts", "alerts");
    alerts_sub.register_doc("rules", include_str!("../docs/alerts/rules.md"));

    module.with_submodule(alerts_sub)
}
```

---

## 📄 License

Dual-licensed under MIT or Apache 2.0. Copyright 2026 Ferrox Architecture Group.
