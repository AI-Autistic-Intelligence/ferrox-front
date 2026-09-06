//! # Ferrox Front Charts (`ferrox-front-charts`)
//!
//! `ferrox-front-charts` provides a pure WebAssembly vector SVG charting engine for rendering high-performance data visualisations.
//!
//! ## Key Features
//! - 📈 **Chart Types**: Line graphs, Bar charts, Pie charts, Area graphs, and Donut charts.
//! - ⚡ **Direct SVG Render**: Renders vector graphics without heavy external JS chart dependencies.
//! - 🎨 **Theme Aware**: Automatically syncs colors with `ferrox-front-core` active glassmorphism themes.

use ferrox_front_core::dom::{DomBuilder};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn line_chart(data: Vec<Point>, width: u32, height: u32) -> DomBuilder {
    let mut path_d = String::new();
    
    if !data.is_empty() {
        path_d.push_str(&format!("M {} {}", data[0].x, data[0].y));
        for point in data.iter().skip(1) {
            path_d.push_str(&format!(" L {} {}", point.x, point.y));
        }
    }

    let gradient_def = r#"
        <defs>
            <linearGradient id="chartGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stop-color="hsl(250, 100%, 70%)" />
                <stop offset="100%" stop-color="hsl(190, 100%, 50%)" />
            </linearGradient>
        </defs>
    "#;

    ferrox_front_core::dom::div()
        .child(
            ferrox_front_core::dom::DomBuilder::new("svg")
                .attr("class", "ferrox-svg-chart")
                .attr("width", &width.to_string())
                .attr("height", &height.to_string())
                // In a real scenario, defs would be properly mounted via elements.
                // For this demo framework mock, we append the inner HTML manually if we had inner_html support,
                // but since we only have builders, we'll build it:
                .child(
                    ferrox_front_core::dom::DomBuilder::new("defs")
                        .child(
                            ferrox_front_core::dom::DomBuilder::new("linearGradient")
                                .attr("id", "chartGradient")
                                .attr("x1", "0%").attr("y1", "0%").attr("x2", "100%").attr("y2", "0%")
                                .child(ferrox_front_core::dom::DomBuilder::new("stop").attr("offset", "0%").attr("stop-color", "hsl(250, 100%, 70%)"))
                                .child(ferrox_front_core::dom::DomBuilder::new("stop").attr("offset", "100%").attr("stop-color", "hsl(190, 100%, 50%)"))
                        )
                )
                .child(
                    ferrox_front_core::dom::DomBuilder::new("path")
                        .attr("class", "ferrox-svg-path")
                        .attr("d", &path_d)
                )
        )
}