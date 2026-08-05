# Project Overview — Antigraviiti Extension Protect (AEP)

---

## 1. Executive Summary

**Antigraviiti Extension Protect (AEP)** is an enterprise-grade, **AI-Powered Browser Extension Security Platform**. It protects individual users, developers, security researchers, SOC analysts, and enterprise fleets against malicious, vulnerable, or over-privileged browser extensions.

Unlike basic static analyzers or file-upload utilities, AEP operates as a comprehensive 4-tier security ecosystem:
1. **Desktop Agent**: Monitors locally installed extensions across Chromium-based browsers (Chrome, Edge, Brave, Opera), performs local file extraction and Static Application Security Testing (SAST), and dispatches privacy-preserving telemetry to the Cloud.
2. **Cloud Backend**: Hosts the core deterministic **Risk Engine**, integrates Threat Intelligence feeds and CVE databases, orchestrates AI explanation pipelines, and manages user state.
3. **Web Dashboard**: Modern enterprise console (Next.js 14 / TailwindCSS / Shadcn UI) providing security monitoring, historic scan archives, detailed AST forensic trees, and executive PDF reports.
4. **Chrome Companion Extension**: Lightweight extension rendering real-time risk indicators, badge notifications, and local agent communication bridging directly inside the user's browser.

---

## 2. Problem Statement & Real-World Threat Landscape

Browser extensions have become one of the most under-monitored attack vectors in modern cybersecurity. Operating with elevated privileges inside the browser context, malicious or compromised extensions pose severe risks:

### 2.1 Key Threat Vectors
- **WhatsApp Web & Messaging Data Exfiltration**: Malicious content scripts injected into web messaging applications (e.g., WhatsApp Web, Telegram Web) can inspect DOM trees, capture active session tokens, harvest contact lists, and exfiltrate private messages without triggering network firewalls.
- **Over-Privileged Permission Creep**: Extensions frequently demand broad host permissions such as `<all_urls>`, `*://*/*`, or `webRequest`, allowing unrestricted access to read and modify sensitive HTTP requests, auth headers, and cookies across all visited websites.
- **Code Obfuscation & Dynamic Execution**: Attackers evade Web Store review processes by employing heavy JavaScript obfuscation (string array encodings, variable renaming) combined with dynamic code execution (`eval()`, `Function()`, `atob()`, dynamic `<script>` tag injections).
- **Silent C2 Telemetry & Data Exfiltration**: Background service workers quietly transmit device metadata, browsing history, keystrokes, and hardcoded API keys to unvetted third-party Command & Control (C2) domains.
- **Unpatched CVEs & Abandonware**: Benign extensions that are no longer maintained by their original authors often contain unpatched JavaScript library vulnerabilities (e.g., outdated jQuery, lodash, or crypto libraries) susceptible to Cross-Site Scripting (XSS) or prototype pollution.

---

## 3. Product Vision & Core Philosophy

### 3.1 Non-Binary Risk Assessment Philosophy
AEP explicitly rejects binary safety classifications (such as declaring an extension strictly "SAFE" or "MALICIOUS"). Security analysis in browser extensions involves nuanced context.

Instead, AEP delivers:
1. **Normalized Risk Score ($0.0 - 100.0$)**: A multi-dimensional risk score derived from permission weights, AST SAST anomalies, network telemetry, and threat intelligence.
2. **Empirical Technical Findings**: Clear, auditable evidence showing exact line numbers, AST node types, manifest permissions, and network endpoints triggering security alerts.
3. **Actionable Mitigation Guidance**: Practical recommendations helping users decide whether to keep, isolate, or remove the extension.

### 3.2 Strict Separation of Deterministic Rules and AI Synthesizer
To guarantee reproducibility, auditability, and freedom from AI hallucinations:
- **Risk Scores are calculated 100% deterministically by the Rule Engine.**
- **AI (OpenAI / Ollama) is used strictly as a narrative synthesizer**, translating raw technical SAST outputs, AST nodes, and permission matrices into clear, structured explanations tailored to the user's technical background.

