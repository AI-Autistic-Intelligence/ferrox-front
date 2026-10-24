//! # Documentation Components & Design System Primitives (`ferrox-front-ui::doc_components`)
//!
//! Provides enterprise-grade, reusable documentation UI components built in pure Rust WebAssembly:
//! 1. **Admonition / Callout**: Glassmorphic alert callouts (`Note`, `Tip`, `Warning`, `Danger`, `Caution`, `Info`).
//! 2. **CodeBlock**: Interactive code blocks with language badge, optional filename, and native Wasm clipboard copy.
//! 3. **TabGroup & TabPanel**: Reactive multi-panel tab switchers (e.g., Cargo / npm / pip / composer code blocks).
//! 4. **TableOfContents (TOC)**: Right-hand sidebar navigation with scrollspy tracking and smooth scrolling.
//! 5. **DocSidebar & NavTreeView**: Hierarchical collapsible documentation sidebar with category folding and active link tracking.

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, ScrollIntoViewOptions, ScrollBehavior};
use ferrox_front_core::dom::*;
use ferrox_front_core::reactivity::Signal;
use crate::icons::Icon;

// =============================================================================
// 1. ADMONITION / CALLOUT
// =============================================================================

/// Varieties of callout / admonition boxes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmonitionType {
    Note,
    Tip,
    Warning,
    Danger,
    Caution,
    Info,
}

impl AdmonitionType {
    pub fn default_title(&self) -> &'static str {
        match self {
            Self::Note => "NOTE",
            Self::Tip => "TIP",
            Self::Warning => "WARNING",
            Self::Danger => "DANGER",
            Self::Caution => "CAUTION",
            Self::Info => "INFO",
        }
    }

    /// Alias for default_title
    pub fn title(&self) -> &'static str {
        self.default_title()
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Note => "info",
            Self::Tip => "zap",
            Self::Warning => "alert-triangle",
            Self::Danger => "alert-circle",
            Self::Caution => "shield",
            Self::Info => "info",
        }
    }

    /// Color tokens: (accent_border, background_tint, text_color, icon_color)
    pub fn style_tokens(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Note => ("#6366f1", "rgba(99, 102, 241, 0.08)", "#c7d2fe", "#818cf8"),
            Self::Tip => ("#10b981", "rgba(16, 185, 129, 0.08)", "#a7f3d0", "#34d399"),
            Self::Warning => ("#f59e0b", "rgba(245, 158, 11, 0.08)", "#fde68a", "#fbbf24"),
            Self::Danger => ("#ef4444", "rgba(239, 68, 68, 0.08)", "#fecaca", "#f87171"),
            Self::Caution => ("#f97316", "rgba(249, 115, 22, 0.08)", "#fed7aa", "#fb923c"),
            Self::Info => ("#06b6d4", "rgba(6, 182, 212, 0.08)", "#a5f3fc", "#22d3ee"),
        }
    }

    pub fn border_color(&self) -> &'static str {
        self.style_tokens().0
    }

    pub fn bg_tint(&self) -> &'static str {
        self.style_tokens().1
    }

    pub fn text_color(&self) -> &'static str {
        self.style_tokens().2
    }

    pub fn icon_color(&self) -> &'static str {
        self.style_tokens().3
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "tip" => Self::Tip,
            "warning" => Self::Warning,
            "danger" | "error" => Self::Danger,
            "caution" => Self::Caution,
            "info" => Self::Info,
            _ => Self::Note,
        }
    }
}

/// Renders a design system Admonition / Callout box with glassmorphism styling.
pub fn admonition(
    kind: AdmonitionType,
    title: Option<&str>,
    content: DomBuilder,
) -> DomBuilder {
    let (border_col, bg_tint, text_col, icon_col) = kind.style_tokens();
    let display_title = title.unwrap_or_else(|| kind.default_title());

    let header = div()
        .class("ferrox-admonition-header flex items-center gap-2 mb-2 font-bold text-xs tracking-wider uppercase")
        .style_attr(&format!("color: {};", text_col))
        .child(Icon::render(kind.icon_name(), 16, icon_col))
        .child(span().text(display_title));

    let body = div()
        .class("ferrox-admonition-body text-sm leading-relaxed text-slate-300")
        .child(content);

    div()
        .class("ferrox-admonition my-4 p-3.5 sm:p-4 rounded-xl backdrop-blur-md transition-all duration-200 text-xs sm:text-sm")
        .style_attr(&format!(
            "background: {}; border: 1px solid rgba(255,255,255,0.06); border-left: 4px solid {}; box-shadow: 0 4px 20px -2px rgba(0,0,0,0.25);",
            bg_tint, border_col
        ))
        .child(header)
        .child(body)
}

pub fn admonition_note(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Note, title, content)
}

pub fn admonition_tip(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Tip, title, content)
}

pub fn admonition_warning(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Warning, title, content)
}

