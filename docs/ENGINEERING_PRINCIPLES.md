# Engineering Handbook & Principles — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Engineering Handbook & Governance Principles |
| **Document ID** | `DOC-ENG-001` |
| **Current Status** | DRAFT — Pending CTO Review |
| **Document Version** | `1.1.0` |
| **Document Owner** | Lead Software Engineer & Software Architect |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Target Audience** | All Software Engineers, Security Engineers, DevOps Engineers, and Technical Contributors |
| **Related Documents** | [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md), [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) |

---

## 2. Purpose & Principle Relationships

### 2.1 Why Engineering Principles Exist
The **Engineering Handbook** establishes the mandatory operational standards, coding methodologies, security lifecycles, and review workflows for every contributor to **Antigraviiti Extension Protect (AEP)**. While product requirements define *what* we build, and system architecture defines *how components interact*, engineering principles define **how engineers work, collaborate, write code, and verify safety**.

Adhering to a unified engineering standard ensures that code written across different components (Desktop Agent, Cloud API, Web Dashboard, Chrome Companion) remains maintainable, secure, readable, and free of technical debt as the engineering organization scales.

### 2.2 The Three-Tier Governance Framework

AEP enforces a three-tier governance hierarchy. Each tier operates at a distinct scope and must never contradict the tier above it:

```
+-----------------------------------------------------------------------------------+
|                            TIER 1: PROJECT PRINCIPLES                             |
|                    (Constitution: Product, Business & Scope)                      |
|                      Location: docs/PROJECT_PRINCIPLES.md                         |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v (Governs)
+-----------------------------------------+-----------------------------------------+
|                          TIER 2: ENGINEERING PRINCIPLES                           |
|                  (Handbook: SSDLC, Coding Standards, Git, QA)                     |
|                    Location: docs/ENGINEERING_PRINCIPLES.md                       |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v (Governs)
+-----------------------------------------+-----------------------------------------+
|                         TIER 3: ARCHITECTURE PRINCIPLES                           |
|                (System Specifications: Clean Arch, Schemas, ADRs)                 |
|                      Location: docs/ARCHITECTURE_PRINCIPLES.md                    |
+-----------------------------------------------------------------------------------+
```

1. **Project Principles (Tier 1)**: Defines business philosophy, non-binary risk models, offline-first constraints, and product boundaries. (Non-negotiable product constitution).
2. **Engineering Principles (Tier 2 - THIS DOCUMENT)**: Governs the Secure Software Development Life Cycle (SSDLC), clean code standards, git hygiene, testing policies, and secure coding practices.
3. **Architecture Principles (Tier 3)**: Specifies system topology, module interfaces, database schemas, API contracts, and Architecture Decision Records (`adr/`).

---

## 3. Engineering Decision Hierarchy

When two valid engineering principles or architectural desires come into conflict, engineers MUST resolve the tradeoff according to the following strict **7-Level Priority Hierarchy**:

```
 1. Security              (Highest Priority — Non-Negotiable)
    └── 2. Privacy
         └── 3. Correctness
              └── 4. Maintainability
                   └── 5. Scalability
                        └── 6. Performance
                             └── 7. Developer Convenience (Lowest Priority)
```

### 3.1 Priority Level Definitions

1. **Security (Priority 1 - Highest)**: System security, sandbox isolation, vulnerability prevention, and exploit mitigation take precedence over all other considerations. Code that introduces a security flaw is unacceptable under any circumstance.
2. **Privacy (Priority 2)**: User data minimization, PII sanitization, and local source code isolation yield only to core security controls. Privacy cannot be sacrificed for features or performance.
3. **Correctness (Priority 3)**: Code must strictly do what it claims to do. Deterministic behavior, edge-case coverage, and lack of bugs supersede speed of implementation or performance optimizations.
4. **Maintainability (Priority 4)**: Clean Architecture, SOLID principles, readability, and self-documenting designs take precedence over clever performance micro-optimizations.
5. **Scalability (Priority 5)**: Decoupled queue architectures, efficient DB indexing, and horizontal expansion capability supersede micro-benchmarks or developer convenience.
6. **Performance (Priority 6)**: Sub-millisecond execution times and minimal memory footprints are desirable, but MUST NEVER be achieved by compromising Security, Privacy, Correctness, or Maintainability.
7. **Developer Convenience (Priority 7 - Lowest)**: IDE setup ease, shortcut libraries, or personal coding style preferences sit at the absolute bottom. Convenience MUST yield to all higher priorities.

