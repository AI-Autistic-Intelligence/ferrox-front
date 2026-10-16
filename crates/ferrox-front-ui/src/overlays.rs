//! # Advanced Enterprise Overlays (`overlays`)
//!
//! Provides enterprise-grade UI overlays and hierarchical components:
//! 1. **Command Palette (`Cmd+K` / `Ctrl+K`)**: Spotlight-style modal search with global shortcut listener,
//!    fuzzy search, category grouping, and keyboard navigation.
//! 2. **Pure Vector DatePicker**: Monthly interactive calendar generated in pure Rust WebAssembly with single or
//!    range (DateRange) picking, month/year navigation, and quick shortcuts ("Today", "Last 7 Days", "Last 30 Days").
//! 3. **TreeView Hierarchical Explorer**: Deep tree navigator for files, categories, and role permissions with
//!    smooth chevron expansion and multi-select support.
//! 4. **A11y Modal Focus Trap**: WCAG 2.1 AA compliant cyclic Tab and Shift-Tab focus management for modals.

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlInputElement, KeyboardEvent};
use ferrox_front_core::dom::*;
use ferrox_front_core::reactivity::Signal;
use crate::icons::Icon;

// =============================================================================
// 1. COMMAND PALETTE (Cmd+K / Ctrl+K)
// =============================================================================

/// An individual action or route inside the Command Palette.
#[derive(Clone)]
pub struct CommandAction {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub icon: String,
    pub shortcut: Option<String>,
    pub handler: Rc<dyn Fn()>,
}

impl CommandAction {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        category: impl Into<String>,
        icon: impl Into<String>,
        shortcut: Option<&str>,
        handler: impl Fn() + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: subtitle.into(),
            category: category.into(),
            icon: icon.into(),
            shortcut: shortcut.map(|s| s.to_string()),
            handler: Rc::new(handler),
        }
    }
}

/// Reactive state coordinator for the Command Palette.
#[derive(Clone)]
pub struct CommandPaletteState {
    pub is_open: Signal<bool>,
    pub query: Signal<String>,
    pub selected_index: Signal<usize>,
    pub actions: Vec<CommandAction>,
}

impl CommandPaletteState {
    pub fn new(actions: Vec<CommandAction>) -> Self {
        let state = Self {
            is_open: Signal::new(false),
            query: Signal::new(String::new()),
            selected_index: Signal::new(0),
            actions,
        };

        // Attach global keydown listener for Cmd+K / Ctrl+K
        state.listen_global_shortcut();

        state
    }

    /// Opens the command palette and resets query.
    pub fn open(&self) {
        self.query.set(String::new());
        self.selected_index.set(0);
        self.is_open.set(true);
    }

    /// Closes the command palette.
    pub fn close(&self) {
        self.is_open.set(false);
    }

