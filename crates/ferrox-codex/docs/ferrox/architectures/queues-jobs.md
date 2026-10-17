---
sidebar_position: 5
---

# 📬 Background Jobs & Worker Queues

Long-running operations (PDF report compilation, video transcoding, external API syncs, batch emails) must never run inside HTTP request handlers.

Ferrox provides `ferrox-jobs`, integrating [Apalis](https://github.com/geoffraey/apalis) and Redis to manage persistent background job queues with automatic worker scaling and retries.

---

## 1. Defining a `Job`

Define your job payload struct implementing `apalis::prelude::Job`:

```rust
use serde::{Deserialize, Serialize};
use apalis::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessVideoJob {
    pub video_id: String,
    pub storage_path: String,
}

impl Job for ProcessVideoJob {
    const NAME: &'static str = "ferrox::ProcessVideoJob";
}
```

---

## 2. Worker Processing Function

Write the asynchronous processing logic for the job:

```rust
use apalis::prelude::*;

pub async fn process_video_worker(job: ProcessVideoJob, _ctx: JobContext) -> Result<(), apalis::prelude::Error> {
    println!("🎬 Worker starting video transcoding for ID: {}", job.video_id);
    
    // Simulate long-running processing
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    println!("✅ Video processing complete!");
    Ok(())
}
```

---

## 3. Starting the Background Worker Engine

Launch worker pools during `main.rs` application startup:

```rust
use ferrox_jobs::start_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start Redis worker pool with 4 concurrent worker threads
    let redis_url = "redis://127.0.0.1:6379";
    start_worker(redis_url).await?;

    println!("⚡ Background Job Engine active!");
    Ok(())
}
```

---

## 4. ✅ Best Practices

- **Set up dead-letter queues**: Capture failed jobs after max retries so developers can inspect and replay problematic payloads.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*