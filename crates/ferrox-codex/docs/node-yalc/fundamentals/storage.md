---
id: storage
title: "@node-yalc/storage"
sidebar_position: 9
---

# 🗄️ Universal Storage Engine (`@node-yalc/storage`)

## 💡 1. What It Is & Architectural Purpose
The `@node-yalc/storage` module provides a universal, abstraction-driven wrapper around file storage (S3, GCS, Local Disk). Its architectural purpose is to strictly decouple the business logic of uploading and downloading files from the underlying cloud provider's SDK (e.g., `aws-sdk`).

---

## ⚙️ 2. Comprehensive Taxonomy & Key Features

| Component | Description | Use Case |
| :--- | :--- | :--- |
| **`IStorageAdapter`** | The strict interface contract. | Dictates that all storage providers must implement `upload`, `download`, `delete`, and `getPresignedUrl`. |
| **`MemoryStorageAdapter`** | In-memory `Map`-based adapter. | Used natively during test suites or fast local development so developers do not need AWS credentials. |
| **`StorageEngine`** | The dependency-injected orchestration class. | Exposes the universal API to the rest of the application. |

---

## 🔬 3. How It Works Under the Hood

When a file upload request arrives, the `StorageEngine` defers the byte writing to whichever `IStorageAdapter` was injected at bootstrap.

```mermaid
flowchart TD
    Controller[Route Controller]
    Engine[StorageEngine]
    
    Controller -->|upload('avatar.png', buffer)| Engine
    
    Engine -.->|Injected: MemoryAdapter| MemMap[(RAM Map)]
    Engine -.->|Injected: S3Adapter| AWS[(AWS S3 Bucket)]
    Engine -.->|Injected: GcpAdapter| GCP[(Google Cloud Storage)]
```

This prevents the notorious "Vendor Lock-in". If a startup migrates from AWS to Google Cloud, they only need to write one `GcpStorageAdapter` rather than hunting down every single `s3.upload()` call in their codebase.

---

## 🧠 4. Why It Was Designed This Way (Rationale)

Most developers `npm install aws-sdk` directly into their User Service to upload avatars. This is a severe architectural anti-pattern. 
1. It breaks the **Dependency Inversion Principle (DIP)**.
2. It makes the User Service completely untestable in CI pipelines (as it attempts to connect to real AWS buckets).

The `StorageEngine` solves this by forcing all byte transfers through the `IStorageAdapter` contract, ensuring perfect testability via the `MemoryStorageAdapter`.

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

### 1. Bootstrapping with Adapters

```typescript
import { StorageEngine, MemoryStorageAdapter } from '@node-yalc/storage';
// import { S3StorageAdapter } from '@ferrox/aws-helpers';

// In Testing/Local
const engine = new StorageEngine(new MemoryStorageAdapter());

// In Production
// const engine = new StorageEngine(new S3StorageAdapter({ bucket: 'user-avatars' }));
```

### 2. Standard Upload & Pre-Signed URLs

Instead of streaming the image bytes directly through the Node.js server (which consumes heavy RAM), the best practice is to generate a pre-signed URL and let the frontend upload directly to the storage provider.

```typescript
// 1. Controller generates an upload URL
export async function getUploadUrl(req, res) {
  const url = await engine.getPresignedUrl(`avatars/${req.user.id}.png`, 3600);
  res.json({ uploadUrl: url });
}

// 2. Or, if downloading directly through the API
export async function downloadFile(req, res) {
  const buffer = await engine.download(req.params.key);
  res.setHeader('Content-Type', 'application/octet-stream');
  res.end(buffer);
}
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

> [!CAUTION]
> **Anti-Pattern 1: Buffering Huge Files in RAM**
> The `IStorageAdapter` interface accepts `Buffer | string`. Do not read a 4GB ISO file into a `Buffer` and pass it to `engine.upload()`. Node's V8 heap maxes out at ~1.5GB by default and will crash. For massive files, utilize the `getPresignedUrl` flow to bypass Node.js entirely, or extend the interface to support Node `ReadStream` chunks.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Pro-Tip: Pre-Signed URLs for Private Assets**
> Always keep your Cloud Buckets strictly private. If a user needs to view a PDF receipt, use `getPresignedUrl('receipt.pdf', 300)` to generate a URL that naturally expires in 5 minutes, preventing authorized users from sharing permanent links to private data.
