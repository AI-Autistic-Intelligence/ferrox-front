---
sidebar_position: 3
---

# 🔗 Middlewares

Middleware is a function which is called **before** the route handler. Middleware functions have access to the request and response objects, and the `next()` middleware function in the application’s request-response cycle.

Ferrox middlewares are equivalent to Express/NestJS middlewares but execute with zero-cost abstractions thanks to `tower::Service`.

Middleware functions can perform the following tasks:
- Execute any code.
- Make changes to the request and the response objects (e.g. injecting Headers or Extensions).
- End the request-response cycle early (e.g. returning 401 Unauthorized before hitting the DB).
- Call the next middleware function in the stack.

## Functional Middleware

The simplest way to define a middleware in Ferrox is by using a standard asynchronous function.

```rust
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};

// 1. Define the Middleware function
pub async fn logger_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    println!("👉 Incoming Request: {} {}", method, uri);

    // Call the next middleware (or the controller)
    let response = next.run(req).await;

    println!("👈 Outgoing Response: {}", response.status());

    response
}
```

### Applying Middleware

You can apply middleware to specific routes, or globally to the entire application.

```rust
use axum::{routing::get, Router, middleware};

pub fn app_router() -> Router {
    Router::new()
        // The logger_middleware will only apply to routes ABOVE this layer
        .route("/cats", get(get_cats))
        .route_layer(middleware::from_fn(logger_middleware))
        
        // It will NOT apply to this route! (Order matters in Axum)
        .route("/health", get(health_check))
}
```

> [!WARNING]
> In Ferrox/Axum, middleware layers are evaluated **bottom-to-top** when applied via `.layer()`, but the routing matches top-to-bottom. Always be mindful of where you place `.route_layer()` to ensure it only protects the routes you intend it to!

## Class Middleware (Tower Services)

For highly complex enterprise middlewares that require their own internal state (e.g., Rate Limiters, Circuit Breakers, Request Batchers), functional middlewares might not be enough.

You can implement the `tower::Service` trait directly. This is the equivalent of a `class` Middleware in NestJS.

```rust
use tower::{Service, Layer};
use std::task::{Context, Poll};
use futures::future::BoxFuture;

// The Middleware "Class"
#[derive(Clone)]
pub struct RateLimiterMiddleware<S> {
    inner: S,
    max_requests: u32,
}

impl<S, ReqBody, ResBody> Service<axum::http::Request<ReqBody>> for RateLimiterMiddleware<S>
where
    S: Service<axum::http::Request<ReqBody>, Response = axum::http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::http::Request<ReqBody>) -> Self::Future {
        // Complex stateful logic here...
        
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        
        Box::pin(async move {
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
```

## Passing Data to Controllers

A very common use case for Middleware is extracting a JWT token, looking up the User ID, and passing it to the Controller so the Controller doesn't have to duplicate the logic.

In Ferrox, you use `Extensions` to mutate the request context.

```rust
pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    
    // 1. Verify token...
    let user_id = "user-uuid-123".to_string(); 
    
    // 2. Attach data to the request extensions
    req.extensions_mut().insert(user_id);
    
    // 3. Continue execution
    Ok(next.run(req).await)
}
```

Inside your Controller, you simply extract it!

```rust
use axum::extract::Extension;

async fn profile(
    Extension(user_id): Extension<String>, // Extracts the data injected by the middleware
) -> String {
    format!("Hello, user {}", user_id)
}
```
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*