---
sidebar_position: 4
---

# 🛠️ Code Factory & Generic CRUD Generator

Building standard CRUD (Create, Read, Update, Delete) endpoints for dozens of entities generates massive amounts of repetitive boilerplate.

Ferrox solves this with `ferrox-crud-gen`—a macro system that auto-generates REST endpoints, AutoZod payload validation, and GraphQL schemas with a single line of Rust code.

---

## 1. `crud_router!` Macro

The `crud_router!` macro constructs an Axum `Router` providing 5 standard CRUD endpoints:
- `GET /` -> `repo.find_all()`
- `GET /:id` -> `repo.find_by_id(id)`
- `POST /` -> `repo.insert(payload)` (with `ValidatedJson` validation)
- `PATCH /:id` -> `repo.update(id, payload)`
- `DELETE /:id` -> `repo.delete(id)`

### Usage Example

```rust
use axum::Router;
use std::sync::Arc;
use ferrox_crud_gen::crud_router;
use crate::models::{Product, ProductRepository};

pub fn products_router() -> Router<Arc<ProductRepository>> {
    crud_router!(Product, u64, ProductRepository)
}
```

---

## 2. `vertical_slice!` Macro

For ultra-rapid prototyping, the `vertical_slice!` macro generates the entire vertical slice of a feature: DTO struct definition, `validator::Validate` derivations, GraphQL inputs, and the REST Router.

```rust
use ferrox_crud_gen::vertical_slice;
use crate::repositories::UserRepository;

// Defines DTO, validation rules, GraphQL schema, and REST router!
vertical_slice!(User, u64, UserRepository, {
    username: String,
    email: String,
    age: u8
});
```

---

## 3. How `crud_router!` Works Under the Hood

The macro leverages Axum's generic state extraction and Ferrox's `Repository<Entity, Id>` trait:

```rust
macro_rules! crud_router {
    ($entity:ty, $id:ty, $repo:ty) => {{
        use axum::{
            extract::{Path, State},
            routing::{get, post, delete, patch},
            Json, Router,
        };
        use std::sync::Arc;
        use ferrox_database_core::Repository;
        use ferrox_validation::ValidatedJson;

        Router::<Arc<$repo>>::new()
            .route("/", get(|State(repo): State<Arc<$repo>>| async move {
                let items = repo.find_all().await?;
                Ok::<_, ferrox_errors::AppError>(Json(items))
            }))
            .route("/:id", get(|Path(id): Path<$id>, State(repo): State<Arc<$repo>>| async move {
                match repo.find_by_id(id).await? {
                    Some(item) => Ok::<_, ferrox_errors::AppError>(Json(item)),
                    None => Err(ferrox_errors::AppError::NotFound("Entity not found".into())),
                }
            }))
            .route("/", post(|State(repo): State<Arc<$repo>>, ValidatedJson(payload): ValidatedJson<$entity>| async move {
                let created = repo.insert(payload).await?;
                Ok::<_, ferrox_errors::AppError>(Json(created))
            }))
    }};
}
```

---

## 4. ✅ Best Practices

- **Use macros for simple entity tables**: Save developer time on standard admin tables and simple resource management endpoints.
- **Override routes when custom logic is needed**: For routes requiring complex business orchestrations or email triggers, attach custom route handlers alongside or instead of the auto-generated router.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*