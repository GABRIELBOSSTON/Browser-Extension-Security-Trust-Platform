# ADR 003: Rule Engine Architecture

## Status
**Approved**

## Context
Antigraviiti Extension Protect (AEP) requires a deterministic mechanism to evaluate an extension's privileges, behaviors, and threat vectors. Following the implementation of the `ExtensionCapabilityModel` (ADR-002), we need a standardized Rule Engine that can evaluate these capabilities against a set of security rules to output actionable findings. This ADR defines the long-term architecture of the Rule Engine, ensuring it remains scalable, extensible, and decoupled from underlying parsers.

## Problem Statement
Hardcoding security rules directly into the application logic creates technical debt, limits scalability, and makes dynamic updates impossible. AEP needs an architecture where rules act as independent, version-controlled definitions that can be dynamically loaded and evaluated against an extension's abstract model (`ExtensionCapabilityModel`), yielding consistent and deterministic risk assessments.

## Goals
- Decouple rule definitions from application source code.
- Ensure deterministic rule execution against the `ExtensionCapabilityModel`.
- Support hot-swapping or cloud-updating rule sets in the future.
- Establish a strict schema for Rule output (Findings) to feed into the Risk Engine.
- Provide a standardized framework for Rule versioning and categorization.

## Non-Goals
- Defining the mathematical algorithm for the final Risk Score (this belongs to the Risk Engine).
- Parsing raw manifests or ASTs (handled by upstream builders).
- Executing dynamic runtime analysis (AEP is strictly a static analysis tool).

## Rule Lifecycle
1. **Authoring**: Rules are written by security analysts in a platform-agnostic format (e.g., JSON).
2. **Distribution**: Rules are bundled locally with the application or downloaded via secure cloud updates.
3. **Loading**: The Rule Engine deserializes the rules into memory at application startup.
4. **Execution**: The Engine evaluates active rules against an `ExtensionCapabilityModel`.
5. **Output**: The Engine emits a collection of `Findings`.

## Rule Categories
Rules are grouped into distinct categories based on the threat vector they assess:
- **Permission**: Evaluating over-privileged API requests (e.g., `debugger`, `proxy`).
- **Network**: Evaluating dangerous host access (e.g., `<all_urls>`, broad wildcards).
- **Behavioral**: Evaluating structural combinations (e.g., executing remote scripts via CSP vulnerabilities).
- **Code**: Evaluating specific AST signatures (e.g., `eval()`, obfuscation techniques).

## Rule Priority
Each rule operates with an assigned Priority (e.g., `Low`, `Medium`, `High`, `Critical`). Priority dictates the execution order within the engine and significantly influences the deductive weight applied to the final Risk Score if the rule is triggered.

## Rule Versioning
Rules adhere strictly to Semantic Versioning (`MAJOR.MINOR.PATCH`). 
- `MAJOR` changes indicate a structural change to the rule schema.
- `MINOR` changes indicate a refinement of the rule logic.
- `PATCH` changes indicate bug fixes (e.g., fixing a false-positive regex match).

## Rule Execution Flow
1. The Rule Engine receives an `ExtensionCapabilityModel`.
2. The Engine filters the loaded rules based on applicability (e.g., bypassing AST rules if AST parsing failed).
3. The Engine iterates through the applicable rules in priority order.
4. Each rule evaluates the model.
5. If a violation is detected, a `Finding` is generated.
6. The aggregate collection of `Findings` is returned.

## Rule Output Model
A triggered rule results in a `Finding`. The Finding explicitly contains:
- `RuleId`: The unique identifier of the triggered rule.
- `Severity`: The inherent severity of the violation.
- `Description`: A human-readable explanation of why the rule fired.
- `Evidence`: Contextual data (e.g., the specific `CapabilityId` or AST line number that triggered the rule).

## Rule Extensibility
The architecture allows for custom rule definitions. By abstracting the execution interface, future enhancements could support a Domain-Specific Language (DSL) or WebAssembly (Wasm) modules for highly complex, Turing-complete rule evaluations without modifying the core Rust daemon.

## Rule Source
- **Current State (MVP)**: Rules are stored as offline, embedded JSON files to maintain strict adherence to ADR-001 (Offline-First Architecture).
- **Future State**: AEP will support secure, cryptographically signed delta-updates from a centralized Threat Intelligence cloud, allowing the Rule Engine to ingest zero-day threat definitions.

## Trade-offs
- **Performance Overhead**: Evaluating hundreds of independent rules in memory is computationally heavier than hardcoded logic.
- **Complexity**: Designing a rule schema that is expressive enough to evaluate both simple permissions and complex AST trees requires careful schema engineering.

## Alternatives Considered
### 1. Hardcoded Rust Functions
- **Pros**: Maximum performance, type safety.
- **Cons**: Requires a full binary release for every new rule. Fails the requirement for dynamic cloud updates.
### 2. Embedded Scripting (e.g., Lua, rhai)
- **Pros**: Infinite flexibility.
- **Cons**: High security risk (executing untrusted rule scripts), potential performance bottlenecks, and violates the principle of least privilege. 

## Consequences
- The Rule Engine assumes a black-box design; it expects an `ExtensionCapabilityModel` and outputs `Findings`.
- Rule schemas must be strictly defined and validated upon load. Malformed rules will be discarded to prevent engine crashes.
- The Risk Engine (subsequent subsystem) will rely entirely on the `Findings` output to calculate the mathematical score.

## References
* ADR-001_OFFLINE_FIRST_ARCHITECTURE.md
* ADR-002_EXTENSION_CAPABILITY_MODEL.md
* DOMAIN_MODEL.md
* BOUNDED_CONTEXT.md
* SYSTEM_ARCHITECTURE.md
