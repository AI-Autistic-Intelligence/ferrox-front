---
id: ecosystem-showcase-guide
title: Enterprise Showcase & Polyglot Mesh Guide 2.0
sidebar_label: Ecosystem Showcase Guide
---

# Ferrox & Yalc Polyglot Enterprise Showcase Guide 2.0

The definitive master guide and architectural reference for the multi-language enterprise microservices ecosystem located in `C:\Users\nn\Desktop\code`.

---

## 1. Ecosystem Overview

The Ferrox and Yalc ecosystems span **6 core programming runtimes and 1 WebAssembly frontend**, unified by shared enterprise architectural standards (Zero-Trust PASETO v4, Shannon Entropy Threat Engine, CQRS & Distributed Sagas, OpenTelemetry):

1. **`ferrox`**: Ultra-high-performance Rust workspace (Axum, Utoipa OpenAPI, SeaORM, Limit Order Book engine).
2. **`ferrox-java`**: Enterprise Java 21 LTS microservice stack (Project Loom Virtual Threads, Spring Boot 3.3.0, Project Panama `MemorySegment` zero-GC off-heap cache, Singleflight concurrency controller).
3. **`ferrox-php`**: Resident-memory PHP 8.3 enterprise framework (FrankenPHP Worker Mode, multi-warehouse Distributed SAGA, PASETO v4).
4. **`ferrox-py`**: Asynchronous Python 3.11+ data platform (FastAPI, Polars high-throughput analytics, Shannon entropy WAF).
5. **`ferrox-node`**: High-speed modular Node.js 20+ framework (Fastify 5, `@ferrox-node/*` modules, AcidSagaCoordinator, ReliableQueueWorker).
6. **`nestjs-yalc`**: NestJS enterprise framework with progressive 3-tier didactic showcases (`skeleton` -> `omnikernel` -> `task`) and multi-tenant SaaS backend.
7. **`ferrox-front`**: High-performance WebAssembly Admin Dashboard built in Rust with Trunk, Signals reactivity, virtualized DataGrid (1M rows), and glassmorphism theming.

---

## 2. Polyglot Comparative Architectural Matrix

