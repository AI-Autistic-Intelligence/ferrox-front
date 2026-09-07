//! # Glassmorphism Design System Components (`ferrox-front-ui::components`)
//!
//! `components` provides an enterprise-grade Glassmorphism component library:
//! Buttons, Badges, Cards, Form Controls, Progress Bars, Spinners, Avatars, User Pills, and Alerts.

use ferrox_front_core::dom::{div, span, button as dom_button, p, DomBuilder};

/// Renders a Glassmorphism Card container
pub fn card() -> DomBuilder {
    div()
        .attr("class", "ferrox-glass-card")
        .attr("style", "background: var(--bg-glass); backdrop-filter: blur(12px); border: var(--border-glass); border-radius: 12px; padding: 1.25rem; box-shadow: 0 8px 32px rgba(0,0,0,0.37); display: flex; flex-direction: column;")
}

/// Renders a Card Header with title text
pub fn card_header(title: &str) -> DomBuilder {
    div()
        .attr("style", "font-weight: 700; font-size: 1.15rem; margin-bottom: 1rem; border-bottom: var(--border-glass); padding-bottom: 0.6rem; color: var(--text-primary);")
        .text(title)
}

/// Renders a Card Body content container
pub fn card_body() -> DomBuilder {
    div().attr("style", "flex: 1;")
}

/// Renders a Card Footer container
pub fn card_footer() -> DomBuilder {
    div().attr("style", "margin-top: 1rem; padding-top: 0.6rem; border-top: var(--border-glass); display: flex; gap: 8px; justify-content: flex-end;")
}

/// Renders a Design System Button with variants: "primary", "secondary", "outline", "glass", "danger", "ghost"
pub fn button(text: &str, variant: &str) -> DomBuilder {
    let (bg, color, border) = match variant {
        "primary" => ("var(--btn-bg, linear-gradient(135deg, #7e22ce, #3b82f6))", "#ffffff", "none"),
        "secondary" => ("rgba(255,255,255,0.1)", "var(--text-primary)", "var(--border-glass)"),
        "danger" => ("linear-gradient(135deg, #dc2626, #991b1b)", "#ffffff", "none"),
        "outline" => ("transparent", "var(--accent-cyan, #06b6d4)", "1px solid var(--accent-cyan, #06b6d4)"),
        "ghost" => ("transparent", "var(--text-primary)", "none"),
        _ => ("var(--bg-glass)", "var(--text-primary)", "var(--border-glass)"), // "glass"
    };

    let style = format!(
        "background: {}; color: {}; border: {}; padding: 8px 16px; border-radius: 6px; font-weight: 600; font-size: 0.9rem; cursor: pointer; transition: all 0.2s ease; display: inline-flex; align-items: center; justify-content: center; gap: 6px;",
        bg, color, border
    );

    dom_button().attr("class", &format!("ferrox-btn ferrox-btn-{}", variant)).attr("style", &style).text(text)
}

/// Renders a Status Badge pill: "primary", "success", "warning", "danger", "info"
pub fn badge(text: &str, variant: &str) -> DomBuilder {
    let (bg, color) = match variant {
        "success" => ("rgba(16, 185, 129, 0.2)", "#34d399"),
        "danger" => ("rgba(239, 68, 68, 0.2)", "#fca5a5"),
        "warning" => ("rgba(245, 158, 11, 0.2)", "#fde047"),
        "info" => ("rgba(14, 165, 233, 0.2)", "#7dd3fc"),
        _ => ("rgba(147, 51, 234, 0.2)", "#c084fc"),
    };

    let style = format!(
        "background: {}; color: {}; border: var(--border-glass); padding: 3px 10px; border-radius: 9999px; font-size: 0.78rem; font-weight: 700; display: inline-block;",
        bg, color
    );

    span().attr("class", &format!("ferrox-badge ferrox-badge-{}", variant)).attr("style", &style).text(text)
}

