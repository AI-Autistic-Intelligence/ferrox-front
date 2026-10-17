---
sidebar_position: 3
---

# 🤖 Part 3: The Code Factory

Our Backend is solid, but writing the exact same interfaces and API calls manually in TypeScript for the Frontend is an anti-pattern that leads to massive synchronization bugs.

Let's use the **Ferrox Code Factory** to generate our entire API Client with zero dependencies.

## 1. Extracting the Types

In Chapter 2, we annotated our Rust DTOs with `#[derive(TS)]` and `#[ts(export)]`.

When you compile the project, the `ts-rs` library automatically generates the exact TypeScript representations. 

Run:
```bash
cargo test
```

This generates `bindings/CreateTicketDto.ts` and `bindings/Ticket.ts`.

```typescript
// bindings/CreateTicketDto.ts
export interface CreateTicketDto {
    title: string;
}
```

## 2. Generating the API Client

Now, we use the Ferrox CLI to generate the networking layer.

```bash
ferrox generate --lang ts --output ./frontend/api
```

This scans your Rust routes and generates a unified `FerroxClient.ts` that uses the native JavaScript `fetch` API. It automatically handles serialization and JWT injection.

## 3. Using it in React / Next.js

Your frontend developers never have to write an Axios call again. They simply instantiate the client.

```tsx
import { FerroxClient } from './api/FerroxClient';
import { CreateTicketDto } from './api/CreateTicketDto';

// The client automatically injects the Bearer Token from local storage
const client = new FerroxClient("http://localhost:3000", localStorage.getItem("token"));

export default function TicketForm() {
    const handleSubmit = async () => {
        const payload: CreateTicketDto = { title: "Fix the server!" };
        
        // This is 100% type-safe!
        // If the backend changed the payload, this would throw a TS error.
        const ticket = await client.request("/api/tickets", {
            method: "POST",
            body: JSON.stringify(payload)
        });
        
        console.log("Ticket created!", ticket.id);
    }
    
    return <button onClick={handleSubmit}>Create Ticket</button>
}
```

## Conclusion

Congratulations! You have built a highly scalable, secure, and type-safe full-stack feature using Ferrox.

You experienced:
- **Thin Controllers** passing requests to **Fat Providers**.
- **Asynchronous Mutex Locking** to avoid blocking Tokio threads.
- **Zero-Trust Security** at the routing layer.
- **Automated Frontend Generation** via the Code Factory.
\n\n---\n\n## 1. Philosophy / Purpose\n\n*Document the philosophy and core purpose of this component here.*\n\n## 2. Architectural Layering\n\n*Explain where this component sits within the Onion Architecture (e.g., Domain, Application, Infrastructure, or Presentation).* \n\n## 3. How it Works (Under the hood)\n\n*Detail the internal mechanics, memory model, and execution flow.*\n\n## 4. Why it was designed this way\n\n*Explain the historical context, trade-offs, and design rationale.*\n\n## 5. Usage Guide & Code Examples\n\n*Provide integration examples, setup guides, and typical use cases.*\n\n## 6. Anti-Patterns\n\n*List common mistakes, misconfigurations, and patterns to avoid when using this component.*\n\n## 7. Pro-Tips / Best Practices\n\n*Provide advanced tips, performance optimizations, and recommended patterns.*