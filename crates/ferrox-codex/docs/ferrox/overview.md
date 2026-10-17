---
id: overview
title: Overview
sidebar_position: 1
---

# Ferrox Core Overview

**Ferrox** is a high-performance, event-driven backend framework built in Rust. It is designed for maximum throughput, memory safety, and seamless integration with event streaming platforms like Kafka.

## Architecture

Ferrox is built around a robust asynchronous core using `tokio`. It provides a structured way to build microservices that can handle massive amounts of concurrent connections with minimal latency.

### Key Components

- **Event Manager**: A core abstraction over Kafka, allowing for declarative event publishing and subscription.
- **Database Layer**: A highly optimized wrapper around `sqlx` providing connection pooling and fast serialization.
- **CRUD Generator**: Macro-driven code generation that instantly creates high-performance REST and gRPC endpoints for your database entities.
- **Node Bridge (`ferrox-node`)**: Native bindings to allow Node.js applications (like NestJS) to call into Ferrox's Rust core seamlessly using N-API.

## Performance Profile

Because it is built in Rust, Ferrox does not suffer from Garbage Collection pauses and provides predictable tail latencies. It is heavily optimized for:

- **High-throughput event streaming** (consuming and producing to Kafka).
- **CPU-bound operations** (data transformation, complex mathematical modeling).
- **Memory-constrained environments** (containers with tight limits).

## Getting Started

A basic Ferrox service initializes the event manager and database connection pool:

```rust
use ferrox_core::app::FerroxApp;
use ferrox_database::DatabaseConfig;
use ferrox_event_manager::KafkaConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = FerroxApp::builder()
        .with_database(DatabaseConfig::from_env()?)
        .with_kafka(KafkaConfig::from_env()?)
        .build()
        .await?;
        
    app.run().await?;
    
    Ok(())
}
```

Explore the documentation to see how to define entities, generate CRUD routes, and handle distributed events.