pub fn admonition_danger(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Danger, title, content)
}

pub fn admonition_caution(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Caution, title, content)
}

pub fn admonition_info(title: Option<&str>, content: DomBuilder) -> DomBuilder {
    admonition(AdmonitionType::Info, title, content)
}

// =============================================================================
// 2. CODE BLOCK WITH NATIVE WASM CLIPBOARD COPY
// =============================================================================

/// Copies text to clipboard using the browser navigator.clipboard Web API.
pub fn copy_to_clipboard(text: &str) {
    if let Some(win) = window() {
        let nav = win.navigator();
        let clip = nav.clipboard();
        let _ = clip.write_text(text);
    }
}

/// Interactive CodeBlock with language badge, optional filename, and live copy-to-clipboard feedback.
pub fn code_block(
    code: &str,
    lang: Option<&str>,
    title: Option<&str>,
) -> DomBuilder {
    code_block_highlighted(None, code, lang, title)
}

/// Interactive CodeBlock supporting pre-rendered syntax-highlighted HTML or plain formatted code.
pub fn code_block_highlighted(
    highlighted_html: Option<&str>,
    raw_code: &str,
    lang: Option<&str>,
    title: Option<&str>,
) -> DomBuilder {
    let raw_code_owned = raw_code.to_string();
    let copied_signal = Signal::new(false);
    let lang_display = lang.unwrap_or("text").to_uppercase();

    // Unique ID for copy button
    let btn_id = format!("copy-btn-{}", (js_sys::Math::random() * 1_000_000.0) as u32);
    let btn_id_clone = btn_id.clone();
    let code_to_copy = raw_code_owned.clone();
    let copied_signal_clone = copied_signal.clone();

    // Copy button
    let copy_btn = button()
        .id(&btn_id)
        .class("ferrox-code-copy-btn flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-semibold text-slate-400 hover:text-white bg-slate-800/60 hover:bg-slate-700/80 border border-slate-700/50 transition-all duration-150 cursor-pointer user-select-none")
        .children(vec![
            Icon::render("copy", 13, "#94a3b8"),
            span().text("Copy"),
        ])
        .on_click(move || {
            copy_to_clipboard(&code_to_copy);
            copied_signal_clone.set(true);

            // Directly update button label for instantaneous user feedback
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id(&btn_id_clone) {
                    el.set_inner_html(
                        r##"<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="ferrox-icon" style="display: inline-block; vertical-align: middle;"><polyline points="20 6 9 17 4 12"></polyline></svg><span style="color: #10b981;">Copied!</span>"##
                    );
                    
                    let id_restore = btn_id_clone.clone();
                    let cb = Closure::wrap(Box::new(move || {
                        if let Some(d) = window().and_then(|w| w.document()) {
                            if let Some(b) = d.get_element_by_id(&id_restore) {
                                b.set_inner_html(
                                    r##"<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ferrox-icon" style="display: inline-block; vertical-align: middle;"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg><span>Copy</span>"##
                                );
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    
                    if let Some(w) = window() {
                        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 2000);
                        cb.forget();
                    }
                }
            }
        });

    // Top Header Bar
    let mut header_left = div().class("flex items-center gap-2");
    
    // Optional window control dots
    let dots = div()
        .class("flex items-center gap-1.5 mr-2")
        .children(vec![
            span().class("w-2.5 h-2.5 rounded-full bg-rose-500/80 inline-block"),
            span().class("w-2.5 h-2.5 rounded-full bg-amber-500/80 inline-block"),
            span().class("w-2.5 h-2.5 rounded-full bg-emerald-500/80 inline-block"),
        ]);
    header_left = header_left.child(dots);

    if let Some(t) = title {
        header_left = header_left.child(
            span().class("text-xs font-mono font-medium text-slate-300").text(t)
        );
    }

    // Language Badge
    let lang_badge = span()
        .class("px-2 py-0.5 rounded text-[10px] font-mono font-bold tracking-wider bg-slate-800 text-indigo-300 border border-indigo-500/20")
        .text(&lang_display);

    let header = div()
        .class("ferrox-code-header flex flex-wrap sm:flex-nowrap items-center justify-between px-3 sm:px-4 py-2 border-b border-slate-800/80 bg-slate-900/70 gap-2")
        .children(vec![
            header_left,
            div().class("flex items-center gap-2 shrink-0").child(lang_badge).child(copy_btn),
        ]);

    // Code Content
    let code_el = if let Some(html) = highlighted_html {
        div()
            .class("p-3.5 sm:p-5 overflow-x-auto text-xs sm:text-sm md:text-base font-mono leading-relaxed text-slate-200 whitespace-pre")
            .inner_html(html)
    } else {
        div()
            .class("p-3.5 sm:p-5 overflow-x-auto text-xs sm:text-sm md:text-base font-mono leading-relaxed text-slate-200 whitespace-pre")
            .text(&raw_code_owned)
    };

    div()
        .class("ferrox-code-block my-6 rounded-2xl overflow-hidden bg-slate-950/80 border border-slate-800/80 shadow-2xl backdrop-blur-xl transition-all duration-200")
        .child(header)
        .child(code_el)
}

// =============================================================================
// 3. REACTIVE TAB GROUP & TAB PANELS
// =============================================================================

/// Single tab panel definition.
pub struct TabPanelDef {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub content: DomBuilder,
}

impl TabPanelDef {
    pub fn new(id: impl Into<String>, label: impl Into<String>, content: DomBuilder) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            content,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

static TAB_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1000);

/// Renders a dynamic, reactive TabGroup for multi-panel code snippets or documentation sections.
pub fn tab_group(panels: Vec<TabPanelDef>, default_index: usize) -> DomBuilder {
    let group_id = format!("tabgroup-{}", TAB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    let active_index = Signal::new(default_index.min(panels.len().saturating_sub(1)));

    let mut header_bar = div()
        .class("ferrox-tabgroup-header flex items-center gap-1.5 p-1 sm:p-1.5 rounded-xl bg-slate-900/60 border border-slate-800/80 mb-3 backdrop-blur-md overflow-x-auto max-w-full scrollbar-none");

    let mut panels_container = div().class("ferrox-tabgroup-panels w-full");

    let count = panels.len();

    for (idx, panel) in panels.into_iter().enumerate() {
        let is_active = idx == active_index.get();
        let idx_owned = idx;
        let group_id_clone = group_id.clone();

        let mut tab_btn = button()
            .id(&format!("{}-tab-{}", group_id, idx))
            .class(&format!(
                "shrink-0 flex items-center gap-1.5 sm:gap-2 px-3 sm:px-3.5 py-1.5 rounded-lg text-xs font-semibold whitespace-nowrap transition-all duration-150 cursor-pointer {}",
                if is_active {
                    "bg-indigo-600 text-white shadow-md font-bold"
                } else {
                    "text-slate-400 hover:text-white hover:bg-slate-800/60"
                }
            ))
            .on_click(move || {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    for i in 0..count {
                        if let Some(btn) = doc.get_element_by_id(&format!("{}-tab-{}", group_id_clone, i)) {
                            if i == idx_owned {
                                btn.set_attribute("class", "flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold bg-indigo-600 text-white shadow-md cursor-pointer transition-all duration-150").unwrap();
                            } else {
                                btn.set_attribute("class", "flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-semibold text-slate-400 hover:text-white hover:bg-slate-800/60 cursor-pointer transition-all duration-150").unwrap();
                            }
                        }
                        if let Some(p) = doc.get_element_by_id(&format!("{}-panel-{}", group_id_clone, i)) {
                            if i == idx_owned {
                                p.set_attribute("style", "display: block;").unwrap();
                            } else {
                                p.set_attribute("style", "display: none;").unwrap();
                            }
                        }
                    }
                }
            });

        if let Some(ref ico) = panel.icon {
            tab_btn = tab_btn.child(Icon::render(ico, 14, if is_active { "#ffffff" } else { "#94a3b8" }));
        }

        tab_btn = tab_btn.child(span().text(&panel.label));
        header_bar = header_bar.child(tab_btn);

        // Panel Body
        let panel_body = div()
            .id(&format!("{}-panel-{}", group_id, idx))
            .class("ferrox-tab-panel w-full transition-opacity duration-150")
            .style_attr(if is_active { "display: block;" } else { "display: none;" })
            .child(panel.content);

        panels_container = panels_container.child(panel_body);
    }

    div()
        .id(&group_id)
        .class("ferrox-tab-group my-4 w-full")
        .child(header_bar)
        .child(panels_container)
}

// =============================================================================
// 4. TABLE OF CONTENTS (TOC) WITH SCROLLSPY
// =============================================================================

/// Single Table of Contents Heading item.
#[derive(Debug, Clone)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: u32,
}

impl TocItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>, level: u32) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            level,
        }
    }
}

