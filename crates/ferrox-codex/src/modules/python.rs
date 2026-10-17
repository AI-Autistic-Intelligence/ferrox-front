//! # Ferrox Python Microfrontend Module with Nested Submodules (`ferrox-codex::modules::python`)

use super::CodexModule;

/// Assembles the Python microfrontend module with nested auth, commerce, and utils submodules.
pub fn create_python_module() -> CodexModule {
    let mut main_module = CodexModule::new("py", "🐍 Ferrox Python (3.12+)", "/py")
        .with_icon("terminal")
        .with_badge("Asyncio");

    // 1. Python Root Overview
    main_module.register_doc(
        "overview",
        include_str!("../../docs/ferrox-py/overview.md"),
    );

    // 2. Auth Submodule (/py/auth)
    let mut auth_sub = CodexModule::new("py-auth", "🔐 Authentication & RBAC", "auth")
        .with_icon("shield")
        .with_badge("PASETO");

    auth_sub.register_doc(
        "overview",
        include_str!("../../docs/ferrox-py/auth/overview.md"),
    );
    auth_sub.register_doc(
        "auth_service",
        include_str!("../../docs/ferrox-py/auth/auth_service.md"),
    );
    auth_sub.register_doc(
        "gdpr_service",
        include_str!("../../docs/ferrox-py/auth/gdpr_service.md"),
    );
    auth_sub.register_doc(
        "models",
        include_str!("../../docs/ferrox-py/auth/models.md"),
    );
    auth_sub.register_doc(
        "rbac",
        include_str!("../../docs/ferrox-py/auth/rbac.md"),
    );

    // 3. Commerce Submodule (/py/commerce)
    let mut commerce_sub = CodexModule::new("py-commerce", "🛒 Commerce & Payments", "commerce")
        .with_icon("activity")
        .with_badge("Stripe/PayPal");

    commerce_sub.register_doc(
        "overview",
        include_str!("../../docs/ferrox-py/commerce/overview.md"),
    );
    commerce_sub.register_doc(
        "controllers",
        include_str!("../../docs/ferrox-py/commerce/controllers.md"),
    );
    commerce_sub.register_doc(
        "gateways",
        include_str!("../../docs/ferrox-py/commerce/gateways.md"),
    );
    commerce_sub.register_doc(
        "transaction_state",
        include_str!("../../docs/ferrox-py/commerce/transaction_state.md"),
    );
    commerce_sub.register_doc(
        "webhooks",
        include_str!("../../docs/ferrox-py/commerce/webhooks.md"),
    );

    // 4. Utils Submodule (/py/utils)
    let mut utils_sub = CodexModule::new("py-utils", "🛠️ Utilities & Pipelines", "utils")
        .with_icon("cpu")
        .with_badge("Resilience");

    utils_sub.register_doc(
        "overview",
        include_str!("../../docs/ferrox-py/utils/overview.md"),
    );
    utils_sub.register_doc(
        "connectors",
        include_str!("../../docs/ferrox-py/utils/connectors.md"),
    );
    utils_sub.register_doc(
        "pipelines",
        include_str!("../../docs/ferrox-py/utils/pipelines.md"),
    );
    utils_sub.register_doc(
        "schemas",
        include_str!("../../docs/ferrox-py/utils/schemas.md"),
    );

    // Attach recursive submodules
    main_module = main_module
        .with_submodule(auth_sub)
        .with_submodule(commerce_sub)
        .with_submodule(utils_sub);

    main_module
}
