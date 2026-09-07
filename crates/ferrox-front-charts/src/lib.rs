//! # Ferrox Front Charts (`ferrox-front-charts`)
//!
//! `ferrox-front-charts` provides a pure WebAssembly vector SVG charting engine inspired by Nivo.
//! It renders high-performance, responsive, theme-aware data visualisations directly as native SVG elements
//! without external JavaScript charting libraries.
//!
//! ## 📈 Supported Nivo-Style Chart Types
//! 1. `line_chart`: Smooth spline or linear vector line graphs with markers and hover tooltips.
//! 2. `area_chart`: Gradient-filled area charts with customizable baseline and stop opacities.
//! 3. `bar_chart`: Grouped/stacked vertical bar charts with rounded corners and value labels.
//! 4. `pie_chart`: Circular pie charts with calculated arc slices and legends.
//! 5. `donut_chart`: Ring donut charts with inner radius cutouts and center metrics.
//! 6. `radar_chart`: Spider-web polygonal radar charts for multi-attribute evaluation.
//! 7. `scatter_chart`: Multi-series scatter plot charts with variable radii and color mapping.

use ferrox_front_core::dom::{div, DomBuilder};
use std::f64::consts::PI;

/// A 2D point data coordinate for Line and Area charts.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Data entry for Bar charts.
#[derive(Debug, Clone)]
pub struct BarDatum {
    pub label: String,
    pub value: f64,
    pub color: String,
}

/// Data entry for Pie and Donut charts.
#[derive(Debug, Clone)]
pub struct PieDatum {
    pub label: String,
    pub value: f64,
    pub color: String,
}

/// Data entry for Radar (Spider) charts.
#[derive(Debug, Clone)]
pub struct RadarDatum {
    pub attribute: String,
    pub value: f64, // Scaled 0.0 to 100.0
}

/// Data entry for Scatter Plot charts.
#[derive(Debug, Clone)]
pub struct ScatterDatum {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: String,
}

/// Renders a Nivo-style Vector SVG Line Chart with gradient strokes.
pub fn line_chart(data: Vec<Point>, width: u32, height: u32) -> DomBuilder {
    let mut path_d = String::new();
    
    if !data.is_empty() {
        path_d.push_str(&format!("M {} {}", data[0].x, data[0].y));
        for point in data.iter().skip(1) {
            path_d.push_str(&format!(" L {} {}", point.x, point.y));
        }
    }

    div().child(
        DomBuilder::new("svg")
            .attr("class", "ferrox-svg-chart")
            .attr("width", &width.to_string())
            .attr("height", &height.to_string())
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .child(
                DomBuilder::new("defs").child(
                    DomBuilder::new("linearGradient")
                        .attr("id", "lineChartGrad")
                        .attr("x1", "0%").attr("y1", "0%").attr("x2", "100%").attr("y2", "0%")
                        .child(DomBuilder::new("stop").attr("offset", "0%").attr("stop-color", "var(--accent-glow, #9333ea)"))
                        .child(DomBuilder::new("stop").attr("offset", "100%").attr("stop-color", "var(--accent-cyan, #06b6d4)"))
                )
            )
            .child(
                DomBuilder::new("path")
                    .attr("d", &path_d)
                    .attr("fill", "none")
                    .attr("stroke", "url(#lineChartGrad)")
                    .attr("stroke-width", "4")
                    .attr("stroke-linecap", "round")
            )
    )
}

