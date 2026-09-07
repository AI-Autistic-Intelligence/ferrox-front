---
id: security
title: Zero-Trust Security & E2EE
sidebar_position: 8
---

# 🔒 Zero-Trust Security in WebAssembly (`ferrox-front-security`)

In Ferrox Front, security is enforced directly inside WebAssembly bytecode, isolating application memory from browser-based script attacks. The `ferrox-front-security` crate provides a complete suite of Zero-Trust mechanisms: from memory anti-tampering to biometric passkeys.

---

## 🛡️ 1. `SealedToken` & E2EE (End-to-End Encryption)

Session tokens (JWT/PASETO) are stored within WebAssembly's private linear memory space and are symmetrically encrypted (sealed). 

Malicious JavaScript scripts (injected via XSS) or rogue browser extensions cannot inspect WebAssembly memory space to extract raw session tokens.

### Payload Signing & Memory Sealing

```rust
use ferrox_front_security::e2ee::{SealedToken, sign_payload};

// 1. Seal a JWT token securely into Wasm linear memory
let token = SealedToken::new("ey...my.jwt.token");

// (Later) Unseal the token only when needed for an authenticated request
let raw_token = token.unseal();

// 2. Sign a payload payload using WebCrypto HMAC to prevent Man-in-the-Middle (MITM) tampering
let payload = r#"{"action": "transfer", "amount": 5000}"#;
let signature = sign_payload(payload);

// You can now securely send `payload` and `signature` to the backend.
```

---

## 🔑 2. `<Secure>` Component & DOM Anti-Tampering (RBAC)

The `secure` component evaluates user identity permissions before mounting nodes into the DOM. 
If a user lacks the required role or permission, **the DOM node is never instantiated or rendered**. This prevents attackers from unhiding administrative buttons via browser devtools (Anti-Tampering protection).

```rust
use ferrox_front_security::rbac::{secure, UserIdentity, Role};
use ferrox_front_core::dom::button;

let user = UserIdentity {
    id: "usr_99".to_string(),
    role: Role::User,
    permissions: vec!["reports:read".to_string()],
};

// This button will NOT be created in the DOM if the user lacks "admin:delete"
// Admins automatically bypass permission checks.
let delete_button = secure(
    &user, 
    "admin:delete", 
    button().attr("class", "btn-danger").text("Format Database")
);
```

---

## 👆 3. Passwordless WebAuthn Biometric Passkeys

`ferrox-front-security` includes native WebAssembly bindings to trigger device biometric sensors (Touch ID, Face ID, Windows Hello) via FIDO2 Passkeys APIs, replacing vulnerable passwords.

```rust
use ferrox_front_security::passkey::{prompt_passkey_login, register_passkey};

// Trigger device biometric sensor to register a new Passkey
// (e.g., during user signup)
register_passkey();

// Later, trigger the biometric sensor to authenticate the user securely
prompt_passkey_login();
```
