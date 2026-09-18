//! # Ferrox Security Dashboard Microfrontend (`ferrox-front-ui::ferrox_security_dashboard`)
//!
//! Reusable, enterprise-grade Security Analytics, Telemetry & OWASP WSTG Audit Dashboard.
//! Includes live ML threat scores, telemetry metrics grid, vulnerability matrix, and executive PDF report exporter.

use ferrox_front_core::dom::{div, h1, span, button as dom_button, DomBuilder};
use crate::components::{card, card_header, card_body, badge, progress_bar, alert};

/// Configuration data structure for initializing the Security Microfrontend
#[derive(Debug, Clone)]
pub struct SecurityDashboardConfig {
    pub target_url: String,
    pub compliance_score: f64,
    pub passed_tests: usize,
    pub total_tests: usize,
    pub active_banned_ips: usize,
    pub brute_force_attempts_today: usize,
    pub rate_limit_violations_today: usize,
    pub sentinel_threat_score: f64,
    pub sentinel_level: String,
}

impl Default for SecurityDashboardConfig {
    fn default() -> Self {
        Self {
            target_url: "http://127.0.0.1:3000".to_string(),
            compliance_score: 100.0,
            passed_tests: 14,
            total_tests: 14,
            active_banned_ips: 0,
            brute_force_attempts_today: 0,
            rate_limit_violations_today: 0,
            sentinel_threat_score: 0.09,
            sentinel_level: "Benign".to_string(),
        }
    }
}

/// An individual security finding item for display in the dashboard matrix
#[derive(Debug, Clone)]
pub struct SecurityFindingItem {
    pub wstg_id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub passed: bool,
    pub title: &'static str,
    pub practical_risk: &'static str,
    pub remediation: &'static str,
}

/// Renders the complete Ferrox Security Microfrontend Dashboard
pub fn ferrox_security_dashboard(config: &SecurityDashboardConfig) -> DomBuilder {
    let findings = get_sample_findings();

    div()
        .attr("class", "ferrox-security-dashboard-container")
        .attr("style", "padding: 1.5rem; display: flex; flex-direction: column; gap: 1.5rem; font-family: system-ui, -apple-system, sans-serif;")
        .child(render_dashboard_header(config))
        .child(render_metrics_grid(config))
        .child(render_three_step_pipeline_section(config))
        .child(render_threat_sentinel_panel(config))
        .child(render_gateway_topology_section())
        .child(render_ip_firewall_section())
        .child(render_vulnerability_matrix(&findings))
        .child(render_vulnerability_dictionary_section())
        .child(render_log_pointers_section())
        .child(render_live_log_terminal_section())
        .child(render_user_moderation_section())
        .child(render_pdf_print_styles())
}

fn render_dashboard_header(config: &SecurityDashboardConfig) -> DomBuilder {
    div()
        .attr("style", "display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; background: var(--bg-glass, rgba(15, 23, 42, 0.75)); padding: 1.25rem 1.5rem; border-radius: 12px; border: var(--border-glass, 1px solid rgba(255,255,255,0.1));")
        .child(
            div()
                .child(
                    h1()
                        .attr("style", "margin: 0; font-size: 1.5rem; font-weight: 800; color: #f8fafc; display: flex; align-items: center; gap: 10px;")
                        .text("🛡️ Ferrox Security Telemetry & OWASP Audit Dashboard")
                )
                .child(
                    span()
                        .attr("style", "font-size: 0.88rem; opacity: 0.8; color: #94a3b8; margin-top: 4px; display: block;")
                        .text(&format!("Target System: {} | Real-time SOAR & ML Telemetry Active", config.target_url))
                )
        )
        .child(
            div()
                .attr("style", "display: flex; align-items: center; gap: 12px;")
                .child(badge("LIVE TELEMETRY", "success"))
                .child(
                    dom_button()
                        .attr("class", "ferrox-btn ferrox-btn-primary")
                        .attr("style", "background: linear-gradient(135deg, #0284c7, #2563eb); color: #ffffff; border: none; padding: 10px 18px; border-radius: 8px; font-weight: 700; font-size: 0.9rem; cursor: pointer; display: inline-flex; align-items: center; gap: 8px; box-shadow: 0 4px 14px rgba(37,99,235,0.4);")
                        .attr("onclick", "window.print()")
                        .text("📄 Export Executive PDF Report")
                )
        )
}

