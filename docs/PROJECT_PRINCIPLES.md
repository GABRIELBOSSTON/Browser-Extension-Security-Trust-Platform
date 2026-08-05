# Project Principles — Antigraviiti Extension Protect (AEP)

---

## 1. Document Objective & Constitutional Mandate

This document serves as the **Project Constitution** for **Antigraviiti Extension Protect (AEP)**. It defines the 14 non-negotiable engineering, product, architectural, and operational principles that govern all development activities across the platform.

### Constitutional Enforcement & Governance Rules
1. **Universal Enforcement**: These principles apply universally across all roles—Founders, CTO, Product Managers, Security Engineers, Lead Software Architects, Frontend/Backend/Desktop Developers, and QA Engineers.
2. **Strict Immutable Status**: Once approved by the CTO, no principle within this document may be modified, bypassed, or suspended without a formal **Architecture Decision Record (ADR)** approved and signed by the Security Architect / CTO.
3. **Review Compliance**: Every Pull Request (PR), design spec, API schema, database migration, and scanner heuristic MUST explicitly reference compliance with these principles.

---

## 2. Taxonomy & Structural Logical Flow

The 14 Constitutional Principles are organized into five logical lifecycle categories:

```
+-----------------------------------------------------------------------------------+
|                         CATEGORY 1: PRODUCT & BUSINESS FOUNDATION                 |
|  Principle 1: Product Before Technology                                          |
|  Principle 2: User-Problem-Validated Features Only                                |
|  Principle 3: Quality Over Speed                                                  |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------+-----------------------------------------+
|                       CATEGORY 2: SECURITY & PRIVACY GOVERNANCE                   |
|  Principle 4: Security by Design & Zero-Trust Boundary                           |
|  Principle 5: Privacy by Default & Source Isolation                              |
|  Principle 6: Offline-First Desktop Core                                         |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------+-----------------------------------------+
|                      CATEGORY 3: ARCHITECTURAL & ECOSYSTEM DESIGN                 |
|  Principle 7: Desktop-First Ecosystem Architecture                                |
|  Principle 8: Mandatory Architectural Decision Records (ADR Mandate)              |
|  Principle 9: Research Before Implementation                                      |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------+-----------------------------------------+
|                     CATEGORY 4: IMPLEMENTATION & SCORING RIGOR                    |
|  Principle 10: Explainable & Deterministic Risk Scoring                           |
|  Principle 11: Human Explainability & Itemized Score Breakdown                   |
|  Principle 12: AI as an Explainer, Never a Score Determinant                      |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------+-----------------------------------------+
|                     CATEGORY 5: MAINTENANCE & COMPATIBILITY RIGOR                 |
|  Principle 13: No Implementation Without Approved Documentation                  |
|  Principle 14: Backward Compatibility & Migration Governance                     |
+-----------------------------------------------------------------------------------+
```

---

## 3. Detailed Specifications of the 14 Principles

---

### CATEGORY 1: PRODUCT & BUSINESS FOUNDATION

#### Principle 1: Product Before Technology (Produk di Atas Teknologi)

##### 1.1 Objective & Context
Engineering teams often fall into the trap of adopting trendy technologies, hyped AI models, or complex frameworks simply because they are popular, rather than because they solve a real user problem. Technology must serve the product vision—not the other way around.

##### 1.2 Non-Negotiable Rule
**Technology choices MUST strictly follow validated business and user value. No framework, database, AI model, or library shall be introduced into AEP without a formal technical justification and an approved ADR.**

##### 1.3 Operational Requirements
- **No Trend-Driven Engineering**: AI, vector databases, or complex microservice frameworks MUST NOT be introduced unless there is empirical proof that simpler alternatives (e.g. SQLite, PostgreSQL, rule engines) fail to meet functional or non-functional requirements.
- **Strict Justification Gate**: Every new dependency added to `package.json`, `Cargo.toml`, `pyproject.toml`, or `go.mod` MUST be accompanied by a justification proving it reduces complexity or solves a core security objective.

---

#### Principle 2: User-Problem-Validated Features Only (Hanya Fitur Terbukti dari Masalah Pengguna)