/// Renders a Notification Alert banner: "success", "danger", "warning", "info"
pub fn alert(message: &str, variant: &str) -> DomBuilder {
    let (bg, color) = match variant {
        "danger" => ("rgba(239, 68, 68, 0.15)", "#fca5a5"),
        "success" => ("rgba(16, 185, 129, 0.15)", "#34d399"),
        "warning" => ("rgba(245, 158, 11, 0.15)", "#fde047"),
        _ => ("rgba(147, 51, 234, 0.15)", "#c084fc"),
    };
    
    let style = format!(
        "padding: 1rem; border-radius: 8px; background: {}; color: {}; border: var(--border-glass); margin-bottom: 1rem; font-weight: 500; font-size: 0.92rem;",
        bg, color
    );

    div().attr("class", &format!("ferrox-alert ferrox-alert-{}", variant)).attr("style", &style).text(message)
}

/// Renders a Form Input field
pub fn input(input_type: &str, placeholder: &str) -> DomBuilder {
    let style = "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-primary); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none;";
    DomBuilder::new("input")
        .attr("type", input_type)
        .attr("placeholder", placeholder)
        .attr("class", "ferrox-input")
        .attr("style", style)
}

/// Wraps a label and input element in a form group
pub fn form_group(label_text: &str, input_element: DomBuilder) -> DomBuilder {
    div()
        .attr("class", "ferrox-form-group")
        .attr("style", "margin-bottom: 1rem; display: flex; flex-direction: column; gap: 6px;")
        .child(
            DomBuilder::new("label")
                .attr("style", "font-size: 0.85rem; font-weight: 600; color: var(--accent-cyan);")
                .text(label_text)
        )
        .child(input_element)
}

/// Renders a Progress Bar (percentage 0.0 to 100.0)
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

/// Renders an Avatar circle with user initials
pub fn avatar(initials: &str) -> DomBuilder {
    div()
        .attr("class", "ferrox-avatar")
        .attr("style", "width: 38px; height: 38px; border-radius: 50%; background: var(--btn-bg); color: #fff; font-weight: 700; font-size: 0.88rem; display: inline-flex; align-items: center; justify-content: center; border: var(--border-glass);")
        .text(initials)
}

/// Renders a User Pill component (Avatar + Name + Role)
pub fn user_pill(name: &str, role: &str) -> DomBuilder {
    let initials = name.split_whitespace().filter_map(|w| w.chars().next()).collect::<String>();
    div()
        .attr("class", "ferrox-user-pill")
        .attr("style", "display: inline-flex; align-items: center; gap: 10px; padding: 4px 12px 4px 4px; background: var(--bg-glass); border: var(--border-glass); border-radius: 9999px;")
        .child(avatar(&initials))
        .child(
            div()
                .attr("style", "display: flex; flex-direction: column;")
                .child(span().attr("style", "font-weight: 600; font-size: 0.85rem; color: var(--text-primary);").text(name))
                .child(span().attr("style", "font-size: 0.72rem; opacity: 0.7; color: var(--accent-cyan);").text(role))
        )
}

/// Renders a Loading Spinner
pub fn spinner() -> DomBuilder {
    div()
        .attr("class", "ferrox-spinner")
        .attr("style", "width: 24px; height: 24px; border: 3px solid rgba(255,255,255,0.2); border-top-color: var(--accent-cyan); border-radius: 50%; animation: ferrox-spin 0.8s linear infinite;")
}

/// Renders a multi-line Textarea input
pub fn textarea(placeholder: &str, rows: u32) -> DomBuilder {
    let style = "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-primary); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; font-family: inherit; resize: vertical;";
    DomBuilder::new("textarea")
        .attr("rows", &rows.to_string())
        .attr("placeholder", placeholder)
        .attr("class", "ferrox-textarea")
        .attr("style", style)
}

/// Renders a Select Dropdown input with options
pub fn select(options: Vec<(&str, &str)>) -> DomBuilder {
    let style = "width: 100%; padding: 10px 14px; border-radius: 6px; background: var(--bg-primary); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none; cursor: pointer;";
    let mut sel = DomBuilder::new("select")
        .attr("class", "ferrox-select")
        .attr("style", style);

    for (val, label_text) in options {
        let opt = DomBuilder::new("option")
            .attr("value", val)
            .text(label_text);
        sel = sel.child(opt);
    }
    sel
}

