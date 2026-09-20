use ferrox_front_core::dom::{div, h2, span, button as dom_button, document, style, DomBuilder};
use ferrox_front_charts::{line_chart, bar_chart, Point, BarDatum};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::{MouseEvent, window, ResizeObserver, ResizeObserverEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub title: String,
    pub description: Option<String>,
    pub panels: Vec<PanelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PanelConfig {
    #[serde(rename = "line_chart")]
    LineChart { title: String, data: Vec<PointData>, width: Option<u32>, height: Option<u32>, alert: Option<bool> },
    #[serde(rename = "bar_chart")]
    BarChart { title: String, data: Vec<BarData>, width: Option<u32>, height: Option<u32>, alert: Option<bool> },
    #[serde(rename = "stat_badge")]
    StatBadge { title: String, value: String, color: Option<String>, alert: Option<bool> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointData { pub x: f64, pub y: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarData { pub label: String, pub value: f64, pub color: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LayoutItem {
    id: String,
    span: u8,
    #[serde(default = "default_row_span")]
    row_span: u8,
    hidden: bool,
    #[serde(default)]
    muted: bool,
}

fn default_row_span() -> u8 {
    4
}

fn get_panel_id(panel: &PanelConfig) -> String {
    let title = match panel {
        PanelConfig::LineChart { title, .. } => title,
        PanelConfig::BarChart { title, .. } => title,
        PanelConfig::StatBadge { title, .. } => title,
    };
    title.replace(" ", "").replace("(", "").replace(")", "").replace(".", "")
}

pub fn render_json_dashboard(json_str: &str) -> DomBuilder {
    let config: Result<DashboardConfig, _> = serde_json::from_str(json_str);
    match config {
        Ok(cfg) => build_dashboard(cfg),
        Err(e) => div().text(&format!("Error: {}", e)),
    }
}

fn build_dashboard(cfg: DashboardConfig) -> DomBuilder {
    setup_global_resize_listeners();
    setup_resize_observer();
    
    if let Ok(json) = serde_json::to_string(&cfg) {
        if let Some(w) = web_sys::window() {
            if let Ok(Some(ls)) = w.local_storage() {
                let _ = ls.set_item("current_dashboard_data", &json);
            }
        }
    }

    let css = r#"
        .view-mode #dashboard-palette-container,
        .view-mode .palette-label { display: none !important; }
        
        .view-mode .panel-controls { display: none !important; }
        
        .view-mode .ferrox-dashboard-panel { cursor: pointer !important; }
        .edit-mode .ferrox-dashboard-panel { cursor: grab !important; }
        
        .ferrox-dashboard-panel {
            position: relative;
            min-width: 0; 
            overflow: hidden;
            display: flex; 
            flex-direction: column;
            transition: box-shadow 0.2s ease-in-out, border 0.2s ease-in-out;
        }

        .edit-mode {
            outline: 2px dashed rgba(6, 182, 212, 0.3);
            outline-offset: 4px;
        }

        .resize-handle-right {
            position: absolute;
            right: 0;
            top: 0;
            bottom: 0;
            width: 10px;
            cursor: col-resize;
            background: transparent;
            z-index: 10;
        }
        .resize-handle-right:hover {
            background: rgba(6, 182, 212, 0.3);
        }
        
        .resize-handle-bottom {
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 10px;
            cursor: row-resize;
            background: transparent;
            z-index: 10;
        }
        .resize-handle-bottom:hover {
            background: rgba(6, 182, 212, 0.3);
        }
        
        .view-mode .resize-handle-right,
        .view-mode .resize-handle-bottom { display: none !important; }

        .panel-header-title {
            font-weight: 600; 
            color: var(--text-primary); 
            white-space: nowrap; 
            overflow: hidden; 
            text-overflow: ellipsis;
            min-width: 0;
            flex-shrink: 1;
        }

        .ferrox-btn {
            background-color: var(--accent-cyan, #06b6d4);
            color: #000;
            border: none;
            border-radius: 6px;
            padding: 8px 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
        }
        .ferrox-btn:hover {
            background-color: #0891b2;
            transform: translateY(-1px);
        }

        @keyframes pulse-alert {
            0% { box-shadow: 0 0 0 0 rgba(244, 63, 94, 0.7); }
            70% { box-shadow: 0 0 0 10px rgba(244, 63, 94, 0); }
            100% { box-shadow: 0 0 0 0 rgba(244, 63, 94, 0); }
        }
        
        .panel-alert {
            border: 1px solid rgba(244, 63, 94, 0.8) !important;
            animation: pulse-alert 2s infinite;
        }
    "#;

    let style_dom = style().text(css);

    let mut container = div()
        .attr("id", "dashboard-main-view")
        .attr("class", "ferrox-json-dashboard view-mode")
        .attr("style", "display: flex; flex-direction: column; gap: 24px; padding: 20px; background: var(--bg-primary, #0a0a16); border-radius: 12px; border: var(--border-glass);")
        .child(style_dom);

    let toggle_edit_btn = dom_button()
        .attr("id", "edit-mode-toggle")
        .attr("class", "ferrox-btn primary")
        .attr("style", "padding: 6px 12px; font-size: 0.9rem;")
        .text("Edit Layout")
        .on_click(|| {
            let doc = document();
            if let Some(main) = doc.get_element_by_id("dashboard-main-view") {
                let current_class = main.class_name();
                if current_class.contains("edit-mode") {
                    main.set_class_name(&current_class.replace("edit-mode", "view-mode"));
                    if let Some(btn) = doc.get_element_by_id("edit-mode-toggle") {
                        btn.set_text_content(Some("Edit Layout"));
                    }
                } else {
                    main.set_class_name(&current_class.replace("view-mode", "edit-mode"));
                    if let Some(btn) = doc.get_element_by_id("edit-mode-toggle") {
                        btn.set_text_content(Some("Save Layout"));
                    }
                }
            }
        });

    let mute_btn = dom_button()
        .attr("id", "mute-toggle-btn")
        .attr("class", "ferrox-btn secondary")
        .attr("style", "padding: 6px 12px; font-size: 0.9rem; margin-right: 8px; background: rgba(255,255,255,0.1); color: white; border-radius: 6px; border: none; cursor: pointer;")
        .text("🔊 Unmuted")
        .on_click(|| {
            let doc = document();
            if let Some(body) = doc.body() {
                let current = body.get_attribute("data-muted").unwrap_or("false".to_string());
                if current == "true" {
                    body.set_attribute("data-muted", "false").unwrap();
                    if let Some(btn) = doc.get_element_by_id("mute-toggle-btn") {
                        btn.set_text_content(Some("🔊 Unmuted"));
                    }
                } else {
                    body.set_attribute("data-muted", "true").unwrap();
                    if let Some(btn) = doc.get_element_by_id("mute-toggle-btn") {
                        btn.set_text_content(Some("🔇 Muted"));
                    }
                }
            }
        });

    let reset_layout_btn = dom_button()
        .attr("class", "ferrox-btn secondary")
        .attr("style", "padding: 6px 12px; font-size: 0.9rem; margin-right: 8px; background: rgba(244,63,94,0.2); color: #f43f5e; border-radius: 6px; border: 1px solid rgba(244,63,94,0.3); cursor: pointer;")
        .text("Reset Layout")
        .on_click(|| {
            if let Some(w) = web_sys::window() {
                if let Ok(Some(ls)) = w.local_storage() {
                    let _ = ls.remove_item("ferrox_layout");
                    let _ = w.location().reload();
                }
            }
        });

    let header_actions = div().attr("style", "display: flex; align-items: center;")
        .child(reset_layout_btn)
        .child(mute_btn)
        .child(toggle_edit_btn);

    let header_row = div().attr("style", "display: flex; justify-content: space-between; align-items: center;")
        .child(h2().attr("style", "color: var(--text-primary); font-size: 1.5rem; margin: 0;").text(&cfg.title))
        .child(header_actions);

    container = container.child(header_row);

    if let Some(desc) = cfg.description {
        container = container.child(div().attr("style", "color: var(--text-muted); font-size: 0.95rem; margin-top: -16px;").text(&desc));
    }

    let palette = div()
        .attr("id", "dashboard-palette-container")
        .attr("style", "display: flex; gap: 12px; min-height: 50px; padding: 12px; background: rgba(255,255,255,0.02); border-radius: 8px; border: 1px dashed rgba(255,255,255,0.1); overflow-x: auto; align-items: center;")
        .on("dragover", |e| { e.prevent_default(); })
        .on("drop", |e| { handle_drop(e, "dashboard-palette-container"); });

    let palette_label = span().attr("class", "palette-label").attr("style", "color: var(--text-muted); font-size: 0.85rem;").text("Hidden Panels (Drag & Drop to Grid)");
    container = container.child(div().child(palette_label).child(palette));

    let mut panels_grid = div()
        .attr("id", "dashboard-grid-container")
        .attr("style", "display: grid; grid-template-columns: repeat(24, 1fr); grid-auto-rows: max(35px, calc((100vh - 400px) / 12)); gap: 20px; min-height: 300px; padding: 10px; border-radius: 8px; background: rgba(0,0,0,0.1);")
        .on("dragover", |e| { e.prevent_default(); })
        .on("drop", |e| { handle_drop(e, "dashboard-grid-container"); });

    let state_json = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("ferrox_layout").unwrap_or(None);
    let mut layout_state: Vec<LayoutItem> = match state_json {
        Some(json) => serde_json::from_str::<Vec<LayoutItem>>(&json).unwrap_or_default(),
        None => Vec::new(),
    };

    if layout_state.is_empty() || layout_state.len() != cfg.panels.len() {
        let order_and_spans = vec![
            ("ServerCPUUsage%", 4, 3),
            ("ServerRAMUsage%", 4, 3),
            ("OnlinePlayersac_world_online_players", 4, 3),
            ("TotalPacketsac_world_packets_total", 4, 3),
            ("MemoryRSS", 4, 3),
            ("AveragePing", 4, 3),
            ("WorldUpdateDurations", 12, 9),
            ("DatabaseQueueSize", 4, 9),
            ("NetworkTrafficMB", 4, 9),
            ("UpdatePhasess", 4, 9),
        ];

        let mut new_layout = Vec::new();
        for (id_str, span, row_span) in order_and_spans {
            if cfg.panels.iter().any(|p| get_panel_id(p) == id_str) {
                new_layout.push(LayoutItem {
                    id: id_str.to_string(),
                    span,
                    row_span,
                    hidden: false,
                    muted: false,
                });
            }
        }
        
        for p in &cfg.panels {
            let id = get_panel_id(p);
            if !new_layout.iter().any(|item| item.id == id) {
                new_layout.push(LayoutItem {
                    id,
                    span: 6,
                    row_span: 4,
                    hidden: false,
                    muted: false,
                });
            }
        }
        
        layout_state = new_layout;
        save_layout_state(&layout_state);
    }

    let mut sorted_panels = Vec::new();
    for layout_item in &layout_state {
        if let Some(panel) = cfg.panels.iter().find(|p| get_panel_id(p) == layout_item.id) {
            sorted_panels.push((panel.clone(), layout_item.clone()));
        }
    }

    for (panel, item_state) in sorted_panels {
        let panel_dom = build_panel_dom(panel, item_state.span, item_state.row_span, item_state.hidden, item_state.muted);
        panels_grid = panels_grid.child(panel_dom);
    }

    container.child(panels_grid)
}

fn get_panel_alert(config: &PanelConfig) -> bool {
    match config {
        PanelConfig::LineChart { alert, .. } => alert.unwrap_or(false),
        PanelConfig::BarChart { alert, .. } => alert.unwrap_or(false),
        PanelConfig::StatBadge { alert, .. } => alert.unwrap_or(false),
    }
}

fn build_panel_content(config: &PanelConfig, id: &str) -> DomBuilder {
    let (actual_w, actual_h) = if let Some(el) = document().get_element_by_id(&format!("panel-content-{}", id)) {
        (el.client_width().max(50) as u32, el.client_height().max(50) as u32)
    } else {
        (0, 0)
    };

    match config.clone() {
        PanelConfig::LineChart { data, width, height, .. } => {
            let points: Vec<Point> = data.into_iter().map(|p| Point { x: p.x, y: p.y }).collect();
            let final_w = if actual_w > 0 { actual_w } else { width.unwrap_or(300) };
            let final_h = if actual_h > 0 { actual_h } else { height.unwrap_or(200) };
            line_chart(id, points, final_w, final_h, true)
        }
        PanelConfig::BarChart { data, width, height, .. } => {
            let bars: Vec<BarDatum> = data.into_iter().map(|b| BarDatum { label: b.label, value: b.value, color: b.color }).collect();
            let final_w = if actual_w > 0 { actual_w } else { width.unwrap_or(300) };
            let final_h = if actual_h > 0 { actual_h } else { height.unwrap_or(200) };
            bar_chart(id, bars, final_w, final_h, true)
        }
        PanelConfig::StatBadge { value, color, .. } => {
            let badge_color = color.unwrap_or_else(|| "var(--accent-cyan, #06b6d4)".to_string());
            div().attr("style", &format!("font-size: 2rem; font-weight: 700; color: {}; text-align: center; word-break: break-all;", badge_color)).text(&value)
        }
    }
}

fn build_panel_dom(config: PanelConfig, col_span: u8, row_span: u8, is_hidden: bool, is_muted: bool) -> DomBuilder {
    let id = get_panel_id(&config);
    let title_str = match &config {
        PanelConfig::LineChart { title, .. } => title.clone(),
        PanelConfig::BarChart { title, .. } => title.clone(),
        PanelConfig::StatBadge { title, .. } => title.clone(),
    };

    let content = build_panel_content(&config, &id);

    let panel_id_clone = id.clone();
    
    let header = div()
        .attr("style", "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 8px; margin-bottom: 12px; cursor: grab; gap: 8px;")
        .child(span().attr("class", "panel-header-title").text(&title_str));

    let controls = div().attr("class", "panel-controls").attr("style", "display: flex; gap: 4px; flex-shrink: 0;");
    
    let btn_style = "background: rgba(255,255,255,0.1); border: none; color: white; border-radius: 4px; width: 22px; height: 22px; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 12px;";
    
    let id_c = id.clone();
    let btn_reset = dom_button().attr("style", btn_style).attr("title", "Reset Size").text("↺").on_click(move || reset_panel_span(&id_c));
    let id_c = id.clone();
    let btn_hide = dom_button().attr("style", btn_style).attr("title", "Hide Panel").text("x").on_click(move || toggle_panel_visibility(&id_c));
    let id_c = id.clone();
    let btn_mute = dom_button()
        .attr("id", &format!("panel-mute-btn-{}", id_c))
        .attr("style", btn_style)
        .attr("title", "Toggle Panel Alert Mute")
        .text(if is_muted { "🔕" } else { "🔔" })
        .on_click(move || toggle_panel_mute(&id_c));

    let header = header.child(controls.child(btn_mute).child(btn_reset).child(btn_hide));
    
    let style = if is_hidden {
        "display: flex; min-width: 150px; max-width: 200px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px; cursor: grab; align-items: center;"
    } else {
        &format!("grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", col_span, row_span)
    };

    let title_string = title_str.to_string();
    let config_clone = config.clone();
    let panel_id_drop_clone = id.clone();
    
    let is_alert = get_panel_alert(&config);
    let mut base_classes = "ferrox-dashboard-panel".to_string();
    if is_alert {
        base_classes.push_str(" panel-alert");
    }

    let handle_id = id.clone();
    let resize_handle = div()
        .attr("class", "resize-handle-right")
        .on("mousedown", move |e| {
            if let Some(mouse_event) = e.dyn_ref::<MouseEvent>() {
                mouse_event.prevent_default();
                mouse_event.stop_propagation();
                if let Some(body) = document().body() {
                    body.set_attribute("data-resizing-panel-x", &format!("panel-{}", handle_id)).unwrap();
                    body.set_attribute("data-resize-start-x", &mouse_event.client_x().to_string()).unwrap();
                }
            }
        });

    let handle_id_y = id.clone();
    let resize_handle_bottom = div()
        .attr("class", "resize-handle-bottom")
        .on("mousedown", move |e| {
            if let Some(mouse_event) = e.dyn_ref::<MouseEvent>() {
                mouse_event.prevent_default();
                mouse_event.stop_propagation();
                if let Some(body) = document().body() {
                    body.set_attribute("data-resizing-panel-y", &format!("panel-{}", handle_id_y)).unwrap();
                    body.set_attribute("data-resize-start-y", &mouse_event.client_y().to_string()).unwrap();
                }
            }
        });

    let content_container = div()
        .attr("id", &format!("panel-content-{}", id))
        .attr("style", "flex: 1; display: flex; align-items: center; justify-content: center; min-height: 0; min-width: 0; position: relative;")
        .on_click(move || {
            let d = document();
            if let Some(main) = d.get_element_by_id("dashboard-main-view") {
                if !main.class_name().contains("edit-mode") {
                    navigate_to_detail(&title_string, config_clone.clone());
                }
            }
        })
        .child(content);

    let content_node = content_container.build();

    if let Some(w) = window() {
        if let Ok(ro_val) = js_sys::Reflect::get(&w, &JsValue::from_str("global_resize_observer")) {
            if ro_val.is_object() {
                let ro: ResizeObserver = ro_val.unchecked_into();
                if let Some(element) = content_node.dyn_ref::<web_sys::Element>() {
                    ro.observe(element);
                }
            }
        }
    }

    div()
        .attr("id", &format!("panel-{}", id))
        .attr("class", &base_classes)
        .attr("data-span", &col_span.to_string())
        .attr("data-row-span", &row_span.to_string())
        .attr("data-hidden", if is_hidden { "true" } else { "false" })
        .attr("data-muted", if is_muted { "true" } else { "false" })
        .attr("style", style)
        .attr("draggable", "true")
        .on("dragstart", move |e| {
            if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
                if let Some(dt) = drag_event.data_transfer() {
                    let _ = dt.set_data("text/plain", &panel_id_clone);
                }
            }
        })
        .on("dragover", |e| {
            e.prevent_default(); 
        })
        .on("drop", move |e| {
            e.prevent_default();
            e.stop_propagation();
            if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
                if let Some(dt) = drag_event.data_transfer() {
                    if let Ok(dragged_id) = dt.get_data("text/plain") {
                        swap_panels(&dragged_id, &panel_id_drop_clone);
                    }
                }
            }
        })
        .child(resize_handle)
        .child(resize_handle_bottom)
        .child(header)
        .child(DomBuilder::from_node(content_node))
}

fn handle_drop(e: web_sys::Event, container_id: &str) {
    e.prevent_default();
    if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
        if let Some(dt) = drag_event.data_transfer() {
            if let Ok(dragged_id) = dt.get_data("text/plain") {
                let doc = document();
                if let Some(dragged_el) = doc.get_element_by_id(&format!("panel-{}", dragged_id)) {
                    if let Some(container) = doc.get_element_by_id(container_id) {
                        let _ = container.append_child(&dragged_el);
                        
                        let is_hidden = container_id == "dashboard-palette-container";
                        dragged_el.set_attribute("data-hidden", if is_hidden { "true" } else { "false" }).unwrap();
                        
                        if is_hidden {
                            dragged_el.set_attribute("style", "display: flex; min-width: 150px; max-width: 200px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px; cursor: grab; align-items: center;").unwrap();
                        } else {
                            let span = dragged_el.get_attribute("data-span").unwrap_or("6".to_string());
                            let row_span = dragged_el.get_attribute("data-row-span").unwrap_or("4".to_string());
                            dragged_el.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", span, row_span)).unwrap();
                        }

                        sync_layout_state();
                    }
                }
            }
        }
    }
}

fn swap_panels(dragged_id: &str, target_id: &str) {
    if dragged_id == target_id { return; }
    let doc = document();
    let dragged_el = doc.get_element_by_id(&format!("panel-{}", dragged_id));
    let target_el = doc.get_element_by_id(&format!("panel-{}", target_id));
    
    if let (Some(dragged), Some(target)) = (dragged_el, target_el) {
        if let Some(parent) = target.parent_node() {
            let dragged_next = dragged.next_sibling();
            
            let _ = parent.insert_before(&dragged, Some(&target));
            
            if let Some(next) = dragged_next {
                let _ = parent.insert_before(&target, Some(&next));
            } else {
                let _ = parent.append_child(&target);
            }
            
            sync_layout_state();
        }
    }
}

fn reset_panel_span(id: &str) {
    let doc = document();
    if let Some(panel) = doc.get_element_by_id(&format!("panel-{}", id)) {
        let span = 6;
        let row_span = 4;
        panel.set_attribute("data-span", &span.to_string()).unwrap();
        panel.set_attribute("data-row-span", &row_span.to_string()).unwrap();
        if panel.get_attribute("data-hidden").unwrap_or("false".to_string()) == "false" {
            panel.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", span, row_span)).unwrap();
        }
        sync_layout_state();
    }
}

fn toggle_panel_visibility(id: &str) {
    let doc = document();
    if let Some(panel) = doc.get_element_by_id(&format!("panel-{}", id)) {
        let is_hidden = panel.get_attribute("data-hidden").unwrap_or("false".to_string()) == "true";
        if is_hidden {
            if let Some(grid) = doc.get_element_by_id("dashboard-grid-container") {
                grid.append_child(&panel).unwrap();
                panel.set_attribute("data-hidden", "false").unwrap();
                let span = panel.get_attribute("data-span").unwrap_or("6".to_string());
                let row_span = panel.get_attribute("data-row-span").unwrap_or("4".to_string());
                panel.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", span, row_span)).unwrap();
            }
        } else {
            if let Some(palette) = doc.get_element_by_id("dashboard-palette-container") {
                palette.append_child(&panel).unwrap();
                panel.set_attribute("data-hidden", "true").unwrap();
                panel.set_attribute("style", "display: flex; min-width: 150px; max-width: 200px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px; cursor: grab; align-items: center;").unwrap();
            }
        }
        sync_layout_state();
    }
}

fn toggle_panel_mute(id: &str) {
    let doc = document();
    if let Some(panel) = doc.get_element_by_id(&format!("panel-{}", id)) {
        let is_muted = panel.get_attribute("data-muted").unwrap_or("false".to_string()) == "true";
        if is_muted {
            panel.set_attribute("data-muted", "false").unwrap();
            if let Some(btn) = doc.get_element_by_id(&format!("panel-mute-btn-{}", id)) {
                btn.set_text_content(Some("🔔"));
            }
        } else {
            panel.set_attribute("data-muted", "true").unwrap();
            if let Some(btn) = doc.get_element_by_id(&format!("panel-mute-btn-{}", id)) {
                btn.set_text_content(Some("🔕"));
            }
            // Remove alert styling if it's currently alerting
            let current_classes = panel.class_name();
            if current_classes.contains("panel-alert") {
                panel.set_class_name(&current_classes.replace("panel-alert", "").trim());
            }
        }
        sync_layout_state();
    }
}

fn sync_layout_state() {
    let doc = document();
    let mut layout = Vec::new();
    
    if let Some(grid) = doc.get_element_by_id("dashboard-grid-container") {
        if let Some(html_grid) = grid.dyn_ref::<web_sys::HtmlElement>() {
            let children = html_grid.children();
            for i in 0..children.length() {
                let el = children.item(i).unwrap();
                if let Some(id_attr) = el.id().strip_prefix("panel-") {
                    let span = el.get_attribute("data-span").unwrap_or("6".to_string()).parse().unwrap_or(6);
                    let row_span = el.get_attribute("data-row-span").unwrap_or("4".to_string()).parse().unwrap_or(4);
                    let muted = el.get_attribute("data-muted").unwrap_or("false".to_string()) == "true";
                    layout.push(LayoutItem { id: id_attr.to_string(), span, row_span, hidden: false, muted });
                }
            }
        }
    }
    
    if let Some(palette) = doc.get_element_by_id("dashboard-palette-container") {
        if let Some(html_palette) = palette.dyn_ref::<web_sys::HtmlElement>() {
            let children = html_palette.children();
            for i in 0..children.length() {
                let el = children.item(i).unwrap();
                if let Some(id_attr) = el.id().strip_prefix("panel-") {
                    let span = el.get_attribute("data-span").unwrap_or("6".to_string()).parse().unwrap_or(6);
                    let row_span = el.get_attribute("data-row-span").unwrap_or("4".to_string()).parse().unwrap_or(4);
                    let muted = el.get_attribute("data-muted").unwrap_or("false".to_string()) == "true";
                    layout.push(LayoutItem { id: id_attr.to_string(), span, row_span, hidden: true, muted });
                }
            }
        }
    }
    
    save_layout_state(&layout);
}

fn save_layout_state(layout: &[LayoutItem]) {
    if let Ok(json) = serde_json::to_string(layout) {
        if let Some(w) = web_sys::window() {
            if let Ok(Some(ls)) = w.local_storage() {
                let _ = ls.set_item("ferrox_layout", &json);
            }
        }
    }
}

fn navigate_to_detail(title: &str, config: PanelConfig) {
    let doc = document();

    if let Some(main_view) = doc.get_element_by_id("dashboard-main-view") {
        main_view.set_attribute("style", "display: none;").unwrap();
    }

    if let Some(old_detail) = doc.get_element_by_id("dashboard-detail-view") {
        old_detail.remove();
    }

    let mut raw_data_dom = div().attr("style", "display: flex; flex-direction: column; gap: 8px; font-family: monospace; font-size: 0.9rem;");
    let mut stats_dom = div().attr("style", "display: flex; gap: 24px; padding: 16px; background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; margin-bottom: 24px;");

    let large_chart = match config {
        PanelConfig::LineChart { data, .. } => {
            stats_dom = stats_dom.child(span().text(&format!("Data Points: {}", data.len())));
            stats_dom = stats_dom.child(span().text(&format!("Latest Value: {:.3}", data.last().map(|p| p.y).unwrap_or(0.0))));
            stats_dom = stats_dom.child(span().text("Frequency: 1s"));

            for (i, p) in data.iter().rev().take(10).enumerate() {
                raw_data_dom = raw_data_dom.child(
                    div().attr("style", "display: grid; grid-template-columns: 100px 1fr; border-bottom: 1px solid rgba(255,255,255,0.1); padding: 4px 0;")
                        .child(span().attr("style", "color: var(--accent-cyan);").text(&format!("T-{}s", i)))
                        .child(span().text(&format!("{:.4}", p.y)))
                );
            }

            let id = format!("{}_detail", title.replace(" ", "").replace("(", "").replace(")", ""));
            let points: Vec<Point> = data.into_iter().map(|p| Point { x: p.x, y: p.y }).collect();
            line_chart(&id, points, 1200, 500, true)
        }
        PanelConfig::BarChart { data, .. } => {
            stats_dom = stats_dom.child(span().text(&format!("Categories: {}", data.len())));
            
            for b in data.iter() {
                raw_data_dom = raw_data_dom.child(
                    div().attr("style", "display: grid; grid-template-columns: 150px 1fr; border-bottom: 1px solid rgba(255,255,255,0.1); padding: 4px 0;")
                        .child(span().attr("style", &format!("color: {};", b.color)).text(&b.label))
                        .child(span().text(&format!("{:.4}", b.value)))
                    );
            }

            let id = format!("{}_detail", title.replace(" ", "").replace("(", "").replace(")", ""));
            let bars: Vec<BarDatum> = data.into_iter().map(|b| BarDatum { label: b.label, value: b.value, color: b.color }).collect();
            bar_chart(&id, bars, 1200, 500, true)
        }
        PanelConfig::StatBadge { value, color, .. } => {
            div()
                .attr("style", &format!("font-size: 8rem; font-weight: 700; color: {}; text-align: center; padding: 120px;", color.unwrap_or_else(|| "var(--accent-cyan, #06b6d4)".to_string())))
                .text(&value)
        }
    };

    let back_btn = dom_button()
        .attr("class", "ferrox-btn primary")
        .text("← Back to Dashboard")
        .attr("style", "margin-bottom: 24px; align-self: flex-start; box-shadow: 0 4px 12px rgba(6, 182, 212, 0.3);")
        .on_click(|| {
            let d = document();
            if let Some(detail) = d.get_element_by_id("dashboard-detail-view") {
                detail.remove();
            }
            if let Some(main) = d.get_element_by_id("dashboard-main-view") {
                let current_class = main.class_name();
                main.set_attribute("style", "display: flex; flex-direction: column; gap: 24px; padding: 20px; background: var(--bg-primary, #0a0a16); border-radius: 12px; border: var(--border-glass);").unwrap();
                main.set_class_name(&current_class);
                
                let mut hidden_panels = Vec::new();
                if let Some(grid) = d.get_element_by_id("dashboard-grid-container") {
                    if let Some(html_grid) = grid.dyn_ref::<web_sys::HtmlElement>() {
                        let children = html_grid.children();
                        for i in 0..children.length() {
                            let el = children.item(i).unwrap();
                            if el.get_attribute("data-hidden").unwrap_or("false".to_string()) == "true" {
                                hidden_panels.push(el);
                            }
                        }
                    }
                }
                if let Some(palette) = d.get_element_by_id("dashboard-palette-container") {
                    for panel in hidden_panels {
                        palette.append_child(&panel).unwrap();
                        panel.set_attribute("style", "display: flex; min-width: 150px; max-width: 200px; padding: 8px; background: rgba(255,255,255,0.1); border-radius: 4px; cursor: grab; align-items: center;").unwrap();
                    }
                }
            }
        });

    let header = h2()
        .attr("style", "font-size: 2rem; color: var(--text-primary); margin: 0 0 8px 0;")
        .text(title);

    let detail_view = div()
        .attr("id", "dashboard-detail-view")
        .attr("style", "padding: 20px; background: var(--bg-primary); min-height: 100vh; display: flex; flex-direction: column;")
        .child(back_btn)
        .child(header)
        .child(stats_dom)
        .child(
            div()
                .attr("style", "background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05); border-radius: 12px; padding: 20px; margin-bottom: 24px; display: flex; justify-content: center; overflow-x: auto;")
                .child(large_chart)
        )
        .child(
            div()
                .attr("style", "background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05); border-radius: 12px; padding: 20px;")
                .child(span().attr("style", "font-weight: 600; font-size: 1.1rem; margin-bottom: 16px; display: block;").text("Real-time Data Stream"))
                .child(raw_data_dom)
        );

    if let Some(app_container) = doc.get_element_by_id("app") {
        app_container.append_child(&detail_view.build()).unwrap();
    }
}

pub fn update_json_dashboard(json_str: &str) {
    let config: Result<DashboardConfig, _> = serde_json::from_str(json_str);
    if let Ok(cfg) = config {
        
        if let Ok(json) = serde_json::to_string(&cfg) {
            if let Some(w) = web_sys::window() {
                if let Ok(Some(ls)) = w.local_storage() {
                    let _ = ls.set_item("current_dashboard_data", &json);
                }
            }
        }
        
        let doc = document();
        for panel in cfg.panels {
            let id = get_panel_id(&panel);
            
            if let Some(content_wrapper) = doc.get_element_by_id(&format!("panel-content-{}", id)) {
                let new_content_dom = build_panel_content(&panel, &id).build();
                content_wrapper.set_inner_html("");
                let _ = content_wrapper.append_child(&new_content_dom);
            }
            
            if let Some(panel_el) = doc.get_element_by_id(&format!("panel-{}", id)) {
                let is_alert = get_panel_alert(&panel);
                let current_classes = panel_el.class_name();
                let has_alert_class = current_classes.contains("panel-alert");
                let panel_muted = panel_el.get_attribute("data-muted").unwrap_or("false".to_string()) == "true";
                
                if is_alert && !has_alert_class && !panel_muted {
                    panel_el.set_class_name(&format!("{} panel-alert", current_classes));
                    
                    if let Some(body) = doc.body() {
                        if body.get_attribute("data-muted").unwrap_or("false".to_string()) != "true" {
                            if let Ok(audio) = web_sys::HtmlAudioElement::new_with_src("https://assets.mixkit.co/active_storage/sfx/2869/2869-preview.mp3") {
                                let _ = audio.play();
                            }
                        }
                    }
                } else if (!is_alert || panel_muted) && has_alert_class {
                    panel_el.set_class_name(&current_classes.replace("panel-alert", "").trim());
                }
            }
        }
    }
}

fn setup_global_resize_listeners() {
    let window = window().unwrap();
    let doc = document();
    
    if let Some(body) = doc.body() {
        if body.get_attribute("data-resize-init").is_some() {
            return;
        }
        body.set_attribute("data-resize-init", "true").unwrap();
    }
    
    let mousemove = Closure::wrap(Box::new(move |e: MouseEvent| {
        let doc = document();
        if let Some(body) = doc.body() {
            if let Some(panel_id) = body.get_attribute("data-resizing-panel-x") {
                let start_x: f64 = body.get_attribute("data-resize-start-x").unwrap_or("0".into()).parse().unwrap_or(0.0);
                let current_x = e.client_x() as f64;
                let delta_x = current_x - start_x;
                
                let col_width = body.client_width() as f64 / 24.0;
                
                if delta_x.abs() > col_width {
                    let steps = (delta_x / col_width).trunc() as i32;
                    if steps != 0 {
                        if let Some(panel) = doc.get_element_by_id(&panel_id) {
                            let mut current_span: i32 = panel.get_attribute("data-span").unwrap_or("6".into()).parse().unwrap_or(6);
                            let row_span: i32 = panel.get_attribute("data-row-span").unwrap_or("4".into()).parse().unwrap_or(4);
                            let next_sibling = panel.next_element_sibling();
                            
                            let mut next_span: Option<i32> = None;
                            if steps > 0 {
                                if let Some(ref next) = next_sibling {
                                    let ns = next.get_attribute("data-span").unwrap_or("6".into()).parse().unwrap_or(6);
                                    if ns - steps >= 2 {
                                        next_span = Some(ns - steps);
                                    }
                                }
                            }
                            
                            let mut can_grow = false;
                            if steps > 0 {
                                if next_span.is_some() || next_sibling.is_none() {
                                    can_grow = true;
                                }
                            } else {
                                can_grow = true;
                            }
                            
                            if can_grow {
                                current_span += steps;
                                if current_span < 2 { current_span = 2; }
                                if current_span > 24 { current_span = 24; }
                                
                                panel.set_attribute("data-span", &current_span.to_string()).unwrap();
                                panel.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", current_span, row_span)).unwrap();
                                
                                if let (Some(next), Some(ns)) = (next_sibling, next_span) {
                                    next.set_attribute("data-span", &ns.to_string()).unwrap();
                                    let next_row_span = next.get_attribute("data-row-span").unwrap_or("4".into()).parse().unwrap_or(4);
                                    next.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", ns, next_row_span)).unwrap();
                                }
                                
                                body.set_attribute("data-resize-start-x", &(start_x + (steps as f64 * col_width)).to_string()).unwrap();
                                sync_layout_state();
                            }
                        }
                    }
                }
            } else if let Some(panel_id) = body.get_attribute("data-resizing-panel-y") {
                let start_y: f64 = body.get_attribute("data-resize-start-y").unwrap_or("0".into()).parse().unwrap_or(0.0);
                let current_y = e.client_y() as f64;
                let delta_y = current_y - start_y;
                
                let vh = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap();
                let computed_row = (vh - 400.0) / 12.0;
                let row_height = computed_row.max(35.0) + 20.0;
                
                if delta_y.abs() > row_height * 0.5 {
                    let steps = (delta_y / row_height).trunc() as i32;
                    if steps != 0 {
                        if let Some(panel) = doc.get_element_by_id(&panel_id) {
                            let current_span: i32 = panel.get_attribute("data-span").unwrap_or("6".into()).parse().unwrap_or(6);
                            let mut current_row_span: i32 = panel.get_attribute("data-row-span").unwrap_or("4".into()).parse().unwrap_or(4);
                            
                            current_row_span += steps;
                            if current_row_span < 2 { current_row_span = 2; }
                            
                            panel.set_attribute("data-row-span", &current_row_span.to_string()).unwrap();
                            panel.set_attribute("style", &format!("position: relative; grid-column: span {}; grid-row: span {}; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 8px; padding: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);", current_span, current_row_span)).unwrap();
                            
                            body.set_attribute("data-resize-start-y", &(start_y + (steps as f64 * row_height)).to_string()).unwrap();
                            sync_layout_state();
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(MouseEvent)>);
    
    let mouseup = Closure::wrap(Box::new(move |_e: MouseEvent| {
        let doc = document();
        if let Some(body) = doc.body() {
            let _ = body.remove_attribute("data-resizing-panel-x");
            let _ = body.remove_attribute("data-resizing-panel-y");
        }
    }) as Box<dyn FnMut(MouseEvent)>);
    
    let window_et: &web_sys::EventTarget = window.as_ref();
    window_et.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
    window_et.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();
    
    mousemove.forget();
    mouseup.forget();
}

fn setup_resize_observer() {
    let doc = document();
    if let Some(body) = doc.body() {
        if body.get_attribute("data-ro-init").is_some() {
            return;
        }
        body.set_attribute("data-ro-init", "true").unwrap();
        
        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            if let Some(w) = window() {
                if let Ok(Some(json_str)) = w.local_storage().unwrap().unwrap().get_item("current_dashboard_data") {
                    if let Ok(config) = serde_json::from_str::<DashboardConfig>(&json_str) {
                        for i in 0..entries.length() {
                            let entry: ResizeObserverEntry = entries.get(i).unchecked_into();
                            let target = entry.target();
                            if let Some(id_attr) = target.id().strip_prefix("panel-content-") {
                                if let Some(panel) = config.panels.iter().find(|p| get_panel_id(p) == id_attr) {
                                    let new_content_dom = build_panel_content(panel, id_attr).build();
                                    target.set_inner_html("");
                                    let _ = target.append_child(&new_content_dom);
                                }
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);
        
        let ro = ResizeObserver::new(callback.as_ref().unchecked_ref()).unwrap();
        
        let w = window().unwrap();
        let _ = js_sys::Reflect::set(&w, &JsValue::from_str("global_resize_observer"), &ro);
        
        callback.forget();
    }
}
