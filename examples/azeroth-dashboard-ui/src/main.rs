use ferrox_front_ui::json_dashboard_builder::{render_json_dashboard, update_json_dashboard};
use ferrox_front_core::dom::DomBuilder;
use wasm_bindgen::prelude::*;
use reqwest;
use gloo_timers::future::sleep;
use std::time::Duration;

pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = run().await {
            web_sys::console::error_1(&e);
        }
    });
}

pub async fn run() -> Result<(), JsValue> {
    // Log startup
    web_sys::console::log_1(&"AzerothDashboard UI starting...".into());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let app_div = document.get_element_by_id("app").expect("should have #app on the page");

    let mut is_first_render = true;

    loop {
        // Fetch dashboard config from backend
        match fetch_dashboard_config().await {
            Ok(json_str) => {
                if is_first_render {
                    // Render the dashboard using ferrox-front-ui builder
                    let dom_builder = render_json_dashboard(&json_str);
                    
                    // Clear loading text
                    app_div.set_inner_html("");
                    
                    // Append rendered DOM
                    let node = dom_builder.build();
                    app_div.append_child(&node)?;
                    is_first_render = false;
                } else {
                    // Update in-place to avoid flickering and layout reset
                    update_json_dashboard(&json_str);
                }
            },
            Err(err) => {
                if is_first_render {
                    app_div.set_inner_html(&format!("<div style='color: red;'>Failed to load dashboard: {}</div>", err));
                }
                // If it fails on a subsequent poll, we just ignore it and try again later
            }
        }
        
        sleep(Duration::from_secs(2)).await;
    }
}

async fn fetch_dashboard_config() -> Result<String, String> {
    let res = reqwest::get("http://127.0.0.1:8086/api/dashboard/config")
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("HTTP Error: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}