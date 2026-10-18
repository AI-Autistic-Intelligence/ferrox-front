//! # Ferrox Front Theme Studio & Design System Handbook
//!
//! Provides real-time design system token customization, theme presets switching,
//! interactive component playground with knobs, and one-click export for CSS custom properties,
//! Rust `ThemeConfig` structs, and synchronized `DomBuilder` component snippets.

use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::window;
use ferrox_front_core::dom::{div, span, p, h2, DomBuilder};
use ferrox_front_core::theme::set_theme;
use ferrox_front_ui::components::*;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_ui::overlays::{tree_view, TreeNode};
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
    pub glass_opacity: u32,
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
        glass_opacity: 75,
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
        glass_opacity: 95,
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
        glass_opacity: 80,
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
        glass_opacity: 78,
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
        glass_opacity: 82,
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
        glass_opacity: 82,
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
    pub glass_opacity: u32,
    // Component Knobs
    pub button_size: String,
    pub button_variant: String,
    pub switch_checked: bool,
    pub active_showcase_tab: String,
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
            glass_opacity: 75,
            button_size: "sm".into(),
            button_variant: "primary".into(),
            switch_checked: true,
            active_showcase_tab: "buttons".into(),
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
                        let _ = style.set_property("--glass-opacity", &format!("{:.2}", state.glass_opacity as f64 / 100.0));
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
  --glass-opacity: {:.2};
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
        state.glass_blur,
        state.glass_opacity as f64 / 100.0
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

