//! # Ferrox Front Vector Chart Studio & Component Playbook
//!
//! High-Density Engineering Workbench ("Playbook & Live Knobs Inspector")
//! with dedicated, strongly-typed domain architectures per component (Line, Area, Bar,
//! Donut, Radar, Gauge, Sparkline), real-time interactive Knobs Inspector, pervasive
//! 6-palette Color Engine, deterministic algorithmic data generators (Brownian Motion,
//! Harmonic Wave, Traffic Spikes, Normal Bell), interactive data table editor, and
//! live synchronized pure Rust WebAssembly code generation with 1-click clipboard copy.

use std::cell::RefCell;
use ferrox_front_core::dom::{div, span, p, h2, DomBuilder};
use ferrox_front_ui::components::*;
use ferrox_front_ui::layout::container;
use ferrox_front_charts::{
    line_chart_styled, area_chart_styled, bar_chart_styled, donut_chart, radar_chart,
    gauge_chart_styled, sparkline_chart, LineCurve, LineChartOptions, AreaChartOptions,
    BarChartOptions, GaugeChartOptions, SparklineType, Point, BarDatum, PieDatum, RadarDatum,
};
use crate::data_generators::*;
use crate::{card_advanced, render_app};
use crate::theme_studio::copy_to_clipboard;

// ============================================================================
// 1. TYPED COMPONENT DATA ARCHITECTURES (Milestone 1)
// ============================================================================

/// Identifies the component type active in the workbench.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentKind {
    Line,
    Area,
    Bar,
    Donut,
    Radar,
    Gauge,
    Sparkline,
}

/// Dedicated domain configuration for Line charts.
#[derive(Clone, Debug)]
pub struct LineConfig {
    pub points: Vec<(f64, f64)>,
    pub curve: LineCurve,
    pub stroke_width: f64,
    pub show_dots: bool,
    pub show_axes: bool,
}

impl Default for LineConfig {
    fn default() -> Self {
        Self {
            points: vec![
                (0.0, 18.0), (1.0, 32.0), (2.0, 26.0), (3.0, 52.0),
                (4.0, 44.0), (5.0, 68.0), (6.0, 75.0), (7.0, 58.0),
                (8.0, 89.0), (9.0, 94.0),
            ],
            curve: LineCurve::Monotone,
            stroke_width: 3.0,
            show_dots: true,
            show_axes: true,
        }
    }
}

/// Dedicated domain configuration for Area charts.
#[derive(Clone, Debug)]
pub struct AreaConfig {
    pub points: Vec<(f64, f64)>,
    pub curve: LineCurve,
    pub stroke_width: f64,
    pub fill_opacity: f64,
    pub show_axes: bool,
}

impl Default for AreaConfig {
    fn default() -> Self {
        Self {
            points: vec![
                (0.0, 25.0), (1.0, 42.0), (2.0, 38.0), (3.0, 65.0),
                (4.0, 58.0), (5.0, 82.0), (6.0, 70.0), (7.0, 88.0),
                (8.0, 92.0), (9.0, 85.0),
            ],
            curve: LineCurve::Monotone,
            stroke_width: 3.0,
            fill_opacity: 0.35,
            show_axes: true,
        }
    }
}

/// Individual categorical record for Bar charts.
#[derive(Clone, Debug)]
pub struct BarItem {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

/// Dedicated domain configuration for Bar charts.
#[derive(Clone, Debug)]
pub struct BarConfig {
    pub items: Vec<BarItem>,
    pub padding: f64,
    pub rounded_corners: bool,
    pub show_axes: bool,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            items: vec![
                BarItem { label: "API Gateway".into(), value: 84.0, color: None },
                BarItem { label: "Auth Proxy".into(), value: 62.0, color: None },
                BarItem { label: "Wasm Core".into(), value: 95.0, color: None },
                BarItem { label: "Redis Cache".into(), value: 48.0, color: None },
                BarItem { label: "Kafka PubSub".into(), value: 76.0, color: None },
                BarItem { label: "Edge CDN".into(), value: 91.0, color: None },
            ],
            padding: 18.0,
            rounded_corners: true,
            show_axes: true,
        }
    }
}

/// Individual slice for Donut/Pie charts.
#[derive(Clone, Debug)]
pub struct DonutItem {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

/// Dedicated domain configuration for Donut / Pie charts.
#[derive(Clone, Debug)]
pub struct DonutConfig {
    pub segments: Vec<DonutItem>,
    pub inner_radius_pct: f64,
    pub pad_angle: f64,
}

impl Default for DonutConfig {
    fn default() -> Self {
        Self {
            segments: vec![
                DonutItem { label: "Frontend Wasm".into(), value: 35.0, color: None },
                DonutItem { label: "Core Tokio".into(), value: 28.0, color: None },
                DonutItem { label: "Storage Engine".into(), value: 18.0, color: None },
                DonutItem { label: "Sentinel Security".into(), value: 12.0, color: None },
                DonutItem { label: "E2E Chaos Suite".into(), value: 7.0, color: None },
            ],
            inner_radius_pct: 62.0,
            pad_angle: 2.0,
        }
    }
}

/// Individual multi-dimensional axis for Radar charts.
#[derive(Clone, Debug)]
pub struct RadarAxis {
    pub label: String,
    pub value: f64,
}

/// Dedicated domain configuration for Radar charts.
#[derive(Clone, Debug)]
pub struct RadarConfig {
    pub dimensions: Vec<RadarAxis>,
    pub ring_count: usize,
    pub fill_opacity: f64,
}

impl Default for RadarConfig {
    fn default() -> Self {
        Self {
            dimensions: vec![
                RadarAxis { label: "Throughput".into(), value: 96.0 },
                RadarAxis { label: "Low Latency".into(), value: 92.0 },
                RadarAxis { label: "Zero Alloc".into(), value: 88.0 },
                RadarAxis { label: "Security".into(), value: 94.0 },
                RadarAxis { label: "Scalability".into(), value: 90.0 },
                RadarAxis { label: "Fault Tolerant".into(), value: 85.0 },
            ],
            ring_count: 4,
            fill_opacity: 0.25,
        }
    }
}

/// Dedicated domain configuration for Gauge telemetry speedometer.
#[derive(Clone, Debug)]
pub struct GaugeConfig {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub target: f64,
    pub unit: String,
    pub show_ticks: bool,
    pub warning_thresh: f64,
    pub critical_thresh: f64,
}

impl Default for GaugeConfig {
    fn default() -> Self {
        Self {
            value: 73.5,
            min: 0.0,
            max: 100.0,
            target: 85.0,
            unit: "%".into(),
            show_ticks: true,
            warning_thresh: 0.60,
            critical_thresh: 0.85,
        }
    }
}

/// Dedicated domain configuration for Sparklines.
#[derive(Clone, Debug)]
pub struct SparklineConfig {
    pub ticks: Vec<f64>,
    pub spark_type: SparklineType,
    pub show_trend: bool,
}

impl Default for SparklineConfig {
    fn default() -> Self {
        Self {
            ticks: vec![14.0, 22.0, 19.0, 35.0, 28.0, 42.0, 39.0, 58.0, 52.0, 68.0, 64.0, 78.0],
            spark_type: SparklineType::Area,
            show_trend: true,
        }
    }
}

// ============================================================================
// 2. PERVASIVE COLOR ENGINE PALETTES (Milestone 4)
// ============================================================================

/// Curated color palette metadata.
#[derive(Clone, Debug)]
pub struct ColorPaletteOption {
    pub id: &'static str,
    pub name: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
    pub accent: &'static str,
    pub colors: &'static [&'static str],
}