| Enterprise Pattern | Rust (`ferrox`) | Java 21 (`ferrox-java`) | PHP (`ferrox-php`) | Python (`ferrox-py`) | Node.js (`ferrox-node`) | NestJS (`nestjs-yalc`) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Concurrency Model** | Async Tokio Green Threads | Java 21 Loom Virtual Threads (`spring.threads.virtual.enabled`) | FrankenPHP Caddy Worker Mode Threads | Asyncio event loop + Polars Rust GIL-free parallelism | Node 20 libuv async event loop | Node 20 async/await + RxJS Observables |
| **CQRS Bus** | `ferrox-core::cqrs` strongly-typed Command/Query bus | `CommandBus` & `QueryBus` with Virtual Thread dispatch & `Result<T,E>` | `Application\CommandBus` and `QueryBus` with attributes | `ferrox_py.pipelines.dag` DAG command execution | `@ferrox-node/cqrs` `CommandBus` with transactional middleware | `@nest-yalc-2/crud-gen` & TypeORM query/command splits |
| **Distributed Sagas** | `ferrox-saga` coordinator with compensating rollbacks | Async Virtual Thread Saga steps with exponential retry backoff | `Domain\Sagas\OrderFulfillmentSaga` (Inventory, Payment, Warehouse) | `DAG` orchestrator with node dependencies and rollback hooks | `@ferrox-node/cqrs` `AcidSagaCoordinator` with forward/compensate steps | `TaskSyncStateProjection` with revision-based optimistic locks |
| **Authentication** | PASETO v4 Local (`XChaCha20-Poly1305`) + TOTP 2FA | PASETO v4 (`jpaseto` + BouncyCastle) + 2FA TOTP | `Http\PasetoGuard` PASETO v4 symmetric encryption | `ferrox_py.security.paseto` PASETO v4 Local token validation | `@ferrox-node/security` `PasetoAuthService` + `TotpService` | `@nest-yalc-2/auth` PASETO v4 validation + Nest Guards |
| **Threat Engine (WAF)**| `ferrox-security::sentinel` SIMD regex & Shannon entropy check | `SentinelThreatFilter` Shannon Entropy $H(X) > 4.8$ + SQLi/XSS/Traversal | `Http\SentinelThreatEngine` O(N) linear scanning (SQLi, XSS) | `SentinelThreatEngineMiddleware` Shannon entropy $H(X) > 4.8$ | `@ferrox-node/security` `SentinelThreatEngine` middleware | `@nest-yalc-2/security` rate limiting & threat filters |
| **High-Traffic Cache** | Redis 7 + Zero-copy in-memory buffers | Project Panama `MemorySegment` Native Off-Heap Zero-GC LRU | APCu worker cache + Redis 7 | Redis 7 + In-memory Arrow tables | Redis 7 + In-memory LRU | Redis 7 cache manager |
| **Concurrency Guard** | Tokio broadcast / watch deduplication | `SingleflightController` eliminating cache stampedes | Redlock distributed locks in Redis | Redis distributed lease lock | Redis token bucket & locking | Optimistic entity version locking |
| **Observability** | `tracing` with OTLP exporter + `/metrics` | Structured JSON Logging (W3C Trace IDs) + `/actuator/prometheus` | `Infrastructure\Metrics\PrometheusExporter` (APCu / Redis) | `StructuredJsonLoggingMiddleware` (W3C TraceContext) + `/metrics` | `@ferrox-node/observability` OpenTelemetry SDK + Prometheus | `@nest-yalc-2/observability` OTel collector + AppLoggerFactory |
| **Health Probes** | `/health/live`, `/health/ready` | `/actuator/health/liveness`, `/actuator/health/readiness` | `/health/live`, `/health/ready` | `/health/live`, `/health/ready` | `/health/live`, `/health/ready` | `/health/live`, `/health/ready` |
| **Persistence** | SeaORM (PostgreSQL 16) | Spring Data / JPA (PostgreSQL 16) | PDO UnitOfWork + PostgreSQL 16 | SQLAlchemy (PostgreSQL 16) + InMemory | Knex / TypeORM + PostgreSQL 16 | TypeORM + OmniKernel document persistence |

---

## 3. Central API Gateway Routing & Master Ports

Under `ferrox-infrastructure/docker-compose.mesh.yml`, a centralized **Traefik v3 API Gateway** unifies ingress traffic onto port `:80` (and `:8080`), automatically stripping routing prefixes and forwarding requests to the respective runtime microservices:

| Gateway Route Prefix | Target Microservice | Native Port | Health Probe | Interactive API Documentation |
| :--- | :--- | :---: | :--- | :--- |
| **`/api/v1/rust/*`** | `ferrox-saas-backend` | `8080` | `/health/live` | `http://localhost:80/api/v1/rust/swagger-ui` |
| **`/api/v1/java/*`** | `ferrox-java-showcase` | `8082` | `/actuator/health/liveness` | `http://localhost:8082/swagger-ui.html` |
| **`/api/v1/php/*`** | `ferrox-php-eshop` | `8000` | `/health/live` | `http://localhost:80/api/v1/php/docs` |
| **`/api/v1/py/*`** | `ferrox-py-platform` | `8001` | `/health/live` | `http://localhost:80/api/v1/py/docs` |
| **`/api/v1/node/*`** | `ferrox-node-showcase` | `3000` | `/health/live` | `http://localhost:80/api/v1/node/docs` |
| **`/api/v1/nest/*`** | `nestjs-yalc-task` | `3002` | `/health/live` | `http://localhost:80/api/v1/nest/docs` |
| **`/*` (Fallback)** | `ferrox-front-dashboard`| `4000` | `/` | WebAssembly Single Page Application |

### Observability & Infrastructure Ports
- **Central API Gateway**: `http://localhost:80`
- **Traefik Web Dashboard**: `http://localhost:8088`
- **Prometheus Scrape Server**: `http://localhost:9090`
- **Grafana Pre-Configured Dashboard**: `http://localhost:3001` (`admin` / `admin`)
- **OpenTelemetry Collector**: `http://localhost:4318` (HTTP OTLP) / `localhost:4317` (gRPC OTLP)
- **PostgreSQL 16**: `localhost:5432` (`ferrox` / `secret_ferrox_db`)
- **Redis 7**: `localhost:6379`

