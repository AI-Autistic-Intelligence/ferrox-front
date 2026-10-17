# Schema Registry

## 1. Overview (What does this do?)
The `SchemaRegistry` module guarantees data quality during ETL processes. It provides a centralized dictionary that maps string identifiers to Pydantic models (or Dataclasses), allowing the system to perform rigorous data casting and validation on raw, untyped datasets.

## 2. Philosophy (Why does it exist?)
When ingesting data from external sources (like a third-party CSV or a public JSON API), the data is inherently untrusted and untyped. Directly inserting this raw data into the Database (the Bronze/Silver/Gold layers) leads to corrupted records and fatal crashes. This module enforces the philosophy that data must be formally validated at the application boundary. If a CSV row says an ID is `"123"`, the Schema Registry mathematically proves and casts it to an `int(123)` before it is allowed further into the pipeline.

## 3. Target Audience (Who is it for?)
This component is vital for Data Engineers who manage data lakes or data warehouses, and backend developers who need to ensure that bulk data imports adhere strictly to the internal domain models of the application.

## 4. Architecture (How does it work?)
- **Registration**: At application boot, developers map string names (e.g., `"user_import"`) to corresponding Pydantic schemas.
- **Validation**: During a pipeline execution, a transformation step can invoke `SchemaRegistry.validate("user_import", raw_dict)`. The registry locates the schema and processes the dictionary.
- **Error Handling**: If the raw data violates the schema (e.g., a missing required field or wrong data type), Pydantic raises a detailed validation error, allowing the orchestrator to log the specific malformed row and continue or halt appropriately.

## 5. Installation / Setup
The Schema Registry relies heavily on the `pydantic` library to perform its fast, strict type checking.

```bash
pip install pydantic
```

## 6. Quickstart (Usage)
```python
from ferrox_py_utils.schemas.registry import SchemaRegistry
from pydantic import BaseModel

# 1. Define the strict data contract
class UserImportSchema(BaseModel):
    id: int
    email: str

# 2. Register the schema centrally
registry = SchemaRegistry()
registry.register("user_import", UserImportSchema)

# 3. Automatic casting and validation inside a Pipeline step
raw_data = {"id": "123", "email": "test@test.com"}
valid_user = registry.validate("user_import", raw_data)

# The string "123" has been safely cast to an integer
print(type(valid_user.id))  # <class 'int'>
```

## 7. Ecosystem Integration
The Schema Registry serves the exact same purpose in the Data Engineering layer as the **Validation Pipe** (Layer 5) serves in the Web/HTTP layer of the core `ferrox-py` framework. It ensures that the **CQRS Bus** and **Data Repositories** only ever interact with strongly-typed, predictable Domain Objects.
