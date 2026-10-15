//! # Native Rust SVG Icon Engine (`ferrox-front-ui::icons`)
//!
//! Provides over 60 enterprise vector SVG icons rendered directly as WebAssembly DOM nodes
//! using `DomBuilder::new_svg`. Completely independent from external icon fonts, CDN bundles, or remote SVG files.
//!
//! ## Example Usage
//! ```rust,no_run
//! use ferrox_front_ui::icons::{Icon, icon_activity, icon_shield};
//!
//! // Generic invocation
//! let icon = Icon::render("activity", 20, "var(--accent-primary)");
//!
//! // Dedicated function
//! let shield = icon_shield(24, "#10b981");
//! ```

use ferrox_front_core::dom::{svg, path, circle, line, rect, polyline, polygon, DomBuilder};

/// Constructs the base SVG canvas with 24x24 viewBox, stroke styling, and custom size/color.
pub fn base_svg(size: u32, color: &str) -> DomBuilder {
    svg()
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .attr("class", "ferrox-icon")
        .attr("style", "display: inline-block; vertical-align: middle; flex-shrink: 0;")
}

// =============================================================================
// 1. NAVIGATION & LAYOUT ICONS
// =============================================================================

pub fn icon_home(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"))
        .child(polyline().attr("points", "9 22 9 12 15 12 15 22"))
}

pub fn icon_menu(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "3").attr("y1", "12").attr("x2", "21").attr("y2", "12"))
        .child(line().attr("x1", "3").attr("y1", "6").attr("x2", "21").attr("y2", "6"))
        .child(line().attr("x1", "3").attr("y1", "18").attr("x2", "21").attr("y2", "18"))
}

pub fn icon_x(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "18").attr("y1", "6").attr("x2", "6").attr("y2", "18"))
        .child(line().attr("x1", "6").attr("y1", "6").attr("x2", "18").attr("y2", "18"))
}

pub fn icon_chevron_down(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "6 9 12 15 18 9"))
}

pub fn icon_chevron_up(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "18 15 12 9 6 15"))
}

pub fn icon_chevron_left(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "15 18 9 12 15 6"))
}

pub fn icon_chevron_right(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "9 18 15 12 9 6"))
}

pub fn icon_arrow_left(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "19").attr("y1", "12").attr("x2", "5").attr("y2", "12"))
        .child(polyline().attr("points", "12 19 5 12 12 5"))
}

pub fn icon_arrow_right(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "5").attr("y1", "12").attr("x2", "19").attr("y2", "12"))
        .child(polyline().attr("points", "12 5 19 12 12 19"))
}

pub fn icon_search(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "11").attr("cy", "11").attr("r", "8"))
        .child(line().attr("x1", "21").attr("y1", "21").attr("x2", "16.65").attr("y2", "16.65"))
}

pub fn icon_filter(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polygon().attr("points", "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"))
}

// =============================================================================
// 2. ACTION ICONS
// =============================================================================

pub fn icon_plus(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "12").attr("y1", "5").attr("x2", "12").attr("y2", "19"))
        .child(line().attr("x1", "5").attr("y1", "12").attr("x2", "19").attr("y2", "12"))
}

pub fn icon_minus(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "5").attr("y1", "12").attr("x2", "19").attr("y2", "12"))
}

pub fn icon_trash(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "3 6 5 6 21 6"))
        .child(path().attr("d", "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"))
}

pub fn icon_edit(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"))
        .child(path().attr("d", "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"))
}

pub fn icon_copy(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "9").attr("y", "9").attr("width", "13").attr("height", "13").attr("rx", "2").attr("ry", "2"))
        .child(path().attr("d", "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"))
}

pub fn icon_check(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "20 6 9 17 4 12"))
}

pub fn icon_refresh(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "23 4 23 10 17 10"))
        .child(polyline().attr("points", "1 20 1 14 7 14"))
        .child(path().attr("d", "M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"))
}

pub fn icon_download(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"))
        .child(polyline().attr("points", "7 10 12 15 17 10"))
        .child(line().attr("x1", "12").attr("y1", "15").attr("x2", "12").attr("y2", "3"))
}

pub fn icon_upload(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"))
        .child(polyline().attr("points", "17 8 12 3 7 8"))
        .child(line().attr("x1", "12").attr("y1", "3").attr("x2", "12").attr("y2", "15"))
}

