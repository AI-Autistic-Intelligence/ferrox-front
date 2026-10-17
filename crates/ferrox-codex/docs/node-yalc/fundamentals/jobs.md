---
id: jobs
title: "@node-yalc/jobs"
sidebar_position: 8
---

# ⏱️ Background Jobs & SSE (`@node-yalc/jobs`)

## 💡 1. What It Is & Architectural Purpose
The `@node-yalc/jobs` module provides a suite of lightweight, in-memory task management tools. It offers three distinct architectural capabilities:
1. **Asynchronous Background Queues** for deferring heavy tasks without blocking the HTTP response.
2. **Cron Schedulers** for recurrent, interval-based polling.
3. **Server-Sent Events (SSE)** helpers to stream the real-time status of these background jobs back to a frontend client.

---

## ⚙️ 2. Comprehensive Taxonomy & Key Features

| Component | Description | Use Case |
| :--- | :--- | :--- |
| **`FerroxJobQueue`** | In-memory asynchronous queue manager. | Processing PDF generation, email sending, or data aggregations immediately after responding 200 OK to the client. |
| **`FerroxCronScheduler`** | In-memory recurrent timer registry. | Fetching exchange rates every 5 minutes or cleaning up temporary database tables nightly. |
| **`FerroxSseStream`** | HTTP Server-Sent Event abstraction. | Opening an SSE connection so the frontend can receive live updates on job progress (e.g. `PROCESSING` -> `COMPLETED`). |

---

## 🔬 3. How It Works Under the Hood

### Event Loop Delegation
When a job is pushed to the `FerroxJobQueue`, it leverages Node's native `setImmediate()` to defer execution. 

```mermaid
flowchart TD
    Client[HTTP Request]
    Router[Controller / API]
    Queue[(FerroxJobQueue)]
    Worker((Background Worker))
    Response[200 OK]

    Client --> Router
    Router -->|enqueue('SEND_EMAIL', data)| Queue
    Router --> Response
    
    Queue -.->|setImmediate| Worker
    Worker -->|Execute| Email[SendGrid API]
```
This guarantees the HTTP response is dispatched to the client instantly, while the V8 engine picks up the job on the next tick of the Event Loop.

---

## 🧠 4. Why It Was Designed This Way (Rationale)

Not every microservice needs the immense infrastructure overhead of a Redis cluster and BullMQ just to send a "Welcome Email" in the background. The `@node-yalc/jobs` module provides a powerful, zero-dependency alternative for monolithic or small-scale Node.js deployments. 

If scaling requirements eventually outgrow a single server instance, the strict interface contracts (`name`, `payload`, `status`) make it trivial to refactor the `enqueue()` method to push messages to RabbitMQ instead of the local array.

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

### 1. Job Queue Setup and Execution

```typescript
import { FerroxJobQueue } from '@node-yalc/jobs';

const queue = new FerroxJobQueue();

// 1. Register a background worker
queue.registerWorker('process-video', async (payload) => {
  console.log(`Processing video ID: ${payload.videoId}...`);
  await new Promise(resolve => setTimeout(resolve, 5000));
  console.log('Video finished.');
});

// 2. Enqueue the job from an API endpoint
export function uploadVideoHandler(req, res) {
  const job = queue.enqueue('process-video', { videoId: 'vid_999' });
  
  // Return immediately. The job executes asynchronously.
  res.status(202).json({ message: 'Accepted', jobId: job.id });
}
```

### 2. Streaming Progress via SSE
To give the frontend real-time updates on that background job:

```typescript
import { FerroxSseStream } from '@node-yalc/jobs';

export function sseStatusEndpoint(req, res) {
  // 1. Establish an open connection
  FerroxSseStream.initSseResponse(res);
  
  // 2. Stream events (In a real app, this would be tied to an EventEmitter)
  FerroxSseStream.sendEvent(res, 'status', { jobId: 'vid_999', status: 'PROCESSING' });
  
  setTimeout(() => {
    FerroxSseStream.sendEvent(res, 'status', { jobId: 'vid_999', status: 'COMPLETED' });
    res.end(); // Close connection when done
  }, 5000);
}
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

> [!CAUTION]
> **Anti-Pattern 1: In-Memory Queues in Multi-Instance Deployments**
> If you deploy your API across 5 load-balanced Kubernetes pods, jobs enqueued in memory are **only** processed by the specific pod that received the HTTP request. Do not use `FerroxJobQueue` for stateful or long-running tasks in horizontally scaled environments. Upgrade to an external broker (Redis/SQS).

> [!WARNING]
> **Anti-Pattern 2: Missing Memory Bounds**
> The `FerroxJobQueue` retains jobs in a local array. If you enqueue 1,000,000 jobs per minute but the worker only processes 10, the array will grow infinitely until the Node process crashes with an Out Of Memory (OOM) error.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Pro-Tip: Graceful Shutdowns**
> When your Node server receives a `SIGTERM`, ensure you block the process exit until the `queue` array is entirely empty to prevent terminating jobs mid-execution.
