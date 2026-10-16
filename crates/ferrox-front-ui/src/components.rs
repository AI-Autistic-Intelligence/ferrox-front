//! # Glassmorphism & Enterprise UI Components (`ferrox-front-ui::components`)
//!
//! Provides a comprehensive component library with Shadcn/Reactstrap feature parity:
//! - Form Controls (Inputs with icons/validity, Textareas with counters, Switches, Custom Checkboxes/Radios, Selects, Sliders)
//! - Surfaces & Overlays (Modals with sizing, Drawers/Offcanvas, Accordions, Tabs, Tooltips, Popovers, Toasts, Skeletons)
//! - Navigation (Navbar, Collapsible Sidebar, Breadcrumbs, Pagination)
//! - Buttons & Button Groups (All variants, sizes, loading spinners)

use ferrox_front_core::dom::{
    div, span, button as dom_button, input as dom_input, textarea as dom_textarea,
    select as dom_select, option as dom_option, label as dom_label, nav as dom_nav,
    a as dom_a, svg, circle, DomBuilder
};
use crate::icons::Icon;

// =============================================================================
// 1. BUTTONS & BUTTON GROUPS
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Glass,
    Danger,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
}

/// Renders a Design System Button with variants: "primary", "secondary", "outline", "glass", "danger", "ghost", "link"
pub fn button(text: &str, variant: &str) -> DomBuilder {
    button_advanced(text, variant, "md", false, None)
}

/// Advanced Button supporting sizes ("xs", "sm", "md", "lg"), loading spinner, and icon prefix
pub fn button_advanced(
    text: &str,
    variant: &str,
    size: &str,
    loading: bool,
    icon_name: Option<&str>,
) -> DomBuilder {
    let (bg, color, border) = match variant {
        "primary" => ("var(--btn-bg, linear-gradient(135deg, #7e22ce, #3b82f6))", "#ffffff", "none"),
        "secondary" => ("rgba(255,255,255,0.1)", "var(--text-primary)", "var(--border-glass)"),
        "danger" => ("linear-gradient(135deg, #dc2626, #991b1b)", "#ffffff", "none"),
        "outline" => ("transparent", "var(--accent-cyan, #06b6d4)", "1px solid var(--accent-cyan, #06b6d4)"),
        "ghost" => ("transparent", "var(--text-primary)", "none"),
        "link" => ("transparent", "var(--accent-primary, #3b82f6)", "none"),
        _ => ("var(--bg-glass, rgba(255,255,255,0.06))", "var(--text-primary)", "var(--border-glass)"), // "glass"
    };

    let (padding, font_size, height) = match size {
        "xs" => ("4px 8px", "0.75rem", "24px"),
        "sm" => ("6px 12px", "0.82rem", "32px"),
        "lg" => ("12px 24px", "1.05rem", "48px"),
        _ => ("8px 16px", "0.9rem", "40px"), // "md"
    };

    let style = format!(
        "background: {}; color: {}; border: {}; padding: {}; min-height: {}; font-size: {}; border-radius: 6px; font-weight: 600; cursor: pointer; transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1); display: inline-flex; align-items: center; justify-content: center; gap: 8px; user-select: none; position: relative;",
        bg, color, border, padding, height, font_size
    );

    let mut btn = dom_button()
        .attr("class", &format!("ferrox-btn ferrox-btn-{} ferrox-btn-{}", variant, size))
        .attr("style", &style);

    if loading {
        btn = btn
            .attr("disabled", "true")
            .attr("style", &format!("{}; opacity: 0.8; pointer-events: none;", style))
            .child(micro_spinner(14))
            .child(span().text(text));
    } else {
        if let Some(ico) = icon_name {
            btn = btn.child(Icon::render(ico, 16, color));
        }
        btn = btn.child(span().text(text));
    }

    btn
}

/// Renders a segmented Button Group (e.g. timeframe selection: 1D, 1W, 1M, 1Y)
pub fn button_group(buttons: Vec<DomBuilder>) -> DomBuilder {
    let mut group = div()
        .attr("class", "ferrox-button-group")
        .attr("style", "display: inline-flex; border-radius: 8px; overflow: hidden; border: var(--border-glass); background: rgba(0,0,0,0.2); padding: 2px; gap: 2px;");

    for b in buttons {
        group = group.child(b);
    }
    group
}

// =============================================================================
// 2. FORM CONTROLS & ADVANCED INPUTS
// =============================================================================

#[derive(Debug, Clone)]
pub struct InputConfig {
    pub input_type: String,
    pub placeholder: String,
    pub value: String,
    pub prefix_icon: Option<String>,
    pub suffix_icon: Option<String>,
    pub is_valid: Option<bool>,
    pub helper_text: Option<String>,
    pub clearable: bool,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            input_type: "text".to_string(),
            placeholder: String::new(),
            value: String::new(),
            prefix_icon: None,
            suffix_icon: None,
            is_valid: None,
            helper_text: None,
            clearable: false,
        }
    }
}

/// Standard Form Input field
pub fn input(input_type: &str, placeholder: &str) -> DomBuilder {
    dom_input()
        .attr("type", input_type)
        .attr("placeholder", placeholder)
        .attr("class", "ferrox-input")
        .attr("style", "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-surface, rgba(255,255,255,0.04)); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; transition: border-color 0.2s, box-shadow 0.2s;")
}