pub const COLOR_PALETTES: &[ColorPaletteOption] = &[
    ColorPaletteOption {
        id: "cyber-neon",
        name: "Cyber Neon",
        primary: "#ec4899",
        secondary: "#06b6d4",
        accent: "#a855f7",
        colors: &["#ec4899", "#06b6d4", "#a855f7", "#3b82f6", "#10b981", "#f59e0b"],
    },
    ColorPaletteOption {
        id: "emerald-flow",
        name: "Emerald Flow",
        primary: "#10b981",
        secondary: "#059669",
        accent: "#14b8a6",
        colors: &["#10b981", "#059669", "#14b8a6", "#34d399", "#065f46", "#6ee7b7"],
    },
    ColorPaletteOption {
        id: "amber-solar",
        name: "Amber Solar",
        primary: "#f59e0b",
        secondary: "#f97316",
        accent: "#ef4444",
        colors: &["#f59e0b", "#f97316", "#ef4444", "#fbbf24", "#ea580c", "#dc2626"],
    },
    ColorPaletteOption {
        id: "deep-ocean",
        name: "Deep Ocean",
        primary: "#0284c7",
        secondary: "#38bdf8",
        accent: "#6366f1",
        colors: &["#0284c7", "#38bdf8", "#6366f1", "#0369a1", "#0ea5e9", "#4f46e5"],
    },
    ColorPaletteOption {
        id: "zinc-mono",
        name: "Zinc Mono",
        primary: "#94a3b8",
        secondary: "#71717a",
        accent: "#f8fafc",
        colors: &["#94a3b8", "#71717a", "#cbd5e1", "#52525b", "#e2e8f0", "#a1a1aa"],
    },
    ColorPaletteOption {
        id: "synthwave-violet",
        name: "Synthwave Violet",
        primary: "#8b5cf6",
        secondary: "#d946ef",
        accent: "#6366f1",
        colors: &["#8b5cf6", "#d946ef", "#6366f1", "#a855f7", "#c084fc", "#ec4899"],
    },
];

// ============================================================================
// 3. PLAYBOOK GLOBAL REACTIVE STATE
// ============================================================================

/// Reactive state encompassing all typed configs, current component, and palette.
#[derive(Clone, Debug)]
pub struct PlaybookState {
    pub active_kind: ComponentKind,
    pub line_config: LineConfig,
    pub area_config: AreaConfig,
    pub bar_config: BarConfig,
    pub donut_config: DonutConfig,
    pub radar_config: RadarConfig,
    pub gauge_config: GaugeConfig,
    pub sparkline_config: SparklineConfig,
    pub active_palette_id: String,
    pub custom_hex: String,
}

impl Default for PlaybookState {
    fn default() -> Self {
        Self {
            active_kind: ComponentKind::Line,
            line_config: LineConfig::default(),
            area_config: AreaConfig::default(),
            bar_config: BarConfig::default(),
            donut_config: DonutConfig::default(),
            radar_config: RadarConfig::default(),
            gauge_config: GaugeConfig::default(),
            sparkline_config: SparklineConfig::default(),
            active_palette_id: "cyber-neon".into(),
            custom_hex: "#ec4899".into(),
        }
    }
}

thread_local! {
    pub static PLAYBOOK_STATE: RefCell<PlaybookState> = RefCell::new(PlaybookState::default());
}

impl PlaybookState {
    pub fn current_palette(&self) -> ColorPaletteOption {
        COLOR_PALETTES
            .iter()
            .find(|p| p.id == self.active_palette_id)
            .cloned()
            .unwrap_or(COLOR_PALETTES[0].clone())
    }
}

// ============================================================================
// 4. CODE GENERATOR (Milestone 7 / DomBuilder Synchronizer)
// ============================================================================