pub fn generate_dom_builder_snippet(state: &ThemeStudioState) -> String {
    format!(
r#"use ferrox_front_core::dom::{{div, span, DomBuilder}};
use ferrox_front_ui::components::{{button_advanced, badge, card_advanced}};

// Live Rust Wasm Component adhering to active design tokens ({}px radius, {}px blur)
let container_card = card_advanced(
    "Live Analytics Pod",
    "Constructed dynamically in Wasm linear memory with zero external JS",
    "activity",
    "glass",
    false,
    "p-6",
)
.child(
    div().class("flex flex-wrap items-center gap-3")
        .children(vec![
            button_advanced("Execute Query", "{}", "{}", false, Some("zap")),
            badge("● Online", "success"),
            badge("Tokens Synced", "primary"),
        ])
);"#,
        state.border_radius,
        state.glass_blur,
        state.button_variant,
        state.button_size,
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
                        cur.glass_opacity = p_clone.glass_opacity;
                    });
                    set_theme(id_str);
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    // 2. Custom Token Controls Card
    let primary_colors = vec![
        ("#6366f1", "Indigo"),
        ("#10b981", "Emerald"),
        ("#f43f5e", "Rose"),
        ("#f59e0b", "Amber"),
        ("#06b6d4", "Cyan"),
        ("#8b5cf6", "Violet"),
        ("#0f172a", "Slate"),
    ];

    let mut primary_color_pills = Vec::new();
    for (hex, name) in primary_colors {
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
        ("#d946ef", "Fuchsia"),
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

    let surface_themes = vec![
        ("Obsidian", "#060913", "rgba(15, 23, 42, 0.72)"),
        ("Midnight", "#030712", "rgba(17, 24, 39, 0.75)"),
        ("Deep Violet", "#0b0716", "rgba(24, 15, 45, 0.78)"),
        ("Light Zinc", "#f8fafc", "rgba(255, 255, 255, 0.95)"),
    ];

    let mut surface_buttons = Vec::new();
    for (name, base, surf) in surface_themes {
        let is_sel = state.bg_base == base;
        let base_s = base.to_string();
        let surf_s = surf.to_string();
        surface_buttons.push(
            button_advanced(name, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.bg_base = base_s.clone();
                        st.bg_surface = surf_s.clone();
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let radiuses = vec![
        (0, "0px (Sharp)"),
        (6, "6px (Subtle)"),
        (12, "12px (Compact)"),
        (16, "16px (Standard)"),
        (24, "24px (Curved)"),
        (32, "32px (Pill)"),
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
        (8, "8px (Light)"),
        (16, "16px (Soft)"),
        (24, "24px (Medium)"),
        (32, "32px (Deep Glass)"),
        (40, "40px (Frost)"),
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

    let opacities = vec![
        (15, "15% (Ultra Clear)"),
        (40, "40% (Light)"),
        (60, "60% (Balanced)"),
        (75, "75% (Glass Medium)"),
        (90, "90% (Dense)"),
    ];

    let mut opacity_buttons = Vec::new();
    for (o_val, label) in opacities {
        let is_sel = state.glass_opacity == o_val;
        opacity_buttons.push(
            button_advanced(label, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().glass_opacity = o_val;
                    });
                    apply_active_theme_tokens();
                    render_app("theme-studio");
                })
        );
    }

    let token_controls_card = card_advanced(
        "Design Token Live Inspector",
        "Fine-tune theme variables in real-time across the entire WebAssembly engine",
        "sliders",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-5")
            .children(vec![
                div().children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Primary Accent Color (--accent-primary):"),
                    div().class("flex flex-wrap items-center gap-2").children(primary_color_pills),
                ]),
                div().class("pt-3 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Secondary Accent Color (--accent-secondary):"),
                    div().class("flex flex-wrap items-center gap-2").children(secondary_color_pills),
                ]),
                div().class("pt-3 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Surface Background & Canvas Tone:"),
                    div().class("flex flex-wrap items-center gap-2").children(surface_buttons),
                ]),
                div().class("pt-3 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Corner Radius Scale (--border-radius):"),
                    div().class("flex flex-wrap items-center gap-2").children(radius_buttons),
                ]),
                div().class("pt-3 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Backdrop Glass Blur (--backdrop-blur):"),
                    div().class("flex flex-wrap items-center gap-2").children(blur_buttons),
                ]),
                div().class("pt-3 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2")
                        .text("Glass Opacity (--glass-opacity):"),
                    div().class("flex flex-wrap items-center gap-2").children(opacity_buttons),
                ]),
            ])
    );

    // 3. Component Showcase Playground with Knobs
    let showcase_tabs = vec![
        ("buttons", "🔘 Buttons & Badges"),
        ("forms", "📝 Forms & Switches"),
        ("cards", "🃏 Cards & Elevation"),
        ("grid", "📊 DataGrid Mini"),
        ("overlays", "📂 Overlays & Trees"),
    ];

    let mut tab_buttons = Vec::new();
    for (tab_id, tab_label) in showcase_tabs {
        let is_sel = state.active_showcase_tab == tab_id;
        let id_s = tab_id.to_string();
        tab_buttons.push(
            button_advanced(tab_label, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", if is_sel { "font-weight: 700; box-shadow: 0 2px 10px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.8;" })
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        s.borrow_mut().active_showcase_tab = id_s.clone();
                    });
                    render_app("theme-studio");
                })
        );
    }

    let showcase_content = match state.active_showcase_tab.as_str() {
        "forms" => {
            let input_demo = input_advanced(InputConfig {
                input_type: "text".into(),
                placeholder: "Live reactive token input...".into(),
                value: "Ferrox Wasm Theme Active".into(),
                prefix_icon: Some("zap".into()),
                suffix_icon: Some("check".into()),
                is_valid: Some(true),
                helper_text: Some("Adapts dynamically to active accent color and border radius".into()),
                clearable: false,
            });

            let sw_checked = state.switch_checked;
            let sw = switch_toggle("theme-switch-toggle", sw_checked, "Reactive Feature Flag")
                .on_click(move || {
                    THEME_STUDIO_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.switch_checked = !st.switch_checked;
                    });
                    render_app("theme-studio");
                });

            div().class("space-y-5")
                .children(vec![
                    div().children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2").text("Form Input Control:"),
                        input_demo,
                    ]),
                    div().class("pt-3 border-t border-slate-800/80").children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2").text("Switch Toggle Control:"),
                        sw,
                    ]),
                    div().class("pt-3 border-t border-slate-800/80").children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2").text("Range Slider with Live Bubble:"),
                        range_slider_with_bubble("theme-slippage-demo", 0, 100, 48, "%"),
                    ]),
                ])
        },
        "cards" => {
            div().class("grid grid-cols-1 sm:grid-cols-2 gap-4")
                .children(vec![
                    div().class("p-5 rounded-2xl bg-slate-900/60 border border-slate-800/80 backdrop-blur-md shadow-lg flex flex-col gap-2")
                        .children(vec![
                            span().class("text-xs text-indigo-400 font-bold uppercase tracking-wider").text("Standard Glass Card"),
                            p().class("text-xs text-slate-300").text("Uses synchronized border-radius and backdrop-blur tokens seamlessly."),
                            badge("Elevation: Medium", "primary"),
                        ]),
                    div().class("p-5 rounded-2xl bg-slate-900/85 border-2 border-indigo-500/50 shadow-2xl flex flex-col gap-2")
                        .children(vec![
                            span().class("text-xs text-emerald-400 font-bold uppercase tracking-wider").text("Luminescent Highlight Card"),
                            p().class("text-xs text-slate-300").text("High-contrast container for critical KPIs, metrics, and callouts."),
                            badge("Accent Glow Active", "success"),
                        ]),
                ])
        },
        "grid" => {
            div().class("space-y-3")
                .children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block").text("Enterprise Virtual DataGrid Preview:"),
                    div().class("w-full overflow-x-auto rounded-xl border border-slate-800 bg-slate-950/80")
                        .child(
                            div().class("min-w-[400px] text-xs font-mono")
                                .children(vec![
                                    div().class("grid grid-cols-4 p-2.5 bg-slate-900/90 font-bold text-slate-400 border-b border-slate-800")
                                        .children(vec![
                                            span().text("Asset"),
                                            span().text("Price"),
                                            span().text("24h Change"),
                                            span().text("Status"),
                                        ]),
                                    div().class("grid grid-cols-4 p-2.5 items-center border-b border-slate-800/50 hover:bg-slate-800/30")
                                        .children(vec![
                                            span().class("font-bold text-white").text("FERROX/USDC"),
                                            span().text("$142.80"),
                                            span().class("text-emerald-400 font-bold").text("+12.4%"),
                                            badge("HEALTHY", "success"),
                                        ]),
                                    div().class("grid grid-cols-4 p-2.5 items-center border-b border-slate-800/50 hover:bg-slate-800/30")
                                        .children(vec![
                                            span().class("font-bold text-white").text("WASM/SOL"),
                                            span().text("$18.45"),
                                            span().class("text-emerald-400 font-bold").text("+5.8%"),
                                            badge("SYNCED", "primary"),
                                        ]),
                                    div().class("grid grid-cols-4 p-2.5 items-center hover:bg-slate-800/30")
                                        .children(vec![
                                            span().class("font-bold text-white").text("RUST/ETH"),
                                            span().text("$3,420.00"),
                                            span().class("text-rose-400 font-bold").text("-1.2%"),
                                            badge("STABLE", "info"),
                                        ]),
                                ])
                        ),
                ])
        },
        "overlays" => {
            let tree_nodes = vec![
                TreeNode::new("node-core", "crates/ferrox-front-core")
                    .with_icon("folder")
                    .with_badge("Core Engine"),
                TreeNode::new("node-ui", "crates/ferrox-front-ui")
                    .with_icon("layers")
                    .with_badge("Design System"),
                TreeNode::new("node-charts", "crates/ferrox-front-charts")
                    .with_icon("trend-up")
                    .with_badge("SVG Vector"),
            ];

            let tree = tree_view(tree_nodes, |_| {});

            div().class("space-y-4")
                .children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block").text("Hierarchical TreeView & Overlays:"),
                    tree,
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-between")
                        .children(vec![
                            span().class("text-xs text-slate-300").text("Vector DatePicker & Modal Traps:"),
                            badge("WCAG 2.1 AA Compliant", "success"),
                        ]),
                ])
        },
        _ => {
            // Default "buttons"
            let sizes = ["xs", "sm", "md", "lg"];
            let mut size_pills = Vec::new();
            for sz in sizes {
                let is_sel = state.button_size == sz;
                let sz_s = sz.to_string();
                size_pills.push(
                    button_advanced(sz, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                        .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                        .on_click(move || {
                            THEME_STUDIO_STATE.with(|s| {
                                s.borrow_mut().button_size = sz_s.clone();
                            });
                            render_app("theme-studio");
                        })
                );
            }

            let variants = ["primary", "glass", "outline", "danger"];
            let mut variant_pills = Vec::new();
            for vr in variants {
                let is_sel = state.button_variant == vr;
                let vr_s = vr.to_string();
                variant_pills.push(
                    button_advanced(vr, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                        .attr("style", if is_sel { "font-weight: 700;" } else { "opacity: 0.8;" })
                        .on_click(move || {
                            THEME_STUDIO_STATE.with(|s| {
                                s.borrow_mut().button_variant = vr_s.clone();
                            });
                            render_app("theme-studio");
                        })
                );
            }

            let current_btn = button_advanced("Selected Button Knobs", &state.button_variant, &state.button_size, false, Some("zap"));

            div().class("space-y-4")
                .children(vec![
                    div().class("flex flex-wrap items-center justify-between gap-2 p-2.5 rounded-xl bg-slate-950/70 border border-slate-800/80")
                        .children(vec![
                            div().class("flex items-center gap-1.5")
                                .children(vec![
                                    span().class("text-xs font-bold text-slate-400 uppercase mr-1").text("Size:"),
                                    div().class("flex items-center gap-1").children(size_pills),
                                ]),
                            div().class("flex items-center gap-1.5")
                                .children(vec![
                                    span().class("text-xs font-bold text-slate-400 uppercase mr-1").text("Variant:"),
                                    div().class("flex items-center gap-1").children(variant_pills),
                                ]),
                        ]),
                    div().class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex items-center justify-center")
                        .child(current_btn),
                    div().class("pt-3 border-t border-slate-800/80").children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2.5").text("All Variants Gallery:"),
                        div().class("flex flex-wrap items-center gap-2.5")
                            .children(vec![
                                button_advanced("Primary", "primary", "sm", false, Some("zap")),
                                button_advanced("Glass", "glass", "sm", false, Some("layers")),
                                button_advanced("Outline", "outline", "sm", false, Some("activity")),
                                button_advanced("Danger", "danger", "sm", false, Some("trash")),
                            ]),
                    ]),
                    div().class("pt-3 border-t border-slate-800/80").children(vec![
                        span().class("text-xs font-bold text-slate-300 uppercase tracking-wider block mb-2.5").text("Badges & Indicators:"),
                        div().class("flex flex-wrap items-center gap-2")
                            .children(vec![
                                badge("PRIMARY", "primary"),
                                badge("SUCCESS", "success"),
                                badge("WARNING", "warning"),
                                badge("DANGER", "danger"),
                                badge("INFO", "info"),
                            ]),
                    ]),
                ])
        }
    };

    let preview_card = card_advanced(
        "Interactive Component Playground",
        "Living UI controls reflecting modified tokens instantly with zero runtime JavaScript",
        "layers",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-4")
            .children(vec![
                div().class("flex flex-wrap items-center gap-1.5 pb-3 border-b border-slate-800/80")
                    .children(tab_buttons),
                showcase_content,
                div().class("pt-4 border-t border-slate-800/80 flex flex-wrap items-center justify-between gap-4")
                    .children(vec![
                        div().class("flex flex-col items-center")
                            .child(gauge_chart("theme-studio-gauge", 76.5, 0.0, 100.0, "Efficiency", "%", 220, 125)),
                        div().class("flex flex-col items-center gap-2 p-3 rounded-xl bg-slate-900/60 border border-slate-800/80")
                            .children(vec![
                                span().class("text-xs text-slate-400 font-medium").text("Live Vector Sparkline:"),
                                sparkline_chart(vec![12.0, 19.0, 15.0, 28.0, 34.0, 48.0, 52.0], 120, 34, SparklineType::Area, &state.accent_primary),
                            ]),
                    ]),
            ])
    );

    // 4. Exporter Panel (CSS + Rust ThemeConfig + Rust DomBuilder Snippet)
    let css_code = generate_css_tokens(&state);
    let rust_code = generate_rust_theme(&state);
    let dom_code = generate_dom_builder_snippet(&state);

    let css_for_copy = css_code.clone();
    let rust_for_copy = rust_code.clone();
    let dom_for_copy = dom_code.clone();

    let export_card = card_advanced(
        "Playbook Snippet Exporter (CSS + Rust Wasm)",
        "Ready-to-use CSS tokens, native Rust ThemeConfig, and DomBuilder component snippets for instant application integration",
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
                            .text("Export formats ready to drop into your frontend codebase or styles.css:"),
                        div().class("flex flex-wrap items-center gap-2")
                            .children(vec![
                                button_advanced("📋 Copy CSS Tokens", "primary", "xs", false, Some("download"))
                                    .on_click(move || {
                                        copy_to_clipboard(&css_for_copy, "✅ CSS Theme tokens copied to clipboard!");
                                    }),
                                button_advanced("🦀 Copy ThemeConfig", "outline", "xs", false, Some("code"))
                                    .on_click(move || {
                                        copy_to_clipboard(&rust_for_copy, "✅ Rust ThemeConfig struct copied to clipboard!");
                                    }),
                                button_advanced("⚡ Copy DomBuilder Rust", "glass", "xs", false, Some("zap"))
                                    .on_click(move || {
                                        copy_to_clipboard(&dom_for_copy, "✅ Rust DomBuilder snippet copied to clipboard!");
                                    }),
                            ]),
                    ]),
                div().class("grid grid-cols-1 md:grid-cols-3 gap-4")
                    .children(vec![
                        div().class("flex flex-col gap-1.5")
                            .children(vec![
                                span().class("text-[11px] font-mono text-indigo-400 font-bold").text("1. CSS Custom Properties (:root):"),
                                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-emerald-300 overflow-x-auto max-h-64 leading-relaxed whitespace-pre")
                                    .text(&css_code),
                            ]),
                        div().class("flex flex-col gap-1.5")
                            .children(vec![
                                span().class("text-[11px] font-mono text-indigo-400 font-bold").text("2. Rust ThemeConfig:"),
                                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-sky-300 overflow-x-auto max-h-64 leading-relaxed whitespace-pre")
                                    .text(&rust_code),
                            ]),
                        div().class("flex flex-col gap-1.5")
                            .children(vec![
                                span().class("text-[11px] font-mono text-indigo-400 font-bold").text("3. Rust DomBuilder Snippet:"),
                                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-purple-300 overflow-x-auto max-h-64 leading-relaxed whitespace-pre")
                                    .text(&dom_code),
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
                        h2().class("text-2xl font-black text-white tracking-tight").text("🎨 Theme Studio & Design System Handbook"),
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
