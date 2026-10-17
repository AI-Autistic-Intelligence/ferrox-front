---
sidebar_position: 4
---

# 🔴 In-Memory Caching & Redis

Redis is the standard in-memory storage engine for high-speed session management, caching, distributed locks, rate limiting, and real-time pub/sub.

Ferrox provides `ferrox-database-redis`, integrating the asynchronous `redis` crate with Tokio.

---

## 1. Connecting to Redis

Initialize the Redis client:

```rust
use redis::{AsyncCommands, Client};
use ferrox_errors::AppError;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Self { client })
    }

    pub async fn set_str(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<(), AppError> {
        let mut con = self.client.get_async_connection().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let _: () = con.set_ex(key, value, ttl_seconds).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_str(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut con = self.client.get_async_connection().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let value: Option<String> = con.get(key).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(value)
    }
}
```

---

## 2. JSON Object Caching

Store complex Rust structs in Redis using `serde_json`:

```rust
use serde::{Deserialize, Serialize};

impl RedisCache {
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: u64) -> Result<(), AppError> {
        let json_str = serde_json::to_string(value)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        self.set_str(key, &json_str, ttl_seconds).await
    }

    pub async fn get_json<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Result<Option<T>, AppError> {
        if let Some(json_str) = self.get_str(key).await? {
            let obj = serde_json::from_str(&json_str)
                .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
            Ok(Some(obj))
        } else {
            Ok(None)
        }
    }
}
```

---

## 3. Real-Time Pub/Sub Messaging

Publish events across distributed microservice instances using Redis channels:

```rust
pub async fn publish_event(&self, channel: &str, message: &str) -> Result<(), AppError> {
    let mut con = self.client.get_async_connection().await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let _: () = con.publish(channel, message).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
```

---

## 4. ✅ Best Practices

- **Always set TTLs on cache entries**: Never store keys indefinitely without an expiration policy to avoid exhausting Redis RAM.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*