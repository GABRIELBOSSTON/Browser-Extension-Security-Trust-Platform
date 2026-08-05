# Use Case Specifications — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Authoritative Use Case Specification |
| **Document ID** | `DOC-UC-001` |
| **Current Status** | DRAFT — Pending CTO Architecture Readiness Review |
| **Document Version** | `1.0.0` |
| **Document Owner** | Lead Systems Analyst & Software Architect |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Target Audience** | Software Architects, Systems Analysts, Engineering Leads, and QA Engineers |
| **Source References** | [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md), [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md), [`docs/USER_PERSONA.md`](file:///d:/ExtensionProtect/docs/USER_PERSONA.md), [`docs/CUSTOMER_JOURNEY.md`](file:///d:/ExtensionProtect/docs/CUSTOMER_JOURNEY.md), [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) |

---

## 2. Document Purpose & Architectural Readiness

The **Use Case Specification** bridges **Product Requirements & Feature Catalogs** with formal **System Architecture & Software Design**. It defines the step-by-step interaction flows between actors (users, system daemons, enterprise admins) and platform capabilities to achieve specific business goals.

Every use case defined in this document is derived 100% from approved feature specifications in [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md). This document contains zero UI layout details, database schemas, API specs, or code implementations.

```
+-----------------------------------------------------------------------------------+
|                        PRODUCT STRATEGY & REQUIREMENTS                            |
|    PRODUCT_REQUIREMENTS.md  <───>  FEATURE_CATALOG.md  <───> USER_PERSONA.md      |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v (Defines Behavioral Flows)
+-----------------------------------------+-----------------------------------------+
|                       USE CASE SPECIFICATION (DOC-UC-001)                         |
|                         Location: docs/USE_CASE.md                                |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v (Governs Architecture & Module Design)
+-----------------------------------------+-----------------------------------------+
|                        TECHNICAL SYSTEM ARCHITECTURE                              |
|   SYSTEM_ARCHITECTURE | SOFTWARE_ARCHITECTURE | SCANNER_ENGINE | MODULE_DESIGN    |
+-----------------------------------------------------------------------------------+
```

---

## 3. Textual Use Case Diagram (Actor-to-Use-Case Mapping)

```
=====================================================================================
ACTOR                    USE CASE DOMAIN                  USE CASE ID & NAME
=====================================================================================
Maya Lin (Home User) ───► [Extension Discovery] ────────► UC-DISC-01: Auto-Discover Local Extensions
                     ───► [Risk & AI Review] ───────────► UC-RISK-01: Review Risk & AI Breakdown
                     ───► [Local Alerts] ───────────────► UC-NOTIF-01: Receive OS Risk Alert

Alex Chen (Developer)───► [Manual Package Scan] ────────► UC-SCAN-01: Audit Build Package (CRX/ZIP)
                     ───► [SAST Code Inspection] ───────► UC-SAST-01: Inspect Dynamic Code & Secrets
                     ───► [Local Scan History] ─────────► UC-HIST-01: Compare Build Version History

Marcus Vance (Hunter)───► [Manual Package Scan] ────────► UC-SCAN-01: Audit Build Package (CRX/ZIP)
                     ───► [AST Forensic Triage] ────────► UC-SAST-01: Inspect Dynamic Code & Secrets

Dr. Aris Thorne (Res)───► [Threat Intelligence] ────────► UC-INTEL-01: Cross-Reference Hashes & CVEs

Sarah Jenkins (SOC) ───► [Alerting & Reporting] ────────► UC-NOTIF-01: Receive High-Risk Alert
                     ───► [Report Exporting] ───────────► UC-REPORT-01: Export Executive PDF Report

David Ross (CISO)    ───► [Enterprise Governance] ──────► UC-FLEET-01: Audit Fleet Extension Posture

Robert Kowalski (Admin)► [Silent MDM Deployment] ───────► UC-AGENT-01: Deploy Silent Agent via MDM
=====================================================================================
```

---

## 4. Itemized Use Case Specifications

---

### UC-DISC-01: Auto-Discover Installed Browser Extensions

- **Use Case ID**: `UC-DISC-01`
- **Use Case Name**: Auto-Discover Installed Browser Extensions
- **Goal**: Automatically scan local user profile directories across Chromium browsers to compile a comprehensive, local extension inventory without manual user file upload.
- **Primary Actor**: Maya Lin (Home User)
- **Supporting Actors**: Desktop Agent Background Daemon
- **Preconditions**: Desktop Agent is installed and running on the endpoint operating system.
- **Trigger**: Desktop Agent launches or detects a local browser profile extension installation event.
- **Main Success Scenario**:
  1. Desktop Agent initiates background scanning of standard Chromium browser profile paths (Chrome, Edge, Brave, Opera).
  2. System locates active extension installation folders and reads `manifest.json` files.
  3. System extracts extension ID, version, name, requested permissions, background scripts, and content scripts.
  4. System computes SHA-256 cryptographic asset hashes for all `.js` and binary assets.
  5. System sanitizes local file paths (removing user profile names).
  6. System displays the compiled local extension inventory to the user.
- **Alternative Flows**:
  - *Alt 1*: User has only one browser installed (e.g. Chrome). System skips non-existent browser profile paths and compiles inventory cleanly.
- **Exception Flows**:
  - *Exc 1 (Permission Denied)*: Desktop Agent lacks read permission to a specific browser directory. System logs local permission warning, skips restricted directory, and displays discovered extensions from accessible profiles.
- **Postconditions**: Installed extension inventory is compiled and ready for static analysis and risk scoring.
- **Related Features**: `FEAT-DISC-001` ([`FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md#feat-disc-001-local-installed-extension-auto-discovery))
- **Related Requirements**: `REQ-DISC-01` ([`PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md#3-requirement-traceability-matrix-rtm))
- **Business Rules**: Rule 4 (Privacy Precedence — local user profile names MUST be sanitized).
- **Acceptance Criteria**: Auto-discovers 100% of extensions installed across default local profiles within 3 seconds of execution without prompting the user for file paths.

---

### UC-SCAN-01: Audit Standalone Extension Build Package (CRX/ZIP)

- **Use Case ID**: `UC-SCAN-01`
- **Use Case Name**: Audit Standalone Extension Build Package
- **Goal**: Safely unpack and statically analyze an uncompressed extension directory or archive package (`.crx` / `.zip`) to verify security prior to Web Store submission or triage.
- **Primary Actor**: Alex Chen (Extension Developer) / Marcus Vance (Bug Hunter)
- **Supporting Actors**: Sandboxed Extraction Engine, SAST Parser
- **Preconditions**: User has a local extension build package or `.crx` file available on disk.
- **Trigger**: User drops a `.crx` or `.zip` file into the scanner interface or selects an unpacked directory.
- **Main Success Scenario**:
  1. User initiates scan by providing local package path or dropping file into scanner.
  2. System validates file size (<50 MB) and verifies archive integrity.
  3. System extracts package files into an isolated, ephemeral sandbox directory.
  4. System verifies canonical paths (`canonical_path.starts_with(sandbox_dir)`) to prevent path traversal (Zip-Slip).
  5. System parses `manifest.json` and initiates AST static analysis across JavaScript assets.
  6. System calculates Risk Score ($0.0 - 100.0$) and displays line-item findings report.
  7. System deletes temporary sandboxed extraction directory upon scan completion.
- **Alternative Flows**:
  - *Alt 1*: Package is an unpacked folder. System bypasses archive extraction and executes SAST parser directly on target directory.
- **Exception Flows**:
  - *Exc 1 (Zip-Slip Attempt)*: Archive contains relative path traversal vectors (e.g. `../../system32/`). System aborts extraction immediately, logs security alert, and flags package as High Risk.
  - *Exc 2 (Zip Bomb)*: Uncompressed ratio exceeds 100:1. System aborts extraction immediately and logs decompression attack alert.
- **Postconditions**: Standalone package is audited; scan findings are displayed; ephemeral extraction files are purged.
- **Related Features**: `FEAT-DISC-002`, `FEAT-MANI-001`, `FEAT-SAST-001`
- **Related Requirements**: `REQ-DISC-01`, `REQ-SAST-01`
- **Business Rules**: Rule 4 (Privacy Precedence & Sandboxed Isolation).
- **Acceptance Criteria**: Unpacks and audits valid ZIP/CRX packages in <5 seconds while blocking path-traversal payload archives 100% of the time.

---

### UC-SAST-01: Inspect Dynamic Code Execution & Hardcoded Secrets

- **Use Case ID**: `UC-SAST-01`
- **Use Case Name**: Inspect Dynamic Code Execution & Hardcoded Secrets
- **Goal**: Perform static code analysis on JavaScript files to detect dangerous dynamic execution (`eval()`, `Function()`), base64 decoders (`atob()`), obfuscated string arrays, and exposed API keys.
- **Primary Actor**: Alex Chen (Extension Developer) / Marcus Vance (Bug Hunter)
- **Supporting Actors**: AST SAST Engine
- **Preconditions**: Extension files have been extracted and parsed by `UC-DISC-01` or `UC-SCAN-01`.
- **Trigger**: SAST Engine receives JavaScript file list for static analysis.
- **Main Success Scenario**:
  1. SAST Engine parses JavaScript source files into Abstract Syntax Trees (AST).
  2. System inspects AST nodes for dynamic execution calls (`eval()`, `Function()`, `setTimeout(string)`).
  3. System scans for string array decoders, high-entropy variable naming, and base64 payload calls (`atob()`).
  4. System scans for hardcoded cloud API keys (AWS, Stripe, OpenAI) and Webhook secret signatures.
  5. System inspects content scripts targeting sensitive domains (e.g. WhatsApp Web, banking portals) for DOM scraping sinks.
  6. System logs each finding with exact file name, line number, code snippet, and rule severity weight.
- **Alternative Flows**:
  - *Alt 1*: JavaScript file is minified. System applies AST entropy detection and flags minified obfuscation patterns.
- **Exception Flows**:
  - *Exc 1 (Unparseable Syntax)*: File contains corrupted or non-standard JS syntax. System flags file for manual security review with elevated risk weight rather than marking as "Safe".
- **Postconditions**: AST findings and hardcoded secret logs are compiled for Risk Engine calculation.
- **Related Features**: `FEAT-SAST-001`, `FEAT-SAST-002`, `FEAT-SAST-003`, `FEAT-SAST-004`
- **Related Requirements**: `REQ-SAST-01`, `REQ-SAST-02`, `REQ-DOM-01`
- **Business Rules**: Rule 1 & 5 (Deterministic scoring & Fail-Secure defaults).
- **Acceptance Criteria**: Identifies `eval()`, `atob()`, and exposed API keys in minified or un-minified JS files with >95% precision.

---

### UC-RISK-01: Review Risk Score & Itemized Breakdown

- **Use Case ID**: `UC-RISK-01`
- **Use Case Name**: Review Risk Score & Itemized Breakdown
- **Goal**: Compute a deterministic Risk Score ($0.0 - 100.0$) and present an itemized, line-by-line mathematical breakdown explaining every risk factor.
- **Primary Actor**: Maya Lin (Home User) / Clara Gomez (Student) / Sarah Jenkins (SOC)
- **Supporting Actors**: Deterministic Risk Engine
- **Preconditions**: Manifest inspection and SAST findings have been compiled by `UC-SAST-01`.
- **Trigger**: System completes static analysis phase.
- **Main Success Scenario**:
  1. Risk Engine evaluates manifest permissions, AST anomalies, DOM access sinks, and secret findings.
  2. Risk Engine applies explicit mathematical rule weights to compute the total Risk Score:
     $$\text{Risk Score} = \min\left(100.0, \sum \text{Rule Weights}\right)$$
  3. System assigns Risk Category: Low ($0-29.9$), Medium ($30-69.9$), or High ($70-100.0$).
  4. System compiles itemized point breakdown log detailing exact points added, category, file name, line number, and human-readable reason.
  5. System presents numerical Risk Score and itemized breakdown to the user.
- **Alternative Flows**:
  - *Alt 1*: Extension has zero risk factors. System returns Risk Score $0.0$ (Low Risk) with clean breakdown log.
- **Exception Flows**:
  - *Exc 1 (Scoring Anomaly)*: A rule weight calculation fails. System defaults to Fail-Secure elevated risk status and logs internal scoring exception.
- **Postconditions**: Risk score and itemized breakdown are rendered and saved to local scan history.
- **Related Features**: `FEAT-RISK-001`, `FEAT-RISK-002`
- **Related Requirements**: `REQ-RISK-01`, `REQ-EXPL-01`
- **Business Rules**: Rule 1 & 3 (100% Deterministic scoring & Itemized Score Transparency).
- **Acceptance Criteria**: Produces identical numerical scores and itemized breakdowns across 1,000 execution runs for identical extension packages.

---

### UC-AI-01: Synthesize Qualitative AI Security Explanation

- **Use Case ID**: `UC-AI-01`
- **Use Case Name**: Synthesize Qualitative AI Security Explanation
- **Goal**: Translate raw static analysis findings and itemized risk breakdowns into a clear, plain-language executive summary and actionable remediation guidance using AI.
- **Primary Actor**: Maya Lin (Home User) / David Ross (CISO)
- **Supporting Actors**: AI Explanation Engine (OpenAI API / Local Ollama)
- **Preconditions**: Risk Score and itemized breakdown have been computed by `UC-RISK-01`.
- **Trigger**: User requests plain-language explanation or system automatically generates summary for scan report.
- **Main Success Scenario**:
  1. System constructs a sanitized prompt containing pre-verified SAST findings, permission list, and calculated Risk Score.
  2. System enforces prompt-injection guards and anti-hallucination system instructions.
  3. System dispatches prompt to AI service (Cloud OpenAI or local Ollama).
  4. AI service returns structured qualitative narrative (Executive Summary, Technical Explanation, Remediation Action).
  5. System displays AI summary alongside the numerical Risk Score.
- **Alternative Flows**:
  - *Alt 1*: Device is offline. System uses local Ollama instance or falls back to template-based qualitative rules.
- **Exception Flows**:
  - *Exc 1 (AI API Failure / Rate Limit)*: Cloud AI API fails to respond. System falls back cleanly to static rule narrative template without blocking Risk Score display.
- **Postconditions**: Qualitative explanation is displayed and appended to scan report.
- **Related Features**: `FEAT-AI-001`
- **Related Requirements**: `REQ-AI-01`
- **Business Rules**: Rule 2 (AI MUST NEVER calculate or modify numerical Risk Scores).
- **Acceptance Criteria**: Generates plain-language summary without altering numerical score values or fabricating non-existent vulnerabilities.

---

### UC-NOTIF-01: Dispatch Local OS Native Risk Banners

- **Use Case ID**: `UC-NOTIF-01`
- **Use Case Name**: Dispatch Local OS Native Risk Banners
- **Goal**: Immediately alert the user via native operating system notifications when a newly installed extension or background update exceeds acceptable risk thresholds.
- **Primary Actor**: Maya Lin (Home User) / Robert Kowalski (IT Admin)
- **Supporting Actors**: Desktop Agent OS Notification Service
- **Preconditions**: Desktop Agent is running in system tray; an extension scan completes with Risk Score $\ge 70.0$ (High Risk).
- **Trigger**: Risk Engine finishes evaluation and detects High Risk score on installed extension.
- **Main Success Scenario**:
  1. System identifies extension with Risk Score $\ge 70.0$ (High Risk).
  2. System constructs OS native banner alert payload containing extension name, Risk Score, and plain-language warning message.
  3. System dispatches native OS notification banner (Windows Action Center / macOS Notification Center).
  4. User clicks notification banner.
  5. System opens scanner report interface highlighting the flagged extension.
- **Alternative Flows**:
  - *Alt 1*: Extension score is Medium ($30-69.9$). System logs finding silently without firing intrusive desktop popup.
- **Exception Flows**:
  - *Exc 1 (OS Notification Disabled)*: User disabled OS notifications. System logs alert locally and updates system tray icon to warning state.
- **Postconditions**: User is alerted to High Risk extension; notification event is logged.
- **Related Features**: `FEAT-NOTIF-001`
- **Related Requirements**: `REQ-NOTIF-01`
- **Business Rules**: Rule 4 (Privacy Precedence & Offline-first notification).
- **Acceptance Criteria**: Triggers native desktop notification within 2 seconds of detecting a High Risk extension installation or background update.

---

### UC-HIST-01: Manage Local Scan History & Version Timeline

- **Use Case ID**: `UC-HIST-01`
- **Use Case Name**: Manage Local Scan History & Version Timeline
- **Goal**: Persist scan reports locally to allow users to review historical audit logs, track score changes across extension updates, and operate fully offline.
- **Primary Actor**: Maya Lin (Home User) / Alex Chen (Developer)
- **Supporting Actors**: Local Storage Engine (SQLite)
- **Preconditions**: Scan report has been generated by `UC-RISK-01`.
- **Trigger**: Scan report generation completes.
- **Main Success Scenario**:
  1. System serializes scan report JSON (extension ID, version, timestamp, Risk Score, itemized breakdown, AI summary).
  2. System stores report in local SQLite database on the endpoint.
  3. User opens "Scan History" tab in scanner interface.
  4. System queries local SQLite database and renders historic scan timeline.
  5. User selects a past scan entry to view full historical audit report offline.
- **Alternative Flows**:
  - *Alt 1*: User re-scans updated extension version. System renders side-by-side score comparison timeline.
- **Exception Flows**:
  - *Exc 1 (Database Corrupted)*: Local SQLite file is corrupted. System backs up corrupted file, initializes clean local database, and logs recovery event.
- **Postconditions**: Historic scan reports remain accessible locally fully offline.
- **Related Features**: `FEAT-HIST-001`
- **Related Requirements**: `REQ-DISC-01`, `REQ-RISK-01`
- **Business Rules**: Rule 5 (Mandatory Backward Compatibility for historical scan data).
- **Acceptance Criteria**: Persists scan history locally in SQLite database; allows viewing past reports fully offline.

---

### UC-INTEL-01: Cross-Reference Asset Hashes & CVE Vulnerabilities

- **Use Case ID**: `UC-INTEL-01`
- **Use Case Name**: Cross-Reference Asset Hashes & CVE Vulnerabilities
- **Goal**: Match extension asset cryptographic hashes (`SHA-256`) against malware databases and cross-reference embedded JS libraries against CVE vulnerability records.
- **Primary Actor**: Dr. Aris Thorne (Researcher) / Sarah Jenkins (SOC Analyst)
- **Supporting Actors**: Threat Intelligence Service
- **Preconditions**: Extension asset hashes and manifest libraries have been extracted by `UC-DISC-01`.
- **Trigger**: Asynchronous threat intelligence lookup stage executes.
- **Main Success Scenario**:
  1. System queries Threat Intelligence Service using asset SHA-256 cryptographic hashes.
  2. System identifies third-party JavaScript libraries (jQuery, Lodash) and version numbers.
  3. System cross-references library versions against NVD CVE vulnerability records.
  4. If hash matches known malware DB, System sets Risk Score directly to $100.0$ (Critical Risk).
  5. If vulnerable library is found, System appends CVE IDs and vulnerability weights to risk report.
- **Alternative Flows**:
  - *Alt 1*: Device is offline. System queries local cached threat intelligence database.
- **Exception Flows**:
  - *Exc 1 (Threat API Timeout)*: Threat Intel service times out. System completes scan using local SAST rules and marks Threat Intel status as "Pending Sync".
- **Postconditions**: Scan report is enriched with CVE records and malware hash correlation metrics.
- **Related Features**: `FEAT-INTEL-001`, `FEAT-INTEL-002`
- **Related Requirements**: `REQ-INTEL-01`
- **Business Rules**: Rule 1 & 5 (Deterministic scoring & Backward compatibility).
- **Acceptance Criteria**: Flags matching malware asset hashes with instant Critical Risk status ($100.0$) and maps outdated JS libraries to official CVE IDs.

---

### UC-REPORT-01: Export Executive PDF Security Compliance Report

- **Use Case ID**: `UC-REPORT-01`
- **Use Case Name**: Export Executive PDF Security Compliance Report
- **Goal**: Generate high-resolution, server-side executive PDF security compliance reports summarizing extension inventory health, risk scores, and audit logs for CISOs and compliance auditors.
- **Primary Actor**: Sarah Jenkins (SOC Analyst) / David Ross (CISO)
- **Supporting Actors**: Reporting Service
- **Preconditions**: Scan reports exist for one or more endpoints.
- **Trigger**: User clicks "Export Executive PDF Report" in web console or desktop scanner.
- **Main Success Scenario**:
  1. User selects target scan report or fleet summary date range.
  2. System compiles executive summary, permission matrix, AST findings, and remediation logs.
  3. System renders multi-page PDF document complete with risk distribution charts and compliance summaries.
  4. System prompts user to save PDF file locally or downloads file via browser.
- **Alternative Flows**:
  - *Alt 1*: Enterprise CISO requests bulk PDF export for all corporate laptops. System packages individual endpoint reports into a ZIP archive.
- **Exception Flows**:
  - *Exc 1 (PDF Render Error)*: PDF engine fails. System logs error and provides fallback HTML report download.
- **Postconditions**: PDF report is generated and saved to user's local disk.
- **Related Features**: `FEAT-REPORT-001`
- **Related Requirements**: `REQ-REPORT-01`
- **Business Rules**: Rule 3 & 4 (Itemized transparency & Privacy sanitization).
- **Acceptance Criteria**: Generates multi-page executive PDF report with risk charts, permission matrices, and remediation logs within 3 seconds.

---

### UC-FLEET-01: Audit Enterprise Fleet Extension Risk Posture

- **Use Case ID**: `UC-FLEET-01`
- **Use Case Name**: Audit Enterprise Fleet Extension Risk Posture
- **Goal**: Provide enterprise CISOs and SOC teams with a centralized console aggregating extension inventories, risk distribution maps, and alert feeds across corporate laptop fleets.
- **Primary Actor**: David Ross (CISO) / Sarah Jenkins (SOC Analyst)
- **Supporting Actors**: Desktop Agents, Cloud Enterprise API
- **Preconditions**: Desktop Agents are deployed across corporate employee endpoints via MDM (Intune/Jamf).
- **Trigger**: CISO logs into Enterprise Web Console.
- **Main Success Scenario**:
  1. System queries central database and aggregates extension telemetry from all registered corporate endpoints.
  2. System displays high-level enterprise risk dashboard showing overall risk posture, total extensions installed, and high-risk extension counts.
  3. User searches for a specific extension (e.g. "WebHelper") to see which employee laptops have it installed.
  4. User defines enterprise policy threshold (e.g. "Auto-block extensions with Risk Score $\ge 70.0$").
  5. System pushes updated policy rules to Desktop Agents for browser policy enforcement.
- **Alternative Flows**:
  - *Alt 1*: User filters dashboard by department (e.g. Finance, Engineering) to evaluate team-specific risk posture.
- **Exception Flows**:
  - *Exc 1 (Agent Disconnected)*: Endpoint agent has not synced in 14 days. System marks endpoint status as "Offline / Stale Inventory".
- **Postconditions**: Enterprise extension posture is audited; policy threshold rules are updated.
- **Related Features**: `FEAT-FLEET-001`
- **Related Requirements**: `REQ-FLEET-01`
- **Business Rules**: Rule 4 (Privacy Precedence — zero employee browsing history or personal messages collected).
- **Acceptance Criteria**: Aggregates extension inventories from 10,000+ endpoints with real-time risk filtering and policy enforcement capabilities.

---

## 5. Master Use Case Traceability Matrix

The matrix below provides complete requirements traceability connecting Use Cases to Features, Requirements, Personas, Business Goals, and Target Releases:

| Use Case ID | Use Case Name | Related Feature ID | Related Requirement ID | Primary Actor | Business Goal Achieved | Target Release |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **UC-DISC-01** | Auto-Discover Installed Extensions | `FEAT-DISC-001` | `REQ-DISC-01` | Maya Lin (Home User) | Zero-friction local extension inventory visibility. | Version 1.0 MVP |
| **UC-SCAN-01** | Audit Build Package (CRX/ZIP) | `FEAT-DISC-002` | `REQ-DISC-01`, `REQ-SAST-01` | Alex Chen (Developer) | Pre-submission SAST auditing for developers & researchers. | Version 1.0 MVP |
| **UC-SAST-01** | Inspect Dynamic Code & Secrets | `FEAT-SAST-001`..`004` | `REQ-SAST-01`, `REQ-SAST-02`, `REQ-DOM-01` | Marcus Vance (Hunter) | Pinpoints dynamic `eval`, obfuscation, secrets, and DOM scraping. | Version 1.0 MVP |
| **UC-RISK-01** | Review Risk Score & Breakdown | `FEAT-RISK-001`, `FEAT-RISK-002` | `REQ-RISK-01`, `REQ-EXPL-01` | Maya Lin (Home User) | 100% deterministic score ($0.0-100.0$) with itemized log. | Version 1.0 MVP |
| **UC-AI-01** | Synthesize AI Narrative | `FEAT-AI-001` | `REQ-AI-01` | Maya Lin (Home User) | Plain-language executive summary of technical SAST findings. | Version 1.0 MVP |
| **UC-NOTIF-01** | Dispatch Local OS Risk Banners | `FEAT-NOTIF-001` | `REQ-NOTIF-01` | Maya Lin (Home User) | Instant native OS alert when high-risk extension is detected. | Version 1.0 MVP |
| **UC-HIST-01** | Manage Local Scan History | `FEAT-HIST-001` | `REQ-DISC-01`, `REQ-RISK-01` | Maya Lin (Home User) | Local offline history tracking and version comparison timeline. | Version 1.0 MVP |
| **UC-INTEL-01** | Cross-Reference Hashes & CVEs | `FEAT-INTEL-001`, `FEAT-INTEL-002` | `REQ-INTEL-01` | Dr. Thorne (Researcher) | Matches malware asset hashes and vulnerable JS libraries to CVEs. | Version 2.0 |
| **UC-REPORT-01**| Export Executive PDF Report | `FEAT-REPORT-001` | `REQ-REPORT-01` | Sarah Jenkins (SOC) | Generates audit-ready PDF compliance reports for SOC 2/GDPR. | Version 4.0 |
| **UC-FLEET-01**| Audit Enterprise Fleet Posture | `FEAT-FLEET-001` | `REQ-FLEET-01` | David Ross (CISO) | Centralized extension shadow IT inventory and policy enforcement. | Version 5.0 Enterprise |

---

## 6. Version 1.0 MVP Use Cases Specification

The **Version 1.0 MVP Core** encompasses seven mandatory use cases:

```
+-----------------------------------------------------------------------------------+
|                        VERSION 1.0 MVP USE CASE CORE                              |
+-----------------------------------------------------------------------------------+
|  1. UC-DISC-01 : Auto-Discover Installed Browser Extensions                       |
|  2. UC-SCAN-01 : Audit Standalone Extension Build Package (CRX/ZIP)               |
|  3. UC-SAST-01 : Inspect Dynamic Code Execution & Hardcoded Secrets               |
|  4. UC-RISK-01 : Review Risk Score & Itemized Breakdown                           |
|  5. UC-AI-01   : Synthesize Qualitative AI Security Explanation                   |
|  6. UC-NOTIF-01: Dispatch Local OS Native Risk Banners                            |
|  7. UC-HIST-01 : Manage Local Scan History & Version Timeline                     |
+-----------------------------------------------------------------------------------+
```

### Strategic Justification for MVP Execution
These seven use cases fulfill 100% of the standalone **Desktop Agent** operational lifecycle. They enable local extension discovery, sandboxed package extraction, local AST SAST parsing, deterministic score calculation, plain-language AI explanation, local OS alerting, and local SQLite history tracking—satisfying our primary target persona (**Maya Lin - Home User**) completely offline without requiring cloud database infrastructure.

---

## 7. Future Release Expansion Use Cases

- **Version 2.0**: `UC-INTEL-01` (Threat Intelligence Hash Correlation & CVE Lookup).
- **Version 3.0**: In-Browser Companion Extension Quick-Status & Toolbar Badge Interaction.
- **Version 4.0**: `UC-REPORT-01` (Executive PDF Report Exporter) & Interactive Code AST Tree Inspector.
- **Version 5.0**: `UC-FLEET-01` (Enterprise Fleet Risk Posture Audit & MDM Policy Enforcement).

---

## 8. Related Documents

- [`docs/FEATURE_CATALOG.md`](file:///d:/ExtensionProtect/docs/FEATURE_CATALOG.md) — Product Feature Catalog
- [`docs/PRODUCT_REQUIREMENTS.md`](file:///d:/ExtensionProtect/docs/PRODUCT_REQUIREMENTS.md) — Product Requirements Specification (PRD)
- [`docs/USER_PERSONA.md`](file:///d:/ExtensionProtect/docs/USER_PERSONA.md) — User Persona Specifications
- [`docs/CUSTOMER_JOURNEY.md`](file:///d:/ExtensionProtect/docs/CUSTOMER_JOURNEY.md) — Customer Journey Specifications
- [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) — Strategic Product Vision
- [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) — Master Documentation Index
