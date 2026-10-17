---
sidebar_position: 2
---

# 💉 Providers & Dependency Injection

Providers are a fundamental concept in Ferrox. Many of the basic Ferrox classes may be treated as a provider – services, repositories, factories, helpers, and so on.

The main idea of a provider is that it can be **injected** as a dependency. In a traditional OOP language like TypeScript or Java, this is done via an IoC (Inversion of Control) container that reads class constructors.

In Rust, runtime reflection is notoriously difficult. Ferrox achieves powerful, compile-time Dependency Injection (DI) leveraging Axum's Application `State` and dynamic `Extensions`.

## Services

Let's start by creating a simple `CatsService`. This service will be responsible for data storage and retrieval, and is designed to be used by the `CatsController`.

```rust
// cats_service.rs
use std::sync::Arc;
use tokio::sync::Mutex;

// In Ferrox, a Provider is usually just a struct containing its dependencies
#[derive(Clone)]
pub struct CatsService {
    // We use Arc<Mutex<T>> to safely share state across threads
    cats: Arc<Mutex<Vec<Cat>>>,
}

impl CatsService {
    pub fn new() -> Self {
        Self { cats: Arc::new(Mutex::new(Vec::new())) }
    }

    pub async fn create(&self, cat: Cat) {
        let mut lock = self.cats.lock().await;
        lock.push(cat);
    }

    pub async fn find_all(&self) -> Vec<Cat> {
        let lock = self.cats.lock().await;
        lock.clone()
    }
}
```

> [!NOTE]
> Unlike NestJS where you decorate the class with `@Injectable()`, in Ferrox you just need to ensure your service derives `Clone`. This allows Tokio to cheaply pass references to the service across multiple worker threads.

## Dependency Injection (Application State)

To inject the `CatsService` into our `CatsController`, we register it in the application's **State**.

```rust
use axum::{extract::State, routing::get, Json, Router};

// 1. Define the Global State of your application
#[derive(Clone)]
pub struct AppState {
    pub cats_service: CatsService,
}

// 2. The Controller injects the State automatically
async fn find_all(
    State(state): State<AppState>, // <-- Dependency Injection happens here!
) -> Json<Vec<Cat>> {
    let cats = state.cats_service.find_all().await;
    Json(cats)
}

// 3. Register the Provider at bootstrap
#[tokio::main]
async fn main() {
    let state = AppState {
        cats_service: CatsService::new(),
    };

    let app = Router::new()
        .route("/cats", get(find_all))
        .with_state(state); // <-- Registering the providers

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Injection Scopes

In NestJS, you have three injection scopes: `DEFAULT` (Singleton), `REQUEST`, and `TRANSIENT`.

In Ferrox, Scopes are handled natively by Rust's memory management:

### 1. Singleton Scope (Default)
By wrapping your service in an `Arc` (Atomic Reference Counted) pointer and putting it in the `AppState`, the service is instantiated exactly **once** when the application boots. All requests share the same instance. This is highly performant and the recommended approach.

### 2. Request Scope
If you need a new instance of a service for every incoming request (e.g., a service that holds a temporary database transaction or request-specific metadata), you use Axum's `Extensions`. A Middleware intercepts the request, instantiates the service, and attaches it to the request extensions.

```rust
use axum::extract::Extension;

async fn request_scoped_controller(
    // Extracted from the current request only
    Extension(transaction_service): Extension<TransactionService>, 
) {
    // ...
}
```

### 3. Transient Scope
If you want a new instance every single time the service is requested within the same request lifecycle, you can use a **Factory Provider**.

## Custom Providers (Factory Providers)

Sometimes you want to construct a provider dynamically. For instance, maybe you need to read a configuration file asynchronously, or choose an implementation based on an environment variable.

```rust
pub async fn database_factory() -> Arc<dyn DatabaseInterface> {
    let env = std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string());
    
    if env == "production" {
        Arc::new(PostgresDatabase::connect().await)
    } else {
        Arc::new(InMemoryDatabase::new())
    }
}
```

You can `await` this factory during your application bootstrap and then inject the resulting `Arc<dyn DatabaseInterface>` trait object into your `AppState`.

## Interfaces (Traits)

In TypeScript, you often inject interfaces. In Rust, we use **Traits** and dynamic dispatch (`dyn Trait`) to achieve the exact same decoupling.

```rust
// 1. Define the Interface
pub trait PaymentGateway: Send + Sync {
    fn process_payment(&self, amount: u32) -> Result<(), AppError>;
}

// 2. Implement it
pub struct StripeGateway;
impl PaymentGateway for StripeGateway {
    fn process_payment(&self, amount: u32) -> Result<(), AppError> {
        Ok(())
    }
}

// 3. Inject it via dynamic dispatch
#[derive(Clone)]
pub struct AppState {
    pub payment_gateway: Arc<dyn PaymentGateway>, // Can be Stripe, PayPal, or a Mock!
}
```

This is the cornerstone of Test-Driven Development (TDD) in Ferrox, allowing you to seamlessly swap real providers with Mocks during unit testing.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*