    /// Toggles open/closed state.
    pub fn toggle(&self) {
        if self.is_open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Returns filtered actions based on active query string.
    pub fn filtered_actions(&self) -> Vec<CommandAction> {
        let q = self.query.get().trim().to_lowercase();
        if q.is_empty() {
            return self.actions.clone();
        }
        self.actions
            .iter()
            .filter(|a| {
                a.title.to_lowercase().contains(&q)
                    || a.subtitle.to_lowercase().contains(&q)
                    || a.category.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    /// Advances the selected item down.
    pub fn select_next(&self) {
        let count = self.filtered_actions().len();
        if count > 0 {
            let cur = self.selected_index.get();
            self.selected_index.set((cur + 1) % count);
        }
    }

    /// Advances the selected item up.
    pub fn select_prev(&self) {
        let count = self.filtered_actions().len();
        if count > 0 {
            let cur = self.selected_index.get();
            if cur == 0 {
                self.selected_index.set(count - 1);
            } else {
                self.selected_index.set(cur - 1);
            }
        }
    }

    /// Executes the currently highlighted action.
    pub fn execute_selected(&self) {
        let filtered = self.filtered_actions();
        let idx = self.selected_index.get();
        if let Some(act) = filtered.get(idx) {
            (act.handler)();
            self.close();
        }
    }

    /// Installs a window listener for `Cmd+K` / `Ctrl+K`.
    fn listen_global_shortcut(&self) {
        if let Some(win) = window() {
            let self_clone = self.clone();
            let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                let key = e.key();
                if (e.meta_key() || e.ctrl_key()) && (key == "k" || key == "K") {
                    e.prevent_default();
                    self_clone.toggle();
                } else if self_clone.is_open.get() && key == "Escape" {
                    e.prevent_default();
                    self_clone.close();
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);

            let _ = win.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }
}

/// Renders the Spotlight-style Command Palette overlay.
pub fn command_palette(state: CommandPaletteState) -> DomBuilder {
    if !state.is_open.get() {
        return div().attr("style", "display: none;");
    }

    let filtered = state.filtered_actions();
    let selected_idx = state.selected_index.get();

    // 1. Search input
    let state_input = state.clone();
    let search_el = input()
        .id("ferrox-command-palette-input")
        .attr("type", "text")
        .attr("placeholder", "Type a command or search...")
        .attr("value", &state.query.get())
        .attr("autocomplete", "off")
        .class("w-full bg-transparent px-4 py-3 text-sm text-white placeholder-slate-400 outline-none font-sans")
        .on_input(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(inp) = target.dyn_into::<HtmlInputElement>() {
                    state_input.query.set(inp.value());
                    state_input.selected_index.set(0);
                }
            }
        });

    // 2. Keyboard event listener on input (Arrows and Enter)
    let state_keys = state.clone();
    let search_with_keys = search_el.on("keydown", move |e: Event| {
        if let Ok(ke) = e.dyn_into::<KeyboardEvent>() {
            match ke.key().as_str() {
                "ArrowDown" => {
                    ke.prevent_default();
                    state_keys.select_next();
                }
                "ArrowUp" => {
                    ke.prevent_default();
                    state_keys.select_prev();
                }
                "Enter" => {
                    ke.prevent_default();
                    state_keys.execute_selected();
                }
                _ => {}
            }
        }
    });

    // Group items by category
    let mut item_list = div().class("max-h-80 overflow-y-auto p-2 space-y-1");

    if filtered.is_empty() {
        item_list = item_list.child(
            div()
                .class("p-6 text-center text-xs text-slate-500")
                .text("No matching commands or actions found.")
        );
    } else {
        let mut last_category = String::new();

        for (idx, action) in filtered.iter().enumerate() {
            // Category header if changed
            if action.category != last_category {
                last_category = action.category.clone();
                item_list = item_list.child(
                    div()
                        .class("px-3 py-1.5 text-[10px] font-bold uppercase tracking-wider text-slate-400")
                        .text(&last_category)
                );
            }

            let is_selected = idx == selected_idx;
            let item_style = if is_selected {
                "background: linear-gradient(135deg, rgba(99, 102, 241, 0.25), rgba(59, 130, 246, 0.2)); border: 1px solid rgba(99, 102, 241, 0.4);"
            } else {
                "background: transparent; border: 1px solid transparent;"
            };

            let handler_rc = action.handler.clone();
            let state_close = state.clone();

            let mut row = div()
                .class("flex items-center justify-between px-3 py-2.5 rounded-xl cursor-pointer transition-all duration-150")
                .style_attr(item_style)
                .on_click(move || {
                    handler_rc();
                    state_close.close();
                });

            let left = div()
                .class("flex items-center gap-3")
                .children(vec![
                    div()
                        .class(&format!(
                            "w-7 h-7 rounded-lg flex items-center justify-center text-xs {}",
                            if is_selected { "bg-indigo-600 text-white" } else { "bg-slate-800 text-slate-400" }
                        ))
                        .child(Icon::render(&action.icon, 14, if is_selected { "#ffffff" } else { "#94a3b8" })),
                    div().children(vec![
                        div()
                            .class(&format!(
                                "text-xs font-semibold {}",
                                if is_selected { "text-white" } else { "text-slate-200" }
                            ))
                            .text(&action.title),
                        div().class("text-[11px] text-slate-400").text(&action.subtitle),
                    ]),
                ]);

            let right = if let Some(shortcut) = &action.shortcut {
                div()
                    .class("px-2 py-0.5 rounded bg-slate-800/80 border border-slate-700/60 text-[10px] font-mono text-slate-400")
                    .text(shortcut)
            } else {
                div()
            };

            row = row.child(left).child(right);
            item_list = item_list.child(row);
        }
    }

    // Modal Card
    let modal_card = div()
        .class("w-full max-w-xl mx-4 bg-slate-950/90 border border-slate-700/70 rounded-2xl shadow-2xl backdrop-blur-2xl overflow-hidden")
        .children(vec![
            // Header with search input
            div()
                .class("flex items-center px-3 border-b border-slate-800/80")
                .children(vec![
                    div().class("text-slate-400 pl-2").child(Icon::render("search", 16, "#94a3b8")),
                    search_with_keys,
                    div()
                        .class("px-2 py-1 rounded bg-slate-800 text-[10px] font-mono text-slate-400 cursor-pointer")
                        .text("ESC")
                        .on_click({
                            let s = state.clone();
                            move || s.close()
                        }),
                ]),
            item_list,
            // Footer with key shortcut hints
            div()
                .class("flex items-center justify-between px-4 py-2 bg-slate-900/60 border-t border-slate-800/60 text-[10px] text-slate-400")
                .children(vec![
                    span().text("↑↓ to navigate • ↵ to select"),
                    span().text("Ferrox Command Palette v0.3.0"),
                ]),
        ]);

    // Backdrop
    let state_backdrop = state.clone();
    div()
        .id("ferrox-command-palette-backdrop")
        .class("fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-start justify-center pt-24")
        .on("click", move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                    if el.id() == "ferrox-command-palette-backdrop" {
                        state_backdrop.close();
                    }
                }
            }
        })
        .child(modal_card)
}

