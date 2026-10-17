---
id: config
title: "@node-yalc/config"
sidebar_position: 5
---

# ⚙️ Configuration Management (`@node-yalc/config`)

## 💡 1. What It Is & Architectural Purpose
The `@node-yalc/config` module provides a centralized, robust `ConfigEngine` for managing application configuration. Its primary architectural purpose is to establish a strict hierarchy of configuration retrieval (Environment Variables > In-Memory Store > Default Fallbacks) while automatically casting string-based environment variables into their native TypeScript types (booleans, numbers) without external bloated dependencies.

---

## ⚙️ 2. Comprehensive Taxonomy & Key Features

| Utility | Description | Use Case |
| :--- | :--- | :--- |
| **`ConfigEngine`** | The core configuration engine class. | Instantiated at application bootstrap to manage the global or module-scoped configuration state. |
| **`.get<T>(key, fallback)`** | Type-safe retrieval mechanism. | Extracting `PORT`, `DEBUG_MODE`, or feature flags with automatic type casting. |
| **`.set(key, value)`** | In-memory configuration override. | Changing configuration dynamically at runtime (without mutating `process.env`). |

---

## 🔬 3. How It Works Under the Hood

The `ConfigEngine` evaluates configuration keys in a strict, descending order of precedence:

```mermaid
flowchart TD
    Request[config.get('IS_ACTIVE')]
    ENV{1. process.env['IS_ACTIVE']}
    Store{2. In-Memory Store}
    Fallback{3. Fallback Argument}

    Request --> ENV
    ENV -- Found --> Cast[Parse 'true'/'false'/numeric]
    Cast --> Return(Return Value)
    
    ENV -- Not Found --> Store
    Store -- Found --> Return
    
    Store -- Not Found --> Fallback
    Fallback --> Return
```

When an environment variable is found, the engine inherently understands that `process.env` only stores strings. It proactively casts `"true"` to `true`, `"false"` to `false`, and numeric strings (e.g. `"8080"`) to numbers before returning the value to the caller.

---

## 🧠 4. Why It Was Designed This Way (Rationale)

Most Node.js applications rely on scattered `process.env.MY_VAR || 'default'` calls. This creates two problems:
1. **Type Pollution**: Environment variables are always strings, leading to bugs when evaluating `if (process.env.FEATURE === 'false')` (which evaluates to true since it's a non-empty string).
2. **Testing Fragility**: Mutating `process.env` during test suites is a notorious source of flaky tests due to global state pollution.

`ConfigEngine` solves this by handling the type coercion internally and providing an isolated `.set()` method for overriding config in memory, heavily stabilizing the testing environment without touching the global `process.env` state.

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

### Initialization and Basic Retrieval

```typescript
import { ConfigEngine } from '@node-yalc/config';

// Initialize with some default overrides
const config = new ConfigEngine({
  DB_TIMEOUT: 5000,
  ENABLE_ANALYTICS: true
});

// Environment variable: "PORT=3000"
// Auto-casted to a Number
const port = config.get<number>('PORT', 8080); 

// Environment variable: "ENABLE_CACHE=false"
// Auto-casted to a Boolean
const cacheEnabled = config.get<boolean>('ENABLE_CACHE', true);

if (cacheEnabled) {
    // ...
}
```

### Isolated Overrides during Testing

```typescript
import { ConfigEngine } from '@node-yalc/config';
import { myService } from './my-service';

describe('MyService', () => {
  it('should behave differently when mock mode is enabled', () => {
    const testConfig = new ConfigEngine();
    
    // Override internally without polluting process.env
    testConfig.set('MOCK_MODE', true);
    
    const result = myService.run(testConfig);
    expect(result).toBe('mocked');
  });
});
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

> [!WARNING]
> **Anti-Pattern 1: Leaking Engine Instances**
> Do not instantiate `new ConfigEngine()` deeply inside functions or loops. Create a single instance at the module or application root and inject/export it to maintain a consistent in-memory configuration state.

> [!CAUTION]
> **Anti-Pattern 2: Mutating process.env Directly**
> If you need to change a configuration variable at runtime, use `config.set('KEY', 'VALUE')`. Do not execute `process.env.KEY = 'VALUE'` at runtime, as this can trigger performance penalties in Node.js worker threads and create race conditions.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Pro-Tip: Typings with Interfaces**
> When retrieving highly nested or specific configuration interfaces, pass the interface type to `.get<MyConfigInterface>('APP_CONFIG')` to ensure your editor retains full intellisense auto-completion on the returned object from the memory store.
