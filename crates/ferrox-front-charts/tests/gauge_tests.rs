use ferrox_front_charts::{gauge_arc_geometry, gauge_resolve_color, GaugeChartOptions};

#[test]
fn test_gauge_arc_geometry_large_arc_flag_is_strictly_zero() {
    let cx = 150.0;
    let cy = 120.0;
    let radius = 80.0;

    // Test across the entire domain of ratios [0.0 .. 1.0]
    let test_ratios = [0.0, 0.15, 0.33, 0.50, 0.62, 0.75, 0.88, 1.0];

    for ratio in test_ratios {
        let (path_d, _end_x, _end_y) = gauge_arc_geometry(cx, cy, radius, ratio);

        // Path format is: "M x1 y1 A rx ry 0 <large_arc> 1 x2 y2"
        // Must strictly use large-arc = 0 so the arc never bulges outwards
        assert!(
            path_d.contains(" 0 0 1 "),
            "Path for ratio {} must contain ' 0 0 1 ' but was: {}",
            ratio,
            path_d
        );
        assert!(
            !path_d.contains(" 0 1 1 "),
            "Path for ratio {} must NEVER contain ' 0 1 1 ' (large-arc = 1 bug): {}",
            ratio,
            path_d
        );
    }
}

#[test]
fn test_gauge_arc_geometry_concentric_needle_alignment() {
    let cx = 200.0;
    let cy = 160.0;
    let radius = 100.0;

    // At ratio 0.0 (left horizontal)
    let (_, x_start, y_start) = gauge_arc_geometry(cx, cy, radius, 0.0);
    assert!((x_start - 100.0).abs() < 1e-4, "Expected x=100.0, got {}", x_start);
    assert!((y_start - 160.0).abs() < 1e-4, "Expected y=160.0, got {}", y_start);

    // At ratio 0.5 (zenith / apex)
    let (_, x_apex, y_apex) = gauge_arc_geometry(cx, cy, radius, 0.5);
    assert!((x_apex - 200.0).abs() < 1e-4, "Expected x=200.0, got {}", x_apex);
    assert!((y_apex - 60.0).abs() < 1e-4, "Expected y=60.0, got {}", y_apex);

    // At ratio 1.0 (right horizontal)
    let (_, x_end, y_end) = gauge_arc_geometry(cx, cy, radius, 1.0);
    assert!((x_end - 300.0).abs() < 1e-4, "Expected x=300.0, got {}", x_end);
    assert!((y_end - 160.0).abs() < 1e-4, "Expected y=160.0, got {}", y_end);

    // For any ratio in [0, 1], the tip must always lie in the upper semicircle (y <= cy)
    // and radius distance must equal exact concentric radius
    for step in 0..=20 {
        let ratio = step as f64 / 20.0;
        let (_, x, y) = gauge_arc_geometry(cx, cy, radius, ratio);
        assert!(y <= cy + 1e-4, "Gauge arc y ({}) must stay <= cy ({}) for ratio {}", y, cy, ratio);
        let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        assert!((dist - radius).abs() < 1e-3, "Distance from center must equal radius for ratio {}", ratio);
    }
}

#[test]
fn test_gauge_resolve_color_thresholds() {
    let opts = GaugeChartOptions::default();

    // Normal zone (< 0.60)
    assert_eq!(gauge_resolve_color(0.20, &opts), "#10b981");
    assert_eq!(gauge_resolve_color(0.59, &opts), "#10b981");

    // Warning zone (0.60 .. 0.82)
    assert_eq!(gauge_resolve_color(0.60, &opts), "#f59e0b");
    assert_eq!(gauge_resolve_color(0.75, &opts), "#f59e0b");
    assert_eq!(gauge_resolve_color(0.81, &opts), "#f59e0b");

    // Critical danger zone (>= 0.82)
    assert_eq!(gauge_resolve_color(0.82, &opts), "#ef4444");
    assert_eq!(gauge_resolve_color(0.99, &opts), "#ef4444");
    assert_eq!(gauge_resolve_color(1.00, &opts), "#ef4444");
}

#[test]
fn test_gauge_resolve_color_custom_override() {
    let mut opts = GaugeChartOptions::default();
    opts.custom_color = Some("#6366f1".to_string()); // Indigo brand

    // Should return custom color regardless of ratio
    assert_eq!(gauge_resolve_color(0.10, &opts), "#6366f1");
    assert_eq!(gauge_resolve_color(0.70, &opts), "#6366f1");
    assert_eq!(gauge_resolve_color(0.95, &opts), "#6366f1");
}

#[test]
fn test_gauge_options_custom_thresholds() {
    let mut opts = GaugeChartOptions::default();
    opts.warning_threshold = 0.40;
    opts.critical_threshold = 0.70;
    opts.normal_color = "#3b82f6".to_string();
    opts.warning_color = "#eab308".to_string();
    opts.critical_color = "#dc2626".to_string();

    assert_eq!(gauge_resolve_color(0.35, &opts), "#3b82f6");
    assert_eq!(gauge_resolve_color(0.50, &opts), "#eab308");
    assert_eq!(gauge_resolve_color(0.85, &opts), "#dc2626");
}