### 3.2 Practical Tradeoff Decision Examples

- **Example A: Security vs Performance**
  - *Scenario*: Caching unsanitized raw AST parsing trees in global shared memory speeds up repeated scans by 10x, but introduces cross-process memory leakage risks.
  - *Resolution*: **Security wins over Performance**. Shared memory caching is rejected. Ephemeral sandboxed parsing memory is enforced.
- **Example B: Privacy vs Cloud Feature Richness**
  - *Scenario*: Sending full extension source code to Cloud LLMs provides slightly richer AI summaries, but violates local source isolation.
  - *Resolution*: **Privacy wins over Feature Richness**. Source code transmission is blocked; only anonymized AST metadata is sent.
- **Example C: Correctness vs Developer Convenience**
  - *Scenario*: Using a loose untyped `any` dictionary in TypeScript speeds up feature delivery, but risks runtime null-pointer crashes.
  - *Resolution*: **Correctness wins over Developer Convenience**. Strict Pydantic / Zod schemas are required.

---

## 4. Definition of Ready (DoR)

To prevent wasted engineering effort and architectural churn, **no Work Order, feature task, or code implementation may begin** until it meets the strict **Definition of Ready (DoR)** criteria:

```
+-----------------------------------------------------------------------------------+
|                            DEFINITION OF READY (DoR)                              |
+-----------------------------------------------------------------------------------+
|  [✓] 1. Clear Objective         : Precise problem statement defined               |
|  [✓] 2. Clear Scope             : Explicit technical deliverables itemized         |
|  [✓] 3. Clear Out of Scope      : Explicit list of excluded tasks                   |
|  [✓] 4. Acceptance Criteria     : Verifiable, measurable quality conditions        |
|  [✓] 5. Related Documents       : Cross-linked research papers, ADRs, or specs    |
|  [✓] 6. Zero Unresolved Deps    : No reliance on unapproved parent documents       |
|  [✓] 7. Formal CTO Sign-Off     : Approved by Security Architect / CTO            |
+-----------------------------------------------------------------------------------+
```

If any condition in the DoR checklist is missing, the Work Order is classified as **NOT READY** and work MUST NOT proceed.

---

## 5. Secure Software Development Life Cycle (SSDLC)

AEP strictly implements a **Secure Software Development Life Cycle (SSDLC)**. Security controls, threat modeling, and architectural validation are embedded into every stage of development, rather than bolted on after implementation.

```
[1. Planning] ──> [2. Research] ──> [3. Threat Modeling] ──> [4. Architecture Design]
                                                                        │
[8. CTO Review] <── [7. Security Test] <── [6. Automated Test] <── [5. Implementation]
       │
       v
[9. Release Candidate] ──> [10. Production] ──> [11. Maintenance]
```

### Stage 1: Planning
- **Goal**: Define feature scope based on validated security problems or user needs.
- **Entry Gate**: User problem validation aligned with `docs/PROJECT_PRINCIPLES.md`.
- **Deliverable**: Scope document in `planning/`.

### Stage 2: Research
- **Goal**: Perform domain, threat intelligence, and vulnerability research prior to design.
- **Entry Gate**: Unresolved technical or domain questions.
- **Deliverable**: Research whitepaper in `research/<category>/`.

### Stage 3: Threat Modeling
- **Goal**: Identify attack vectors, trust boundaries, STRIDE threat classifications, and mitigation controls before writing code.
- **Entry Gate**: Completion of research.
- **Deliverable**: Threat Model document in `security/THREAT_MODEL.md`.

### Stage 4: Architecture & Design
- **Goal**: Establish Clean Architecture layers, module boundaries, API schemas, and ADRs.
- **Entry Gate**: Threat model approval.
- **Deliverable**: Approved technical specification in `docs/` and formal `adr/ADR-xxx.md`.