/// Smoothly scrolls the window to an element by its HTML anchor ID.
pub fn scroll_to_anchor(id: &str) {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(el) = doc.get_element_by_id(id) {
                let options = ScrollIntoViewOptions::new();
                options.set_behavior(ScrollBehavior::Smooth);
                el.scroll_into_view_with_scroll_into_view_options(&options);
                
                // Update URL hash without causing a page jump
                if let Ok(hist) = win.history() {
                    let _ = hist.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{}", id)));
                }
            }
        }
    }
}

/// Dynamic Table of Contents component with active anchor highlight and smooth scrolling.
pub fn table_of_contents(
    items: Vec<TocItem>,
    active_id: Signal<Option<String>>,
) -> DomBuilder {
    let mut nav_list = div().class("flex flex-col space-y-1.5 text-sm font-medium");

    let cur_active = active_id.get();

    for item in items {
        let is_active = cur_active.as_deref() == Some(&item.id);
        let id_owned = item.id.clone();
        let indent_class = match item.level {
            1 | 2 => "pl-3",
            3 => "pl-5",
            4 => "pl-7",
            _ => "pl-9",
        };

        let link_class = if is_active {
            "text-indigo-400 font-semibold border-l-2 border-indigo-500 -ml-[1px]"
        } else {
            "text-slate-400 hover:text-slate-200 border-l-2 border-transparent -ml-[1px]"
        };

        let anchor_link = div()
            .class(&format!(
                "py-1.5 {} {} transition-all duration-150 cursor-pointer flex items-center justify-between",
                indent_class, link_class
            ))
            .child(span().class("break-words leading-snug").text(&item.title))
            .on_click(move || {
                scroll_to_anchor(&id_owned);
            });

        nav_list = nav_list.child(anchor_link);
    }

    div()
        .class("ferrox-toc w-full py-4 px-3 border-l border-slate-800/80")
        .children(vec![
            div()
                .class("flex items-center gap-2 mb-3.5 text-xs font-bold uppercase tracking-wider text-slate-400")
                .children(vec![
                    Icon::render("layers", 15, "#818cf8"),
                    span().text("On this page"),
                ]),
            nav_list,
        ])
}

