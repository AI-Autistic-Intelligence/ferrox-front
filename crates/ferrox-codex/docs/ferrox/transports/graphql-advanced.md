---
sidebar_position: 3
---

# 🚀 Advanced GraphQL: DataLoader & Subscriptions

Standard GraphQL query resolvers suffer from the infamous **N+1 Query Problem**. If a client fetches a list of 100 posts along with their authors, a naive resolver fires 1 initial query for posts plus 100 individual database queries for each author (101 total DB roundtrips!).

Ferrox integrates `async-graphql::dataloader` to batch and deduplicate database requests automatically into a single SQL `IN (...)` query.

---

## 1. Solving N+1 Queries with `DataLoader`

```mermaid
graph TD
    Client((Client)) -->|1 GraphQL Query| GQL[GraphQL Engine]
    
    subgraph Without DataLoader (N+1 Problem)
        GQL -->|DB Query 1| DB1[(Post 1 Author)]
        GQL -->|DB Query 2| DB2[(Post 2 Author)]
        GQL -->|DB Query 100| DB3[(Post 100 Author)]
    end
    
    subgraph With Ferrox DataLoader
        GQL -->|Batches 100 IDs| DL[DataLoader]
        DL -->|1 SQL Query: SELECT * WHERE id IN ...| DB[(Database)]
    end
```

### Implementing a Loader

```rust
use async_graphql::dataloader::*;
use std::collections::HashMap;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use crate::entities::user::{Entity as UserEntity, Model as UserModel, Column};

pub struct UserLoader {
    pub db: DatabaseConnection,
}

#[async_trait::async_trait]
impl Loader<u64> for UserLoader {
    type Value = UserModel;
    type Error = std::sync::Arc<sea_orm::DbErr>;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        // Single batch SQL query: SELECT * FROM users WHERE id IN (1, 2, 3, ... 100)
        let users = UserEntity::find()
            .filter(Column::Id.one_of(keys.to_vec()))
            .all(&self.db)
            .await
            .map_err(std::sync::Arc::new)?;

        Ok(users.into_iter().map(|u| (u.id, u)).collect())
    }
}
```

---

## 2. Using `DataLoader` in Resolvers

```rust
use async_graphql::{Context, Object, Result, dataloader::DataLoader};

#[Object]
impl PostGql {
    async fn author(&self, ctx: &Context<'_>) -> Result<UserGql> {
        let loader = ctx.data::<DataLoader<UserLoader>>()?;
        let user = loader.load_one(self.author_id).await?
            .ok_or("Author not found")?;
            
        Ok(UserGql::from(user))
    }
}
```

---

## 3. Real-Time GraphQL Subscriptions

`ferrox-graphql` supports WebSocket subscriptions using Tokio broadcast channels:

```rust
use async_graphql::{Subscription, SimpleObject};
use futures::Stream;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn live_stock_price(&self, symbol: String) -> impl Stream<Item = PriceTick> {
        // Stream ticks over WebSockets
        tokio_stream::wrappers::IntervalStream::new(
            tokio::time::interval(std::time::Duration::from_secs(1))
        ).map(move |_| PriceTick { symbol: symbol.clone(), price: 150.25 })
    }
}

#[derive(SimpleObject, Clone)]
pub struct PriceTick {
    pub symbol: String,
    pub price: f64,
}
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*