---
sidebar_position: 2
---

# 🤖 CI/CD Automation & GitHub Actions

Automating testing, code quality linting, security vulnerability auditing, and documentation deployment ensures continuous delivery without regressions.

This guide provides complete GitHub Actions workflow configurations.

---

## 1. CI Pipeline Workflow (`.github/workflows/ci.yml`)

```yaml
name: Ferrox CI Pipeline

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  check-and-test:
    name: Code Quality & Tests
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Rust Cache
        uses: Swatinem/rust-cache@v2

      - name: Check Format
        run: cargo fmt --all -- --check

      - name: Run Clippy Linter
        run: cargo clippy --workspace -- -D warnings

      - name: Run Workspace Unit Tests
        run: cargo test --workspace

  security-audit:
    name: Security Vulnerability Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check-action@v1.4.1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

---

## 2. Documentation Build & Deploy Workflow (`.github/workflows/docs.yml`)

```yaml
name: Deploy Docusaurus Portal

on:
  push:
    branches: [ main ]
    paths:
      - 'docs/**'

jobs:
  deploy-docs:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: docs

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'npm'
          cache-dependency-path: docs/package-lock.json

      - name: Install Dependencies
        run: npm ci

      - name: Build Docusaurus Website
        run: npm run build

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/build
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*