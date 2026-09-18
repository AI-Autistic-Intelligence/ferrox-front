use ferrox_front_core::dom::{div, h1, DomBuilder};
use crate::ferrox_datagrid::{ferrox_datagrid, FerroxDataGridConfig};
use crate::ferrox_theme::{ferrox_theme_provider, ThemeMode};
use crate::ferrox_kanban::{ferrox_kanban, KanbanColumn};
use crate::ferrox_security_dashboard::{ferrox_security_dashboard, SecurityDashboardConfig};
use crate::components::{card, card_header, card_body};

pub fn ferrox_admin_dashboard(
    title: &str,
    datagrid_cfg: FerroxDataGridConfig,
    kanban_cols: Vec<KanbanColumn>,
) -> DomBuilder {
    let sec_config = SecurityDashboardConfig::default();

    ferrox_theme_provider(
        ThemeMode::Cyber,
        vec![
            div().attr("style", "padding: 2rem; display: flex; flex-direction: column; gap: 2rem;").child(
                h1().text(title)
            ).child(
                ferrox_security_dashboard(&sec_config)
            ).child(
                div().attr("style", "display: grid; grid-template-columns: 1fr; gap: 2rem;").child(
                    card().child(
                        card_header("Database Schema (DataGrid)")
                    ).child(
                        card_body().child(
                            ferrox_datagrid(&datagrid_cfg)
                        )
                    )
                ).child(
                    card().child(
                        card_header("Support Tickets (Kanban)")
                    ).child(
                        card_body().child(
                            ferrox_kanban(kanban_cols)
                        )
                    )
                )
            )
        ]
    )
}
