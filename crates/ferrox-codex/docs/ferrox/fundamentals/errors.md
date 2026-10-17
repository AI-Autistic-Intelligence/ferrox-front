---
sidebar_position: 5
---

# 🚨 Centralized Error Handling & Exception Filters

Robust applications require predictable, strongly typed error handling. In Ferrox, `ferrox-errors` provides a centralized `AppError` enum that implements Axum's `IntoResponse` trait.

This guarantees that unhandled exceptions, database errors, validation failures, and authorization checks automatically map to structured JSON responses with correct HTTP status codes.

---

## 1. The `AppError` Enum

The core error type is `AppError`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal Server Error")]
    InternalServerError(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("Database Error: {0}")]
    DatabaseError(String),
}
```

---

## 2. Standardized JSON Error Payload Format

When an `AppError` is returned from a controller handler, Ferrox converts it into a standardized JSON response:

```json
{
  "status": 404,
  "message": "Not Found: User with ID 42 does not exist"
}
```

### HTTP Status Code Mappings

| `AppError` Variant | HTTP Status Code | Description |
|---|---|---|
| `AppError::NotFound(msg)` | `404 Not Found` | Requested entity or resource missing |
| `AppError::ValidationError(msg)` | `400 Bad Request` | Malformed JSON or DTO validation failure |
| `AppError::Unauthorized(msg)` | `401 Unauthorized` | Missing or invalid PASETO JWT token |
| `AppError::DatabaseError(msg)` | `500 Internal Server Error` | Database query or connection pool error |
| `AppError::InternalServerError(err)` | `500 Internal Server Error` | Unexpected server panic or system error |

---

## 3. Returning Errors from Controllers

Controllers return `Result<T, AppError>` and use Rust's `?` operator for clean error propagation:

```rust
use axum::{extract::{Path, State}, Json};
use ferrox_errors::AppError;

pub async fn get_user_by_id(
    Path(id): Path<u64>,
    State(service): State<UserService>,
) -> Result<Json<UserDto>, AppError> {
    let user = service.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFound(format!("User {}", id)))?;

    Ok(Json(user.into()))
}
```

---

## 4. Security & Error Sanitization

> [!IMPORTANT]
> To prevent **Information Disclosure** vulnerabilities, `AppError::InternalServerError` and `AppError::DatabaseError` print full diagnostic stack traces to server console logs, but only return generic `"Internal Server Error"` messages to HTTP clients.

---

## 5. ✅ Best Practices

- **Never `unwrap()` inside controllers or services**: Always propagate errors with `?` or map custom errors into `AppError`.
- **Use domain-specific error messages**: Provide human-readable details for `AppError::ValidationError` so frontend clients can render form field hints.

---

## 6. ❌ Anti-Patterns

- **❌ Exposing database connection strings or raw SQL tracebacks**: Raw database errors contain schema details that attackers exploit. Use `AppError::DatabaseError` to sanitize responses.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*