---
sidebar_position: 2
---

# 🛡️ Guards (Authorization)

Guards have a single responsibility: they determine whether a given request will be handled by the route handler or not, depending on certain conditions (like permissions, roles, ACLs, etc.) present at run-time.

In Ferrox, Guards are evaluated right after Middlewares but *before* Pipes and Controllers.

## High-Level Example

To protect a route so that only `Admin` users can access it, you simply inject the Role Extractor in your controller arguments.

```rust
use axum::{Json, routing::get, Router};
use ferrox_security::{AuthPayload, Role};

async fn delete_database(
    // The Guard! If the JWT doesn't contain the Admin role, this throws 403 Forbidden
    auth: AuthPayload, 
) -> Json<&'static str> {
    
    if auth.role != Role::Admin {
        // You can also handle it manually, but Ferrox Security Middlewares 
        // usually prevent the request from even reaching this point.
        return Json("Nice try.");
    }
    
    Json("Database Deleted!")
}
```

## Low-Level Internal Details

Under the hood, a Guard in Ferrox is simply an implementation of the `FromRequestParts` trait in Axum.

Unlike `FromRequest` (which consumes the request body, like a JSON Pipe), `FromRequestParts` only looks at the HTTP Headers (like the `Authorization` header). This means multiple Guards can be chained sequentially.

If a Guard fails (e.g., the JWT signature is invalid), it returns a `ferrox_errors::AppError::Unauthorized`. Because of Ferrox's early-return architecture, the TCP connection is immediately answered with a 401/403 status code, saving database connections and memory.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*