```
+-----------------------------------------------------------------------------+
|                            DETERMINISTIC PIPELINE                           |
| Unpack -> Manifest SAST -> AST SAST -> Pattern Matching -> Rule Engine      |
+--------------------------------------+--------------------------------------+
                                       |
                                       v
                               Calculated Risk Score (e.g., 78.5 / 100)
                                       |
                                       v
+--------------------------------------+--------------------------------------+
|                              AI SYNTHESIZER                          |
| Generates qualitative summary, threat explanation & mitigation steps        |
+-----------------------------------------------------------------------------+
```

---

## 4. Target Audience & User Personas

AEP is engineered to serve diverse user tiers, ranging from non-technical end-users to enterprise SOC teams:

| Persona | Technical Background | Primary Goal / Use Case |
| :--- | :--- | :--- |
| **General User** | Non-technical | Quickly check if installed extensions pose privacy risks to banking or messaging accounts. |
| **Developer** | High (Software Dev) | Audit own extension prior to Web Store submission; check for exposed API keys or dangerous dynamic calls. |
| **Security Researcher & Bug Hunter** | Expert (Reverse Engineering) | Inspect AST syntax trees, decode obfuscated strings, trace hardcoded C2 endpoints, analyze CSP bypasses. |
| **SOC Analyst / Incident Responder** | High (Threat Analysis) | Monitor enterprise endpoint fleet, audit extension inventories across company laptops, investigate active exfiltration alerts. |

---

## 5. 4-Tier Component Architecture Overview

```
+-----------------------------------------------------------------------------------+
|                                  DESKTOP AGENT                                    |
| - Automatic discovery of installed extensions (Chrome, Edge, Brave, Opera)        |
| - Local manifest & file extraction with Zip-Slip protection                       |
| - Fast local regex SAST & privacy-preserving telemetry generation                 |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v (Encrypted HTTPS REST / gRPC Telemetry)
+-----------------------------------------+-----------------------------------------+
|                                  CLOUD BACKEND                                    |
| - Core Rule Engine & Weighted Risk Calculation Algorithm                         |
| - CVE Database & Threat Intelligence Feed Integration                             |
| - AI Service (OpenAI GPT-4o / Local Ollama fallback engine)                       |
| - RESTful API Gateway & PostgreSQL / Redis Persistence Layer                      |
+--------------------+------------------------------------+-------------------------+
                     |                                    |
                     v                                    v
+--------------------+--------------------+   +-----------+-------------------------+
|           WEB DASHBOARD                 |   |       CHROME COMPANION EXTENSION    |
| - Next.js 14 App Router UI              |   | - Real-time Risk Score Indicator Badge  |
| - Interactive AST Forensic Inspector    |   | - Desktop Agent Local Bridge Interop    |
| - Enterprise Fleet & History Management |   | - Quick Warning Modal & Deep Link UI    |
+-----------------------------------------+   +-----------------------------------------+
```

---

## 6. Scope Boundaries & Non-Goals

### 6.1 In-Scope (Core Platform Boundaries)
- Static analysis of Chromium browser extension packages (`.crx`, unpacked folders, and `.zip` archives).
- Automatic detection of local extension installations across Windows, macOS, and Linux endpoints via the Desktop Agent.
- Manifest V2 and Manifest V3 compatibility auditing.
- AST parsing for JavaScript dynamic execution, string obfuscation, and dangerous API dereferencing.
- Threat Intelligence integration (known bad hashes, C2 domain blocklists, CVE vulnerability cross-referencing).
- AI-synthesized risk remediation guidance.

### 6.2 Out-of-Scope (Non-Goals)
- **Active Memory Injection Prevention**: AEP does not run in-kernel driver hooks to kill browser processes in real-time. (Mitigation is recommended via browser extension management policies or manual removal).
- **Non-Chromium Extensions**: Firefox (`.xpi`) or Safari extensions are excluded from MVP phase (planned for Roadmap Phase 5).
- **Automated Malicious Code Modification**: AEP will not alter or auto-patch an extension's underlying JavaScript source code.
