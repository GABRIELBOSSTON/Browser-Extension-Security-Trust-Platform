# Antigraviiti Extension Protect (AEP) — Documentation Index

Welcome to the official technical architecture documentation for **Antigraviiti Extension Protect (AEP)**, an enterprise-grade, AI-powered browser extension security platform.

---

## 1. Documentation Structure & Map

The documentation is organized in a modular structure to ensure clarity, maintainability, and clear separation of concerns across engineering teams:

```
docs/
├── README.md                      # [THIS FILE] Documentation Index & Architectural Conventions
├── PROJECT_OVERVIEW.md            # Product Vision, Problem Statement, Scope & Philosophy
├── SYSTEM_ARCHITECTURE.md         # High-Level Architecture, C4 Container Models, Protocols & Component Topology
├── BUSINESS_FLOW.md               # End-to-End Business Processes, Scan Lifecycles & Security Operations
├── USER_FLOW.md                   # Detailed User Journeys, Sequence Diagrams & UI State Transitions
└── FEATURE_LIST.md                # Comprehensive MVP Specifications & Version 1.0 - 5.0 Roadmap
```

---

## 2. Core Architectural Philosophy

AEP is engineered around three non-negotiable architectural tenets:

1. **Deterministic Risk Calculation**: Risk Scores ($0 - 100$) are calculated strictly by a deterministic **Rule Engine** executing static analysis heuristics, permission weighting, and threat intelligence matching. **AI models DO NOT calculate or modify the Risk Score**.
2. **AI as an Explainer & Remediation Synthesizer**: Generative AI (OpenAI / Ollama) is strictly used to translate complex technical findings (AST anomalies, obfuscation patterns, risky Chrome APIs) into human-readable narratives, contextual threat summaries, and actionable mitigation steps.
3. **Secure by Design & Zero-Trust Boundary**: Local analysis performed by the **Desktop Agent** strips Personal Identifiable Information (PII) and browser session tokens before emitting telemetry to the **Cloud Backend**. Local file extraction operates within constrained sandbox directories to prevent path-traversal (Zip-Slip) attacks.

---

## 3. Component Architecture Summary

AEP consists of four tightly integrated components:

| Component | Technology | Primary Responsibility |
| :--- | :--- | :--- |
| **Desktop Agent** | Go / Rust / Electron | Discovers installed extensions, performs local AST/Manifest SAST, handles file extraction, emits metadata to Cloud. |
| **Cloud Backend** | Python (FastAPI), PostgreSQL, Redis | Executes Risk Engine, manages Threat Intel & CVE databases, orchestrates AI explanations, serves REST APIs. |
| **Web Dashboard** | Next.js 14 (App Router), TailwindCSS, Shadcn UI | Provides enterprise security monitoring, scan history, deep-dive forensic UI, and PDF report exports. |
| **Chrome Companion** | Manifest V3 (JavaScript) | Lightweight browser extension displaying real-time risk badges, extension status, and bridging to local Desktop Agent. |

---

## 4. Glossary & Standard Terminology

To prevent ambiguity across engineering teams, the following standardized terms are used across all documentation:

- **AEP**: Antigraviiti Extension Protect (the overall platform).
- **SAST**: Static Application Security Testing (static code and manifest analysis).
- **Rule Engine**: The deterministic python module responsible for evaluating security rules and producing numerical risk scores.
- **Risk Score**: A normalized score between $0.0$ (Safe) and $100.0$ (Critical Risk) representing an extension's threat exposure.
- **Host Permissions**: URL patterns that a browser extension is granted access to intercept, modify, or read.
- **Content Scripts**: JavaScript files injected directly into targeted webpage DOMs.
- **Background Worker**: Extension background service worker (Manifest V3) or background page (Manifest V2) handling event logic.
- **Zip-Slip**: A path-traversal vulnerability occurring during archive extraction where path manipulation extracts files outside target directories.

---

## 5. Engineering Standards & Compliance

All system components specified within this documentation suite adhere strictly to:
- **Clean Architecture & SOLID Principles**
- **OWASP API Security Top 10 (2023)**
- **OWASP Top 10 Web Application Security Risk**
- **Principle of Least Privilege (PoLP)**
- **Secure System Development Lifecycle (SSDLC)**
