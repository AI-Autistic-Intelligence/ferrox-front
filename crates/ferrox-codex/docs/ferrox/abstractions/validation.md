---
sidebar_position: 3
---

# 🎯 DTO Validation & AutoZod Extractor

In NestJS, developer productivity relies heavily on `class-validator` and `ValidationPipe`. In Rust, `ferrox-validation` provides the `ValidatedJson<T>` extractor—a mechanism that behaves like Zod's `.parse()`.

`ValidatedJson<T>` parses incoming HTTP request JSON payloads and automatically executes validation rules defined on your DTO structs using `validator::Validate`.

---

## 1. Defining a Validated DTO

Annotate your Data Transfer Object (DTO) with `#[derive(Deserialize, Validate)]` and add field constraints:

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email address format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[validate(range(min = 18, max = 120, message = "Age must be between 18 and 120"))]
    pub age: u8,
}
```

---

## 2. Using `ValidatedJson<T>` in Controller Handlers

Replace Axum's standard `Json<T>` extractor with `ValidatedJson<T>`:

```rust
use axum::{routing::post, Json, Router};
use ferrox_validation::ValidatedJson;
use ferrox_errors::AppError;

pub async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserDto>,
) -> Result<Json<UserResponseDto>, AppError> {
    // 100% Guaranteed: payload is valid JSON AND satisfies all validation rules!
    println!("Creating user: {} ({})", payload.username, payload.email);

    Ok(Json(UserResponseDto {
        id: 1,
        username: payload.username,
        email: payload.email,
    }))
}
```

---

## 3. How "Fail Fast" Validation Works

If a client sends an invalid payload (e.g. `age: 15` or malformed email `not-an-email`), `ValidatedJson<T>` intercepts the request **before** executing your handler and returns a `400 Bad Request` response:

```json
{
  "status": 400,
  "message": "Validation failed: email: Invalid email address format, age: Age must be between 18 and 120"
}
```

```
HTTP Client ---> [ ValidatedJson Extractor ] 
                        |
                        +---> Invalid? ---> Return 400 Bad Request immediately (0 DB/CPU overhead)
                        |
                        v
                 [ Controller Handler ]
```

---

## 4. Custom Validation Functions

You can attach custom validation functions to complex DTOs using `#[validate(custom = "path::to::fn")]`:

```rust
use validator::ValidationError;

fn validate_domain(email: &str) -> Result<(), ValidationError> {
    if !email.ends_with("@company.com") {
        return Err(ValidationError::new("invalid_corporate_email"));
    }
    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct CorporateSignupDto {
    #[validate(custom = "validate_domain")]
    pub corporate_email: String,
}
```

---

## 5. ✅ Best Practices

- **Validate all user input DTOs**: Never trust raw JSON strings. Always wrap handler parameters in `ValidatedJson<T>`.
- **Provide clear error messages**: Use descriptive `message` parameters in `#[validate(...)]` annotations for smooth frontend user experience.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*