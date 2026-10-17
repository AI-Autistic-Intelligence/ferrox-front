# Ferrox-Py-Utils Overview

## 1. Overview (What does this do?)
The `ferrox-py-utils` package implements a decoupled data architecture based on interchangeable connectors and a linear execution pipeline. It provides the scaffolding required to build robust Data Platforms entirely within the Ferrox ecosystem.

## 2. Philosophy (Why does it exist?)
Traditional ETL (Extract, Transform, Load) scripts are often written as massive, procedural files (e.g., `import_users.py`). These scripts are incredibly fragile, hard to test, and impossible to reuse. This package mandates a functional, "matryoshka" approach: extraction is isolated to Connectors, transformation is isolated to pure functions (Steps), and execution is managed by an Orchestrator. This ensures that a bug in one transformation step doesn't crash the entire data ingestion process without proper logging.

## 3. Target Audience (Who is it for?)
This overview is for Software Architects and Data Engineers who are designing the data ingestion, processing, and warehousing layers of their application and need a structured, maintainable framework rather than a collection of disorganized Python scripts.

## 4. Architecture (How does it work?)
The data engineering process is strictly divided into three phases:
1. **Isolated Extraction (I/O)**: Data is read from external sources (S3, Databases, APIs) exclusively through specialized *Connectors*.
2. **In-Memory Transformation**: A *PipelineOrchestrator* executes a sequence of functional steps, mutating a shared state object (`context`) in a predictable order.
3. **Guaranteed Validation**: The *Schema Registry* ensures that the data being transformed strictly adheres to predefined contracts (DTOs) before it is passed to the next step or loaded into the final Database (the Bronze/Silver/Gold layers).

## 5. Installation / Setup
The package is installed via standard pip mechanics. It relies heavily on the core `ferrox-py` framework for architectural consistency.

```bash
pip install ferrox-py-utils
```

## 6. Quickstart (Usage)
```python
from ferrox_py.core.container import Container
from ferrox_py_utils.connectors.s3 import S3Connector

# Centralized setup of data dependencies
container = Container()
container.register("s3_connector", S3Connector(
    endpoint_url="https://s3.amazonaws.com",
    access_key="...",
    secret_key="...",
    bucket_name="my-data-lake"
))

# The connector is now available globally for any Pipeline to use
```

## 7. Ecosystem Integration
The entire philosophy of this package is built upon the **Inversion of Control (IoC) Container** provided by the `ferrox-py` core. Connectors and Pipeline steps are resolved dynamically via Dependency Injection, ensuring that unit testing an ETL pipeline doesn't require spinning up an actual AWS environment.
