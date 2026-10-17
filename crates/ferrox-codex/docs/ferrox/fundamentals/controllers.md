---
sidebar_position: 1
---

# 🕹️ Controllers: The Masterpiece Guide

In a Ferrox application, a **Controller** is the ultimate boundary between the unpredictable outside world (HTTP TCP streams) and the strict, type-safe inner world of your business logic. 

Its sole responsibility is to **receive HTTP requests**, route them to the correct service, and **return HTTP responses**. While this sounds simple, an Enterprise application demands robust handling of streaming, multipart uploads, WebSockets, timeouts, and body size limits.

This guide explores the full arsenal available to a Ferrox Controller, explaining not just *how* to use it, but *why* and *how not to*.

---

## 1. The Core Concept

In Ferrox (which runs on Axum and Tokio), there is no `@Controller()` class decorator like in NestJS. Instead, controllers are composed of pure, asynchronous functions (`Handlers`) grouped inside a `Router`. 

When the HTTP server receives a TCP byte stream, it attempts to match the URL against a routing tree (Radix Tree). Once matched, it triggers your handler.

```rust
use axum::{routing::{get, post}, Router};

pub fn users_controller() -> Router<AppState> {
    // We group routes that share the same prefix
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user_by_id))
}
```

---

## 2. The Arsenal (Toolbox)

Ferrox provides a massive array of **Extractors** to pull data out of the raw HTTP request. Extractors are strictly typed: if the client sends invalid data, Ferrox intercepts it and returns a `400 Bad Request` *before* your controller is even executed.

### 2.1 Standard Extractors
| Extractor | Use Case | NestJS Equivalent |
| --- | --- | --- |
| `Path<T>` | Route Parameters (`/users/:id`) | `@Param()` |
| `Query<T>` | Query String (`?name=ferrox`) | `@Query()` |
| `Json<T>` | JSON Request Body | `@Body()` |
| `HeaderMap` | All HTTP Headers | `@Headers()` |

### 2.2 Advanced Extractors (Multipart, SSE, WebSockets)

When you need to handle complex payloads, Ferrox provides specialized extractors.

#### Multipart Forms (File Uploads)
Handling massive file uploads efficiently requires streaming the bytes, rather than loading a 5GB video into RAM.

```rust
use axum::extract::Multipart;

async fn upload_video(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        
        // We can process the file chunks asynchronously here
        // avoiding Memory Out Of Bounds errors.
        let data = field.bytes().await.unwrap();
        println!("Received {} bytes for {}", data.len(), file_name);
    }
}
```

#### Server-Sent Events (SSE)
For real-time unidirectional data (like a ChatGPT typing response), you can return an Event Stream.

```rust
use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Create a stream that yields an event every second
    let stream = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
        .map(|_| Ok(Event::default().data("Server time tick!")));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
```

---

## 3. Advanced Under the Hood

How does Ferrox manage memory when a client sends a 10GB JSON payload? 

By default, the `Json<T>` extractor has a **strict 2MB limit**. If a payload exceeds this limit, Ferrox drops the TCP connection to prevent **OOM (Out of Memory) DDoS attacks**.

If you need to accept larger payloads for a specific route, you explicitly configure the limit per-route using `axum::extract::DefaultBodyLimit`.

```rust
use axum::extract::DefaultBodyLimit;
use axum::Router;
use axum::routing::post;

let app = Router::new()
    .route("/massive-upload", post(massive_handler))
    // Override the limit to 50MB for this specific route
    .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); 
```

---

## 4. ✅ Best Practices

### 4.1 Thin Controllers, Fat Providers
A Controller should never contain business logic, complex IF statements, or direct database queries. It should only extract data, pass it to a Provider (Service), and format the Provider's result into an HTTP Response.

**✅ DO:**
```rust
async fn create_user(
    State(user_service): State<UserService>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<User>, AppError> {
    // The controller delegates everything to the service
    let user = user_service.create(payload).await?;
    Ok(Json(user))
}
```

### 4.2 Sub-Router Composition (Nesting)
Don't define all your routes in a giant `main.rs` file. Nest routers to keep your codebase modular, exactly like NestJS Modules.

**✅ DO:**
```rust
let api_router = Router::new()
    .nest("/users", users_controller())
    .nest("/orders", orders_controller());

let app = Router::new().nest("/api/v1", api_router);
```

---

## 5. ❌ Anti-Patterns

Understanding what *not* to do in Rust is critical, as some mistakes can bring down your entire server cluster.

### 5.1 Blocking the Tokio Thread 💀
Ferrox runs on **Tokio**, an asynchronous runtime. Tokio uses a pool of Worker Threads (usually equal to your CPU cores). If you run a synchronous, blocking operation (like reading a huge file with `std::fs` or heavy cryptography) inside an `async fn` controller, you "steal" that thread. If you have 8 cores and 8 requests do this concurrently, **your entire server stops responding to all other users**.

**❌ DON'T:**
```rust
async fn blocking_controller() -> &'static str {
    // 💀 FATAL: This blocks the Tokio Worker thread!
    std::thread::sleep(std::time::Duration::from_secs(5)); 
    
    // 💀 FATAL: Synchronous File I/O blocks the thread!
    let data = std::fs::read_to_string("huge_log.txt").unwrap(); 
    
    "Done"
}
```

**✅ DO:**
```rust
async fn non_blocking_controller() -> &'static str {
    // ✅ SAFE: Yields the thread back to Tokio while waiting
    tokio::time::sleep(std::time::Duration::from_secs(5)).await; 
    
    // ✅ SAFE: Asynchronous File I/O
    let data = tokio::fs::read_to_string("huge_log.txt").await.unwrap(); 
    
    // ✅ SAFE: For CPU-heavy math/crypto, offload to a blocking thread pool
    let hash = tokio::task::spawn_blocking(|| {
        bcrypt::hash("password", 10).unwrap()
    }).await.unwrap();
    
    "Done"
}
```

### 5.2 Unwrapping inside a Controller
If a client sends an unexpected header, and you call `.unwrap()` to parse it, your thread will Panic. While Axum catches panics and prevents the server from crashing entirely, it is a severe anti-pattern that disrupts the connection abruptly. Always use `?` and return proper `AppError` types.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*