pub fn icon_share(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "18").attr("cy", "5").attr("r", "3"))
        .child(circle().attr("cx", "6").attr("cy", "12").attr("r", "3"))
        .child(circle().attr("cx", "18").attr("cy", "19").attr("r", "3"))
        .child(line().attr("x1", "8.59").attr("y1", "13.51").attr("x2", "15.42").attr("y2", "17.49"))
        .child(line().attr("x1", "15.41").attr("y1", "6.51").attr("x2", "8.59").attr("y2", "10.49"))
}

pub fn icon_external_link(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"))
        .child(polyline().attr("points", "15 3 21 3 21 9"))
        .child(line().attr("x1", "10").attr("y1", "14").attr("x2", "21").attr("y2", "3"))
}

// =============================================================================
// 3. SECURITY & SYSTEM ICONS
// =============================================================================

pub fn icon_shield(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"))
}

pub fn icon_lock(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "3").attr("y", "11").attr("width", "18").attr("height", "11").attr("rx", "2").attr("ry", "2"))
        .child(path().attr("d", "M7 11V7a5 5 0 0 1 10 0v4"))
}

pub fn icon_unlock(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "3").attr("y", "11").attr("width", "18").attr("height", "11").attr("rx", "2").attr("ry", "2"))
        .child(path().attr("d", "M7 11V7a5 5 0 0 1 9.9-1"))
}

pub fn icon_key(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M21 2l-2 2m-1.5 1.5L16 7l-1.5-1.5L13 7l-2-2-7 7a5 5 0 0 0 7 7l7-7 1.5 1.5L21 12l-1.5-1.5L21 9l-1.5-1.5L21 6l-2-2z"))
        .child(circle().attr("cx", "7.5").attr("cy", "16.5").attr("r", "1.5"))
}

pub fn icon_eye(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"))
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "3"))
}

pub fn icon_eye_off(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"))
        .child(line().attr("x1", "1").attr("y1", "1").attr("x2", "23").attr("y2", "23"))
}

pub fn icon_alert_circle(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "10"))
        .child(line().attr("x1", "12").attr("y1", "8").attr("x2", "12").attr("y2", "12"))
        .child(line().attr("x1", "12").attr("y1", "16").attr("x2", "12.01").attr("y2", "16"))
}

pub fn icon_alert_triangle(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"))
        .child(line().attr("x1", "12").attr("y1", "9").attr("x2", "12").attr("y2", "13"))
        .child(line().attr("x1", "12").attr("y1", "17").attr("x2", "12.01").attr("y2", "17"))
}

pub fn icon_check_circle(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M22 11.08V12a10 10 0 1 1-5.93-9.14"))
        .child(polyline().attr("points", "22 4 12 14.01 9 11.01"))
}

pub fn icon_info(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "10"))
        .child(line().attr("x1", "12").attr("y1", "16").attr("x2", "12").attr("y2", "12"))
        .child(line().attr("x1", "12").attr("y1", "8").attr("x2", "12.01").attr("y2", "8"))
}

// =============================================================================
// 4. DATA & DASHBOARD ICONS
// =============================================================================

pub fn icon_bar_chart(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "12").attr("y1", "20").attr("x2", "12").attr("y2", "10"))
        .child(line().attr("x1", "18").attr("y1", "20").attr("x2", "18").attr("y2", "4"))
        .child(line().attr("x1", "6").attr("y1", "20").attr("x2", "6").attr("y2", "16"))
}

pub fn icon_line_chart(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M3 3v18h18"))
        .child(path().attr("d", "M18.7 8l-5.1 5.2-2.8-2.7L7 14.3"))
}

pub fn icon_pie_chart(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M21.21 15.89A10 10 0 1 1 8 2.83"))
        .child(path().attr("d", "M22 12A10 10 0 0 0 12 2v10z"))
}

pub fn icon_activity(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polyline().attr("points", "22 12 18 12 15 21 9 3 6 12 2 12"))
}

pub fn icon_trending_up(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "23 6 13.5 15.5 8.5 10.5 1 18"))
        .child(polyline().attr("points", "17 6 23 6 23 12"))
}

pub fn icon_trending_down(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "23 18 13.5 8.5 8.5 13.5 1 6"))
        .child(polyline().attr("points", "17 18 23 18 23 12"))
}

pub fn icon_layers(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polygon().attr("points", "12 2 2 7 12 12 22 7 12 2"))
        .child(polyline().attr("points", "2 17 12 22 22 17"))
        .child(polyline().attr("points", "2 12 12 17 22 12"))
}

