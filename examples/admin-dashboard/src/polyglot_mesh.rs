//! # Ferrox Polyglot Mesh Hub & Super-Showcase
//!
//! Interactive WebAssembly view demonstrating the end-to-end distributed orchestration
//! of the 6 Ferrox polyglot microservice runtimes, Redpanda event broker, Redis Redlock,
//! and Traefik central API Gateway with zero JavaScript dependencies.

use ferrox_front_core::dom::{div, span, p, h2, h3, DomBuilder};
use ferrox_front_ui::components::*;
use ferrox_front_ui::icons::Icon;
use ferrox_front_ui::layout::{container, row, col};
use web_sys::window;
use crate::card_advanced;

pub struct ServiceNodeInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub runtime: &'static str,
    pub role: &'static str,
    pub port: &'static str,
    pub status: &'static str,
    pub status_badge: &'static str,
    pub memory: &'static str,
    pub latency: &'static str,
    pub icon: &'static str,
    pub accent_color: &'static str,
    pub features: &'static [&'static str],
}

pub fn get_mesh_nodes() -> Vec<ServiceNodeInfo> {
    vec![
        ServiceNodeInfo {
            id: "match",
            name: "Ferrox-Match Core",
            runtime: "Rust 1.80+ (Native Engine)",
            role: "Sub-Microsecond Order Matching & LOB",
            port: "8086 / 8080",
            status: "ACTIVE",
            status_badge: "primary",
            memory: "14.2 MB",
            latency: "380 ns",
            icon: "cpu",
            accent_color: "var(--accent-primary, #6366f1)",
            features: &["Zero-Alloc Fast-Path", "Price-Time Priority", "Node-API & Axum Sockets"],
        },
        ServiceNodeInfo {
            id: "py",
            name: "Ferrox-Py FastData",
            runtime: "Python 3.12+ (Polars / PyO3)",
            role: "High-Frequency Market Feeds & Ticks",
            port: "8001",
            status: "STREAMING",
            status_badge: "info",
            memory: "38.4 MB",
            latency: "1.4 ms",
            icon: "activity",
            accent_color: "#06b6d4",
            features: &["Polars Vectorized Series", "Binance Live Ingestion", "CloudEvents Emitter"],
        },
        ServiceNodeInfo {
            id: "java",
            name: "Ferrox-Java Loom",
            runtime: "Java 21 (Project Loom & Panama)",
            role: "High-Concurrency Ledger Settlement",
            port: "8082 / 8081",
            status: "ONLINE",
            status_badge: "success",
            memory: "84.1 MB",
            latency: "2.1 ms",
            icon: "server",
            accent_color: "#f59e0b",
            features: &["65,536 Virtual Threads", "Off-Heap Panama FFI", "ACID Double-Entry Ledger"],
        },
        ServiceNodeInfo {
            id: "php",
            name: "Ferrox-PHP FrankenPHP",
            runtime: "PHP 8.3+ (Worker Mode)",
            role: "Distributed E-Shop Saga Coordinator",
            port: "8000",
            status: "STANDBY",
            status_badge: "primary",
            memory: "22.8 MB",
            latency: "3.8 ms",
            icon: "shopping-cart",
            accent_color: "#8b5cf6",
            features: &["FrankenPHP Worker State", "PostgreSQL Outbox", "Compensating Sagas"],
        },
        ServiceNodeInfo {
            id: "node",
            name: "Ferrox-Node & YALC",
            runtime: "Node.js 20 (Fastify 5 / NestJS)",
            role: "Central API Gateway & CQRS Command",
            port: "3000 / 3002",
            status: "HEALTHY",
            status_badge: "success",
            memory: "41.6 MB",
            latency: "1.1 ms",
            icon: "shield",
            accent_color: "#10b981",
            features: &["PASETO v4 Auth Guard", "CQRS Command Bus", "Fastify Zero-Overhead HTTP"],
        },
        ServiceNodeInfo {
            id: "redpanda",
            name: "Redpanda Event Stream",
            runtime: "Native C++ (Kafka API v24.1)",
            role: "Sub-Millisecond Event Streaming",
            port: "9092 / 19092",
            status: "CONNECTED",
            status_badge: "warning",
            memory: "185 MB",
            latency: "0.8 ms",
            icon: "layers",
            accent_color: "#f43f5e",
            features: &["Zero JVM Overhead", "CloudEvents Streaming", "Raft Consensus Log"],
        },
        ServiceNodeInfo {
            id: "redis",
            name: "Redis Redlock Cluster",
            runtime: "Redis 7 Alpine",
            role: "Distributed Mutex & Token Caching",
            port: "6379",
            status: "LOCKED",
            status_badge: "secondary",
            memory: "18.5 MB",
            latency: "0.3 ms",
            icon: "lock",
            accent_color: "#ef4444",
            features: &["Redlock Distributed Mutex", "Shannon Entropy Filter", "Singleflight Guard"],
        },
        ServiceNodeInfo {
            id: "wasm",
            name: "Ferrox-Front Wasm",
            runtime: "Rust WebAssembly (Zero-JS)",
            role: "Reactive Presentation & Charting Hub",
            port: "4000 (Browser)",
            status: "MOUNTED",
            status_badge: "primary",
            memory: "16.8 MB (DOM)",
            latency: "< 16 ms (60 FPS)",
            icon: "layout",
            accent_color: "#ec4899",
            features: &["Signal<T> Reactivity", "Pure Vector SVG Charts", "Direct Memory Zero-JS"],
        },
    ]
}

