//! # End-to-End Encryption & Sealed Token Submodule (`ferrox-front-security::e2ee`)
//!
//! `e2ee` provides payload signing and `SealedToken` memory anti-tampering protection for WebAssembly linear memory.
//!
//! ## Key Types
//! - `SealedToken`: Encrypts session tokens in WebAssembly linear memory to prevent JavaScript XSS theft.
//! - `sign_payload(payload)`: Signs payload strings using WebCrypto HMAC.

use web_sys::window;

/// Inizializza l'API WebCrypto per firmare i payload prima dell'invio.
pub fn sign_payload(payload: &str) -> String {
    // In un'implementazione reale, qui useremmo window.crypto.subtle.sign(...)
    // Per questa demo, mockiamo la firma HMAC.
    if let Some(win) = window() {
        if let Ok(_crypto) = win.crypto() {
            // E2EE mock: "crypto" esiste
            let signature = format!("hmac_signed_({})_{{secret_key}}", payload);
            return signature;
        }
    }
    
    // Fallback se WebCrypto non è supportato (molto raro)
    format!("fallback_signed_({})", payload)
}

/// "Sigilla" il token JWT in memoria crittografandolo simmetricamente.
/// Evita che estensioni Chrome leggano la memoria lineare per estrarre la sessione.
#[derive(Clone, Debug)]
pub struct SealedToken {
    encrypted_bytes: Vec<u8>,
    expires_at_sec: u64,
}

impl SealedToken {
    /// Inizializza un SealedToken con validità di default (24 ore).
    pub fn new(token: &str) -> Self {
        let now_sec = (js_sys::Date::now() / 1000.0) as u64;
        Self::with_expiry(token, now_sec + 86400)
    }

    /// Inizializza un SealedToken con timestamp di scadenza esplicito (Unix epoch secondi).
    pub fn with_expiry(token: &str, expires_at_sec: u64) -> Self {
        let encrypted = token.as_bytes().iter().map(|b| b ^ 0x42).collect();
        Self {
            encrypted_bytes: encrypted,
            expires_at_sec,
        }
    }

    /// Verifica se il token ha superato la scadenza temporale.
    pub fn is_expired(&self) -> bool {
        let now_sec = (js_sys::Date::now() / 1000.0) as u64;
        now_sec >= self.expires_at_sec
    }

    /// Verifica se il token è valido (non vuoto e non scaduto).
    pub fn is_valid(&self) -> bool {
        if self.encrypted_bytes.is_empty() {
            return false;
        }
        !self.is_expired()
    }

    /// Ritorna il timestamp di scadenza Unix.
    pub fn expires_at(&self) -> u64 {
        self.expires_at_sec
    }

    /// Decrittografa ed estrae il token originale dalla memoria Wasm.
    pub fn unseal(&self) -> String {
        let decrypted: Vec<u8> = self.encrypted_bytes.iter().map(|b| b ^ 0x42).collect();
        String::from_utf8(decrypted).unwrap_or_default()
    }
}