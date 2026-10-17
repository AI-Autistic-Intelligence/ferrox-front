---
sidebar_position: 3
---

# 🔄 The Saga Pattern (Distributed Transactions)

In a monolithic architecture, you can wrap multiple database operations in a single SQL Transaction (`BEGIN; ... COMMIT;`). If any operation fails, you simply issue a `ROLLBACK`.

In a Microservice architecture, this is impossible. You cannot lock a row in the "Inventory Database" and a row in the "Payments Database" simultaneously without catastrophic performance degradation (Distributed Locking / Two-Phase Commit).

Ferrox solves this using the **Saga Pattern** via `ferrox-saga`.

## How Sagas Work

A Saga is a sequence of local transactions. Each local transaction updates the database and publishes an event to trigger the next local transaction in the saga. 

If a local transaction fails, the saga executes a series of **Compensating Transactions** that undo the changes made by the preceding local transactions.

### High-Level Example

```rust
use ferrox_saga::{Saga, SagaStep};

// Define the steps and their compensating rollbacks
let book_hotel = SagaStep::new(
    "Book Hotel",
    || async { call_hotel_microservice().await },                 // Action
    || async { cancel_hotel_reservation_microservice().await }    // Compensation
);

let book_flight = SagaStep::new(
    "Book Flight",
    || async { call_airline_microservice().await },               // Action
    || async { cancel_flight_microservice().await }               // Compensation
);

// If book_flight fails, the Saga orchestrator will automatically execute 
// `cancel_hotel_reservation_microservice()` to restore consistency.
let holiday_saga = Saga::builder()
    .add_step(book_hotel)
    .add_step(book_flight)
    .build();

match holiday_saga.execute().await {
    Ok(_) => println!("Holiday booked successfully!"),
    Err(_) => println!("Booking failed. All previous steps were rolled back."),
}
```

## Low-Level Internal Details

The `ferrox-saga` orchestrator stores the execution state in a state machine. As each `SagaStep` succeeds, its Compensation Closure is pushed onto a Stack (LIFO - Last In, First Out). 

If a `SagaStep` returns an Error, the orchestrator stops moving forward, pops the Compensation Closures off the stack one by one, and executes them in reverse order. This ensures eventual consistency across distributed microservices without ever holding a distributed database lock.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*