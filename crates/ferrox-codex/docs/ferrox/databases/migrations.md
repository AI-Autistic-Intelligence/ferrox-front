---
sidebar_position: 5
---

# 🔄 Schema Migrations Management

As production systems evolve, database schemas require incremental versioning without downtime. `ferrox-migrations` provides automated schema migrations runner for relational and document databases.

---

## 1. Migration File Structure

Migrations are organized chronologically with up/down SQL or Rust migration steps:

```text
migrations/
├── m20260101_000001_create_users_table.rs
├── m20260201_000002_add_roles_to_users.rs
└── mod.rs
```

---

## 2. Defining a SeaORM Migration

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
}
```

---

## 3. Running Migrations Programmatically

Execute pending migrations automatically on startup during `main.rs` bootstrap:

```rust
use sea_orm_migration::prelude::*;
use ferrox_migrations::Migrator;

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), AppError> {
    println!("🔄 Running database migrations...");
    Migrator::up(db, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    println!("✅ Database schema up-to-date!");
    Ok(())
}
```

---

## 4. ✅ Best Practices

- **Never modify published migrations**: Always create new migration files for schema changes to ensure CI/CD repeatability across environments.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*