fn render_metrics_grid(config: &SecurityDashboardConfig) -> DomBuilder {
    div()
        .attr("style", "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem;")
        .child(render_metric_card("OWASP Compliance Score", &format!("{:.1}%", config.compliance_score), &format!("{}/{} Passed", config.passed_tests, config.total_tests), "success"))
        .child(render_metric_card("Sentinel Threat Score", &format!("{:.2}", config.sentinel_threat_score), &format!("Level: {}", config.sentinel_level), if config.sentinel_threat_score < 0.5 { "info" } else { "danger" }))
        .child(render_metric_card("Active Banned IPs", &config.active_banned_ips.to_string(), "Zero-Trust MTD Active", "warning"))
        .child(render_metric_card("Brute Force Blocked", &config.brute_force_attempts_today.to_string(), &format!("Rate Limit: {} Violations", config.rate_limit_violations_today), "primary"))
}

fn render_metric_card(title: &str, value: &str, subtitle: &str, variant: &str) -> DomBuilder {
    card()
        .child(
            div()
                .attr("style", "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;")
                .child(span().attr("style", "font-size: 0.85rem; font-weight: 600; color: #94a3b8;").text(title))
                .child(badge(variant, variant))
        )
        .child(
            div()
                .attr("style", "font-size: 1.85rem; font-weight: 800; color: #f8fafc; margin-bottom: 0.25rem;")
                .text(value)
        )
        .child(
            span()
                .attr("style", "font-size: 0.8rem; color: #cbd5e1; opacity: 0.9;")
                .text(subtitle)
        )
}

fn render_threat_sentinel_panel(config: &SecurityDashboardConfig) -> DomBuilder {
    card()
        .child(card_header("🧠 Ferrox Sentinel ML Engine Threat Gauge"))
        .child(
            card_body().child(
                div()
                    .attr("style", "display: flex; flex-direction: column; gap: 1rem;")
                    .child(
                        alert(
                            &format!("Sentinel Machine Learning Threat Score is {:.2} ({}) under active Shannon Entropy and Feature Reduction monitoring.", config.sentinel_threat_score, config.sentinel_level),
                            if config.sentinel_threat_score < 0.5 { "success" } else { "danger" }
                        )
                    )
                    .child(
                        div()
                            .attr("style", "display: flex; flex-direction: column; gap: 6px;")
                            .child(
                                div()
                                    .attr("style", "display: flex; justify-content: space-between; font-size: 0.85rem; font-weight: 600; color: #94a3b8;")
                                    .child(span().text("Adversarial Threat Spectrum"))
                                    .child(span().text(&format!("{:.1}% Risk", config.sentinel_threat_score * 100.0)))
                            )
                            .child(progress_bar(config.sentinel_threat_score * 100.0))
                    )
            )
        )
}

fn render_vulnerability_matrix(findings: &[SecurityFindingItem]) -> DomBuilder {
    card()
        .child(card_header("🔍 OWASP WSTG Audit Control & Vulnerability Matrix"))
        .child(
            card_body().child(
                div()
                    .attr("style", "overflow-x: auto;")
                    .child(
                        DomBuilder::new("table")
                            .attr("style", "width: 100%; border-collapse: collapse; text-align: left; font-size: 0.88rem; color: #cbd5e1;")
                            .child(
                                DomBuilder::new("thead")
                                    .attr("style", "background: rgba(255,255,255,0.05); border-bottom: 1px solid rgba(255,255,255,0.1); color: #38bdf8; text-transform: uppercase; font-size: 0.75rem;")
                                    .child(
                                        DomBuilder::new("tr")
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("WSTG Reference"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Category"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Status"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Test Title"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Practical Risk Impact"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("OWASP Remediation Blueprint"))
                                    )
                            )
                            .child(render_matrix_body(findings))
                    )
            )
        )
}

