---
sidebar_position: 2
---

# ⚡ Caching & Stampede Protection

In high-traffic Enterprise applications, Caching is mandatory. However, caching introduces a catastrophic vulnerability known as a **Cache Stampede** (or Dog-Piling).

If a highly requested cache key expires (e.g., the homepage configuration of an e-commerce site on Black Friday), 1,000 concurrent requests will miss the cache simultaneously. All 1,000 requests will query the database at the exact same millisecond, instantly crashing your PostgreSQL server.

Ferrox mitigates this natively using the `ferrox-singleflight` crate.

## High-Level Example

Wrapping your database queries in a `SingleFlight` orchestrator ensures that duplicate concurrent queries are deduplicated.

```rust
use axum::{extract::State, Json};
use ferrox_singleflight::SingleFlight;

async fn get_heavy_data(State(sf): State<SingleFlight>) -> Json<String> {
    
    // If 1,000 users hit this route concurrently, the closure is only executed ONCE.
    // The other 999 users will wait, and then receive the exact same result automatically!
    let data = sf.do_async("heavy_db_query_key", || async {
        
        // Simulate heavy database query
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok::<String, String>("Massive Data Payload".into())
        
    }).await.unwrap();

    Json(data)
}
```

## Low-Level Internal Details

How does `SingleFlight` suspend 999 requests and wake them up with the result of the 1st request?

It uses `tokio::sync::broadcast` channels and a `DashMap` (a blazing fast concurrent Hash Map).

1. Request #1 asks for `"key_A"`. The `DashMap` is empty. Request #1 inserts a `tokio::sync::broadcast::Sender` into the `DashMap` and starts the actual database query closure.
2. Requests #2 to #1000 ask for `"key_A"`. They see the `Sender` is already in the `DashMap`. Instead of querying the database, they subscribe to the channel (`sender.subscribe()`) and yield the CPU (`.await`).
3. Request #1 finishes the database query. It broadcasts the result to the channel.
4. Requests #2 to #1000 instantly wake up, receive the broadcasted data, and return it to the client. The `DashMap` entry is then cleared.

This eliminates Cache Stampedes completely, allowing your Rust backend to absorb millions of requests gracefully.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*