pub fn update_mesh_telemetry_dom(
    rust_matching_latency_ns: u64,
    python_feed_latency_ms: f64,
    java_loom_settled_per_sec: u64,
    java_loom_latency_micros: u64,
    php_saga_active_count: u32,
    php_saga_success_rate: f64,
    redpanda_throughput_eps: u64,
) {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(el) = doc.get_element_by_id("mesh-latency-match") {
                el.set_text_content(Some(&format!("{} ns", rust_matching_latency_ns)));
            }
            if let Some(el) = doc.get_element_by_id("mesh-latency-py") {
                el.set_text_content(Some(&format!("{:.1} ms", python_feed_latency_ms)));
            }
            if let Some(el) = doc.get_element_by_id("mesh-latency-java") {
                el.set_text_content(Some(&format!("{:.2} ms", java_loom_latency_micros as f64 / 1000.0)));
            }
            if let Some(el) = doc.get_element_by_id("mesh-metric-java") {
                el.set_text_content(Some(&format!("{} trades/s", java_loom_settled_per_sec)));
            }
            if let Some(el) = doc.get_element_by_id("mesh-latency-php") {
                el.set_text_content(Some(&format!("Active: {} ({:.1}%)", php_saga_active_count, php_saga_success_rate)));
            }
            if let Some(el) = doc.get_element_by_id("mesh-metric-redpanda") {
                el.set_text_content(Some(&format!("{} ev/s", redpanda_throughput_eps)));
            }
        }
    }
}

