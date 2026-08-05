# Known Limitations (v0.1.0)

While Sprint 1 establishes a rock-solid static analysis orchestration pipeline, the system in its v0.1.0 RC state has strict functional boundaries. The following features are unsupported or intentionally postponed to future sprints.

## 1. Unsupported AST Analysis
v0.1.0 does not peek inside JavaScript files. It is entirely blind to internal source code vectors (e.g. keylogging logic, cryptocurrency mining loops). The current risk score is derived entirely from the extension's structural `manifest.json` metadata (requested permissions, host patterns, externally connectable resources).

## 2. Unsupported Dynamic Analysis
AEP is inherently a Static Application Security Testing (SAST) tool. It will **never** support dynamic analysis (DAST), meaning it will not execute the extension in a headless browser to monitor live network traffic. All risk is calculated strictly from the static package.

## 3. Unsupported Browsers
The architecture is designed to support Chrome, Edge, Brave, and Opera. However, the `DiscoveryEngine` (auto-locating installed profiles on the OS) is currently postponed. The system must be fed manual, raw extension extraction paths via the pipeline entry point.

## 4. Unsupported Manifest Features
- Complex multi-language localization parsing (`_locales/`) is omitted.
- Declarative Net Request (DNR) rule evaluation (MV3 ad-blocking formats) is not yet deeply mapped into the `ExtensionCapabilityModel`.

## 5. Technical Debt
- **Blocking File I/O**: The asynchronous Tokio pipeline currently relies on synchronous `std::fs` operations deep inside the Manifest Service. Under extreme loads (e.g., batch scanning 1,000 massive extensions concurrently), this may temporarily stall the async runtime scheduler.
- **Strict MatchPattern Parsing**: The parser for URLs relies on naive string splitting rather than a fully compliant Chromium `MatchPattern` regex evaluation matrix.