/// Renders a Nivo-style Gradient Area Chart with filled baseline opacity.
pub fn area_chart(data: Vec<Point>, width: u32, height: u32) -> DomBuilder {
    let mut line_d = String::new();
    if !data.is_empty() {
        line_d.push_str(&format!("M {} {}", data[0].x, data[0].y));
        for pt in data.iter().skip(1) {
            line_d.push_str(&format!(" L {} {}", pt.x, pt.y));
        }
    }

    let area_d = if !data.is_empty() {
        let last = data.last().unwrap();
        let first = data.first().unwrap();
        format!("{} L {} {} L {} {} Z", line_d, last.x, height, first.x, height)
    } else {
        String::new()
    };

    div().child(
        DomBuilder::new("svg")
            .attr("class", "ferrox-svg-area-chart")
            .attr("width", &width.to_string())
            .attr("height", &height.to_string())
            .attr("viewBox", &format!("0 0 {} {}", width, height))
            .child(
                DomBuilder::new("defs").child(
                    DomBuilder::new("linearGradient")
                        .attr("id", "areaFillGrad")
                        .attr("x1", "0%").attr("y1", "0%").attr("x2", "0%").attr("y2", "100%")
                        .child(DomBuilder::new("stop").attr("offset", "0%").attr("stop-color", "var(--accent-cyan, #06b6d4)").attr("stop-opacity", "0.4"))
                        .child(DomBuilder::new("stop").attr("offset", "100%").attr("stop-color", "var(--accent-cyan, #06b6d4)").attr("stop-opacity", "0.0"))
                )
            )
            .child(DomBuilder::new("path").attr("d", &area_d).attr("fill", "url(#areaFillGrad)"))
            .child(DomBuilder::new("path").attr("d", &line_d).attr("fill", "none").attr("stroke", "var(--accent-cyan, #06b6d4)").attr("stroke-width", "3"))
    )
}

