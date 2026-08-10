# Security Notes

## GitHub Secret Scanning Alerts
If GitHub Secret Scanning detects a credential in this repository, please review the alert carefully.

### Known False Positives (Synthetic Test Fixtures)
The Browser Extension Security Trust Platform includes an IOC (Indicator of Compromise) scanning engine. To ensure the engine correctly identifies embedded secrets in extensions, the test suite (`SecretIocDetector` tests in `src-tauri/src/application/ioc/detector.rs` and `engine.rs`) must simulate malicious files containing patterns that match typical credentials.

To avoid triggering GitHub's Secret Scanning, these synthetic strings are constructed dynamically at runtime (e.g., via `format!()` concatenating string fragments). However, if an alert is triggered on this repository:

1. **Google API Keys**: Fragments like `AIzaSy` are used in tests. They do not represent real keys and are not associated with any GCP projects.
2. **AWS Keys**: Fragments like `AKIA` and `IOSFODNN7EXAMPLE` are used for AWS secret detection testing.
3. **GitHub PATs**: Fragments like `ghp_` combined with dummy characters are used.
4. **Stripe Keys**: `sk_live_`, `sk_test_`, etc.
5. **Private Keys & JWTs**: Dummy headers and synthetic base64 blobs are used to ensure the detector correctly flags them.

**Action**: Such alerts can be safely dismissed in GitHub as **False Positive / Test Data**, or treated as known safe artifacts. Do not delete the test coverage.

**Do not hardcode full credential patterns** in the source files. Always split them or obfuscate them if adding new test fixtures.

## General Security Guidelines
When contributing to this repository, you must adhere to the following rules:
- **Never commit real secrets**: Do not commit any real passwords, API keys, access tokens, or private keys to the repository.
- **Synthetic IOC fixtures must be non-functional**: Any credential-like patterns used for testing the IOC engine must be clearly synthetic, non-functional, and obfuscated (e.g., via string splitting).
- **Environment variables for local secrets**: Local secrets (such as `VT_API_KEY`) should be provided via environment variables. Refer to `.env.example`. Do not commit `.env` files.
- **Security reports**: Do not include sensitive local machine information, such as IP addresses, real browser extension IDs from local machines, or absolute paths, in tracked security reports or test artifacts.
