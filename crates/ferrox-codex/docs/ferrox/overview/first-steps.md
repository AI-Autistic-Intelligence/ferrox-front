---
sidebar_position: 2
---

# 🚀 First Steps: Getting Started

This guide walks you through building your very first **Ferrox** application from scratch. We will cover environment preparation, application bootstrapping, router setup, and starting the HTTP transport server.

---

## 1. Prerequisites

Before starting, ensure you have the following installed on your system:
- **Rust Toolchain**: 1.75 or later (`rustup update stable`)
- **Cargo**: Standard package manager included with Rust

---

## 2. Setting Up a New Ferrox Project

Create a new binary Cargo package:

```bash
cargo new my-ferrox-app --bin
cd my-ferrox-app
```

Add the core Ferrox dependencies to your `Cargo.toml`:

```toml
[dependencies]
ferrox-app = { path = "../ferrox/crates/ferrox-app" }
ferrox-transports = { path = "../ferrox/crates/ferrox-transports" }
ferrox-logger = { path = "../ferrox/crates/ferrox-logger" }
ferrox-errors = { path = "../ferrox/crates/ferrox-errors" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
```

---

## 3. Creating Your First Controller Route

In Ferrox, route handlers are asynchronous functions returning Axum responses or `Result<T, AppError>`.

Create a route handler in `src/main.rs`:

```rust
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use ferrox_errors::AppError;

async fn get_health() -> Result<Json<Value>, AppError> {
    Ok(Json(json!({
        "status": "up",
        "service": "my-ferrox-app",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
```

---

## 4. Bootstrapping `FerroxApp`

Ferrox uses `FerroxApp` as a central application manager to control lifecycle hooks and manage concurrent transports (HTTP, gRPC, WebSockets).

Complete `src/main.rs`:

```rust
use axum::{routing::get, Json, Router};
use ferrox_app::FerroxApp;
use ferrox_logger::{setup_logger, LoggerConfig};
use ferrox_transports::http::HttpTransport;
use serde_json::{json, Value};
use ferrox_errors::AppError;

async fn get_health() -> Result<Json<Value>, AppError> {
    Ok(Json(json!({
        "status": "up",
        "service": "my-ferrox-app"
    })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Logger & Tracing
    let mut logger_config = LoggerConfig::default();
    logger_config.service_name = "my-ferrox-app".to_string();
    let _sentry_guard = setup_logger(logger_config)?;

    // 2. Assemble Controller Router
    let router = Router::new()
        .route("/health", get(get_health));

    // 3. Configure HTTP Transport Layer
    let http_transport = HttpTransport::new(router, 3000)
        .with_strict_cors(vec!["http://localhost:3000"]);

    // 4. Start Ferrox Lifecycle Manager
    println!("🚀 Launching Ferrox application on port 3000...");
    FerroxApp::new()
        .add_transport(http_transport)
        .start()
        .await?;

    Ok(())
}
```

---

## 5. Running the Application

Run the app using `cargo run`:

```bash
cargo run
```

Test your endpoint using `curl`:

```bash
curl http://localhost:3000/health
```

Output:
```json
{
  "status": "up",
  "service": "my-ferrox-app"
}
```

---

## 6. What's Next?

Now that your first Ferrox server is running, explore:
- 🕹️ [Controllers](../fundamentals/controllers.md) to structure complex routing trees.
- 🧱 [Providers](../fundamentals/providers.md) to inject database connections and business logic.
- 🛡️ [Guards & Security](../security/jwt.md) to secure routes with PASETO JWT tokens.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*