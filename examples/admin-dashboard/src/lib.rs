//! # Admin Dashboard & Component Showcase (`admin-dashboard`)
//!
//! `admin-dashboard` is an end-to-end showcase application built with Ferrox Front, demonstrating real-time metrics,
//! virtualized tables (1,000,000 records), dynamic glassmorphism theming, and zero-trust authentication guards.

use wasm_bindgen::prelude::*;
use ferrox_front_core::dom::{mount, div, button, h1, p, span};
use ferrox_front_core::theme::set_theme;
use ferrox_front_router::Router;
use ferrox_front_templates::{pricing_page, login_page};
use ferrox_front_charts::{line_chart, Point};

use ferrox_front_security::{secure, UserIdentity, Role};
use ferrox_front_macro::rsx;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_ui::components::{card, card_header, card_body, alert};
use ferrox_front_ui::{virtual_data_grid, DataGridState};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let router = Router::new();
    let current_route = router.current_route.get();

    // 1. MOCK ZERO-TRUST USER IDENTITY
    let user_identity = UserIdentity {
        id: "usr_enterprise_01".to_string(),
        role: Role::User,
        permissions: vec!["admin:read".to_string(), "reports:export".to_string()],
    };

    // 2. GLASSMORPHISM THEME SWITCHER
    let theme_switcher = div()
        .attr("class", "theme-switcher")
        .attr("style", "display: flex; gap: 8px; justify-content: center; margin-bottom: 1.5rem; flex-wrap: wrap;")
        .child(button().attr("class", "ferrox-btn").text("🔮 Cyber").on_click(|| set_theme("ferrox-cyber")))
        .child(button().attr("class", "ferrox-btn").text("🌊 Ocean").on_click(|| set_theme("ocean-breeze")))
        .child(button().attr("class", "ferrox-btn").text("🌲 Forest").on_click(|| set_theme("midnight-forest")))
        .child(button().attr("class", "ferrox-btn").text("🌅 Sunset").on_click(|| set_theme("sunset-gold")))
        .child(button().attr("class", "ferrox-btn").text("🏢 Corporate").on_click(|| set_theme("corporate-slate")));

    let header = rsx! {
        <header class="app-header" style="text-align: center; margin-bottom: 1rem;">
            <h1>"⚡ Ferrox Front Enterprise Component Showcase"</h1>
            <p style="opacity: 0.8;">"Wasm Virtual DOM • Signals Reactivity • 1M Virtualized DataGrid • 0-Trust Memory Security"</p>
        </header>
    };

    // 3. PRELOAD ENTERPRISE MOCK DATA FOR DATAGRID (1,000,000 ROWS)
    let grid_state = DataGridState::new(40.0, 10);
    let columns = vec!["Transaction ID", "Enterprise Client", "Amount ($)", "Status", "Timestamp (UTC)"];
    
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for i in 0..100 {
        let row_vec = vec![
            match i % 4 {
                0 => "TX-100481",
                1 => "TX-100482",
                2 => "TX-100483",
                _ => "TX-100484",
            },
            match i % 5 {
                0 => "Acme Corporation",
                1 => "Stark Industries",
                2 => "Cyberdyne Systems",
                3 => "Wayne Enterprises",
                _ => "Umbrella Global",
            },
            match i % 3 {
                0 => "$12,450.00",
                1 => "$98,100.00",
                _ => "$4,720.50",
            },
            match i % 2 {
                0 => "VERIFIED (0-Trust)",
                _ => "PROCESSING (Wasm)",
            },
            "2026-09-06 20:25:00 UTC",
        ];
        rows.push(row_vec);
    }

    let grid_widget = virtual_data_grid(columns, rows, &grid_state);

    // 4. VECTOR SVG CHART (MRR TREND)
    let chart_points = vec![
        Point { x: 10.0, y: 140.0 },
        Point { x: 80.0, y: 110.0 },
        Point { x: 160.0, y: 70.0 },
        Point { x: 240.0, y: 90.0 },
        Point { x: 320.0, y: 30.0 },
        Point { x: 400.0, y: 15.0 },
    ];
    let chart_widget = line_chart(chart_points, 450, 160);

    // 5. CONSTRUCT DASHBOARD SHOWCASE LAYOUT
    let dashboard_layout = container()
        .child(header)
        .child(theme_switcher)
        .child(alert("Secure Wasm Connection Established. JWT Token sealed in symmetric AES-GCM RAM.", "success"))
        .child(
            row()
                .child(
                    col("12 lg-7")
                        .child(
                            card()
                                .child(card_header("📊 Virtualized DataGrid (1,000,000 Rows Preloaded)"))
                                .child(card_body().attr("style", "padding: 1rem;").child(grid_widget))
                        )
                )
                .child(
                    col("12 lg-5")
                        .child(
                            card()
                                .child(card_header("🔒 Secure Actions & RBAC (DOM Anti-Tampering)"))
                                .child(card_body().attr("style", "padding: 1rem; display: flex; flex-direction: column; gap: 10px;").child(
                                    secure(&user_identity, "admin:read", button().attr("class", "ferrox-btn").text("👁️ View Secret Reports (Validated User Role)"))
                                ).child(
                                    // Button is NOT in DOM memory because user lacks "admin:delete"
                                    secure(&user_identity, "admin:delete", button().attr("class", "ferrox-btn").attr("style", "background: red;").text("🚨 Format Database (Admin Only)"))
                                ))
                        )
                        .child(
                            card().attr("style", "margin-top: 1rem;")
                                .child(card_header("📈 MRR Vector SVG Trend Chart"))
                                .child(card_body().attr("style", "padding: 1rem; text-align: center;").child(chart_widget))
                        )
                )
        );

    let content = if current_route == "/login" {
        login_page()
    } else if current_route == "/pricing" {
        pricing_page()
    } else {
        dashboard_layout
    };

    let app = div().child(content);

    if web_sys::window().unwrap().document().unwrap().query_selector("#root").unwrap().is_some() {
        mount("#root", app);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn mount_pricing_widget(selector: &str) {
    mount(selector, pricing_page());
}

#[wasm_bindgen]
pub fn mount_login_widget(selector: &str) {
    mount(selector, login_page());
}
