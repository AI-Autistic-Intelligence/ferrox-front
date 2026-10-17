---
sidebar_position: 2
---

# 🕸️ GraphQL API Integration

GraphQL allows frontend clients to request precisely the data fields they need, reducing over-fetching and network payload sizes.

Ferrox provides `ferrox-graphql`, integrating [Async-GraphQL](https://async-graphql.github.io/) with Axum.

---

## 1. Building a GraphQL Schema

Define GraphQL query objects using `async_graphql`:

```rust
use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns application health ping
    async fn ping(&self) -> &'static str {
        "pong"
    }

    /// Returns a list of products
    async fn products(&self) -> Vec<ProductGql> {
        vec![
            ProductGql { id: 1, name: "Ferrox Pro".into() }
        ]
    }
}

pub struct ProductGql {
    pub id: u64,
    pub name: String,
}

#[Object]
impl ProductGql {
    async fn id(&self) -> u64 { self.id }
    async fn name(&self) -> &str { &self.name }
}

pub fn create_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
```

---

## 2. Axum GraphQL Route Handler

```rust
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub fn graphql_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
}
```

---

## 3. Exporting GraphQL SDL for Frontend Code Generation

`ferrox-graphql` includes helper utilities to export your schema in Schema Definition Language (SDL) format during build phase:

```rust
use ferrox_graphql::export_sdl;

let schema = create_schema();
export_sdl(&schema, "./schema.graphql")?;
// Generates schema.graphql for GraphQL Code Generator in React/Vue/Angular
```

---

## 4. Testing GraphQL Queries

```rust
#[tokio::test]
async fn test_ping_query() {
    let schema = create_schema();
    let response = schema.execute("{ ping }").await;
    assert_eq!(response.data.into_json().unwrap()["ping"], "pong");
}
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*