---
title: Installation, Workspace Setup & Validation
sidebar_position: 13
description: Guide for installing the Ferrox Agent Toolkit globally or linking it locally to workspace roots, running verification scripts, and CI setup.
keywords: [ferrox, install, powershell, bash, setup, verification, antigravity]
---

# 🚀 Installation, Workspace Setup & Validation

The **Ferrox Agent Toolkit** is designed to install with zero friction on Windows, Linux, and macOS, supporting both global AI agent environments (like Antigravity IDE) and per-workspace development trees.

---

## 🛠️ Automated Installation Modes

The installer supports three target modes:
- **`all` (Default)**: Installs both globally to `~/.gemini/config/plugins/ferrox-agent-toolkit` and links locally to the workspace root `.agents/`.
- **`workspace`**: Only configures the local development workspace (`.agents/` and `AGENTS.md`).
- **`antigravity`**: Only updates the global Antigravity plugin directory.

### Windows (PowerShell)

```powershell
# Full installation into Antigravity IDE and workspace root
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1 -Target all -WorkspaceDir "C:\Users\nn\Desktop\code"
```

```powershell
# Only update workspace junction links
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1 -Target workspace
```

### Linux & macOS (Bash)

```bash
# Full installation
chmod +x ./scripts/install.sh
./scripts/install.sh all "/path/to/your/workspace"
```

---

## 🔗 NTFS Junctions & Live Synchronization

When installed to a workspace, the installer creates **NTFS Junctions** (Windows) or **Symbolic Links** (POSIX):

```text
C:\Users\nn\Desktop\code\
  ├── .agents\
  │   ├── rules\   <=== Junction ===> C:\Users\nn\Desktop\code\ferrox-agent-toolkit\rules\
  │   ├── skills\  <=== Junction ===> C:\Users\nn\Desktop\code\ferrox-agent-toolkit\skills\
  │   └── agents\  <=== Junction ===> C:\Users\nn\Desktop\code\ferrox-agent-toolkit\agents\
  ├── AGENTS.md    (Copied root operating guide)
  ├── ferrox\
  ├── ferrox-front\
  ├── ferrox-docs\
  └── ...
```

### Advantages of Junction Links:
1. **Zero Drift**: Editing a rule or agent in `ferrox-agent-toolkit` instantly updates all repositories in the workspace without re-running installers.
2. **Instant Inheritance**: Any new project added to `code/` immediately inherits all rules and skills.
3. **Transparent to Git**: Parent `.agents` directory can be committed or ignored as preferred.

---

## 🧪 Validating Installation

To verify that all components are functioning properly, run the test suites:

```powershell
# Run the complete test suite across rules, skills, and agents
python -m unittest discover -s tests -p "test_*.py"
```

```bash
# Verify coverage and security tools
python scripts/check_coverage.py --help
python scripts/run_wstg_security_scanner.py --help
```