/// Renders an in-page, collapsible mobile Table of Contents accordion card.
/// Automatically hidden on desktop (xl:hidden) and expanded/collapsed on tap.
pub fn mobile_table_of_contents(
    items: Vec<TocItem>,
    _active_id: Signal<Option<String>>,
) -> DomBuilder {
    if items.is_empty() {
        return div().class("hidden");
    }

    let items_len = items.len();
    let toc_list_id = "ferrox-mobile-toc-list";
    let toc_chev_id = "ferrox-mobile-toc-chev";

    let mut nav_list = div()
        .id(toc_list_id)
        .class("hidden flex-col space-y-1 pt-2 pb-1 border-t border-slate-800/80 text-xs sm:text-sm");

    for item in items {
        let id_owned = item.id.clone();
        let indent = match item.level {
            1 | 2 => "pl-2",
            3 => "pl-4",
            4 => "pl-6",
            _ => "pl-8",
        };

        let link = div()
            .class(&format!(
                "py-1.5 {} text-slate-300 hover:text-white hover:bg-slate-800/50 rounded-lg transition-colors cursor-pointer flex items-center gap-1.5",
                indent
            ))
            .child(Icon::render("chevron-right", 12, "#818cf8"))
            .child(span().class("truncate font-medium").text(&item.title))
            .on_click(move || {
                scroll_to_anchor(&id_owned);
                // Collapse the mobile TOC dropdown after tapping a section
                if let Some(doc) = window().and_then(|w| w.document()) {
                    if let Some(list_el) = doc.get_element_by_id(toc_list_id) {
                        let _ = list_el.set_attribute("class", "hidden flex-col space-y-1 pt-2 pb-1 border-t border-slate-800/80 text-xs sm:text-sm");
                    }
                    if let Some(chev_el) = doc.get_element_by_id(toc_chev_id) {
                        let _ = chev_el.set_attribute("style", "transform: rotate(0deg);");
                    }
                }
            });

        nav_list = nav_list.child(link);
    }

    let header = div()
        .class("flex items-center justify-between py-2 px-3.5 cursor-pointer hover:bg-slate-800/40 select-none transition-colors")
        .children(vec![
            div().class("flex items-center gap-2").children(vec![
                Icon::render("layers", 14, "#818cf8"),
                span().class("text-xs font-bold uppercase tracking-wider text-slate-200").text("On this page"),
                span().class("text-[10px] font-mono text-indigo-400 font-semibold").text(&format!("({} sections)", items_len)),
            ]),
            span()
                .id(toc_chev_id)
                .class("w-4 h-4 flex items-center justify-center transition-transform duration-200")
                .child(Icon::render("chevron-down", 14, "#94a3b8")),
        ])
        .on_click(move || {
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(list_el) = doc.get_element_by_id(toc_list_id) {
                    let cur_class = list_el.get_attribute("class").unwrap_or_default();
                    if cur_class.contains("hidden") {
                        let _ = list_el.set_attribute("class", "flex flex-col space-y-1 pt-2 pb-1 border-t border-slate-800/80 text-xs sm:text-sm");
                        if let Some(chev_el) = doc.get_element_by_id(toc_chev_id) {
                            let _ = chev_el.set_attribute("style", "transform: rotate(180deg);");
                        }
                    } else {
                        let _ = list_el.set_attribute("class", "hidden flex-col space-y-1 pt-2 pb-1 border-t border-slate-800/80 text-xs sm:text-sm");
                        if let Some(chev_el) = doc.get_element_by_id(toc_chev_id) {
                            let _ = chev_el.set_attribute("style", "transform: rotate(0deg);");
                        }
                    }
                }
            }
        });

    div()
        .class("ferrox-mobile-toc xl:hidden mb-6 rounded-xl bg-slate-900/60 border border-slate-800/90 backdrop-blur-md overflow-hidden shadow-lg transition-all")
        .child(header)
        .child(nav_list)
}