### Stage 5: Implementation
- **Goal**: Write clean, maintainable, self-documenting source code following SSDLC standards.
- **Entry Gate**: Signed-off architecture specification, approved ADR, and DoR compliance.
- **Deliverable**: Feature branch commit history adhering to git conventions.

### Stage 6: Automated Testing
- **Goal**: Verify correctness via unit, integration, and regression test suites.
- **Entry Gate**: Code completion.
- **Deliverable**: 100% passing test execution in CI/CD pipeline.

### Stage 7: Security Testing (SAST & DAST)
- **Goal**: Perform static code analysis, vulnerability scanning (Bandit, Cargo Audit, Semgrep), and dependency vulnerability auditing.
- **Entry Gate**: Automated test pass.
- **Deliverable**: Clean security audit report with zero high/critical vulnerabilities.

### Stage 8: CTO Review & Audit
- **Goal**: Formal review by the Security Architect / CTO evaluating design compliance, security posture, and code quality.
- **Entry Gate**: Passing tests and completed Engineering Checklist.
- **Deliverable**: CTO Sign-off on Pull Request.

### Stage 9: Release Candidate (RC)
- **Goal**: Package binaries (Desktop Agent, Cloud Docker container, Extension ZIP) in staging environment.
- **Deliverable**: Signed RC build artifacts.

### Stage 10: Production Deployment
- **Goal**: Roll out release candidate using canary or blue-green deployment strategies.
- **Deliverable**: Production deployment with monitoring active.

### Stage 11: Maintenance & Patch Governance
- **Goal**: Monitor telemetry, resolve reported bugs, audit CVE updates, and maintain backward compatibility.

---

## 6. Engineering Philosophy & Core Design Patterns

To maintain code quality across diverse technical stacks (Go, Rust, Python, TypeScript), engineers must apply these eleven core software engineering principles.

### 6.1 Simplicity First
- **Why Adopted**: Complex code hides security flaws and bugs. Simple code is easy to audit, debug, and maintain.
- **Application**: Avoid premature abstraction. Prefer explicit code flow over clever tricks or obscure language hacks.

### 6.2 SOLID Principles
- **Single Responsibility Principle (SRP)**: A module or class must have one, and only one, reason to change.
  - *Why*: Prevents monolith classes where modifying parsing logic breaks risk scoring.
- **Open/Closed Principle (OCP)**: Software entities should be open for extension, but closed for modification.
  - *Why*: Allows adding new static analysis rules without modifying core engine pipeline orchestrators.
- **Liskov Substitution Principle (LSP)**: Subtypes must be substitutable for their base types without altering program correctness.
  - *Why*: Ensures mock interfaces in test suites behave identically to production infrastructure adapters.
- **Interface Segregation Principle (ISP)**: Clients should not be forced to depend upon interfaces they do not use.
  - *Why*: Prevents fat interfaces; scanner modules depend only on `ManifestReader`, not full database storage instances.
- **Dependency Inversion Principle (DIP)**: High-level modules must not depend on low-level modules. Both must depend on abstractions.
  - *Why*: Core business logic depends on storage interfaces, allowing seamless swapping of PostgreSQL and SQLite.

### 6.3 DRY (Don't Repeat Yourself)
- **Why Adopted**: Code duplication creates maintenance nightmares where security bug fixes are applied in one location but forgotten in another.
- **Application**: Extract shared domain logic, permission mapping tables, and validation regex into central, reusable utility modules.

### 6.4 KISS (Keep It Simple, Stupid)
- **Why Adopted**: Over-engineered solutions introduce unnecessary moving parts, increasing system failure points.
- **Application**: Choose straightforward data structures (arrays, maps) over complex custom graph implementations unless performance profiling proves necessity.

### 6.5 YAGNI (You Aren't Gonna Need It)
- **Why Adopted**: Writing code for hypothetical future requirements bloats the codebase and diverts engineering focus from core MVP goals.
- **Application**: Implement strictly what is specified in the current approved architecture doc. Do not add unused parameters or speculative abstract classes.

### 6.6 Separation of Concerns
- **Why Adopted**: Conflating UI rendering, database persistence, and security analysis into single files prevents modular testing and creates tight coupling.
- **Application**: Enforce strict directory and module boundaries between UI representation, API controllers, domain logic, and data access layers.