/// Advanced Input supporting prefix/suffix icons, validity states (is-valid, is-invalid), and helper text
pub fn input_advanced(config: InputConfig) -> DomBuilder {
    let border_color = match config.is_valid {
        Some(true) => "#10b981",
        Some(false) => "#ef4444",
        None => "rgba(255,255,255,0.15)",
    };

    let pl = if config.prefix_icon.is_some() { "38px" } else { "12px" };
    let pr = if config.suffix_icon.is_some() || config.clearable { "38px" } else { "12px" };

    let input_style = format!(
        "width: 100%; padding: 10px {} 10px {}; border-radius: 6px; background: var(--bg-surface, rgba(0,0,0,0.25)); color: var(--text-primary); border: 1px solid {}; font-size: 0.9rem; outline: none; transition: all 0.2s;",
        pr, pl, border_color
    );

    let mut wrapper = div().attr("style", "position: relative; width: 100%; display: flex; flex-direction: column; gap: 4px;");
    let mut field_container = div().attr("style", "position: relative; width: 100%; display: flex; align-items: center;");

    if let Some(ref p_icon) = config.prefix_icon {
        let p_el = span()
            .attr("style", "position: absolute; left: 12px; top: 50%; transform: translateY(-50%); pointer-events: none; opacity: 0.7;")
            .child(Icon::render(p_icon, 16, "var(--text-secondary, #94a3b8)"));
        field_container = field_container.child(p_el);
    }

    let mut field = dom_input()
        .attr("type", &config.input_type)
        .attr("placeholder", &config.placeholder)
        .attr("class", &format!("ferrox-input-adv {}", match config.is_valid {
            Some(true) => "is-valid",
            Some(false) => "is-invalid",
            None => "",
        }))
        .attr("style", &input_style);

    if !config.value.is_empty() {
        field = field.attr("value", &config.value);
    }

    field_container = field_container.child(field);

    if let Some(ref s_icon) = config.suffix_icon {
        let s_el = span()
            .attr("style", "position: absolute; right: 12px; top: 50%; transform: translateY(-50%); opacity: 0.7;")
            .child(Icon::render(s_icon, 16, "var(--text-secondary, #94a3b8)"));
        field_container = field_container.child(s_el);
    }

    wrapper = wrapper.child(field_container);

    if let Some(helper) = config.helper_text {
        let color = match config.is_valid {
            Some(true) => "#10b981",
            Some(false) => "#ef4444",
            None => "var(--text-secondary, #94a3b8)",
        };
        wrapper = wrapper.child(span().attr("style", &format!("font-size: 0.75rem; color: {}; margin-left: 2px;", color)).text(&helper));
    }

    wrapper
}

/// Advanced Textarea with character counter and dynamic expansion
pub fn textarea_advanced(id: &str, placeholder: &str, max_chars: usize, value: &str) -> DomBuilder {
    let char_count = value.chars().count();
    let style = "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-surface, rgba(0,0,0,0.25)); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; font-family: inherit; resize: vertical; min-height: 80px;";

    div()
        .attr("class", "ferrox-textarea-wrapper")
        .attr("style", "width: 100%; display: flex; flex-direction: column; gap: 4px;")
        .child(
            dom_textarea()
                .attr("id", id)
                .attr("placeholder", placeholder)
                .attr("maxlength", &max_chars.to_string())
                .attr("style", style)
                .text(value)
        )
        .child(
            div()
                .attr("style", "display: flex; justify-content: flex-end; font-size: 0.75rem; opacity: 0.6;")
                .text(&format!("{}/{}", char_count, max_chars))
        )
}

/// Animated Switch Toggle with smooth CSS transition
pub fn switch_toggle(id: &str, checked: bool, label_text: &str) -> DomBuilder {
    let bg = if checked { "var(--accent-primary, #3b82f6)" } else { "rgba(255,255,255,0.2)" };
    let translate = if checked { "transform: translateX(18px);" } else { "transform: translateX(0px);" };

    let track = div()
        .attr("id", id)
        .attr("class", "ferrox-switch-track")
        .attr("style", &format!("width: 40px; height: 22px; border-radius: 9999px; background: {}; position: relative; transition: background 0.3s cubic-bezier(0.4,0,0.2,1); padding: 2px; cursor: pointer; flex-shrink: 0;", bg))
        .child(
            div()
                .attr("class", "ferrox-switch-thumb")
                .attr("style", &format!("width: 18px; height: 18px; border-radius: 50%; background: #ffffff; box-shadow: 0 1px 3px rgba(0,0,0,0.3); transition: transform 0.3s cubic-bezier(0.4,0,0.2,1); {}", translate))
        );

    div()
        .attr("class", "ferrox-switch-wrapper")
        .attr("style", "display: inline-flex; align-items: center; gap: 10px; cursor: pointer; user-select: none;")
        .child(track)
        .child(span().attr("style", "font-size: 0.88rem; font-weight: 500;").text(label_text))
}

