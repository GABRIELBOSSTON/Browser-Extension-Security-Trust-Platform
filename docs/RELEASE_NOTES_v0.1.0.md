# Release Notes v0.1.0 (Sprint 1 RC)

## Overview
Antigraviiti Extension Protect (AEP) v0.1.0 marks the completion of the core offline static-analysis pipeline infrastructure. The primary focus of this release is establishing mathematical determinism, type-safe Rust architectures, and uncompromised engine boundaries for evaluating Chromium browser extension risk without relying on external cloud intelligence.

## Completed Work Orders
- **WO #019**: Manifest Parser Engine
- **WO #020**: Extension Capability Model (ADR-002)
- **WO #021**: Rule Engine Architecture (ADR-003)
- **WO #022**: Rule Engine Foundation Implementation
- **WO #023**: Deterministic Risk Engine Implementation
- **WO #024**: Sprint 1 Documentation Stabilization

## Implemented Architecture
- **Clean Architecture**: Domain Models, Application Services, and Infrastructure Adapters are strictly segregated.
- **Fail-Fast & Batch Orchestration**: The asynchronous `AnalysisPipeline` supports isolated thread execution for both single file scans (Fail-Fast) and bulk directory scans (Continue-on-Error).
- **Zero Panic Guarantee**: Built entirely on Rust's `Result<T, E>` pattern; absolutely zero `unwrap()` or `panic!()` exist in the evaluation pipeline.

## Major Domain & Engine Implementations
1. **Manifest Service**: Safely extracts deep structural configuration from `manifest.json` under 10MB bounds (MV2 and MV3 compatible).
2. **Capability Builder**: Translates raw JSON manifests into a specialized `ExtensionCapabilityModel`, isolating static security vectors.
3. **Rule Engine**: Sandboxes execution by mapping predefined `MatcherId` definitions directly to natively compiled Rust functions, nullifying JSON injection vulnerabilities.
4. **Risk Engine**: Introduces bounded normalization (0-100) scoring logic tied to dynamic `RiskProfile` thresholds. Features robust `AggregationPolicy` math for duplicate penalty decay.

## Testing Status
- **Golden Dataset Initiated**: Categorized directories (`/mv2`, `/mv3`, `/broken`, `/malicious`, etc.) established.
- **Integration Test Strategy**: Complete pipeline test logic defined, utilizing `EmbeddedRuleSource` to mock I/O boundaries.

## Repository Readiness
The core daemon is functionally stabilized. System consistency checks are at 100%. The pipeline is ready to be exposed via IPC to the UI, and ready to digest AST code trees in subsequent releases.