fn render_matrix_body(findings: &[SecurityFindingItem]) -> DomBuilder {
    let mut tbody = DomBuilder::new("tbody");
    for f in findings {
        let status_badge = if f.passed { badge("PASS", "success") } else { badge("FAIL", "danger") };
        let sev_badge = match f.severity {
            "CRITICAL" => badge("CRITICAL", "danger"),
            "HIGH" => badge("HIGH", "danger"),
            "MEDIUM" => badge("MEDIUM", "warning"),
            "LOW" => badge("LOW", "info"),
            _ => badge("INFO", "primary"),
        };

        let tr = DomBuilder::new("tr")
            .attr("style", "border-bottom: 1px solid rgba(255,255,255,0.05); transition: background 0.15s ease;")
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 700; color: #f8fafc;").text(f.wstg_id))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-family: monospace;").text(f.category))
            .child(DomBuilder::new("td").attr("style", "padding: 12px;").child(status_badge))
            .child(
                DomBuilder::new("td")
                    .attr("style", "padding: 12px; font-weight: 600; color: #f1f5f9;")
                    .child(div().text(f.title))
                    .child(div().attr("style", "margin-top: 4px;").child(sev_badge))
            )
            .child(DomBuilder::new("td").attr("style", "padding: 12px; max-width: 260px; color: #e2e8f0;").text(f.practical_risk))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; max-width: 260px; color: #38bdf8; font-family: monospace; font-size: 0.82rem;").text(f.remediation));

        tbody = tbody.child(tr);
    }
    tbody
}