---

## 4. Step-by-Step Operations Guide

### 4.1 Launching the Unified Mesh Orchestration
```powershell
# From workspace root:
cd ferrox-infrastructure
./mesh-up.ps1

# To stop:
./mesh-down.ps1
```

### 4.2 Verifying Endpoints with cURL

#### 1. Java 21 Project Loom & Panama Off-Heap Cache
```bash
# Health probes
curl -i http://localhost:80/api/v1/java/actuator/health/liveness

# Issue PASETO Token
curl -X POST http://localhost:80/api/v1/java/api/v1/auth/token \
  -H "Content-Type: application/json" \
  -d '{"subject": "admin@ferrox.dev", "role": "ADMIN"}'

# Store bid in Project Panama MemorySegment (Native Off-Heap RAM)
curl -X POST http://localhost:80/api/v1/java/api/v1/auctions/antique-vase/bid \
  -H "Content-Type: application/json" \
  -d '{"amount": 150000.00}'

# Singleflight deduplicated quote
curl http://localhost:80/api/v1/java/api/v1/market/quote/BTC
```

#### 2. Rust High-Frequency Limit Order Book
```bash
curl -X POST http://localhost:80/api/v1/rust/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"id": "ord-01", "side": "BUY", "price": 68500.0, "amount": 1.25}'
```

#### 3. PHP FrankenPHP Distributed Saga Order
```bash
curl -X POST http://localhost:80/api/v1/php/api/orders \
  -H "Content-Type: application/json" \
  -d '{"customer_id": "cust-99", "amount": 420.50}'
```

#### 4. Sentinel Threat Engine (Shannon Entropy Interception)
```bash
# Clean request passes with HTTP 200
curl -X POST http://localhost:80/api/v1/java/api/v1/secure/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Valid business payload"}'

# Malicious SQLi in query string blocked with HTTP 403 RFC 7807
curl -i -X POST "http://localhost:80/api/v1/java/api/v1/secure/echo?queryParam=SELECT%20*%20FROM%20users" \
  -H "Content-Type: application/json" \
  -d '{"message": "innocent"}'
```

---

## 5. Automated Security Hardening & Penetration Testing

The security posture of the ecosystem is verified via automated suites under `ferrox-pentest-private` conforming to **OWASP WSTG v4.2 / v5.0**:

```powershell
cd ferrox-pentest-private
./run_wstg_audit.ps1
```

### Key Verification Domains
1. **Sentinel Bypass Fuzzing**: Validates that obfuscated shellcode ($H(X) > 4.8$) and multi-layer encoded SQLi/traversals are intercepted with HTTP 403.
2. **PASETO Cryptographic Validation**: Validates rejection of weak/null symmetric keys, unauthenticated footer tampering, and expired replay tokens.
3. **Saga Concurrency & Locking**: Tests 20 concurrent threads against limited stock to guarantee zero overselling and singleflight deduplication.
4. **Rate Limiting & DoS**: Verifies token-bucket sliding window throttling (HTTP 429) and circuit breaker transitions to `OPEN` state.

Full results and remediation details are documented in [`ferrox-docs/docs/SECURITY_AUDIT_REPORT.md`](file:///C:/Users/nn/Desktop/code/ferrox-docs/docs/SECURITY_AUDIT_REPORT.md).

---

## 6. Inspecting Real-Time Telemetry on Grafana

1. Open **Grafana**: `http://localhost:3001` (Credentials: `admin` / `admin`).
2. Navigate to **Dashboards -> Ferrox -> Ferrox Polyglot Ecosystem Overview**.
3. Telemetry Panes Available:
   - **Polyglot Ingestion Throughput**: Real-time req/sec across all 6 runtimes.
   - **Sub-Millisecond & Virtual Thread Latency**: Live p99 comparisons between Rust (0.04ms) and Java Loom (1.1ms).
   - **Loom Virtual Threads Gauge**: Active virtual threads count (`jvm_threads_live_threads`).
   - **Panama Off-Heap Native Allocation**: Zero-GC memory segment allocations.
   - **Sentinel Intercepted Threat Attacks**: Real-time counter of blocked HTTP 403 threats grouped by attack vector.