### 6.7 Security by Default
- **Why Adopted**: Default configurations must protect users even if they perform no manual configuration.
- **Application**: Enable strict CORS policies, restrictive CSP headers, non-root container users, and read-only filesystem mounts by default.

### 6.8 Principle of Least Privilege (PoLP)
- **Why Adopted**: Minimizes blast radius in the event of a component compromise.
- **Application**: Desktop Agent processes run under standard user privileges. Database user accounts possess only `SELECT`, `INSERT`, `UPDATE` permissions on specific tables (no `DROP` or `SUPERUSER` privileges).

### 6.9 Fail Secure (Fail-Safe Defaults)
- **Why Adopted**: When an exception, panic, or network disruption occurs, the system must default to a secure state rather than an insecure bypass.
- **Application**: If an archive extraction fails halfway or an AST parser encounters unparseable syntax, the extension MUST be flagged for manual review with an elevated risk flag, rather than marked as "Safe".

### 6.10 Defense in Depth
- **Why Adopted**: No single security control is foolproof. Multiple redundant defensive layers prevent catastrophic breaches.
- **Application**: Zip-Slip attacks are mitigated at three distinct layers: (1) Archive extraction size validation, (2) Canonical path prefix validation, and (3) Ephemeral sandboxed extraction directories.

---

## 7. Clean Architecture Requirements

AEP enforces Clean Architecture across all backend and desktop modules to decouple core business logic from frameworks, UI, and storage technologies.

```
+-----------------------------------------------------------------------+
|                    FRAMEWORKS, DB, DRIVERS, UI                        |
|             (FastAPI, Next.js, PostgreSQL, FileSystem)                |
|  +-----------------------------------------------------------------+  |
|  |                  INTERFACE ADAPTERS / CONTROLLERS               |  |
|  |             (REST Controllers, DTOs, Repositories)              |  |
|  |  +-----------------------------------------------------------+  |  |
|  |  |                   APPLICATION USE CASES                   |  |  |
|  |  |            (ExecuteScan, ComputeRiskScore)                |  |  |
|  |  |  +-----------------------------------------------------+  |  |  |
|  |  |  |                   DOMAIN ENTITIES                   |  |  |  |
|  |  |  |         (ExtensionPackage, Permission, Finding)     |  |  |  |
|  |  |  +-----------------------------------------------------+  |  |  |
|  |  +-----------------------------------------------------------+  |  |
|  +-----------------------------------------------------------------+  |
+-----------------------------------------------------------------------+
```

### 7.1 Layer Rules & Inviolable Dependency Inversion
1. **Domain Layer (Core)**: Contains enterprise entities, value objects, and domain errors. **MUST HAVE ZERO DEPENDENCIES** on external frameworks, ORMs (SQLAlchemy), or HTTP libraries.
2. **Application Use Cases Layer**: Contains orchestration logic (`ScanExtensionUseCase`). Depends ONLY on the Domain layer and abstract interface ports (`StorageRepositoryPort`).
3. **Interface Adapters Layer**: Converts data from HTTP/JSON DTOs into domain objects and implements repository interfaces using ORMs.
4. **Frameworks & Infrastructure Layer**: External tools (FastAPI, PostgreSQL, Redis, React, OS APIs). **High-level layers never know which framework is executing them.**

### 7.2 Architectural Constraints
- **UI Cannot Access DB**: Next.js or Desktop UI components MUST NEVER query database drivers directly. All data access flows through API endpoints or application use cases.
- **Framework Independence**: Swapping FastAPI for another web framework or switching PostgreSQL to SQLite MUST NOT require modifying any business logic inside `Domain` or `Application` layers.
- **Dependency Injection**: Use Cases receive repository implementations via constructor injection, enabling seamless unit testing with mock repositories.

---

## 8. Dependency Management Policy

Third-party libraries represent significant supply-chain attack risks. Every external dependency introduced into AEP must undergo strict auditing and governance.