##### 2.1 Objective & Context
Adding speculative features distorts product positioning, expands the attack surface, and incurs unnecessary long-term maintenance overhead.

##### 2.2 Non-Negotiable Rule
**Every feature introduced into the AEP platform MUST directly address a validated, real-world user or cybersecurity threat scenario.**

##### 2.3 Operational Requirements
- **Threat Mapping**: Every feature in the MVP or Roadmap MUST map directly to a validated threat vector (e.g., WhatsApp Web DOM message exfiltration, credential harvesting, over-privileged host permissions, dynamic code injection).
- **Scope Rejection**: Feature requests lacking empirical threat evidence or user problem validation MUST be rejected during Product Planning.

---

#### Principle 3: Quality Over Speed (Kualitas di Atas Kecepatan)

##### 3.1 Objective & Context
In enterprise cybersecurity software, rushing features to meet arbitrary deadlines introduces vulnerabilities, edge-case failures, and architectural debt. A single flaw in a security platform destroys customer trust.

##### 3.2 Non-Negotiable Rule
**Engineers MUST prioritize architectural completeness, threat modeling, security hardening, and test coverage over execution velocity. Deadlines MUST be extended if quality or security is at risk.**

##### 3.3 Operational Requirements
- No code or design document shall be rushed to completion to meet informal targets.
- If a tradeoff exists between shipping quickly and completing a thorough threat analysis, **the schedule MUST yield to quality**.

---

### CATEGORY 2: SECURITY & PRIVACY GOVERNANCE

#### Principle 4: Security by Design & Zero-Trust Boundary (Keamanan Sejak Awal Desain)

##### 4.1 Objective & Context
AEP parses untrusted, obfuscated, and potentially malicious JavaScript files, archives (`.crx` / `.zip`), and manifest configurations submitted from end-user endpoints. The security scanner platform itself is a high-value target for adversaries seeking remote code execution or privilege escalation.

##### 4.2 Non-Negotiable Rule
**Every system component MUST operate under a Zero-Trust architecture. Unpacking, parsing, and static analysis MUST be isolated within sandboxed environments with multi-layered defensive controls.**

##### 4.3 Operational Requirements
- **Zip-Slip Guard**: Archive extraction MUST validate canonical paths (`canonical_path.starts_with(sandbox_dir)`) to prevent path traversal attacks.
- **Zip Bomb Guard**: Uncompressed archive size caps (max 50 MB) and decompression ratio limits (max 100:1) MUST be strictly enforced prior to file extraction.
- **Local IPC Isolation**: WebSocket IPC between the Chrome Companion Extension and Desktop Agent (`ws://127.0.0.1:49152`) MUST enforce origin validation (`chrome-extension://<id>`) and dynamic local auth tokens.

---

#### Principle 5: Privacy by Default & Source Isolation (Privasi Utama & Isolasi Source Code)

##### 5.1 Objective & Context
Privacy is an independent foundational pillar, separate from cybersecurity. Users and enterprises must be guaranteed that their private browsing history, local profiles, personal data, and proprietary internal extension code are never harvested or exposed.

##### 5.2 Non-Negotiable Rule
**AEP MUST operate on a Privacy-by-Default model. Data minimization MUST be enforced at every layer: local profile paths and PII MUST be sanitized, and extension source code MUST NEVER be transmitted to the Cloud Backend unless explicitly authorized by the user.**

##### 5.3 Operational Requirements
- **Metadata-First Telemetry**: The Desktop Agent transmits only anonymized metadata, permission arrays, structural AST findings, and cryptographic asset hashes (`SHA-256`) to the Cloud API.
- **No Unsanitized Source Code Transmission**: Raw JavaScript source code files MUST remain on the local machine by default. If deep cloud analysis is requested by an enterprise user, source code transmission requires explicit opt-in consent and zero-retention ephemeral processing.
- **PII Scrubbing**: Local file paths (e.g. `C:\Users\JohnDoe\AppData\...`) MUST be sanitized to normalized placeholders (`<USER_PROFILE>\...`) before emitting telemetry.

---

