/**
 * SAM Examples
 *
 * Demonstrates SAM search, status, and history with the hlquery Rust client
 */
use hlquery_rust_client::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let client = Client::new("http://localhost:9200", None)?;
    let sam = client.sam();

    let status = sam.status(Some("music"), None).await?;
    println!("SAM status: {}", status.get_body());

    let mut params = HashMap::new();
    params.insert("limit".to_string(), "5".to_string());

    let search = sam.search("music", "queen of pop", Some(params)).await?;
    println!("SAM search: {}", search.get_body());

    let history = sam.history(Some("music"), 5, None).await?;
    println!("SAM history: {}", history.get_body());

    Ok(())
}
