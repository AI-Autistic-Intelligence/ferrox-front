---
sidebar_position: 1
---

# 💾 Persistence Architecture & Repository Pattern

In enterprise software development, coupling controllers or business logic directly to raw database queries creates brittle, untestable code.

Ferrox enforces the **Repository Pattern** through `ferrox-database-core`, decoupling domain logic from the underlying storage mechanism (SQL relational, NoSQL document, or In-memory key-value).

---

## 1. The `Repository<Entity, Id>` Trait

All persistence modules in Ferrox implement the generic `Repository` trait:

```rust
use async_trait::async_trait;
use ferrox_errors::AppError;

#[async_trait]
pub trait Repository<Entity, Id>: Send + Sync {
    /// Finds a single entity by its primary key
    async fn find_by_id(&self, id: Id) -> Result<Option<Entity>, AppError>;
    
    /// Finds all entities
    async fn find_all(&self) -> Result<Vec<Entity>, AppError>;
    
    /// Inserts a new entity
    async fn insert(&self, entity: Entity) -> Result<Entity, AppError>;
    
    /// Updates an existing entity
    async fn update(&self, id: Id, entity: Entity) -> Result<Entity, AppError>;
    
    /// Deletes an entity by its primary key
    async fn delete(&self, id: Id) -> Result<(), AppError>;
}
```

---

## 2. Multi-Database Architecture

Ferrox provides specialized crates for different storage engines:

```
                          +--------------------------------+
                          |   Repository<Entity, Id> Trait  |
                          +--------------------------------+
                                          |
          +-------------------------------+-------------------------------+
          |                               |                               |
          v                               v                               v
+-------------------+           +-------------------+           +-------------------+
|  ferrox-database- |           |  ferrox-database- |           |  ferrox-database- |
|      seaorm       |           |       mongo       |           |       redis       |
| (Postgres/MySQL)  |           |     (MongoDB)     |           |  (Cache & Locks)  |
+-------------------+           +-------------------+           +-------------------+
```

---

## 3. Benefits of the Repository Pattern

1. **Testability & Mocking**: You can swap real database implementations with in-memory mock repositories in unit tests without starting a Docker database container.
2. **Database Flexibility**: Migrating from PostgreSQL to MongoDB or SQLite requires changing repository trait implementations without modifying service or controller layers.
3. **Clean Architecture**: Services operate exclusively on domain structs rather than SQL rows or BSON documents.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*