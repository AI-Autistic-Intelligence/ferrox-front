---
sidebar_position: 2
---

# 🐘 Relational Databases (SeaORM)

For relational databases (PostgreSQL, MySQL, SQLite), Ferrox integrates with [SeaORM](https://www.sea-ql.org/SeaORM/) via `ferrox-database-seaorm`.

SeaORM provides an async ORM built on top of `sqlx`, offering compile-time safety, dynamic query building, and seamless integration with Tokio.

---

## 1. Connecting to PostgreSQL / MySQL

Initialize the SeaORM database connection pool during application bootstrap:

```rust
use sea_orm::{Database, DatabaseConnection};
use ferrox_errors::AppError;

pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, AppError> {
    let db = Database::connect(database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    println!("✅ Connected to Relational Database via SeaORM");
    Ok(db)
}
```

---

## 2. Implementing a SeaORM Repository

Implement the `Repository` trait for your entity:

```rust
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use ferrox_database_core::Repository;
use ferrox_errors::AppError;
use crate::entities::user::{Entity as UserEntity, ActiveModel as UserActiveModel, Model as UserModel};

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Repository<UserModel, i32> for SeaOrmUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<UserModel>, AppError> {
        UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<UserModel>, AppError> {
        UserEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn insert(&self, user: UserModel) -> Result<UserModel, AppError> {
        let active_model = UserActiveModel {
            name: Set(user.name),
            email: Set(user.email),
            ..Default::default()
        };

        active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn update(&self, _id: i32, _user: UserModel) -> Result<UserModel, AppError> {
        // Implement update logic
        todo!()
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        UserEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
```

---

## 3. Database Transactions

SeaORM supports async transactions for multi-step atomic operations:

```rust
use sea_orm::TransactionTrait;

let txn = db.begin().await?;
// Perform multiple queries inside txn...
txn.commit().await?;
```

---

## 4. ✅ Best Practices

- **Configure connection pool bounds**: Always set max pool size (`max_connections`) based on database CPU core count to avoid exhaustion under high concurrency.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*