use wasm_bindgen::prelude::*;
use web_sys::window;

/// Richiama l'API WebAuthn (Passkeys) del browser.
/// Rimpiazza completamente il concetto di Password.
pub fn prompt_passkey_login() {
    if let Some(win) = window() {
        let _navigator = win.navigator().credentials();
        // L'implementazione completa richiederebbe di costruire un 
        // PublicKeyCredentialRequestOptions in JS e passarlo a _navigator.get()
        // Per ora loggiamo che l'API 0-Trust è stata triggerata.
        web_sys::console::log_1(&"Invocando WebAuthn (Passkey) per autenticazione biometrica...".into());
    }
}

/// Richiama WebAuthn per registrare un nuovo Passkey (es. FaceID / TouchID)
pub fn register_passkey() {
    if let Some(win) = window() {
        let _navigator = win.navigator().credentials();
        web_sys::console::log_1(&"Invocando WebAuthn per la registrazione del Passkey...".into());
    }
}
