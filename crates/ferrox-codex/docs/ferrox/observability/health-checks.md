---
sidebar_position: 2
---

# 🩺 Kubernetes Health Probes (Liveness & Readiness)

Container orchestrators like Kubernetes require standardized HTTP endpoints to monitor application health and manage pod lifecycles:
- **Liveness Probe (`/healthz`)**: Indicates whether the application process is running. If it fails, Kubernetes restarts the container.
- **Readiness Probe (`/readyz`)**: Indicates whether the application is ready to handle incoming traffic (e.g. database pools connected, caches warmed). If it fails, Kubernetes removes the pod from load balancer routing.

`ferrox-health` provides built-in HTTP health check routers.

---

## 1. Attaching `health_router`

Attach `health_router()` to your main HTTP router:

```rust
use axum::Router;
use ferrox_health::health_router;

pub fn app_router() -> Router {
    Router::new()
        .merge(health_router()) // Adds /healthz and /readyz
}
```

---

## 2. Health Endpoint Responses

### Liveness Probe (`GET /healthz`)
```bash
curl http://localhost:3000/healthz
```

Response (`200 OK`):
```json
{
  "status": "UP",
  "timestamp": 1788602400
}
```

### Readiness Probe (`GET /readyz`)
```bash
curl http://localhost:3000/readyz
```

Response (`200 OK`):
```json
{
  "status": "READY",
  "timestamp": 1788602400
}
```

---

## 3. Kubernetes Deployment YAML Configuration

Configure your Kubernetes deployment manifest:

```yaml
livenessProbe:
  httpGet:
    path: /healthz
    port: 3000
  initialDelaySeconds: 5
  periodSeconds: 10
readinessProbe:
  httpGet:
    path: /readyz
    port: 3000
  initialDelaySeconds: 10
  periodSeconds: 5
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*