/// Renders a Styled Checkbox input with label
pub fn checkbox(label_text: &str, checked: bool) -> DomBuilder {
    let mut cb = DomBuilder::new("input")
        .attr("type", "checkbox")
        .attr("style", "accent-color: var(--accent-cyan); width: 16px; height: 16px; cursor: pointer;");
    if checked {
        cb = cb.attr("checked", "true");
    }

    DomBuilder::new("label")
        .attr("style", "display: inline-flex; align-items: center; gap: 8px; font-size: 0.88rem; cursor: pointer; color: var(--text-primary);")
        .child(cb)
        .child(span().text(label_text))
}

/// Renders a Radio Button input with label
pub fn radio(name: &str, value: &str, label_text: &str, checked: bool) -> DomBuilder {
    let mut rb = DomBuilder::new("input")
        .attr("type", "radio")
        .attr("name", name)
        .attr("value", value)
        .attr("style", "accent-color: var(--accent-cyan); width: 16px; height: 16px; cursor: pointer;");
    if checked {
        rb = rb.attr("checked", "true");
    }

    DomBuilder::new("label")
        .attr("style", "display: inline-flex; align-items: center; gap: 8px; font-size: 0.88rem; cursor: pointer; color: var(--text-primary);")
        .child(rb)
        .child(span().text(label_text))
}

/// Renders a Toggle Switch input with label
pub fn toggle_switch(label_text: &str, checked: bool) -> DomBuilder {
    let bg = if checked { "var(--accent-cyan, #06b6d4)" } else { "rgba(255,255,255,0.2)" };
    let translate = if checked { "transform: translateX(18px);" } else { "transform: translateX(0px);" };

    let track = div()
        .attr("style", &format!("width: 38px; height: 20px; border-radius: 9999px; background: {}; position: relative; transition: background 0.3s; padding: 2px; cursor: pointer;", bg))
        .child(
            div()
                .attr("style", &format!("width: 16px; height: 16px; border-radius: 50%; background: #ffffff; transition: transform 0.3s; {}", translate))
        );

    div()
        .attr("style", "display: inline-flex; align-items: center; gap: 10px; cursor: pointer;")
        .child(track)
        .child(span().attr("style", "font-size: 0.88rem; font-weight: 500;").text(label_text))
}

/// Renders a Range Slider input
pub fn range_slider(min: i32, max: i32, value: i32) -> DomBuilder {
    DomBuilder::new("input")
        .attr("type", "range")
        .attr("min", &min.to_string())
        .attr("max", &max.to_string())
        .attr("value", &value.to_string())
        .attr("class", "ferrox-range")
        .attr("style", "width: 100%; accent-color: var(--accent-cyan); cursor: pointer;")
}

/// Renders a Search Bar Input with Icon
pub fn search_input(placeholder: &str) -> DomBuilder {
    div()
        .attr("style", "position: relative; width: 100%;")
        .child(
            DomBuilder::new("input")
                .attr("type", "search")
                .attr("placeholder", placeholder)
                .attr("style", "width: 100%; padding: 10px 14px 10px 36px; border-radius: 6px; background: var(--bg-primary); color: var(--text-primary); border: var(--border-glass); font-size: 0.9rem; outline: none;")
        )
        .child(
            span()
                .attr("style", "position: absolute; left: 12px; top: 50%; transform: translateY(-50%); opacity: 0.6; font-size: 0.9rem;")
                .text("🔍")
        )
}

/// Renders a File Upload Dropzone
pub fn file_upload(accept: &str) -> DomBuilder {
    div()
        .attr("style", "border: 2px dashed var(--border-glass); border-radius: 8px; padding: 1.5rem; text-align: center; background: rgba(0,0,0,0.2); cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 6px;")
        .child(span().attr("style", "font-size: 1.5rem;").text("📁"))
        .child(span().attr("style", "font-size: 0.88rem; font-weight: 600; color: var(--accent-cyan);").text("Click or drop files to upload"))
        .child(span().attr("style", "font-size: 0.75rem; opacity: 0.6;").text(&format!("Supported formats: {}", accept)))
        .child(
            DomBuilder::new("input")
                .attr("type", "file")
                .attr("accept", accept)
                .attr("style", "display: none;")
        )
}
