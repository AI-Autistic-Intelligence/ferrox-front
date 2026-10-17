//! # Ferrox Core Microfrontend Module (`ferrox-codex::modules::core`)

use super::CodexModule;

/// Assembles the Core Engine microfrontend module.
pub fn create_core_module() -> CodexModule {
    let mut module = CodexModule::new("core", "🦀 Ferrox Core (Rust)", "/core")
        .with_icon("cpu")
        .with_badge("1M RPS");

    // 1. Core Overview & Architecture
    module.register_doc(
        "overview",
        include_str!("../../../../../ferrox-docs/docs/ferrox/overview.md"),
    );
    module.register_doc(
        "architecture",
        include_str!("../../../../../ferrox-docs/docs/ferrox/overview.md"),
    );

    // 2. 1,000,000 RPS Performance Benchmark
    module.register_doc(
        "benchmarks/1m-rps",
        include_str!("../../../../../ferrox-docs/docs/benchmarks/1m-rps.md"),
    );

    // 3. Security Audit & Zero-Trust Architecture
    module.register_doc(
        "security/audit",
        include_str!("../../../../../ferrox-docs/docs/SECURITY_AUDIT_REPORT.md"),
    );

    // 4. Ecosystem Showcase Guide
    module.register_doc(
        "showcase-guide",
        include_str!("../../../../../ferrox-docs/docs/ECOSYSTEM_SHOWCASE_GUIDE.md"),
    );

    module
}