### 8.1 Dependency Selection Criteria
Before proposing a new dependency, engineers must verify:
1. **Necessity**: Can this functionality be implemented cleanly using standard libraries in <50 lines of code? If yes, **DO NOT ADD THE DEPENDENCY**.
2. **Maintenance Health**: Is the repository actively maintained? (Recent commits within 90 days, multiple contributors, no abandoned open issues).
3. **Security History**: Has the library experienced unpatched critical vulnerabilities or supply-chain compromises?

### 8.2 Mandatory Governance Rules
- **Explicit ADR Approval**: Adding any new direct third-party dependency requires a brief justification in a Pull Request and approval from the Technical Lead / CTO.
- **Exact Version Pinning**: All dependencies MUST be strictly pinned to exact versions in lockfiles (`package-lock.json`, `Cargo.lock`, `poetry.lock`, `go.sum`). Range operators (`^`, `~`, `>=`) are strictly prohibited in lockfiles.
- **License Compatibility**: Only open-source licenses compatible with commercial enterprise distribution are permitted:
  - **Permitted**: MIT, Apache 2.0, BSD-2-Clause, BSD-3-Clause, ISC.
  - **Prohibited Without Legal Review**: GPL v2/v3, AGPL, SSPL, Reciprocal licenses.
- **Automated Supply-Chain Auditing**: CI/CD pipelines execute automated vulnerability scanning (`npm audit`, `cargo audit`, `pip-audit`, `trivy`) on every build. Any direct dependency with a CVE severity of `HIGH` or `CRITICAL` fails the build immediately.

---

## 9. Git Workflow & Collaboration Standard

AEP uses a structured, trunk-based Git workflow to ensure code quality, auditable commit histories, and reliable release branching.

### 9.1 Branch Naming Convention
Branches must adhere to the following prefix format:

```
<type>/<issue-or-ticket-id>-<short-description>
```

- **Allowed Types**:
  - `feat/`: New feature implementation (e.g., `feat/AEP-102-ast-eval-detector`)
  - `fix/`: Bug fix (e.g., `fix/AEP-204-zip-slip-canonical-path`)
  - `docs/`: Documentation updates (e.g., `docs/AEP-05-engineering-principles`)
  - `refactor/`: Code refactoring without behavioral changes (e.g., `refactor/AEP-88-decouple-parser`)
  - `sec/`: Security patch or hardening (e.g., `sec/AEP-301-sanitize-telemetry-pii`)

### 9.2 Commit Message Convention
Commits MUST follow the **Conventional Commits** standard:

```
<type>(<scope>): <short imperative description>

[optional detailed body explaining WHY the change was made]

[optional footer referencing issue numbers or ADRs]
```

- **Example**:
  ```git
  feat(scanner): add AST detection rule for dynamic Script tag creation

  Implements static analysis rule SAST-JS-014 targeting document.createElement('script')
  injected by extension content scripts. Includes unit test suite with 100% coverage.

  Closes #142
  Ref: docs/STATIC_ANALYSIS_ENGINE.md
  ```

### 9.3 Pull Request & Code Review Governance
1. **Self-Audit First**: The author MUST complete the **Engineering Review Checklist** (Section 12) before requesting review.
2. **PR Description Template**: Every PR must document: (1) Purpose of change, (2) Architectural reference, (3) Testing performed, and (4) Security considerations.
3. **Review Requirements**: Every PR requires **at least one approval from a Senior/Lead Engineer AND explicit sign-off from the CTO**.
4. **Merge Policy**: Merging requires linear history. **Squash and Merge** is enforced. Force pushing to `main` or `develop` branches is strictly disabled by branch protection rules.

---

## 10. Testing Policy & Quality Assurance

Code without automated testing is considered incomplete and unmaintainable. AEP enforces a comprehensive test pyramid across all components.

```
       / \
      / E2E \        (End-to-End Tests: Full Scanner Pipeline & UI)
     /-------\
    / Integr. \      (Integration Tests: API Gateways, DB & File Systems)
   /-----------\
  /  Unit Tests  \   (Unit Tests: AST Rules, Risk Engine Math, Parsers)
 /-----------------\
```

### 10.1 Testing Requirements Matrix