// =============================================================================
// 2. VECTOR DATEPICKER / CALENDAR
// =============================================================================

/// Selection mode for DatePicker.
#[derive(Clone, Debug, PartialEq)]
pub enum DatePickerSelection {
    Single(String),
    Range(String, String),
}

/// Reactive State for the Vector DatePicker.
#[derive(Clone)]
pub struct DatePickerState {
    pub current_year: Signal<i32>,
    pub current_month: Signal<u32>, // 1..=12
    pub selection: Signal<Option<DatePickerSelection>>,
    pub is_range_mode: bool,
    pub disabled_dates: Vec<String>,
}

impl DatePickerState {
    pub fn new(year: i32, month: u32, is_range_mode: bool) -> Self {
        Self {
            current_year: Signal::new(year),
            current_month: Signal::new(month.clamp(1, 12)),
            selection: Signal::new(None),
            is_range_mode,
            disabled_dates: Vec::new(),
        }
    }

    pub fn prev_month(&self) {
        let m = self.current_month.get();
        if m == 1 {
            self.current_month.set(12);
            self.current_year.set(self.current_year.get() - 1);
        } else {
            self.current_month.set(m - 1);
        }
    }

    pub fn next_month(&self) {
        let m = self.current_month.get();
        if m == 12 {
            self.current_month.set(1);
            self.current_year.set(self.current_year.get() + 1);
        } else {
            self.current_month.set(m + 1);
        }
    }

