/**
 * Collections Examples
 * 
 * Demonstrates collection management operations
 */

use hlquery_rust_client::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;
    
    // List collections
    let collections = client.collections().list(0, 10).await?;
    println!("Collections: {}", collections.get_body());
    
    // Get collection
    let collection = client.collections().get("my_collection").await?;
    println!("Collection details: {}", collection.get_body());
    
    // Create collection
    let schema = json!({
        "fields": [
            { "name": "title", "type": "string" },
            { "name": "content", "type": "string" },
            { "name": "embedding", "type": "float[]" }
        ]
    });
    let create_result = client.collections().create("new_collection", schema).await?;
    println!("Create result: {}", create_result.get_body());
    
    // Get formatted fields
    let fields = client.collections().get_fields("my_collection").await?;
    println!("Formatted fields: {}", fields.get_body());
    
    // Delete collection
    let delete_result = client.collections().delete("collection_name").await?;
    println!("Delete result: {}", delete_result.get_body());
    
    Ok(())
}