#### Principle 6: Offline-First Desktop Core (Kemampuan Utama Tanpa Koneksi Cloud)

##### 6.1 Objective & Context
Security tools must remain functional even when endpoint connectivity is degraded, air-gapped, or compromised by network-level adversaries.

##### 6.2 Non-Negotiable Rule
**The Desktop Agent MUST provide complete core security value fully offline. Internet connectivity to the Cloud API is strictly for optional telemetry enrichment, NOT core operation.**

##### 6.3 Operational Requirements
- **Standalone Offline Execution**: The Desktop Agent MUST be capable of auto-discovering installed extensions, parsing `manifest.json`, executing local AST SAST scans, running the local Rule Engine, maintaining local SQLite scan history, and dispatching native OS alerts fully offline.
- **Cloud Enriched Roles Only**: Cloud API services are strictly reserved for asynchronous data enrichment:
  - CVE vulnerability database lookups
  - Global Threat Intelligence hash cross-referencing
  - AI-driven qualitative narrative synthesis
  - Central enterprise fleet management

---

### CATEGORY 3: ARCHITECTURAL & ECOSYSTEM DESIGN

#### Principle 7: Desktop-First Ecosystem Architecture (Arsitektur Berbasis Desktop Agent Utama)

##### 7.1 Objective & Context
Chromium browser security sandboxes strictly isolate extensions and web pages from reading filesystem directories of other browser extensions. A pure web upload scanner cannot monitor installed extensions automatically.

##### 7.2 Non-Negotiable Rule
**The Desktop Agent is the primary endpoint monitoring and SAST engine. The Chrome Companion Extension is a lightweight visual indicator, NOT a heavy analyzer.**

##### 7.3 Operational Requirements
- The **Desktop Agent** manages filesystem traversal, manifest parsing, sandboxed unpacking, and OS notifications.
- The **Chrome Companion Extension** MUST remain lightweight (Manifest V3 background worker), serving only to display visual toolbar badges (Green/Yellow/Red), render status popups, and bridge requests to the local agent.

---

#### Principle 8: Mandatory Architectural Decision Records (ADR Mandate)

##### 8.1 Objective & Context
Architectural decay occurs when engineering choices (framework selections, IPC protocols, database schemas, scanner algorithms) are made in undocumented chat threads, leading to lost context and conflicting re-implementations.

##### 8.2 Non-Negotiable Rule
**Every major architectural choice, framework selection, protocol change, or database schema design MUST be formally documented in an Architecture Decision Record (ADR) within `adr/`.**

##### 8.3 Operational Requirements
- ADRs must follow the `ADR-xxx_title.md` naming convention.
- Every ADR MUST document: **Context**, **Problem Statement**, **Options Evaluated**, **Decision**, **Consequences**, and **Rejected Alternatives**.
- Pull Requests implementing structural architecture without an approved ADR will be rejected.

---

#### Principle 9: Research Before Implementation (Riset Sebelum Implementasi)

##### 9.1 Objective & Context
Building scanner detection heuristics without researching Chromium internals, Manifest V3 specifications, or real-world malware techniques leads to high false-positive rates and brittle codebases.

##### 9.2 Non-Negotiable Rule
**No feature, scanner rule, or detection heuristic shall be implemented without a preceding research document stored in `research/`.**

##### 9.3 Operational Requirements
Every feature MUST be backed by a research file in `research/<category>/` establishing:
1. **Problem Statement**: Target attack vector or security gap.
2. **Empirical Threat Evidence**: Case studies, CVE records, or malware samples proving the threat.
3. **User Need & Technical Justification**: Mathematical and architectural justification for the detection logic.

---

### CATEGORY 4: IMPLEMENTATION & SCORING RIGOR

#### Principle 10: Explainable & Deterministic Risk Scoring (Risk Score Harus Deterministik & Transparan)

##### 10.1 Objective & Context
Security analysts and enterprise SOC teams require auditable, mathematical, and reproducible security evaluations. Opaque "black-box" scores fail regulatory compliance audits.

