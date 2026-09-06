//! # Ferrox Front Templates (`ferrox-front-templates`)
//!
//! `ferrox-front-templates` provides pre-built, responsive SaaS page templates for bootstrapping enterprise frontends instantly.
//!
//! ## Key Features
//! - 🏢 **Admin Dashboard Layout**: Sidebar navigation, header controls, metric cards, and data tables.
//! - 💳 **Pricing Tables**: Interactive billing toggle (monthly/annual) and feature matrices.
//! - 🔑 **Auth Pages**: Login, Registration, 2FA prompt, and Password Reset screens.

use ferrox_front_core::dom::{div, h1, h2, h3, p, button, DomBuilder};
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_ui::components::{card, card_body};

pub fn pricing_page() -> DomBuilder {
    let header = div().attr("style", "text-align: center; margin-bottom: 3rem;")
        .child(h1().text("Piani e Prezzi (Stripe Ready)").attr("class", "app-header"))
        .child(p().text("Scegli il piano ideale per il tuo business. Integrato nativamente con Stripe."));

    let hobby_card = col("12 md-4")
        .child(
            card().attr("style", "padding: 2rem; text-align: center;")
            .child(h3().text("Hobby"))
            .child(h2().text("Gratis").attr("style", "color: var(--text-primary); margin: 1rem 0;"))
            .child(p().text("Perfetto per iniziare piccoli progetti."))
            .child(button().attr("class", "ferrox-btn").attr("style", "margin-top: auto;").text("Inizia Ora"))
        );

    let pro_card = col("12 md-4")
        .child(
            card().attr("style", "padding: 2rem; text-align: center; border: 1px solid var(--accent-primary); transform: scale(1.05);")
            .child(h3().text("Pro").attr("style", "color: var(--accent-primary);"))
            .child(h2().text("€29 / mese").attr("style", "color: var(--text-primary); margin: 1rem 0;"))
            .child(p().text("Tutte le funzionalità enterprise."))
            .child(button().attr("class", "ferrox-btn").attr("style", "margin-top: auto; background: var(--accent-primary);").text("Scegli Pro"))
        );

    let ent_card = col("12 md-4")
        .child(
            card().attr("style", "padding: 2rem; text-align: center;")
            .child(h3().text("Enterprise"))
            .child(h2().text("Contattaci").attr("style", "color: var(--text-primary); margin: 1rem 0;"))
            .child(p().text("Supporto dedicato 24/7 e server isolati."))
            .child(button().attr("class", "ferrox-btn").attr("style", "margin-top: auto; background: transparent; border: 1px solid var(--text-secondary);").text("Contatta Vendite"))
        );

    container()
        .attr("class", "fx-container fade-in-up")
        .attr("style", "padding: 4rem 0;")
        .child(header)
        .child(
            row()
            .attr("style", "align-items: center;")
            .child(hobby_card)
            .child(pro_card)
            .child(ent_card)
        )
}

pub fn login_page() -> DomBuilder {
    let login_form = card().attr("style", "padding: 3rem; max-width: 400px; margin: 5rem auto; text-align: center;")
        .child(h2().text("Autenticazione 0-Trust").attr("class", "app-header"))
        .child(p().text("Accedi in totale sicurezza senza password."))
        .child(div().attr("style", "margin-top: 2rem;")
            .child(
                button()
                .attr("class", "ferrox-btn")
                .attr("style", "width: 100%; margin-bottom: 1rem; background: var(--accent-primary); font-size: 1.1rem;")
                .text("🔐 Accedi con Passkey (WebAuthn)")
                .on_click(|| {
                    // Trigger the Passkey prompt
                    web_sys::console::log_1(&"Triggering Passkey Auth...".into());
                    // We would call ferrox_front_security::prompt_passkey_login() here
                })
            )
            .child(button().attr("class", "ferrox-btn").attr("style", "width: 100%; background: var(--bg-surface-hover); border: 1px solid var(--border-glass);").text("Opzioni Fallback (Email)"))
        );

    container().attr("class", "fade-in-up").child(login_form)
}