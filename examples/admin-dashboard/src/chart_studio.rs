//! # Ferrox Front Vector Chart Studio & Interactive Playbook
//!
//! Interactive vector SVG charting workbench: switch chart types, modify data points,
//! add/remove records, adjust stroke and fill styling, preview real-time SVG geometry,
//! and dynamically generate and copy ready-to-use pure Rust WebAssembly code snippets.

use std::cell::RefCell;
use ferrox_front_core::dom::{div, span, p, h2, DomBuilder};
use ferrox_front_ui::components::*;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_charts::{
    line_chart, area_chart, bar_chart, donut_chart, radar_chart, gauge_chart_styled, sparkline_chart,
    SparklineType, Point, BarDatum, PieDatum, RadarDatum, GaugeChartOptions,
};
use crate::{card_advanced, render_app};
use crate::theme_studio::copy_to_clipboard;

#[derive(Clone, Debug, PartialEq)]
pub enum StudioChartType {
    Line,
    Area,
    Bar,
    Donut,
    Radar,
    Gauge,
    Sparkline,
}

#[derive(Clone, Debug)]
pub struct ColorPaletteOption {
    pub id: &'static str,
    pub name: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
}

pub const COLOR_PALETTES: &[ColorPaletteOption] = &[
    ColorPaletteOption { id: "cyber-indigo", name: "Cyber Indigo", primary: "#6366f1", secondary: "#06b6d4" },
    ColorPaletteOption { id: "emerald-surge", name: "Emerald Surge", primary: "#10b981", secondary: "#34d399" },
    ColorPaletteOption { id: "amber-sunset", name: "Amber Sunset", primary: "#f59e0b", secondary: "#ef4444" },
    ColorPaletteOption { id: "neon-fuchsia", name: "Neon Fuchsia", primary: "#d946ef", secondary: "#8b5cf6" },
    ColorPaletteOption { id: "zinc-mono", name: "Zinc Mono", primary: "#e2e8f0", secondary: "#94a3b8" },
];

#[derive(Clone, Debug)]
pub struct ChartStudioState {
    pub chart_type: StudioChartType,
    pub values: Vec<f64>,
    pub stroke_width: u32,
    pub show_axes: bool,
    pub active_palette_id: String,
    pub custom_hex: String,
    pub gauge_warning_thresh: f64,
    pub gauge_critical_thresh: f64,
}

impl Default for ChartStudioState {
    fn default() -> Self {
        Self {
            chart_type: StudioChartType::Line,
            values: vec![18.0, 32.0, 26.0, 52.0, 44.0, 68.0, 75.0, 58.0, 89.0, 94.0],
            stroke_width: 3,
            show_axes: true,
            active_palette_id: "cyber-indigo".into(),
            custom_hex: "#6366f1".into(),
            gauge_warning_thresh: 0.60,
            gauge_critical_thresh: 0.82,
        }
    }
}

thread_local! {
    pub static CHART_STUDIO_STATE: RefCell<ChartStudioState> = RefCell::new(ChartStudioState::default());
}

impl ChartStudioState {
    pub fn current_palette(&self) -> ColorPaletteOption {
        COLOR_PALETTES
            .iter()
            .find(|p| p.id == self.active_palette_id)
            .cloned()
            .unwrap_or(COLOR_PALETTES[0].clone())
    }
}

