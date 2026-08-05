use crate::infrastructure::scanner::provider::BrowserProvider;
use crate::infrastructure::scanner::models::BrowserScanResult;
use crate::infrastructure::scanner::providers::{
    ChromeProvider, EdgeProvider, BraveProvider, OperaStableProvider, OperaGXProvider
};

pub struct DiscoveryEngine {
    providers: Vec<Box<dyn BrowserProvider>>,
}

impl DiscoveryEngine {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Box::new(ChromeProvider),
                Box::new(EdgeProvider),
                Box::new(BraveProvider),
                Box::new(OperaStableProvider),
                Box::new(OperaGXProvider),
            ],
        }
    }

    pub fn scan_all(&self) -> Vec<BrowserScanResult> {
        self.providers
            .iter()
            .map(|provider| provider.scan())
            .collect()
    }
}
