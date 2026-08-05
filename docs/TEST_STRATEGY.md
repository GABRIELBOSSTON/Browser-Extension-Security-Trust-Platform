# Test Strategy: Antigraviiti Extension Protect (AEP)

## Overview
This document defines the overarching testing strategy for the AEP Desktop Agent, specifically focusing on the Rust Core Daemon. The goal is to mathematically and deterministically prove that the engine securely and efficiently analyzes browser extensions without causing runtime panics or executing dynamic javascript.

## 1. Unit Testing Strategy
Unit tests focus on the absolute smallest units of domain logic.
- **Scope**: Value Objects, Enum mappings, and pure functions.
- **Constraints**: 
  - Must not interact with the filesystem or I/O.
  - Must execute instantly.
- **Key Areas**: 
  - `RiskScore` bounded clamping (0.0 - 100.0).
  - `MatchPattern` parsing logic (`<all_urls>`, `*://*.com/*`).
  - `Severity` mapping via `RiskProfile`.

## 2. Integration Testing Strategy
Integration tests validate the sequence of events across multiple application boundaries without crossing the final IPC layer.
- **Scope**: `AnalysisPipeline` orchestrator, `ManifestService`, `CapabilityBuilder`, `RuleEngine`, `RiskEngine`.
- **Methodology**: 
  - Drive the `AnalysisPipeline` using the "Golden Dataset" (`tests/golden_data/`).
  - Assert the structural equivalence of the output `PipelineResult` against deterministic JSON snapshots.
  - **No mocks** for domain logic; we only mock the `RuleSource` (using `EmbeddedRuleSource`) and standard filesystem reads.

## 3. The Golden Dataset
A centralized `golden_data/` repository acts as the absolute source of truth for end-to-end integration tests. It is divided into scenarios:
- `/mv2`: Validates legacy Manifest V2 parsing (e.g. background page logic).
- `/mv3`: Validates modern Manifest V3 parsing (e.g. service workers, host permissions separation).
- `/broken`: Validates fail-fast error handling by injecting corrupted JSON or path traversal attacks.
- `/malicious`: Provides explicit known-bad extensions to verify `RuleEngine` triggering mechanisms.
- `/enterprise`: Tests deduplication behavior via massive, overly complex extensions.
- `/performance`: Extremely bloated manifests simulating Zip bombs.
- `/regression`: Real-world edge cases mapped from future bug reports.

## 4. Performance Benchmarking Strategy
Performance stability is critical to prevent UI freezing during batch scans.
- **Framework**: `cargo bench` (Criterion.rs).
- **Scales**: Benchmarks will measure the `AnalysisPipeline` processing arrays of 10, 100, 500, and 1,000 extensions.
- **Metrics Tracked**:
  - Total elapsed time (Target P95 < 50ms per extension).
  - Peak memory usage.
  - CPU utilization.
  - Disk read I/O rates.

## 5. Security Validation
- No code will contain `unwrap()` or `panic!()` in the production pathways.
- The `CapabilityBuilder` strictly acts as a structural mapper.
- The `RuleEngine` operates via constrained `matcher_id` mapping to prevent JSON rule injections from acting as executable payloads.

## 6. Tauri IPC Testing (Future Sprint)
The IPC boundary between the React frontend and the Rust backend will be tested via Tauri's mock runner configuration, ensuring UI state hydration matches the `PipelineResult` shapes perfectly.