| Test Level | Scope & Target | Execution Frequency | Coverage Target | Mandatory Requirement |
| :--- | :--- | :--- | :--- | :--- |
| **Unit Test** | Isolated functions, risk score formulas, AST node detectors, permission mappers. | Every commit / Local CI | **>90% Line Coverage** | Fast execution (<5ms per test). Mocks external I/O. |
| **Integration Test** | Repository ORM operations, sandboxed zip extractors, REST endpoints, Redis tasks. | Pull Request pipeline | **>80% Coverage** | Tests interactions between adapters and infrastructure. |
| **End-to-End (E2E)** | Full workflow from Desktop Agent scan trigger to Cloud API report generation. | Staging / Pre-Release | Core Critical Paths | Tests real binaries with sample `.crx`/`.zip` extension packages. |
| **Security Test** | SAST code scanning, dependency CVE audits, Zip-Slip/Bomb payload resistance. | CI/CD Nightly & PR | 100% Vulnerability Free | Tests system against known malicious extension samples. |
| **Regression Test** | Historical bug reproduction suites and backward compatibility rule checks. | Every Pull Request | 100% Pass Rate | Ensures fixed security bugs never re-emerge. |

---

## 11. Engineering Metrics & Quality Indicators

To objectively measure codebase health, team performance, and security posture, AEP tracks seven core **Engineering Metrics**:

| Metric Name | Target Benchmark | Function & Governance Purpose |
| :--- | :--- | :--- |
| **1. Build Success Rate** | **$\ge 99.5\%$** | Measures CI pipeline stability and commit hygiene. Prevents broken main branches. |
| **2. Test Pass Rate** | **$100\%$** | Zero failing tests allowed in `main` or release branches. Flaky tests are isolated immediately. |
| **3. Unit Test Coverage** | **$>90\%$ (Domain)** | Ensures critical business logic, risk scoring algorithms, and AST rules are fully verified. |
| **4. Security Vulnerability Count** | **$0$ High / Critical** | Tracks SAST and dependency CVE findings. Any High/Critical issue blocks production deployment. |
| **5. Documentation Coverage** | **$100\%$** | Measures alignment between codebase, OpenAPI schemas, DB schemas, and technical specs. |
| **6. Mean Time To Fix (MTTF)** | **$<24$h (Critical)**<br/>**$<72$h (High)** | Measures engineering responsiveness in deploying patches for verified security vulnerabilities. |
| **7. Review Turnaround Time** | **$<24$ hours** | Measures velocity of CTO and peer PR code reviews, preventing pull request staleness. |

---

## 12. Secure Coding Standard

All code written for AEP must implement defensive security controls aligned with the **OWASP Top 10** and **OWASP API Security Top 10**.

### 12.1 Input Validation & Sanitization
- **Strict Whitelisting**: Validate all incoming parameters (file uploads, API request bodies, path arguments) against strict whitelist schemas using Pydantic or Zod.
- **Path Traversal Defense**: All file paths supplied from archives or APIs MUST be canonicalized (`os.path.realpath` / `fs.canonicalize`) and verified to reside inside assigned sandbox directories before file I/O operations occur.

### 12.2 Secret Management
- **Zero Hardcoded Secrets**: Storing API keys, database passwords, JWT signing secrets, or private certificates in source code or git history is strictly prohibited.
- **Environment & Key Vault Storage**: Secrets MUST be injected via environment variables or secure key vaults (HashiCorp Vault, AWS Secrets Manager). Local development configuration uses `.env.example` templates (`.env` files are in `.gitignore`).

### 12.3 Error Handling & Information Leakage
- **Fail Secure**: Unhandled exceptions must terminate processing safely and log internal error details locally without exposing raw stack traces, internal paths, or database schemas to external HTTP clients.
- **Generic Error Responses**: Public API endpoints return standardized, sanitized error payloads (e.g. `{"error": "INVALID_PACKAGE_STRUCTURE", "message": "The uploaded package structure is invalid."}`).

### 12.4 Logging & Audit Standards
- **Structured JSON Logging**: Logs must be emitted in structured JSON format (`timestamp`, `level`, `component`, `trace_id`, `message`).
- **Zero Log Poisoning & PII Redaction**: Passwords, authorization tokens, user PII, and raw cookies MUST be scrubbed before emitting log entries. Log outputs must sanitize newline characters to prevent log injection attacks.