// =============================================================================
// 5. DOCUMENTATION SIDEBAR (NAV TREE VIEW)
// =============================================================================

/// Single navigation item inside the doc sidebar.
#[derive(Clone)]
pub struct DocNavItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub is_expanded: Signal<bool>,
    pub children: Vec<DocNavItem>,
}

impl DocNavItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: None,
            icon: None,
            badge: None,
            is_expanded: Signal::new(false),
            children: Vec::new(),
        }
    }

    pub fn has_active_child(&self, current_path: &str) -> bool {
        let clean_cur = current_path.trim_start_matches('/');
        if let Some(ref h) = self.href {
            if h.trim_start_matches('/') == clean_cur {
                return true;
            }
        }
        for child in &self.children {
            if child.has_active_child(current_path) {
                return true;
            }
        }
        false
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_children(mut self, children: Vec<DocNavItem>) -> Self {
        self.children = children;
        self
    }

    pub fn expanded(self, exp: bool) -> Self {
        self.is_expanded.set(exp);
        self
    }
}

/// Recursively renders a documentation navigation item.
fn render_doc_nav_item(
    item: &DocNavItem,
    current_path: &str,
    depth: usize,
    on_navigate: &Rc<dyn Fn(&str)>,
) -> DomBuilder {
    let has_children = !item.children.is_empty();
    let clean_cur = current_path.trim_start_matches('/');

    // On home /overview, all menus start closed!
    // When navigating to a specific document inside a section, keep that branch expanded.
    let has_active = if clean_cur.is_empty() || clean_cur == "overview" {
        false
    } else {
        item.has_active_child(current_path)
    };

    let is_exp = item.is_expanded.get() || has_active;
    let is_active = item.href.as_deref().map(|h| h.trim_start_matches('/') == clean_cur).unwrap_or(false);

    let on_nav_clone = on_navigate.clone();
    let href_opt = item.href.clone();

    // Unique DOM IDs for this expandable category
    let clean_id = item.id.replace('/', "-").replace(' ', "-").replace(':', "-");
    let box_id = format!("doc-nav-box-{}", clean_id);
    let chev_id = format!("doc-nav-chev-{}", clean_id);
    let box_id_clone = box_id.clone();
    let chev_id_clone = chev_id.clone();

    let (row_class, icon_size) = if depth == 0 {
        (
            format!(
                "flex items-center justify-between min-h-[40px] py-2 px-3 rounded-xl cursor-pointer transition-all duration-150 text-[14px] sm:text-[15px] font-semibold tracking-tight {}",
                if is_active {
                    "bg-indigo-600/25 text-indigo-300 border border-indigo-500/40 shadow-sm"
                } else {
                    "text-slate-200 hover:text-white hover:bg-slate-900/90 border border-transparent"
                }
            ),
            16,
        )
    } else if has_children {
        (
            format!(
                "flex items-center justify-between min-h-[36px] py-1.5 px-2.5 rounded-lg cursor-pointer transition-all duration-150 text-xs sm:text-sm font-medium {}",
                if is_active {
                    "bg-indigo-600/20 text-indigo-300 font-semibold border border-indigo-500/30 shadow-sm"
                } else {
                    "text-slate-300 hover:text-white hover:bg-slate-800/70 border border-transparent"
                }
            ),
            15,
        )
    } else {
        (
            format!(
                "flex items-center justify-between min-h-[34px] py-1.5 px-2.5 rounded-lg cursor-pointer transition-all duration-150 text-xs sm:text-sm {}",
                if is_active {
                    "bg-indigo-600/25 text-indigo-300 font-semibold border border-indigo-500/30 shadow-sm"
                } else {
                    "text-slate-400 hover:text-slate-100 hover:bg-slate-800/60 border border-transparent font-normal"
                }
            ),
            14,
        )
    };

    let mut row = div()
        .class(&row_class)
        .style_attr(&format!("padding-left: {}px;", depth * 10 + 10))
        .on_click(move || {
            if has_children {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    if let Some(b_el) = doc.get_element_by_id(&box_id_clone) {
                        let style_str = b_el.get_attribute("style").unwrap_or_default();
                        if style_str.contains("display: none") {
                            let _ = b_el.set_attribute("style", "display: flex; flex-direction: column;");
                            if let Some(c_el) = doc.get_element_by_id(&chev_id_clone) {
                                let _ = c_el.set_attribute("style", "transform: rotate(90deg);");
                            }
                        } else {
                            let _ = b_el.set_attribute("style", "display: none;");
                            if let Some(c_el) = doc.get_element_by_id(&chev_id_clone) {
                                let _ = c_el.set_attribute("style", "transform: rotate(0deg);");
                            }
                        }
                    }
                }
            }
            if let Some(ref h) = href_opt {
                on_nav_clone(h);
            }
        });

    let mut left = div().class("flex items-center gap-2.5 min-w-0");

    if has_children {
        let chevron_style = if is_exp { "transform: rotate(90deg);" } else { "transform: rotate(0deg);" };
        left = left.child(
            span()
                .id(&chev_id)
                .class("w-4 h-4 flex items-center justify-center shrink-0 transition-transform duration-150")
                .style_attr(chevron_style)
                .child(Icon::render("chevron-right", 14, "#94a3b8"))
        );
    } else {
        left = left.child(span().class("w-4 h-4 shrink-0"));
    }

    if let Some(ref ico) = item.icon {
        left = left.child(Icon::render(ico, icon_size, if is_active { "#818cf8" } else { "#94a3b8" }));
    }

    left = left.child(span().class("truncate").text(&item.label));
    row = row.child(left);

    if let Some(ref b) = item.badge {
        row = row.child(
            span()
                .class("shrink-0 ml-1.5 px-1.5 py-0.5 rounded text-[10px] font-mono font-bold bg-slate-800 text-indigo-300 border border-indigo-500/25 shadow-sm")
                .text(b)
        );
    }

    let mut container = div().class("flex flex-col").child(row);

    if has_children {
        let initial_display = if is_exp { "display: flex; flex-direction: column;" } else { "display: none;" };
        let mut children_box = div()
            .id(&box_id)
            .class("space-y-0.5 mt-0.5")
            .style_attr(initial_display);
        for child in &item.children {
            children_box = children_box.child(render_doc_nav_item(child, current_path, depth + 1, on_navigate));
        }
        container = container.child(children_box);
    }

    container
}

