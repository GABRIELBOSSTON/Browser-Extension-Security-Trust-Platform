use super::manager::DetectorRegistry;

pub struct DetectorFactory;

impl DetectorFactory {
    pub fn create_registry(_profile: &str) -> DetectorRegistry {
        // Current sprint: Only instantiates Mock Detectors if needed.
        // We return an empty registry by default. Real configurations will populate this.
        DetectorRegistry::new()
    }
}
