use ferrox_front_core::dom::{div, DomBuilder};

pub fn card() -> DomBuilder {
    div().attr("class", "ferrox-glass-card").attr("style", "display: flex; flex-direction: column;")
}

pub fn card_header(title: &str) -> DomBuilder {
    div().attr("style", "font-weight: 600; font-size: 1.2rem; margin-bottom: 1rem; border-bottom: var(--border-glass); padding-bottom: 0.5rem;").text(title)
}

pub fn card_body() -> DomBuilder {
    div().attr("style", "flex: 1;")
}

pub fn alert(message: &str, variant: &str) -> DomBuilder {
    let bg = match variant {
        "danger" => "hsla(0, 80%, 50%, 0.2)",
        "success" => "hsla(120, 80%, 30%, 0.2)",
        _ => "var(--accent-glow)",
    };
    
    div()
        .attr("style", &format!("padding: 1rem; border-radius: 8px; background: {}; border: var(--border-glass); margin-bottom: 1rem;", bg))
        .text(message)
}
