---
sidebar_position: 1
---

# 🚀 Part 1: Project Setup

Welcome to the Ferrox Zero-to-Hero Tutorial.

In this tutorial, we will build a complete **Ticketing / Task Management** system. We won't just write a "Hello World". We will build a production-ready microservice with in-memory state, JWT Security, DTO Validation, and automatic TypeScript Client generation.

## 1. Installation

First, ensure you have Rust installed. We will create a new binary project using Cargo.

```bash
cargo new ferrox-ticketing
cd ferrox-ticketing
```

## 2. Adding Dependencies

We need the Ferrox core ecosystem, along with `tokio` (our async runtime), `serde` (for JSON serialization), and `ts-rs` (for TypeScript generation).

Open your `Cargo.toml` and add the following dependencies:

```toml
[dependencies]
ferrox-core = { version = "1.0", features = ["full"] }
ferrox-security = "1.0"
ferrox-errors = "1.0"

tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
validator = { version = "0.16", features = ["derive"] }
ts-rs = "8.1"
```

## 3. The Application Entrypoint

Ferrox applications use `tokio::main` to start the asynchronous runtime.

Open `src/main.rs` and set up the basic scaffolding. We will define an `AppState` that will eventually hold our Database Provider.

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    // We will inject our TaskService here in the next chapter
}

#[tokio::main]
async fn main() {
    // 1. Initialize the Global State
    let state = AppState {};

    // 2. Create the Ferrox Router
    let app = Router::new()
        .route("/health", get(|| async { "API is running!" }))
        .with_state(state);

    // 3. Bind the TCP Listener and Start the Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Ferrox Ticketing Server running on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Run `cargo run` and visit `http://localhost:3000/health`. You have successfully bootstrapped a Ferrox application!

In the next chapter, we will build the core domain logic: DTOs, the Task Service, and the Controller.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*