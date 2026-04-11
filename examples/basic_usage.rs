/**
 * Basic Usage Examples
 * 
 * Demonstrates basic operations with the hlquery Rust client
 */

use hlquery_rust_client::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new("http://localhost:9200", None)?;
    
    // Health check
    let health = client.health().await?;
    println!("Health Status: {}", health.get_status_code());
    println!("Health Body: {}", health.get_body());
    
    // List collections
    let collections = client.list_collections(0, 10).await?;
    if collections.is_success() {
        let body = collections.get_body();
        if let Some(collections_array) = body.get("collections").and_then(|c| c.as_array()) {
            println!("Found {} collections", collections_array.len());
        }
    }
    
    // With authentication
    let mut options = HashMap::new();
    options.insert("token".to_string(), "your_token_here".to_string());
    options.insert("auth_method".to_string(), "bearer".to_string());
    let authenticated_client = Client::new("http://localhost:9200", Some(options))?;
    
    // Or set token dynamically
    client.set_auth_token("your_token_here".to_string(), "bearer".to_string());
    
    Ok(())
}
