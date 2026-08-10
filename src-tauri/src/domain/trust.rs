pub struct TrustRegistry;

impl TrustRegistry {
    /// Returns true if the given extension ID is in the trusted allowlist.
    pub fn is_trusted(extension_id: &str) -> bool {
        // A hardcoded list of known-good extension IDs for the Accuracy Hardening Sprint.
        // In a future iteration, this could be loaded from an external JSON file or remote endpoint.
        let trusted_ids = [
            // Google Docs Offline
            "ghbmnnjooekpmoecnnnilnnbdlolhkhi",
            // Add other known-good IDs here as needed
        ];
        
        trusted_ids.contains(&extension_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_trusted() {
        assert!(TrustRegistry::is_trusted("ghbmnnjooekpmoecnnnilnnbdlolhkhi"));
        assert!(!TrustRegistry::is_trusted("unknown_extension_id"));
    }
}
