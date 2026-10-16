//! # Ferrox Front Theme Studio & Token Exporter
//!
//! Provides real-time design system token customization, theme presets switching,
//! interactive component live preview, and one-click export for CSS custom properties
//! and Rust `ThemeConfig` structs.

use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::window;
use ferrox_front_core::dom::{div, span, p, h2, DomBuilder};
use ferrox_front_core::theme::set_theme;
use ferrox_front_ui::components::*;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_charts::{gauge_chart, sparkline_chart, SparklineType};
use crate::{card_advanced, render_app};

#[derive(Clone, Debug)]
pub struct ThemePreset {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub bg_base: &'static str,
    pub bg_surface: &'static str,
    pub accent_primary: &'static str,
    pub accent_secondary: &'static str,
    pub text_primary: &'static str,
    pub border_radius: u32,
    pub glass_blur: u32,
}

pub const PRESETS: &[ThemePreset] = &[
    ThemePreset {
        id: "ferrox-cyber",
        label: "🔮 Cyber Dark",
        icon: "moon",
        bg_base: "#060913",
        bg_surface: "rgba(15, 23, 42, 0.72)",
        accent_primary: "#6366f1",
        accent_secondary: "#06b6d4",
        text_primary: "#f8fafc",
        border_radius: 16,
        glass_blur: 20,
    },
    ThemePreset {
        id: "enterprise-zinc",
        label: "🏢 Zinc Light",
        icon: "sun",
        bg_base: "#f8fafc",
        bg_surface: "rgba(255, 255, 255, 0.95)",
        accent_primary: "#0f172a",
        accent_secondary: "#2563eb",
        text_primary: "#0f172a",
        border_radius: 12,
        glass_blur: 10,
    },
    ThemePreset {
        id: "cyberpunk-neon",
        label: "⚡ Cyberpunk Neon",
        icon: "zap",
        bg_base: "#05050d",
        bg_surface: "rgba(18, 11, 38, 0.82)",
        accent_primary: "#ec4899",
        accent_secondary: "#00f0ff",
        text_primary: "#ffffff",
        border_radius: 14,
        glass_blur: 24,
    },
    ThemePreset {
        id: "ocean-breeze",
        label: "🌊 Ocean Breeze",
        icon: "activity",
        bg_base: "#071326",
        bg_surface: "rgba(11, 28, 54, 0.78)",
        accent_primary: "#00b4d8",
        accent_secondary: "#90e0ef",
        text_primary: "#f0f8ff",
        border_radius: 18,
        glass_blur: 20,
    },
    ThemePreset {
        id: "sunset-gold",
        label: "🌅 Sunset Gold",
        icon: "trend-up",
        bg_base: "#180905",
        bg_surface: "rgba(38, 16, 10, 0.82)",
        accent_primary: "#f59e0b",
        accent_secondary: "#ef4444",
        text_primary: "#fffbeb",
        border_radius: 16,
        glass_blur: 22,
    },
    ThemePreset {
        id: "forest-emerald",
        label: "🌲 Emerald Forest",
        icon: "shield",
        bg_base: "#04140d",
        bg_surface: "rgba(6, 36, 23, 0.82)",
        accent_primary: "#10b981",
        accent_secondary: "#34d399",
        text_primary: "#ecfdf5",
        border_radius: 16,
        glass_blur: 20,
    },
];

#[derive(Clone, Debug)]
pub struct ThemeStudioState {
    pub preset_name: String,
    pub bg_base: String,
    pub bg_surface: String,
    pub accent_primary: String,
    pub accent_secondary: String,
    pub text_primary: String,
    pub border_radius: u32,
    pub glass_blur: u32,
}

impl Default for ThemeStudioState {
    fn default() -> Self {
        Self {
            preset_name: "ferrox-cyber".into(),
            bg_base: "#060913".into(),
            bg_surface: "rgba(15, 23, 42, 0.72)".into(),
            accent_primary: "#6366f1".into(),
            accent_secondary: "#06b6d4".into(),
            text_primary: "#f8fafc".into(),
            border_radius: 16,
            glass_blur: 20,
        }
    }
}

thread_local! {
    pub static THEME_STUDIO_STATE: RefCell<ThemeStudioState> = RefCell::new(ThemeStudioState::default());
}