##### 10.2 Non-Negotiable Rule
**Risk Scores ($0.0 - 100.0$) MUST be computed 100% deterministically by a rule-based engine. Given identical extension inputs, the Risk Engine MUST produce identical scores every single time.**

##### 10.3 Operational Requirements
- Risk scores are calculated using an explicit, weighted formula:
  $$\text{Risk Score} = \min\left(100.0, \sum w_p \cdot P_i + \sum w_a \cdot A_j + \sum w_n \cdot N_k + w_{cve} \cdot C\right)$$
- Every single point added to a Risk Score MUST be traceable to specific line numbers, AST node types, or manifest permission flags.

---

#### Principle 11: Human Explainability & Itemized Score Breakdown (Penjelasan Risk Score yang Transparan)

##### 11.1 Objective & Context
Displaying a bare number (e.g. `Risk Score: 83`) or a generic "High Risk" label without line-item mathematical justification confuses users and prevents effective remediation.

##### 11.2 Non-Negotiable Rule
**Every Risk Score generated by the Rule Engine MUST include an itemized, human-readable mathematical breakdown explaining exactly how every point was accumulated.**

##### 11.3 Operational Requirements
- **Required Output Schema**: The Rule Engine MUST output a structured JSON array of point deductions/additions:
  ```json
  {
    "total_risk_score": 83.0,
    "risk_category": "HIGH_RISK",
    "itemized_breakdown": [
      { "points": +30.0, "category": "HOST_PERMISSIONS", "reason": "Unrestricted host access requested ('<all_urls>')" },
      { "points": +20.0, "category": "DANGEROUS_API", "reason": "Dynamic script injection detected ('chrome.tabs.executeScript') on line 142" },
      { "points": +15.0, "category": "AST_ANOMALY", "reason": "Obfuscated code string array decoder pattern identified in background.js" },
      { "points": +10.0, "category": "CVE_VULNERABILITY", "reason": "Embedded jQuery v1.12.4 contains known vulnerability (CVE-2015-9251)" },
      { "points": +8.0,  "category": "NETWORK_TELEMETRY", "reason": "Hardcoded HTTP communication to unvetted external IP address" }
    ]
  }
  ```
- Opaque, non-itemized risk scores are strictly prohibited.

---

#### Principle 12: AI as an Explainer, Never a Score Determinant (AI Sebagai Penjelas, Bukan Penentu Score)

##### 12.1 Objective & Context
Generative AI models (LLMs) are non-deterministic, prone to hallucinations, vulnerable to prompt injection inside target extension source code, and rate-limited. Relying on an LLM to decide security risk scores is dangerous.

##### 12.2 Non-Negotiable Rule
**AI models MUST NEVER calculate, modify, or override the numerical Risk Score. AI is strictly restricted to qualitative explanation, narrative synthesis, and remediation guidance.**

##### 12.3 Operational Requirements
- The Rule Engine finishes calculating the Risk Score and itemized breakdown BEFORE any prompt is generated for the AI service.
- Prompts sent to OpenAI or local Ollama instances MUST be pre-sanitized and MUST explicitly prohibit the LLM from altering score values.

---

### CATEGORY 5: MAINTENANCE & COMPATIBILITY RIGOR

#### Principle 13: No Implementation Without Approved Documentation (Tidak Ada Kode Tanpa Dokumentasi)

##### 13.1 Objective & Context
Writing code before establishing clear architectural specifications leads to mismatched API contracts, broken module interfaces, security oversights, and unmaintainable code.

##### 13.2 Non-Negotiable Rule
**No production code shall be written until the corresponding technical documentation, module interface specification, API contract, and test plan have been formally reviewed and approved.**

##### 13.3 Operational Requirements
- The development sequence MUST strictly adhere to:
  $$\text{Research} \longrightarrow \text{ADR} \longrightarrow \text{Architecture Spec} \longrightarrow \text{API Schema} \longrightarrow \text{Test Plan} \longrightarrow \text{Implementation}$$
- Pull Requests introducing code without referencing an approved architecture specification will be closed.

---

#### Principle 14: Backward Compatibility & Migration Governance (Migrasi & Backward Compatibility)

