# User Persona Specifications — Antigraviiti Extension Protect (AEP)

---

## 1. Document Information

| Metadata Field | Document Detail |
| :--- | :--- |
| **Document Title** | AEP User Persona & Target Audience Specifications |
| **Document ID** | `DOC-PER-001` |
| **Current Status** | DRAFT — Pending CTO Review |
| **Document Version** | `1.0.0` |
| **Document Owner** | Product Manager & User Experience Strategist |
| **Technical Reviewer** | Chief Technology Officer (CTO) / Security Architect |
| **Last Updated** | 2026-08-04 |
| **Target Audience** | Founders, Product Managers, UI/UX Designers, Product Marketers, and Engineers |
| **Related Documents** | [`docs/VALUE_PROPOSITION.md`](file:///d:/ExtensionProtect/docs/VALUE_PROPOSITION.md), [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md), [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md) |

---

## 2. Executive Summary

Understanding our target users is fundamental to building a product that delivers real value. **Antigraviiti Extension Protect (AEP)** serves a diverse spectrum of users—from everyday non-technical internet users seeking privacy peace of mind, to extension developers needing pre-submission security audits, to enterprise SOC teams managing security posture across corporate laptop fleets.

This document establishes the official specifications for **eight distinct user personas**. It outlines their identities, daily workflows, security pain points, success criteria, feature priorities, and day-in-the-life usage scenarios. Furthermore, it designates **Persona 1: Everyday Home User (Maya)** as our **Primary Target Persona for Version 1.0 (MVP)**, providing strategic business and product justifications for this focus.

---

## 3. Comprehensive Persona Specifications

---

### Persona 1: Everyday Home User (Primary MVP Target)

```
+-----------------------------------------------------------------------------------+
|  NAME: Maya Lin                       ROLE: Freelance Graphic Designer & Consumer |
|  EXPERIENCE: Non-Technical                ORG SIZE: Individual / Home Desktop     |
|  TECHNICAL SKILL: Low (Non-Technical)     SECURITY AWARENESS: Moderate            |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Maya Lin (The Everyday Consumer)
- **Job / Role**: Freelance Graphic Designer & Digital Creator
- **Experience Level**: Non-technical consumer
- **Organization Size**: Individual / Home User
- **Technical Skill Level**: Low (Uses web browsers for work, banking, social media, and messaging, but does not understand JavaScript or source code).

#### 2. Goals
- Use productivity browser extensions (color pickers, ad blockers, discount finders, font managers) without risking personal identity theft or account takeover.
- Keep personal banking credentials, credit cards, and private WhatsApp Web communications safe from data harvesting.
- Have peace of mind without needing to learn complex technical cybersecurity jargon.

#### 3. Pain Points
- **Forced Blind Trust**: Installed over 15 browser extensions to help with design work, but has zero visibility into what those extensions are doing in the background.
- **Fear of Hidden Data Leaks**: Heard news about browser extensions stealing WhatsApp Web messages and banking cookies, creating constant low-level anxiety.
- **Opaque Permission Prompts**: Confused by browser prompts asking for "Read and change all your data on all websites." Does not know if this is normal or dangerous.

#### 4. Daily Workflow
- Spends 8+ hours daily in Google Chrome and Brave. Uses Web WhatsApp for client communication, accesses online banking, uploads design work to cloud storage, and uses multiple browser utility extensions throughout the day.

#### 5. Security Concerns
- Personal financial account takeover.
- Private client messages leaked from web messaging portals.
- Malicious extension updates silently converting a safe extension into adware or keyloggers.

#### 6. Success Criteria
- Instant visual confirmation that all installed extensions are safe (e.g. Green status).
- Immediate, plain-language notification if an extension becomes risky or demands suspicious access.
- Zero manual file uploads or technical setup required.

#### 7. Why They Choose AEP
Maya chooses AEP because traditional antivirus tools do not warn her about excessive extension permissions, and web store reviews are untrustworthy. AEP auto-discovers her installed extensions locally and explains risks in plain language.

#### 8. Feature Priorities
1. **Local Auto-Scanning**: 100% automated discovery of installed extensions without manual CRX export.
2. **AI-Driven Plain Explanations**: Translates technical risks into simple human sentences.
3. **Local OS Notifications**: Fires clear desktop banners when high-risk extensions are detected.
4. **Offline-First Processing**: Runs silently on her laptop without requiring account registration.

#### 9. Typical Usage Scenario
Maya installs AEP on her laptop. Upon launch, AEP automatically audits her 14 installed browser extensions. It flags a free "PDF Converter" extension with a High Risk warning ($82.0$), explaining: *"This extension requests access to read all your data on web messaging sites and sends data to an unvetted server."* Realizing she installed it recently, Maya clicks "Remove Extension", instantly restoring her peace of mind.

---

### Persona 2: Professional Extension Developer

```
+-----------------------------------------------------------------------------------+
|  NAME: Alex Chen                      ROLE: Senior Frontend & Extension Developer |
|  EXPERIENCE: 8+ Years Software Dev       ORG SIZE: Mid-Sized Software Startup     |
|  TECHNICAL SKILL: High (Expert JS/TS)     SECURITY AWARENESS: High                |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Alex Chen (The Extension Creator)
- **Job / Role**: Senior Frontend Engineer & Extension Developer
- **Experience Level**: 8+ years software engineering experience
- **Organization Size**: Mid-Sized Software Startup (50 employees)
- **Technical Skill Level**: High (Proficient in JavaScript, TypeScript, WebExtension APIs, and Manifest V3).

#### 2. Goals
- Build feature-rich browser extensions and publish them successfully on Chrome Web Store and Edge Add-ons without policy rejections.
- Ensure proprietary extension code contains zero hardcoded API keys, unhandled `eval()` dynamic calls, or vulnerable third-party JavaScript dependencies.
- Pass enterprise security audits requested by B2B clients adopting his extension.

#### 3. Pain Points
- **Opaque Web Store Rejections**: Spent 3 weeks building a major release, only for Web Store reviewers to reject it without explaining which line of code violated policy.
- **Supply-Chain Dependency Risks**: Relies on third-party npm packages that might introduce hidden security vulnerabilities into his extension bundle.
- **Lack of Pre-Submission Audit Tools**: Has no reliable local tool to scan his extension build package (`dist.zip`) for Manifest V3 compliance before submission.

#### 4. Daily Workflow
- Writes TypeScript source code, bundles assets using Webpack/Vite, tests unpackaged extensions locally in Chrome Developer Mode, and submits packaged ZIP files to Web Store developer portals.

#### 5. Security Concerns
- Accidental exposure of private API secrets (AWS, Stripe, OpenAI) inside client-side JS bundles.
- Rejection or store delisting due to accidental inclusion of dangerous dynamic code calls.
- Unpatched CVEs in open-source JavaScript libraries included in his build package.

#### 6. Success Criteria
- Ability to run a pre-submission security audit on `dist.zip` in seconds.
- Itemized line-number report identifying exact policy violations, unhandled `eval()` calls, or exposed secrets.
- 100% confidence that submitted extensions pass Web Store review on the first attempt.

#### 7. Why They Choose AEP
Alex chooses AEP as a local pre-submission audit tool. Instead of waiting days for Web Store reviewers to reject his build, AEP gives him instant line-item feedback during his local development build process.

#### 8. Feature Priorities
1. **Pre-Submission SAST Scanner**: Scans local ZIP/CRX build packages directly.
2. **Hardcoded Secrets & API Key Detector**: Catches exposed tokens before Git commit.
3. **AST Dynamic Code Detector**: Highlights `eval()`, `Function()`, and `atob()` occurrences with line numbers.
4. **CVE Library Cross-Referencer**: Flags outdated third-party JS libraries in `dist/`.

#### 9. Typical Usage Scenario
Before submitting version 2.1 of his productivity extension to Chrome Web Store, Alex drops `build.zip` into AEP. Within 3 seconds, AEP highlights a line in `vendor.js` where an old utility library uses `eval()`, generating a Risk Score of $68.0$. Alex replaces the utility with a modern JSON parser, re-scans (Risk Score drops to $12.0$), and submits to the Web Store with complete confidence.

---

### Persona 3: Bug Bounty Hunter

```
+-----------------------------------------------------------------------------------+
|  NAME: Marcus Vance                   ROLE: Independent Security Researcher       |
|  EXPERIENCE: 5+ Years Vulnerability Research  ORG SIZE: Independent / Freelance   |
|  TECHNICAL SKILL: Expert (Offensive Cyber)    SECURITY AWARENESS: Expert          |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Marcus Vance (The Vulnerability Hunter)
- **Job / Role**: Independent Bug Bounty Hunter & Security Researcher
- **Experience Level**: 5+ years in Web Application & Browser Security research
- **Organization Size**: Independent Freelancer
- **Technical Skill Level**: Expert (Deep knowledge of AST parsing, reverse engineering, DOM XSS, Content Security Policy bypasses, and Chrome extension internals).

#### 2. Goals
- Rapidly triage popular browser extensions to discover high-severity zero-day vulnerabilities (DOM XSS, privilege escalation, credential theft).
- Submit high-quality, reproducible vulnerability reports to Bug Bounty programs (Bugcrowd, HackerOne) for financial bounties.
- Automate repetitive static analysis steps during initial target reconnaissance.

#### 3. Pain Points
- **Time-Consuming Manual Reconnaissance**: Manually unzipping `.crx` files, formatting minified code, and searching thousands of lines of JavaScript for attack vectors is tedious.
- **Obfuscation Barriers**: Malicious or suspicious extensions hide attack logic behind multi-layer string array obfuscation and dynamic execution routines.
- **Lack of AST Structural Analysis Tools**: Standard grep tools fail to catch complex structural code anomalies across minified extension bundles.

#### 4. Daily Workflow
- Downloads top 100 popular extensions from Web Stores, inspects manifest permission matrices, reverse-engineers background scripts, traces message-passing interfaces between content scripts and background service workers, and crafts proof-of-concept exploits.

#### 5. Security Concerns
- Spending days analyzing an extension only to find no exploitable vulnerabilities.
- Missing subtle DOM XSS sinks inside complex injected content scripts.

#### 6. Success Criteria
- Ability to audit 50 extension packages in an hour.
- Instant extraction of host permissions, dynamic execution calls, and hardcoded C2 network endpoints.
- High-precision identification of high-entropy code sections and obfuscation layers.

#### 7. Why They Choose AEP
Marcus chooses AEP as an automated triage acceleration engine. Instead of manually inspecting every file in an extension package, AEP automatically pinpoints the exact files and lines containing suspicious AST anomalies.

#### 8. Feature Priorities
1. **AST Structural Forensic Inspector**: Deep AST node breakdown highlighting dynamic execution and sinks.
2. **Entropy & Obfuscation Detector**: Pinpoints string array decoders and obfuscated code blocks.
3. **Hardcoded Domain & URL Extractor**: Maps all external network endpoints referenced in code.
4. **Deterministic Risk Score Engine**: Quickly ranks target extensions by threat score for triage priority.

#### 9. Typical Usage Scenario
Marcus targets a popular shopping assistant extension with 1 million installs. He opens the package in AEP. The AST scanner flags `content_script.js` line 412 for high code entropy ($+25.0$ points) and `chrome.tabs.executeScript` usage ($+20.0$ points). Opening the highlighted line, Marcus discovers an unhandled DOM injection sink, allowing him to craft a zero-day proof-of-concept exploit and claim a $3,000 bounty.

---

### Persona 4: Security Researcher & Malware Analyst

```
+-----------------------------------------------------------------------------------+
|  NAME: Dr. Aris Thorne                ROLE: Senior Threat Intelligence Researcher |
|  EXPERIENCE: 12+ Years Cyber Intelligence ORG SIZE: Global Cyber Security Vendor|
|  TECHNICAL SKILL: Expert (Reverse Eng)    SECURITY AWARENESS: Expert              |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Dr. Aris Thorne (The Malware Analyst)
- **Job / Role**: Senior Threat Intelligence Researcher at a Global Cybersecurity Vendor
- **Experience Level**: 12+ years in malware reverse engineering and threat intelligence
- **Organization Size**: Global Enterprise (1,000+ employees)
- **Technical Skill Level**: Expert (Specializes in reverse engineering, C2 infrastructure tracking, cryptographic hash correlation, and APT campaign attribution).

#### 2. Goals
- Uncover coordinated malicious extension campaigns operating on official Web Stores.
- Map extension attack techniques to global frameworks (such as MITRE ATT&CK).
- Publish technical threat intelligence reports and feed IoCs (Indicators of Compromise) to enterprise security clients.

#### 3. Pain Points
- **Evasive Malicious Campaigns**: Advanced persistent threat (APT) groups deploy benign extensions that dynamically load malicious payloads from external servers post-installation.
- **Correlating Campaign Infrastructure**: Difficulty mapping shared C2 domain infrastructure and code patterns across hundreds of malicious extension samples.
- **Lack of Standardized Threat Taxonomies**: Difficulty categorizing browser extension threat behaviors in standard threat intelligence formats.

#### 4. Daily Workflow
- Analyzes telemetry from global threat traps, dissects malicious extension binaries, performs cryptographic asset hashing (`SHA-256`), extracts C2 IP/domain indicators, and writes threat intelligence whitepapers.

#### 5. Security Concerns
- Malicious extension campaigns operating undetected for months, exfiltrating enterprise user data.
- Evasion techniques bypassing standard static signature detection engines.

#### 6. Success Criteria
- Rapid correlation of cryptographic asset hashes against threat intelligence databases.
- Automated extraction of hardcoded C2 infrastructure across extension bundles.
- Precise mapping of extension behaviors to MITRE ATT&CK TTP categories.

#### 7. Why They Choose AEP
Dr. Thorne uses AEP for deep static analysis and threat correlation. AEP’s AST parser and threat intelligence modules instantly extract hashes, network endpoints, and structural permission anomalies across large datasets of extension samples.

#### 8. Feature Priorities
1. **Threat Intelligence & Hash Lookup**: Cross-references asset SHA-256 hashes against known threat DBs.
2. **Hardcoded C2 & Network Map**: Extracts all IP addresses, WebSocket URLs, and domain endpoints.
3. **MITRE ATT&CK Mapping**: Categorizes behaviors (e.g. T1114 Data Exfiltration, T1056 Keylogging).
4. **AST Obfuscation & Dynamic Script SAST**: Deconstructs complex multi-layer JavaScript encodings.

#### 9. Typical Usage Scenario
Investigating a wave of compromised extensions, Dr. Thorne uploads 20 suspect `.crx` samples into AEP. AEP processes the batch, revealing that 14 samples share identical SHA-256 content script hashes and connect to the exact same hardcoded C2 domain. Dr. Thorne uses these findings to publish an industry threat report and update global blocklists.

---

### Persona 5: Blue Team / SOC Analyst

```
+-----------------------------------------------------------------------------------+
|  NAME: Sarah Jenkins                  ROLE: L2 SOC Analyst & Incident Responder   |
|  EXPERIENCE: 4 Years Security Ops        ORG SIZE: Enterprise Financial Bank      |
|  TECHNICAL SKILL: Moderate to High        SECURITY AWARENESS: High                |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Sarah Jenkins (The Frontline Defender)
- **Job / Role**: L2 SOC Analyst & Incident Responder
- **Experience Level**: 4 years in Security Operations Center (SOC) environments
- **Organization Size**: Enterprise Financial Institution (5,000+ employees)
- **Technical Skill Level**: Moderate to High (Skilled in SIEM monitoring, log analysis, alert triage, and incident response, but not a dedicated software developer).

#### 2. Goals
- Monitor enterprise endpoints for active data exfiltration threats and suspicious extension behavior.
- Rapidly triage security alerts without getting overwhelmed by false positives.
- Isolate and remediate compromised employee laptops before data loss occurs.

#### 3. Pain Points
- **EDR Blind Spot**: Her enterprise EDR agent alerts on OS-level process anomalies, but provides zero visibility when an employee's Chrome extension reads DOM data from internal banking portals.
- **Alert Fatigue**: Overwhelmed by thousands of daily SIEM events; needs clear, high-fidelity alerts when genuine high-risk extensions are detected.
- **Slow Remediation Workflows**: Lacks central tools to immediately identify which employees have a specific high-risk extension installed.

#### 4. Daily Workflow
- Monitors SIEM dashboards (Splunk, Elastic), investigates endpoint security alerts, conducts triage investigations, and coordinates endpoint isolation with IT teams.

#### 5. Security Concerns
- Employee credentials or customer financial records stolen via malicious browser extensions.
- Unpatched extension vulnerabilities exposing the internal corporate network to XSS or session hijacking.

#### 6. Success Criteria
- Clear, high-fidelity security alerts whenever an employee endpoint installs an extension with Risk Score $\ge 70.0$.
- Instant visibility into which employee laptops have the flagged extension installed.
- One-click export of forensic reports to attach to incident response tickets.

#### 7. Why They Choose AEP
Sarah relies on AEP to close the browser application layer blind spot. AEP feeds clean, deterministic extension risk data into her SOC workflow, giving her complete visibility over extension threats that her host EDR misses.

#### 8. Feature Priorities
1. **Real-Time SOC Alerting**: Dispatches high-fidelity alerts when high-risk extensions are detected.
2. **Itemized Risk Score Breakdown**: Provides clear mathematical evidence explaining the alert.
3. **SIEM / Webhook Integration**: Streams structured scan logs directly into Splunk/Elastic.
4. **PDF Forensic Report Export**: Generates clean executive reports for incident tickets.

#### 9. Typical Usage Scenario
At 09:15 AM, Sarah receives an AEP alert in her SIEM: *"Employee Laptop #412 installed extension 'ScreenCap Utility 1.4' (Risk Score: 88.5 - Critical Risk)."* AEP highlights that the extension requests `<all_urls>` host permissions and contains a hardcoded telemetry server. Sarah isolates the laptop, revokes the extension, and closes the incident in under 10 minutes.

---

### Persona 6: Enterprise Security Manager / CISO

```
+-----------------------------------------------------------------------------------+
|  NAME: David Ross                     ROLE: Chief Information Security Officer    |
|  EXPERIENCE: 18+ Years Executive Security ORG SIZE: Global Tech Enterprise (10k+)|
|  TECHNICAL SKILL: Executive / Strategic   SECURITY AWARENESS: Expert (Strategic)  |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: David Ross (The Enterprise Security Leader)
- **Job / Role**: Chief Information Security Officer (CISO)
- **Experience Level**: 18+ years in executive security leadership
- **Organization Size**: Global Tech Enterprise (10,000+ employees)
- **Technical Skill Level**: Executive / Strategic (Focuses on risk management, regulatory compliance, enterprise architecture, and security budgets).

#### 2. Goals
- Protect corporate IP, customer data, and brand reputation from security breaches.
- Ensure strict compliance with global data privacy frameworks (GDPR, SOC 2, ISO 27001, HIPAA).
- Eliminate shadow IT risks across the organization's browser environment.

#### 3. Pain Points
- **Zero Extension Governance**: Has no global visibility or policy control over the 3,000+ unique browser extensions installed across 10,000 employee laptops.
- **Audit Compliance Liabilities**: Auditor inquiries regarding browser application security cannot be answered due to lack of inventory reporting.
- **Third-Party Vendor Risk**: Employee usage of unvetted third-party AI/writing assistant extensions uploading corporate secrets to external servers.

#### 4. Daily Workflow
- Reviews enterprise risk posture dashboards, presents security metrics to the Board of Directors, establishes corporate security policies, and oversees CISO organization operations.

#### 5. Security Concerns
- Massive corporate data breach originating from an unmonitored employee browser extension.
- Multi-million dollar regulatory fines due to GDPR/HIPAA data exfiltration violations.

#### 6. Success Criteria
- Complete, centralized inventory of all browser extensions active across the enterprise fleet.
- Automated enforcement of extension installation policies (e.g. auto-blocking extensions with Risk Score $\ge 70.0$).
- High-level executive compliance reports showing enterprise risk reduction over time.

#### 7. Why They Choose AEP
David chooses AEP Enterprise Fleet Governance because it provides enterprise-wide extension visibility, policy enforcement, and audit compliance reporting without invading employee personal privacy or harvesting browsing history.

#### 8. Feature Priorities
1. **Centralized Enterprise Fleet Console**: Aggregates extension inventory across all endpoints.
2. **Automated Policy Enforcement**: Sets organization-wide threshold rules and blocking policies.
3. **Executive Compliance Reporting**: Generates SOC 2 / GDPR compliance audit summary reports.
4. **Privacy-First Telemetry Architecture**: Guarantees zero employee PII or raw browsing data collection.

#### 9. Typical Usage Scenario
Preparing for a annual SOC 2 audit, David opens the AEP Enterprise Console. The dashboard shows that enterprise extension risk posture improved by 74% over 6 months, with zero unauthorized high-risk extensions active across 10,000 laptops. David exports the executive compliance summary and presents it to auditors and the Board.

---

### Persona 7: IT Administrator

```
+-----------------------------------------------------------------------------------+
|  NAME: Robert Kowalski                ROLE: Senior Systems & Endpoint Admin       |
|  EXPERIENCE: 10 Years IT Operations      ORG SIZE: Mid-Sized Enterprise (1,200)   |
|  TECHNICAL SKILL: High (SysAdmin/DevOps) SECURITY AWARENESS: High                |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Robert Kowalski (The Endpoint Systems Admin)
- **Job / Role**: Senior Systems & Endpoint Administrator
- **Experience Level**: 10 years in IT Operations and Systems Administration
- **Organization Size**: Mid-Sized Enterprise (1,200 employees)
- **Technical Skill Level**: High (Proficient in Active Directory, Group Policy Objects (GPO), MDM solutions like Jamf/Intune, OS scripting, and endpoint deployment).

#### 2. Goals
- Deploy and manage software packages across 1,200 Windows and macOS laptops efficiently.
- Enforce corporate IT configuration standards without generating user support tickets.
- Maintain seamless endpoint agent health with minimal CPU and memory overhead.

#### 3. Pain Points
- **Heavy Bloatware Agents**: Existing security agents consume excessive CPU/RAM, causing employee laptops to slow down and generating constant IT helpdesk complaints.
- **Complex Agent Deployment**: Security software that requires complex kernel drivers or complex setup scripts creates deployment headaches.
- **Manual Extension Whitelisting**: Managing browser GPO extension extension blocklists manually is an endless, frustrating game of whack-a-mole.

#### 4. Daily Workflow
- Manages Microsoft Intune / Jamf Pro policy profiles, packages software installers (`.msi`, `.pkg`), monitors endpoint agent health, and resolves Tier 3 IT support escalations.

#### 5. Security Concerns
- Security software crashing employee endpoints or causing system instability.
- Unapproved software installations compromising endpoint integrity.

#### 6. Success Criteria
- Desktop Agent installs silently in <30 seconds via Intune/Jamf scripts.
- Desktop Agent runs lightweight with <30 MB RAM and <1% CPU usage.
- Automated synchronization of extension policies without manual GPO editing.

#### 7. Why They Choose AEP
Robert chooses AEP because the Desktop Agent is lightweight, installer-friendly (`.msi`/`.pkg`), runs cleanly without kernel drivers, and integrates seamlessly with Intune/Jamf endpoint deployment workflows.

#### 8. Feature Priorities
1. **Lightweight Endpoint Footprint**: Minimal CPU and RAM utilization (<30 MB RAM).
2. **Silent MDM Deployment Support**: Easy deployment via Intune, Jamf, GPO.
3. **Automated Extension Policy Sync**: Automatically pushes extension blocklists to browsers.
4. **Offline Local Agent Stability**: Operates reliably without network crashes or agent freezes.

#### 9. Typical Usage Scenario
Robert creates a deployment package for the AEP Desktop Agent in Microsoft Intune. He pushes the silent installer to 1,200 employee laptops overnight. By morning, all agents are running silently (<20 MB RAM footprint), reporting local extension inventories to the central console with zero helpdesk support tickets logged.

---

### Persona 8: Educational User (Student / University Researcher)

```
+-----------------------------------------------------------------------------------+
|  NAME: Clara Gomez                    ROLE: Computer Science Student & Researcher |
|  EXPERIENCE: 2nd Year University Student ORG SIZE: Educational University         |
|  TECHNICAL SKILL: Moderate (Learning CS)  SECURITY AWARENESS: Moderate            |
+-----------------------------------------------------------------------------------+
```

#### 1. Identity & Profile
- **Persona Name**: Clara Gomez (The Academic Learner)
- **Job / Role**: Computer Science Undergraduate Student & Student Researcher
- **Experience Level**: 2nd Year CS Student
- **Organization Size**: Educational University (20,000 students)
- **Technical Skill Level**: Moderate (Learning JavaScript, Python, Web Architecture, and Cybersecurity fundamentals).

#### 2. Goals
- Learn how web extension security models, Manifest V3 permissions, and static application security testing (SAST) work in practice.
- Audit extensions she uses for university study (grammar tools, PDF annotators, reference managers) to ensure her student research data is safe.
- Utilize open technical documentation and educational resources for university cybersecurity projects.

#### 3. Pain Points
- **Academic Resource Scarcity**: Commercial security tools are locked behind expensive enterprise paywalls that students cannot afford.
- **Complex Theoretical Concepts**: Finds security specifications difficult to apply to real-world code without concrete tools.
- **Limited Financial Budget**: Requires free or open-access tools for academic research and personal laptop security.

#### 4. Daily Workflow
- Attends university lectures, writes code in VS Code, uses Chrome for academic research, installs study aid extensions, and works on university lab assignments.

#### 5. Security Concerns
- Academic research papers or login credentials stolen by sketchy study extensions.
- Malware infection on her personal study laptop.

#### 6. Success Criteria
- Free access to local static analysis scanner capabilities for personal study.
- Clear, educational explanations connecting permission flags to actual code behavior.
- High-quality open documentation and architectural guidelines.

#### 7. Why They Choose AEP
Clara chooses AEP because it offers an open, transparent, privacy-first local scanner with clear educational value, detailed documentation, and itemized mathematical risk breakdowns.

#### 8. Feature Priorities
1. **Free Local Desktop Scanner**: Accessible local endpoint scanning capability.
2. **Itemized Mathematical Breakdown**: Shows exact point deduction rules for learning.
3. **Open Architecture Documentation**: Excellent educational resource for CS studies.
4. **AI Educational Explanations**: Explains technical concepts clearly.

#### 9. Typical Usage Scenario
Working on a web security coursework project, Clara runs AEP on her laptop to scan a free "Citation Generator" extension. AEP flags the extension for requesting `<all_urls>` and using `atob()` decoders. Clara uses AEP's itemized breakdown and open architectural docs to write her university lab report on browser permission security, earning top marks.

---

## 4. Persona Comparison Matrix

The matrix below compares all eight user personas across key decision attributes:

| Persona Attribute | 1. Home User (Maya) | 2. Developer (Alex) | 3. Bug Hunter (Marcus) | 4. Researcher (Dr. Thorne) | 5. SOC Analyst (Sarah) | 6. CISO (David) | 7. IT Admin (Robert) | 8. Student (Clara) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Technical Skill** | Low | High | Expert | Expert | Moderate-High | Executive | High | Moderate |
| **Security Awareness** | Moderate | High | Expert | Expert | High | Expert | High | Moderate |
| **Decision Authority** | Personal | Individual/Dev | Individual | Team Lead | Operations | Executive CISO | IT Operations | Personal |
| **Primary Goal** | Privacy Peace of Mind | Pre-Audit Code Build | Rapid Zero-Day Triage | Campaign Intelligence | Real-Time Incident Response | Fleet Governance | Lightweight MDM Deploy | Learn Security SAST |
| **Highest Priority Feature** | Local Auto Scanner | Pre-Submission SAST | AST Forensic Tree | Hash & C2 Lookup | SIEM SOC Alerts | Central Fleet Console | Intune/MDM Deploy | Free Local Scanner |
| **Expected Value Delivered** | Plain Risk Alerts | Zero Store Rejections | Fast Bounty Research | Deep Campaign Data | Fast Incident Triage | Audit Compliance | Zero RAM/CPU Complaints | Educational Clarity |

---

## 5. Primary Persona Designation for Version 1.0 (MVP)

```
+-----------------------------------------------------------------------------------+
|                     PRIMARY MVP PERSONA: MAYA LIN (EVERYDAY HOME USER)             |
+-----------------------------------------------------------------------------------+
|  REASON 1: Broadest Market Need       - Millions of consumers have zero visibility.|
|  REASON 2: Product Validation Proof   - If Maya understands it, anyone can.       |
|  REASON 3: Desktop Scanner Focus      - Matches Version 1.0 Local Agent Roadmap.  |
|  REASON 4: Lowest Friction Adoption   - Requires zero complex enterprise setup.   |
+-----------------------------------------------------------------------------------+
```

### 5.1 Business and Product Justification
**Persona 1: Maya Lin (Everyday Home User)** is officially designated as the **Primary Target Persona for Version 1.0 (MVP)**. 

The strategic justifications for this decision are:

1. **Alignment with Version 1.0 Roadmap Scope**: Version 1.0 focuses strictly on the local **Desktop Agent** (scanning installed extensions, reading manifests, executing local SAST, running the local Rule Engine, and dispatching OS notifications). Maya represents the ideal end-user for this standalone capability.
2. **The "Explainability" Acid Test**: If AEP's risk scores and AI explanations are clear enough for Maya (a non-technical consumer) to immediately understand and act upon, the platform will automatically succeed for technical users. Designing for non-technical users forces the product to maintain ultimate simplicity and clarity.
3. **Broadest Initial User Base**: Millions of non-technical users install browser extensions daily with zero security visibility. Solving Maya's pain point establishes immediate organic product adoption and brand trust.
4. **Zero Configuration Friction**: Maya requires zero complex network setup, cloud integrations, or enterprise policies. She simply installs the Desktop Agent and gains instant value.

---

## 6. Secondary Personas Analysis

While Maya Lin is our Primary MVP Target, the remaining seven personas remain vital to AEP’s multi-phase strategic roadmap:

### Phase 2 Target Expansion (Developers & Researchers)
- **Persona 2 (Developer - Alex)** and **Persona 3 (Bug Hunter - Marcus)** will become the primary focus during Version 1.5 - 2.0. As we introduce manual ZIP package uploading, deep AST forensic views, and secret scanners, developers and bug bounty hunters will drive core technical adoption.

### Phase 3 & 4 Target Expansion (In-Browser & Threat Intel)
- **Persona 4 (Researcher - Dr. Thorne)** and **Persona 8 (Student - Clara)** will be targeted as we roll out Threat Intelligence hash lookups, C2 blocklists, and the Chrome Companion Extension.

### Phase 5 Target Expansion (Enterprise Fleet Governance)
- **Persona 5 (SOC Analyst - Sarah)**, **Persona 6 (CISO - David)**, and **Persona 7 (IT Admin - Robert)** represent our **Commercial Enterprise Focus for Version 5.0**. Enterprise fleet management, SIEM webhooks, Intune deployment, and compliance reporting require a mature cloud infrastructure and will drive enterprise monetization.

---

## 7. Related Documents

- [`docs/VALUE_PROPOSITION.md`](file:///d:/ExtensionProtect/docs/VALUE_PROPOSITION.md) — Value Proposition Strategy
- [`docs/PRODUCT_VISION.md`](file:///d:/ExtensionProtect/docs/PRODUCT_VISION.md) — Strategic Product Vision & Roadmap
- [`docs/PROJECT_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/PROJECT_PRINCIPLES.md) — AEP Constitutional Principles
- [`docs/ENGINEERING_PRINCIPLES.md`](file:///d:/ExtensionProtect/docs/ENGINEERING_PRINCIPLES.md) — AEP Engineering Handbook
- [`docs/README.md`](file:///d:/ExtensionProtect/docs/README.md) — Master Documentation Index
