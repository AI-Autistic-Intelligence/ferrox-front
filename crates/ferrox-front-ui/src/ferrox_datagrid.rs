use ferrox_front_core::dom::{div, DomBuilder};

pub struct FerroxDataGridConfig {
    pub schema_url: String,
    pub entity_name: String,
}

pub fn ferrox_datagrid(config: &FerroxDataGridConfig) -> DomBuilder {
    // Basic representation of dynamic schema driven datagrid
    div()
        .attr("class", "ferrox-datagrid")
        .attr("style", "padding: 1rem; border: var(--border-glass); background: var(--bg-glass); border-radius: 8px;")
        .child(
            div().attr("style", "font-size: 1.2rem; font-weight: bold; margin-bottom: 1rem;")
                 .text(&format!("Entity DataGrid: {}", config.entity_name))
        )
        .child(
            div().text(&format!("Fetching schema and rows from {}...", config.schema_url))
        )
}