/// Custom Styled Checkbox with SVG indicator and indeterminate support
pub fn checkbox_advanced(id: &str, label_text: &str, checked: bool, indeterminate: bool) -> DomBuilder {
    let bg = if checked || indeterminate { "var(--accent-primary, #3b82f6)" } else { "rgba(255,255,255,0.1)" };
    let border = if checked || indeterminate { "none" } else { "1px solid rgba(255,255,255,0.3)" };

    let mut box_indicator = div()
        .attr("style", &format!("width: 18px; height: 18px; border-radius: 4px; background: {}; border: {}; display: inline-flex; align-items: center; justify-content: center; transition: all 0.2s; flex-shrink: 0;", bg, border));

    if checked {
        box_indicator = box_indicator.child(Icon::render("check", 14, "#ffffff"));
    } else if indeterminate {
        box_indicator = box_indicator.child(Icon::render("minus", 14, "#ffffff"));
    }

    dom_label()
        .attr("for", id)
        .attr("class", "ferrox-checkbox-wrapper")
        .attr("style", "display: inline-flex; align-items: center; gap: 8px; cursor: pointer; user-select: none; font-size: 0.88rem; color: var(--text-primary);")
        .child(box_indicator)
        .child(span().text(label_text))
}

/// Custom Styled Radio Group
pub fn radio_group(name: &str, options: Vec<(&str, &str)>, selected_value: &str) -> DomBuilder {
    let mut group = div()
        .attr("class", "ferrox-radio-group")
        .attr("data-radio-group", name)
        .attr("style", "display: flex; flex-direction: column; gap: 8px;");

    for (val, label_text) in options {
        let is_selected = val == selected_value;
        let ring_color = if is_selected { "var(--accent-primary, #3b82f6)" } else { "rgba(255,255,255,0.3)" };

        let mut circle_el = div()
            .attr("style", &format!("width: 18px; height: 18px; border-radius: 50%; border: 2px solid {}; display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;", ring_color));

        if is_selected {
            circle_el = circle_el.child(
                div().attr("style", "width: 8px; height: 8px; border-radius: 50%; background: var(--accent-primary, #3b82f6);")
            );
        }

        let item = dom_label()
            .attr("data-name", name)
            .attr("data-value", val)
            .attr("style", "display: inline-flex; align-items: center; gap: 8px; cursor: pointer; font-size: 0.88rem; user-select: none;")
            .child(circle_el)
            .child(span().text(label_text));

        group = group.child(item);
    }

    group
}

/// Custom Filterable Select Dropdown
pub fn select_dropdown(id: &str, options: Vec<(&str, &str)>, selected_value: &str) -> DomBuilder {
    let style = "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-surface, rgba(0,0,0,0.25)); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; cursor: pointer;";
    let mut sel = dom_select()
        .attr("id", id)
        .attr("class", "ferrox-select-dropdown")
        .attr("style", style);

    for (val, label_text) in options {
        let mut opt = dom_option()
            .attr("value", val)
            .text(label_text);
        if val == selected_value {
            opt = opt.attr("selected", "true");
        }
        sel = sel.child(opt);
    }
    sel
}

/// Range Slider with interactive tooltip bubble
pub fn range_slider_with_bubble(id: &str, min: i32, max: i32, value: i32, unit: &str) -> DomBuilder {
    let pct = ((value - min) as f64 / (max - min) as f64) * 100.0;

    div()
        .attr("class", "ferrox-slider-bubble-container")
        .attr("style", "position: relative; width: 100%; padding-top: 24px;")
        .child(
            div()
                .attr("style", &format!("position: absolute; top: 0; left: {}%; transform: translateX(-50%); font-size: 0.75rem; font-weight: 700; background: var(--accent-primary, #3b82f6); color: white; padding: 2px 6px; border-radius: 4px; pointer-events: none;", pct))
                .text(&format!("{}{}", value, unit))
        )
        .child(
            dom_input()
                .attr("id", id)
                .attr("type", "range")
                .attr("min", &min.to_string())
                .attr("max", &max.to_string())
                .attr("value", &value.to_string())
                .attr("style", "width: 100%; accent-color: var(--accent-primary, #3b82f6); cursor: pointer;")
        )
}

// =============================================================================
// 3. SURFACES, OVERLAYS & FEEDBACK
// =============================================================================

/// Renders an Enterprise Modal dialog with blurred backdrop, size variants, and close handler
pub fn modal(title: &str, content: DomBuilder) -> DomBuilder {
    modal_advanced("modal-default", title, content, None, "md")
}

