use antigraviiti_extension_protect::infrastructure::scanner::DiscoveryEngine;
use std::time::Instant;

fn main() {
    println!("--- EXTENSION DISCOVERY ENGINE VALIDATION ---");
    let start = Instant::now();
    let engine = DiscoveryEngine::new();
    let results = engine.scan_all();
    let duration = start.elapsed();

    let mut total_extensions = 0;
    
    println!("Scan Duration: {:?}", duration);
    println!("\nBrowsers Detected:");
    for result in &results {
        println!("- {:?}", result.browser_family);
        if let Some(err) = &result.error {
            println!("  Error: {}", err);
        }
        
        let count = result.extensions.len();
        total_extensions += count;
        println!("  Extensions Found: {}", count);
        
        if count > 0 {
            // Count profiles
            let mut profiles = std::collections::HashSet::new();
            for ext in &result.extensions {
                profiles.insert(&ext.profile_name);
            }
            println!("  Profiles Detected: {:?}", profiles);
        }
    }
    
    println!("\nTotal Extensions Discovered: {}", total_extensions);
    println!("----------------------------------------------");
}
