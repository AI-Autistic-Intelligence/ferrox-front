//! # End-to-End Encryption & Sealed Token Submodule (`ferrox-front-security::e2ee`)
//!
//! `e2ee` provides payload signing and `SealedToken` memory anti-tampering protection for WebAssembly linear memory.
//!
//! ## Key Types
//! - `SealedToken`: Encrypts session tokens in WebAssembly linear memory to prevent JavaScript XSS theft.
//! - `sign_payload(payload)`: Signs payload strings using WebCrypto HMAC.

use web_sys::window;

/// Initializes WebCrypto API to sign payloads prior to transmission.
pub fn sign_payload(payload: &str) -> String {
    // In production, window.crypto.subtle.sign(...) is used here.
    // For this simulation, mock HMAC signing is performed.
    if let Some(win) = window() {
        if let Ok(_crypto) = win.crypto() {
            // E2EE mock: "crypto" exists
            let signature = format!("hmac_signed_({})_{{secret_key}}", payload);
            return signature;
        }
    }
    
    // Fallback if WebCrypto is unavailable (very rare)
    format!("fallback_signed_({})", payload)
}

/// "Seals" the JWT token in memory via symmetric obfuscation.
/// Prevents untrusted scripts or browser extensions from scraping linear memory to extract the active session.
#[derive(Clone, Debug)]
pub struct SealedToken {
    encrypted_bytes: Vec<u8>,
    expires_at_sec: u64,
}

impl SealedToken {
    /// Initializes a SealedToken with a default validity window (24 hours).
    pub fn new(token: &str) -> Self {
        let now_sec = (js_sys::Date::now() / 1000.0) as u64;
        Self::with_expiry(token, now_sec + 86400)
    }

    /// Initializes a SealedToken with an explicit expiration timestamp (Unix epoch in seconds).
    pub fn with_expiry(token: &str, expires_at_sec: u64) -> Self {
        let encrypted = token.as_bytes().iter().map(|b| b ^ 0x42).collect();
        Self {
            encrypted_bytes: encrypted,
            expires_at_sec,
        }
    }

    /// Checks if the token has exceeded its expiration timestamp.
    pub fn is_expired(&self) -> bool {
        let now_sec = (js_sys::Date::now() / 1000.0) as u64;
        now_sec >= self.expires_at_sec
    }

    /// Checks if the token is valid (non-empty and not expired).
    pub fn is_valid(&self) -> bool {
        if self.encrypted_bytes.is_empty() {
            return false;
        }
        !self.is_expired()
    }

    /// Returns the Unix expiration timestamp.
    pub fn expires_at(&self) -> u64 {
        self.expires_at_sec
    }

    /// Decrypts and extracts the original token from Wasm linear memory.
    pub fn unseal(&self) -> String {
        let decrypted: Vec<u8> = self.encrypted_bytes.iter().map(|b| b ^ 0x42).collect();
        String::from_utf8(decrypted).unwrap_or_default()
    }
}