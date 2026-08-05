# Technology Stack Decision Document — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Authoritative Technology Stack Specification & Evaluation |
| **Document ID** | `DOC-TECH-001` |
| **Current Status** | FINAL — Approved Engineering Reference |
| **Document Version** | `1.0.0` |
| **Document Owner** | Lead Software Architect & Systems Engineer |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Target Audience** | Software Engineers, Desktop Developers, Backend Engineers, DevOps, and Security Auditors |
| **Source References** | [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md), [`docs/ENGINEERING_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md), [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md), [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md), [`docs/USE_CASE.md`](file:///d:/ExtensionProtect/docs/USE_CASE.md), [`docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md), [`docs/DOMAIN_MODEL.md`](file:///d:/ExtensionProtect/docs/DOMAIN_MODEL.md), [`docs/BOUNDED_CONTEXT.md`](file:///d:/ExtensionProtect/docs/BOUNDED_CONTEXT.md), [`docs/SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md) |

---

## 2. Technology Selection Principles

All technology choices for Antigraviiti Extension Protect (AEP) are governed by nine strict architectural evaluation criteria:

```
+-----------------------------------------------------------------------------------+
|                        TECHNOLOGY SELECTION CRITERIA                              |
+-----------------------------------------------------------------------------------+
| 1. Performance & Speed      : Sub-second local execution (<3s scan time).       |
| 2. Minimal Footprint        : Low memory consumption (<30 MB RAM idle).          |
| 3. Privacy & Security       : Zero default cloud leaks; memory safety.           |
| 4. 100% Offline Capability  : Zero runtime dependence on external cloud APIs.    |
| 5. Cross-Platform Support   : Native support for Windows, macOS, and Linux.       |
| 6. Ecosystem Stability      : Proven long-term support (LTS) & active community.  |
| 7. Maintainability          : Strong typing, modular boundaries, and clean APIs.  |
| 8. Developer Velocity       : Fast prototyping and rapid MVP iteration.            |
| 9. Infrastructure Economy   : Minimal server compute costs during MVP tier.       |
+-----------------------------------------------------------------------------------+
```

---

## 3. Desktop Application Framework: Tauri v2 vs Electron

### 3.1 Comparative Analysis

| Evaluation Metric | Electron (Chromium + Node.js) | Tauri v2 (OS WebView + Rust Core) | Selected Winner |
| :--- | :--- | :--- | :--- |
| **Idle RAM Usage** | High (~150 MB - 300 MB per window) | **Extremely Low (~15 MB - 30 MB)** | **Tauri v2** |
| **Binary Installer Size** | Large (~80 MB - 120 MB installer) | **Tiny (~10 MB - 15 MB installer)** | **Tauri v2** |
| **Memory Safety & Security**| Medium (V8 Node.js attack surface) | **Critical (Rust memory safety & IPC isolation)** | **Tauri v2** |
| **Sub-Second AST Performance**| Moderate (V8 JS thread execution) | **Blazing (Native Rust multi-threaded AST parsing)**| **Tauri v2** |
| **Rust Core Integration** | Requires complex C++ Node-addons | **Native (Built-in Rust backend process)** | **Tauri v2** |
| **Cross-Platform Delivery** | Excellent (Windows, macOS, Linux) | **Excellent (Windows, macOS, Linux, Mobile ready)** | **Tauri v2** |
| **Operating Footprint** | Heavy (Bundles full Chromium runtime)| Lightweight (Uses system native WebView2/WebKit)| **Tauri v2** |

### 3.2 Decision & Technical Justification
- **SELECTED TECHNOLOGY**: **Tauri v2 (Rust Core + OS Native WebView)**
- **Technical Justification**: Electron bundles a full Chromium browser instance and Node.js runtime, creating a heavy footprint (~150 MB RAM idle, ~100 MB installer size) that violates our engineering metrics ([`ENGINEERING_PRINCIPLES.md`, Section 11](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md#11-engineering-metrics--quality-indicators)). Tauri v2 uses the operating system's native webview (WebView2 on Windows, WebKit on macOS/Linux) and a **Rust core daemon**. Rust delivers memory safety, zero-cost abstractions, multi-threaded AST static code analysis, and sub-30 MB RAM usage, satisfying 100% of our offline-first desktop requirements ([`ADR-001`](file:///d:/ExtensionProtect/docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md)).

---

## 4. Desktop UI Layer Framework

### 4.1 Comparative Analysis

| Criteria | React 18 + Vite | Vue 3 + Vite | SvelteKit | Selected Winner |
| :--- | :--- | :--- | :--- | :--- |
| **Rendering Performance** | Excellent | Excellent | Superior | **React 18 + Vite** |
| **Component Ecosystem** | Dominant (Shadcn UI, Lucide Icons) | Moderate | Growing | **React 18 + Vite** |
| **Developer Familiarity** | Universal | High | Moderate | **React 18 + Vite** |
| **Styling Flexibility** | Vanilla CSS / Tailwind CSS | Scoped CSS | Scoped CSS | **React 18 + Vite** |

### 4.2 Decision & Technical Justification
- **SELECTED TECHNOLOGY**: **React 18 + Vite + Tailwind CSS / Vanilla CSS**
- **Technical Justification**: React 18 paired with Vite provides lightning-fast Hot Module Replacement (HMR), a massive ecosystem of security visualization libraries, and seamless integration with Tauri's webview bridge. Combined with Tailwind CSS and custom Vanilla CSS tokens ([`PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md)), it enables responsive dark-mode dashboards and forensic tree inspectors without runtime overhead.

---

## 5. Cloud Backend API Framework: Go vs FastAPI vs Rust

### 5.1 Comparative Analysis

| Evaluation Metric | Go (Gin / Chi) | FastAPI (Python 3.11+) | Rust (Axum / Actix-web) | Selected Winner |
| :--- | :--- | :--- | :--- | :--- |
| **Development Velocity** | High | **Superior (Fastest for MVP)** | Moderate | **FastAPI** |
| **AI / LLM Integration** | Moderate (Third-party SDKs) | **Native (Official LangChain/OpenAI SDKs)** | Moderate | **FastAPI** |
| **Type Safety & Validation**| High | **High (Pydantic v2 + Python Type Hints)** | Superior | **FastAPI** |
| **Execution Performance** | Superior (Compiled) | High (AsyncIO + Uvicorn) | Maximum | **FastAPI** |
| **Async Task Ecosystem** | Goroutines | **Celery + Redis Task Queue** | Tokio tasks | **FastAPI** |

### 5.2 Decision & Technical Justification
- **SELECTED TECHNOLOGY**: **FastAPI (Python 3.11+ with Pydantic v2 & Uvicorn)**
- **Technical Justification**: While Rust powers the local Desktop Agent scanner for performance, the Cloud Backend API focuses on rapid integration with Threat Intelligence feeds, CVE databases, and AI LLM prompt orchestrators. Python 3.11+ with **FastAPI** delivers unmatched developer velocity, native ecosystem support for AI/LLM SDKs (OpenAI, LangChain, Ollama adapters), automatic OpenAPI documentation, and strict runtime type validation via Pydantic v2.

---

## 6. Database Tier: Local & Cloud Persistence

```
+-----------------------------------------------------------------------------------+
|                           DATABASE ARCHITECTURE SELECTION                         |
+-----------------------------------------------------------------------------------+
|  LOCAL DESKTOP PERSISTENCE  : Embedded SQLite 3 (WAL Mode + Rusqlite)              |
|  CLOUD BACKEND PERSISTENCE  : PostgreSQL 16 (Hosted on Supabase / Neon)           |
+-----------------------------------------------------------------------------------+
```

### 6.1 Local Embedded Database: SQLite 3
- **Selection**: **SQLite 3** (accessed via Rust `rusqlite` / `sqlx` driver).
- **Justification**: SQLite requires zero server installation, runs entirely embedded inside the Desktop Agent binary, provides zero-configuration ACID compliance, and stores local scan history in a single local file (`~/.aep/storage.db`). Configured with **Write-Ahead Logging (WAL) mode** to prevent database locks during concurrent scan write ops ([`ADR-001`, Section 3.3](file:///d:/ExtensionProtect/docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md#33-strict-privacy--infrastructure-governance-rules)).

### 6.2 Cloud Relational Database: PostgreSQL 16
- **Selection**: **PostgreSQL 16** (Serverless hosting on Supabase / Neon).
- **Justification**: PostgreSQL provides industry-standard relational integrity, JSONB support for storing complex manifest structures, and robust security access controls. Hosting on serverless PostgreSQL (Neon / Supabase free tier) eliminates cloud database maintenance costs during MVP releases.

---

## 7. ORM & Database Access Layer

- **Desktop Agent (Rust)**: **`rusqlite` / `sqlx`**
  - Direct compile-time type-checked SQL queries ensuring zero ORM performance overhead on endpoint hardware.
- **Cloud Backend (Python)**: **`SQLAlchemy 2.0` + `Alembic`**
  - Industry-standard Python ORM featuring async I/O support, explicit relationship mappings, and robust database migration management via Alembic.

---

## 8. Static Analysis Engine: AST Code Parsing

### 8.1 Comparative Analysis

| AST Parser Engine | Language Base | Execution Speed | Obfuscation Entropy Detection | Selected Winner |
| :--- | :--- | :--- | :--- | :--- |
| **Babel Parser** | JavaScript | Moderate | Good | Rejected |
| **Tree-sitter** | C / Rust | High | Moderate (Grammar-based) | Secondary |
| **SWC (Speedy Web Compiler)**| **Rust** | **Blazing (10x faster than Babel)**| **Superior (Native Rust AST Nodes)**| **SWC (Primary)** |

### 8.2 Decision & Technical Justification
- **SELECTED TECHNOLOGY**: **SWC (Speedy Web Compiler - Rust) + Tree-sitter**
- **Technical Justification**: Parsing thousands of obfuscated JavaScript files inside browser extensions requires an ultra-high-speed AST parser. **SWC**, written in Rust, parses JavaScript and TypeScript source files up to 10x-40x faster than JavaScript-based parsers (Babel/Esprima). Integrating SWC directly into the Desktop Agent Rust core allows instant, sub-second AST node traversal to detect dynamic `eval()`, `Function()`, `atob()`, exposed cloud secrets, and DOM scraping sinks ([`BOUNDED_CONTEXT.md`, Section 4.2](file:///d:/ExtensionProtect/docs/BOUNDED_CONTEXT.md#42-context-2-analysis-context-core-domain)).

---

## 9. AI Layer Architecture: Hybrid Dual-Engine Strategy

```
+-----------------------------------------------------------------------------------+
|                        HYBRID DUAL-ENGINE AI ARCHITECTURE                         |
+-----------------------------------------------------------------------------------+
|  PRIMARY (CLOUD CONNECTED)  : OpenAI API (GPT-4o-mini) via Async Cloud Backend    |
|  FALLBACK (OFFLINE / LOCAL) : Local Ollama Instance (llama3 / mistral)            |
+-----------------------------------------------------------------------------------+
```

### 9.1 Technical Architecture
- **Primary Cloud Mode**: When internet connectivity is available, the Cloud Backend dispatches sanitized SAST findings to **OpenAI API (`gpt-4o-mini`)** for rapid, low-cost qualitative security narrative generation.
- **Offline / Local Fallback Mode**: When internet connectivity is absent or air-gapped, the Desktop Agent queries a local **Ollama** instance (`llama3` or `mistral`) or falls back to template-based qualitative rules.
- **Strict Invariant Guarantee**: As mandated by Principle 12 ([`PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md)), AI models MUST NEVER compute or modify numerical Risk Scores ($0.0 - 100.0$). AI is strictly isolated to qualitative explanation.

---

## 10. Cloud Infrastructure & Hosting Stack (MVP Free Tier)

| Service Layer | Selected Vendor / Platform | Free Tier Capability / Rationale |
| :--- | :--- | :--- |
| **Frontend Web Dashboard** | **Vercel** | Free global CDN hosting for Next.js 14 Web Dashboard. |
| **Cloud API Backend** | **Render / Railway** | Free tier web service for FastAPI Python API container. |
| **Cloud Database** | **Supabase / Neon** | Free managed PostgreSQL 16 database with 500 MB storage. |
| **Task Queue & Cache** | **Upstash Redis** | Free serverless Redis for Celery async task management. |

---

## 11. Browser Platform Support Matrix

- **Version 1.0 MVP Target**: Full auto-discovery and manifest V2/V3 parsing for **Google Chrome**, **Microsoft Edge**, **Brave**, and **Opera** across Windows, macOS, and Linux profiles.
- **Version 5.0 Future Target**: Expansion to **Mozilla Firefox** (`.xpi`) and **Apple Safari** extension profile structures.

---

## 12. Development Tools & Quality Assurance Ecosystem

```
+-----------------------------------------------------------------------------------+
|                      DEVELOPMENT & QA ECOSYSTEM TOOLING                           |
+-----------------------------------------------------------------------------------+
|  Version Control & Repo   : Git + GitHub (Monorepo architecture)                  |
|  CI/CD Automation Pipeline: GitHub Actions (Automated build, test & linter)       |
|  Rust Toolchain          : Cargo + Clippy + Rustfmt                              |
|  Python Toolchain        : Pnpm + Ruff (Linter/Formatter) + Pytest + Mypy        |
|  Frontend Toolchain      : Vitest + Biome / ESLint + Tailwind Compiler           |
+-----------------------------------------------------------------------------------+
```

---

## 13. Master Technology Matrix

The table below provides a complete summary of all approved technology selections:

| System Component | Selected Technology | Alternative Evaluated | Selection Rationale | Decision Status |
| :--- | :--- | :--- | :--- | :--- |
| **Desktop Application** | **Tauri v2 (Rust)** | Electron | <30 MB RAM idle, tiny binary size (~10 MB), memory safety. | **Approved** |
| **Desktop UI Framework** | **React 18 + Vite** | Vue 3 / Svelte | Massive ecosystem, fast HMR, rich UI components (Shadcn). | **Approved** |
| **Desktop UI Styling** | **Tailwind CSS + Custom CSS** | Plain CSS | Rapid UI layout development with curated dark-mode tokens. | **Approved** |
| **Local Database Engine**| **SQLite 3 (WAL Mode)** | LiteFS / JSON | Embedded, zero-config, ACID compliant, single local file. | **Approved** |
| **Local AST Parser** | **SWC (Rust) + Tree-sitter**| Babel / Esprima | Blazing 10x-40x faster JavaScript/TypeScript AST parsing. | **Approved** |
| **Cloud Backend API** | **FastAPI (Python 3.11+)**| Go / Express | Fastest MVP dev velocity, native OpenAI/Ollama SDK support. | **Approved** |
| **Cloud Database Engine**| **PostgreSQL 16** | MySQL | Robust JSONB support, relational integrity, Supabase hosting. | **Approved** |
| **Cloud Task Broker** | **Celery + Redis** | RabbitMQ | Standard async task queue for CVE lookups and AI prompts. | **Approved** |
| **AI Narrative Engine** | **Hybrid (OpenAI + Ollama)**| Pure Cloud OpenAI | 100% offline fallback support; zero cloud lock-in. | **Approved** |
| **Cloud Hosting (MVP)** | **Vercel + Render + Supabase**| AWS / GCP | Zero server compute cost during MVP launch phase. | **Approved** |

---

## 14. Technology Risk Analysis & Mitigations

| Risk ID | Technology Choice Risk | Impact | Mitigation Strategy |
| :--- | :--- | :--- | :--- |
| **TECH-RISK-01** | **Tauri v2 OS WebView Divergence**: Rendering inconsistencies across Windows (WebView2) vs macOS (WebKit). | Medium | Standardize UI on cross-browser CSS reset tokens and run automated Visual Regression tests. |
| **TECH-RISK-02** | **Rust Learning Curve**: Engineering team friction with Rust borrow checker during initial Desktop Agent development. | Medium | Enforce clear domain module boundaries; keep Rust code focused strictly on Scanner and Analysis contexts. |
| **TECH-RISK-03** | **Cloud Free-Tier Cold Starts**: Render/Supabase free instance sleeping causing latency on initial API queries. | Low | Desktop Agent UI operates 100% offline using local SQLite data; cloud latency never blocks local UI. |

---

## 15. Multi-Release Multi-Phase Evolution Strategy

The selected technology stack is designed to scale across future product versions without requiring platform rewrites:

```
Version 1.0 MVP Core ──> Version 2.0 Cloud Intel ──> Version 3.0 Companion Ext ──> Version 5.0 Enterprise Fleet
 (Tauri v2 + Rust Core)  (FastAPI + PostgreSQL)   (Manifest V3 Chrome Ext)    (Enterprise Fleet Console)
```

- **Version 1.0 MVP Core**: Validates Tauri v2 + Rust SWC AST parser + SQLite 3 fully offline on endpoint hardware.
- **Version 2.0 (Cloud Enrichment)**: Activates FastAPI backend, Celery task queues, PostgreSQL, and Threat Intel lookup microservices.
- **Version 3.0 (Chrome Companion Extension)**: Builds Manifest V3 extension communicating with Tauri Rust core over local WebSocket IPC (`ws://127.0.0.1:49152`).
- **Version 5.0 (Enterprise Fleet Governance)**: Deploys compiled Tauri Desktop Agent binaries via corporate MDM (Intune/Jamf) connected to a centralized enterprise PostgreSQL fleet console.

---

## 16. Self-Audit & Approval Sign-Off

### Self-Audit Verification
- [x] **Fully Justified**: Every technology choice includes detailed comparative evaluation and technical justification.
- [x] **Alternatives Evaluated**: Evaluated Electron vs Tauri, Go vs FastAPI, SQLite vs LiteFS, SWC vs Babel, etc.
- [x] **Master Architecture Alignment**: Synchronized 100% with [`SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md), [`BOUNDED_CONTEXT.md`](file:///d:/ExtensionProtect/docs/BOUNDED_CONTEXT.md), [`DOMAIN_MODEL.md`](file:///d:/ExtensionProtect/docs/DOMAIN_MODEL.md), and [`ADR-001`](file:///d:/ExtensionProtect/docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md).
- [x] **Implementation Independent Boundary**: Free of SQL schema code, REST endpoint paths, or component code implementations.

### Formal Sign-Off
- **Architectural Decision Lead**: Lead Software Architect & Systems Engineer
- **Approved By**: Chief Technology Officer (CTO) / Security Architect  
- **Approval Date**: 2026-08-04  
- **Decision Status**: **FINAL — Approved Engineering Reference**
