# Implementation Task Board — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Operational Task Board & Implementation Backlog |
| **Document ID** | `DOC-TASK-001` |
| **Current Status** | ACTIVE — Operational Engineering Backlog |
| **Document Version** | `1.0.0` |
| **Current Active Sprint**| **Sprint 1: Desktop Agent Foundation** |
| **Last Updated** | 2026-08-04 |
| **Source References** | [`docs/DEVELOPMENT_ROADMAP.md`](file:///d:/ExtensionProtect/docs/DEVELOPMENT_ROADMAP.md), [`docs/TECHNOLOGY_STACK.md`](file:///d:/ExtensionProtect/docs/TECHNOLOGY_STACK.md), [`docs/SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md) |

---

## 2. Sprint 1 Task Board: Desktop Agent Foundation

Sprint 1 focuses on building the foundational Desktop Agent application shell using Tauri v2, initializing the embedded SQLite 3 database, establishing the Rust scanner module, implementing Zip-Slip archive extraction, and auto-discovering local Chromium extension profiles.

```
+-----------------------------------------------------------------------------------+
|                        SPRINT 1 TASK STATUS SUMMARY                               |
+-----------------------------------------------------------------------------------+
| TOTAL TASKS: 10  | TODO: 4   | IN PROGRESS: 0  | CODE REVIEW: 0  | DONE: 6        |
+-----------------------------------------------------------------------------------+
```

### Sprint 1 Implementation Task Checklist

| Task ID | Task Description | Priority | Target Module | Dependencies | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TSK-S1-001** | Initialize Monorepo Project Structure & Root Packaging Configuration | High | Workspace | Architecture Approval | **DONE** |
| **TSK-S1-002** | Scaffolding Tauri v2 Application Shell with Rust Core Daemon | Critical | Desktop Core | TSK-S1-001 | **DONE** |
| **TSK-S1-003** | Initialize React 18 + Vite + Tailwind CSS Frontend Scaffold | High | Desktop UI | TSK-S1-002 | **TODO** |
| **TSK-S1-004** | Embedded SQLite 3 Storage Initialization & Migration Manager | Critical | Storage Engine | TSK-S1-002 | **DONE** |
| **TSK-S1-005** | Implement Sandboxed Package Unpacker with Zip-Slip Path Traversal Defense | Critical | Scanner Context | TSK-S1-002 | **TODO** |
| **TSK-S1-006** | Implement Risk Engine Deduplication and Scoring Context | Critical | Analysis Context | TSK-S1-005 | **DONE** |
| **TSK-S1-007** | End-to-End Analysis Pipeline Integration | Critical | Analysis Context | TSK-S1-002 | **DONE** |
| **TSK-S1-008** | Implement Basic Manifest V2/V3 Parser Structure in Rust | High | Analysis Context | TSK-S1-005 | **DONE** |
| **TSK-S1-009** | Establish Deterministic Risk Engine Data Structures & Scoring Interfaces | Critical | Analysis Context | TSK-S1-008 | **DONE** |
| **TSK-S1-010** | Wire Initial Tauri IPC Commands (`get_installed_extensions`, `scan_package`) | High | IPC Bridge | TSK-S1-003, TSK-S1-007 | **TODO** |

---

## 3. Sprint 1 Detailed Task Specifications

#### TSK-S1-001: Monorepo & Workspace Initialization
- **Description**: Setup workspace directory structure, Cargo workspace manifest, Node package dependencies (`pnpm`), Git configuration, and `.gitignore` rules.
- **Priority**: High | **Target Module**: Root / Workspace | **Dependencies**: Architecture Approval Sign-Off | **Status**: **DONE**

#### TSK-S1-002: Tauri v2 Application Shell Setup
- **Description**: Configure Tauri v2 application core in Rust (`src-tauri/`), setup app window configuration, system tray handlers, and cross-platform native compilation scripts.
- **Priority**: Critical | **Target Module**: Desktop Core | **Dependencies**: TSK-S1-001 | **Status**: DONE

#### TSK-S1-003: React 18 + Vite + Tailwind UI Scaffold
- **Description**: Initialize React 18 TypeScript frontend app in `src/`, configure Vite build options, install Tailwind CSS, and establish core UI layout shell with dark-mode styling tokens.
- **Priority**: High | **Target Module**: Desktop UI | **Dependencies**: TSK-S1-002 | **Status**: **DONE**

#### TSK-S1-004: Embedded SQLite 3 Storage Initialization
- **Description**: Integrate `rusqlite` / `sqlx` in Rust core, configure Write-Ahead Logging (WAL) mode, create initial `storage.db` file manager, and write schema migrations for `scans` and `extensions` tables.
- **Priority**: Critical | **Target Module**: Storage Engine | **Dependencies**: TSK-S1-002 | **Status**: DONE

#### TSK-S1-005: Sandboxed Package Unpacker & Zip-Slip Defense
- **Description**: Implement file archive extraction module in Rust (`zip` crate), extracting uploaded `.crx` / `.zip` files into an ephemeral `/tmp/sandbox/` folder. Enforce canonical path validation (`canonical_path.starts_with(sandbox_dir)`) to prevent path traversal attacks.
- **Priority**: Critical | **Target Module**: Scanner Context | **Dependencies**: TSK-S1-002 | **Status**: **DONE**

#### TSK-S1-006: Implement Risk Engine Deduplication and Scoring Context
- **Description**: Define Rust data structures for `RiskAssessment`, `RiskProfile`, `AggregationPolicy`. Implement deterministic math bounding RawScore into NormalizedScore with specific ONCE, SUM, and DECAY tracking.
- **Priority**: Critical | **Target Module**: Analysis Context | **Dependencies**: TSK-S1-005 | **Status**: DONE

#### TSK-S1-007: End-to-End Analysis Pipeline Integration
- **Description**: Centralize all engines (`Manifest`, `Capability`, `Rule`, `Risk`) into `AnalysisPipeline`. Build `PipelineResult` DTO, handle Single/Batch async flows with cancellation tokens, and structure execution metadata.
- **Priority**: Critical | **Target Module**: Analysis Context | **Dependencies**: TSK-S1-002 | **Status**: DONE

#### TSK-S1-008: Manifest V2/V3 Parser Structure in Rust
- **Description**: Implement JSON manifest deserializer (`serde_json`) in Rust to extract extension name, version, manifest_version (2 vs 3), background service workers, content script match patterns, and requested permissions.
- **Priority**: High | **Target Module**: Analysis Context | **Dependencies**: TSK-S1-005 | **Status**: DONE

#### TSK-S1-009: Deterministic Risk Engine Foundations
- **Description**: Define Rust data structures for `RiskScore` (bounded float $0.0-100.0$), `Severity` enum, `Finding` struct, and `RiskAssessment` aggregate root. Implement basic mathematical score accumulator interfaces.
- **Priority**: Critical | **Target Module**: Analysis Context | **Dependencies**: TSK-S1-008 | **Status**: DONE

#### TSK-S1-010: Tauri IPC Commands Wiring
- **Description**: Expose Rust core functions to React frontend via Tauri IPC command channels (`#[tauri::command]`), enabling the UI to trigger local extension discovery and read scan results asynchronously.
- **Priority**: High | **Target Module**: IPC Bridge | **Dependencies**: TSK-S1-003, TSK-S1-007 | **Status**: **DONE**

---

## 4. Backlog Preview: Upcoming Sprint Tasks

### Sprint 2 Backlog: AST SAST Engine & Scoring Rules
- `TSK-S2-001`: Integrate SWC (Speedy Web Compiler) Rust AST parser for JavaScript files. **(DONE)**
- `TSK-S2-002`: AST Walker Foundation. **(DONE)**
- `TSK-S2-003`: AST Visitor & Detector Foundation. **(DONE)**
- `TSK-S2-004`: Call Graph Foundation. **(DONE)**
- `TSK-S2-005`: Chrome API Detector Foundation. **(DONE)**
- `TSK-S2-006`: Dangerous API Detector Foundation. **(DONE)**
- `TSK-S2-007`: Secret Detector Foundation. **(DONE)**
- `TSK-S2-008`: Rule Matcher Foundation. **(DONE)**
- `TSK-S2-009`: Finalize mathematical Risk Score calculator ($0.0 - 100.0$) and itemized breakdown builder. **(DONE)**

### Sprint 3 Backlog: Scan Persistence, OS Alerts & AI Fallback
- `TSK-S3-001`: Write SQLite persistence repository for scan JSON reports and audit logs. **(DONE)**
- `TSK-S3-002`: Implement native desktop notification manager (Windows Action Center / macOS).
- `TSK-S3-003`: Implement local PII path sanitization manager (`<USER_PROFILE>`).
- `TSK-S3-004`: Build local Ollama / qualitative template AI adapter in Rust/Python.

---

## 5. Related Engineering Documents

- [`docs/DEVELOPMENT_ROADMAP.md`](file:///d:/ExtensionProtect/docs/DEVELOPMENT_ROADMAP.md) — Operational Engineering Development Roadmap
- [`docs/CHANGELOG.md`](file:///d:/ExtensionProtect/docs/CHANGELOG.md) — Project Version Changelog
- [`docs/TECHNOLOGY_STACK.md`](file:///d:/ExtensionProtect/docs/TECHNOLOGY_STACK.md) — Approved Technology Stack Specification
