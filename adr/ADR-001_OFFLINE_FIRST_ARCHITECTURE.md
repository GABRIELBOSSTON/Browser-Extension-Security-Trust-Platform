# Architecture Decision Record — ADR-001: Offline-First Desktop Agent Architecture

---

## Document Metadata

| Metadata Field | Record Detail |
| :--- | :--- |
| **ADR ID** | `ADR-001` |
| **ADR Title** | Offline-First Desktop Agent Architecture & Local Analysis Boundary |
| **Status** | **Accepted (Draft)** |
| **Date** | 2026-08-04 |
| **Decision Drivers** | Privacy-by-Default, Local Endpoint Isolation, Zero Black-Box Scoring, Offline Independence |
| **Authors** | Lead Software Architect & Lead Security Engineer |
| **Reviewers** | Chief Technology Officer (CTO) / Security Architect |
| **Related Documents** | [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md), [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md), [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md), [`docs/USE_CASE.md`](file:///d:/ExtensionProtect/docs/USE_CASE.md) |

---

## 1. Context & Problem Statement

### 1.1 The Privacy & Latency Challenge of Cloud-First Extension Security
Standard web-based security tools and traditional SAST platforms operate on a **Cloud-First Paradigm**, requiring users to upload local source code, configuration manifests, and archive packages (`.crx` / `.zip`) to remote cloud servers for unpacking, parsing, and static analysis.

When applied to browser extension security, a pure Cloud-First architecture introduces severe operational, security, and privacy liabilities:

1. **Severe User Privacy Concerns**: Browser extensions contain intimate access patterns, proprietary internal source code, and private user configurations. Uploading raw extension source code or un-sanitized endpoint file paths to third-party cloud servers violates fundamental privacy principles (GDPR, CCPA) and destroys user trust.
2. **Chromium Sandbox Boundaries**: Web browsers strictly isolate web pages and cloud services from reading local filesystem directories where installed browser extensions reside. A pure cloud application cannot automatically discover or monitor installed extensions across Chrome, Edge, Brave, and Opera profiles.
3. **Network Dependency & Latency**: Forcing every extension scan to upload multi-megabyte package archives over the network creates high latency ($>30$ seconds per scan) and renders the tool useless when network connectivity is degraded, air-gapped, or compromised by network adversaries.
4. **Cloud Infrastructure Cost Explosion**: Processing static analysis (AST parsing, regex scans, file extraction) for millions of local extension updates on cloud infrastructure creates unsustainable server compute costs during early product stages.

---

## 2. Decision Statement

During **Architecture Meeting #1**, the engineering leadership team formally resolved that:

> **The core operational product of Antigraviiti Extension Protect (AEP) is the local DESKTOP AGENT. The application MUST maintain complete core security functionality fully offline without requiring Internet connectivity. The Cloud Platform operates strictly as an optional data enrichment layer.**

---

## 3. Decision Details & Component Boundary Division

```
+-----------------------------------------------------------------------------------+
|                        OFFLINE-FIRST ARCHITECTURE BOUNDARY                        |
+-----------------------------------------------------------------------------------+
|                                                                                   |
|  LOCAL ENDPOINT (DESKTOP AGENT CORE - 100% OFFLINE)                               |
|  ├── Extension Profile Auto-Discovery                                             |
|  ├── Manifest V2/V3 Parsing & Structure Inspection                               |
|  ├── Host & API Permission Risk Scoring                                           |
|  ├── Static Application Security Testing (AST & Secret SAST)                      |
|  ├── Deterministic Risk Score Engine (0.0 - 100.0)                                |
|  ├── Local SQLite History & Persistence Archive                                   |
|  └── Local OS Native Desktop Alert Banners                                        |
|                                                                                   |
|  -------------------------------------------------------------------------------  |
|                                                                                   |
|  CLOUD PLATFORM (ASYNC ENRICHMENT LAYER - OPTIONAL CONNECTIVITY)                  |
|  ├── Global Threat Intelligence Hash Cross-Referencing (SHA-256)                  |
|  ├── Outdated Library Vulnerability Database (CVE Lookups)                        |
|  ├── AI Qualitative Security Narrative Synthesis                                  |
|  ├── Web Console Dashboard Analytics                                              |
|  └── Asynchronous Security Rule Updates & Telemetry Sync                         |
|                                                                                   |
+-----------------------------------------------------------------------------------+
```

### 3.1 Local Desktop Agent Responsibilities (Offline Core)
The local Desktop Agent operates as a self-contained security engine on the user's operating system (Windows, macOS, Linux). It performs:
- **Auto-Discovery**: Traverses local browser profile paths to discover installed extension folders.
- **Manifest Parsing**: Reads `manifest.json` files and inspects Manifest V2/V3 structures.
- **Permission Analysis**: Calculates permission risk weights for host patterns (`<all_urls>`) and dangerous APIs (`cookies`, `webRequest`, `scripting`).
- **Static Analysis (SAST)**: Executes local JavaScript AST parsing to detect dynamic code execution (`eval()`, `Function()`, `atob()`), string array obfuscation, and exposed API keys.
- **Deterministic Risk Engine**: Calculates normalized Risk Scores ($0.0 - 100.0$) and generates itemized point breakdowns.
- **Local Persistence**: Stores scan history, reports, and rule definitions locally in an embedded **SQLite** database.
- **Local OS Notifications**: Fires native operating system desktop alerts when High-Risk extensions ($\ge 70.0$) are detected.

### 3.2 Cloud Platform Responsibilities (Optional Enrichment)
The Cloud API executes strictly asynchronous, non-blocking enrichment services:
- Cross-referencing asset `SHA-256` hashes against known malware databases.
- Auditing embedded JavaScript library version numbers against NVD CVE vulnerability records.
- Synthesizing qualitative plain-language AI narratives using OpenAI API or local Ollama fallbacks.
- Serving Web Dashboard analytics and administrative fleet views.
- Delivering periodic security rule updates.

### 3.3 Strict Privacy & Infrastructure Governance Rules
1. **Offline Usability Guarantee**: The Desktop Agent MUST remain 100% functional (discovery, SAST, risk scoring, local alerts) when network interfaces are disabled or air-gapped.
2. **Zero Default Source Code Upload**: Raw JavaScript source code files MUST NEVER be uploaded to Cloud Backend services by default.
3. **Local Embedded Persistence**: Embedded **SQLite** is selected as the mandatory local database engine for the Desktop Agent due to its zero-configuration footprint, serverless architecture, and local file storage reliability.
4. **Cloud Infrastructure Cost Policy**: Cloud Backend services for MVP MUST utilize free-tier / serverless infrastructure (e.g. Supabase, Render, Railway, Vercel) to maintain zero server overhead prior to enterprise commercialization.

---

## 4. Alternatives Considered

```
+-----------------------------------------------------------------------------------+
|                           EVALUATION OF ALTERNATIVES                              |
+-----------------------------------------------------------------------------------+
|  CRITERIA               | OPTION A: CLOUD-FIRST         | OPTION B: OFFLINE-FIRST |
|                         | (Pure Web Upload SAST)        | (Desktop Agent Core)    |
+-------------------------+-------------------------------+-------------------------+
|  User Privacy           | Low (Uploads code to cloud)   | High (Code stays local) |
|  Extension Discovery   | Manual upload required        | 100% Automated local    |
|  Offline Availability   | Zero (Fails without internet) | 100% Fully functional   |
|  Scan Performance       | High Latency (>30s upload)    | Sub-second (<3s local)  |
|  Cloud Infrastructure   | High ($$$ Server compute)    | Minimal (Free-tier MVP) |
|  Enterprise Trust       | Low (Legal privacy risk)      | High (Zero data leak)   |
+-----------------------------------------------------------------------------------+
```

### Option A: Cloud-First Architecture (Pure Web Upload SAST)
- **Description**: Users or agents upload raw `.crx` / `.zip` files to a cloud server where microservices extract archives, run AST parsers, calculate scores, and return JSON responses.
- **Rejected Rationale**:
  - Requires uploading proprietary or private extension source code to third-party servers.
  - Cannot auto-discover installed extensions across local browser profiles due to browser sandbox limits.
  - Generates high network bandwidth consumption and multi-megabyte payload upload latencies.
  - Incurs massive server infrastructure compute costs during free tier usage.

### Option B: Offline-First Architecture (Desktop Agent Core) — SELECTED
- **Description**: The Desktop Agent contains a complete local SAST parser and Rule Engine, performing analysis locally on endpoint hardware. Cloud endpoints are queried asynchronously for optional metadata enrichment (CVEs, AI explanations).
- **Selection Justification**: Satisfies 100% of our constitutional principles ([`PROJECT_PRINCIPLES.md`, Principles 5, 6, 10](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md#principle-5-privacy-by-default--source-isolation-privasi-utama--isolasi-source-code)). Guarantees user privacy, eliminates server compute costs, provides instant sub-second scan performance, and enables total offline independence.

---

## 5. Architectural Advantages & Benefits

1. **Uncompromising User & Corporate Privacy**: Extension source code and local user profile paths remain isolated on the endpoint device, eliminating legal privacy liabilities under GDPR, CCPA, and HIPAA.
2. **Instant Sub-Second Scan Performance**: Local AST parsing and manifest inspection execute directly in endpoint memory, eliminating multi-megabyte network upload latencies (<3.0s total scan time vs >30.0s cloud upload).
3. **100% Air-Gapped & Offline Usability**: High-security, financial, government, and educational users can audit extensions in secure offline environments without internet access.
4. **Zero-Configuration Auto-Discovery**: The Desktop Agent reads local Chromium profile directories directly on disk, solving the browser sandbox limitation that prevents web applications from discovering installed extensions.
5. **Zero Infrastructure Cost Explosion**: Offloading processing to client endpoint hardware reduces cloud server compute requirements by >95%, allowing AEP to operate on free-tier cloud infrastructure during initial releases.

---

## 6. Disadvantages & Trade-Offs

1. **OS Binary Maintenance Overhead**: Developing and supporting native Desktop Agent binaries across multiple operating systems (Windows, macOS, Linux) requires maintaining platform-specific packaging and MDM deployment scripts.
2. **Local Rule Update Synchronization**: Rule engine heuristics stored locally on the Desktop Agent require background update mechanisms to receive new threat detection patterns when internet connectivity is available.
3. **Endpoint Resource Consumption Constraints**: The Desktop Agent must be highly optimized (<30 MB RAM, <1% CPU) to ensure background scanning never degrades endpoint performance or triggers user complaints ([`ENGINEERING_PRINCIPLES.md`, Section 11](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md#11-engineering-metrics--quality-indicators)).

---

## 7. Technical Risk Analysis

| Risk ID | Technical Risk Description | Risk Severity | Likelihood |
| :--- | :--- | :--- | :--- |
| **RISK-01** | **Outdated Local Security Rules**: Offline agents running stale local rule sets may miss newly identified extension threat vectors. | Medium | High |
| **RISK-02** | **Desktop OS Permission Restrictions**: Endpoint security policies (e.g. macOS Gatekeeper, Windows AppLocker) may restrict Desktop Agent directory access. | High | Medium |
| **RISK-03** | **SQLite Database Corruption**: Unexpected power loss or hard reboot during local scan write ops could corrupt the local scan history database. | Medium | Low |

---

## 8. Risk Mitigation Strategies

- **Mitigation for RISK-01 (Stale Rules)**: Implement an asynchronous background Rule Update Worker that checks for rule updates whenever internet connectivity is restored. Rule definitions are compiled into lightweight JSON assets with semantic versioning (`rule_version: "1.2.0"`).
- **Mitigation for RISK-02 (OS Permissions)**: Build the Desktop Agent to run entirely under standard user-space privileges without requiring Administrator/Root access or kernel driver hooks.
- **Mitigation for RISK-03 (SQLite Corruption)**: Enable SQLite Write-Ahead Logging (WAL) mode with atomic transaction commits and automated backup snapshots on startup.

---

## 9. Future Release Impact & Roadmap Alignment

This decision directly underpins and enables our multi-release product strategy:

- **Version 1.0 MVP Core**: Fully validates the standalone Offline-First Desktop Agent (local auto-discovery, sandboxed extraction, local AST SAST, SQLite persistence, local OS alerts) for non-technical home users ([`USER_PERSONA.md`, Persona 1](file:///d:/ExtensionProtect/docs/USER_PERSONA.md#persona-1-everyday-home-user-primary-mvp-target)).
- **Version 2.0 (Cloud Enrichment)**: Integrates optional cloud API lookups for CVE cross-referencing and threat intelligence hash matching without altering the offline local core scanner.
- **Version 3.0 (Chrome Companion)**: Uses local WebSocket IPC (`ws://127.0.0.1:49152`) to connect the toolbar companion extension directly to the local Desktop Agent.
- **Version 5.0 (Enterprise Fleet Governance)**: Enables Desktop Agents deployed via Intune/Jamf to stream anonymized telemetry to the central Cloud Console while preserving local offline processing safety.

---

## 10. References & Approval Sign-Off

### Source Document Traceability
- [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md) — Principles 5 (Privacy-by-Default), 6 (Offline-First Core), 7 (ADR Mandate)
- [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) — Section 7 (Offline-First Independence)
- [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md) — Requirement `REQ-DISC-01`, Section 5.3 (NFR Offline Capability)
- [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md) — Feature `FEAT-DISC-001`, `FEAT-PRIV-001`
- [`docs/USE_CASE.md`](file:///d:/ExtensionProtect/docs/USE_CASE.md) — Use Case `UC-DISC-01`, `UC-RISK-01`

### Formal Sign-Off
- **Architectural Decision Owner**: Lead Software Architect & Lead Security Engineer
- **Approved By**: Chief Technology Officer (CTO) / Security Architect  
- **Approval Date**: 2026-08-04  
- **Status**: **Accepted (Draft)**
