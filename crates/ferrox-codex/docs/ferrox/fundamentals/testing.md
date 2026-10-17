---
sidebar_position: 7
---

# 🧪 Testing & Quality Assurance Strategy

Testing in enterprise Rust applications must cover multiple levels of isolation: fast unit tests for business logic, in-memory route tests for controllers without binding to network sockets, and ephemeral containerized integration tests for database operations.

This guide explores the complete testing suite strategy in Ferrox.

---

## 1. Unit Testing Controllers & Services

Unit tests verify business logic in complete isolation using Tokio's test runner (`#[tokio::test]`).

### Example: Testing a Service with Mock Repositories

Using Rust's generic `Repository<Entity, Id>` trait, you can pass mock repository structs to services without starting external database servers:

```rust
use async_trait::async_trait;
use ferrox_database_core::Repository;
use ferrox_errors::AppError;

pub struct MockUserRepository {
    pub users: Vec<User>,
}

#[async_trait]
impl Repository<User, u64> for MockUserRepository {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, AppError> {
        Ok(self.users.iter().find(|u| u.id == id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        Ok(self.users.clone())
    }

    async fn insert(&self, user: User) -> Result<User, AppError> {
        Ok(user)
    }

    async fn update(&self, _id: u64, user: User) -> Result<User, AppError> {
        Ok(user)
    }

    async fn delete(&self, _id: u64) -> Result<(), AppError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_user_service_lookup() {
    let mock_repo = MockUserRepository {
        users: vec![User { id: 1, name: "Alice".into() }],
    };

    let result = mock_repo.find_by_id(1).await.unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Alice");
}
```

---

## 2. In-Memory Axum Controller Tests (`tower::ServiceExt`)

Axum applications implement `tower::Service`. You can execute HTTP requests directly against your router in memory **without opening network ports or sockets**:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt; // for `oneshot`
use serde_json::json;

fn app() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = app();

    // Send mock HTTP GET /health request
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## 3. Database Integration Testing with `testcontainers-rs`

For true integration tests against real PostgreSQL or MongoDB instances without cluttering shared dev databases, use `testcontainers-rs` to automatically launch ephemeral Docker containers for the duration of the test:

```rust
use testcontainers::{clients, images::postgres};
use sea_orm::{Database, DatabaseConnection};

#[tokio::test]
async fn test_postgres_integration() {
    let docker = clients::Cli::default();
    let postgres_node = docker.run(postgres::Postgres::default());

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        postgres_node.get_host_port_ipv4(5432)
    );

    let db = Database::connect(&connection_string).await.unwrap();
    
    // Run schema migrations and execute real DB queries...
    assert!(db.ping().await.is_ok());
}
```

---

## 4. ✅ Best Practices

- **Keep Unit Tests Under 5ms**: Use mock repositories for unit tests so `cargo test` runs instantly during local development.
- **Isolate Integration Tests**: Annotate slow Docker integration tests with `#[ignore]` or feature flags so they run primarily during CI/CD pipeline execution (`cargo test -- --ignored`).
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*