pub fn modal_advanced(
    id: &str,
    title: &str,
    content: DomBuilder,
    footer: Option<DomBuilder>,
    size: &str,
) -> DomBuilder {
    let max_w = match size {
        "sm" => "380px",
        "lg" => "720px",
        "xl" => "960px",
        "fullscreen" => "98vw",
        _ => "520px", // "md"
    };

    let overlay_style = "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(12px); display: flex; align-items: center; justify-content: center; z-index: 10000; animation: ferrox-fade-in 0.2s ease;";
    let modal_style = format!(
        "background: var(--bg-surface, #0f172a); border: var(--border-glass); border-radius: 14px; padding: 24px; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.7); width: 90%; max-width: {}; max-height: 90vh; overflow-y: auto; display: flex; flex-direction: column; gap: 16px; position: relative; animation: ferrox-scale-up 0.25s cubic-bezier(0.16, 1, 0.3, 1);",
        max_w
    );

    let modal_id_owned = id.to_string();
    let close_btn = dom_button()
        .attr("class", "ferrox-btn ferrox-btn-ghost")
        .attr("style", "position: absolute; top: 16px; right: 16px; padding: 6px; border-radius: 50%; opacity: 0.7;")
        .on_click(move || {
            if let Some(el) = ferrox_front_core::dom::get_element_by_id(&modal_id_owned) {
                el.remove();
            }
        })
        .child(Icon::render("x", 18, "var(--text-primary)"));

    let header = div()
        .attr("style", "display: flex; align-items: center; border-bottom: var(--border-glass); padding-bottom: 12px; padding-right: 40px;")
        .child(span().attr("style", "font-size: 1.25rem; font-weight: 700; color: var(--text-primary);").text(title));

    let mut body = div().attr("style", "flex: 1; overflow-y: auto;").child(content);

    if let Some(f) = footer {
        body = body.child(
            div().attr("style", "margin-top: 16px; padding-top: 12px; border-top: var(--border-glass); display: flex; justify-content: flex-end; gap: 10px;").child(f)
        );
    }

    div()
        .attr("id", id)
        .attr("class", "ferrox-modal-backdrop")
        .attr("style", overlay_style)
        .child(
            div()
                .attr("class", "ferrox-modal-content")
                .attr("style", &modal_style)
                .child(close_btn)
                .child(header)
                .child(body)
        )
}

/// Sliding Drawer / Offcanvas Panel (left or right)
pub fn drawer(id: &str, title: &str, content: DomBuilder, side: &str) -> DomBuilder {
    let (pos, transform) = match side {
        "left" => ("left: 0; top: 0; bottom: 0;", "animation: ferrox-slide-left 0.3s ease;"),
        _ => ("right: 0; top: 0; bottom: 0;", "animation: ferrox-slide-right 0.3s ease;"),
    };

    let panel_style = format!(
        "position: fixed; {}; width: 340px; max-width: 85vw; background: var(--bg-surface, #0f172a); border-{}: var(--border-glass); box-shadow: 0 20px 25px -5px rgba(0,0,0,0.5); z-index: 10001; padding: 20px; display: flex; flex-direction: column; gap: 16px; {}",
        pos, match side { "left" => "right", _ => "left" }, transform
    );

    let drawer_id_owned = id.to_string();
    let close_btn = dom_button()
        .attr("class", "ferrox-btn ferrox-btn-ghost")
        .attr("style", "padding: 6px; border-radius: 50%; opacity: 0.7;")
        .on_click(move || {
            if let Some(el) = ferrox_front_core::dom::get_element_by_id(&drawer_id_owned) {
                el.remove();
            }
        })
        .child(Icon::render("x", 18, "var(--text-primary)"));

    let header = div()
        .attr("style", "display: flex; justify-content: space-between; align-items: center; border-bottom: var(--border-glass); padding-bottom: 12px;")
        .child(span().attr("style", "font-weight: 700; font-size: 1.15rem;").text(title))
        .child(close_btn);

    div()
        .attr("id", id)
        .attr("class", "ferrox-drawer-backdrop")
        .attr("style", "position: fixed; inset: 0; background: rgba(0,0,0,0.5); backdrop-filter: blur(4px); z-index: 10000;")
        .child(
            div()
                .attr("class", "ferrox-drawer-panel")
                .attr("style", &panel_style)
                .child(header)
                .child(div().attr("style", "flex: 1; overflow-y: auto;").child(content))
        )
}

/// Collapsible Accordion section item
pub struct AccordionItem {
    pub title: String,
    pub content: DomBuilder,
    pub is_expanded: bool,
}

/// Renders collapsible Accordion sections
pub fn accordion(items: Vec<AccordionItem>) -> DomBuilder {
    let mut acc = div()
        .attr("class", "ferrox-accordion")
        .attr("style", "display: flex; flex-direction: column; gap: 8px; width: 100%;");

    for (idx, item) in items.into_iter().enumerate() {
        let chevron_icon = if item.is_expanded { "chevron-up" } else { "chevron-down" };
        let content_display = if item.is_expanded { "display: block;" } else { "display: none;" };

        let header = div()
            .attr("class", "ferrox-accordion-header")
            .attr("style", "display: flex; justify-content: space-between; align-items: center; padding: 14px 18px; background: rgba(255,255,255,0.03); border: var(--border-glass); border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.95rem;")
            .child(span().text(&item.title))
            .child(Icon::render(chevron_icon, 18, "var(--text-secondary)"));

        let body = div()
            .attr("id", &format!("accordion-body-{}", idx))
            .attr("class", "ferrox-accordion-body")
            .attr("style", &format!("padding: 14px 18px; border: var(--border-glass); border-top: none; border-radius: 0 0 8px 8px; background: rgba(0,0,0,0.15); font-size: 0.88rem; line-height: 1.5; {}", content_display))
            .child(item.content);

        let section = div().child(header).child(body);
        acc = acc.child(section);
    }

    acc
}

/// Tab navigation item
#[derive(Debug, Clone)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
}