---

## 13. Engineering Review Checklist

Before requesting a CTO review on any Pull Request, the author MUST complete and verify every item in this checklist:

```markdown
### Pre-Review Engineering Checklist

#### 1. Code Quality & Clean Architecture
- [ ] Work Order meets Definition of Ready (DoR) prior to PR submission.
- [ ] Code strictly follows Clean Architecture layer boundaries (Domain has zero framework dependencies).
- [ ] Functions are short, focused, and adhere to Single Responsibility Principle (SRP).
- [ ] Code is self-documenting with clear naming; complex security logic includes explanatory comments.
- [ ] No dead code, commented-out debug blocks, or print statements remain.

#### 2. Security & SSDLC Compliance
- [ ] Input validation is implemented using whitelist schemas (Pydantic/Zod).
- [ ] Archive extraction includes Zip-Slip canonical path checks and Zip Bomb size limits.
- [ ] PII and local user profile paths are scrubbed prior to network transmission.
- [ ] Zero secrets, API keys, or credentials exist in source code or git history.
- [ ] Security dependencies audited with zero high/critical vulnerabilities.

#### 3. Testing & Verification
- [ ] Unit tests written and passing with >90% code coverage on new domain logic.
- [ ] Integration tests verify database migrations and external adapter boundaries.
- [ ] Regression test added if this PR addresses a reported bug or vulnerability.

#### 4. Governance & Backward Compatibility
- [ ] Change maps to a validated requirement in `docs/PROJECT_PRINCIPLES.md`.
- [ ] Approved ADR exists in `adr/` if introducing structural architectural changes or new dependencies.
- [ ] Database migrations include reversible `upgrade()` and `downgrade()` scripts.
- [ ] REST API endpoints enforce explicit path versioning (`/api/v1/`).
- [ ] All commit messages adhere to Conventional Commits format.
```

---

## 14. Trade-Off Discussion: Quality vs Velocity

Adopting these rigorous engineering standards introduces explicit short-term development overhead:

| Engineering Constraint | Short-Term Impact | Long-Term Strategic Benefit |
| :--- | :--- | :--- |
| **Strict Clean Architecture & Layer Decoupling** | Requires writing interface abstractions and DTO converters upfront (+20% boilerplate). | Enables replacing database engines, web frameworks, or desktop UI stacks without rewriting business logic. |
| **Mandatory Research, ADRs & Threat Models** | Delays code writing by requiring formal design approvals. | Prevents architectural rewrites, eliminates security regressions, and aligns entire team on design context. |
| **Zero Third-Party Dependency Rule** | Requires writing custom light utilities instead of pulling npm/pip packages. | Protects platform against supply-chain attacks, malicious package takeovers, and license conflicts. |
| **100% Test & Security Audit Gate** | Extends CI/CD build execution time and requires writing comprehensive test suites. | Enables confident continuous deployment with zero downtime or security incidents. |

**Conclusion**: The short-term reduction in initial coding velocity yields exponential gains in product stability, security posture, code maintainability, and long-term engineering velocity.

---

## 15. Future Evolution of the Handbook

As the AEP engineering organization grows from a core founding team into larger specialized units (Frontend, Backend, Security Engineering, OS Endpoint Agents), this handbook will evolve through formal revision processes:

1. **Periodic Architectural Reviews**: This handbook will undergo formal review by the CTO and Technical Leads every six months.
2. **Team-Specific Addendums**: Specialized sub-handbooks (e.g. `docs/DESKTOP_AGENT_GUIDELINES.md`, `docs/SECURITY_AUDITING_GUIDELINES.md`) may be created to extend these core rules for specific technical domains.
3. **ADR Governance for Policy Changes**: Modifying any engineering policy within this document requires drafting an ADR in `adr/` and obtaining unanimous approval from the lead engineering team and CTO.

---

## 16. Related Documents

- [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md) — AEP Constitutional Principles
- [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) — Master Documentation Index
- [`adr/README.md`](file:///d:/ExtensionProtect/adr/README.md) — Architecture Decision Record Register
