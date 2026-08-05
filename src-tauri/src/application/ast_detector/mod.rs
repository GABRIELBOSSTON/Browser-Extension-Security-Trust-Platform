pub mod manager;
pub mod factory;
pub mod chrome_api_detector;
pub mod dangerous_api_detector;
pub mod secret_detector;

pub use manager::{DetectorRegistry, DetectorManager};
pub use factory::DetectorFactory;
pub use chrome_api_detector::ChromeApiDetector;
pub use dangerous_api_detector::DangerousApiDetector;
pub use secret_detector::{SecretDetector, PatternRegistry};