fn render_pdf_print_styles() -> DomBuilder {
    DomBuilder::new("style")
        .text(r#"
            @media print {
                body {
                    background: #ffffff !important;
                    color: #0f172a !important;
                }
                .ferrox-security-dashboard-container {
                    padding: 0 !important;
                }
                .ferrox-glass-card {
                    background: #ffffff !important;
                    border: 1px solid #cbd5e1 !important;
                    box-shadow: none !important;
                    break-inside: avoid;
                }
                .ferrox-btn {
                    display: none !important;
                }
            }
        "#)
}

fn get_sample_findings() -> Vec<SecurityFindingItem> {
    vec![
        SecurityFindingItem {
            wstg_id: "WSTG-CONF-001.1",
            category: "WSTG-CONF",
            severity: "HIGH",
            passed: true,
            title: "HTTP Strict Transport Security (HSTS)",
            practical_risk: "Enables MitM attackers on public networks to intercept unencrypted traffic.",
            remediation: "Strict-Transport-Security: max-age=31536000; includeSubDomains; preload",
        },
        SecurityFindingItem {
            wstg_id: "WSTG-CONF-001.2",
            category: "WSTG-CONF",
            severity: "HIGH",
            passed: true,
            title: "Content Security Policy (CSP)",
            practical_risk: "Allows execution of malicious inline scripts (XSS) and token theft.",
            remediation: "Define strict Content-Security-Policy header restricting origins.",
        },
        SecurityFindingItem {
            wstg_id: "WSTG-CONF-001.3",
            category: "WSTG-CONF",
            severity: "MEDIUM",
            passed: true,
            title: "Clickjacking Protection (X-Frame-Options)",
            practical_risk: "Attackers embed site in iFrames to trick users into state-changing actions.",
            remediation: "X-Frame-Options: DENY or SAMEORIGIN in middleware.",
        },
        SecurityFindingItem {
            wstg_id: "WSTG-CONF-001.4",
            category: "WSTG-CONF",
            severity: "LOW",
            passed: true,
            title: "MIME-Type Sniffing Protection",
            practical_risk: "Browsers treat non-executable files as HTML/JS, introducing secondary XSS.",
            remediation: "X-Content-Type-Options: nosniff header.",
        },
        SecurityFindingItem {
            wstg_id: "WSTG-ATHN-003",
            category: "WSTG-ATHN",
            severity: "HIGH",
            passed: true,
            title: "Argon2 Password Hashing Verification",
            practical_risk: "Non-memory-hard password hashing permits high-speed GPU offline cracking.",
            remediation: "ferrox-security Argon2id standard (m=19456, t=2, p=1).",
        },
        SecurityFindingItem {
            wstg_id: "WSTG-SESS-003",
            category: "WSTG-SESS",
            severity: "HIGH",
            passed: true,
            title: "PASETO v4 Session Token Verification",
            practical_risk: "Flawed session token formatting permits forgery and session hijacking.",
            remediation: "ferrox-security PASETO v4 local token with 32-byte key.",
        },
    ]
}

fn render_log_pointers_section() -> DomBuilder {
    let pointers = vec![
        ("Brute Force & Rate Limiting", "logs/sentinel-2026-W37.log", "#L120-L380", 42, "2026-09-09 to 2026-09-16"),
        ("Sentinel ML Threat Anomalies", "logs/sentinel-2026-W37.log", "#L381-L520", 14, "2026-09-09 to 2026-09-16"),
        ("File Upload Heuristic Blocks", "logs/uploads-2026-09.log", "#L15-L85", 6, "2026-09-01 to 2026-09-16"),
    ];

    let mut tbody = DomBuilder::new("tbody");
    for (cat, file, lines, count, time_range) in pointers {
        let tr = DomBuilder::new("tr")
            .attr("style", "border-bottom: 1px solid rgba(255,255,255,0.05);")
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 600; color: #f8fafc;").text(cat))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-family: monospace; color: #38bdf8;").text(file))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-family: monospace; font-weight: 700; color: #f59e0b;").text(lines))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 700; color: #34d399;").text(&count.to_string()))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; color: #94a3b8; font-size: 0.82rem;").text(time_range));

        tbody = tbody.child(tr);
    }

    card()
        .child(card_header("🔍 Forensic Log Block Audit Pointers (Weekly / Monthly)"))
        .child(
            card_body().child(
                div()
                    .attr("style", "overflow-x: auto;")
                    .child(
                        DomBuilder::new("table")
                            .attr("style", "width: 100%; border-collapse: collapse; text-align: left; font-size: 0.88rem; color: #cbd5e1;")
                            .child(
                                DomBuilder::new("thead")
                                    .attr("style", "background: rgba(255,255,255,0.05); border-bottom: 1px solid rgba(255,255,255,0.1); color: #38bdf8; text-transform: uppercase; font-size: 0.75rem;")
                                    .child(
                                        DomBuilder::new("tr")
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Threat Category"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Log File Location"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Line Range"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Event Count"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Timestamp Range"))
                                    )
                            )
                            .child(tbody)
                    )
            )
        )
}

fn render_gateway_topology_section() -> DomBuilder {
    let instances = vec![
        ("node-eu-west-1a", "10.0.1.42:3000", 142, 500, "Healthy"),
        ("node-eu-west-1b", "10.0.1.43:3000", 118, 500, "Healthy"),
        ("node-us-east-1a", "10.0.2.11:3000", 86, 500, "Healthy"),
    ];

    let mut tbody = DomBuilder::new("tbody");
    for (name, addr, conns, cap, status) in instances {
        let load_pct = (conns as f64 / cap as f64) * 100.0;
        let tr = DomBuilder::new("tr")
            .attr("style", "border-bottom: 1px solid rgba(255,255,255,0.05);")
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 700; color: #f8fafc;").text(name))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-family: monospace; color: #38bdf8;").text(addr))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 700; color: #cbd5e1;").text(&format!("{}/{}", conns, cap)))
            .child(DomBuilder::new("td").attr("style", "padding: 12px; font-weight: 600; color: #34d399;").text(&format!("{:.1}%", load_pct)))
            .child(DomBuilder::new("td").attr("style", "padding: 12px;").child(badge(status, "success")));

        tbody = tbody.child(tr);
    }

    card()
        .child(card_header("🌐 Gateway Cluster & Load Balancer Topology"))
        .child(
            card_body().child(
                div()
                    .attr("style", "overflow-x: auto;")
                    .child(
                        DomBuilder::new("table")
                            .attr("style", "width: 100%; border-collapse: collapse; text-align: left; font-size: 0.88rem; color: #cbd5e1;")
                            .child(
                                DomBuilder::new("thead")
                                    .attr("style", "background: rgba(255,255,255,0.05); border-bottom: 1px solid rgba(255,255,255,0.1); color: #38bdf8; text-transform: uppercase; font-size: 0.75rem;")
                                    .child(
                                        DomBuilder::new("tr")
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Node ID"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Internal Address"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Active Conns / Cap"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Load Factor"))
                                            .child(DomBuilder::new("th").attr("style", "padding: 12px;").text("Node Health"))
                                    )
                            )
                            .child(tbody)
                    )
            )
        )
}