/// Renders reactive Tab navigation bar: variants: "line", "pills", "segmented"
pub fn tabs(items: Vec<TabItem>, active_id: &str, variant: &str) -> DomBuilder {
    let style = match variant {
        "pills" => "display: flex; gap: 8px; background: transparent; padding: 4px 0;",
        "segmented" => "display: inline-flex; gap: 2px; background: rgba(0,0,0,0.3); border-radius: 8px; padding: 3px; border: var(--border-glass);",
        _ => "display: flex; gap: 24px; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 2px;", // "line"
    };

    let mut tab_bar = div().attr("class", &format!("ferrox-tabs ferrox-tabs-{}", variant)).attr("style", style);

    for item in items {
        let is_active = item.id == active_id;

        let item_style = match variant {
            "pills" => {
                if is_active {
                    "padding: 8px 16px; border-radius: 6px; background: var(--accent-primary, #3b82f6); color: #fff; font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 6px;"
                } else {
                    "padding: 8px 16px; border-radius: 6px; background: rgba(255,255,255,0.04); color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; gap: 6px;"
                }
            },
            "segmented" => {
                if is_active {
                    "padding: 6px 14px; border-radius: 6px; background: var(--bg-surface, #1e293b); color: #fff; font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 6px; box-shadow: 0 1px 3px rgba(0,0,0,0.2);"
                } else {
                    "padding: 6px 14px; border-radius: 6px; background: transparent; color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; gap: 6px;"
                }
            },
            _ => { // line
                if is_active {
                    "padding: 10px 0; border-bottom: 2px solid var(--accent-primary, #3b82f6); color: var(--text-primary); font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 6px;"
                } else {
                    "padding: 10px 0; border-bottom: 2px solid transparent; color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; gap: 6px;"
                }
            }
        };

        let mut tab_button = div()
            .attr("class", "ferrox-tab-item")
            .attr("style", item_style);

        if let Some(ref ico) = item.icon {
            tab_button = tab_button.child(Icon::render(ico, 16, if is_active { "#ffffff" } else { "var(--text-secondary)" }));
        }

        tab_button = tab_button.child(span().text(&item.label));
        tab_bar = tab_bar.child(tab_button);
    }

    tab_bar
}

/// Notification Toast with timed progress bar
pub fn toast(title: &str, message: &str, variant: &str) -> DomBuilder {
    let (icon_name, border_col) = match variant {
        "success" => ("check-circle", "#10b981"),
        "danger" => ("alert-circle", "#ef4444"),
        "warning" => ("alert-triangle", "#f59e0b"),
        _ => ("info", "#3b82f6"),
    };

    div()
        .attr("class", &format!("ferrox-toast ferrox-toast-{}", variant))
        .attr("style", &format!("background: var(--bg-surface, #1e293b); border-left: 4px solid {}; border-radius: 8px; padding: 14px 18px; box-shadow: 0 10px 25px rgba(0,0,0,0.4); display: flex; flex-direction: column; gap: 6px; min-width: 280px; max-width: 380px; position: relative; overflow: hidden;", border_col))
        .child(
            div()
                .attr("style", "display: flex; align-items: center; gap: 8px;")
                .child(Icon::render(icon_name, 18, border_col))
                .child(span().attr("style", "font-weight: 700; font-size: 0.92rem; color: var(--text-primary);").text(title))
        )
        .child(
            span().attr("style", "font-size: 0.82rem; opacity: 0.8; line-height: 1.4; color: var(--text-secondary);").text(message)
        )
        .child(
            div()
                .attr("class", "ferrox-toast-progress")
                .attr("style", &format!("position: absolute; bottom: 0; left: 0; height: 3px; background: {}; width: 100%; animation: ferrox-toast-shrink 4s linear;", border_col))
        )
}

/// Skeleton loading placeholder with pulsed light shimmer
pub fn skeleton(variant: &str) -> DomBuilder {
    let (w, h, r) = match variant {
        "avatar" => ("40px", "40px", "50%"),
        "title" => ("60%", "24px", "4px"),
        "card" => ("100%", "160px", "10px"),
        _ => ("100%", "16px", "4px"), // "text"
    };

    div()
        .attr("class", &format!("ferrox-skeleton ferrox-skeleton-{}", variant))
        .attr("style", &format!("width: {}; height: {}; border-radius: {}; background: linear-gradient(90deg, rgba(255,255,255,0.04) 25%, rgba(255,255,255,0.09) 50%, rgba(255,255,255,0.04) 75%); background-size: 200% 100%; animation: ferrox-shimmer 1.5s infinite;", w, h, r))
}

/// Tooltip wrapper rendering tooltip bubble on hover
pub fn tooltip(target: DomBuilder, tooltip_text: &str) -> DomBuilder {
    div()
        .attr("class", "ferrox-tooltip-wrapper")
        .attr("style", "position: relative; display: inline-block;")
        .child(target)
        .child(
            span()
                .attr("class", "ferrox-tooltip-bubble")
                .attr("style", "position: absolute; bottom: 125%; left: 50%; transform: translateX(-50%); background: #0f172a; color: white; padding: 4px 8px; border-radius: 4px; font-size: 0.75rem; white-space: nowrap; pointer-events: none; opacity: 0; transition: opacity 0.2s; box-shadow: 0 4px 12px rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.1);")
                .text(tooltip_text)
        )
}

