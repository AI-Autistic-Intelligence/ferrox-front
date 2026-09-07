//! # Ferrox Front UI (`ferrox-front-ui`)
//!
//! `ferrox-front-ui` provides an enterprise-grade Glassmorphism Design System for WebAssembly:
//! 12-column responsive layout grid (`Container`, `Row`, `Col`), Virtualized DataGrid (1,000,000+ rows),
//! Design Tokens, and UI components (Buttons, Badges, Cards, Form Controls, Avatars, Progress Bars, Toasts, Spinners).

pub mod layout;
pub mod components;
pub mod tokens;

use ferrox_front_core::dom::{div, span, DomBuilder};
use ferrox_front_core::reactivity::Signal;

pub struct DataGridState {
    pub scroll_top: Signal<f64>,
    pub row_height: f64,
    pub visible_rows: usize,
}

impl DataGridState {
    pub fn new(row_height: f64, visible_rows: usize) -> Self {
        Self {
            scroll_top: Signal::new(0.0),
            row_height,
            visible_rows,
        }
    }
}

pub fn virtual_data_grid(
    columns: Vec<&str>,
    rows: Vec<Vec<&str>>,
    state: &DataGridState
) -> DomBuilder {
    let total_rows = rows.len();
    let total_height = total_rows as f64 * state.row_height;

    let start_idx = (state.scroll_top.get() / state.row_height).floor() as usize;
    let end_idx = (start_idx + state.visible_rows).min(total_rows);

    let mut container = div()
        .attr("class", "ferrox-table-wrapper")
        .attr("style", &format!("height: {}px; overflow-y: auto; position: relative;", state.row_height * state.visible_rows as f64));
    
    let ghost = div()
        .attr("style", &format!("height: {}px; width: 1px;", total_height));
    container = container.child(ghost);

    let mut content = div()
        .attr("style", &format!("position: absolute; top: {}px; left: 0; width: 100%;", start_idx as f64 * state.row_height));

    let mut thead = div().attr("class", "ferrox-thead").attr("style", "display: flex; background: rgba(0,0,0,0.4); font-weight: 700; border-bottom: var(--border-glass);");
    for col in columns {
        thead = thead.child(span().attr("style", "flex: 1; padding: 12px 16px;").text(col));
    }
    content = content.child(thead);

    for i in start_idx..end_idx {
        let mut tr = div().attr("class", "ferrox-tr").attr("style", &format!("display: flex; height: {}px; align-items: center; border-bottom: 1px solid rgba(255,255,255,0.05);", state.row_height));
        for cell in &rows[i] {
            tr = tr.child(span().attr("style", "flex: 1; padding: 0 16px; font-size: 0.95rem;").text(cell));
        }
        content = content.child(tr);
    }

    container.child(content)
}
