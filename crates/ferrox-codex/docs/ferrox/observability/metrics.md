---
sidebar_position: 3
---

# 📈 Prometheus Metrics Export

Monitoring real-time application throughput, request latencies, and error rates requires exporting metrics to monitoring platforms like Prometheus and Grafana.

`ferrox-metrics` initializes a Prometheus HTTP exporter running on a dedicated telemetry listener port.

---

## 1. Initializing Prometheus Exporter

Call `setup_metrics` during application bootstrap:

```rust
use ferrox_metrics::{setup_metrics, record_http_request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starts Prometheus HTTP listener on port 9000
    setup_metrics()?;

    println!("📊 Prometheus metrics available at http://0.0.0.0:9000/metrics");
    Ok(())
}
```

---

## 2. Pre-Registered Metric Counters

`ferrox-metrics` pre-registers standard counters:

| Metric Name | Type | Description |
|---|---|---|
| `http_requests_total` | Counter | Total number of HTTP requests processed |
| `db_queries_total` | Counter | Total number of database queries executed |
| `events_published_total` | Counter | Total number of domain events published |

---

## 3. Incrementing Counters in Controllers

Increment counters using helper functions or standard `metrics` macros:

```rust
use ferrox_metrics::record_http_request;
use metrics::counter;

pub async fn my_controller_handler() {
    record_http_request();
    counter!("orders_created_total").increment(1);
}
```

---

## 4. Scraping `/metrics` Output

```bash
curl http://localhost:9000/metrics
```

Output:
```text
# HELP http_requests_total Total number of HTTP requests processed
# TYPE http_requests_total counter
http_requests_total 15420
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*