/// Circular Progress Ring
pub fn progress_bar_circular(percentage: f64, size: u32) -> DomBuilder {
    let pct = percentage.clamp(0.0, 100.0);
    let r = (size as f64 - 10.0) / 2.0;
    let circ = 2.0 * std::f64::consts::PI * r;
    let offset = circ - (pct / 100.0) * circ;

    svg()
        .attr("width", &size.to_string())
        .attr("height", &size.to_string())
        .attr("style", "transform: rotate(-90deg);")
        .child(
            circle()
                .attr("cx", &(size / 2).to_string())
                .attr("cy", &(size / 2).to_string())
                .attr("r", &r.to_string())
                .attr("stroke", "rgba(255,255,255,0.1)")
                .attr("stroke-width", "6")
                .attr("fill", "transparent")
        )
        .child(
            circle()
                .attr("cx", &(size / 2).to_string())
                .attr("cy", &(size / 2).to_string())
                .attr("r", &r.to_string())
                .attr("stroke", "var(--accent-primary, #3b82f6)")
                .attr("stroke-width", "6")
                .attr("stroke-dasharray", &circ.to_string())
                .attr("stroke-dashoffset", &offset.to_string())
                .attr("stroke-linecap", "round")
                .attr("fill", "transparent")
                .attr("style", "transition: stroke-dashoffset 0.5s ease;")
        )
}

// =============================================================================
// 4. NAVIGATION & STRUCTURE
// =============================================================================

/// Renders an Enterprise Top Navbar with logo/brand, navigation links, and actions
pub fn navbar(brand_name: &str, links: Vec<(&str, &str)>) -> DomBuilder {
    let nav = dom_nav()
        .attr("class", "ferrox-navbar")
        .attr("style", "display: flex; justify-content: space-between; align-items: center; padding: 1rem 2rem; background: var(--bg-surface, rgba(15,23,42,0.8)); backdrop-filter: blur(12px); border-bottom: var(--border-glass); position: sticky; top: 0; z-index: 1000;");

    let brand = div()
        .attr("style", "display: flex; align-items: center; gap: 10px; font-weight: 700; font-size: 1.2rem; color: var(--text-primary); cursor: pointer;")
        .child(Icon::render("zap", 22, "var(--accent-cyan, #06b6d4)"))
        .child(span().text(brand_name));

    let mut link_container = div()
        .attr("class", "ferrox-nav-links")
        .attr("style", "display: flex; gap: 20px; align-items: center;");

    for (url, label_text) in links {
        let l = dom_a()
            .attr("href", url)
            .attr("style", "color: var(--text-secondary); text-decoration: none; font-size: 0.9rem; font-weight: 500; transition: color 0.2s;")
            .text(label_text);
        link_container = link_container.child(l);
    }

    nav.child(brand).child(link_container)
}

pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub badge_count: Option<usize>,
    pub is_active: bool,
}

/// Renders a Collapsible Sidebar with icons, labels, and badges
pub fn sidebar(items: Vec<SidebarItem>, collapsed: bool) -> DomBuilder {
    let w = if collapsed { "68px" } else { "240px" };

    let mut sb = div()
        .attr("class", "ferrox-sidebar")
        .attr("style", &format!("width: {}; min-height: 100vh; background: var(--bg-surface, #0f172a); border-right: var(--border-glass); padding: 1.5rem 10px; display: flex; flex-direction: column; gap: 6px; transition: width 0.3s cubic-bezier(0.4,0,0.2,1);", w));

    for item in items {
        let (bg, col) = if item.is_active {
            ("var(--accent-primary, #3b82f6)", "#ffffff")
        } else {
            ("transparent", "var(--text-secondary, #94a3b8)")
        };

        let mut row = div()
            .attr("class", &format!("ferrox-sidebar-item {}", if item.is_active { "active" } else { "" }))
            .attr("style", &format!("display: flex; align-items: center; gap: 12px; padding: 10px 12px; border-radius: 8px; background: {}; color: {}; cursor: pointer; transition: all 0.2s;", bg, col))
            .child(Icon::render(&item.icon, 20, col));

        if !collapsed {
            row = row.child(span().attr("style", "flex: 1; font-size: 0.88rem; font-weight: 500;").text(&item.label));

            if let Some(cnt) = item.badge_count {
                let b = span()
                    .attr("style", "background: rgba(255,255,255,0.2); font-size: 0.72rem; font-weight: 700; padding: 2px 6px; border-radius: 999px;")
                    .text(&cnt.to_string());
                row = row.child(b);
            }
        }

        sb = sb.child(row);
    }

    sb
}

pub struct BreadcrumbItem {
    pub label: String,
    pub url: Option<String>,
}

/// Renders a Breadcrumb navigation path
pub fn breadcrumb(items: Vec<BreadcrumbItem>) -> DomBuilder {
    let mut bc = div()
        .attr("class", "ferrox-breadcrumb")
        .attr("style", "display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: var(--text-secondary);");

    let count = items.len();
    for (i, item) in items.into_iter().enumerate() {
        if let Some(url) = item.url {
            bc = bc.child(
                dom_a()
                    .attr("href", &url)
                    .attr("style", "color: var(--text-secondary); text-decoration: none;")
                    .text(&item.label)
            );
        } else {
            bc = bc.child(
                span().attr("style", "color: var(--text-primary); font-weight: 600;").text(&item.label)
            );
        }

        if i + 1 < count {
            bc = bc.child(Icon::render("chevron-right", 14, "var(--text-secondary)"));
        }
    }

    bc
}