pub fn apply_active_theme_tokens() {
    THEME_STUDIO_STATE.with(|s| {
        let state = s.borrow();
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(root) = doc.document_element() {
                    if let Ok(elem) = root.dyn_into::<web_sys::HtmlElement>() {
                        let style = elem.style();
                        let _ = style.set_property("--bg-base", &state.bg_base);
                        let _ = style.set_property("--bg-surface", &state.bg_surface);
                        let _ = style.set_property("--accent-primary", &state.accent_primary);
                        let _ = style.set_property("--accent-secondary", &state.accent_secondary);
                        let _ = style.set_property("--text-primary", &state.text_primary);
                        let _ = style.set_property("--border-radius", &format!("{}px", state.border_radius));
                        let _ = style.set_property("--backdrop-blur", &format!("{}px", state.glass_blur));
                    }
                }
            }
        }
    });
}

pub fn copy_to_clipboard(text: &str, alert_msg: &str) {
    if let Some(win) = window() {
        let nav = win.navigator();
        let clip = nav.clipboard();
        let _ = clip.write_text(text);
        let _ = win.alert_with_message(alert_msg);
    }
}

pub fn generate_css_tokens(state: &ThemeStudioState) -> String {
    format!(
r#"/* Ferrox Front Generated Design Tokens */
:root, [data-theme="{}"] {{
  --bg-base: {};
  --bg-surface: {};
  --accent-primary: {};
  --accent-secondary: {};
  --text-primary: {};
  --border-radius: {}px;
  --backdrop-blur: {}px;
  --border-glass: 1px solid rgba(255, 255, 255, 0.12);
  --shadow-glass: 0 20px 40px -15px rgba(0, 0, 0, 0.55);
}}"#,
        state.preset_name,
        state.bg_base,
        state.bg_surface,
        state.accent_primary,
        state.accent_secondary,
        state.text_primary,
        state.border_radius,
        state.glass_blur
    )
}

pub fn generate_rust_theme(state: &ThemeStudioState) -> String {
    format!(
r#"use ferrox_front_ui::tokens::ThemeConfig;

/// Custom generated Ferrox Front Theme Configuration
pub fn create_custom_theme() -> ThemeConfig {{
    ThemeConfig {{
        name: "{}".into(),
        bg_base: "{}".into(),
        bg_surface: "{}".into(),
        accent_primary: "{}".into(),
        accent_secondary: "{}".into(),
        text_primary: "{}".into(),
        border_radius: "{}px".into(),
    }}
}}"#,
        state.preset_name,
        state.bg_base,
        state.bg_surface,
        state.accent_primary,
        state.accent_secondary,
        state.text_primary,
        state.border_radius
    )
}

