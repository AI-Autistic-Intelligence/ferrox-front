# Pipelines

## 1. Overview (What does this do?)
The `PipelineOrchestrator` is the execution engine at the heart of the `ferrox-py-utils` data engineering toolkit. It provides a linear, procedural runner that executes a chain of responsibility (a sequence of isolated tasks or "steps") to process data safely and predictably.

## 2. Philosophy (Why does it exist?)
When data transformations are written as one large function, handling specific errors (like a network timeout on step 4 of 10) is a nightmare. The Orchestrator exists to enforce the single-responsibility principle. By breaking an ETL job down into small, functional steps that only mutate a shared `context` dictionary, developers can easily unit test individual transformations, insert retry logic automatically, and halt the pipeline gracefully upon failure.

## 3. Target Audience (Who is it for?)
This module is for Data Engineers who are writing complex ETL or ELT jobs, data migration scripts, or nightly batch processes that require clear execution boundaries, error isolation, and detailed logging at each stage.

## 4. Architecture (How does it work?)
- **Shared Context**: The orchestrator initializes a `context` (usually a dictionary) that is passed sequentially from one step to the next.
- **Adding Steps**: Tasks are registered in the orchestrator via `add_step(name, function)`. The function must accept the `context` as an argument and return it.
- **Execution Engine**: The `execute()` method runs the steps in order. Because it manages the execution loop, it can natively implement advanced features like automated retry policies, step-timing metrics, or alert dispatching if a specific node crashes.

## 5. Installation / Setup
The `PipelineOrchestrator` is natively included in the `ferrox-py-utils` package. It uses standard Python synchronous or asynchronous mechanics and requires no external dependencies.

## 6. Quickstart (Usage)
```python
from ferrox_py_utils.pipelines.orchestrator import PipelineOrchestrator

# 1. Define isolated, pure functions (Steps)
def extract_data(ctx):
    ctx['raw_data'] = [1, 2, 3]
    return ctx

def transform_data(ctx):
    # Mutates the shared context
    ctx['transformed'] = [x * 10 for x in ctx['raw_data']]
    return ctx

# 2. Build the Pipeline
pipeline = PipelineOrchestrator()
pipeline.add_step("Extraction Step", extract_data)
pipeline.add_step("Transformation Step", transform_data)

# 3. Execute the chain
final_context = pipeline.execute()
assert final_context['transformed'] == [10, 20, 30]
```

## 7. Ecosystem Integration
The `PipelineOrchestrator` relies heavily on the **Observability Component** from the core `ferrox-py` framework. Because the orchestrator controls the execution loop, it automatically injects logging trace IDs and emits structured logs (e.g., `Step 'Extraction Step' completed in 0.4s`) into the centralized logging system, ensuring complete visibility over background data jobs.
