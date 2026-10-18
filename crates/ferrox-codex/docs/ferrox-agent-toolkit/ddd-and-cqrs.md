---
title: Pure Domain-Driven Design & CQRS Architecture
sidebar_position: 5
description: Architectural principles of pure Domain-Driven Design, Aggregate Invariants, Value Objects, Domain Events, and CQRS contracts.
keywords: [ferrox, ddd, cqrs, domain-driven-design, aggregates, value-objects, invariants]
---

# 🏛️ Pure Domain-Driven Design & CQRS Architecture

The **Ferrox Agent Toolkit** enforces pure Domain-Driven Design (DDD) through the `ferrox-ddd-planner` skill and agent, governed by `rules/ferrox-ddd-architecture.md`.

Domain logic represents the core intellectual property of any enterprise system. Frameworks, databases, web servers, and third-party APIs are ephemeral implementation details that must remain strictly isolated from domain models.

---

## 📐 The Onion Boundary Model

```
+---------------------------------------------------------------------------------+
|                       INFRASTRUCTURE / ADAPTERS LAYER                           |
|       (Tokio, Axum, Actix, NestJS, Pydantic, SQLAlchemy, Redpanda, Redis)       |
|                                                                                 |
|       +-----------------------------------------------------------------+       |
|       |                     APPLICATION / CQRS LAYER                    |       |
|       |              (Command Handlers, Query Handlers, Ports)          |       |
|       |                                                                 |       |
|       |       +-------------------------------------------------+       |       |
|       |       |               PURE DOMAIN LAYER                 |       |       |
|       |       |        (Aggregates, Invariants, Entities,       |       |       |
|       |       |         Value Objects, Pure Domain Events)      |       |       |
|       |       +-------------------------------------------------+       |       |
|       +-----------------------------------------------------------------+       |
+---------------------------------------------------------------------------------+
```

### Dependency Rule
Dependencies **point strictly inward**. The Pure Domain Layer has **ZERO** outward dependencies. It cannot import:
- No HTTP frameworks (`axum`, `actix-web`, `@nestjs/common`, `fastapi`).
- No Database ORMs (`diesel`, `sqlx`, `typeorm`, `prisma`, `sqlalchemy`).
- No Clock or File I/O (`std::time::Instant`, `fs`, `Date.now()`). Time and randomness must be injected as pure inputs.

---

## 🧱 Pure Domain Primitives

### 1. Value Objects
- Immutable structures defined purely by their attributes rather than a persistent identity.
- Self-validating upon construction: invalid values cannot be instantiated.
- Zero-cost allocation and value equality semantics.

```rust
// Pure Rust Value Object Example (Zero framework dependencies)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyIso(String);

impl CurrencyIso {
    pub fn new(code: &str) -> Result<Self, DomainError> {
        let trimmed = code.trim().to_uppercase();
        if trimmed.len() != 3 || !trimmed.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(DomainError::InvalidCurrencyIso(code.to_string()));
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### 2. Aggregates & Invariants
- An Aggregate Root is the sole gateway for mutating domain state.
- Invariants are business rules that must hold true at all times before and after every state mutation.
- Atomic mutation: operations return pure `DomainResult<Vec<DomainEvent>, DomainError>`.

```rust
pub struct AccountAggregate {
    id: AccountId,
    balance: Money,
    status: AccountStatus,
}

impl AccountAggregate {
    pub fn debit(&mut self, amount: Money) -> Result<AccountDebitedEvent, DomainError> {
        if self.status != AccountStatus::Active {
            return Err(DomainError::AccountInactive);
        }
        if self.balance < amount {
            return Err(DomainError::InsufficientFunds);
        }
        self.balance = self.balance - amount;
        Ok(AccountDebitedEvent::new(self.id.clone(), amount))
    }
}
```

---

## ⚡ Command & Query Responsibility Segregation (CQRS)

Write operations (Commands) and Read operations (Queries) follow separate contractual pipelines:

```mermaid
graph LR
    subgraph Write Pipeline (Commands)
        Cmd[Command: DebitAccount] --> CmdHandler[CommandHandler]
        CmdHandler --> Agg[AccountAggregate]
        Agg --> Evt[DomainEvent: AccountDebited]
        Evt --> Outbox[Transactional Outbox]
    end

    subgraph Read Pipeline (Queries)
        Qry[Query: GetAccountBalance] --> QryHandler[QueryHandler]
        QryHandler --> ReadModel[(Optimized Read Store)]
        ReadModel --> Res[AccountBalanceDTO]
    end
```

### CQRS Rules:
1. **Commands Mutate State**: Return success or failure; do NOT return full entity trees.
2. **Queries Never Mutate State**: Return immutable DTOs or projections; execute with zero side effects.
3. **Eventual Consistency**: Read models update asynchronously via Domain Events streamed through the Transactional Outbox.
