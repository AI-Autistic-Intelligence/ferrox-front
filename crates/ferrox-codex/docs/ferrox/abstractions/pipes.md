---
sidebar_position: 1
---

# 🛑 Pipes (Validation)

In NestJS, a **Pipe** is a class annotated with `@Injectable()` that implements the `PipeTransform` interface, typically used to validate or transform input data before it reaches the Controller.

In Ferrox, Pipes are implemented automatically through the **Validation Extractor** pattern, leveraging the `validator` crate.

## High-Level Example

To validate incoming JSON data, you simply need to decorate your Data Transfer Object (DTO) with validation rules, and use the `Json` extractor in your controller.

```rust
use axum::{Json, routing::post, Router};
use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password too short"))]
    pub password: String,
}

// The framework automatically runs the Pipe (validation) before executing this function
async fn register_user(Json(payload): Json<CreateUserDto>) -> Json<&'static str> {
    payload.validate().unwrap(); // Trigger the Pipe
    
    Json("User Created!")
}
```

## Low-Level Internal Details

While you can manually call `.validate()` inside the controller, Ferrox allows you to build custom Extractors that act as automatic Pipes. 

By implementing `FromRequest` on a wrapper struct (e.g., `ValidJson<T>`), Ferrox will intercept the raw TCP bytes, deserialize them into JSON, and run the `validator` rules. If the rules fail, Ferrox intercepts the error and immediately returns an `HTTP 400 Bad Request` with a structured `AppError::ValidationError`, completely bypassing the Controller logic.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*