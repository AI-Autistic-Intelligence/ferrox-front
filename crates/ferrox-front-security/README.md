# Ferrox Front Security (`ferrox-front-security`)

`ferrox-front-security` provides Zero-Trust RBAC and memory-safe cryptography for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-security` crate is the dedicated cryptographic and access-control layer of the Ferrox framework. It provides WebCrypto API wrappers (e.g., AES-256-GCM) and a `SealedToken` primitive for managing authentication state. Its architectural purpose is to eliminate Cross-Site Scripting (XSS) token theft by storing sensitive session credentials exclusively within the isolated WebAssembly linear memory space, making them invisible to malicious JavaScript running in the browser.

## 2. Architectural Layering
This crate operates in parallel with the UI and Routing layers. It relies on `ferrox-front-core` to inject security signals (like user roles or authentication status) into the reactive context. The `<Secure>` component provided by this crate wraps UI elements, allowing the rendering engine to completely omit unauthorized DOM subtrees at the WebAssembly level, rather than just hiding them with CSS.

## 3. How It Works Under the Hood
When a user logs in, the backend issues an encrypted token. Instead of storing this in `localStorage` or JavaScript variables, `ferrox-front-security` ingests it directly into a `SealedToken` struct inside Rust's memory heap. If a secure component (e.g., `<Secure require="admin">`) is evaluated, the macro checks the in-memory Role-Based Access Control (RBAC) claims. If the user lacks the role, the `rsx!` macro returns an empty DOM node or a fallback. 

## 4. Why It Was Designed This Way
Modern SPAs suffer from systemic XSS vulnerabilities because JavaScript's dynamic nature allows third-party scripts to access `window.localStorage` and global variables easily. This crate was designed to leverage WebAssembly's primary security feature: isolated linear memory. By keeping tokens in Wasm and decrypting them just-in-time for API calls using native browser WebCrypto, Ferrox applications achieve a "Zero-Trust" client architecture.

## 5. Practical Usage Guide & Extended Code Examples
Wrap sensitive UI sections in the `Secure` component and manage tokens via the `AuthContext`.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_security::prelude::*;

#[component]
pub fn AdminSettings() -> Element {
    rsx! {
        div { class: "page-wrapper",
            h1 { "Settings Page" }
            
            // Only renders into the DOM if the user has the 'admin' role
            Secure { require: "admin",
                fallback: rsx! { div { class: "alert", "Admin privileges required." } },
                div { class: "danger-zone card",
                    h3 { "Danger Zone" }
                    button { class: "btn-danger", "Delete Database" }
                }
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Client-Side Security as a Replacement for Backend Auth**
> Do not assume that hiding a component via `<Secure>` means the data is secure. Client-side RBAC is strictly for UI/UX purposes (preventing unauthorized users from seeing buttons they cannot click). Your backend API must still strictly validate the `SealedToken` and user roles on every incoming request.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Rotate Keys and Use Short-Lived Tokens**
> While `SealedToken` protects against XSS token extraction, it cannot prevent an attacker with a fully compromised browser from issuing requests *through* the Wasm application. Always use short-lived JWTs (e.g., 15 minutes) coupled with HttpOnly secure cookies for refresh tokens to minimize the attack window.