/// Renders a full collapsible documentation navigation sidebar.
pub fn doc_sidebar(
    items: Vec<DocNavItem>,
    current_path: Signal<String>,
    on_navigate: impl Fn(&str) + 'static,
) -> DomBuilder {
    let on_nav_rc: Rc<dyn Fn(&str)> = Rc::new(on_navigate);
    let cur = current_path.get();

    let mut root = div().class("ferrox-doc-sidebar w-full flex flex-col space-y-1 p-2");

    for item in &items {
        root = root.child(render_doc_nav_item(item, &cur, 0, &on_nav_rc));
    }

    root
}

// =============================================================================
// 6. MERMAID DIAGRAM CONTAINER
// =============================================================================

/// Renders a dedicated interactive Mermaid vector SVG container with pan, zoom, and fullscreen capabilities.
pub fn mermaid_container(raw_code: &str, title: Option<&str>) -> DomBuilder {
    let title_str = title.unwrap_or("Mermaid SVG Vector Diagram");

    div()
        .class("ferrox-mermaid group relative my-6 flex flex-col items-center justify-center p-3 sm:p-4 md:p-5 rounded-2xl bg-slate-900/70 border border-slate-800/90 shadow-2xl backdrop-blur-xl transition-all duration-200 max-w-full")
        .children(vec![
            // Header with title and interactive toolbar controls
            div()
                .class("w-full flex flex-wrap items-center justify-between pb-3 mb-3 border-b border-slate-800/80 text-xs font-mono text-indigo-400 font-semibold gap-2")
                .children(vec![
                    span().class("flex items-center gap-1.5").children(vec![
                        span().text("⚡"),
                        span().text(title_str),
                    ]),
                    div().class("flex items-center gap-1.5 flex-wrap").children(vec![
                        // Zoom percentage badge
                        span()
                            .class("ferrox-mermaid-zoom-badge px-2 py-0.5 rounded text-[10px] bg-slate-950/80 text-cyan-300 border border-cyan-500/30 font-mono shadow-inner")
                            .text("100%"),
                        // Zoom in button
                        button()
                            .class("ferrox-mermaid-btn ferrox-mermaid-zoom-in px-2.5 py-1 rounded-lg bg-slate-800 hover:bg-slate-700 active:scale-95 text-slate-200 border border-slate-700/80 text-xs font-bold transition-all cursor-pointer shadow-sm")
                            .attr("title", "Zoom In (+25%)")
                            .attr("aria-label", "Zoom In")
                            .text("+"),
                        // Zoom out button
                        button()
                            .class("ferrox-mermaid-btn ferrox-mermaid-zoom-out px-2.5 py-1 rounded-lg bg-slate-800 hover:bg-slate-700 active:scale-95 text-slate-200 border border-slate-700/80 text-xs font-bold transition-all cursor-pointer shadow-sm")
                            .attr("title", "Zoom Out (-25%)")
                            .attr("aria-label", "Zoom Out")
                            .text("-"),
                        // Reset button
                        button()
                            .class("ferrox-mermaid-btn ferrox-mermaid-reset px-2.5 py-1 rounded-lg bg-slate-800 hover:bg-slate-700 active:scale-95 text-slate-200 border border-slate-700/80 text-xs transition-all cursor-pointer shadow-sm flex items-center gap-1")
                            .attr("title", "Reset Zoom & Pan")
                            .attr("aria-label", "Reset Zoom and Pan")
                            .text("↺ Reset"),
                        // Fullscreen toggle button
                        button()
                            .class("ferrox-mermaid-btn ferrox-mermaid-fullscreen px-2.5 py-1 rounded-lg bg-indigo-950/60 hover:bg-indigo-900/80 active:scale-95 text-indigo-300 border border-indigo-500/40 text-xs transition-all cursor-pointer shadow-sm flex items-center gap-1")
                            .attr("title", "Toggle Fullscreen View")
                            .attr("aria-label", "Toggle Fullscreen View")
                            .text("⛶ Expand"),
                        span()
                            .class("hidden sm:inline-block px-2 py-0.5 rounded text-[10px] bg-slate-800/80 text-indigo-300 border border-indigo-500/20")
                            .text("Vector SVG"),
                    ]),
                ]),
            // Interactive Viewport with Pan & Zoom Canvas
            div()
                .class("ferrox-mermaid-viewport relative w-full overflow-hidden rounded-xl bg-slate-950/60 border border-slate-800/60 flex items-center justify-center min-h-[240px] select-none")
                .attr("style", "touch-action: none; cursor: grab;")
                .children(vec![
                    div()
                        .class("ferrox-mermaid-canvas w-full flex items-center justify-center p-2")
                        .attr("style", "transform-origin: center center; will-change: transform;")
                        .children(vec![
                            div()
                                .class("ferrox-mermaid-raw text-slate-200 text-sm font-mono w-full flex justify-center")
                                .text(raw_code),
                        ]),
                    // Interactive navigation hint overlay
                    div()
                        .class("ferrox-mermaid-hint absolute bottom-2 right-2 text-[10px] font-mono text-slate-400 bg-slate-900/90 px-2.5 py-1 rounded-md border border-slate-800 pointer-events-none opacity-50 group-hover:opacity-100 transition-opacity backdrop-blur-sm")
                        .text("Drag to pan • Scroll to zoom"),
                ]),
        ])
}

