//! # Ferrox Node & Yalc Microfrontend Module (`ferrox-codex::modules::node`)

use super::CodexModule;

/// Assembles the Node & Yalc ecosystem microfrontend module.
pub fn create_node_module() -> CodexModule {
    let mut module = CodexModule::new("node", "⚡ Ferrox Node & Yalc", "/node")
        .with_icon("terminal")
        .with_badge("NestJS/Fastify");

    // 1. Ferrox Node Overview
    module.register_doc(
        "overview",
        include_str!("../../docs/ferrox-node/overview.md"),
    );

    // 2. Node-Yalc Core Transports & Abstractions
    module.register_doc(
        "yalc",
        include_str!("../../docs/node-yalc/overview.md"),
    );
    module.register_doc(
        "node-yalc",
        include_str!("../../docs/node-yalc/overview.md"),
    );

    // 3. NestJS-Yalc Enterprise Architecture
    module.register_doc(
        "nestjs-yalc",
        include_str!("../../docs/nestjs-yalc/overview.md"),
    );

    module
}
