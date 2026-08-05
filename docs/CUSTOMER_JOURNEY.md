# Customer Journey Specifications — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP Customer Journey & User Lifecycle Mapping |
| **Document ID** | `DOC-JOUR-001` |
| **Current Status** | DRAFT — Pending CTO Review |
| **Document Version** | `1.0.0` |
| **Document Owner** | Product Manager & User Experience Strategist |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Target Audience** | Founders, Product Managers, UI/UX Designers, Product Marketers, and Engineers |
| **Related Documents** | [`docs/USER_PERSONA.md`](file:///d:/ExtensionProtect/docs/USER_PERSONA.md), [`docs/VALUE_PROPOSITION.md`](file:///d:/ExtensionProtect/docs/VALUE_PROPOSITION.md), [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) |

---

## 2. Executive Summary

Mapping the customer journey is essential to designing an intuitive, high-value product that removes user friction and delivers immediate time-to-value. **Antigraviiti Extension Protect (AEP)** serves eight distinct user personas, each entering the product ecosystem via unique triggers, evaluation criteria, and desired outcomes.

This document defines the complete end-to-end user lifecycle across eight target personas: from initial trigger event, awareness, evaluation, onboarding, first success moment ("Aha!" moment), to long-term usage and community advocacy. Additionally, it establishes **Persona 1: Home User (Maya Lin)** as the **Primary MVP Customer Journey for Version 1.0**, mapping product improvement opportunities (Delight, Automation, Education, and Community) to guide future UI/UX and product design decisions.

---

## 3. The 9-Stage Standard Customer Journey Lifecycle

Every persona progresses through nine standardized lifecycle stages within the AEP ecosystem:

```
[1. Trigger] ──> [2. Discover] ──> [3. Evaluate] ──> [4. Install] ──> [5. First Scan]
                                                                            │
[9. Recommend] <── [8. Retain] <── [7. Take Action] <── [6. Aha! Moment] <──┘
```

1. **Trigger**: An external news event, security scare, policy rejection, or audit requirement prompts the user to seek a browser extension security solution.
2. **Discover**: The user discovers AEP through organic search, security blogs, developer communities, or corporate IT recommendations.
3. **Evaluate**: The user assesses AEP’s safety, privacy policy, cost, and system overhead before installing.
4. **Install**: The user installs the Desktop Agent, Web Dashboard, or Chrome Companion Extension.
5. **First Scan**: AEP performs its initial automated audit of installed extensions or build packages.
6. **First Success ("Aha!" Moment)**: The user receives their first clear, itemized risk report or plain-language security explanation.
7. **Take Action**: The user removes a dangerous extension, fixes a code flaw, isolates an endpoint, or exports an audit report.
8. **Retain**: AEP runs silently in the background, providing continuous protection and value over weeks and months.
9. **Recommend**: The user advocates for AEP to peers, colleagues, developer teams, or online communities.

---

## 4. Persona Customer Journeys

---

### Persona 1: Home User (Maya Lin - Primary MVP Journey)

#### 1. Persona Overview
Maya Lin is a freelance designer with low technical skills. She relies on 14+ browser extensions for daily work and personal tasks, but fears hidden keyloggers, identity theft, and message leaks on Web WhatsApp.

#### 2. Trigger
Maya reads a news article about malicious Chrome extensions secretly exfiltrating WhatsApp Web messages and banking tokens from unsuspecting users. She realizes she has installed numerous unverified design utility extensions and experiences immediate anxiety.

#### 3. Awareness Stage
Searches Google for *"how to check if browser extensions are safe"* and finds a security tech blog recommending AEP as a free, privacy-first desktop scanner.

#### 4. Evaluation Stage
Maya asks:
- *Is this tool free and safe?*
- *Will it steal my browsing data or personal photos?*
- *Is it too complicated for me to understand?*
She is reassured by AEP’s explicit Privacy-by-Default guarantee: local scanning only, zero personal data harvesting.

#### 5. Onboarding Stage
Maya downloads and runs the Desktop Agent. She expects a clean, zero-configuration interface that does not ask for complex technical setups or forced user account creation.

#### 6. First Success Moment ("Aha!" Moment)
Within 5 seconds of launch, AEP displays a clear, color-coded health summary of her 14 installed extensions. It highlights a free "PDF Converter" extension with a **High Risk** badge ($82.0$) and explains in plain language: *"This extension requests access to read all your data on web messaging portals and communicates with an unvetted external server."*

#### 7. Long-Term Usage
AEP runs silently in Maya's system tray. She checks it whenever she installs a new extension. When an installed extension silently updates and requests broader permissions, AEP fires a local desktop banner notification, giving her continuous peace of mind.

#### 8. Success Outcome
Maya maintains a clean, secure browser environment. She browses the web and conducts client messaging without fear of identity theft or data leaks.

#### 9. Potential Friction & Mitigations
- **Friction**: Fear that AEP itself might collect her browsing data.
  - *Mitigation*: Prominent onboarding banner confirming: *"100% Offline Local Scan. Zero PII or browsing data leaves your computer."*
- **Friction**: Confused by technical jargon (e.g. `atob()`, `manifest permissions`).
  - *Mitigation*: AI Explanation engine translates all technical terms into plain non-technical sentences.

---

### Persona 2: Professional Extension Developer (Alex Chen)

#### 1. Persona Overview
Alex Chen is a Senior Frontend Engineer building commercial browser extensions. He needs to pass Web Store reviews without sudden, unexplained rejections.

#### 2. Trigger
Alex receives a sudden Chrome Web Store submission rejection notice stating his extension update violates "Single Purpose & Code Security Policies"—without giving specific line numbers or code context.

#### 3. Awareness Stage
Discovers AEP on GitHub and Twitter/X, where fellow extension developers discuss using AEP for pre-submission SAST auditing.

#### 4. Evaluation Stage
Alex asks:
- *Can AEP scan my packaged build ZIP locally before I upload to the store?*
- *Does it catch hardcoded API secrets, dynamic `eval` calls, and Manifest V3 flaws?*
- *Will it upload my proprietary source code to a public server?*
He is satisfied knowing AEP runs local AST parsing without cloud code transmission.

#### 5. Onboarding Stage
Alex installs AEP and drags his extension build package (`dist.zip`) into the scanner interface.

#### 6. First Success Moment ("Aha!" Moment)
AEP completes the local SAST audit in 3 seconds. It flags line 214 of `vendor.js` where an old utility library uses an unhandled `eval()` call, generating a Risk Score of $68.0$. AEP explicitly highlights the line and recommends replacing it with `JSON.parse()`.

#### 7. Long-Term Usage
Alex integrates AEP into his local pre-release checklist. Every time he builds a new version, he runs an AEP audit to verify zero secrets or policy violations exist before submitting to Web Stores.

#### 8. Success Outcome
Alex achieves a 100% store submission approval rate with zero surprise rejections or delistings.

#### 9. Potential Friction & Mitigations
- **Friction**: False-positive security alerts on legitimate minified library code.
  - *Mitigation*: Provide clear itemized score breakdowns so developers can inspect and verify rule contexts easily.

---

### Persona 3: Bug Bounty Hunter (Marcus Vance)

#### 1. Persona Overview
Marcus Vance is an independent security researcher looking for zero-day extension vulnerabilities (DOM XSS, privilege escalation, C2 endpoints) to submit to bug bounty programs.

#### 2. Trigger
A major bug bounty platform launches a high-reward security campaign targeting popular browser extensions.

#### 3. Awareness Stage
Reads an open-source security research paper citing AEP’s AST structural parser as a premier tool for rapid extension vulnerability triage.

#### 4. Evaluation Stage
Marcus asks:
- *Can AEP de-obfuscate string array decoders automatically?*
- *Does it extract all hardcoded IP endpoints and risky Chrome API calls?*
- *Can I triage 50 extensions quickly?*

#### 5. Onboarding Stage
Marcus sets up AEP and imports a batch of 30 target extension `.crx` files for automated analysis.

#### 6. First Success Moment ("Aha!" Moment)
AEP triages the batch in under two minutes, ranking an extension with 500,000 installs at the top (Risk Score $91.5$). AEP’s AST inspector highlights an unhandled DOM injection sink on line 412 of `content_script.js`.

#### 7. Long-Term Usage
Marcus uses AEP daily as his primary automated triage engine prior to performing manual exploit development.

#### 8. Success Outcome
Marcus discovers three verified zero-day vulnerabilities in popular extensions, earning $8,000 in bug bounty rewards.

#### 9. Potential Friction & Mitigations
- **Friction**: Desires raw AST export formats for custom scripting.
  - *Mitigation*: Provide structured JSON export of AST findings and network telemetry.

---

### Persona 4: Security Researcher & Malware Analyst (Dr. Aris Thorne)

#### 1. Persona Overview
Dr. Aris Thorne is a threat intelligence researcher at a cybersecurity vendor, tracking APT extension malware campaigns and malicious C2 infrastructure.

#### 2. Trigger
A global enterprise client reports a suspected data breach caused by a fleet of compromised browser extensions exfiltrating corporate tokens.

#### 3. Awareness Stage
Discovers AEP at an international cybersecurity conference presentation detailing AEP's cryptographic hash correlation and MITRE ATT&CK technique mapping.

#### 4. Evaluation Stage
Dr. Thorne asks:
- *Does AEP cross-reference asset cryptographic SHA-256 hashes against threat intelligence databases?*
- *Can it map extension behaviors to MITRE ATT&CK TTPs?*

#### 5. Onboarding Stage
Dr. Thorne deploys AEP to analyze a corpus of 200 suspicious extension samples collected from threat honeypots.

#### 6. First Success Moment ("Aha!" Moment)
AEP processes the corpus and groups 35 disparate extension samples under a single campaign cluster based on matching content script SHA-256 hashes and hardcoded C2 IP endpoints.

#### 7. Long-Term Usage
Dr. Thorne integrates AEP threat data into his firm's global intelligence feeds, continuously cataloging malicious extension infrastructure.

#### 8. Success Outcome
Publishes a landmark threat intelligence whitepaper exposing a malicious extension network, resulting in the removal of 50+ malicious extensions from public Web Stores.

#### 9. Potential Friction & Mitigations
- **Friction**: Incomplete threat intelligence coverage on newly registered C2 domains.
  - *Mitigation*: Enable daily asynchronous synchronization with global CVE and threat intelligence feeds.

---

### Persona 5: Blue Team / SOC Analyst (Sarah Jenkins)

#### 1. Persona Overview
Sarah Jenkins is an L2 SOC Analyst at a financial bank. She needs to monitor employee laptop endpoints for extension data exfiltration threats that bypass traditional host EDR.

#### 2. Trigger
Her SOC receives a security bulletin warning of a new browser extension keylogger targeting financial web portals.

#### 3. Awareness Stage
Introduced to AEP by her CISO during a security architecture review focusing on closing browser application blind spots.

#### 4. Evaluation Stage
Sarah asks:
- *Will AEP feed high-fidelity alerts directly into our SIEM (Splunk/Elastic)?*
- *Does it provide clear mathematical proof explaining why an alert fired?*

#### 5. Onboarding Stage
Sarah configures the AEP Web Dashboard and connects SIEM webhook endpoints to receive real-time alert telemetry.

#### 6. First Success Moment ("Aha!" Moment)
Sarah receives a real-time SIEM alert: *"Employee Laptop #412 installed High-Risk Extension 'QuickCap 1.2' (Score: 88.5)."* Clicking the alert, AEP highlights that the extension requests `<all_urls>` host access and communicates with an unvetted foreign IP.

#### 7. Long-Term Usage
Sarah uses AEP daily to monitor corporate fleet alerts, investigate suspicious employee extension installations, and export incident audit reports.

#### 8. Success Outcome
Sarah isolates and remediates compromised employee endpoints in under 10 minutes, preventing corporate data exfiltration.

#### 9. Potential Friction & Mitigations
- **Friction**: Low-fidelity alert noise overwhelming the SOC queue.
  - *Mitigation*: Allow custom threshold configuration so SOC teams receive alerts only for High/Critical risk scores ($\ge 70.0$).

---

### Persona 6: Enterprise Security Manager / CISO (David Ross)

#### 1. Persona Overview
David Ross is a CISO at a global enterprise managing security posture, shadow IT risks, and GDPR/SOC 2 compliance across 10,000 employee laptops.

#### 2. Trigger
An external SOC 2 compliance audit auditor issues a formal finding: *"Organization lacks inventory visibility and security controls over employee browser extensions."*

#### 3. Awareness Stage
Attends an executive CISO roundtable where peers recommend AEP Enterprise Fleet Governance for automated extension inventory and compliance monitoring.

#### 4. Evaluation Stage
David asks:
- *Can AEP give me a centralized console showing extension inventory across 10,000 laptops?*
- *Can we automate extension blocking policies based on risk score thresholds?*
- *Does it comply with employee data privacy laws (GDPR)?*

#### 5. Onboarding Stage
David authorises an enterprise pilot deploying the AEP Desktop Agent to 500 employee laptops via MDM scripts.

#### 6. First Success Moment ("Aha!" Moment)
David opens the AEP Enterprise Dashboard to discover 1,200 unique extensions active across the pilot fleet, including 14 unauthorized extensions with Critical Risk Scores ($\ge 80.0$).

#### 7. Long-Term Usage
David establishes organization-wide policies in AEP that automatically block extensions exceeding acceptable risk thresholds, maintaining continuous compliance.

#### 8. Success Outcome
David passes the SOC 2 audit with zero findings and presents an executive report demonstrating a 75% reduction in enterprise extension risk.

#### 9. Potential Friction & Mitigations
- **Friction**: Employee privacy concerns regarding enterprise endpoint software.
  - *Mitigation*: Provide immutable privacy guarantees confirming zero browsing history or personal message content is collected.

---

### Persona 7: IT Administrator (Robert Kowalski)

#### 1. Persona Overview
Robert Kowalski is a Senior Systems Administrator managing MDM software deployment (Intune/Jamf) across 1,200 employee laptops.

#### 2. Trigger
The CISO directs IT to deploy a browser extension security agent to all employee laptops immediately.

#### 3. Awareness Stage
Receives an IT work order specifying the silent deployment of the AEP Desktop Agent via Microsoft Intune.

#### 4. Evaluation Stage
Robert asks:
- *Can I deploy this silently via `.msi` / `.pkg` installers without user interaction?*
- *Will it consume excessive CPU/RAM and cause employee helpdesk complaints?*
- *Does it run cleanly without kernel-level drivers that crash operating systems?*

#### 5. Onboarding Stage
Robert tests the AEP Desktop Agent installer package on 10 test laptops in his IT sandbox.

#### 6. First Success Moment ("Aha!" Moment)
The installer completes silently in 15 seconds. Task Manager verifies the Desktop Agent runs under standard user privileges using <20 MB RAM and 0.1% CPU.

#### 7. Long-Term Usage
Robert rolls out AEP to all 1,200 endpoints. The agent updates local extension blocklists automatically without requiring manual IT GPO edits.

#### 8. Success Outcome
100% deployment coverage across employee laptops with zero agent crashes and zero helpdesk support tickets logged.

#### 9. Potential Friction & Mitigations
- **Friction**: Agent conflicts with host OS security software.
  - *Mitigation*: Ensure Desktop Agent runs as a clean, user-space daemon with zero kernel driver dependencies.

---

### Persona 8: Educational User (Clara Gomez)

#### 1. Persona Overview
Clara Gomez is a 2nd-year CS student studying web application security and Manifest V3 extension architectures.

#### 2. Trigger
Her cybersecurity professor assigns a lab project: *"Audit five browser extensions and analyze their permission risk models."*

#### 3. Awareness Stage
Discovers AEP through university CS lab resource lists and open-source security documentation.

#### 4. Evaluation Stage
Clara asks:
- *Is it free for students?*
- *Does it explain why a permission is risky so I can write my lab report?*

#### 5. Onboarding Stage
Clara downloads AEP on her study laptop and scans extensions she uses for university work.

#### 6. First Success Moment ("Aha!" Moment)
AEP scans a free "PDF Annotator" extension, breaking down its $74.0$ Risk Score into itemized deductions (`+30.0 Host Permissions`, `+20.0 Dynamic Script Creation`, `+10.0 Outdated jQuery`).

#### 7. Long-Term Usage
Clara uses AEP as an educational reference tool for her coursework and keeps her personal study laptop safe.

#### 8. Success Outcome
Clara completes her lab assignment with distinction, gaining a deep understanding of browser SAST analysis.

#### 9. Potential Friction & Mitigations
- **Friction**: Paywall barriers restricting access to core scanner features.
  - *Mitigation*: Keep core local desktop scanner and educational breakdowns 100% free forever.

---

## 5. Cross-Persona Comparison Matrix

| Lifecycle Stage | 1. Home User (Maya) | 2. Developer (Alex) | 3. Bug Hunter (Marcus) | 4. Researcher (Dr. Thorne) | 5. SOC Analyst (Sarah) | 6. CISO (David) | 7. IT Admin (Robert) | 8. Student (Clara) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Trigger** | News of WhatsApp leak | Store rejection notice | Bounty campaign launch | Data breach incident | SIEM security bulletin | Audit compliance gap | CISO deployment mandate | University CS lab assignment |
| **Discovery** | Tech blog search | GitHub / Developer X | Academic paper | Cyber conference | CISO recommendation | Executive CISO roundtable | Internal IT work order | University CS lab list |
| **Evaluation** | Privacy & ease of use | Local ZIP scan & speed | AST & de-obfuscation depth | SHA256 & C2 hash matching | SIEM integration & fidelity | Central fleet & compliance | Intune deployment & RAM | Free cost & education value |
| **Installation** | Single-click Desktop installer | Local CLI / Desktop App | Standalone CLI / Desktop | Enterprise Server / CLI | Web Dashboard link | Enterprise Cloud Console | Intune MDM silent script | Free Desktop Agent |
| **First Success** | Plain-language risk warning | Line number of `eval()` bug | Pinpoints DOM XSS sink | Clusters 35 malware samples | Real-time SIEM alert on laptop | Fleet risk map across 1.2k laptops | Silent deploy <20MB RAM | Itemized point breakdown |
| **Retention Driver** | Silent background alerts | Pre-release build pipeline | Daily zero-day triage | Global threat intel feed | Daily SOC alert queue | Quarterly board compliance | Zero helpdesk complaints | Coursework & study safety |

---

## 6. MVP Customer Journey Designation

```
+-----------------------------------------------------------------------------------+
|               DESIGNATED MVP CUSTOMER JOURNEY: PERSONA 1 (MAYA LIN)               |
+-----------------------------------------------------------------------------------+
|  VERSION 1.0 SCOPE  : Standalone Desktop Agent Local Auto-Discovery & SAST        |
|  TARGET USER        : Non-technical Home User (Maya Lin)                          |
|  SUCCESS CRITERIA   : Auto-scan installed extensions -> Plain AI summary        |
+-----------------------------------------------------------------------------------+
```

### Strategic Justification for MVP Choice
**Persona 1: Home User (Maya Lin)** is designated as the primary customer journey for **Version 1.0 (MVP)**:

1. **Alignment with Version 1.0 Scope**: Version 1.0 delivers the standalone **Desktop Agent** (local extension auto-discovery, manifest parsing, local AST SAST, rule engine scoring, plain AI summary, and local OS notifications). Maya's journey exercises 100% of this scope without requiring cloud backend persistence or enterprise fleet integrations.
2. **User Experience Validation**: Maya is non-technical. If Maya can successfully install AEP, understand her risk report in 5 seconds, and take correct action without confusion, the UX is proven for all higher-level technical personas.

---

## 7. Product Improvement Opportunities Matrix

| Persona Category | Delight Opportunities | Automation Opportunities | Education Opportunities | Community Opportunities |
| :--- | :--- | :--- | :--- | :--- |
| **Home User (Maya)** | One-click "Clean Up" button for risky extensions. | Background auto-scanning upon extension update. | Plain-language "Privacy Health Score" tooltips. | Anonymous community trust ratings. |
| **Developer (Alex)** | Direct VS Code extension plugin for inline SAST. | Automated pre-commit git hooks for extension ZIPs. | Inline remediation guide links (e.g. how to fix CSP). | Verified Developer security badges. |
| **Bug Hunter (Marcus)** | Interactive AST code graph visualizer. | Auto-decoding of base64 & array string decoders. | Taxonomy guide linking findings to OWASP Web API. | Shareable PoC report exports. |
| **Researcher (Thorne)**| Campaign cluster visualization graphs. | Automated C2 domain IoC export format. | Malware family classification tags. | Global threat intelligence sharing network. |
| **SOC Analyst (Sarah)** | One-click laptop quarantine action. | Automated SIEM alert enrichment webhooks. | Incident response playbook integration. | Cross-organization threat sharing. |
| **CISO (David)** | Executive PDF Board presentation generator. | Automated GPO / Intune policy sync rules. | Regulatory compliance mapping matrices (SOC2/GDPR).| CISO executive benchmark benchmarking. |
| **IT Admin (Robert)** | Zero-reboot silent installer packages. | Self-healing endpoint background daemon scripts. | MDM deployment script library. | Admin deployment knowledge base. |
| **Student (Clara)** | Interactive "Learn SAST" sandbox mode. | Step-by-step mathematical score breakdown log. | Open cybersecurity whitepaper library. | Student security research forum. |

---

## 8. Related Documents

- [`docs/USER_PERSONA.md`](file:///d:/ExtensionProtect/docs/USER_PERSONA.md) — User Persona Specifications
- [`docs/VALUE_PROPOSITION.md`](file:///d:/ExtensionProtect/docs/VALUE_PROPOSITION.md) — Value Proposition Strategy
- [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) — Strategic Product Vision & Roadmap
- [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md) — AEP Constitutional Principles
- [`docs/ENGINEERING_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md) — AEP Engineering Handbook
- [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) — Master Documentation Index