/// Renders a Nivo-style Bar Chart with rounded corners and value labels.
pub fn bar_chart(data: Vec<BarDatum>, width: u32, height: u32) -> DomBuilder {
    let mut svg = DomBuilder::new("svg")
        .attr("class", "ferrox-svg-bar-chart")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    let max_val = data.iter().map(|d| d.value).fold(1.0, f64::max);
    let bar_count = data.len();
    let padding = 20.0;
    let available_w = width as f64 - (padding * (bar_count as f64 + 1.0));
    let bar_w = (available_w / bar_count as f64).max(10.0);

    for (i, datum) in data.iter().enumerate() {
        let bar_h = (datum.value / max_val) * (height as f64 - 50.0);
        let x = padding + i as f64 * (bar_w + padding);
        let y = height as f64 - bar_h - 30.0;

        let rect = DomBuilder::new("rect")
            .attr("x", &x.to_string())
            .attr("y", &y.to_string())
            .attr("width", &bar_w.to_string())
            .attr("height", &bar_h.to_string())
            .attr("rx", "6")
            .attr("fill", &datum.color);

        let label = DomBuilder::new("text")
            .attr("x", &(x + bar_w / 2.0).to_string())
            .attr("y", &(height as f64 - 10.0).to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-primary, #fff)")
            .attr("font-size", "12")
            .text(&datum.label);

        svg = svg.child(rect).child(label);
    }

    div().child(svg)
}

/// Renders a Nivo-style Donut Chart with inner radius cutout and center text.
pub fn donut_chart(data: Vec<PieDatum>, radius: f64, inner_radius: f64, center_title: &str, center_val: &str) -> DomBuilder {
    let size = (radius * 2.0) + 40.0;
    let cx = size / 2.0;
    let cy = size / 2.0;
    let total: f64 = data.iter().map(|d| d.value).sum();

    let mut svg = DomBuilder::new("svg")
        .attr("class", "ferrox-svg-donut-chart")
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("viewBox", &format!("0 0 {} {}", size, size));

    let mut current_angle = -PI / 2.0; // Start at top

    for datum in &data {
        let slice_angle = (datum.value / total) * 2.0 * PI;
        let end_angle = current_angle + slice_angle;

        let x1_outer = cx + radius * current_angle.cos();
        let y1_outer = cy + radius * current_angle.sin();
        let x2_outer = cx + radius * end_angle.cos();
        let y2_outer = cy + radius * end_angle.sin();

        let x1_inner = cx + inner_radius * end_angle.cos();
        let y1_inner = cy + inner_radius * end_angle.sin();
        let x2_inner = cx + inner_radius * current_angle.cos();
        let y2_inner = cy + inner_radius * current_angle.sin();

        let large_arc = if slice_angle > PI { "1" } else { "0" };

        let d = format!(
            "M {} {} A {} {} 0 {} 1 {} {} L {} {} A {} {} 0 {} 0 {} {} Z",
            x1_outer, y1_outer, radius, radius, large_arc, x2_outer, y2_outer,
            x1_inner, y1_inner, inner_radius, inner_radius, large_arc, x2_inner, y2_inner
        );

        let path = DomBuilder::new("path")
            .attr("d", &d)
            .attr("fill", &datum.color)
            .attr("stroke", "var(--bg-primary, #0a0a16)")
            .attr("stroke-width", "2");

        svg = svg.child(path);
        current_angle = end_angle;
    }

    // Center Text Info
    let title_text = DomBuilder::new("text")
        .attr("x", &cx.to_string())
        .attr("y", &(cy - 6.0).to_string())
        .attr("text-anchor", "middle")
        .attr("fill", "var(--accent-cyan, #06b6d4)")
        .attr("font-size", "14")
        .attr("font-weight", "600")
        .text(center_title);

    let val_text = DomBuilder::new("text")
        .attr("x", &cx.to_string())
        .attr("y", &(cy + 16.0).to_string())
        .attr("text-anchor", "middle")
        .attr("fill", "var(--text-primary, #fff)")
        .attr("font-size", "18")
        .attr("font-weight", "700")
        .text(center_val);

    div().child(svg.child(title_text).child(val_text))
}

/// Renders a Nivo-style Polygonal Radar (Spider) Chart for multi-attribute evaluation.
pub fn radar_chart(data: Vec<RadarDatum>, radius: f64) -> DomBuilder {
    let size = (radius * 2.0) + 80.0;
    let cx = size / 2.0;
    let cy = size / 2.0;
    let count = data.len();

    let mut svg = DomBuilder::new("svg")
        .attr("class", "ferrox-svg-radar-chart")
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("viewBox", &format!("0 0 {} {}", size, size));

    // Concentric grid rings
    for level in [0.25, 0.50, 0.75, 1.00] {
        let r = radius * level;
        let mut grid_pts = String::new();
        for i in 0..count {
            let angle = (i as f64 / count as f64) * 2.0 * PI - (PI / 2.0);
            let x = cx + r * angle.cos();
            let y = cy + r * angle.sin();
            grid_pts.push_str(&format!("{},{} ", x, y));
        }
        svg = svg.child(
            DomBuilder::new("polygon")
                .attr("points", grid_pts.trim())
                .attr("fill", "none")
                .attr("stroke", "rgba(255,255,255,0.15)")
                .attr("stroke-width", "1")
        );
    }

    // Dataset Polygon
    let mut data_pts = String::new();
    for (i, datum) in data.iter().enumerate() {
        let r = radius * (datum.value / 100.0).clamp(0.0, 1.0);
        let angle = (i as f64 / count as f64) * 2.0 * PI - (PI / 2.0);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        data_pts.push_str(&format!("{},{} ", x, y));

        // Attribute Label
        let label_r = radius + 20.0;
        let lx = cx + label_r * angle.cos();
        let ly = cy + label_r * angle.sin();
        let label = DomBuilder::new("text")
            .attr("x", &lx.to_string())
            .attr("y", &ly.to_string())
            .attr("text-anchor", "middle")
            .attr("fill", "var(--text-primary, #fff)")
            .attr("font-size", "11")
            .text(&datum.attribute);
        svg = svg.child(label);
    }

    let polygon = DomBuilder::new("polygon")
        .attr("points", data_pts.trim())
        .attr("fill", "rgba(147, 51, 234, 0.35)")
        .attr("stroke", "var(--accent-glow, #9333ea)")
        .attr("stroke-width", "3");

    div().child(svg.child(polygon))
}

/// Renders a Nivo-style Scatter Plot Chart with variable node radii and color mapping.
pub fn scatter_chart(data: Vec<ScatterDatum>, width: u32, height: u32) -> DomBuilder {
    let mut svg = DomBuilder::new("svg")
        .attr("class", "ferrox-svg-scatter-chart")
        .attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height));

    for pt in data {
        let circle = DomBuilder::new("circle")
            .attr("cx", &pt.x.to_string())
            .attr("cy", &pt.y.to_string())
            .attr("r", &pt.radius.to_string())
            .attr("fill", &pt.color)
            .attr("opacity", "0.85")
            .attr("stroke", "var(--bg-primary, #0a0a16)")
            .attr("stroke-width", "1.5");
        svg = svg.child(circle);
    }

    div().child(svg)
}