fn render_ip_firewall_section() -> DomBuilder {
    card()
        .child(card_header("🧱 IP Access & Emergency Firewall Control"))
        .child(
            card_body().child(
                div()
                    .attr("style", "display: flex; flex-direction: column; gap: 1rem;")
                    .child(
                        div()
                            .attr("style", "display: flex; gap: 12px; align-items: center;")
                            .child(
                                DomBuilder::new("input")
                                    .attr("type", "text")
                                    .attr("placeholder", "Enter IP Address to Ban (e.g. 192.168.1.50)")
                                    .attr("style", "flex: 1; padding: 10px 14px; border-radius: 6px; background: rgba(0,0,0,0.4); color: #fff; border: 1px solid rgba(255,255,255,0.15); font-size: 0.9rem;")
                            )
                            .child(
                                dom_button()
                                    .attr("class", "ferrox-btn ferrox-btn-danger")
                                    .attr("style", "background: linear-gradient(135deg, #dc2626, #991b1b); color: #ffffff; border: none; padding: 10px 18px; border-radius: 6px; font-weight: 700; cursor: pointer;")
                                    .text("⛔ Emergency Ban IP")
                            )
                    )
                    .child(
                        alert("Zero-Trust MTD Active: Banned IPs are immediately rejected by Sentinel Middleware prior to handler dispatch.", "info")
                    )
            )
        )
}

fn render_live_log_terminal_section() -> DomBuilder {
    let sample_logs = r#"[2026-09-16T15:16:14Z INFO ferrox_logger] Telemetria registrata: conns=142, rooms=12, banned_ips=0
[2026-09-16T15:16:29Z INFO ferrox_sentinel::middleware] GET /api/version - Threat Score: 0.09 (Benign)
[2026-09-16T15:16:45Z WARN ferrox_security::threats] Rate limiter check passed for client IP 127.0.0.1
[2026-09-16T15:17:01Z INFO ferrox_selftest::auditor] OWASP WSTG Audit scan executed against http://127.0.0.1:3000 - Compliance: 100.0% PASS"#;

    card()
        .child(card_header("💻 Live System Log Terminal Viewer"))
        .child(
            card_body().child(
                div()
                    .attr("style", "background: #090d16; border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; padding: 1rem; font-family: monospace; font-size: 0.85rem; color: #4ade80; line-height: 1.6; white-space: pre-wrap; overflow-x: auto; max-height: 220px;")
                    .text(sample_logs)
            )
        )
}

fn render_user_moderation_section() -> DomBuilder {
    card()
        .child(card_header("👤 Player & Account Security Moderation"))
        .child(
            card_body().child(
                div()
                    .attr("style", "display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;")
                    .child(
                        div()
                            .child(span().attr("style", "font-weight: 700; color: #f8fafc; font-size: 1rem;").text("PASETO Token Session Revocation & MTD Seed Mutation"))
                            .child(span().attr("style", "display: block; font-size: 0.83rem; color: #94a3b8; margin-top: 2px;").text("Instantly invalidate active user sessions across the cluster and trigger MTD seed rotation."))
                    )
                    .child(
                        dom_button()
                            .attr("class", "ferrox-btn ferrox-btn-warning")
                            .attr("style", "background: linear-gradient(135deg, #d97706, #b45309); color: #ffffff; border: none; padding: 10px 18px; border-radius: 6px; font-weight: 700; cursor: pointer;")
                            .text("🔄 Force MTD Seed Rotation")
                    )
            )
        )
}

