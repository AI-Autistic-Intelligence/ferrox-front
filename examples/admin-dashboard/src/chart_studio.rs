//! # Ferrox Front Nivo-Style Chart Studio & Playbook
//!
//! Interactive vector SVG charting workbench: switch chart types, modify data points,
//! add/remove records, adjust stroke and fill styling, preview real-time SVG geometry,
//! and dynamically generate and copy ready-to-use pure Rust code snippets.

use std::cell::RefCell;
use ferrox_front_core::dom::{div, span, p, h2, DomBuilder};
use ferrox_front_ui::components::*;
use ferrox_front_ui::layout::{container, row, col};
use ferrox_front_charts::{
    line_chart, area_chart, bar_chart, gauge_chart, sparkline_chart,
    SparklineType, Point, BarDatum,
};
use crate::{card_advanced, render_app};
use crate::theme_studio::copy_to_clipboard;

#[derive(Clone, Debug, PartialEq)]
pub enum StudioChartType {
    Line,
    Area,
    Bar,
    Gauge,
    Sparkline,
}

#[derive(Clone, Debug)]
pub struct ChartStudioState {
    pub chart_type: StudioChartType,
    pub values: Vec<f64>,
    pub stroke_width: u32,
    pub show_axes: bool,
    pub color_scheme: &'static str,
}

impl Default for ChartStudioState {
    fn default() -> Self {
        Self {
            chart_type: StudioChartType::Line,
            values: vec![18.0, 32.0, 26.0, 52.0, 44.0, 68.0, 75.0, 58.0, 89.0, 94.0],
            stroke_width: 3,
            show_axes: true,
            color_scheme: "indigo-cyan",
        }
    }
}

thread_local! {
    pub static CHART_STUDIO_STATE: RefCell<ChartStudioState> = RefCell::new(ChartStudioState::default());
}

pub fn generate_rust_chart_code(state: &ChartStudioState) -> String {
    let vals_str = state.values.iter().map(|v| format!("{:.1}", v)).collect::<Vec<_>>().join(", ");
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

// 2. Render Nivo-style SVG Line Chart directly to DOM
let chart = line_chart("nivo-line-chart", points, 760, 340, {});"#,
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

// 2. Render Gradient Area Chart
let chart = area_chart(points, 760, 340);"#,
                pts_code
            )
        },
        StudioChartType::Bar => {
            let bars_code = state.values.iter().enumerate().map(|(i, v)| {
                let color = match i % 4 {
                    0 => "#6366f1",
                    1 => "#06b6d4",
                    2 => "#10b981",
                    _ => "#a855f7",
                };
                format!("        BarDatum {{ label: \"T{}\".into(), value: {:.1}, color: \"{}\".into() }}", i + 1, v, color)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{bar_chart, BarDatum}};

// 1. Define discrete bar records
let data = vec![
{}
];

// 2. Render Bar Chart with axes & labels
let chart = bar_chart("nivo-bar-chart", data, 760, 340, {});"#,
                bars_code, state.show_axes
            )
        },
        StudioChartType::Gauge => {
            let latest = state.values.last().copied().unwrap_or(50.0);
            format!(
r#"use ferrox_front_charts::gauge_chart;

// Render High-Precision Telemetry Speedometer Gauge
let gauge = gauge_chart("telemetry-gauge", {:.1}, 0.0, 100.0, "Performance KPI", "%", 280, 160);"#,
                latest
            )
        },
        StudioChartType::Sparkline => {
            format!(
r##"use ferrox_front_charts::{{sparkline_chart, SparklineType}};

// Render Ultra-Compact Vector Sparkline for table cells
let data = vec![{}];
let spark = sparkline_chart(data, 180, 48, SparklineType::Area, "#10b981");"##,
                vals_str
            )
        }
    }
}