pub fn generate_rust_chart_code(state: &ChartStudioState) -> String {
    let vals_str = state.values.iter().map(|v| format!("{:.1}", v)).collect::<Vec<_>>().join(", ");
    let pal = state.current_palette();

    match state.chart_type {
        StudioChartType::Line => {
            let pts_code = state.values.iter().enumerate().map(|(i, v)| {
                format!("        Point {{ x: {:.1}, y: {:.1} }}", i as f64, v)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{line_chart, Point}};

// 1. Vector data coordinates
let points = vec![
{}
];

// 2. Render Ferrox Vector SVG Line Chart directly to DOM
let chart = line_chart("vector-line-chart", points, 760, 340, {});"#,
                pts_code, state.show_axes
            )
        },
        StudioChartType::Area => {
            let pts_code = state.values.iter().enumerate().map(|(i, v)| {
                let x = 40.0 + (i as f64 / (state.values.len().max(2) - 1) as f64) * 680.0;
                let y = 300.0 - (v / 100.0) * 240.0;
                format!("        Point {{ x: {:.1}, y: {:.1} }}", x, y)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{area_chart, Point}};

// 1. Baseline coordinates for gradient area fill
let points = vec![
{}
];

// 2. Render High-Performance Gradient Area Chart
let chart = area_chart(points, 760, 340);"#,
                pts_code
            )
        },
        StudioChartType::Bar => {
            let colors = [pal.primary, pal.secondary, "#10b981", "#a855f7", "#f59e0b"];
            let bars_code = state.values.iter().enumerate().map(|(i, v)| {
                let color = colors[i % colors.len()];
                format!("        BarDatum {{ label: \"T{}\".into(), value: {:.1}, color: \"{}\".into() }}", i + 1, v, color)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{bar_chart, BarDatum}};

// 1. Define discrete bar records with theme colors
let data = vec![
{}
];

// 2. Render Vector Bar Chart with axes & labels
let chart = bar_chart("vector-bar-chart", data, 760, 340, {});"#,
                bars_code, state.show_axes
            )
        },
        StudioChartType::Donut => {
            let colors = [pal.primary, pal.secondary, "#10b981", "#f59e0b", "#a855f7", "#ec4899"];
            let slices_code = state.values.iter().take(6).enumerate().map(|(i, v)| {
                let color = colors[i % colors.len()];
                format!("        PieDatum {{ label: \"Slice {}\".into(), value: {:.1}, color: \"{}\".into() }}", i + 1, v, color)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{donut_chart, PieDatum}};

// 1. Define pie/donut proportional slices
let data = vec![
{}
];

// 2. Render Vector Donut Chart with inner cutout & central KPI
let chart = donut_chart(data, 120.0, 75.0, "Total Score", "{:.1}%");"#,
                slices_code, state.values.iter().copied().fold(0.0, |acc, x| acc + x) / state.values.len().max(1) as f64
            )
        },
        StudioChartType::Radar => {
            let attrs = ["Throughput", "Latency", "Resilience", "Security", "Scalability", "Reliability", "Concurrency", "Efficiency"];
            let radar_code = state.values.iter().take(attrs.len()).enumerate().map(|(i, v)| {
                format!("        RadarDatum {{ attribute: \"{}\".into(), value: {:.1} }}", attrs[i], v.clamp(0.0, 100.0))
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{radar_chart, RadarDatum}};

// 1. Define multi-attribute radar coordinates
let data = vec![
{}
];

// 2. Render Polygonal Spider / Radar Evaluation Chart
let chart = radar_chart(data, 110.0);"#,
                radar_code
            )
        },
        StudioChartType::Gauge => {
            let latest = state.values.last().copied().unwrap_or(50.0);
            format!(
r#"use ferrox_front_charts::{{gauge_chart_styled, GaugeChartOptions}};

// 1. Configure concentric thresholds and styling options
let mut options = GaugeChartOptions::default();
options.warning_threshold = {:.2};
options.critical_threshold = {:.2};
options.custom_color = Some("{}".into());

// 2. Render High-Precision Telemetry Speedometer Gauge (Concentric Arc & Needle)
let gauge = gauge_chart_styled(
    "telemetry-gauge",
    {:.1},
    0.0,
    100.0,
    "Performance KPI",
    "%",
    320,
    180,
    &options,
);"#,
                state.gauge_warning_thresh,
                state.gauge_critical_thresh,
                state.custom_hex,
                latest
            )
        },
        StudioChartType::Sparkline => {
            format!(
r##"use ferrox_front_charts::{{sparkline_chart, SparklineType}};

// Render Ultra-Compact Vector Sparklines with dynamic palette
let data = vec![{}];
let spark_area = sparkline_chart(data.clone(), 180, 48, SparklineType::Area, "{}");
let spark_line = sparkline_chart(data.clone(), 180, 48, SparklineType::Line, "{}");
let spark_bar  = sparkline_chart(data, 180, 48, SparklineType::Bar, "{}");"##,
                vals_str,
                pal.primary,
                pal.secondary,
                "#a855f7"
            )
        }
    }
}