pub fn render_polyglot_mesh_tab() -> DomBuilder {
    let nodes = get_mesh_nodes();

    // 1. Hero / Header Banner
    let hero_banner = div()
        .class("ferrox-card rounded-3xl p-8 backdrop-blur-2xl shadow-2xl relative overflow-hidden border")
        .attr("style", "background: linear-gradient(135deg, rgba(30, 27, 75, 0.7) 0%, rgba(15, 23, 42, 0.9) 100%); border-color: rgba(99, 102, 241, 0.25);")
        .children(vec![
            div().class("flex flex-wrap items-center justify-between gap-6 relative z-10")
                .children(vec![
                    div().class("max-w-2xl").children(vec![
                        div().class("flex items-center gap-3 mb-2").children(vec![
                            badge("Ecosystem Super-Showcase 2.0", "primary"),
                            badge("Polyglot Event Mesh", "success"),
                            badge("Redpanda Streaming", "warning"),
                        ]),
                        h2().class("text-3xl font-black text-white tracking-tight leading-snug")
                            .text("🌐 Ferrox Polyglot Mesh Hub"),
                        p().class("text-sm text-slate-300 mt-2 leading-relaxed")
                            .text("Live end-to-end orchestration of 6 polyglot runtimes: Rust matching engine (380ns), Python Polars feeds, Java 21 Loom virtual threads, PHP FrankenPHP Sagas, Node Fastify gateway, Redpanda Kafka event broker and Redis Redlock distributed locks."),
                    ]),
                    div().class("flex flex-col sm:flex-row items-center gap-3").children(vec![
                        button_advanced("⚡ Simulate Distributed Trade", "primary", "sm", false, Some("activity"))
                            .on_click(|| {
                                if let Some(win) = window() {
                                    let doc = win.document().unwrap();
                                    if let Some(trace_log) = doc.get_element_by_id("mesh-trace-log") {
                                        let current_ts = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
                                        let step_html = format!(
                                            "<div class='p-3.5 rounded-xl border border-indigo-500/40 bg-indigo-950/40 text-xs text-indigo-200 animate-pulse font-mono mb-2'>\
                                            <strong>[{}] DISPATCHED:</strong> Order #ORD-{:04} (1.50 BTC @ $96,800.00)<br/>\
                                            • Traefik → Node Gateway (PASETO v4 Verified)<br/>\
                                            • Redpanda: Published to <code>orders.placed.v1</code><br/>\
                                            • Rust Engine: Matched in 372ns (Price-Time Priority)<br/>\
                                            • Java Loom: Virtual Thread #1844 settled ledger in 1.9ms<br/>\
                                            • PHP Saga: OrderFulfillmentSaga committed to Outbox<br/>\
                                            <span class='text-emerald-400 font-bold'>✔ END-TO-END TRANSACTION SUCCESS (Total: 4.8ms)</span>\
                                            </div>",
                                            &current_ts[11..19],
                                            (js_sys::Math::random() * 9000.0) as u32 + 1000
                                        );
                                        trace_log.set_inner_html(&format!("{}{}", step_html, trace_log.inner_html()));
                                    }
                                }
                            }),
                        button_advanced("📖 Ferrox-Codex Docs", "glass", "sm", false, Some("external-link"))
                            .on_click(|| {
                                if let Some(win) = window() {
                                    let _ = win.open_with_url_and_target("https://ferrox-rust.dev/overview/ecosystem-catalog/", "_blank");
                                }
                            }),
                    ]),
                ]),
        ]);

    // 2. Polyglot Service Cards Grid (4 columns on xl, 2 on md)
    let mut node_cards = Vec::new();
    for node in nodes {
        let feature_badges: Vec<DomBuilder> = node.features.iter().map(|f| {
            span().class("text-[10px] px-2 py-0.5 rounded-md font-mono")
                .attr("style", "background: rgba(255, 255, 255, 0.06); color: var(--text-secondary);")
                .text(f)
        }).collect();

        let card = col("12 md-6 xl-3").child(
            div()
                .class("ferrox-card rounded-2xl p-5 backdrop-blur-xl border shadow-lg transition-all duration-300 hover:shadow-2xl flex flex-col justify-between h-full")
                .attr("style", "background: var(--bg-surface); border-color: var(--border-glass);")
                .children(vec![
                    div().children(vec![
                        div().class("flex items-center justify-between pb-3 mb-3 border-b")
                            .attr("style", "border-bottom-color: var(--border-glass);")
                            .children(vec![
                                div().class("flex items-center gap-2.5").children(vec![
                                    div().class("w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0")
                                        .attr("style", &format!("background: rgba(99, 102, 241, 0.15); color: {};", node.accent_color))
                                        .child(Icon::render(node.icon, 16, node.accent_color)),
                                    div().children(vec![
                                        h3().class("text-sm font-bold text-white tracking-tight").text(node.name),
                                        span().class("text-[10px] text-slate-400 font-mono").text(node.runtime),
                                    ]),
                                ]),
                                badge(node.status, node.status_badge).id(&format!("mesh-status-{}", node.id)),
                            ]),
                        p().class("text-xs text-slate-300 mb-4 min-h-[32px] leading-relaxed").text(node.role),
                        div().class("grid grid-cols-2 gap-2 p-2.5 rounded-xl bg-slate-950/40 border border-slate-800/60 mb-3 font-mono text-[11px]")
                            .children(vec![
                                div().children(vec![
                                    span().class("text-slate-500 block text-[9px] uppercase").text("Port / Protocol"),
                                    span().class("text-slate-200 font-semibold").text(node.port),
                                ]),
                                div().children(vec![
                                    span().class("text-slate-500 block text-[9px] uppercase").text("Latency / Speed"),
                                    span().id(&format!("mesh-latency-{}", node.id)).class("text-emerald-400 font-bold").text(node.latency),
                                ]),
                                div().children(vec![
                                    span().class("text-slate-500 block text-[9px] uppercase").text("Metrics / Footprint"),
                                    span().id(&format!("mesh-metric-{}", node.id)).class("text-slate-300").text(node.memory),
                                ]),
                                div().children(vec![
                                    span().class("text-slate-500 block text-[9px] uppercase").text("Telemetry Guard"),
                                    span().class("text-indigo-400").text("Prometheus / OTel"),
                                ]),
                            ]),
                    ]),
                    div().class("flex flex-wrap gap-1.5 pt-2 border-t border-slate-800/60")
                        .children(feature_badges),
                ])
        );
        node_cards.push(card);
    }
    let nodes_grid = row().children(node_cards);

    // 3. Interactive Transaction Simulator & Trace Console
    let simulator_section = row().children(vec![
        col("12 lg-7").child(
            card_advanced(
                "Cross-Service Transaction Journey",
                "Real-time event choreography across the 6 polyglot runtimes",
                "git-pull-request",
                "glass",
                true,
                "p-6"
            ).children(vec![
                div().class("flex flex-col gap-3 font-mono text-xs").children(vec![
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800 flex items-start gap-3")
                        .children(vec![
                            span().class("w-6 h-6 rounded-full bg-indigo-500/20 text-indigo-400 flex items-center justify-center font-bold flex-shrink-0").text("1"),
                            div().children(vec![
                                span().class("font-bold text-white block").text("Ingress & PASETO v4 Authentication (Node Fastify / nestjs-yalc)"),
                                p().class("text-[11px] text-slate-400 mt-0.5").text("Gateway validates cryptographic footer and generates authenticated CreateOrder Command."),
                            ]),
                        ]),
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800 flex items-start gap-3")
                        .children(vec![
                            span().class("w-6 h-6 rounded-full bg-rose-500/20 text-rose-400 flex items-center justify-center font-bold flex-shrink-0").text("2"),
                            div().children(vec![
                                span().class("font-bold text-white block").text("Streaming Ingestion & Broadcast (Redpanda Kafka Broker)"),
                                p().class("text-[11px] text-slate-400 mt-0.5").text("Order published to topic 'orders.placed.v1' with CloudEvents v1.0 binary header metadata."),
                            ]),
                        ]),
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800 flex items-start gap-3")
                        .children(vec![
                            span().class("w-6 h-6 rounded-full bg-emerald-500/20 text-emerald-400 flex items-center justify-center font-bold flex-shrink-0").text("3"),
                            div().children(vec![
                                span().class("font-bold text-white block").text("Sub-Microsecond Matching (ferrox-match / Rust Engine)"),
                                p().class("text-[11px] text-slate-400 mt-0.5").text("Order matched in 380ns fast-path with zero heap allocation. Emits 'orders.matched.v1'."),
                            ]),
                        ]),
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800 flex items-start gap-3")
                        .children(vec![
                            span().class("w-6 h-6 rounded-full bg-amber-500/20 text-amber-400 flex items-center justify-center font-bold flex-shrink-0").text("4"),
                            div().children(vec![
                                span().class("font-bold text-white block").text("Loom Virtual Thread Settlement (ferrox-java-showcase)"),
                                p().class("text-[11px] text-slate-400 mt-0.5").text("Java 21 Project Loom settles double-entry ledger balance using off-heap memory."),
                            ]),
                        ]),
                    div().class("p-3 rounded-xl bg-slate-900/60 border border-slate-800 flex items-start gap-3")
                        .children(vec![
                            span().class("w-6 h-6 rounded-full bg-purple-500/20 text-purple-400 flex items-center justify-center font-bold flex-shrink-0").text("5"),
                            div().children(vec![
                                span().class("font-bold text-white block").text("Distributed Saga Fulfillment (ferrox-php FrankenPHP)"),
                                p().class("text-[11px] text-slate-400 mt-0.5").text("OrderFulfillmentSaga locks warehouse inventory via Redis Redlock and commits to Outbox."),
                            ]),
                        ]),
                ]),
            ])
        ),
        col("12 lg-5").child(
            card_advanced(
                "Live Mesh Trace Console",
                "Real-time event feed from Docker Mesh Gateway",
                "terminal",
                "glass",
                false,
                "p-6"
            ).children(vec![
                div()
                    .id("mesh-trace-log")
                    .class("flex flex-col gap-2 max-h-[360px] overflow-y-auto pr-1 font-mono text-xs")
                    .children(vec![
                        div().class("p-3 rounded-xl bg-slate-950/80 border border-slate-800 text-slate-400")
                            .children(vec![
                                span().class("text-emerald-400 font-bold block mb-1").text("● Mesh Telemetry Connected"),
                                p().class("text-[11px] text-slate-500").text("Click 'Simulate Distributed Trade' above to trigger an end-to-end multi-runtime order transaction across the entire Ferrox cluster."),
                            ]),
                    ]),
            ])
        ),
    ]);

    container()
        .attr("style", "max-width: 1440px; margin: 0 auto; padding: 2.5rem 2rem; display: flex; flex-direction: column; gap: 28px;")
        .children(vec![
            hero_banner,
            nodes_grid,
            simulator_section,
        ])
}
