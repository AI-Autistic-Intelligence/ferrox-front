---
id: sentinel
title: "@node-yalc/security"
sidebar_position: 3
---

# 🤖 Ferrox Sentinel Security Engine (`@node-yalc/security`)

## 💡 1. What It Is & Architectural Purpose
The `@node-yalc/security` module contains the **Ferrox Sentinel Security Engine**. Unlike standard HTTP guards (which handle authorization and headers), Sentinel is an advanced, multi-layered security kernel designed for zero-trust environments. Its architectural purpose is to provide deep packet inspection, AI threat protection, and kernel-level sandboxing, intercepting highly sophisticated, non-deterministic attacks before they ever reach the application router.

---

## ⚙️ 2. Comprehensive Taxonomy & Key Features

| Engine Component | Description | Use Case |
| :--- | :--- | :--- |
| **`AiPromptGuardrailEngine`** | AI Prompt Injection detector. | Scanning incoming payloads to LLM endpoints to prevent jailbreaks. |
| **`RagHallucinationGroundednessEngine`** | AI Output Verifier. | Detecting and blocking hallucinations in RAG (Retrieval-Augmented Generation) pipelines. |
| **`ShannonEntropyEngine`** | Information Theory analyzer. | Detecting obfuscated SQL injections or encrypted malicious payloads using high-entropy heuristics. |
| **`PolymorphicRouteEngine`** | API Mutator. | Encrypting and mutating API routes dynamically to break automated scrapers and bots. |
| **`MarkovBehaviorEngine`** | Statistical behavior baselining. | Detecting anomalous sequences of API calls (e.g., an admin suddenly querying 1,000 users). |
| **`FerroxSentinelSecurityEngine`** | The central orchestrator. | Bootstrapping the sub-engines and generating OS-level security policies (`seccomp`, `sysctl`). |

---

## 🔬 3. How It Works Under the Hood

### Zero-Trust Sandboxing
Instead of just blocking requests at the Node.js layer, Sentinel generates OS-level security configurations that can be piped directly into Docker or Kubernetes during deployment.

For example, `generateSeccompBpfPolicy()` creates a strict allowlist of Linux Syscalls. 

```mermaid
flowchart TD
    Docker[Container Runtime]
    Node[Node.js Process]
    Seccomp[(Seccomp BPF Filter)]
    OS[Linux Kernel]

    Node -->|Attempts execve (RCE)| Docker
    Docker --> Seccomp
    Seccomp -- "SCMP_ACT_KILL (Blocked)" --> OS
    Note right of Seccomp: Sentinel Policy prevents Zero-Day RCE
```

---

## 🧠 4. Why It Was Designed This Way (Rationale)

Modern cloud-native applications face threats that simple JWT validation and Regex-based Web Application Firewalls (WAFs) cannot stop (e.g., LLM Jailbreaks or polymorphic malware). `@node-yalc/security` pushes the defensive perimeter as far out as possible—using statistical models (Markov Chains, Shannon Entropy) to catch obfuscated zero-day exploits, and generating native Linux kernel restrictions to mitigate remote code execution (RCE) at the OS layer, ensuring true Defense in Depth.

---

## 🚀 5. Practical Usage Guide & Extended Code Examples

### 1. Initializing the Sentinel Engine
```typescript
import { FerroxSentinelSecurityEngine } from '@node-yalc/security';

// Initialize with a strong cryptographic master key
const sentinel = new FerroxSentinelSecurityEngine(process.env.SENTINEL_MASTER_KEY);

// The sub-engines are now available for dependency injection or direct usage
const routeEngine = sentinel.routeEngine;
```

### 2. Generating OS-Level Hardening Configs for CI/CD
You can use the Sentinel engine within your CI/CD pipelines to dynamically generate hardened Docker security profiles based on the exact version of your code.

```typescript
import { FerroxSentinelSecurityEngine } from '@node-yalc/security';
import * as fs from 'fs';

const sentinel = new FerroxSentinelSecurityEngine();

// 1. Generate Seccomp Profile (for Docker --security-opt)
const seccompPolicy = sentinel.generateSeccompBpfPolicy();
fs.writeFileSync('./deploy/seccomp-profile.json', seccompPolicy);

// 2. Generate Sysctl config (for /etc/sysctl.conf in Kubernetes pods)
const sysctlConfig = sentinel.generateSysctlHardeningConfig();
fs.writeFileSync('./deploy/sysctl.conf', sysctlConfig);

console.log('Security profiles successfully generated and hardened.');
```

---

## ⚠️ 6. Anti-Patterns: How NOT to Use It

> [!CAUTION]
> **Anti-Pattern 1: Running AI Guardrails Synchronously on Heavy Traffic**
> Do not execute the `RagHallucinationGroundednessEngine` or complex Markov evaluations synchronously in the main HTTP request loop for every standard REST endpoint. These engines are computationally expensive. They should be offloaded to Worker Threads or only applied to specific, high-risk AI/Admin endpoints.

> [!WARNING]
> **Anti-Pattern 2: Applying Seccomp blindly in Dev Environments**
> If you apply the generated `generateSeccompBpfPolicy()` on a local developer's machine, tools like debuggers (which use `ptrace`) or hot-reloaders (which use `execve`) will immediately trigger a Kernel Kill (`SCMP_ACT_KILL`). These profiles are exclusively for highly locked-down production environments.

---

## 💡 7. Pro-Tips & Best Practices

> [!TIP]
> **Pro-Tip: Integrating Shannon Entropy**
> When accepting file uploads or large JSON payloads in your controllers, pipe the raw buffer through the `ShannonEntropyEngine`. If the entropy score is anomalously high (approaching 8.0), it strongly indicates the payload is encrypted or obfuscated—a common tactic used to bypass traditional antivirus scanners.
