---
sidebar_position: 1
---

# 📊 Structured Logging & Sentry Integration

Production observability begins with structured logging. Plain text `println!` statements are difficult to search in log aggregators like Datadog, Grafana Loki, or AWS CloudWatch.

Ferrox provides `ferrox-logger`—a crate that initializes structured JSON logging via `tracing-subscriber` and configures automatic error reporting via **Sentry**.

---

## 1. Initializing the Logger

Call `setup_logger` at the beginning of `main.rs`:

```rust
use ferrox_logger::{setup_logger, LoggerConfig};
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = LoggerConfig::default();
    config.service_name = "my-service".to_string();
    config.environment = "production".to_string();

    // Initializes tracing subscriber and optional Sentry guard
    let _sentry_guard = setup_logger(config)?;

    info!("🚀 Application initialized with structured logging!");
    Ok(())
}
```

---

## 2. Emitting Structured Log Spans

Use the standard `tracing` macros (`info!`, `warn!`, `error!`, `debug!`) to attach key-value metadata to logs:

```rust
use tracing::info;

pub async fn process_order(order_id: u64, user_id: u64, amount: f64) {
    info!(
        order_id = order_id,
        user_id = user_id,
        amount = amount,
        "Order processing started"
    );
}
```

Output JSON log line:
```json
{
  "timestamp": "2026-09-05T10:00:00Z",
  "level": "INFO",
  "fields": {
    "order_id": 42,
    "user_id": 101,
    "amount": 99.95,
    "message": "Order processing started"
  },
  "target": "my_service::orders"
}
```

---

## 3. Sentry Integration

When `LoggerConfig.sentry_dsn` is configured, all `error!` events automatically create detailed Sentry exception reports complete with stack traces, environment tags, and request contexts.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*