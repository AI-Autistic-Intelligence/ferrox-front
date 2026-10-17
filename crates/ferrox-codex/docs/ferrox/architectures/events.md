---
sidebar_position: 4
---

# 📡 Event-Driven Architecture & Event Bus

Event-driven architecture decouples producers of business events from subscribers that react to them (e.g. sending welcome emails when a `UserCreatedEvent` fires, updating analytics counters, or invalidating caches).

`ferrox-events` provides strongly typed domain events and asynchronous in-memory dispatchers.

---

## 1. Defining a `DomainEvent`

Implement the `DomainEvent` trait on your event structs:

```rust
use serde::{Deserialize, Serialize};
use ferrox_events::DomainEvent;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserCreatedEvent {
    pub user_id: u64,
    pub email: String,
}

impl DomainEvent for UserCreatedEvent {
    fn event_name(&self) -> &'static str {
        "UserCreated"
    }
}
```

---

## 2. Pub/Sub with `InMemoryDispatcher`

`InMemoryDispatcher` uses Tokio broadcast channels for zero-cost, high-speed in-process event streaming:

```rust
use ferrox_events::{InMemoryDispatcher, EventDispatcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Event Bus with capacity for 100 queued events
    let dispatcher = InMemoryDispatcher::<UserCreatedEvent>::new(100);

    // 1. Subscribe Event Listeners
    let mut rx = dispatcher.subscribe().await?;

    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            println!("📧 Welcome Email Listener: Sending email to {}", event.email);
        }
    });

    // 2. Publish Domain Event from Service
    let event = UserCreatedEvent {
        user_id: 101,
        email: "user@ferrox-rust.dev".into(),
    };

    dispatcher.publish(event).await?;

    Ok(())
}
```

---

## 3. ✅ Best Practices

- **Make Event Handlers Idempotent**: Design event listeners to safely process duplicate events without unintended side effects.
- **Offload Heavy Listener Tasks**: Spawn background Tokio tasks for event handlers so publishing domain events never delays HTTP responses.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*