---
id: logger
title: "@node-yalc/logger"
sidebar_position: 3
---

# 📝 Structured Pino Logger (`@node-yalc/logger`)

## 💡 1. What It Is & Architectural Purpose
`@node-yalc/logger` is the high-performance structured logging package for the entire ecosystem. Built on top of **Pino**, it was engineered to provide JSON-structured logging with minimal CPU overhead, **automatic sensitive data redaction** (GDPR/PCI-DSS compliance), and complete decoupling from web framework lifecycles.

---

## ⚙️ 2. What It Does & Key Features
- **JSON Structured Logging**: Native JSON output for ingestion into Datadog, ELK, Grafana Loki, or AWS CloudWatch.
- **Automatic Sensitive Data Redaction**: Automatically masks passwords, authorization headers, credit cards, and secrets.
- **Trace Context Correlation**: Propagates `X-Request-Id` correlation IDs across asynchronous execution contexts.
- **Zero Overhead**: Up to 5x faster than traditional Node.js loggers (`winston`, `bunyan`).

---

## 🔬 3. How It Works Under the Hood

```mermaid
flowchart LR
    LogCall["logger.info(msg, payload)"]
    RedactEngine["SonicBoom Redaction Filter"]
    JsonStream["Async JSON Stream"]
    Stdout["process.stdout / CloudWatch"]

    LogCall --> RedactEngine
    RedactEngine --> JsonStream
    JsonStream --> Stdout
```

1. **Async Logging via SonicBoom**: Logs are serialized and written non-blockingly using high-speed memory buffers.
2. **In-place Redaction**: Pino directly overwrites keys matching redaction patterns prior to stringifying JSON to stdout.

---

## 🧠 4. Why It Was Designed This Way (Rationale vs Winston)

| Metric | 📝 `@node-yalc/logger` (Pino) | 🐢 Winston / Bunyan |
|---|---|---|
| **Serialization Speed** | **Up to 30,000 logs/sec** | ~6,000 logs/sec |
| **Event Loop Blocking** | **Minimal (Non-Blocking SonicBoom)** | High (Synchronous Stringify on Event Loop) |
| **GDPR/PCI-DSS Redaction** | **Native Serialization-Level Masking** | Custom Slow Formatters |

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

```typescript
import { createYalcLogger } from '@node-yalc/logger';

// 1. Initialize Logger with Custom Redaction Rules
const logger = createYalcLogger({
  level: 'debug',
  redact: ['password', 'creditCard.number', 'authorization', 'apiKey']
});

// 2. Log with Contextual Metadata
logger.info('User authenticated successfully', {
  userId: 'usr_99120',
  ip: '192.168.1.50',
  authorization: 'Bearer eyJhbGciOi...' // Automatically masked to "[REDACTED]"
});

// Output JSON on stdout:
// {"level":30,"time":1695062400000,"msg":"User authenticated successfully","userId":"usr_99120","ip":"192.168.1.50","authorization":"[REDACTED]"}
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

1. ❌ **DO NOT use `console.log()` in production**: `console.log` is a synchronous blocking I/O operation that stalls the Node.js event loop and exposes un-redacted sensitive data.
2. ❌ **DO NOT log raw request or database objects directly**: Passing circular objects or active connection instances can cause memory exhaustion during JSON serialization.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Local Development Formatting**: In local development, pipe JSON output into `pino-pretty` (`node app.js | npx pino-pretty`) for colorized logs without impacting production performance.


---

## 🔗 Cross-References

To see how this module integrates with the rest of the Ferrox architecture, refer to the following documentation:

- [Database & TypeORM](../../../nestjs-yalc/docs/databases/database.md)
- [Event Manager](../architectures/event-manager.md)
- [Observability & Logger](../../../nestjs-yalc/docs/observability/logger.md)