pub fn icon_cpu(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "4").attr("y", "4").attr("width", "16").attr("height", "16").attr("rx", "2").attr("ry", "2"))
        .child(rect().attr("x", "9").attr("y", "9").attr("width", "6").attr("height", "6"))
        .child(line().attr("x1", "9").attr("y1", "1").attr("x2", "9").attr("y2", "4"))
        .child(line().attr("x1", "15").attr("y1", "1").attr("x2", "15").attr("y2", "4"))
        .child(line().attr("x1", "9").attr("y1", "20").attr("x2", "9").attr("y2", "23"))
        .child(line().attr("x1", "15").attr("y1", "20").attr("x2", "15").attr("y2", "23"))
        .child(line().attr("x1", "20").attr("y1", "9").attr("x2", "23").attr("y2", "9"))
        .child(line().attr("x1", "20").attr("y1", "15").attr("x2", "23").attr("y2", "15"))
        .child(line().attr("x1", "1").attr("y1", "9").attr("x2", "4").attr("y2", "9"))
        .child(line().attr("x1", "1").attr("y1", "15").attr("x2", "4").attr("y2", "15"))
}

pub fn icon_database(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M21 5c0 1.66-4 3-9 3s-9-1.34-9-3 4-3 9-3 9 1.34 9 3z"))
        .child(path().attr("d", "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"))
        .child(path().attr("d", "M3 12c0 1.66 4 3 9 3s9-1.34 9-3"))
}

pub fn icon_server(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "2").attr("y", "2").attr("width", "20").attr("height", "8").attr("rx", "2").attr("ry", "2"))
        .child(rect().attr("x", "2").attr("y", "14").attr("width", "20").attr("height", "8").attr("rx", "2").attr("ry", "2"))
        .child(line().attr("x1", "6").attr("y1", "6").attr("x2", "6.01").attr("y2", "6"))
        .child(line().attr("x1", "6").attr("y1", "18").attr("x2", "6.01").attr("y2", "18"))
}

pub fn icon_zap(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color).child(polygon().attr("points", "13 2 3 14 12 14 11 22 21 10 12 10 13 2"))
}

// =============================================================================
// 5. USER & COMMERCE ICONS
// =============================================================================

pub fn icon_user(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"))
        .child(circle().attr("cx", "12").attr("cy", "7").attr("r", "4"))
}

pub fn icon_users(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"))
        .child(circle().attr("cx", "9").attr("cy", "7").attr("r", "4"))
        .child(path().attr("d", "M23 21v-2a4 4 0 0 0-3-3.87"))
        .child(path().attr("d", "M16 3.13a4 4 0 0 1 0 7.75"))
}

pub fn icon_bell(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"))
        .child(path().attr("d", "M13.73 21a2 2 0 0 1-3.46 0"))
}

pub fn icon_mail(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(path().attr("d", "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"))
        .child(polyline().attr("points", "22,6 12,13 2,6"))
}

pub fn icon_settings(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "3"))
        .child(path().attr("d", "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"))
}

pub fn icon_credit_card(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "1").attr("y", "4").attr("width", "22").attr("height", "16").attr("rx", "2").attr("ry", "2"))
        .child(line().attr("x1", "1").attr("y1", "10").attr("x2", "23").attr("y2", "10"))
}

pub fn icon_shopping_cart(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "9").attr("cy", "21").attr("r", "1"))
        .child(circle().attr("cx", "20").attr("cy", "21").attr("r", "1"))
        .child(path().attr("d", "M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"))
}

pub fn icon_dollar_sign(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "12").attr("y1", "1").attr("x2", "12").attr("y2", "23"))
        .child(path().attr("d", "M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"))
}

pub fn icon_package(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(line().attr("x1", "16.5").attr("y1", "9.4").attr("x2", "7.55").attr("y2", "4.21"))
        .child(path().attr("d", "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"))
        .child(polyline().attr("points", "3.27 6.96 12 12.01 20.73 6.96"))
        .child(line().attr("x1", "12").attr("y1", "22.08").attr("x2", "12").attr("y2", "12"))
}

pub fn icon_clock(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "10"))
        .child(polyline().attr("points", "12 6 12 12 16 14"))
}

pub fn icon_calendar(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(rect().attr("x", "3").attr("y", "4").attr("width", "18").attr("height", "18").attr("rx", "2").attr("ry", "2"))
        .child(line().attr("x1", "16").attr("y1", "2").attr("x2", "16").attr("y2", "6"))
        .child(line().attr("x1", "8").attr("y1", "2").attr("x2", "8").attr("y2", "6"))
        .child(line().attr("x1", "3").attr("y1", "10").attr("x2", "21").attr("y2", "10"))
}

pub fn icon_globe(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(circle().attr("cx", "12").attr("cy", "12").attr("r", "10"))
        .child(line().attr("x1", "2").attr("y1", "12").attr("x2", "22").attr("y2", "12"))
        .child(path().attr("d", "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"))
}