// =============================================================================
// 7. INSTALLATION TABS SWITCHER
// =============================================================================

/// Builds a ready-made interactive installation tabs switcher (Cargo / npm / pip / composer).
pub fn installation_tabs(
    cargo: Option<&str>,
    npm: Option<&str>,
    pip: Option<&str>,
    composer: Option<&str>,
) -> DomBuilder {
    let mut panels = Vec::new();

    if let Some(cmd) = cargo {
        panels.push(
            TabPanelDef::new(
                "install-cargo",
                "Cargo (Rust)",
                code_block(cmd, Some("bash"), Some("Cargo")),
            )
            .with_icon("terminal")
        );
    }

    if let Some(cmd) = npm {
        panels.push(
            TabPanelDef::new(
                "install-npm",
                "npm (Node.js)",
                code_block(cmd, Some("bash"), Some("npm")),
            )
            .with_icon("terminal")
        );
    }

    if let Some(cmd) = pip {
        panels.push(
            TabPanelDef::new(
                "install-pip",
                "pip (Python)",
                code_block(cmd, Some("bash"), Some("pip")),
            )
            .with_icon("terminal")
        );
    }

    if let Some(cmd) = composer {
        panels.push(
            TabPanelDef::new(
                "install-composer",
                "Composer (PHP)",
                code_block(cmd, Some("bash"), Some("composer")),
            )
            .with_icon("terminal")
        );
    }

    tab_group(panels, 0)
}

// =============================================================================
// 8. MULTI-LANGUAGE (i18n) SELECTOR
// =============================================================================

/// Renders a glassmorphic language switcher with support for ecosystem locales.
pub fn language_selector(
    current_lang: &str,
    on_select: impl Fn(String) + 'static,
) -> DomBuilder {
    let languages = vec![
        ("en", "English 🇺🇸"),
        ("it", "Italiano 🇮🇹"),
        ("es", "Español 🇪🇸"),
        ("fr", "Français 🇫🇷"),
        ("de", "Deutsch 🇩🇪"),
        ("zh", "中文 🇨🇳"),
        ("ja", "日本語 🇯🇵"),
        ("ko", "한국어 🇰🇷"),
    ];

    let on_sel_rc: Rc<dyn Fn(String)> = Rc::new(on_select);

    let mut select_el = select()
        .class("bg-slate-900/80 border border-slate-700/60 rounded-xl px-2 py-1 text-xs font-medium text-slate-200 focus:outline-none focus:border-indigo-500 shadow-sm cursor-pointer max-w-[85px] sm:max-w-none");

    for (code, label) in languages {
        let is_selected = code.eq_ignore_ascii_case(current_lang);
        let mut opt = option().attr("value", code).text(label);
        if is_selected {
            opt = opt.attr("selected", "selected");
        }
        select_el = select_el.child(opt);
    }

    let on_change = on_sel_rc.clone();
    select_el = select_el.on("change", move |e: web_sys::Event| {
        if let Some(target) = e.target() {
            if let Some(sel) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                on_change(sel.value());
            }
        }
    });

    div()
        .class("ferrox-lang-selector flex items-center gap-1.5")
        .child(span().class("text-xs text-slate-400").text("🌐"))
        .child(select_el)
}

