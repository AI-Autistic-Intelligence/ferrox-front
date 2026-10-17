---
id: datagrid
title: "@node-yalc/datagrid"
sidebar_position: 6
---

# 📊 DataGrid & API Decorators (`@node-yalc/datagrid`)

## 💡 1. What It Is & Architectural Purpose
The `@node-yalc/datagrid` module provides the foundational interfaces, types, and decorators required to build massive, highly scalable data tables (DataGrids) across the ecosystem. It acts as the bridging contract between frontend data requests (filtering, sorting, pagination) and backend database execution. 

Crucially, it also provides the `@YalcAgGridObject` and `@YalcAgGridField` decorators that allow developers to map GraphQL or REST payloads directly into complex SQL joins and database columns without writing boilerplate logic.

---

## ⚙️ 2. Comprehensive Taxonomy & Key Features

| Utility | Description | Use Case |
| :--- | :--- | :--- |
| **`DataGridRequest` & `DataGridResponse`** | Standardized payload contracts. | Used in DTOs (Data Transfer Objects) to enforce a unified pagination structure across all microservices. |
| **`FerroxDataGridEngine`** | In-memory processing engine. | Used for sorting, filtering, and paginating arrays (e.g., cached Redis payloads or mocked datasets). |
| **`FerroxCrudGenerator`** | Dynamic CRUD route generator. | Scaffolding standardized REST APIs (`GET`, `POST`, `DELETE`) dynamically. |
| **`@YalcAgGridField`** | Property Decorator for ORM mapping. | Decorating DTO fields to instruct the backend how to query the corresponding database columns (`regular`, `derived`, `virtual`). |
| **`@YalcAgGridObject`** | Class Decorator for Security. | Establishing field-level inclusions/exclusions for DataGrid requests. |

---

## 🔬 3. How It Works Under the Hood

### The AgGrid Decorator Metadata Flow

When a class property is decorated with `@YalcAgGridField`, the engine leverages `reflect-metadata` to register how the frontend requests map to the backend datasource.

```mermaid
flowchart TD
    Request[Frontend Requests 'userName' sort]
    DTO[@YalcAgGridField decorated DTO]
    Metadata[Reflect Metadata Extraction]
    SQL[(Database / TypeORM Query Builder)]

    Request --> DTO
    DTO --> Metadata
    Metadata -->|Resolves 'userName' to 'users.full_name'| SQL
```

### In-Memory Pagination Engine
When `FerroxDataGridEngine.paginate()` is called, it strictly executes operations in this order:
1. **Filtering**: Applies case-insensitive string matching across all object properties if a `searchQuery` is provided.
2. **Sorting**: Sorts based on `sortField` and `sortOrder` (ASC/DESC).
3. **Pagination**: Slices the array based on `(page - 1) * pageSize`.

---

## 🧠 4. Why It Was Designed This Way (Rationale)

Building tables with pagination, sorting, and filtering is notoriously repetitive. By abstracting the `DataGridRequest` and creating metadata decorators, the Ferrox ecosystem ensures that developers do not have to write custom SQL `ORDER BY` and `WHERE` clauses for every single endpoint. Instead, the DTO intrinsically describes how to query the database, enforcing DRY (Don't Repeat Yourself) principles across the entire monorepo.

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

### 1. Mapping a DTO for DataGrid Execution

By utilizing `@YalcAgGridField`, you map a frontend facing property to an internal representation.

```typescript
import { YalcAgGridObject, YalcAgGridField, FilterOptionType } from '@node-yalc/datagrid';

@YalcAgGridObject({
  filters: { type: FilterOptionType.INCLUDE, fields: ['id', 'email', 'status'] }
})
export class UserGridDto {
  
  @YalcAgGridField({ dst: 'users.id' })
  id: string;

  @YalcAgGridField({ dst: 'users.email' })
  email: string;

  // 'mode: derived' signifies this isn't a direct column, but requires computation
  @YalcAgGridField({ 
      mode: 'derived',
      dst: 'UPPER(users.status)' 
  })
  status: string;
}
```

### 2. Fast In-Memory Mocking (DataGrid Engine)

For small datasets or mock APIs, use the in-memory engine to simulate database behavior:

```typescript
import { FerroxDataGridEngine, DataGridRequest } from '@node-yalc/datagrid';

const mockUsers = [
    { id: 1, name: 'Alice' },
    { id: 2, name: 'Bob' },
    { id: 3, name: 'Charlie' }
];

const request: DataGridRequest = { page: 1, pageSize: 2, sortField: 'name', sortOrder: 'DESC' };

const response = FerroxDataGridEngine.paginate(mockUsers, request);

console.log(response);
/*
{
  data: [ { id: 3, name: 'Charlie' }, { id: 2, name: 'Bob' } ],
  total: 3,
  page: 1,
  pageSize: 2,
  totalPages: 2
}
*/
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

> [!CAUTION]
> **Anti-Pattern 1: In-Memory Pagination on Massive Datasets**
> Do not fetch 1,000,000 rows from the database and pass them to `FerroxDataGridEngine.paginate()`. This will crash the Node.js V8 Heap. The in-memory engine is *only* for cached chunks or small bounded arrays. Massive grids must always push the `DataGridRequest` into the SQL engine using TypeORM integrations.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Pro-Tip: Metadata Copying**
> When defining variations of a DTO (e.g., `UserResponse` vs `DetailedUserResponse`), you can easily clone the grid configurations using `@YalcAgGridObject({ copyFrom: BaseUserDto })` to inherit all field mappings without copy-pasting code.
