//! # Design Tokens Submodule (`ferrox-front-ui::tokens`)
//!
//! Provides constants and CSS custom property helpers for Ferrox Front's Glassmorphism Design System.

pub struct DesignTokens;

impl DesignTokens {
    // Spacing Tokens
    pub const SPACE_XS: &'static str = "0.25rem";
    pub const SPACE_SM: &'static str = "0.5rem";
    pub const SPACE_MD: &'static str = "1rem";
    pub const SPACE_LG: &'static str = "1.5rem";
    pub const SPACE_XL: &'static str = "2rem";

    // Border Radius Tokens
    pub const RADIUS_SM: &'static str = "4px";
    pub const RADIUS_MD: &'static str = "8px";
    pub const RADIUS_LG: &'static str = "12px";
    pub const RADIUS_FULL: &'static str = "9999px";

    // Glassmorphism FX Tokens
    pub const BACKDROP_BLUR: &'static str = "backdrop-filter: blur(12px);";
    pub const BORDER_GLASS: &'static str = "border: var(--border-glass);";
    pub const SHADOW_GLASS: &'static str = "box-shadow: 0 8px 32px rgba(0, 0, 0, 0.37);";
}
