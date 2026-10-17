---
sidebar_position: 4
---

# 🔄 Interceptors: Request Execution Pipeline

An **Interceptor** in Ferrox is a middle layer that wraps handler execution, inspired by NestJS Interceptors and Aspect-Oriented Programming (AOP). Interceptors allow you to inspect, transform, or log incoming HTTP requests and outgoing HTTP responses.

The `ferrox-interceptors` crate provides zero-cost middleware hooks for Axum routes.

---

## 1. Responsibilities of an Interceptor

Interceptors excel at:
- **Execution Time Profiling**: Measuring microsecond request processing latency.
- **Response Transformation**: Wrapping response bodies in standard JSON formats.
- **Cache Interception**: Returning cached responses before executing route logic (`CacheInterceptor`).
- **Audit Logging**: Capturing IP addresses, request URIs, and HTTP status codes.

```
Incoming Request ---> [ Interceptor Pre-hook ] ---> Route Handler
                                                        |
                                                        v
Outgoing Response <--- [ Interceptor Post-hook ] <--- Result
```

---

## 2. Built-in `logging_interceptor`

Ferrox includes an out-of-the-box execution time logging interceptor.

### Usage in Routers

```rust
use axum::{routing::get, middleware, Router};
use ferrox_interceptors::logging_interceptor;

pub fn app_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .layer(middleware::from_fn(logging_interceptor))
}
```

When requests hit `/users`, the interceptor automatically outputs structured log lines:

```text
INFO [200] GET /users - 1.24ms
```

---

## 3. Caching Interceptor (`CacheInterceptor`)

For expensive read queries, `CacheInterceptor` intercepts incoming requests, checks an in-memory or Redis key, and returns the response immediately if cached.

```rust
use ferrox_interceptors::cache::CacheInterceptor;

// Create cache interceptor with 60-second TTL
let cache_layer = CacheInterceptor::new(60);

let app = Router::new()
    .route("/analytics/monthly", get(get_monthly_analytics))
    .layer(cache_layer);
```

---

## 4. Writing a Custom Interceptor

To write a custom interceptor, implement an async function taking `Request<Body>` and `axum::middleware::Next`:

```rust
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn custom_header_interceptor(
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    
    // Execute down-stream handlers
    let mut response = next.run(req).await;
    
    // Inject server timing header into outgoing response
    let elapsed = format!("{}ms", start.elapsed().as_millis());
    response.headers_mut().insert(
        "X-Response-Time", 
        elapsed.parse().unwrap()
    );
    
    response
}
```

---

## 5. ✅ Best Practices

- **Use Interceptors for Cross-Cutting Concerns**: Keep controllers clean by moving timing, tracing headers, and compression into interceptors.
- **Order Layers Correctly**: Middleware layers evaluate in reverse order of addition in Axum. Place authentication guards before caching interceptors to prevent caching unauthorized responses.

---

## 6. ❌ Anti-Patterns

- **❌ Mutating Request Bodies synchronously**: Never consume or buffer request bodies synchronously inside an interceptor, as this loads entire request payloads into memory and blocks Tokio worker threads.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*