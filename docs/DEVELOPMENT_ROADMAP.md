# Development Roadmap — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Operational Engineering Development Roadmap |
| **Document ID** | `DOC-ROADMAP-001` |
| **Current Status** | ACTIVE — Operational Engineering Reference |
| **Document Version** | `1.0.0` |
| **Document Owner** | Lead Software Engineer & Engineering Manager |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Current Active Sprint**| **Sprint 1: Desktop Agent Foundation** |
| **Source References** | [`docs/TECHNOLOGY_STACK.md`](file:///d:/ExtensionProtect/docs/TECHNOLOGY_STACK.md), [`docs/SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md), [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md), [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md) |

---

## 2. High-Level Release Phase Overview

```
+-----------------------------------------------------------------------------------+
|                        AEP RELEASE PHASE ROADMAP TIMELINE                         |
+-----------------------------------------------------------------------------------+
|  PHASE 1: DESKTOP AGENT MVP (Sprints 1 - 4)  | S1: Core Foundation & Scanner      |
|                                              | S2: Local AST SAST & Risk Engine   |
|                                              | S3: Local History & OS Alerts      |
|                                              | S4: React UI Polish & Integration  |
|  -------------------------------------------------------------------------------  |
|  PHASE 2: CLOUD INTEL & CVES (Sprints 5 - 6) | S5: FastAPI Backend & PostgreSQL   |
|                                              | S6: Threat Intel & CVE Matching    |
|  -------------------------------------------------------------------------------  |
|  PHASE 3: COMPANION & REPORT (Sprints 7 - 8) | S7: Chrome MV3 Extension IPC       |
|                                              | S8: Executive PDF Exporter         |
|  -------------------------------------------------------------------------------  |
|  PHASE 4: ENTERPRISE FLEET (Sprints 9 - 12)  | S9-12: Multi-Tenant CISO Console   |
+-----------------------------------------------------------------------------------+
```

---

## 3. Sprint Planning & Breakdown

---

### Phase 1: Desktop Agent MVP (Core Offline Product)

#### Sprint 1: Desktop Agent Foundation & Local Ingestion
- **Focus**: Project workspace initialization, Tauri v2 Rust setup, React UI scaffold, embedded SQLite 3 initialization, sandboxed archive extractor with Zip-Slip defense, and local Chromium profile auto-discovery.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-08-18
- **Milestones**:
  - M1.1: Tauri v2 + React 18 workspace building cleanly across Windows/macOS/Linux.
  - M1.2: Embedded SQLite 3 database initialized with WAL mode and initial migrations.
  - M1.3: Local profile discoverer enumerating installed Chrome/Edge/Brave extensions.
  - M1.4: Sandboxed Zip extractor accepting `.crx`/`.zip` files with canonical path checks.
- **Dependencies**: Architecture approval sign-off ([`SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md)).
- **Status**: **IN PROGRESS (ACTIVE)**

#### Sprint 2: Local Static Analysis & Deterministic Risk Engine
- **Focus**: Manifest V2/V3 inspector, SWC Rust AST static analysis parser, dynamic code detection (`eval`, `atob`, secrets), and deterministic Risk Engine calculation ($0.0 - 100.0$).
- **Duration**: 2 Weeks
- **Target Completion**: 2026-09-01
- **Milestones**:
  - M2.1: SWC AST parser pinpointing dynamic code calls and line numbers in JavaScript.
  - M2.2: Hardcoded cloud secret detector matching AWS, Stripe, and OpenAI API key signatures.
  - M2.3: Deterministic Risk Engine executing mathematical rule weights ($0.0 - 100.0$).
  - M2.4: Itemized mathematical point deduction log compilation.
- **Dependencies**: Sprint 1 completed (Tauri Rust Core & File Streamer).
- **Status**: PLANNED

#### Sprint 3: Local Scan History, OS Alerts & AI Fallback Integration
- **Focus**: SQLite scan history persistence, native OS desktop banner notification manager, sanitized local telemetry scrubbing manager, and local Ollama / template AI synthesizer integration.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-09-15
- **Milestones**:
  - M3.1: SQLite persistence storing completed scan JSON reports and version histories.
  - M3.2: Native OS desktop notification dispatching banners for High-Risk scans ($\ge 70.0$).
  - M3.3: PII telemetry scrubbing manager sanitizing local user profile paths.
  - M3.4: Local Ollama AI adapter generating plain-language executive summaries.
- **Dependencies**: Sprint 2 completed (Deterministic Risk Engine).
- **Status**: PLANNED

#### Sprint 4: Desktop UI Integration & MVP Final Polish
- **Focus**: React UI components (Dashboard, Extension List, Forensic AST Tree Inspector, Scan History Timeline), Tailwind styling, Tauri IPC command wiring, and MVP release QA.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-09-29
- **Milestones**:
  - M4.1: Responsive dark-mode React UI displaying extension health cards & risk badges.
  - M4.2: End-to-end Tauri IPC command integration connecting React frontend to Rust backend.
  - M4.3: Version 1.0 MVP installer packaging (`.msi` for Windows, `.dmg` for macOS, `.AppImage` for Linux).
- **Dependencies**: Sprint 3 completed.
- **Status**: PLANNED

---

### Phase 2: Cloud Intelligence & Threat Enrichment

#### Sprint 5: FastAPI Backend & PostgreSQL Storage Setup
- **Focus**: FastAPI application cluster scaffold, Pydantic v2 schemas, PostgreSQL 16 connection pooling, Alembic migrations, and Celery async task queue setup with Redis.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-10-13
- **Dependencies**: Phase 1 MVP completion.
- **Status**: PLANNED

#### Sprint 6: Threat Intelligence & CVE Matching Engine
- **Focus**: SHA-256 asset hash matching worker, NVD CVE library vulnerability scanner, C2 domain blocklist checker, and telemetry ingestion API endpoint.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-10-27
- **Dependencies**: Sprint 5 completed.
- **Status**: PLANNED

---

### Phase 3: Browser Companion Extension & PDF Reporting

#### Sprint 7: Manifest V3 Chrome Companion Extension
- **Focus**: In-browser Manifest V3 extension, toolbar risk badge status indicators, and local WebSocket IPC bridge (`ws://127.0.0.1:49152`) to Desktop Agent.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-11-10
- **Dependencies**: Phase 1 MVP completion.
- **Status**: PLANNED

#### Sprint 8: Executive PDF Compliance Report Exporter
- **Focus**: Server-side multi-page PDF generation engine, executive risk distribution charts, and SOC 2 / GDPR audit export capabilities.
- **Duration**: 2 Weeks
- **Target Completion**: 2026-11-24
- **Dependencies**: Sprint 6 & 7 completed.
- **Status**: PLANNED

---

## 4. Summary Matrix of Milestones & Deliverables

| Phase | Sprint | Core Deliverable | Key Target Date | Milestone Status |
| :--- | :--- | :--- | :--- | :--- |
| **Phase 1** | **Sprint 1** | **Desktop Agent Workspace, Tauri Rust Core, SQLite, Discovery, Zip Extractor** | **2026-08-18** | **IN PROGRESS** |
| **Phase 1** | **Sprint 2** | **SWC AST SAST Engine, Manifest Inspector, Deterministic Risk Engine** | **2026-09-01** | **PLANNED** |
| **Phase 1** | **Sprint 3** | **SQLite History Storage, OS Native Desktop Alerts, AI Ollama Synthesizer** | **2026-09-15** | **PLANNED** |
| **Phase 1** | **Sprint 4** | **React Desktop UI Dashboard, Tauri IPC Wiring, v1.0 MVP Installer Build** | **2026-09-29** | **PLANNED** |
| **Phase 2** | **Sprint 5** | **FastAPI Cloud Backend API, PostgreSQL DB, Celery + Redis Workers** | **2026-10-13** | **PLANNED** |
| **Phase 2** | **Sprint 6** | **Threat Intel Hash Matching (`SHA-256`), NVD CVE Library Scanner** | **2026-10-27** | **PLANNED** |
| **Phase 3** | **Sprint 7** | **Manifest V3 Chrome Companion Extension & Local WebSocket IPC Bridge** | **2026-11-10** | **PLANNED** |
| **Phase 3** | **Sprint 8** | **Executive PDF Compliance Report Exporter & SOC 2 Audit Generator** | **2026-11-24** | **PLANNED** |

---

## 5. Related Engineering Documents

- [`docs/TASK_BOARD.md`](file:///d:/ExtensionProtect/docs/TASK_BOARD.md) — Operational Engineering Task Board
- [`docs/CHANGELOG.md`](file:///d:/ExtensionProtect/docs/CHANGELOG.md) — Project Version Changelog
- [`docs/TECHNOLOGY_STACK.md`](file:///d:/ExtensionProtect/docs/TECHNOLOGY_STACK.md) — Approved Technology Stack Specification
- [`docs/SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md) — Master System Architecture Blueprint