pub fn render_theme_studio_tab() -> DomBuilder {
    let state = THEME_STUDIO_STATE.with(|s| s.borrow().clone());

    // 1. Preset Selector Buttons
    let mut preset_buttons = Vec::new();
    for p_item in PRESETS {
        let is_active = state.preset_name == p_item.id;
        let id_str = p_item.id;
        let p_clone = p_item.clone();
        preset_buttons.push(
            button_advanced(p_item.label, if is_active { "primary" } else { "outline" }, "sm", false, Some(p_item.icon))
                .attr("style", if is_active {
                    "font-weight: 700; box-shadow: 0 4px 14px rgba(99, 102, 241, 0.4);"
                } else {
                    "opacity: 0.85;"
                })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        let mut cur = s.borrow_mut();
                        cur.preset_name = id_str.to_string();
                        cur.bg_base = p_clone.bg_base.to_string();
                        cur.bg_surface = p_clone.bg_surface.to_string();
                        cur.accent_primary = p_clone.accent_primary.to_string();
                        cur.accent_secondary = p_clone.accent_secondary.to_string();
                        cur.text_primary = p_clone.text_primary.to_string();
                        cur.border_radius = p_clone.border_radius;
                        cur.glass_blur = p_clone.glass_blur;
                    });
                    set_theme(id_str);
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    // 2. Custom Token Controls Card
    let primary_colors = vec![
        ("#6366f1", "Indigo", "var(--accent-glow)"),
        ("#10b981", "Emerald", "rgba(16, 185, 129, 0.25)"),
        ("#f43f5e", "Rose", "rgba(244, 63, 94, 0.25)"),
        ("#f59e0b", "Amber", "rgba(245, 158, 11, 0.25)"),
        ("#06b6d4", "Cyan", "rgba(6, 182, 212, 0.25)"),
        ("#8b5cf6", "Violet", "rgba(139, 92, 246, 0.25)"),
        ("#0f172a", "Slate", "rgba(15, 23, 42, 0.25)"),
    ];

    let mut primary_color_pills = Vec::new();
    for (hex, name, _glow) in primary_colors {
        let is_sel = state.accent_primary == hex;
        let hex_str = hex.to_string();
        primary_color_pills.push(
            button_advanced(name, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", &format!(
                    "border-left: 4px solid {}; {}",
                    hex,
                    if is_sel { "font-weight: 700; transform: scale(1.05);" } else { "opacity: 0.8;" }
                ))
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().accent_primary = hex_str.clone();
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let secondary_colors = vec![
        ("#06b6d4", "Cyan"),
        ("#10b981", "Mint"),
        ("#c084fc", "Lavender"),
        ("#fb923c", "Orange"),
        ("#3b82f6", "Blue"),
    ];

    let mut secondary_color_pills = Vec::new();
    for (hex, name) in secondary_colors {
        let is_sel = state.accent_secondary == hex;
        let hex_str = hex.to_string();
        secondary_color_pills.push(
            button_advanced(name, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", &format!(
                    "border-left: 4px solid {}; {}",
                    hex,
                    if is_sel { "font-weight: 700; transform: scale(1.05);" } else { "opacity: 0.8;" }
                ))
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().accent_secondary = hex_str.clone();
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let radiuses = vec![
        (0, "0px (Sharp)"),
        (8, "8px (Compact)"),
        (14, "14px (Smooth)"),
        (20, "20px (Curved)"),
        (28, "28px (Pill)"),
    ];

    let mut radius_buttons = Vec::new();
    for (r_val, label) in radiuses {
        let is_sel = state.border_radius == r_val;
        radius_buttons.push(
            button_advanced(label, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().border_radius = r_val;
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let blurs = vec![
        (0, "0px (Flat)"),
        (12, "12px (Soft)"),
        (20, "20px (Standard)"),
        (32, "32px (Deep Glass)"),
    ];

    let mut blur_buttons = Vec::new();
    for (b_val, label) in blurs {
        let is_sel = state.glass_blur == b_val;
        blur_buttons.push(
            button_advanced(label, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().glass_blur = b_val;
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let token_controls_card = card_advanced(
        "Design Token Modifiers",
        "Fine-tune theme variables in real-time across the entire WebAssembly engine",
        "sliders",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-6")
            .children(vec![
                div().children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Primary Accent Color (--accent-primary):"),
                    div().class("flex flex-wrap items-center gap-2").children(primary_color_pills),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Secondary Accent Color (--accent-secondary):"),
                    div().class("flex flex-wrap items-center gap-2").children(secondary_color_pills),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Corner Radius Scale (--border-radius):"),
                    div().class("flex flex-wrap items-center gap-2").children(radius_buttons),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Backdrop Glass Blur (--backdrop-blur):"),
                    div().class("flex flex-wrap items-center gap-2").children(blur_buttons),
                ]),
            ])
    );

    // 3. Live Component Preview Card
    let input_demo = input_advanced(InputConfig {
        input_type: "text".into(),
        placeholder: "Live reactive token input...".into(),
        value: "Ferrox Wasm Theme Active".into(),
        prefix_icon: Some("zap".into()),
        suffix_icon: Some("check".into()),
        is_valid: Some(true),
        helper_text: Some("Adapts dynamically to --accent-primary and radius".into()),
        clearable: false,
    });

    let live_gauge = gauge_chart("theme-studio-gauge", 76.5, 0.0, 100.0, "Efficiency", "%", 220, 125);
    let sparkline_demo = sparkline_chart(vec![12.0, 19.0, 15.0, 28.0, 34.0, 48.0, 52.0], 120, 34, SparklineType::Area, &state.accent_primary);

    let preview_card = card_advanced(
        "Live Component Showcase",
        "Living UI controls reflecting modified tokens instantly with 0 JavaScript",
        "layers",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-6")
            .children(vec![
                div().children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Interactive Buttons:"),
                    div().class("flex flex-wrap items-center gap-3")
                        .children(vec![
                            button_advanced("Primary Action", "primary", "sm", false, Some("zap")),
                            button_advanced("Glass Surface", "glass", "sm", false, Some("layers")),
                            button_advanced("Accent Outline", "outline", "sm", false, Some("activity")),
                            button_advanced("Destructive", "danger", "sm", false, Some("trash")),
                        ]),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-3").text("Badges & Live Status Indicators:"),
                    div().class("flex flex-wrap items-center gap-2.5")
                        .children(vec![
                            badge("ACTIVE TOKEN", "primary"),
                            badge("WCAG 2.1 AA", "success"),
                            badge("WASM ACCELERATED", "info"),
                            badge("ZERO RUNTIME JS", "warning"),
                        ]),
                ]),
                div().class("pt-4 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2").text("Form Input & Slider:"),
                    input_demo,
                    div().class("pt-3").child(range_slider_with_bubble("theme-slippage-demo", 0, 100, 42, "%")),
                ]),
                div().class("pt-4 border-t border-slate-800/80 flex flex-wrap items-center justify-between gap-4").children(vec![
                    div().class("flex flex-col items-center").child(live_gauge),
                    div().class("flex flex-col items-center gap-2 p-3 rounded-xl bg-slate-900/60 border border-slate-800/80")
                        .children(vec![
                            span().class("text-xs text-slate-400 font-medium").text("Live Vector Sparkline:"),
                            sparkline_demo,
                        ]),
                ]),
            ])
    );

    // 4. Exporter Panel (CSS + Rust)
    let css_code = generate_css_tokens(&state);
    let rust_code = generate_rust_theme(&state);
    let css_for_copy = css_code.clone();
    let rust_for_copy = rust_code.clone();

    let export_card = card_advanced(
        "Export Custom Theme Definitions",
        "Ready-to-use CSS tokens and Rust ThemeConfig structs for instant application integration",
        "download",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-5")
            .children(vec![
                div().class("flex flex-wrap items-center justify-between gap-4 pb-2")
                    .children(vec![
                        span().class("text-xs text-slate-300 font-medium")
                            .text("Export formats ready to drop into your frontend or styles.css:"),
                        div().class("flex items-center gap-3")
                            .children(vec![
                                button_advanced("📋 Copy CSS Tokens", "primary", "xs", false, Some("download"))
                                    .on_click(move || {
                                        copy_to_clipboard(&css_for_copy, "✅ CSS Theme tokens copied to clipboard!");
                                    }),
                                button_advanced("🦀 Copy Rust ThemeConfig", "outline", "xs", false, Some("code"))
                                    .on_click(move || {
                                        copy_to_clipboard(&rust_for_copy, "✅ Rust ThemeConfig struct copied to clipboard!");
                                    }),
                            ]),
                    ]),
                div().class("grid grid-cols-1 md:grid-cols-2 gap-4")
                    .children(vec![
                        div().class("flex flex-col gap-1.5")
                            .children(vec![
                                span().class("text-[11px] font-mono text-indigo-400 font-bold").text("CSS Custom Properties (:root):"),
                                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-emerald-300 overflow-x-auto max-h-60 leading-relaxed whitespace-pre")
                                    .text(&css_code),
                            ]),
                        div().class("flex flex-col gap-1.5")
                            .children(vec![
                                span().class("text-[11px] font-mono text-indigo-400 font-bold").text("Rust Strong-Typed ThemeConfig:"),
                                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-sky-300 overflow-x-auto max-h-60 leading-relaxed whitespace-pre")
                                    .text(&rust_code),
                            ]),
                    ]),
            ])
    );

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 32px;")
        .children(vec![
            div()
                .class("pb-4 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-4")
                .children(vec![
                    div().children(vec![
                        h2().class("text-2xl font-black text-white tracking-tight").text("🎨 Theme Studio & Token Customizer"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("Zero-Runtime WebAssembly design system engine: tweak color palettes, radiuses, and glassmorphism blurs on the fly, and export production-ready code."),
                    ]),
                    badge("Real-Time Token Reactive", "primary"),
                ]),
            div().class("flex flex-wrap items-center gap-3 p-3.5 rounded-2xl bg-slate-900/60 border border-slate-800/80")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-2").text("Presets:"),
                    div().class("flex flex-wrap items-center gap-2").children(preset_buttons),
                ]),
            row().child(col("12 md-6").child(token_controls_card)).child(col("12 md-6").child(preview_card)),
            row().child(col("12").child(export_card)),
        ])
}