pub fn render_chart_studio_tab() -> DomBuilder {
    let state = CHART_STUDIO_STATE.with(|s| s.borrow().clone());

    // 1. Chart Type Switcher Toolbar
    let type_buttons = vec![
        ("Line Chart", StudioChartType::Line, "trend-up"),
        ("Area Chart", StudioChartType::Area, "layers"),
        ("Bar Chart", StudioChartType::Bar, "chart-bar"),
        ("Gauge Meter", StudioChartType::Gauge, "cpu"),
        ("Sparkline Mini", StudioChartType::Sparkline, "activity"),
    ];

    let mut type_nav_items = Vec::new();
    for (label, ctype, icon) in type_buttons {
        let is_active = state.chart_type == ctype;
        type_nav_items.push(
            button_advanced(label, if is_active { "primary" } else { "glass" }, "sm", false, Some(icon))
                .attr("style", if is_active { "font-weight: 700; box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);" } else { "opacity: 0.85;" })
                .on_click(move || {
                    CHART_STUDIO_STATE.with(|s| {
                        s.borrow_mut().chart_type = ctype.clone();
                    });
                    render_app("chart-studio");
                })
        );
    }

    // 2. Playbook Data Operations Bar
    let data_actions = div()
        .class("flex flex-wrap items-center gap-2.5 p-3 rounded-xl bg-slate-900/60 border border-slate-800/80")
        .children(vec![
            span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-2").text("Playbook Data:"),
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
            button_advanced("➖ Remove Point", "danger", "xs", false, None).on_click(|| {
                CHART_STUDIO_STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    if st.values.len() > 3 {
                        st.values.pop();
                    }
                });
                render_app("chart-studio");
            }),
            button_advanced("🎲 Randomize", "glass", "xs", false, None).on_click(|| {
                CHART_STUDIO_STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    st.values = vec![14.0, 48.0, 32.0, 82.0, 56.0, 69.0, 91.0, 46.0, 98.0, 74.0];
                });
                render_app("chart-studio");
            }),
            button_advanced("📈 Bullish Trend", "glass", "xs", false, None).on_click(|| {
                CHART_STUDIO_STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    st.values = vec![12.0, 22.0, 28.0, 36.0, 48.0, 59.0, 71.0, 82.0, 91.0, 98.0];
                });
                render_app("chart-studio");
            }),
            button_advanced("🌊 Volatility Wave", "glass", "xs", false, None).on_click(|| {
                CHART_STUDIO_STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    st.values = vec![85.0, 22.0, 92.0, 18.0, 89.0, 25.0, 94.0, 32.0, 88.0, 42.0];
                });
                render_app("chart-studio");
            }),
            button_advanced("🔄 Reset Defaults", "outline", "xs", false, None).on_click(|| {
                CHART_STUDIO_STATE.with(|s| {
                    *s.borrow_mut() = ChartStudioState::default();
                });
                render_app("chart-studio");
            }),
        ]);

    // 3. Render Chart Preview
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
            let data: Vec<BarDatum> = state.values.iter().enumerate().map(|(i, &v)| {
                let color = match i % 4 {
                    0 => "#6366f1",
                    1 => "#06b6d4",
                    2 => "#10b981",
                    _ => "#a855f7",
                };
                BarDatum {
                    label: format!("T{}", i + 1),
                    value: v,
                    color: color.to_string(),
                }
            }).collect();
            div().class("w-full flex items-center justify-center p-4 overflow-hidden")
                .child(bar_chart("studio-bar-preview", data, 760, 340, state.show_axes))
        },
        StudioChartType::Gauge => {
            let latest = state.values.last().copied().unwrap_or(50.0);
            div().class("w-full flex flex-col items-center justify-center p-6")
                .children(vec![
                    gauge_chart("studio-gauge-preview", latest, 0.0, 100.0, "Live Performance KPI", "%", 320, 180),
                    div().class("mt-4 text-xs text-slate-400 font-mono")
                        .text(&format!("Current Value: {:.1}% (Target: 85.0%)", latest)),
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
                                    sparkline_chart(state.values.clone(), 160, 48, SparklineType::Area, "#10b981"),
                                ]),
                            div().class("p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-sky-400 font-bold").text("Line Sparkline"),
                                    sparkline_chart(state.values.clone(), 160, 48, SparklineType::Line, "#38bdf8"),
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

    // 4. Data Point Chips Inspector
    let mut data_chips = Vec::new();
    for (idx, val) in state.values.iter().enumerate() {
        data_chips.push(
            div().class("flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-slate-900/80 border border-slate-800 text-xs font-mono")
                .children(vec![
                    span().class("text-slate-500 text-[10px]").text(&format!("#{}:", idx + 1)),
                    span().class("text-emerald-400 font-bold").text(&format!("{:.1}", val)),
                ])
        );
    }

    // 5. Dynamic Synchronized Rust Code Generator
    let rust_code = generate_rust_chart_code(&state);
    let rust_code_copy = rust_code.clone();

    let code_card = card_advanced(
        "Synchronized Rust Code Generator",
        "Dynamically updates with current data, curve settings, and options with 1-click copy",
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
                            .text("Zero JavaScript - Pure Rust WebAssembly SVG function call:"),
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
        "Zero canvas rasterization: pure scalable SVG geometry calculated in WebAssembly linear memory",
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
                                span().class("text-xs text-slate-400 font-medium").text("Axes & Coordinates:"),
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
                            ]),
                    ]),
                chart_container,
                div().class("pt-3 border-t border-slate-800/80 flex flex-wrap items-center gap-2")
                    .children(vec![
                        span().class("text-xs text-slate-400 font-medium mr-1").text("Data Points:"),
                        div().class("flex flex-wrap items-center gap-1.5").children(data_chips),
                    ]),
            ])
    );

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            div()
                .class("pb-4 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-4")
                .children(vec![
                    div().children(vec![
                        h2().class("text-2xl font-black text-white tracking-tight").text("📈 Nivo-Style Chart Studio & Playbook"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("Interactive vector SVG data visualization workbench: modify data points, test visual options, and copy generated pure Rust code."),
                    ]),
                    badge("Pure Rust Wasm SVG", "success"),
                ]),
            div().class("flex flex-wrap items-center gap-2.5 p-3 rounded-2xl bg-slate-900/60 border border-slate-800/80")
                .children(type_nav_items),
            data_actions,
            row().child(col("12").child(chart_preview_card)),
            row().child(col("12").child(code_card)),
        ])
}
