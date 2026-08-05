# ADR 002: Introduce Extension Capability Model

## Status
**Approved**

## Context
Antigraviiti Extension Protect (AEP) requires a robust data structure to evaluate the security posture, risk factors, and functional behavior of browser extensions. Following the implementation of the Manifest Parser Engine, the next architectural milestone is the Permission Analyzer, which feeds into the Rule Engine, Risk Engine, AST Parser, and Threat Intelligence context.

## Problem Statement
Relying strictly on the `Manifest` domain entity for security and risk analysis violates the Single Responsibility Principle (SRP). The manifest represents what an extension *declares structurally*, but does not intrinsically map threat vectors, capability combinations (e.g., `activeTab` + `<all_urls>`), or behavioral context derived from AST analysis. A central mechanism is needed to consolidate and model an extension's complete privilege footprint.

## Alternatives Considered

### 1. Direct Manifest Analysis
- **Description:** Allow the Rule Engine and Risk Engine to directly traverse the `Manifest` entity to calculate risk scores.
- **Pros:** No additional data structures required; zero upfront memory overhead.
- **Cons:** Tightly couples the Rule Engine to structural idiosyncrasies (e.g., MV2 vs MV3). Violates SRP by forcing structural declaration models to handle threat logic. Difficult to link code usages (AST) back to the manifest declarations.

### 2. PermissionGraph
- **Description:** Construct a dedicated graph data structure specifically for mapping and analyzing extension permissions.
- **Pros:** Excellent for detecting privilege escalations and linking AST usage patterns to specific nodes. Decouples the parser from the analyzer.
- **Cons:** The name "Graph" exposes an implementation detail rather than a domain concept. Furthermore, it implies a narrow scope limited strictly to "permissions", ignoring other critical security vectors like Content Security Policies (CSP), Web Accessible Resources, Content Scripts, and externally connectable domains.

### 3. ExtensionCapabilityModel
- **Description:** Introduce an abstract, centralized domain aggregate that models everything an extension is *capable* of doing.
- **Pros:** Aligns with Domain-Driven Design (DDD) by focusing on business intent rather than underlying data structures. Infinitely extensible to encompass permissions, host access, CSPs, actions, and network capabilities. Acts as a unified nexus for the Manifest Parser, AST Parser, Rule Engine, and Threat Intelligence.

## Decision
We will adopt the **ExtensionCapabilityModel** as the central abstraction for analyzing extension behavior and privileges. 

## Rationale
The `ExtensionCapabilityModel` provides the highest degree of future-proofing and adherence to DDD principles. It correctly identifies that an extension's threat surface is defined by its *capabilities*, not just its permissions. By decoupling the capability representation from both the raw manifest file and the underlying mathematical data structure (which may evolve from a simple graph into an AST-linked hypergraph), we ensure the core domain remains stable as our SAST capabilities mature.

## Trade-offs
- **Upfront Complexity:** Introduces a translation step where the `Manifest` and AST outputs must be mapped into the `ExtensionCapabilityModel` before analysis can occur.
- **Memory Overhead:** Maintaining a parallel capability state alongside the raw manifest structure requires a marginal increase in RAM during local offline scans.

## Consequences
- The **Manifest Parser** will focus strictly on correctly reading and validating JSON structures without applying any security judgments.
- The **Permission Analyzer** (and subsequent AST analyzer) will act as builders that hydrate the `ExtensionCapabilityModel`.
- The **Rule Engine** and **Risk Engine** will execute their logic strictly against the uniform `ExtensionCapabilityModel`, remaining completely agnostic to whether the extension targets Chrome, Edge, MV2, or MV3.

## Future Evolution
As AEP evolves, the `ExtensionCapabilityModel` will serve as the master context for advanced static analysis (SAST). When we introduce the SWC AST engine, the model will effortlessly map JavaScript function calls (e.g., `chrome.storage.local.get`) directly back to the modeled capabilities, enabling deterministic detection of over-privileged extensions.

## References
* ADR-001_OFFLINE_FIRST_ARCHITECTURE.md
* DOMAIN_MODEL.md
* BOUNDED_CONTEXT.md
* SYSTEM_ARCHITECTURE.md
