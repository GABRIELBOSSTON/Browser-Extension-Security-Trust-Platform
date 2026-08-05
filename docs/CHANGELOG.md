# Changelog — Antigraviiti Extension Protect (AEP)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v0.1.0] - 2026-08-04

### Added
- Completed `TSK-S1-010` Tauri IPC Commands Wiring (WO #038):
  - Injected `AppState` as a Composition Root managing `AnalysisPipeline`.
  - Added Presentation Layer DTOs (`ScanExtensionRequest`, `ScanExtensionResponse`, `ExtensionSummaryResponse`) preventing Domain leakage to Frontend.
  - Mitigated CPU-heavy AST blocking on async Tokio runtime by offloading AnalysisPipeline logic natively to OS threads via `tokio::task::spawn_blocking`.
  - Initialized isolated frontend TypeScript wrappers in `src/services` (`ipc.ts`, `scanService.ts`, `extensionService.ts`).
- Completed `TSK-S1-003` React 18 + Vite + Tailwind CSS Frontend Scaffold (WO #037):
  - Initialized strictly typed Presentation Layer architecture (`features/`, `components/`, `styles/`).
  - Configured Vite and Tailwind CSS with `@tailwindcss/forms` and `@tailwindcss/typography`.
  - Implemented core AppLayout and Header UI shell integrating Dark Theme Ready tokens.
  - Setup absolute path aliasing (`@/*`) natively synchronized across Vite and TypeScript configs.
- Completed `TSK-S1-005` Sandboxed Package Unpacker & Zip-Slip Defense (WO #036):
  - Engineered strongly typed `SandboxId(Uuid)` backed ephemeral directories.
  - Pluggable `ArchiveExtractorRegistry` resolving implementations purely by magic byte signatures.
  - Strict pre-creation in-memory path normalization defending against Zip-Slip traversal vectors explicitly.
  - Complete RAII separation delegating physical directory cleanup to `SandboxHandle::Drop` in the infrastructure layer.
  - Implemented `ManifestExistsValidator` utilizing the new extensible Validation framework chain.
- Completed `TSK-S3-001` SQLite Persistence Repository (WO #035):
  - Strictly decoupled SQL primitives via a dynamic `ConnectionProvider`.
  - Upgraded schema payloads utilizing `Uuid`, `RiskScore`, and `Severity` strong domains natively mapped via Infrastructure bindings.
  - Formally decoupled serialization burdens, restricting `serde_json` explicit usage to SQLite layers exclusively.
  - Implemented transactional database migrations ensuring schemas load seamlessly across cold boots prior to repositories linking.
- Completed `TSK-S2-009` Risk Score Calculator Foundation:
  - Deterministic floating-point mathematical calculation strictly clamped to 100.0.
  - Implements `Once`, `Sum`, `Max`, and geometric `Decay` limits natively.
  - Formally structures top contributor severities isolated cleanly from Rule Matching engine outputs.
- Completed `TSK-S2-008` Rule Matcher Foundation:
  - Strongly typed `RuleMatcherService` depending on `RuleRepository` and `DetectorInventory` abstractions.
  - Scalable structural rule conditions (`Exists`, `Equals`, `CountGreaterThan`, `StartsWith`, etc).
  - Produces structural findings strictly divorced from the Risk Engine severity math.
- Completed `TSK-S2-007` Secret Detector Foundation:
  - Strongly typed `SecretId`, `MatchConfidence`, `SecretSourceKind`, and `PatternRegistry`.
  - Securely masks string literal previews (64-char cap).
  - Isolates scalable statistics utilizing `SecretType` HashMaps natively.
- Completed `TSK-S2-006` Dangerous API Detector Foundation:
  - Captures execution sinks (`eval`, `setTimeout`, `importScripts`) exclusively.
  - `DangerousApiId(u32)` utilizing inline fixed-seed FNV-1a block determinism.
- Completed `TSK-S2-005` Chrome API Detector Foundation:
  - Intercepts Chrome-specific namespace expressions via `CallExpression` interception.
- Completed `TSK-S2-004` Call Graph Foundation:
  - Strongly-typed `FunctionId`, `EdgeId`, `Visibility`, `FunctionKind`, `EdgeType`.
  - Configurable static generation avoiding string allocation overheads.
  - Zero-panic `StaticCallGraphBuilder` decoupling detection pipelines.
- Completed `TSK-S1-002` Extension Discovery Engine:
  - Discovers Chrome, Edge, Brave, and Opera profile paths via safe API mapping.
  - Generates DTO `DiscoveryResult` for cross-boundary integration.
- Completed `TSK-S1-003` Manifest Parser Engine:
  - Safe deserialization of Manifest V2 and V3 models up to 10MB limits using strict JSON serde.
  - Zero-panic domain parser with modularized validation and mapping pipelines.
  - Added Domain Value Objects (`MatchPattern`, `CapabilityId`).
  - Separated `Manifest` capabilities into heavily composed structural capability collections.
  - Created `CapabilityBuilder` Application Service to normalize and merge cross-version permissions.
- Completed `TSK-S1-005` Rule Engine Foundation:
  - Modeled `RuleSet`, `Rule`, and `Evidence` Domain structures without embedded execution logic.
  - Established `RuleSource` repository abstraction for `JsonRuleSource` and `EmbeddedRuleSource`.
  - Built `RuleEngine` to map static JSON string `matcher_id`s to natively compiled Rust matcher logic safely.
- Completed `TSK-S1-006` Risk Engine:
  - Added Domain Value Objects (`RiskProfile`, `RiskAssessment`, `RiskBreakdown`, `AggregationPolicy`).
  - Implemented dynamic risk deduplication utilizing ONCE, SUM, and DECAY strategies.
  - Implemented `RiskEngine` calculation isolating RawScore from bounded NormalizedScore (0-100).
- Completed `TSK-S1-007` End-to-End Analysis Pipeline Integration:
  - Centralized orchestration into `AnalysisPipeline` with `AnalysisContext` tracking intermediate states.
  - Implemented `Single` (fail-fast) and `Batch` (continue-on-error) async execution modalities.
  - Formatted `PipelineResult` with rich diagnostics (`StageResult`, timings, environment metadata, versions).
  - Reserved event-driven hooks for future UI and telemetry integrations.
- Completed `TSK-S1-001` Project Initialization & Clean Architecture Foundation:
  - Configured root workspace (`package.json`, `tsconfig.json`, `vite.config.ts`, `.eslintrc.json`, `.prettierrc`, `.gitignore`).
  - Scaffolding Tauri v2 desktop application shell with Rust core daemon (`src-tauri/`) and React 18 + Vite + Tailwind CSS frontend (`src/`).
  - Implemented Clean Architecture layers (`domain/`, `application/`, `infrastructure/`, `presentation/`) in both Rust and TypeScript.
  - Implemented embedded SQLite 3 database manager (`DatabaseManager`) in Rust with Write-Ahead Logging (WAL) mode initialization (`~/.aep/storage.db`).
  - Exposed Tauri IPC commands (`ping`, `app_version`, `get_database_status`).
  - Created domain model types and Vitest unit tests in `src/__tests__/domain.test.ts` and Cargo unit tests in `src-tauri/src/domain/types.rs` & `src-tauri/src/infrastructure/db.rs`.
- Created official Master Product & Engineering Documentation suite in `docs/`:
  - [`PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md): 14 Constitutional Project Principles across 5 categories.
  - [`ENGINEERING_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md): Engineering Handbook, SSDLC, Clean Architecture, and 7-Level Priority Hierarchy.
  - [`PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md): Strategic Product Vision, Mission, Values, Boundaries, and Roadmap v1.0–v5.0.
  - [`VALUE_PROPOSITION.md`](file:///d:/ExtensionProtect/docs/VALUE_PROPOSITION.md): Value Proposition Canvas, UVP, Market Gap Analysis, and Strategic Positioning.
  - [`USER_PERSONA.md`](file:///d:/ExtensionProtect/docs/USER_PERSONA.md): 8 Detailed Target Personas, Identity, Goals, Pain Points, and Primary MVP Persona (Maya Lin).
  - [`CUSTOMER_JOURNEY.md`](file:///d:/ExtensionProtect/docs/CUSTOMER_JOURNEY.md): 9-Stage Customer Lifecycle, 8 Persona Journeys, and Opportunity Matrix.
  - [`PROBLEM_VALIDATION.md`](file:///d:/ExtensionProtect/docs/PROBLEM_VALIDATION.md): Problem Validation Whitepaper with Evidence Confidence Levels, Severity & Frequency Classifications, and Top 5 Validated Threat Vectors.
  - [`PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md): Product Requirements Document (PRD) with 100% Requirement Traceability Matrix.
  - [`FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md): Authoritative Feature Catalog with 19 itemized feature specifications across 12 domains.
  - [`USE_CASE.md`](file:///d:/ExtensionProtect/docs/USE_CASE.md): Use Case Specification detailing 10 core use case interaction flows and acceptance criteria.
  - [`ADR-001_OFFLINE_FIRST_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/adr/ADR-001_OFFLINE_FIRST_ARCHITECTURE.md): Architecture Decision Record establishing Offline-First Desktop Agent architecture.
  - [`DOMAIN_MODEL.md`](file:///d:/ExtensionProtect/docs/DOMAIN_MODEL.md): Domain-Driven Design (DDD) Domain Model detailing Core/Supporting domains, 9 Entities, 7 Value Objects, Aggregates, Services, Events, and Mermaid class diagrams.
  - [`BOUNDED_CONTEXT.md`](file:///d:/ExtensionProtect/docs/BOUNDED_CONTEXT.md): Bounded Context Specification establishing single entity ownership, negative invariants, and inter-context communication relationships across 7 contexts.
  - [`SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md): Master System Architecture Blueprint detailing C4 Context/Container/Component models, data flows, trust boundaries, and multi-release evolution.
  - [`TECHNOLOGY_STACK.md`](file:///d:/ExtensionProtect/docs/TECHNOLOGY_STACK.md): Technology Stack Decision Document evaluating Tauri v2 (Rust), React 18 + Vite, SWC AST Parser, SQLite 3 WAL, FastAPI Python, and PostgreSQL 16.
  - [`DEVELOPMENT_ROADMAP.md`](file:///d:/ExtensionProtect/docs/DEVELOPMENT_ROADMAP.md): Operational Engineering Development Roadmap starting from Sprint 1.
  - [`TASK_BOARD.md`](file:///d:/ExtensionProtect/docs/TASK_BOARD.md): Implementation Task Board detailing Sprint 1 tasks (`TSK-S1-001` through `TSK-S1-010`).

### Changed
- Formally concluded and locked the **Product Strategy & Architecture Specification Phase** with full sign-off from Chief Technology Officer (CTO).

### Security
- Mandated Zip-Slip path traversal verification (`canonical_path.starts_with(sandbox_dir)`), Zip-Bomb decompression ratio checks (100:1 max), and privacy-by-default PII telemetry scrubbing across all system specifications.

---

## [v0.0.1] - 2026-08-03

### Added
- Initialized workspace structure in `d:\ExtensionProtect\`.
- Created preliminary project overview and initial research directory structures.
