use antigraviiti_extension_protect::application::discovery::service::DiscoveryService;

fn main() {
    match DiscoveryService::execute_discovery() {
        Ok(result) => {
            let json = serde_json::to_string_pretty(&result).unwrap();
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