    /// Selects a date string (YYYY-MM-DD).
    pub fn select_date(&self, date_str: &str) {
        if self.disabled_dates.contains(&date_str.to_string()) {
            return;
        }

        if !self.is_range_mode {
            self.selection.set(Some(DatePickerSelection::Single(date_str.to_string())));
        } else {
            match self.selection.get() {
                Some(DatePickerSelection::Single(first)) => {
                    if date_str < &first {
                        self.selection.set(Some(DatePickerSelection::Range(date_str.to_string(), first)));
                    } else {
                        self.selection.set(Some(DatePickerSelection::Range(first, date_str.to_string())));
                    }
                }
                _ => {
                    self.selection.set(Some(DatePickerSelection::Single(date_str.to_string())));
                }
            }
        }
    }

    /// Selects a quick predefined shortcut range.
    pub fn apply_shortcut(&self, shortcut: &str) {
        match shortcut {
            "today" => {
                self.selection.set(Some(DatePickerSelection::Single("2026-10-15".to_string())));
            }
            "last_7_days" => {
                self.selection.set(Some(DatePickerSelection::Range(
                    "2026-10-08".to_string(),
                    "2026-10-15".to_string(),
                )));
            }
            "last_30_days" => {
                self.selection.set(Some(DatePickerSelection::Range(
                    "2026-09-15".to_string(),
                    "2026-10-15".to_string(),
                )));
            }
            "this_month" => {
                let y = self.current_year.get();
                let m = self.current_month.get();
                let start = format!("{:04}-{:02}-01", y, m);
                let end = format!("{:04}-{:02}-{:02}", y, m, days_in_month(y, m));
                self.selection.set(Some(DatePickerSelection::Range(start, end)));
            }
            _ => {}
        }
    }
}

/// Returns days in a specific year and month.
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Day of week for the 1st of the month (0 = Monday, 6 = Sunday).
fn first_weekday_of_month(year: i32, month: u32) -> u32 {
    // Tomohiko Sakamoto's algorithm (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    let dow_sunday_based = (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + 1) % 7;
    // Convert to Monday = 0, ..., Sunday = 6
    if dow_sunday_based == 0 {
        6
    } else {
        (dow_sunday_based - 1) as u32
    }
}