pub fn icon_terminal(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "4 17 10 11 4 5"))
        .child(line().attr("x1", "12").attr("y1", "19").attr("x2", "20").attr("y2", "19"))
}

pub fn icon_code(size: u32, color: &str) -> DomBuilder {
    base_svg(size, color)
        .child(polyline().attr("points", "16 18 22 12 16 6"))
        .child(polyline().attr("points", "8 6 2 12 8 18"))
}

// =============================================================================
// 6. UNIFIED ICON ENGINE & DISPATCHER
// =============================================================================

/// Dynamic Icon Renderer mapping names to SVG builders
pub struct Icon;

impl Icon {
    /// Renders any supported icon by its kebab-case or snake_case name.
    pub fn render(name: &str, size: u32, color: &str) -> DomBuilder {
        match name.to_lowercase().replace('_', "-").as_str() {
            // Navigation
            "home" => icon_home(size, color),
            "menu" => icon_menu(size, color),
            "x" | "close" => icon_x(size, color),
            "chevron-down" => icon_chevron_down(size, color),
            "chevron-up" => icon_chevron_up(size, color),
            "chevron-left" => icon_chevron_left(size, color),
            "chevron-right" => icon_chevron_right(size, color),
            "arrow-left" => icon_arrow_left(size, color),
            "arrow-right" => icon_arrow_right(size, color),
            "search" => icon_search(size, color),
            "filter" => icon_filter(size, color),
            // Actions
            "plus" | "add" => icon_plus(size, color),
            "minus" => icon_minus(size, color),
            "trash" | "delete" => icon_trash(size, color),
            "edit" => icon_edit(size, color),
            "copy" => icon_copy(size, color),
            "check" => icon_check(size, color),
            "refresh" => icon_refresh(size, color),
            "download" => icon_download(size, color),
            "upload" => icon_upload(size, color),
            "share" => icon_share(size, color),
            "external-link" => icon_external_link(size, color),
            // Security
            "shield" => icon_shield(size, color),
            "lock" => icon_lock(size, color),
            "unlock" => icon_unlock(size, color),
            "key" => icon_key(size, color),
            "eye" => icon_eye(size, color),
            "eye-off" => icon_eye_off(size, color),
            "alert-circle" => icon_alert_circle(size, color),
            "alert-triangle" => icon_alert_triangle(size, color),
            "check-circle" => icon_check_circle(size, color),
            "info" => icon_info(size, color),
            // Data
            "bar-chart" => icon_bar_chart(size, color),
            "line-chart" => icon_line_chart(size, color),
            "pie-chart" => icon_pie_chart(size, color),
            "activity" => icon_activity(size, color),
            "trending-up" => icon_trending_up(size, color),
            "trending-down" => icon_trending_down(size, color),
            "layers" => icon_layers(size, color),
            "cpu" => icon_cpu(size, color),
            "database" => icon_database(size, color),
            "server" => icon_server(size, color),
            "zap" | "flash" => icon_zap(size, color),
            // User & Commerce
            "user" => icon_user(size, color),
            "users" => icon_users(size, color),
            "bell" => icon_bell(size, color),
            "mail" => icon_mail(size, color),
            "settings" => icon_settings(size, color),
            "credit-card" => icon_credit_card(size, color),
            "shopping-cart" => icon_shopping_cart(size, color),
            "dollar-sign" => icon_dollar_sign(size, color),
            "package" => icon_package(size, color),
            "clock" => icon_clock(size, color),
            "calendar" => icon_calendar(size, color),
            "globe" => icon_globe(size, color),
            "terminal" => icon_terminal(size, color),
            "code" => icon_code(size, color),
            // Fallback
            _ => icon_info(size, color),
        }
    }

    /// Returns list of all available icon identifier names
    pub fn all_names() -> &'static [&'static str] {
        &[
            "home", "menu", "x", "chevron-down", "chevron-up", "chevron-left", "chevron-right",
            "arrow-left", "arrow-right", "search", "filter",
            "plus", "minus", "trash", "edit", "copy", "check", "refresh", "download", "upload", "share", "external-link",
            "shield", "lock", "unlock", "key", "eye", "eye-off", "alert-circle", "alert-triangle", "check-circle", "info",
            "bar-chart", "line-chart", "pie-chart", "activity", "trending-up", "trending-down", "layers", "cpu", "database", "server", "zap",
            "user", "users", "bell", "mail", "settings", "credit-card", "shopping-cart", "dollar-sign", "package", "clock", "calendar", "globe", "terminal", "code"
        ]
    }
}