fn render_three_step_pipeline_section(config: &SecurityDashboardConfig) -> DomBuilder {
    card()
        .child(card_header("🔄 Interactive 3-Step Audit & Verification Pipeline"))
        .child(
            card_body().child(
                div()
                    .attr("style", "display: flex; flex-direction: column; gap: 1rem;")
                    .child(
                        div()
                            .attr("style", "display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 1rem;")
                            .child(
                                div()
                                    .attr("style", "background: rgba(15, 23, 42, 0.6); padding: 1rem; border-radius: 8px; border: 1px solid rgba(255,255,255,0.1);")
                                    .child(div().attr("style", "font-weight: 700; color: #38bdf8; margin-bottom: 4px;").text("Step 1: Staging Boot & Hardening"))
                                    .child(div().attr("style", "font-size: 0.85rem; color: #94a3b8; margin-bottom: 8px;").text(&format!("Target: {} (SYSCTL + nftables Active)", config.target_url)))
                                    .child(badge("COMPLETED", "success"))
                            )
                            .child(
                                div()
                                    .attr("style", "background: rgba(15, 23, 42, 0.6); padding: 1rem; border-radius: 8px; border: 1px solid rgba(255,255,255,0.1);")
                                    .child(div().attr("style", "font-weight: 700; color: #38bdf8; margin-bottom: 4px;").text("Step 2: Data & Session Prep"))
                                    .child(div().attr("style", "font-size: 0.85rem; color: #94a3b8; margin-bottom: 8px;").text("PASETO Tokens, Datasets & Baseline Verified"))
                                    .child(badge("READY", "success"))
                            )
                            .child(
                                div()
                                    .attr("style", "background: rgba(15, 23, 42, 0.6); padding: 1rem; border-radius: 8px; border: 1px solid rgba(255,255,255,0.1);")
                                    .child(div().attr("style", "font-weight: 700; color: #38bdf8; margin-bottom: 4px;").text("Step 3: Execute Audit & PDF Export"))
                                    .child(div().attr("style", "font-size: 0.85rem; color: #94a3b8; margin-bottom: 8px;").text("Run OWASP WSTG Scan + Vulnerability Dictionary"))
                                    .child(
                                        dom_button()
                                            .attr("class", "ferrox-btn ferrox-btn-primary")
                                            .attr("style", "background: linear-gradient(135deg, #059669, #10b981); color: #fff; border: none; padding: 6px 14px; border-radius: 6px; font-weight: 700; font-size: 0.85rem; cursor: pointer;")
                                            .attr("onclick", "window.print()")
                                            .text("▶ Esegui Self-Test & PDF")
                                    )
                            )
                    )
            )
        )
}

fn render_vulnerability_dictionary_section() -> DomBuilder {
    card()
        .child(card_header("📖 Vocabolario delle Vulnerabilità OWASP WSTG (Guida Divulgativa)"))
        .child(
            card_body().child(
                div()
                    .attr("style", "display: flex; flex-direction: column; gap: 0.75rem;")
                    .child(render_dict_item("WSTG-CONF-001", "HSTS Header Missing", "Obbliga il browser a usare HTTPS cifrato, impedendo intercettazioni di traffico (MitM).", "Configurare 'Strict-Transport-Security: max-age=31536000'"))
                    .child(render_dict_item("WSTG-CONF-002", "CSP Header Missing", "Definisce le risorse autorizzate, bloccando attacchi di iniezione di script malevoli (XSS).", "Impostare 'Content-Security-Policy: default-src 'self''"))
                    .child(render_dict_item("WSTG-CONF-003", "X-Frame-Options Missing", "Impedisce a siti malevoli di incorporare l'app in iframe invisibili (Clickjacking).", "Impostare 'X-Frame-Options: DENY'"))
                    .child(render_dict_item("WSTG-SESS-002", "Cookie Flags Missing", "Protegge i cookie di sessione dall'accesso via JavaScript in caso di XSS.", "Impostare '; HttpOnly; Secure; SameSite=Strict'"))
            )
        )
}

fn render_dict_item(id: &str, title: &str, concept: &str, remediation: &str) -> DomBuilder {
    div()
        .attr("style", "background: rgba(0,0,0,0.3); padding: 0.85rem 1rem; border-radius: 8px; border-left: 4px solid #38bdf8;")
        .child(
            div()
                .attr("style", "display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;")
                .child(span().attr("style", "font-weight: 700; color: #f8fafc; font-size: 0.95rem;").text(&format!("[{}] {}", id, title)))
                .child(badge("OWASP WSTG", "info"))
        )
        .child(div().attr("style", "font-size: 0.85rem; color: #cbd5e1; margin-bottom: 4px;").text(&format!("💡 Concetto: {}", concept)))
        .child(div().attr("style", "font-size: 0.83rem; color: #34d399; font-weight: 600;").text(&format!("🛠️ Remediation: {}", remediation)))
}