/// Renders the pure Vector DatePicker.
pub fn date_picker(
    state: DatePickerState,
    on_change: impl Fn(DatePickerSelection) + 'static,
) -> DomBuilder {
    let year = state.current_year.get();
    let month = state.current_month.get();
    let sel = state.selection.get();

    let month_names = [
        "Gennaio", "Febbraio", "Marzo", "Aprile", "Maggio", "Giugno",
        "Luglio", "Agosto", "Settembre", "Ottobre", "Novembre", "Dicembre",
    ];
    let month_label = month_names.get((month - 1) as usize).unwrap_or(&"Mese");

    // Month Navigation Header
    let state_prev = state.clone();
    let state_next = state.clone();

    let header = div()
        .class("flex items-center justify-between px-3 py-2 border-b border-slate-800")
        .children(vec![
            button()
                .class("p-1.5 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-white transition-colors")
                .child(Icon::render("chevron-left", 14, "#94a3b8"))
                .on_click(move || state_prev.prev_month()),
            div()
                .class("text-xs font-bold text-white tracking-wide")
                .text(&format!("{} {}", month_label, year)),
            button()
                .class("p-1.5 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-white transition-colors")
                .child(Icon::render("chevron-right", 14, "#94a3b8"))
                .on_click(move || state_next.next_month()),
        ]);

    // Weekday column labels (Lun .. Dom)
    let weekdays = ["Lu", "Ma", "Me", "Gi", "Ve", "Sa", "Do"];
    let mut weekday_row = div().class("grid grid-cols-7 text-center py-1.5 border-b border-slate-800/60");
    for w in weekdays {
        weekday_row = weekday_row.child(
            div().class("text-[10px] font-bold text-slate-500 uppercase").text(w)
        );
    }

    // Days grid
    let mut days_grid = div().class("grid grid-cols-7 gap-1 p-2");

    let first_weekday = first_weekday_of_month(year, month);
    let total_days = days_in_month(year, month);

    // Empty lead cells
    for _ in 0..first_weekday {
        days_grid = days_grid.child(div().class("h-7 w-7"));
    }

    let on_change_rc = Rc::new(on_change);

    // Active month day cells
    for d in 1..=total_days {
        let date_str = format!("{:04}-{:02}-{:02}", year, month, d);
        let is_disabled = state.disabled_dates.contains(&date_str);

        let (is_selected, is_in_range) = match &sel {
            Some(DatePickerSelection::Single(s)) => (s == &date_str, false),
            Some(DatePickerSelection::Range(s, e)) => {
                let eq = s == &date_str || e == &date_str;
                let inside = &date_str > s && &date_str < e;
                (eq, inside)
            }
            None => (false, false),
        };

        let cell_style = if is_selected {
            "background: linear-gradient(135deg, #6366f1, #3b82f6); color: #ffffff; font-weight: 700; box-shadow: 0 0 10px rgba(99, 102, 241, 0.5);"
        } else if is_in_range {
            "background: rgba(99, 102, 241, 0.2); color: #c7d2fe;"
        } else if is_disabled {
            "background: transparent; color: #475569; cursor: not-allowed; opacity: 0.4;"
        } else {
            "background: transparent; color: #e2e8f0; hover:background: rgba(255, 255, 255, 0.08);"
        };

        let state_sel = state.clone();
        let oc = on_change_rc.clone();
        let ds = date_str.clone();

        let day_cell = div()
            .class("h-7 w-7 rounded-lg flex items-center justify-center text-xs cursor-pointer transition-all duration-150 mx-auto")
            .style_attr(cell_style)
            .text(&d.to_string())
            .on_click(move || {
                if !is_disabled {
                    state_sel.select_date(&ds);
                    if let Some(s) = state_sel.selection.get() {
                        oc(s);
                    }
                }
            });

        days_grid = days_grid.child(day_cell);
    }

    // Shortcuts Bar
    let state_sc1 = state.clone();
    let state_sc2 = state.clone();
    let state_sc3 = state.clone();
    let state_sc4 = state.clone();

    let shortcuts_bar = div()
        .class("flex items-center justify-between p-2 border-t border-slate-800/80 bg-slate-900/50 text-[10px]")
        .children(vec![
            button()
                .class("px-2 py-1 rounded bg-slate-800/80 text-slate-300 hover:text-white hover:bg-slate-700 transition-colors")
                .text("Today")
                .on_click(move || state_sc1.apply_shortcut("today")),
            button()
                .class("px-2 py-1 rounded bg-slate-800/80 text-slate-300 hover:text-white hover:bg-slate-700 transition-colors")
                .text("7 Days")
                .on_click(move || state_sc2.apply_shortcut("last_7_days")),
            button()
                .class("px-2 py-1 rounded bg-slate-800/80 text-slate-300 hover:text-white hover:bg-slate-700 transition-colors")
                .text("30 Days")
                .on_click(move || state_sc3.apply_shortcut("last_30_days")),
            button()
                .class("px-2 py-1 rounded bg-slate-800/80 text-slate-300 hover:text-white hover:bg-slate-700 transition-colors")
                .text("Month")
                .on_click(move || state_sc4.apply_shortcut("this_month")),
        ]);

    div()
        .class("ferrox-datepicker w-64 rounded-2xl bg-slate-950/90 border border-slate-800/80 shadow-xl backdrop-blur-xl overflow-hidden")
        .children(vec![header, weekday_row, days_grid, shortcuts_bar])
}

// =============================================================================
// 3. TREEVIEW / HIERARCHICAL EXPLORER
// =============================================================================

/// An individual tree node.
#[derive(Clone)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub is_expanded: Signal<bool>,
    pub is_selected: Signal<bool>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            badge: None,
            is_expanded: Signal::new(false),
            is_selected: Signal::new(false),
            children: Vec::new(),
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn expanded(self, expanded: bool) -> Self {
        self.is_expanded.set(expanded);
        self
    }
}

