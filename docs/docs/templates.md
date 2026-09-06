---
id: templates
title: SaaS Page Templates
sidebar_position: 9
---

# 🏢 Pre-Built SaaS Page Templates (`ferrox-front-templates`)

`ferrox-front-templates` provides pre-built SaaS layouts to accelerate enterprise frontend development.

---

## 💳 1. Pricing Table Template (`pricing_page`)

```rust
use ferrox_front_templates::pricing_page;

let page = pricing_page();
```

### Highlights:
- Dynamic Monthly / Annual billing toggle.
- Starter, Pro, and Enterprise feature comparison cards.

---

## 🔑 2. Authentication Template (`login_page`)

```rust
use ferrox_front_templates::login_page;

let page = login_page();
```

### Highlights:
- 0-Trust login form.
- Passkeys biometric integration & password recovery flow.