pub fn generate_rust_component_code(state: &PlaybookState) -> String {
    let pal = state.current_palette();

    match state.active_kind {
        ComponentKind::Line => {
            let pts_code = state.line_config.points.iter().map(|(x, y)| {
                format!("        Point {{ x: {:.1}, y: {:.1} }}", x, y)
            }).collect::<Vec<_>>().join(",\n");
            let curve_str = match state.line_config.curve {
                LineCurve::Linear => "LineCurve::Linear",
                LineCurve::Monotone => "LineCurve::Monotone",
                LineCurve::Stepped => "LineCurve::Stepped",
            };
            format!(
r#"use ferrox_front_charts::{{line_chart_styled, LineChartOptions, LineCurve, Point}};

// 1. Configure vector curve geometry & styling tokens
let mut options = LineChartOptions::default();
options.curve = {};
options.stroke_width = {:.1};
options.show_axes = {};
options.show_dots = {};
options.stroke_color = Some("{}".into());
options.gradient_start = Some("{}".into());
options.gradient_end = Some("{}".into());

// 2. Strongly-typed coordinate stream
let points = vec![
{}
];

// 3. Render pure vector SVG chart directly into DOM tree
let chart = line_chart_styled("line-chart", points, 720, 320, &options);"#,
                curve_str,
                state.line_config.stroke_width,
                state.line_config.show_axes,
                state.line_config.show_dots,
                pal.primary,
                pal.primary,
                pal.secondary,
                pts_code
            )
        },
        ComponentKind::Area => {
            let pts_code = state.area_config.points.iter().map(|(x, y)| {
                format!("        Point {{ x: {:.1}, y: {:.1} }}", x, y)
            }).collect::<Vec<_>>().join(",\n");
            let curve_str = match state.area_config.curve {
                LineCurve::Linear => "LineCurve::Linear",
                LineCurve::Monotone => "LineCurve::Monotone",
                LineCurve::Stepped => "LineCurve::Stepped",
            };
            format!(
r#"use ferrox_front_charts::{{area_chart_styled, AreaChartOptions, LineCurve, Point}};

// 1. Configure area fill opacity and monotonic Bezier curve
let mut options = AreaChartOptions::default();
options.curve = {};
options.stroke_width = {:.1};
options.fill_opacity = {:.2};
options.stroke_color = Some("{}".into());
options.fill_color = Some("{}".into());

// 2. Baseline coordinate stream
let points = vec![
{}
];

// 3. Render high-performance gradient area chart
let chart = area_chart_styled(points, 720, 320, &options);"#,
                curve_str,
                state.area_config.stroke_width,
                state.area_config.fill_opacity,
                pal.primary,
                pal.secondary,
                pts_code
            )
        },
        ComponentKind::Bar => {
            let colors = pal.colors;
            let bars_code = state.bar_config.items.iter().enumerate().map(|(i, it)| {
                let col = it.color.as_deref().unwrap_or(colors[i % colors.len()]);
                format!("        BarDatum {{ label: \"{}\".into(), value: {:.1}, color: \"{}\".into() }}", it.label, it.value, col)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{bar_chart_styled, BarChartOptions, BarDatum}};

// 1. Configure bar padding and rounded corners
let mut options = BarChartOptions::default();
options.bar_padding = {:.1};
options.rounded_corners = {};
options.show_axes = {};

// 2. Strongly-typed categorical bar records
let data = vec![
{}
];

// 3. Render pure vector SVG bar chart
let chart = bar_chart_styled("bar-chart", data, 720, 320, &options);"#,
                state.bar_config.padding,
                state.bar_config.rounded_corners,
                state.bar_config.show_axes,
                bars_code
            )
        },
        ComponentKind::Donut => {
            let colors = pal.colors;
            let slices_code = state.donut_config.segments.iter().enumerate().map(|(i, seg)| {
                let col = seg.color.as_deref().unwrap_or(colors[i % colors.len()]);
                format!("        PieDatum {{ label: \"{}\".into(), value: {:.1}, color: \"{}\".into() }}", seg.label, seg.value, col)
            }).collect::<Vec<_>>().join(",\n");
            let outer_r = 120.0;
            let inner_r = outer_r * (state.donut_config.inner_radius_pct / 100.0);
            let avg = state.donut_config.segments.iter().map(|s| s.value).sum::<f64>()
                / state.donut_config.segments.len().max(1) as f64;
            format!(
r#"use ferrox_front_charts::{{donut_chart, PieDatum}};

// 1. Proportional slice entries with palette colors
let data = vec![
{}
];

// 2. Render vector donut chart with inner cutout ({:.0}%)
let outer_radius = {:.1};
let inner_radius = {:.1};
let chart = donut_chart(data, outer_radius, inner_radius, "System KPI", "{:.1}%");"#,
                slices_code,
                state.donut_config.inner_radius_pct,
                outer_r,
                inner_r,
                avg
            )
        },
        ComponentKind::Radar => {
            let dims_code = state.radar_config.dimensions.iter().map(|d| {
                format!("        RadarDatum {{ attribute: \"{}\".into(), value: {:.1} }}", d.label, d.value)
            }).collect::<Vec<_>>().join(",\n");
            format!(
r#"use ferrox_front_charts::{{radar_chart, RadarDatum}};

// 1. Multi-axis radar metrics
let data = vec![
{}
];

// 2. Render polygonal spider evaluation chart
let chart = radar_chart(data, 115.0);"#,
                dims_code
            )
        },
        ComponentKind::Gauge => {
            let cfg = &state.gauge_config;
            format!(
r#"use ferrox_front_charts::{{gauge_chart_styled, GaugeChartOptions}};

// 1. Telemetry speedometer configuration with concentric geometry
let mut options = GaugeChartOptions::default();
options.warning_threshold = {:.2};
options.critical_threshold = {:.2};
options.custom_color = Some("{}".into());

// 2. Render calibrated concentric gauge with zero arc distortion
let gauge = gauge_chart_styled(
    "speedometer-gauge",
    {:.1},
    {:.1},
    {:.1},
    "Telemetry Gauge",
    "{}",
    340,
    190,
    &options,
);"#,
                cfg.warning_thresh,
                cfg.critical_thresh,
                pal.primary,
                cfg.value,
                cfg.min,
                cfg.max,
                cfg.unit
            )
        },
        ComponentKind::Sparkline => {
            let ticks_str = state.sparkline_config.ticks.iter()
                .map(|t| format!("{:.1}", t))
                .collect::<Vec<_>>()
                .join(", ");
            let variant_str = match state.sparkline_config.spark_type {
                SparklineType::Area => "SparklineType::Area",
                SparklineType::Line => "SparklineType::Line",
                SparklineType::Bar => "SparklineType::Bar",
            };
            format!(
r#"use ferrox_front_charts::{{sparkline_chart, SparklineType}};

// 1. High-frequency tick metrics
let ticks = vec![{}];

// 2. Render micro-sparkline vector indicator
let sparkline = sparkline_chart(ticks, 180, 48, {}, "{}");"#,
                ticks_str,
                variant_str,
                pal.primary
            )
        }
    }
}

// ============================================================================
// 5. MAIN TAB COMPONENT: WORKBENCH (Milestone 3, 5, 6)
// ============================================================================