/// Renders a Complete Pagination Bar with ellipsis and page numbers
pub fn pagination(current_page: u32, total_pages: u32) -> DomBuilder {
    let mut p_bar = div()
        .attr("class", "ferrox-pagination")
        .attr("style", "display: flex; align-items: center; gap: 6px;");

    // Prev
    let prev = dom_button()
        .attr("class", "ferrox-btn ferrox-btn-secondary")
        .attr("style", "padding: 6px 10px; border-radius: 6px;")
        .child(Icon::render("chevron-left", 14, "var(--text-primary)"));
    p_bar = p_bar.child(prev);

    for p in 1..=total_pages {
        let is_current = p == current_page;
        let (bg, col) = if is_current {
            ("var(--accent-primary, #3b82f6)", "#ffffff")
        } else {
            ("rgba(255,255,255,0.05)", "var(--text-primary)")
        };

        let btn = dom_button()
            .attr("style", &format!("width: 32px; height: 32px; border-radius: 6px; background: {}; color: {}; border: var(--border-glass); font-weight: 600; font-size: 0.85rem; cursor: pointer;", bg, col))
            .text(&p.to_string());
        p_bar = p_bar.child(btn);
    }

    // Next
    let next = dom_button()
        .attr("class", "ferrox-btn ferrox-btn-secondary")
        .attr("style", "padding: 6px 10px; border-radius: 6px;")
        .child(Icon::render("chevron-right", 14, "var(--text-primary)"));
    p_bar = p_bar.child(next);

    p_bar
}

// =============================================================================
// 5. EXISTING SURFACES & CONTAINERS (PRESERVED FOR BACKWARD COMPATIBILITY)
// =============================================================================

pub fn card() -> DomBuilder {
    div()
        .attr("class", "ferrox-glass-card")
        .attr("style", "background: var(--bg-glass, rgba(255,255,255,0.04)); backdrop-filter: blur(12px); border: var(--border-glass); border-radius: 12px; padding: 1.25rem; box-shadow: 0 8px 32px rgba(0,0,0,0.37); display: flex; flex-direction: column;")
}

pub fn card_header(title: &str) -> DomBuilder {
    div()
        .attr("style", "font-weight: 700; font-size: 1.15rem; margin-bottom: 1rem; border-bottom: var(--border-glass); padding-bottom: 0.6rem; color: var(--text-primary);")
        .text(title)
}

pub fn card_body() -> DomBuilder {
    div().attr("style", "flex: 1;")
}

pub fn card_footer() -> DomBuilder {
    div().attr("style", "margin-top: 1rem; padding-top: 0.6rem; border-top: var(--border-glass); display: flex; gap: 8px; justify-content: flex-end;")
}

pub fn badge(text: &str, variant: &str) -> DomBuilder {
    let (bg, color) = match variant {
        "success" => ("rgba(16, 185, 129, 0.2)", "#34d399"),
        "danger" => ("rgba(239, 68, 68, 0.2)", "#fca5a5"),
        "warning" => ("rgba(245, 158, 11, 0.2)", "#fde047"),
        "info" => ("rgba(14, 165, 233, 0.2)", "#7dd3fc"),
        _ => ("rgba(147, 51, 234, 0.2)", "#c084fc"),
    };

    span()
        .attr("class", &format!("ferrox-badge ferrox-badge-{}", variant))
        .attr("style", &format!("background: {}; color: {}; border: var(--border-glass); padding: 3px 10px; border-radius: 9999px; font-size: 0.78rem; font-weight: 700; display: inline-block; white-space: nowrap;", bg, color))
        .text(text)
}

pub fn alert(message: &str, variant: &str) -> DomBuilder {
    let (bg, color) = match variant {
        "danger" => ("rgba(239, 68, 68, 0.15)", "#fca5a5"),
        "success" => ("rgba(16, 185, 129, 0.15)", "#34d399"),
        "warning" => ("rgba(245, 158, 11, 0.15)", "#fde047"),
        _ => ("rgba(147, 51, 234, 0.15)", "#c084fc"),
    };

    div()
        .attr("class", &format!("ferrox-alert ferrox-alert-{}", variant))
        .attr("style", &format!("padding: 1rem; border-radius: 8px; background: {}; color: {}; border: var(--border-glass); margin-bottom: 1rem; font-weight: 500; font-size: 0.92rem;", bg, color))
        .text(message)
}

pub fn form_group(label_text: &str, input_element: DomBuilder) -> DomBuilder {
    div()
        .attr("class", "ferrox-form-group")
        .attr("style", "margin-bottom: 1rem; display: flex; flex-direction: column; gap: 6px;")
        .child(dom_label().attr("style", "font-size: 0.85rem; font-weight: 600; color: var(--accent-cyan);").text(label_text))
        .child(input_element)
}

