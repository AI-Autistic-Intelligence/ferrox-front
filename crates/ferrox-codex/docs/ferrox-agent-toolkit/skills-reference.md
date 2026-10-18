---
title: Specialized Skills Catalog & Automation Scripts
sidebar_position: 11
description: Complete directory of all 19 skills provided by the Ferrox Agent Toolkit, including CLI scripts and verification tools.
keywords: [ferrox, skills, automation, scripts, coverage, entropy, diff, iac, codex]
---

# 🛠️ Specialized Skills Catalog & Automation Scripts

The **Ferrox Agent Toolkit** provides 19 executable skills. Each skill includes a structured `SKILL.md` instruction file and automated automation scripts that agents execute directly during development.

---

## 📂 Skills Directory

### High-Level Governance Skills (Layer 1)

| Skill Name | Path | Purpose & Execution Output |
| :--- | :--- | :--- |
| **`ferrox-scrum-master`** | `skills/ferrox-scrum-master` | Decomposes briefs into Epics, Stories, and Tasks; produces `AGILE_SPRINT_BACKLOG.md`. |
| **`ferrox-forge-lifecycle`** | `skills/ferrox-forge-lifecycle` | Automates remote forge issues, branches, and PRs via `gh`/`glab`. |
| **`ferrox-dep-resolver`** | `skills/ferrox-dep-resolver` | Enforces Ferrox-First dependencies and generates `DEPENDENCY_MATRIX.md`. |
| **`ferrox-ddd-planner`** | `skills/ferrox-ddd-planner` | Models pure aggregates, invariants, and CQRS handlers in `DDD_PLAN.md`. |
| **`ferrox-tdd-orchestrator`** | `skills/ferrox-tdd-orchestrator` | Orchestrates Red-Green-Refactor cycles and verifies 100% test coverage. |
| **`ferrox-e2e-scaffolder`** | `skills/ferrox-e2e-scaffolder` | Scaffolds chaos, double-spend, and Saga tests; outputs `E2E_VERIFICATION.md`. |
| **`ferrox-security-auditor`** | `skills/ferrox-security-auditor` | Runs OWASP WSTG & Shannon entropy audit; outputs `SECURITY_AUDIT_REPORT.md`. |
| **`ferrox-devops-architect`** | `skills/ferrox-devops-architect` | Generates distroless Dockerfiles, docker-compose, and Terraform IaC manifests. |
| **`ferrox-self-reviewer`** | `skills/ferrox-self-reviewer` | Audits git diff against master, strips debug debris, outputs `SELF_REVIEW_REPORT.md`. |
| **`ferrox-showcase-generator`** | `skills/ferrox-showcase-generator` | Builds live Wasm showcases and interactive docs for `ferrox-codex`. |
| **`ferrox-code-reviewer`** | `skills/ferrox-code-reviewer` | Performs multi-dimensional code reviews across DDD purity, TDD, and security. |
| **`ferrox-workflow-handoff`** | `skills/ferrox-workflow-handoff` | Consolidates DoD artifacts, test metrics, and release notes into handoff summaries. |

### Low-Level Polyglot Implementation Skills (Layer 2)

| Skill Name | Path | Target Runtime & Capabilities |
| :--- | :--- | :--- |
| **`ferrox-rust-implementer`** | `skills/ferrox-rust-implementer` | Tokio async services, lock-free concurrency, zero-alloc `bytes::Bytes`, `cargo llvm-cov`. |
| **`ferrox-wasm-builder`** | `skills/ferrox-wasm-builder` | Reactive Signal components, 60 FPS SVG rendering, trunk build, and `wasm-opt`. |
| **`ferrox-nestjs-implementer`** | `skills/ferrox-nestjs-implementer` | Enterprise TypeScript NestJS services conforming to YALC CQRS and PASETO guards. |
| **`ferrox-python-implementer`** | `skills/ferrox-python-implementer` | High-performance AsyncIO CQRS handlers, frozen Pydantic v2 models, PyO3 native extensions. |
| **`ferrox-php-implementer`** | `skills/ferrox-php-implementer` | Modern PHP 8.3/8.4 services, distributed Saga coordinators, PHPStan Level 9, Infection. |
| **`ferrox-java-implementer`** | `skills/ferrox-java-implementer` | Java 21+ Project Loom virtual threads, Project Panama native Rust FFI, JaCoCo 100%. |
| **`ferrox-polyglot-connector`** | `skills/ferrox-polyglot-connector` | Cross-language connectors: zero-copy FFI, gRPC/Protobuf, and CloudEvents message queues. |

---

## 💻 Included Automation Scripts

The toolkit bundles several standalone Python verification scripts located in `scripts/`:

### 1. `scripts/check_coverage.py`
Parses LCOV, Clover, and JaCoCo coverage reports, enforcing the strict 100% line and branch coverage threshold:
```bash
python scripts/check_coverage.py --threshold 100 --report-path ./coverage/lcov.info
```

### 2. `scripts/run_wstg_security_scanner.py`
Executes Shannon entropy analysis across all modified files, detecting potential API keys, unmasked tokens, or insecure configurations:
```bash
python scripts/run_wstg_security_scanner.py --max-entropy 4.8 --target-dir ./src
```

### 3. `skills/ferrox-self-reviewer/scripts/scan_diff_debris.py`
Inspects git diffs against the base branch (`origin/master...HEAD`) to detect forbidden debugging artifacts:
```bash
python skills/ferrox-self-reviewer/scripts/scan_diff_debris.py --base origin/master
```
*(Flags: `dbg!`, `console.log`, `print`, `var_dump`, `echo`, `System.out.println`).*
