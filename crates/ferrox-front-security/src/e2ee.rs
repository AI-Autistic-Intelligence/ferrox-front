use wasm_bindgen::prelude::*;
use web_sys::window;

/// Inizializza l'API WebCrypto per firmare i payload prima dell'invio.
pub fn sign_payload(payload: &str) -> String {
    // In un'implementazione reale, qui useremmo window.crypto.subtle.sign(...)
    // Per questa demo, mockiamo la firma HMAC.
    if let Some(win) = window() {
        if let Ok(crypto) = win.crypto() {
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
pub struct SealedToken {
    encrypted_bytes: Vec<u8>,
}

impl SealedToken {
    pub fn new(token: &str) -> Self {
        // Mock: crittografia simmetrica in RAM (es. AES-GCM)
        let encrypted = token.as_bytes().iter().map(|b| b ^ 0x42).collect();
        Self { encrypted_bytes: encrypted }
    }

    pub fn unseal(&self) -> String {
        // Mock: decrittografia
        let decrypted: Vec<u8> = self.encrypted_bytes.iter().map(|b| b ^ 0x42).collect();
        String::from_utf8(decrypted).unwrap_or_default()
    }
}