// =============================================================================
// 9. WASM INTERACTIVE SANDBOX PRIMITIVES
// =============================================================================

/// Renders a responsive glassmorphic sandbox frame for live WebAssembly playgrounds.
pub fn sandbox_container(
    title: &str,
    badge: &str,
    icon_name: &str,
    description: &str,
    content: DomBuilder,
) -> DomBuilder {
    let header = div()
        .class("flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-5 border-b border-slate-800/80 bg-slate-900/60 backdrop-blur-xl rounded-t-2xl")
        .children(vec![
            div().class("flex items-center gap-3").children(vec![
                div()
                    .class("w-9 h-9 rounded-xl flex items-center justify-center bg-indigo-500/20 text-indigo-400 border border-indigo-500/30")
                    .child(Icon::render(icon_name, 18, "#818cf8")),
                div().children(vec![
                    div().class("flex items-center gap-2").children(vec![
                        h3().class("text-base font-bold text-white tracking-tight").text(title),
                        span().class("px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30").text(badge),
                    ]),
                    p().class("text-xs text-slate-400 mt-0.5").text(description),
                ]),
            ]),
            div().class("flex items-center gap-2").children(vec![
                span().class("flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono font-medium bg-slate-800/80 text-indigo-300 border border-slate-700/60").children(vec![
                    span().class("w-2 h-2 rounded-full bg-emerald-400 animate-pulse"),
                    span().text("LIVE WASM ENGINE"),
                ]),
            ]),
        ]);

    div()
        .class("ferrox-sandbox-frame my-10 rounded-2xl border border-indigo-500/30 bg-slate-950/80 shadow-2xl shadow-indigo-950/40 overflow-hidden")
        .children(vec![header, div().class("p-5 sm:p-6").child(content)])
}

/// Renders a dynamic password strength gauge bar with colored gradient fill.
pub fn password_strength_bar(entropy_percent: f64) -> DomBuilder {
    let clamped = entropy_percent.clamp(0.0, 100.0);
    let (label, color_class, width_style) = if clamped < 25.0 {
        ("Weak (Low Entropy)", "bg-rose-500 shadow-rose-500/50", "25%")
    } else if clamped < 50.0 {
        ("Fair (Basic)", "bg-amber-500 shadow-amber-500/50", "50%")
    } else if clamped < 75.0 {
        ("Strong (Recommended)", "bg-cyan-500 shadow-cyan-500/50", "75%")
    } else {
        ("Cyber-Secured (High Entropy)", "bg-emerald-500 shadow-emerald-500/50", "100%")
    };

    div()
        .class("flex flex-col gap-1.5 w-full mt-2")
        .children(vec![
            div().class("flex justify-between items-center text-xs font-mono font-semibold").children(vec![
                span().class("text-slate-400").text("Strength:"),
                span().class("text-slate-200").text(label),
            ]),
            div()
                .class("w-full h-2 rounded-full bg-slate-800 overflow-hidden")
                .child(
                    div()
                        .class(&format!("h-full rounded-full transition-all duration-300 shadow-sm {}", color_class))
                        .attr("style", &format!("width: {};", width_style))
                ),
        ])
}

/// Renders a telemetry metric card with title, big counter, delta, and icon.
pub fn telemetry_metric_card(
    label: &str,
    value: &str,
    change: Option<&str>,
    icon_name: &str,
    color_hex: &str,
) -> DomBuilder {
    let mut val_col = div().children(vec![
        span().class("text-xs font-bold uppercase tracking-wider text-slate-400").text(label),
        div().class("text-2xl font-black text-white font-mono mt-1").text(value),
    ]);

    if let Some(ch) = change {
        val_col = val_col.child(span().class("text-xs font-semibold text-emerald-400").text(ch));
    }

    div()
        .class("p-4 rounded-xl bg-slate-900/70 border border-slate-800/80 flex items-center justify-between shadow-lg")
        .children(vec![
            val_col,
            div()
                .class("w-10 h-10 rounded-xl flex items-center justify-center bg-slate-800/80 border border-slate-700/50")
                .child(Icon::render(icon_name, 20, color_hex)),
        ])
}


