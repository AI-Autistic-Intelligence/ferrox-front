---
sidebar_position: 4
---

# 🕵️ OpenTelemetry & Distributed Tracing

In a distributed microservice network, a single user HTTP request might pass through an API Gateway, an Authentication service, an Inventory service, and a Payments database.

**Distributed Tracing** tracks the lifecycle of a request across all service boundaries by injecting and propagating unique trace identifiers (`traceparent` and `X-Correlation-ID` headers).

`ferrox-tracing` integrates OpenTelemetry and Jaeger/Zipkin exporters.

---

## 1. Request Correlation ID Middleware

`ferrox-tracing` provides middleware that automatically extracts or generates a unique correlation ID for every incoming request:

```rust
use axum::{
    body::Body,
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn correlation_id_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let correlation_id = req.headers()
        .get("X-Correlation-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Re-inject correlation header into request
    req.headers_mut().insert(
        "X-Correlation-ID",
        HeaderValue::from_str(&correlation_id).unwrap(),
    );

    let mut response = next.run(req).await;

    // Attach correlation ID to response headers
    response.headers_mut().insert(
        "X-Correlation-ID",
        HeaderValue::from_str(&correlation_id).unwrap(),
    );

    response
}
```

---

## 2. Propagating Correlation IDs across Microservice Calls

When calling downstream microservices via `reqwest`, attach the `X-Correlation-ID` header:

```rust
pub async fn call_downstream_service(correlation_id: &str) {
    let client = reqwest::Client::new();
    let res = client.get("http://payments-service/charge")
        .header("X-Correlation-ID", correlation_id)
        .send()
        .await;
}
```

---

## 3. Visualizing Traces in Jaeger / Grafana Tempo

Distributed traces are collected by OpenTelemetry collectors and rendered in Jaeger UI, visualizing call stacks, duration waterfalls, and exact bottlenecks.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*