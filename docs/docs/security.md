---
id: security
title: Zero-Trust Security & RBAC
sidebar_position: 8
---

# 🔒 Zero-Trust Security in WebAssembly

In Ferrox Front, security is enforced directly inside WebAssembly bytecode, isolating application memory from browser-based script attacks.

---

## 🛡️ 1. `SealedToken` Memory Protection

Session tokens (JWT/PASETO) are stored within WebAssembly's private linear memory space.

Malicious JavaScript scripts (injected via XSS) cannot inspect WebAssembly memory space to extract session tokens.

---

## 🔑 2. `<Secure>` Component & DOM Anti-Tampering

The `secure` component evaluates user identity permissions before mounting nodes into the DOM. If a user lacks the required role or permission, **the DOM node is never instantiated or rendered** (Anti-Tampering protection).

```rust
use ferrox_front_security::{secure, UserIdentity, Role};
use ferrox_front_core::dom::button;

let user = UserIdentity {
    id: "usr_99".to_string(),
    role: Role::User,
    permissions: vec!["reports:read".to_string()],
};

// This button will NOT be created in the DOM if the user lacks "admin:delete"
let delete_button = secure(
    &user, 
    "admin:delete", 
    button().attr("class", "btn-danger").text("Format Database")
);
```

---

## 👆 3. WebAuthn Biometric Passkeys

`ferrox-front-security` includes native WebAssembly bindings to trigger device biometric sensors (Touch ID, Face ID, Windows Hello) via FIDO2 Passkeys APIs.
