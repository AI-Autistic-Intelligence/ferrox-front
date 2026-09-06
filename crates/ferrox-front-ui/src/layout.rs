use ferrox_front_core::dom::{div, DomBuilder};

pub fn container() -> DomBuilder {
    div().attr("class", "fx-container")
}

pub fn row() -> DomBuilder {
    div().attr("class", "fx-row")
}

/// Creates a column with specific breakpoint spans, e.g., "12 md-6 lg-4"
pub fn col(spans: &str) -> DomBuilder {
    let mut class_str = String::from("fx-col");
    for span in spans.split_whitespace() {
        if span.contains('-') {
            class_str.push_str(&format!(" fx-col-{}", span));
        } else {
            class_str.push_str(&format!(" fx-col-{}", span));
        }
    }
    div().attr("class", &class_str)
}
