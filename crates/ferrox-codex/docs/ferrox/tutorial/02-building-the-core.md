---
sidebar_position: 2
---

# 🏗️ Part 2: Building the Core

Now that our application is running, let's implement the core logic for our Ticketing System following the **"Thin Controllers, Fat Providers"** best practice.

## 1. The DTOs (Data Transfer Objects)

We need to define how data enters and exits our API. We will use `serde` for JSON, `validator` for security checks, and `ts-rs` to export the types later.

Create a new file `src/dto.rs`:

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;
use ts_rs::TS;
use uuid::Uuid;

// The entity returned to the client
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

// The incoming payload to create a ticket
#[derive(Debug, Deserialize, Validate, TS)]
#[ts(export)]
pub struct CreateTicketDto {
    #[validate(length(min = 5, message = "Title must be at least 5 characters"))]
    pub title: String,
}
```

## 2. The Provider (TaskService)

We need an in-memory database to store our tickets. Since Tokio uses multiple worker threads, we must protect our `Vec<Ticket>` with a thread-safe `Arc<Mutex>`.

Create `src/service.rs`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::dto::{Ticket, CreateTicketDto};

#[derive(Clone)]
pub struct TicketService {
    db: Arc<Mutex<Vec<Ticket>>>,
}

impl TicketService {
    pub fn new() -> Self {
        Self { db: Arc::new(Mutex::new(Vec::new())) }
    }

    pub async fn create_ticket(&self, dto: CreateTicketDto) -> Ticket {
        let ticket = Ticket {
            id: Uuid::new_v4(),
            title: dto.title,
            status: "OPEN".to_string(),
        };

        // We lock the database asynchronously to avoid blocking the Tokio thread!
        let mut db_lock = self.db.lock().await;
        db_lock.push(ticket.clone());
        
        ticket
    }

    pub async fn get_tickets(&self) -> Vec<Ticket> {
        let db_lock = self.db.lock().await;
        db_lock.clone()
    }
}
```

## 3. The Controller & Security

Finally, we expose our Service via HTTP. We will also protect the route using Ferrox's Zero-Trust authentication middleware.

Create `src/controller.rs`:

```rust
use axum::{extract::State, http::StatusCode, Json, Router, routing::{get, post}, middleware};
use ferrox_security::paseto_auth; // Our Zero-Trust JWT verifier
use crate::{AppState, dto::{Ticket, CreateTicketDto}};

// The Controller acts as the routing boundary
pub fn tickets_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tickets).post(create_ticket))
        // 🔒 Protect all routes in this controller
        .route_layer(middleware::from_fn(paseto_auth))
}

// ❌ If the `CreateTicketDto` validation fails, this function is NEVER called.
async fn create_ticket(
    State(state): State<AppState>,
    Json(payload): Json<CreateTicketDto>,
) -> (StatusCode, Json<Ticket>) {
    
    // We delegate the heavy lifting to the Provider
    let ticket = state.ticket_service.create_ticket(payload).await;
    
    (StatusCode::CREATED, Json(ticket))
}

async fn list_tickets(
    State(state): State<AppState>,
) -> Json<Vec<Ticket>> {
    let tickets = state.ticket_service.get_tickets().await;
    Json(tickets)
}
```

## 4. Wiring it all together

Go back to `src/main.rs` and inject the `TicketService` into the `AppState`, then nest the `tickets_router`.

```rust
// ... imports ...
use crate::service::TicketService;
use crate::controller::tickets_router;

#[derive(Clone)]
pub struct AppState {
    pub ticket_service: TicketService,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        ticket_service: TicketService::new(),
    };

    let app = Router::new()
        .nest("/api/tickets", tickets_router()) // Composition!
        .with_state(state);

    // ... bind and serve ...
}
```

Our core architecture is now complete and highly secure! In the final chapter, we will generate the TypeScript client for the Frontend.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*