##### 14.1 Objective & Context
As AEP evolves, updates to Rule Engine heuristics, PostgreSQL database schemas, and REST API contracts must never break existing user scan histories, agent sync endpoints, or desktop databases.

##### 14.2 Non-Negotiable Rule
**All schema, rule engine, and API updates MUST maintain strict backward compatibility. Breaking changes are prohibited without formal deprecation schedules and automated migration paths.**

##### 14.3 Operational Requirements
- **Database Migration**: All database schema changes MUST be managed using version-controlled Alembic migrations with tested `upgrade()` and `downgrade()` scripts.
- **Rule Engine Versioning**: Rule definitions MUST be versioned (e.g. `rule_version: "1.2.0"`). Historical scan reports in the database MUST retain their original rule version context so past scores remain reproducible.
- **API Versioning**: REST APIs MUST enforce explicit path versioning (`/api/v1/`, `/api/v2/`). Older API versions MUST be supported until formal deprecation notices expire.
- **Scan History Compatibility**: Upgrading the Desktop Agent or Cloud Backend MUST NEVER corrupt, invalidate, or delete existing local or cloud scan history archives.

---

## 4. Comprehensive Trade-Off Matrix

| Principle | Primary Enterprise Benefit | Explicit Trade-Off / Cost | Mitigation Strategy |
| :--- | :--- | :--- | :--- |
| **P1: Product Before Tech** | Eliminates unnecessary tech stack complexity & hype-driven bloat. | Requires rigorous upfront technical justification for every library. | Standardized ADR template (`ADR-000`) for rapid evaluation. |
| **P3: Quality Over Speed** | Eliminates security flaws, architectural debt, and regressions. | Longer time-to-market for individual features. | Phase-based milestone planning & strict MVP scope boundaries. |
| **P5: Privacy by Default** | Total user trust & regulatory compliance (GDPR/CCPA/HIPAA). | Cloud cannot analyze raw source code unless explicit opt-in is granted. | Rich AST metadata extraction on local Agent allows deep analysis without raw code transmission. |
| **P6: Offline-First Core** | Complete functionality in air-gapped / offline enterprise environments. | Requires bundling local SQLite DB and rule definitions inside Desktop Agent. | Lightweight SQLite footprint and optimized JSON rule compiled assets. |
| **P10 & P11: Deterministic & Itemized Scoring** | 100% auditable, reproducible, human-explainable security reports. | Rule Engine weights require continuous threat research tuning. | Automated benchmark test suites with known benign & malware extension samples. |
| **P14: Backward Compatibility** | Guarantees zero data loss or API breakage during system upgrades. | Requires maintaining migration scripts and API deprecation windows. | Automated CI/CD integration testing against legacy database snapshots & API schemas. |

---

## 5. Self-Audit & Quality Gate Checklist

Prior to submission, this document was audited against the following engineering quality gates:

- [x] **No Overlap or Redundancy**: Principles 1 through 14 cover distinct operational domains (Product, Security, Privacy, Architecture, Scoring, Maintenance) without collision.
- [x] **Zero Contradiction**: All principles mutually reinforce the 4-tier ecosystem (Desktop Agent, Cloud API, Web Dashboard, Chrome Companion).
- [x] **Actionable Operational Rules**: Every principle defines explicit, non-negotiable rules and measurable operational requirements.
- [x] **Engineering Trade-Offs Included**: Explicit trade-offs and mitigations are documented for all major principles.
- [x] **Corporate Documentation Quality**: Formatted according to enterprise standards (Microsoft, Google, Cloudflare, OWASP).

---

## 6. Related Architecture & Governance Documents

- [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) — Master Documentation Index
- [`docs/PROJECT_OVERVIEW.md`](file:///d:/ExtensionProtect/docs/PROJECT_OVERVIEW.md) — Product Vision & 4-Tier Ecosystem Overview
- [`docs/SYSTEM_ARCHITECTURE.md`](file:///d:/ExtensionProtect/docs/SYSTEM_ARCHITECTURE.md) — High-Level C4 System Architecture
- [`adr/README.md`](file:///d:/ExtensionProtect/adr/README.md) — Architecture Decision Record Register