pub fn progress_bar(percentage: f64) -> DomBuilder {
    let pct = percentage.clamp(0.0, 100.0);
    div()
        .attr("class", "ferrox-progress-track")
        .attr("style", "width: 100%; height: 8px; background: rgba(255,255,255,0.1); border-radius: 9999px; overflow: hidden;")
        .child(
            div()
                .attr("class", "ferrox-progress-fill")
                .attr("style", &format!("width: {}%; height: 100%; background: var(--btn-bg, linear-gradient(135deg, #7e22ce, #3b82f6)); transition: width 0.3s ease;", pct))
        )
}

pub fn avatar(initials: &str, img_url: Option<&str>) -> DomBuilder {
    let size = 40;
    let mut builder = div()
        .attr("class", "ferrox-avatar")
        .attr("style", &format!("width: {}px; height: {}px; border-radius: 50%; display: inline-flex; align-items: center; justify-content: center; font-weight: 700; color: #fff; background: var(--btn-bg, linear-gradient(135deg, #7e22ce, #3b82f6)); overflow: hidden; border: 2px solid rgba(255,255,255,0.1);", size, size));

    if let Some(url) = img_url {
        let img = ferrox_front_core::dom::img()
            .attr("src", url)
            .attr("style", "width: 100%; height: 100%; object-fit: cover;");
        builder = builder.child(img);
    } else {
        builder = builder.text(initials);
    }
    builder
}

pub fn user_pill(name: &str, role: &str) -> DomBuilder {
    let initials = name.split_whitespace().filter_map(|w| w.chars().next()).collect::<String>();
    div()
        .attr("class", "ferrox-user-pill")
        .attr("style", "display: inline-flex; align-items: center; gap: 10px; padding: 4px 12px 4px 4px; background: var(--bg-glass); border: var(--border-glass); border-radius: 9999px;")
        .child(avatar(&initials, None))
        .child(
            div()
                .attr("style", "display: flex; flex-direction: column;")
                .child(span().attr("style", "font-weight: 600; font-size: 0.85rem; color: var(--text-primary);").text(name))
                .child(span().attr("style", "font-size: 0.72rem; opacity: 0.7; color: var(--accent-cyan);").text(role))
        )
}

pub fn spinner() -> DomBuilder {
    micro_spinner(24)
}

pub fn micro_spinner(size: u32) -> DomBuilder {
    div()
        .attr("class", "ferrox-spinner")
        .attr("style", &format!("width: {}px; height: {}px; border: 2px solid rgba(255,255,255,0.2); border-top-color: #ffffff; border-radius: 50%; animation: ferrox-spin 0.8s linear infinite; flex-shrink: 0;", size, size))
}

pub fn textarea(placeholder: &str, rows: u32) -> DomBuilder {
    dom_textarea()
        .attr("rows", &rows.to_string())
        .attr("placeholder", placeholder)
        .attr("class", "ferrox-textarea")
        .attr("style", "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-surface, rgba(0,0,0,0.25)); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; font-family: inherit; resize: vertical;")
}

pub fn select(options: Vec<(&str, &str)>) -> DomBuilder {
    select_dropdown("select-default", options, "")
}

pub fn checkbox(label_text: &str, checked: bool) -> DomBuilder {
    checkbox_advanced("cb-default", label_text, checked, false)
}

pub fn radio(name: &str, value: &str, label_text: &str, checked: bool) -> DomBuilder {
    let mut rb = dom_input()
        .attr("type", "radio")
        .attr("name", name)
        .attr("value", value)
        .attr("style", "accent-color: var(--accent-primary); width: 16px; height: 16px; cursor: pointer;");
    if checked {
        rb = rb.attr("checked", "true");
    }

    dom_label()
        .attr("style", "display: inline-flex; align-items: center; gap: 8px; font-size: 0.88rem; cursor: pointer; color: var(--text-primary);")
        .child(rb)
        .child(span().text(label_text))
}

pub fn toggle_switch(label_text: &str, checked: bool) -> DomBuilder {
    switch_toggle("toggle-default", checked, label_text)
}

pub fn range_slider(min: i32, max: i32, value: i32) -> DomBuilder {
    range_slider_with_bubble("slider-default", min, max, value, "")
}

pub fn search_input(placeholder: &str) -> DomBuilder {
    input_advanced(InputConfig {
        input_type: "search".to_string(),
        placeholder: placeholder.to_string(),
        prefix_icon: Some("search".to_string()),
        clearable: true,
        ..Default::default()
    })
}

pub fn file_upload(accept: &str) -> DomBuilder {
    div()
        .attr("style", "border: 2px dashed var(--border-glass); border-radius: 8px; padding: 1.5rem; text-align: center; background: rgba(0,0,0,0.2); cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 6px;")
        .child(Icon::render("upload", 28, "var(--accent-primary, #3b82f6)"))
        .child(span().attr("style", "font-size: 0.88rem; font-weight: 600; color: var(--text-primary);").text("Click or drop files to upload"))
        .child(span().attr("style", "font-size: 0.75rem; opacity: 0.6;").text(&format!("Supported formats: {}", accept)))
        .child(
            dom_input()
                .attr("type", "file")
                .attr("accept", accept)
                .attr("style", "display: none;")
        )
}
