---
title: Polyglot Runtimes & Zero-Copy Connectors
sidebar_position: 12
description: Architecture of cross-runtime communication, Project Panama, PyO3, Node-API, and high-performance polyglot connectors.
keywords: [ferrox, polyglot, ffi, panama, pyo3, node-api, rust, wasm, nestjs, python, php, java]
---

# 🌐 Polyglot Runtimes & Zero-Copy Connectors

The Ferrox ecosystem spans six major runtime environments. The `ferrox-polyglot-connector` skill and `rules/ferrox-interop-connectors.md` govern how these distinct environments communicate with minimal memory copying and sub-microsecond latency.

---

## 🔌 Cross-Runtime Connector Matrix

| Runtime Pair | Connector Mechanism | Memory Transfer Model | Typical Latency |
| :--- | :--- | :--- | :--- |
| **Rust $\leftrightarrow$ Rust Wasm** | Direct compilation / Shared types | In-process WebAssembly memory | $< 0.05\mu\text{s}$ |
| **Rust $\leftrightarrow$ TypeScript (Node)** | Node-API (`napi-rs`) | Zero-copy buffer pointer sharing | $< 0.2\mu\text{s}$ |
| **Rust $\leftrightarrow$ Python** | PyO3 native C-extension | Borrowed Python buffer protocol | $< 0.3\mu\text{s}$ |
| **Rust $\leftrightarrow$ Java 21+** | Project Panama Off-Heap FFI | `Arena` / `MemorySegment` shared memory | $< 0.1\mu\text{s}$ |
| **Rust $\leftrightarrow$ PHP 8.4** | Modern C FFI (`FFI::cdef`) | C struct pointer passing | $< 0.5\mu\text{s}$ |
| **Distributed Services** | gRPC (HTTP/2) + CloudEvents | Protobuf binary serialization | $< 1.5\text{ms}$ |

---

## ☕ Java 21 Project Panama Integration

Java services connect directly to native Rust shared libraries without legacy JNI boilerplate:

```java
// Java 21 Project Panama zero-copy off-heap memory call
try (Arena arena = Arena.ofConfined()) {
    MemorySegment nativePayload = arena.allocateFrom("Ferrox Transaction Payload");
    MethodHandle nativeProcess = Linker.nativeLinker().downcallHandle(
        symbolLookup.find("ferrox_process_payload").orElseThrow(),
        FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG)
    );
    long status = (long) nativeProcess.invokeExact(nativePayload, nativePayload.byteSize());
}
```

---

## 🐍 Python PyO3 Integration

Python services invoke native Rust modules compiled as Python extensions:

```rust
// Native PyO3 Rust extension module
use pyo3::prelude::*;

#[pyfunction]
fn compute_entropy(data: &[u8]) -> PyResult<f64> {
    // Zero-alloc Shannon entropy computation in pure Rust
    Ok(ferrox_security::entropy::calculate(data))
}

#[pymodule]
fn ferrox_py_native(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_entropy, m)?)?;
    Ok(())
}
```