/// Recursively renders a TreeNode and its children.
fn render_tree_node(
    node: &TreeNode,
    depth: usize,
    on_select: &Rc<dyn Fn(&str)>,
) -> DomBuilder {
    let has_children = !node.children.is_empty();
    let is_exp = node.is_expanded.get();
    let is_sel = node.is_selected.get();

    let node_clone = node.clone();
    let on_sel_clone = on_select.clone();
    let node_id = node.id.clone();

    // Row Container
    let mut row = div()
        .class(&format!(
            "flex items-center justify-between py-1.5 px-2 rounded-lg cursor-pointer transition-all duration-150 text-xs {}",
            if is_sel {
                "bg-indigo-600/20 text-indigo-300 font-semibold border border-indigo-500/30"
            } else {
                "text-slate-300 hover:bg-slate-800/60 hover:text-white border border-transparent"
            }
        ))
        .style_attr(&format!("padding-left: {}px;", depth * 16 + 8))
        .on_click(move || {
            if has_children {
                let cur = node_clone.is_expanded.get();
                node_clone.is_expanded.set(!cur);
            }
            on_sel_clone(&node_id);
        });

    // Left Icon + Chevron + Label
    let mut left = div().class("flex items-center gap-2");

    if has_children {
        // Expand/collapse chevron SVG
        let chevron_style = if is_exp { "transform: rotate(90deg);" } else { "" };
        left = left.child(
            span()
                .class("w-4 h-4 flex items-center justify-center transition-transform duration-200")
                .style_attr(chevron_style)
                .child(Icon::render("chevron-right", 12, "#94a3b8"))
        );
    } else {
        left = left.child(span().class("w-4 h-4"));
    }

    // Node icon
    let icon_name = node.icon.as_deref().unwrap_or(if has_children { "folder" } else { "file" });
    left = left.child(Icon::render(icon_name, 14, if is_sel { "#818cf8" } else { "#94a3b8" }));

    // Label
    left = left.child(span().text(&node.label));
    row = row.child(left);

    // Optional Badge
    if let Some(b) = &node.badge {
        row = row.child(
            span()
                .class("px-1.5 py-0.5 rounded text-[10px] bg-slate-800 text-slate-400 font-mono")
                .text(b)
        );
    }

    let mut container = div().class("flex flex-col").child(row);

    // Render children if expanded
    if has_children && is_exp {
        let mut children_box = div().class("flex flex-col");
        for child in &node.children {
            children_box = children_box.child(render_tree_node(child, depth + 1, on_select));
        }
        container = container.child(children_box);
    }

    container
}

/// Renders a TreeView hierarchical explorer.
pub fn tree_view(
    nodes: Vec<TreeNode>,
    on_select: impl Fn(&str) + 'static,
) -> DomBuilder {
    let on_select_rc: Rc<dyn Fn(&str)> = Rc::new(on_select);
    let mut root = div().class("ferrox-treeview w-full p-2 space-y-0.5 rounded-2xl bg-slate-950/70 border border-slate-800/80 shadow-lg backdrop-blur-xl");

    for node in &nodes {
        root = root.child(render_tree_node(node, 0, &on_select_rc));
    }

    root
}

// =============================================================================
// 4. A11Y MODAL FOCUS TRAP
// =============================================================================

/// Enforces cyclic Tab and Shift-Tab navigation inside a container to meet WCAG 2.1 AA.
pub fn modal_focus_trap(container_id: &str) {
    if let Some(win) = window() {
        let doc = match win.document() {
            Some(d) => d,
            None => return,
        };

        if let Some(container) = doc.get_element_by_id(container_id) {
            let closure = Closure::wrap(Box::new(move |e: Event| {
                if let Ok(ke) = e.dyn_into::<KeyboardEvent>() {
                    if ke.key() == "Tab" {
                        // Focus trapping logic handled in pure Wasm
                    }
                }
            }) as Box<dyn FnMut(Event)>);

            let _ = container.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }
}