pub fn render_chart_studio_tab() -> DomBuilder {
    let state = CHART_STUDIO_STATE.with(|s| s.borrow().clone());
    let current_palette = state.current_palette();

    // 1. Segmented Glass Pill Bar (Component Toolbar)
    let type_definitions = vec![
        ("Line Chart", StudioChartType::Line, "trend-up"),
        ("Area Chart", StudioChartType::Area, "layers"),
        ("Bar Chart", StudioChartType::Bar, "chart-bar"),
        ("Donut Chart", StudioChartType::Donut, "activity"),
        ("Radar Metrics", StudioChartType::Radar, "shield"),
        ("Gauge Meter", StudioChartType::Gauge, "cpu"),
        ("Sparkline Mini", StudioChartType::Sparkline, "zap"),
    ];

    let mut segmented_pill_items = Vec::new();
    for (label, ctype, icon) in type_definitions {
        let is_active = state.chart_type == ctype;
        let ctype_clone = ctype.clone();

        let mut pill = button_advanced(
            label,
            if is_active { "primary" } else { "glass" },
            "sm",
            false,
            Some(icon),
        );

        if is_active {
            pill = pill.attr(
                "style",
                "background: linear-gradient(135deg, var(--accent-primary, #6366f1), var(--accent-secondary, #a855f7)); \
                 box-shadow: 0 4px 18px rgba(99, 102, 241, 0.45); \
                 border: 1px solid rgba(255, 255, 255, 0.35); \
                 font-weight: 700; \
                 color: #ffffff; \
                 padding: 0.5rem 1.1rem; \
                 transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);"
            ).child(
                span()
                    .attr("style", "margin-left: 8px; font-size: 10px; font-weight: 800; background: rgba(0, 0, 0, 0.35); border-radius: 9999px; padding: 2px 7px; letter-spacing: 0.05em; color: #a7f3d0;")
                    .text("● ACTIVE")
            );
        } else {
            pill = pill.attr(
                "style",
                "background: rgba(15, 23, 42, 0.6); \
                 border: 1px solid rgba(148, 163, 184, 0.15); \
                 color: #cbd5e1; \
                 font-weight: 600; \
                 padding: 0.5rem 1.0rem; \
                 transition: all 0.2s ease;"
            );
        }

        pill = pill.on_click(move || {
            CHART_STUDIO_STATE.with(|s| {
                s.borrow_mut().chart_type = ctype_clone.clone();
            });
            render_app("chart-studio");
        });

        segmented_pill_items.push(pill);
    }

    let segmented_toolbar = div()
        .class("flex flex-wrap items-center gap-2.5 p-3 rounded-2xl bg-slate-900/75 border border-slate-800/80 backdrop-blur-md shadow-xl")
        .children(segmented_pill_items);

    // 2. Color Engine Toolbar
    let mut palette_buttons = Vec::new();
    for pal in COLOR_PALETTES {
        let is_sel = state.active_palette_id == pal.id;
        let pal_id = pal.id;
        let pal_primary = pal.primary;
        palette_buttons.push(
            button_advanced(pal.name, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", &format!(
                    "border-left: 4px solid {}; {}",
                    pal.primary,
                    if is_sel { "font-weight: 700; transform: scale(1.04); box-shadow: 0 2px 10px rgba(99, 102, 241, 0.35);" } else { "opacity: 0.85;" }
                ))
                .on_click(move || {
                    CHART_STUDIO_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.active_palette_id = pal_id.to_string();
                        st.custom_hex = pal_primary.to_string();
                    });
                    render_app("chart-studio");
                })
        );
    }

    // Direct color presets
    let direct_colors = vec![
        ("#6366f1", "Indigo"),
        ("#10b981", "Emerald"),
        ("#06b6d4", "Cyan"),
        ("#f59e0b", "Amber"),
        ("#ec4899", "Rose"),
        ("#8b5cf6", "Violet"),
    ];

    let mut direct_color_buttons = Vec::new();
    for (hex, name) in direct_colors {
        let is_sel = state.custom_hex == hex;
        let hex_str = hex.to_string();
        direct_color_buttons.push(
            button_advanced(name, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                .attr("style", &format!(
                    "background: {}; color: #fff; font-weight: 700; opacity: {}; border: {};",
                    hex,
                    if is_sel { "1.0" } else { "0.65" },
                    if is_sel { "2px solid #ffffff" } else { "1px solid transparent" }
                ))
                .on_click(move || {
                    CHART_STUDIO_STATE.with(|s| {
                        s.borrow_mut().custom_hex = hex_str.clone();
                    });
                    render_app("chart-studio");
                })
        );
    }

    let color_engine_panel = div()
        .class("flex flex-wrap items-center justify-between gap-3 p-3 rounded-xl bg-slate-900/60 border border-slate-800/80")
        .children(vec![
            div().class("flex flex-wrap items-center gap-2")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-1").text("Color Engine:"),
                    div().class("flex flex-wrap items-center gap-1.5").children(palette_buttons),
                ]),
            div().class("flex flex-wrap items-center gap-2")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-1").text("Tint:"),
                    div().class("flex flex-wrap items-center gap-1.5").children(direct_color_buttons),
                ]),
        ]);

    // 3. Playbook Data Operations Bar
    let data_actions = div()
        .class("flex flex-wrap items-center justify-between gap-3 p-3 rounded-xl bg-slate-900/60 border border-slate-800/80")
        .children(vec![
            div().class("flex flex-wrap items-center gap-2")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-1").text("Data Points:"),
                    button_advanced("➕ Add Point", "primary", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            let count = st.values.len();
                            let last = st.values.last().copied().unwrap_or(50.0);
                            let delta = ((count * 17) % 35) as f64 - 15.0;
                            let new_val = (last + delta).max(10.0).min(95.0);
                            st.values.push((new_val * 10.0).round() / 10.0);
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("➖ Remove Last", "danger", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            if st.values.len() > 3 {
                                st.values.pop();
                            }
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("⬆️ Sort Asc", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("⬇️ Sort Desc", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("🔀 Reverse", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values.reverse();
                        });
                        render_app("chart-studio");
                    }),
                ]),
            div().class("flex flex-wrap items-center gap-2")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-1").text("Presets:"),
                    button_advanced("📈 Bullish", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values = vec![12.0, 22.0, 28.0, 36.0, 48.0, 59.0, 71.0, 82.0, 91.0, 98.0];
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("🌊 Volatility", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values = vec![85.0, 22.0, 92.0, 18.0, 89.0, 25.0, 94.0, 32.0, 88.0, 42.0];
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("🎲 Random", "glass", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.values = vec![14.0, 48.0, 32.0, 82.0, 56.0, 69.0, 91.0, 46.0, 98.0, 74.0];
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("🔄 Reset", "outline", "xs", false, None).on_click(|| {
                        CHART_STUDIO_STATE.with(|s| {
                            *s.borrow_mut() = ChartStudioState::default();
                        });
                        render_app("chart-studio");
                    }),
                ]),
        ]);

    // 4. Interactive Data Table & Points Inspector
    let mut point_cards = Vec::new();
    for (idx, &val) in state.values.iter().enumerate() {
        let idx_capture = idx;
        point_cards.push(
            div()
                .class("flex items-center justify-between p-2 rounded-xl bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[130px]")
                .children(vec![
                    span().class("text-[11px] font-mono text-slate-400 font-bold").text(&format!("#{}", idx + 1)),
                    span().class("text-xs font-mono font-bold text-emerald-400").text(&format!("{:.1}", val)),
                    div().class("flex items-center gap-1")
                        .children(vec![
                            button_advanced("+5", "glass", "xs", false, None)
                                .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                                .on_click(move || {
                                    CHART_STUDIO_STATE.with(|s| {
                                        let mut st = s.borrow_mut();
                                        if idx_capture < st.values.len() {
                                            st.values[idx_capture] = (st.values[idx_capture] + 5.0).min(100.0);
                                        }
                                    });
                                    render_app("chart-studio");
                                }),
                            button_advanced("-5", "glass", "xs", false, None)
                                .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                                .on_click(move || {
                                    CHART_STUDIO_STATE.with(|s| {
                                        let mut st = s.borrow_mut();
                                        if idx_capture < st.values.len() {
                                            st.values[idx_capture] = (st.values[idx_capture] - 5.0).max(5.0);
                                        }
                                    });
                                    render_app("chart-studio");
                                }),
                            button_advanced("×", "danger", "xs", false, None)
                                .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                                .on_click(move || {
                                    CHART_STUDIO_STATE.with(|s| {
                                        let mut st = s.borrow_mut();
                                        if st.values.len() > 3 && idx_capture < st.values.len() {
                                            st.values.remove(idx_capture);
                                        }
                                    });
                                    render_app("chart-studio");
                                }),
                        ]),
                ])
        );
    }

    let data_table_card = card_advanced(
        "Interactive Data Points Inspector",
        "Direct numeric modification: increment, decrement, and delete individual coordinates with instant Wasm re-rendering",
        "grid",
        "glass",
        false,
        "p-5",
    )
    .child(
        div().class("flex flex-wrap items-center gap-2")
            .children(point_cards)
    );

    // 5. Render Chart Preview
    let chart_container = match state.chart_type {
        StudioChartType::Line => {
            let points: Vec<Point> = state.values.iter().enumerate().map(|(i, &v)| Point { x: i as f64, y: v }).collect();
            div().class("w-full flex items-center justify-center p-4 overflow-hidden")
                .child(line_chart("studio-line-preview", points, 760, 340, state.show_axes))
        },
        StudioChartType::Area => {
            let max_v = state.values.iter().cloned().fold(1.0, f64::max);
            let points: Vec<Point> = state.values.iter().enumerate().map(|(i, &v)| {
                let x = 40.0 + (i as f64 / (state.values.len().max(2) - 1) as f64) * 680.0;
                let y = 300.0 - (v / max_v) * 240.0;
                Point { x, y }
            }).collect();
            div().class("w-full flex items-center justify-center p-4 overflow-hidden")
                .child(area_chart(points, 760, 340))
        },
        StudioChartType::Bar => {
            let colors = [current_palette.primary, current_palette.secondary, "#10b981", "#a855f7", "#f59e0b"];
            let data: Vec<BarDatum> = state.values.iter().enumerate().map(|(i, &v)| {
                let color = colors[i % colors.len()];
                BarDatum {
                    label: format!("T{}", i + 1),
                    value: v,
                    color: color.to_string(),
                }
            }).collect();
            div().class("w-full flex items-center justify-center p-4 overflow-hidden")
                .child(bar_chart("studio-bar-preview", data, 760, 340, state.show_axes))
        },
        StudioChartType::Donut => {
            let colors = [current_palette.primary, current_palette.secondary, "#10b981", "#f59e0b", "#a855f7", "#ec4899"];
            let pie_data: Vec<PieDatum> = state.values.iter().take(6).enumerate().map(|(i, &v)| {
                let color = colors[i % colors.len()];
                PieDatum {
                    label: format!("Part {}", i + 1),
                    value: v,
                    color: color.to_string(),
                }
            }).collect();
            let avg = state.values.iter().copied().fold(0.0, |acc, x| acc + x) / state.values.len().max(1) as f64;
            div().class("w-full flex flex-col items-center justify-center p-6")
                .child(donut_chart(pie_data, 120.0, 75.0, "Efficiency KPI", &format!("{:.1}%", avg)))
        },
        StudioChartType::Radar => {
            let attrs = ["Throughput", "Latency", "Resilience", "Security", "Scalability", "Reliability", "Concurrency", "Efficiency"];
            let radar_data: Vec<RadarDatum> = state.values.iter().take(attrs.len()).enumerate().map(|(i, &v)| {
                RadarDatum {
                    attribute: attrs[i].to_string(),
                    value: v.clamp(0.0, 100.0),
                }
            }).collect();
            div().class("w-full flex items-center justify-center p-6")
                .child(radar_chart(radar_data, 110.0))
        },
        StudioChartType::Gauge => {
            let latest = state.values.last().copied().unwrap_or(50.0);
            let mut opts = GaugeChartOptions::default();
            opts.warning_threshold = state.gauge_warning_thresh;
            opts.critical_threshold = state.gauge_critical_thresh;
            opts.custom_color = Some(state.custom_hex.clone());

            div().class("w-full flex flex-col items-center justify-center p-6")
                .children(vec![
                    gauge_chart_styled("studio-gauge-preview", latest, 0.0, 100.0, "Live Performance KPI", "%", 320, 180, &opts),
                    div().class("mt-4 text-xs text-slate-400 font-mono")
                        .text(&format!("Current Value: {:.1}% | Concentric Semicircle Alignment (large-arc = 0)", latest)),
                ])
        },
        StudioChartType::Sparkline => {
            div().class("w-full flex flex-col items-center justify-center p-8 gap-5")
                .children(vec![
                    span().class("text-xs text-slate-300 font-bold uppercase tracking-wider").text("Inline Sparkline Vector Variants:"),
                    div().class("grid grid-cols-1 sm:grid-cols-3 gap-6 w-full max-w-2xl")
                        .children(vec![
                            div().class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-emerald-400 font-bold").text("Area Sparkline"),
                                    sparkline_chart(state.values.clone(), 160, 48, SparklineType::Area, current_palette.primary),
                                ]),
                            div().class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-sky-400 font-bold").text("Line Sparkline"),
                                    sparkline_chart(state.values.clone(), 160, 48, SparklineType::Line, current_palette.secondary),
                                ]),
                            div().class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-purple-400 font-bold").text("Bar Sparkline"),
                                    sparkline_chart(state.values.clone(), 160, 48, SparklineType::Bar, "#a855f7"),
                                ]),
                        ])
                ])
        }
    };

    // 6. Dynamic Synchronized Rust Code Generator
    let rust_code = generate_rust_chart_code(&state);
    let rust_code_copy = rust_code.clone();

    let code_card = card_advanced(
        "Synchronized Pure Rust Wasm Code Generator",
        "Dynamically updates with current data points, selected color theme, and curve options with 1-click copy",
        "code",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-4")
            .children(vec![
                div().class("flex items-center justify-between pb-2")
                    .children(vec![
                        span().class("text-xs text-slate-300 font-medium")
                            .text("Zero JavaScript - Native Ferrox Front SVG Vector Builder:"),
                        button_advanced("📋 Copy Rust Snippet", "primary", "xs", false, Some("download"))
                            .on_click(move || {
                                copy_to_clipboard(&rust_code_copy, "✅ Rust Chart snippet copied to clipboard!");
                            }),
                    ]),
                div().class("p-4 rounded-xl bg-slate-950 border border-slate-800 text-xs font-mono text-emerald-300 overflow-x-auto leading-relaxed whitespace-pre")
                    .text(&rust_code),
            ])
    );

    let chart_preview_card = card_advanced(
        "Interactive Vector SVG Canvas",
        "Zero rasterization: pure scalable SVG geometry calculated in WebAssembly linear memory",
        "activity",
        "glass",
        false,
        "p-6",
    )
    .child(
        div().class("space-y-4")
            .children(vec![
                div().class("flex flex-wrap items-center justify-between gap-4 pb-2 border-b border-slate-800/80")
                    .children(vec![
                        div().class("flex items-center gap-2")
                            .children(vec![
                                span().class("text-xs text-slate-400 font-medium").text("Coordinates & Axes:"),
                                button_advanced(if state.show_axes { "Axes: ON" } else { "Axes: OFF" }, if state.show_axes { "primary" } else { "outline" }, "xs", false, None)
                                    .on_click(|| {
                                        CHART_STUDIO_STATE.with(|s| {
                                            let mut st = s.borrow_mut();
                                            st.show_axes = !st.show_axes;
                                        });
                                        render_app("chart-studio");
                                    }),
                            ]),
                        div().class("flex items-center gap-2")
                            .children(vec![
                                span().class("text-xs text-slate-400 font-medium").text("Data Records:"),
                                badge(&format!("{} Points", state.values.len()), "primary"),
                                badge(&format!("Palette: {}", current_palette.name), "info"),
                            ]),
                    ]),
                chart_container,
            ])
    );

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("pb-4 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-4")
                .children(vec![
                    div().children(vec![
                        h2().class("text-2xl font-black text-white tracking-tight").text("📈 Vector Chart Studio & Playbook"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("Interactive vector SVG data visualization workbench: modify data points, inspect styling options, and copy generated pure Rust code."),
                    ]),
                    badge("Pure Rust Wasm SVG", "success"),
                ]),
            segmented_toolbar,
            color_engine_panel,
            data_actions,
            data_table_card,
            row().child(col("12").child(chart_preview_card)),
            row().child(col("12").child(code_card)),
        ])
}
