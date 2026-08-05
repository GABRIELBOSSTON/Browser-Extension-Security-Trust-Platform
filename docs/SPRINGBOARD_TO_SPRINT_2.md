# Springboard to Sprint 2

Sprint 1 successfully secured the pipeline orchestration, ensuring the agent won't panic under load and risk is calculated mathematically. 
**Sprint 2 introduces the true brain of the security engine: JavaScript Abstract Syntax Tree (AST) Inspection.**

## Sprint 2 Exact Objectives

The primary goal of Sprint 2 is to peer inside the raw `.js` payload files and augment the `ExtensionCapabilityModel` with definitive code-level threat behaviors.

### 1. SWC Parser Integration
Integrate the highly-performant Rust-based `swc` compiler toolchain. The agent must parse heavily obfuscated or minified Javascript and WebAssembly bindings into memory-safe AST trees.

### 2. AST Walker Implementation
Develop a generalized AST traversing service capable of visiting nodes across the entire extension lifecycle (Background workers, Content scripts, Popups) to aggregate structural semantics.

### 3. Call Graph & Source → Sink Analysis
Establish data-flow tracking. We must trace data entering from high-risk sources (e.g. DOM inputs, `chrome.cookies.get`) and map it to unauthorized external sinks (e.g. `fetch(C2_SERVER)`, `chrome.runtime.sendMessage`).

### 4. Secret Detection
Walk the syntax tree for exposed high-entropy API keys, OAuth tokens, and hardcoded symmetric cryptographic keys.

### 5. Dangerous API Detection
Flag unrestricted access to highly volatile Chromium APIs:
- `chrome.debugger`
- `chrome.processes`
- `chrome.proxy`
- `chrome.declarativeNetRequest` (if overly broad)

### 6. Eval & Dynamic Import Detection
Identify execution evasion techniques designed to bypass static store reviews.
- Flag any use of `eval()`, `setTimeout(string)`, or `new Function(string)`.
- Flag dynamic obfuscated injections via `atob()` or runtime `<script src="...">` appending.

### 7. Behavioral Analysis
Transition from structural capabilities to heuristic behavioral threat models. For instance, detecting keylogging not by seeing the word "keylogger", but by identifying a persistent `document.addEventListener('keydown', ...)` structurally mapped to an outbound asynchronous network request.