pub fn render_chart_studio_tab() -> DomBuilder {
    let state = PLAYBOOK_STATE.with(|s| s.borrow().clone());
    let current_palette = state.current_palette();

    // ------------------------------------------------------------------------
    // A. Vitrified Segmented Pill Bar (Component Toolbar)
    // ------------------------------------------------------------------------
    let type_definitions = [
        ("Line Chart", ComponentKind::Line, "trend-up"),
        ("Area Chart", ComponentKind::Area, "layers"),
        ("Bar Chart", ComponentKind::Bar, "chart-bar"),
        ("Donut / Pie", ComponentKind::Donut, "activity"),
        ("Radar Spider", ComponentKind::Radar, "shield"),
        ("Gauge Meter", ComponentKind::Gauge, "cpu"),
        ("Sparkline Mini", ComponentKind::Sparkline, "zap"),
    ];

    let mut segmented_pill_items = Vec::new();
    for (label, kind, icon) in type_definitions {
        let is_active = state.active_kind == kind;

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
                "background: linear-gradient(135deg, #6366f1, #a855f7); \
                 box-shadow: 0 4px 20px rgba(99, 102, 241, 0.45); \
                 border: 1px solid rgba(255, 255, 255, 0.4); \
                 font-weight: 700; \
                 color: #ffffff; \
                 padding: 0.55rem 1.15rem; \
                 transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);"
            ).child(
                span()
                    .attr("style", "margin-left: 8px; font-size: 10px; font-weight: 800; background: rgba(0, 0, 0, 0.35); border-radius: 9999px; padding: 2px 7px; letter-spacing: 0.05em; color: #a7f3d0;")
                    .text("● ACTIVE")
            );
        } else {
            pill = pill.attr(
                "style",
                "background: rgba(15, 23, 42, 0.65); \
                 border: 1px solid rgba(148, 163, 184, 0.18); \
                 color: #cbd5e1; \
                 font-weight: 600; \
                 padding: 0.55rem 1.05rem; \
                 transition: all 0.2s ease;"
            );
        }

        pill = pill.on_click(move || {
            PLAYBOOK_STATE.with(|s| {
                s.borrow_mut().active_kind = kind;
            });
            render_app("chart-studio");
        });

        segmented_pill_items.push(pill);
    }

    let segmented_toolbar = div()
        .class("flex flex-wrap items-center gap-2.5 p-3 rounded-2xl bg-slate-900/80 border border-slate-800/80 backdrop-blur-xl shadow-2xl")
        .children(segmented_pill_items);

    // ------------------------------------------------------------------------
    // B. Left Column: Props & Knobs Inspector + Pervasive Color Engine (Col 3/12)
    // ------------------------------------------------------------------------
    let mut knob_controls = Vec::new();

    match state.active_kind {
        ComponentKind::Line => {
            // Curve Selector
            let current_curve = state.line_config.curve;
            let curve_btns = vec![
                ("Monotone", LineCurve::Monotone),
                ("Linear", LineCurve::Linear),
                ("Stepped", LineCurve::Stepped),
            ].into_iter().map(|(lbl, c)| {
                let is_sel = current_curve == c;
                button_advanced(lbl, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().line_config.curve = c;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Curve Interpolation:"),
                div().class("flex flex-wrap gap-1.5").children(curve_btns),
            ]));

            // Stroke Width
            let current_w = state.line_config.stroke_width;
            let width_btns = vec![1.5, 2.5, 3.5, 5.0, 7.0].into_iter().map(|w| {
                let is_sel = (current_w - w).abs() < 0.1;
                button_advanced(&format!("{:.1}px", w), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().line_config.stroke_width = w;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Stroke Width:"),
                div().class("flex flex-wrap gap-1.5").children(width_btns),
            ]));

            // Toggles
            let show_dots = state.line_config.show_dots;
            let show_axes = state.line_config.show_axes;
            knob_controls.push(div().class("flex flex-wrap gap-2 pt-2 border-t border-slate-800/80").children(vec![
                button_advanced(
                    if show_dots { "Dots: ON" } else { "Dots: OFF" },
                    if show_dots { "primary" } else { "outline" },
                    "xs", false, None,
                ).on_click(move || {
                    PLAYBOOK_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.line_config.show_dots = !st.line_config.show_dots;
                    });
                    render_app("chart-studio");
                }),
                button_advanced(
                    if show_axes { "Axes: ON" } else { "Axes: OFF" },
                    if show_axes { "primary" } else { "outline" },
                    "xs", false, None,
                ).on_click(move || {
                    PLAYBOOK_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.line_config.show_axes = !st.line_config.show_axes;
                    });
                    render_app("chart-studio");
                }),
            ]));
        },
        ComponentKind::Area => {
            // Curve Selector
            let current_curve = state.area_config.curve;
            let curve_btns = vec![
                ("Monotone", LineCurve::Monotone),
                ("Linear", LineCurve::Linear),
                ("Stepped", LineCurve::Stepped),
            ].into_iter().map(|(lbl, c)| {
                let is_sel = current_curve == c;
                button_advanced(lbl, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().area_config.curve = c;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Curve Interpolation:"),
                div().class("flex flex-wrap gap-1.5").children(curve_btns),
            ]));

            // Fill Opacity
            let current_op = state.area_config.fill_opacity;
            let op_btns = vec![0.15, 0.35, 0.55, 0.75, 0.90].into_iter().map(|op| {
                let is_sel = (current_op - op).abs() < 0.05;
                button_advanced(&format!("{:.0}%", op * 100.0), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().area_config.fill_opacity = op;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Fill Opacity:"),
                div().class("flex flex-wrap gap-1.5").children(op_btns),
            ]));

            // Stroke Width
            let current_w = state.area_config.stroke_width;
            let width_btns = vec![1.5, 3.0, 4.5].into_iter().map(|w| {
                let is_sel = (current_w - w).abs() < 0.1;
                button_advanced(&format!("{:.1}px", w), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().area_config.stroke_width = w;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Stroke Width:"),
                div().class("flex flex-wrap gap-1.5").children(width_btns),
            ]));
        },
        ComponentKind::Bar => {
            // Padding
            let current_pad = state.bar_config.padding;
            let pad_btns = vec![10.0, 18.0, 26.0, 36.0].into_iter().map(|p| {
                let is_sel = (current_pad - p).abs() < 1.0;
                button_advanced(&format!("{:.0}px", p), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().bar_config.padding = p;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Bar Spacing / Padding:"),
                div().class("flex flex-wrap gap-1.5").children(pad_btns),
            ]));

            let show_axes = state.bar_config.show_axes;
            knob_controls.push(div().class("flex flex-wrap gap-2 pt-2 border-t border-slate-800/80").children(vec![
                button_advanced(
                    if show_axes { "Axes: ON" } else { "Axes: OFF" },
                    if show_axes { "primary" } else { "outline" },
                    "xs", false, None,
                ).on_click(move || {
                    PLAYBOOK_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.bar_config.show_axes = !st.bar_config.show_axes;
                    });
                    render_app("chart-studio");
                }),
            ]));
        },
        ComponentKind::Donut => {
            // Cutout Radius
            let current_inner = state.donut_config.inner_radius_pct;
            let cutout_btns = vec![0.0, 40.0, 62.0, 75.0, 85.0].into_iter().map(|pct| {
                let is_sel = (current_inner - pct).abs() < 1.0;
                let lbl = if pct == 0.0 { "0% (Pie)".to_string() } else { format!("{:.0}%", pct) };
                button_advanced(&lbl, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().donut_config.inner_radius_pct = pct;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Cutout Radius (%):"),
                div().class("flex flex-wrap gap-1.5").children(cutout_btns),
            ]));
        },
        ComponentKind::Radar => {
            let current_rings = state.radar_config.ring_count;
            let ring_btns = vec![3, 4, 5, 6].into_iter().map(|r| {
                let is_sel = current_rings == r;
                button_advanced(&format!("{} Rings", r), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().radar_config.ring_count = r;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Polygon Grid Rings:"),
                div().class("flex flex-wrap gap-1.5").children(ring_btns),
            ]));

            let current_op = state.radar_config.fill_opacity;
            let op_btns = vec![0.15, 0.25, 0.40, 0.60].into_iter().map(|op| {
                let is_sel = (current_op - op).abs() < 0.05;
                button_advanced(&format!("{:.0}%", op * 100.0), if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().radar_config.fill_opacity = op;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Polygon Fill Opacity:"),
                div().class("flex flex-wrap gap-1.5").children(op_btns),
            ]));
        },
        ComponentKind::Gauge => {
            let cur_val = state.gauge_config.value;
            knob_controls.push(div().class("space-y-2").children(vec![
                span().class("text-xs font-bold text-slate-300").text(&format!("Current Speedometer: {:.1}%", cur_val)),
                div().class("flex flex-wrap gap-1.5").children(vec![
                    button_advanced("-10", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.value = (st.gauge_config.value - 10.0).max(0.0);
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("-5", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.value = (st.gauge_config.value - 5.0).max(0.0);
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("+5", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.value = (st.gauge_config.value + 5.0).min(100.0);
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("+10", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.value = (st.gauge_config.value + 10.0).min(100.0);
                        });
                        render_app("chart-studio");
                    }),
                ]),
            ]));

            let warn_val = state.gauge_config.warning_thresh;
            let crit_val = state.gauge_config.critical_thresh;
            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text(&format!("Thresholds (Warn: {:.0}%, Crit: {:.0}%):", warn_val * 100.0, crit_val * 100.0)),
                div().class("flex flex-wrap gap-1.5").children(vec![
                    button_advanced("Preset 50/80", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.warning_thresh = 0.50;
                            st.gauge_config.critical_thresh = 0.80;
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("Preset 65/90", "glass", "xs", false, None).on_click(|| {
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            st.gauge_config.warning_thresh = 0.65;
                            st.gauge_config.critical_thresh = 0.90;
                        });
                        render_app("chart-studio");
                    }),
                ]),
            ]));
        },
        ComponentKind::Sparkline => {
            let cur_type = state.sparkline_config.spark_type;
            let type_btns = vec![
                ("Area", SparklineType::Area),
                ("Line", SparklineType::Line),
                ("Bar", SparklineType::Bar),
            ].into_iter().map(|(lbl, t)| {
                let is_sel = cur_type == t;
                button_advanced(lbl, if is_sel { "primary" } else { "glass" }, "xs", false, None)
                    .on_click(move || {
                        PLAYBOOK_STATE.with(|s| {
                            s.borrow_mut().sparkline_config.spark_type = t;
                        });
                        render_app("chart-studio");
                    })
            }).collect::<Vec<_>>();

            knob_controls.push(div().class("space-y-1.5").children(vec![
                span().class("text-xs font-bold text-slate-300").text("Sparkline Representation:"),
                div().class("flex flex-wrap gap-1.5").children(type_btns),
            ]));

            let show_trend = state.sparkline_config.show_trend;
            knob_controls.push(div().class("pt-2 border-t border-slate-800/80").child(
                button_advanced(
                    if show_trend { "Trend Indicator: ON" } else { "Trend Indicator: OFF" },
                    if show_trend { "primary" } else { "outline" },
                    "xs", false, None,
                ).on_click(move || {
                    PLAYBOOK_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.sparkline_config.show_trend = !st.sparkline_config.show_trend;
                    });
                    render_app("chart-studio");
                }),
            ));
        }
    }

    // Color Engine Panel
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
                    if is_sel { "font-weight: 700; transform: scale(1.03); box-shadow: 0 2px 10px rgba(99, 102, 241, 0.35);" } else { "opacity: 0.85;" }
                ))
                .on_click(move || {
                    PLAYBOOK_STATE.with(|s| {
                        let mut st = s.borrow_mut();
                        st.active_palette_id = pal_id.to_string();
                        st.custom_hex = pal_primary.to_string();
                    });
                    render_app("chart-studio");
                })
        );
    }

    let direct_colors = [
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
                    PLAYBOOK_STATE.with(|s| {
                        s.borrow_mut().custom_hex = hex_str.clone();
                    });
                    render_app("chart-studio");
                })
        );
    }

    let left_knobs_column = div().class("space-y-4").children(vec![
        card_advanced(
            "Props & Knobs Inspector",
            "Real-time reactive control panel: mutate props and trigger zero-latency Wasm re-renders",
            "sliders",
            "glass",
            false,
            "p-5",
        ).child(div().class("space-y-4").children(knob_controls)),

        card_advanced(
            "Pervasive Color Engine",
            "Harmonic palettes & dynamic tint overrides applied to all charts",
            "palette",
            "glass",
            false,
            "p-5",
        ).child(
            div().class("space-y-3.5").children(vec![
                div().class("space-y-1.5").children(vec![
                    span().class("text-xs font-bold text-slate-300").text("Curated Color Schemes:"),
                    div().class("flex flex-wrap gap-1.5").children(palette_buttons),
                ]),
                div().class("space-y-1.5 pt-2 border-t border-slate-800/80").children(vec![
                    span().class("text-xs font-bold text-slate-300").text("Direct Tint Override:"),
                    div().class("flex flex-wrap gap-1.5").children(direct_color_buttons),
                ]),
            ])
        ),
    ]);

    // ------------------------------------------------------------------------
    // C. Center Column: SVG Canvas Preview + Generators + Data Table (Col 6/12)
    // ------------------------------------------------------------------------
    let chart_preview = match state.active_kind {
        ComponentKind::Line => {
            let mut opts = LineChartOptions::default();
            opts.curve = state.line_config.curve;
            opts.stroke_width = state.line_config.stroke_width;
            opts.show_axes = state.line_config.show_axes;
            opts.show_dots = state.line_config.show_dots;
            opts.stroke_color = Some(current_palette.primary.into());
            opts.gradient_start = Some(current_palette.primary.into());
            opts.gradient_end = Some(current_palette.secondary.into());
            let pts = state.line_config.points.iter().map(|p| Point { x: p.0, y: p.1 }).collect();
            div().class("w-full flex items-center justify-center p-3 overflow-hidden")
                .child(line_chart_styled("studio-line-preview", pts, 700, 310, &opts))
        },
        ComponentKind::Area => {
            let mut opts = AreaChartOptions::default();
            opts.curve = state.area_config.curve;
            opts.stroke_width = state.area_config.stroke_width;
            opts.fill_opacity = state.area_config.fill_opacity;
            opts.stroke_color = Some(current_palette.primary.into());
            opts.fill_color = Some(current_palette.secondary.into());
            let max_v = state.area_config.points.iter().map(|p| p.1).fold(1.0, f64::max);
            let len = state.area_config.points.len().max(2) - 1;
            let pts = state.area_config.points.iter().enumerate().map(|(i, p)| {
                let x = 40.0 + (i as f64 / len as f64) * 620.0;
                let y = 290.0 - (p.1 / max_v) * 230.0;
                Point { x, y }
            }).collect();
            div().class("w-full flex items-center justify-center p-3 overflow-hidden")
                .child(area_chart_styled(pts, 700, 310, &opts))
        },
        ComponentKind::Bar => {
            let mut opts = BarChartOptions::default();
            opts.bar_padding = state.bar_config.padding;
            opts.rounded_corners = state.bar_config.rounded_corners;
            opts.show_axes = state.bar_config.show_axes;
            let colors = current_palette.colors;
            let data: Vec<BarDatum> = state.bar_config.items.iter().enumerate().map(|(i, it)| {
                let c = it.color.clone().unwrap_or_else(|| colors[i % colors.len()].to_string());
                BarDatum {
                    label: it.label.clone(),
                    value: it.value,
                    color: c,
                }
            }).collect();
            div().class("w-full flex items-center justify-center p-3 overflow-hidden")
                .child(bar_chart_styled("studio-bar-preview", data, 700, 310, &opts))
        },
        ComponentKind::Donut => {
            let colors = current_palette.colors;
            let pie_data: Vec<PieDatum> = state.donut_config.segments.iter().enumerate().map(|(i, seg)| {
                let c = seg.color.clone().unwrap_or_else(|| colors[i % colors.len()].to_string());
                PieDatum {
                    label: seg.label.clone(),
                    value: seg.value,
                    color: c,
                }
            }).collect();
            let outer_r = 120.0;
            let inner_r = outer_r * (state.donut_config.inner_radius_pct / 100.0);
            let avg = state.donut_config.segments.iter().map(|s| s.value).sum::<f64>()
                / state.donut_config.segments.len().max(1) as f64;
            div().class("w-full flex items-center justify-center p-4")
                .child(donut_chart(pie_data, outer_r, inner_r, "System Score", &format!("{:.1}%", avg)))
        },
        ComponentKind::Radar => {
            let radar_data: Vec<RadarDatum> = state.radar_config.dimensions.iter().map(|d| {
                RadarDatum {
                    attribute: d.label.clone(),
                    value: d.value.clamp(0.0, 100.0),
                }
            }).collect();
            div().class("w-full flex items-center justify-center p-4")
                .child(radar_chart(radar_data, 115.0))
        },
        ComponentKind::Gauge => {
            let cfg = &state.gauge_config;
            let mut opts = GaugeChartOptions::default();
            opts.warning_threshold = cfg.warning_thresh;
            opts.critical_threshold = cfg.critical_thresh;
            opts.custom_color = Some(state.custom_hex.clone());

            div().class("w-full flex flex-col items-center justify-center p-4")
                .children(vec![
                    gauge_chart_styled(
                        "studio-gauge-preview",
                        cfg.value,
                        cfg.min,
                        cfg.max,
                        "Telemetry Speedometer",
                        &cfg.unit,
                        340,
                        190,
                        &opts,
                    ),
                    div().class("mt-2 text-xs text-slate-400 font-mono")
                        .text(&format!("Concentric Arc Geometry | Current: {:.1}% | Target: {:.1}%", cfg.value, cfg.target)),
                ])
        },
        ComponentKind::Sparkline => {
            let first = state.sparkline_config.ticks.first().copied().unwrap_or(1.0);
            let last = state.sparkline_config.ticks.last().copied().unwrap_or(1.0);
            let delta = if first > 0.0 { ((last - first) / first) * 100.0 } else { 0.0 };
            let is_pos = delta >= 0.0;

            div().class("w-full flex flex-col items-center justify-center p-6 gap-4")
                .children(vec![
                    if state.sparkline_config.show_trend {
                        div().class("flex items-center gap-2 p-2 px-3 rounded-lg bg-slate-900/90 border border-slate-800")
                            .children(vec![
                                span().class("text-xs font-semibold text-slate-400").text("Trend Velocity:"),
                                span().attr("style", if is_pos { "color: #34d399; font-weight: 700; font-family: monospace;" } else { "color: #f87171; font-weight: 700; font-family: monospace;" })
                                    .text(&format!("{} {:.1}%", if is_pos { "▲ +" } else { "▼ " }, delta)),
                            ])
                    } else {
                        div()
                    },
                    div().class("grid grid-cols-1 sm:grid-cols-3 gap-4 w-full")
                        .children(vec![
                            div().class("p-3 rounded-xl bg-slate-900/70 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-emerald-400 font-bold").text("Area Sparkline"),
                                    sparkline_chart(state.sparkline_config.ticks.clone(), 160, 48, SparklineType::Area, current_palette.primary),
                                ]),
                            div().class("p-3 rounded-xl bg-slate-900/70 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-sky-400 font-bold").text("Line Sparkline"),
                                    sparkline_chart(state.sparkline_config.ticks.clone(), 160, 48, SparklineType::Line, current_palette.secondary),
                                ]),
                            div().class("p-3 rounded-xl bg-slate-900/70 border border-slate-800/80 flex flex-col items-center gap-2")
                                .children(vec![
                                    span().class("text-xs text-purple-400 font-bold").text("Bar Sparkline"),
                                    sparkline_chart(state.sparkline_config.ticks.clone(), 160, 48, SparklineType::Bar, current_palette.accent),
                                ]),
                        ])
                ])
        }
    };

    // Algorithmic Data Generator Buttons (Milestone 2)
    let algo_generator_bar = div()
        .class("flex flex-wrap items-center justify-between gap-2 p-2.5 rounded-xl bg-slate-950/70 border border-slate-800/80")
        .children(vec![
            div().class("flex items-center gap-1.5")
                .children(vec![
                    span().class("text-xs font-bold text-slate-400 uppercase tracking-wider mr-1").text("Simulate:"),
                    button_advanced("🎲 Brownian Walk", "glass", "xs", false, None).on_click(|| {
                        let data = generate_brownian_motion(10, 45.0, 5.5, 1.2);
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            match st.active_kind {
                                ComponentKind::Line => {
                                    st.line_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Area => {
                                    st.area_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Bar => {
                                    for (i, item) in st.bar_config.items.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { item.value = v; }
                                    }
                                },
                                ComponentKind::Donut => {
                                    for (i, seg) in st.donut_config.segments.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { seg.value = v; }
                                    }
                                },
                                ComponentKind::Radar => {
                                    for (i, dim) in st.radar_config.dimensions.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { dim.value = v.clamp(10.0, 100.0); }
                                    }
                                },
                                ComponentKind::Gauge => {
                                    st.gauge_config.value = *data.last().unwrap_or(&50.0);
                                },
                                ComponentKind::Sparkline => {
                                    st.sparkline_config.ticks = data;
                                }
                            }
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("🌊 Harmonic Wave", "glass", "xs", false, None).on_click(|| {
                        let data = generate_harmonic_wave(10, 26.0, 6.0, 2.5);
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            match st.active_kind {
                                ComponentKind::Line => {
                                    st.line_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Area => {
                                    st.area_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Bar => {
                                    for (i, item) in st.bar_config.items.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { item.value = v; }
                                    }
                                },
                                ComponentKind::Donut => {
                                    for (i, seg) in st.donut_config.segments.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { seg.value = v; }
                                    }
                                },
                                ComponentKind::Radar => {
                                    for (i, dim) in st.radar_config.dimensions.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { dim.value = v.clamp(10.0, 100.0); }
                                    }
                                },
                                ComponentKind::Gauge => {
                                    st.gauge_config.value = *data.last().unwrap_or(&50.0);
                                },
                                ComponentKind::Sparkline => {
                                    st.sparkline_config.ticks = data;
                                }
                            }
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("⚡ Traffic Spikes", "glass", "xs", false, None).on_click(|| {
                        let data = generate_traffic_spikes(10, 32.0, 0.30, 2.0);
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            match st.active_kind {
                                ComponentKind::Line => {
                                    st.line_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Area => {
                                    st.area_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Bar => {
                                    for (i, item) in st.bar_config.items.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { item.value = v; }
                                    }
                                },
                                ComponentKind::Donut => {
                                    for (i, seg) in st.donut_config.segments.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { seg.value = v; }
                                    }
                                },
                                ComponentKind::Radar => {
                                    for (i, dim) in st.radar_config.dimensions.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { dim.value = v.clamp(10.0, 100.0); }
                                    }
                                },
                                ComponentKind::Gauge => {
                                    st.gauge_config.value = *data.last().unwrap_or(&50.0);
                                },
                                ComponentKind::Sparkline => {
                                    st.sparkline_config.ticks = data;
                                }
                            }
                        });
                        render_app("chart-studio");
                    }),
                    button_advanced("📊 Normal Bell", "glass", "xs", false, None).on_click(|| {
                        let data = generate_normal_bell(10, 50.0, 44.0);
                        PLAYBOOK_STATE.with(|s| {
                            let mut st = s.borrow_mut();
                            match st.active_kind {
                                ComponentKind::Line => {
                                    st.line_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Area => {
                                    st.area_config.points = data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect();
                                },
                                ComponentKind::Bar => {
                                    for (i, item) in st.bar_config.items.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { item.value = v; }
                                    }
                                },
                                ComponentKind::Donut => {
                                    for (i, seg) in st.donut_config.segments.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { seg.value = v; }
                                    }
                                },
                                ComponentKind::Radar => {
                                    for (i, dim) in st.radar_config.dimensions.iter_mut().enumerate() {
                                        if let Some(&v) = data.get(i) { dim.value = v.clamp(10.0, 100.0); }
                                    }
                                },
                                ComponentKind::Gauge => {
                                    st.gauge_config.value = *data.last().unwrap_or(&50.0);
                                },
                                ComponentKind::Sparkline => {
                                    st.sparkline_config.ticks = data;
                                }
                            }
                        });
                        render_app("chart-studio");
                    }),
                ]),
            button_advanced("🔄 Reset Defaults", "outline", "xs", false, None).on_click(|| {
                PLAYBOOK_STATE.with(|s| {
                    let mut st = s.borrow_mut();
                    match st.active_kind {
                        ComponentKind::Line => st.line_config = LineConfig::default(),
                        ComponentKind::Area => st.area_config = AreaConfig::default(),
                        ComponentKind::Bar => st.bar_config = BarConfig::default(),
                        ComponentKind::Donut => st.donut_config = DonutConfig::default(),
                        ComponentKind::Radar => st.radar_config = RadarConfig::default(),
                        ComponentKind::Gauge => st.gauge_config = GaugeConfig::default(),
                        ComponentKind::Sparkline => st.sparkline_config = SparklineConfig::default(),
                    }
                });
                render_app("chart-studio");
            }),
        ]);

    // Interactive Data Table & Individual Points Inspector (Milestone 4)
    let mut table_rows = Vec::new();

    match state.active_kind {
        ComponentKind::Line => {
            for (idx, pt) in state.line_config.points.iter().enumerate() {
                let idx_cap = idx;
                let y_val = pt.1;
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[140px]").children(vec![
                    span().class("text-[11px] font-mono text-slate-400 font-bold").text(&format!("P{}", idx + 1)),
                    span().class("text-xs font-mono font-bold text-emerald-400").text(&format!("{:.1}", y_val)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.line_config.points.len() {
                                        st.line_config.points[idx_cap].1 = (st.line_config.points[idx_cap].1 + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.line_config.points.len() {
                                        st.line_config.points[idx_cap].1 = (st.line_config.points[idx_cap].1 - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("×", "danger", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if st.line_config.points.len() > 3 && idx_cap < st.line_config.points.len() {
                                        st.line_config.points.remove(idx_cap);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        },
        ComponentKind::Area => {
            for (idx, pt) in state.area_config.points.iter().enumerate() {
                let idx_cap = idx;
                let y_val = pt.1;
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[140px]").children(vec![
                    span().class("text-[11px] font-mono text-slate-400 font-bold").text(&format!("A{}", idx + 1)),
                    span().class("text-xs font-mono font-bold text-sky-400").text(&format!("{:.1}", y_val)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.area_config.points.len() {
                                        st.area_config.points[idx_cap].1 = (st.area_config.points[idx_cap].1 + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.area_config.points.len() {
                                        st.area_config.points[idx_cap].1 = (st.area_config.points[idx_cap].1 - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("×", "danger", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if st.area_config.points.len() > 3 && idx_cap < st.area_config.points.len() {
                                        st.area_config.points.remove(idx_cap);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        },
        ComponentKind::Bar => {
            for (idx, item) in state.bar_config.items.iter().enumerate() {
                let idx_cap = idx;
                let val = item.value;
                let label = item.label.clone();
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[160px]").children(vec![
                    span().class("text-[11px] font-bold text-slate-300 truncate max-w-[70px]").text(&label),
                    span().class("text-xs font-mono font-bold text-amber-400").text(&format!("{:.1}", val)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.bar_config.items.len() {
                                        st.bar_config.items[idx_cap].value = (st.bar_config.items[idx_cap].value + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.bar_config.items.len() {
                                        st.bar_config.items[idx_cap].value = (st.bar_config.items[idx_cap].value - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("×", "danger", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if st.bar_config.items.len() > 2 && idx_cap < st.bar_config.items.len() {
                                        st.bar_config.items.remove(idx_cap);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        },
        ComponentKind::Donut => {
            for (idx, seg) in state.donut_config.segments.iter().enumerate() {
                let idx_cap = idx;
                let val = seg.value;
                let label = seg.label.clone();
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[160px]").children(vec![
                    span().class("text-[11px] font-bold text-slate-300 truncate max-w-[70px]").text(&label),
                    span().class("text-xs font-mono font-bold text-purple-400").text(&format!("{:.1}", val)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.donut_config.segments.len() {
                                        st.donut_config.segments[idx_cap].value = (st.donut_config.segments[idx_cap].value + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.donut_config.segments.len() {
                                        st.donut_config.segments[idx_cap].value = (st.donut_config.segments[idx_cap].value - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("×", "danger", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if st.donut_config.segments.len() > 2 && idx_cap < st.donut_config.segments.len() {
                                        st.donut_config.segments.remove(idx_cap);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        },
        ComponentKind::Radar => {
            for (idx, dim) in state.radar_config.dimensions.iter().enumerate() {
                let idx_cap = idx;
                let val = dim.value;
                let label = dim.label.clone();
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[160px]").children(vec![
                    span().class("text-[11px] font-bold text-slate-300 truncate max-w-[70px]").text(&label),
                    span().class("text-xs font-mono font-bold text-emerald-400").text(&format!("{:.1}", val)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.radar_config.dimensions.len() {
                                        st.radar_config.dimensions[idx_cap].value = (st.radar_config.dimensions[idx_cap].value + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.radar_config.dimensions.len() {
                                        st.radar_config.dimensions[idx_cap].value = (st.radar_config.dimensions[idx_cap].value - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        },
        ComponentKind::Gauge => {
            let val = state.gauge_config.value;
            let warn = state.gauge_config.warning_thresh;
            let crit = state.gauge_config.critical_thresh;
            table_rows.push(div().class("flex items-center justify-between p-3 rounded-lg bg-slate-950/70 border border-slate-800/80 w-full").children(vec![
                span().class("text-xs text-slate-300 font-bold").text("Telemetry Speedometer Target:"),
                span().class("text-xs font-mono font-bold text-emerald-400").text(&format!("{:.1}% (Safe: <{:.0}%, Warning: <{:.0}%)", val, warn * 100.0, crit * 100.0)),
            ]));
        },
        ComponentKind::Sparkline => {
            for (idx, &tick) in state.sparkline_config.ticks.iter().enumerate() {
                let idx_cap = idx;
                table_rows.push(div().class("flex items-center justify-between p-2 rounded-lg bg-slate-950/70 border border-slate-800/80 gap-2 min-w-[130px]").children(vec![
                    span().class("text-[11px] font-mono text-slate-400 font-bold").text(&format!("T{}", idx + 1)),
                    span().class("text-xs font-mono font-bold text-emerald-400").text(&format!("{:.1}", tick)),
                    div().class("flex items-center gap-1").children(vec![
                        button_advanced("+5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.sparkline_config.ticks.len() {
                                        st.sparkline_config.ticks[idx_cap] = (st.sparkline_config.ticks[idx_cap] + 5.0).min(100.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                        button_advanced("-5", "glass", "xs", false, None)
                            .attr("style", "padding: 2px 6px; font-size: 10px; font-weight: 700;")
                            .on_click(move || {
                                PLAYBOOK_STATE.with(|s| {
                                    let mut st = s.borrow_mut();
                                    if idx_cap < st.sparkline_config.ticks.len() {
                                        st.sparkline_config.ticks[idx_cap] = (st.sparkline_config.ticks[idx_cap] - 5.0).max(5.0);
                                    }
                                });
                                render_app("chart-studio");
                            }),
                    ]),
                ]));
            }
        }
    }

    // Add Record Button for active kind
    let add_record_btn = button_advanced("➕ Add Next Point", "primary", "xs", false, None)
        .on_click(|| {
            PLAYBOOK_STATE.with(|s| {
                let mut st = s.borrow_mut();
                match st.active_kind {
                    ComponentKind::Line => {
                        let next_x = st.line_config.points.len() as f64;
                        let last_y = st.line_config.points.last().map(|p| p.1).unwrap_or(50.0);
                        let new_y = (last_y + 8.0).min(98.0);
                        st.line_config.points.push((next_x, new_y));
                    },
                    ComponentKind::Area => {
                        let next_x = st.area_config.points.len() as f64;
                        let last_y = st.area_config.points.last().map(|p| p.1).unwrap_or(50.0);
                        let new_y = (last_y + 6.0).min(98.0);
                        st.area_config.points.push((next_x, new_y));
                    },
                    ComponentKind::Bar => {
                        let count = st.bar_config.items.len();
                        st.bar_config.items.push(BarItem {
                            label: format!("Worker-{}", count + 1),
                            value: 70.0,
                            color: None,
                        });
                    },
                    ComponentKind::Donut => {
                        let count = st.donut_config.segments.len();
                        st.donut_config.segments.push(DonutItem {
                            label: format!("Segment-{}", count + 1),
                            value: 20.0,
                            color: None,
                        });
                    },
                    ComponentKind::Radar => {
                        let count = st.radar_config.dimensions.len();
                        st.radar_config.dimensions.push(RadarAxis {
                            label: format!("Metric-{}", count + 1),
                            value: 80.0,
                        });
                    },
                    ComponentKind::Gauge => {},
                    ComponentKind::Sparkline => {
                        let last = st.sparkline_config.ticks.last().copied().unwrap_or(50.0);
                        st.sparkline_config.ticks.push((last + 5.0).min(100.0));
                    }
                }
            });
            render_app("chart-studio");
        });

    let data_table_card = card_advanced(
        "Interactive Data Points Inspector",
        "Direct numeric modification: increment, decrement, and delete individual points with instant Wasm re-rendering",
        "grid",
        "glass",
        false,
        "p-4",
    ).child(
        div().class("space-y-3").children(vec![
            div().class("flex items-center justify-between pb-2 border-b border-slate-800/80")
                .children(vec![
                    span().class("text-xs font-bold text-slate-300 uppercase tracking-wider").text("Active Points:"),
                    add_record_btn,
                ]),
            div().class("flex flex-wrap gap-2 max-h-[180px] overflow-y-auto pr-1").children(table_rows),
        ])
    );

    let center_canvas_column = div().class("space-y-4").children(vec![
        card_advanced(
            "Interactive Vector SVG Canvas",
            "Zero rasterization: pure scalable SVG geometry calculated in WebAssembly linear memory",
            "activity",
            "glass",
            false,
            "p-5",
        ).child(
            div().class("space-y-4").children(vec![
                div().class("flex flex-wrap items-center justify-between gap-3 pb-3 border-b border-slate-800/80")
                    .children(vec![
                        div().class("flex items-center gap-2")
                            .children(vec![
                                badge("Pure Rust Wasm SVG", "success"),
                                badge(&format!("Palette: {}", current_palette.name), "info"),
                            ]),
                        span().class("text-xs text-slate-400 font-mono").text("60 FPS • Zero JS Engine"),
                    ]),
                chart_preview,
                algo_generator_bar,
            ])
        ),
        data_table_card,
    ]);

    // ------------------------------------------------------------------------
    // D. Right Column: Live Synchronized Rust Wasm Code Snippet (Col 3/12)
    // ------------------------------------------------------------------------
    let rust_code = generate_rust_component_code(&state);
    let rust_code_copy = rust_code.clone();

    let right_code_column = div().class("space-y-4").child(
        card_advanced(
            "Live Code Synchronizer",
            "Generates pure Rust Wasm DomBuilder code mirroring props & dataset",
            "code",
            "glass",
            false,
            "p-5",
        ).child(
            div().class("space-y-3.5").children(vec![
                div().class("flex items-center justify-between pb-2 border-b border-slate-800/80")
                    .children(vec![
                        span().class("text-xs font-semibold text-slate-300").text("Native Rust Vector Snippet:"),
                        button_advanced("📋 Copy Snippet", "primary", "xs", false, Some("download"))
                            .on_click(move || {
                                copy_to_clipboard(&rust_code_copy, "✅ Rust Chart snippet copied to clipboard!");
                            }),
                    ]),
                div().class("p-3.5 rounded-xl bg-slate-950 border border-slate-800 text-[11px] font-mono text-emerald-300 overflow-x-auto leading-relaxed whitespace-pre max-h-[640px]")
                    .text(&rust_code),
            ])
        )
    );

    // ------------------------------------------------------------------------
    // E. Assemble Responsive 3-Zone Layout
    // ------------------------------------------------------------------------
    container()
        .attr("style", "max-width: 1600px; margin: 0 auto; padding: 2rem 1.5rem; display: flex; flex-direction: column; gap: 24px;")
        .children(vec![
            div().class("pb-3 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-4")
                .children(vec![
                    div().children(vec![
                        h2().class("text-2xl font-black text-white tracking-tight").text("📈 Ferrox Front Component Playbook"),
                        p().class("text-xs text-slate-400 mt-1")
                            .text("High-density interactive workbench: typed data models, live knobs inspector, algorithmic generators, and synchronized Rust code."),
                    ]),
                    badge("Interactive Workbench 2.0", "primary"),
                ]),
            segmented_toolbar,
            div().class("grid grid-cols-1 lg:grid-cols-12 gap-5 w-full")
                .children(vec![
                    div().class("lg:col-span-3").child(left_knobs_column),
                    div().class("lg:col-span-6").child(center_canvas_column),
                    div().class("lg:col-span-3").child(right_code_column),
                ]),
        ])
}
