use ferrox_front_core::dom::{div, DomBuilder};

pub struct KanbanColumn {
    pub title: String,
    pub color_border: String,
}

pub fn ferrox_kanban(columns: Vec<KanbanColumn>) -> DomBuilder {
    let mut board = div()
        .attr("class", "ferrox-kanban")
        .attr("style", "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; align-items: start;");

    for col in columns {
        let col_el = div()
            .attr("style", "background: var(--bg-glass); border-radius: 10px; padding: 14px; min-height: 400px; border: var(--border-glass);")
            .child(
                div()
                    .attr("style", &format!("font-size: 15px; font-weight: 700; padding-bottom: 12px; margin-bottom: 12px; border-bottom: 2px solid {};", col.color_border))
                    .text(&col.title)
            )
            .child(
                div().attr("style", "color: var(--text-secondary); font-size: 0.9rem;").text("No tickets")
            );
        board = board.child(col_el);
    }
    
    board
}
