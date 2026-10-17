---
id: example-projects
title: Official Example Projects
sidebar_label: Example Projects
sidebar_position: 4
---

# Official Example Projects & Showcases

As a senior engineer designing a multi-framework ecosystem, I prioritized proving that Ferrox isn't just a theoretical abstraction. The framework must guarantee enterprise-grade security, precision, high-availability, and scalability under extreme loads. To validate these guarantees across Python, Java, PHP, and Node.js, I engineered specific reference applications.

These applications are not trivial "Hello World" examples. They are heavily fortified, domain-specific implementations that expose the full capability of the 7-Layer Onion Request Pipeline, zero-trust security paradigms, ACID transactions, and reactive streaming.

---

## 1. Ferrox-Py: Quantitative Data Platform & HFT Ingestor
**Location:** `ferrox-py/examples/data_platform`

### The "What" and "Why"
In the quantitative finance and High-Frequency Trading (HFT) domain, dropped messages, latency spikes, or security breaches can result in massive financial loss. The Ferrox-Py framework was validated by building a **Quantitative Data Platform**. Its purpose is to ingest high-volume, real-time WebSocket tick data from exchanges (e.g., Binance) and persist it with absolute ACID compliance for downstream machine learning and backtesting.

### How It Works (Techniques & Resources)
- **Engine & Async I/O**: Runs on FastAPI/Starlette under the hood for maximum Python async performance, serving as the outer routing layer.
- **Persistence Layer (SQLAlchemy & PostgreSQL)**: We migrated away from volatile Redis streams to strict PostgreSQL asynchronous sessions (`AsyncSessionLocal`). Models like `CryptoTrade` and `CryptoDepth` handle thousands of inserts per second while maintaining ACID guarantees.
- **Background Workers**: Utilizes Starlette lifecycle events (`@asynccontextmanager`) to spin up background daemon tasks (e.g., `binance_ingestor.py`) that maintain resilient WebSockets connections with exponential backoff.
- **Security ([Sentinel](/docs/ferrox/security/ferrox-sentinel) WAF)**: Employs the `SentinelThreatEngineMiddleware`. Every incoming request to the GraphQL/REST endpoints is scanned for SQL Injection, XSS, and Prompt Injections using Shannon entropy checks and heuristic signatures.
- **Authentication**: Secured via `PASETO v4` token validation, enforcing strict cryptographic claims.
- **Observability**: Implements Prometheus instrumentation (`prometheus-client`) out of the box to track ingestion latency, payload sizes, and threat detection events.

---

## 2. Ferrox-PHP: Enterprise E-Shop & [CQRS](/docs/ferrox/architectures/cqrs) Saga
**Location:** `ferrox-php/examples/eshop_app.php`

### The "What" and "Why"
PHP remains a titan in the e-commerce space. However, standard PHP applications often suffer from leaky abstractions and poor transaction boundaries when scaling. To prove Ferrox-PHP's solidity, I built a highly decoupled **Enterprise E-Shop Showcase**. The objective here is to demonstrate how to safely handle distributed transactions and eventual consistency without compromising the synchronous nature of PHP.

### How It Works (Techniques & Resources)
- **[CQRS](/docs/ferrox/architectures/cqrs) & Command Bus**: The application is strictly divided into Commands (e.g., `SubmitOrderCommand`) and Queries. Handlers are injected with domain-specific repositories.
- **ACID & UnitOfWork**: Database operations are wrapped inside a `UnitOfWork` pattern. Decrementing stock and creating orders happen in an atomic boundary.
- **The Saga Pattern & Fallbacks**: If an order cannot be fulfilled due to missing stock across multiple distributed warehouses, the Saga orchestrator automatically triggers a compensatory refund transaction (`OrderRefundedEvent`).
- **Domain Events & Outbox**: An `EventDispatcher` asynchronously handles side-effects (e.g., triggering the Mailer to notify admins of stock anomalies) without blocking the main HTTP response.
- **Data [Validation](/docs/ferrox/abstractions/validation)**: PHP 8 Attributes (`#[ValidatedDto]`) enforce strict type invariants before the Command even reaches the handler.
- **Native Metrics**: Fully integrated `PrometheusRegistry` to export real-time business metrics (e.g., `ferrox_warehouse_stock`, `ferrox_orders_total`).

---

## 3. Ferrox-Java: Hexagonal Banking Showcase
**Location:** `ferrox-java/ferrox-java-showcase`

### The "What" and "Why"
Java is the undisputed king of core banking and large-scale enterprise microservices. The JVM requires strict memory management to avoid GC pauses under extreme load. Therefore, the Java showcase represents a **Banking / Fintech Microservice** that focuses heavily on domain-driven design (DDD), memory efficiency, and bulletproof security.

### How It Works (Techniques & Resources)
- **Hexagonal Architecture (Ports & Adapters)**: The core domain has zero dependencies on the web framework or database. Inputs (Web Controllers) and Outputs (Repositories/Message Brokers) implement strict interfaces.
- **Spring Boot & Project Reactor**: Uses WebFlux for non-blocking, reactive data streaming.
- **Off-heap Memory Management**: Integrates `ferrox-java-offheap` to manage byte buffers directly in native memory, bypassing the JVM Garbage Collector to ensure deterministic latency for high-throughput trading data.
- **Security & PASETO**: Replaces legacy JWTs with `PasetoTokenService` (`Pasetos.parserBuilder()`), providing secure, tamper-proof, time-bound session claims validated on every request.
- **Event Sourcing**: Bank ledger transactions are stored as an immutable sequence of events rather than mutable state, ensuring 100% auditability.

---

## 4. Ferrox-Node: Standalone Fastify Microservice
**Location:** `ferrox-node/src/dummy-app.ts`

### The "What" and "Why"
Node.js thrives in microservice and BFF (Backend-for-Frontend) layers. Developers need a lightweight, highly performant engine that doesn't compromise on dependency injection or lifecycle management. I designed the Node showcase to demonstrate how Ferrox wraps engines like Fastify while providing an Angular/NestJS-like developer experience without the associated bloat.

### How It Works (Techniques & Resources)
- **Engine Agnosticism**: The `FerroxApp` orchestrator boots up an underlying Fastify instance but abstracts the raw API away.
- **Custom DI Container**: `FerroxDIContainer` uses `reflect-metadata` to automatically resolve and inject services (`HelloService`, `FerroxConfigService`) into controllers.
- **Lifecycle Hooks**: Implements `OnAppStart` and `OnAppDestroy` interfaces to manage graceful degradation, configuration fallbacks, and database connection pooling.
- **Global Onion Middleware**: Showcases the 7-Layer pipeline by registering global wildcard middlewares that intercept all Fastify routes for logging, redaction (`fast-redact`), and rate-limiting before the Controller logic executes.

---

## Conclusion & Senior Engineering Philosophy

When I engineered the Ferrox ecosystem, my core premise was: **Security and Architecture must be solved at the framework boundary, not by the application developer.** 

By strictly adhering to PASETO tokens, providing native [Sentinel](/docs/ferrox/security/ferrox-sentinel) Threat Engine integrations (WAF), natively supporting Prometheus, and enforcing [CQRS](/docs/ferrox/architectures/cqrs)/Hexagonal boundaries across four entirely different languages, I ensured that deploying a Ferrox app—whether on Kubernetes, a bare-metal swarm, or serverless containers—is fundamentally secure by default. The example projects above aren't just tutorials; they are the architectural blueprints for enterprise survival.
