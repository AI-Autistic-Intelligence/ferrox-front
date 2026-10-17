//! # Ferrox Polyglot Microfrontend Module (`ferrox-codex::modules::polyglot`)

use super::CodexModule;

/// Assembles the Polyglot Runtimes microfrontend module (PHP & Java).
pub fn create_polyglot_module() -> CodexModule {
    let mut module = CodexModule::new("polyglot", "🌐 Polyglot Runtimes", "/polyglot")
        .with_icon("globe")
        .with_badge("PHP/Java");

    // 1. Polyglot Overview
    module.register_doc(
        "overview",
        include_str!("../../../../../ferrox-docs/docs/overview/example-projects.md"),
    );

    // 2. Ferrox-PHP Overview
    module.register_doc(
        "php",
        include_str!("../../../../../ferrox-docs/docs/ferrox-php/overview.md"),
    );

    // 3. Ferrox-PHP API Reference & Saga
    module.register_doc(
        "php/api-reference",
        include_str!("../../../../../ferrox-docs/docs/ferrox-php/api-reference.md"),
    );

    // 4. Ferrox-Java (Java 21 Loom, Project Panama Off-Heap, Sentinel)
    module.register_doc(
        "java",
        include_str!("../../../../../ferrox-java/README.md"),
    );

    module
}
