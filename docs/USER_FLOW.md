# User Journeys & Flow Diagrams — Antigraviiti Extension Protect (AEP)

---

## 1. Overview of Primary User Journeys

The **Antigraviiti Extension Protect (AEP)** platform accommodates four core user interaction flows across its multi-component ecosystem:

1. **Journey 1: Automated Endpoint Monitoring (Desktop Agent)**
2. **Journey 2: Forensic Investigation & Audit (Web Dashboard)**
3. **Journey 3: Developer Pre-Submission Analysis (Manual Upload)**
4. **Journey 4: In-Browser Quick Status & Alerts (Chrome Companion)**

---

## 2. Journey 1: Automated Endpoint Monitoring (Desktop Agent)

```mermaid
sequenceDiagram
    autonumber
    actor User as Endpoint User
    participant Agent as Desktop Agent Daemon
    participant OS as OS Notification System
    participant Cloud as AEP Cloud API
    participant Ext as Browser Extensions

    User->>Agent: Installs & Starts Desktop Agent
    Agent->>Ext: Scans local profile directories (Chrome, Edge, Brave)
    Ext-->>Agent: Returns 12 installed extensions
    Agent->>Agent: Extracts manifest.json & computes file SHA256 hashes
    Agent->>Cloud: POST /api/v1/agent/sync (Anonymized Telemetry)
    Cloud->>Cloud: Executes Rule Engine & Threat Intel Check
    Cloud-->>Agent: Returns Scan Results (1 High Risk Extension Detected!)
    
    alt Risk Score >= 70.0 (High Risk)
        Agent->>OS: Fires OS Native Desktop Notification Banner
        OS-->>User: Alerts: "High Risk Extension Detected: 'WebHelper 1.2'"
        User->>Agent: Clicks Notification
        Agent->>User: Opens Web Dashboard at /scan/results/uuid
    else Risk Score < 70.0 (Low/Medium Risk)
        Agent->>Agent: Logs silent scan event in local sqlite/log
    end
```

---

## 3. Journey 2: Forensic Investigation & Audit (Web Dashboard)

```mermaid
stateDiagram-v2
    [*] --> DashboardOverview : User Logs In / Navigates to Dashboard
    
    DashboardOverview --> ScanHistoryList : Views Recent Extension Scans
    DashboardOverview --> UploadModal : Clicks "Upload CRX / ZIP"
    
    ScanHistoryList --> ExtensionDetailView : Selects Specific Extension (e.g. WhatsApp Plus)
    
    state ExtensionDetailView {
        [*] --> RiskScoreSummary : Views Risk Score & AI Executive Summary
        RiskScoreSummary --> PermissionMatrix : Inspects Manifest Permissions
        RiskScoreSummary --> ASTForensicInspector : Views Flagged Obfuscation & Dangerous APIs
        RiskScoreSummary --> NetworkDomainMap : Inspects Hardcoded URLs & Telemetry Domains
        RiskScoreSummary --> CVELibraryList : Inspects Outdated Vulnerable JS Libraries
    }
    
    ExtensionDetailView --> PDFReportGeneration : Clicks "Export Security PDF Report"
    PDFReportGeneration --> [*] : Downloads Clean PDF Report
```

---

## 4. Journey 3: Developer Pre-Submission Audit (Manual Upload)

```mermaid
sequenceDiagram
    autonumber
    actor Dev as Extension Developer
    participant Web as Web Dashboard UI
    participant API as Cloud API Gateway
    participant Engine as SAST & Risk Engine
    participant AI as AI Service

    Dev->>Web: Drag & drops build package (`dist.zip`)
    Web->>API: Uploads multipart/form-data archive
    API->>Engine: Initiates Async Scan Pipeline
    Web->>Web: Displays progress bar (Parsing -> SAST -> Intel -> AI)
    
    Engine->>Engine: Detects unhandled `eval()` on line 142 of background.js
    Engine->>Engine: Detects over-broad permission `<all_urls>`
    Engine->>Engine: Calculates Risk Score = 65.4 (Medium Risk)
    
    Engine->>AI: Requests developer remediation summary
    AI-->>Engine: Returns explanation: "Replace eval() with JSON.parse()"
    
    Engine-->>API: Persists results
    API-->>Web: Returns full JSON scan output
    Web->>Dev: Renders interactive report with highlighted code lines & recommendations
```

---

## 5. Journey 4: In-Browser Quick Status (Chrome Companion Extension)

```mermaid
sequenceDiagram
    autonumber
    actor User as Browser User
    participant Toolbar as Browser Toolbar (Companion Ext)
    participant Companion as Companion Background Worker
    participant LocalIPC as Desktop Agent (Local WS)
    participant Dashboard as Web Dashboard UI

    User->>Toolbar: Clicks AEP Extension Icon on Toolbar
    Toolbar->>Companion: Requests active tab & installed extension risk state
    Companion->>LocalIPC: WS Query: GET /agent/status (127.0.0.1:49152)
    LocalIPC-->>Companion: Returns JSON: Active risk status (1 Warning)
    
    Companion->>Toolbar: Renders Popup UI:
    Note over Toolbar: - Overall Health: 🟡 Warning<br/>- Installed: 14 Extensions<br/>- High Risk: 1 Extension ('SuperVPN Free')
    
    User->>Toolbar: Clicks "Investigate in Dashboard"
    Toolbar->>Dashboard: Opens new tab at https://app.antigraviiti.com/fleet
```
