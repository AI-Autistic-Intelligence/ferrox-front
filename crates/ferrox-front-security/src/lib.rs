//! # Ferrox Front Security (`ferrox-front-security`)
//!
//! `ferrox-front-security` enforces zero-trust security on the frontend client. It seals session tokens crittographically
//! in WebAssembly linear memory (`SealedToken`), prevents DOM inspection tampering, and provides declarative RBAC UI guards (`<Secure require="admin">`).
//!
//! ## Security Rationale
//! JavaScript frontend tokens stored in `localStorage` or JavaScript scope are vulnerable to XSS attacks. `ferrox-front-security`
//! stores tokens inside isolated WebAssembly memory boundaries that are strictly unreachable by external JS code.
//!
//! ## Key Features
//! - 🔒 **`SealedToken` Protection**: Cryptographically encrypted token storage in Wasm linear memory.
//! - 🛡️ **Declarative Role Guards**: `<Secure require="admin">` component conditionally unmounts unauthorized elements.
//! - 🔑 **WebAuthn / Passkeys**: Native WebAssembly bindings for biometrics and FIDO2 authentication.

pub mod rbac;
pub mod e2ee;
pub mod passkey;

pub use rbac::*;
pub use e2ee::*;
pub use passkey::*;