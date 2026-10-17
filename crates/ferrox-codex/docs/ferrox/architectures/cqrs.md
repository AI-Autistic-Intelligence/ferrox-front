---
sidebar_position: 1
---

# 🗃️ CQRS (Command Query Responsibility Segregation)

CQRS is an architectural pattern that separates reading data (Queries) from writing data (Commands). 

In traditional CRUD architectures, the same data model is used to query and update a database. As applications scale, this becomes a severe bottleneck. The read model and the write model have entirely different performance characteristics, caching strategies, and security requirements.

Ferrox natively provides an in-memory **Command Bus** and **Query Bus** via the `ferrox-cqrs` crate, bringing the Developer Experience of `@nestjs/cqrs` to Rust.

## High-Level Example

In CQRS, you never call a database repository directly from a Controller. Instead, you dispatch a Command.

```rust
use axum::{extract::State, Json};
use ferrox_cqrs::{CommandBus, Command};
use serde::{Deserialize, Serialize};

// 1. Define the Command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
}
impl Command for CreateUserCommand {
    type Result = String; // Returns the new User ID
}

// 2. Dispatch it from the Controller
async fn create_user(
    State(bus): State<CommandBus>,
    Json(payload): Json<CreateUserCommand>
) -> Json<String> {
    
    // The Controller doesn't know HOW the user is created.
    // It just dispatches the intent.
    let user_id = bus.execute(payload).await.unwrap();
    
    Json(user_id)
}
```

Behind the scenes, you register a `CommandHandler` that listens for `CreateUserCommand` and performs the actual database `INSERT`.

## Low-Level Internal Details

The Ferrox Command Bus is built on top of `tokio::sync::mpsc` (Multi-Producer, Single-Consumer) channels and dynamic trait objects (`Box<dyn Any>`).

When a Controller calls `bus.execute(command)`, it sends the command payload into a high-throughput Tokio channel. The worker thread that was registered with the specific `CommandHandler` receives the payload, executes the business logic asynchronously, and sends the `Result` back through a `oneshot` channel.

### Why is this Enterprise-grade?
1. **Thread Safety**: Controllers and Database Writers operate on different threads without needing Mutex locks.
2. **Event Sourcing Ready**: Because every mutation is an explicit `Command` struct, it is trivial to log these commands to an Event Store (like Kafka or EventStoreDB) before executing them, achieving full Event Sourcing.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*