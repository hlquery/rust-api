/**
 * Flush Example
 * 
 * Demonstrates the flush operation:
 * 1. Create a fake collection
 * 2. Create a fake document
 * 3. Check collection count
 * 4. Flush all data
 * 5. Re-check collection count (should be 0)
 */

use hlquery_rust_client::Client;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;
    
    println!("{}", "=".repeat(70));
    println!("FLUSH EXAMPLE");
    println!("{}", "=".repeat(70));
    println!();
    
    // Step 1: Create a fake collection
    println!("Step 1: Creating a fake collection...");
    let collection_name = format!("flush_test_collection_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    let schema = json!({
        "fields": [
            { "name": "title", "type": "string" },
            { "name": "content", "type": "string" },
            { "name": "value", "type": "int" }
        ]
    });
    
    let create_result = client.collections().create(&collection_name, schema).await?;
    if create_result.is_success() {
        println!("  ✓ Collection '{}' created successfully", collection_name);
    } else {
        println!("  ✗ Failed to create collection: {}", create_result.get_status_code());
        println!("  Error: {}", create_result.get_body());
        return Ok(());
    }
    
    println!();
    
    // Step 2: Create a fake document
    println!("Step 2: Creating a fake document...");
    let doc_id = format!("flush_test_doc_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    let doc = json!({
        "id": doc_id,
        "title": "Flush Test Document",
        "content": "This is a test document for flush example",
        "value": 42
    });
    
    let add_result = client.documents().add(&collection_name, doc).await?;
    if add_result.is_success() {
        println!("  ✓ Document '{}' added successfully", doc_id);
    } else {
        println!("  ✗ Failed to add document: {}", add_result.get_status_code());
        println!("  Error: {}", add_result.get_body());
    }
    
    println!();
    
    // Step 3: Check collection count before flush
    println!("Step 3: Checking collection count before flush...");
    let collections_before = client.list_collections(0, 1000).await?;
    let mut count_before = 0;
    if collections_before.is_success() {
        let body: serde_json::Value = serde_json::from_str(&collections_before.get_body().to_string())?;
        if let Some(collections) = body.get("collections").and_then(|c| c.as_array()) {
            count_before = collections.len();
            println!("  Collections before flush: {}", count_before);
            if count_before == 0 {
                println!("  ⚠ Warning: No collections found before flush");
            }
        }
    } else {
        println!("  ✗ Failed to list collections: {}", collections_before.get_status_code());
    }
    
    println!();
    
    // Step 4: Flush all data
    println!("Step 4: Flushing all data...");
    let flush_result = client.flush().await?;
    if flush_result.is_success() {
        let body: serde_json::Value = serde_json::from_str(&flush_result.get_body().to_string())?;
        let collections_deleted = body.get("collections_deleted")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        println!("  ✓ Flush completed successfully");
        println!("  Collections deleted: {}", collections_deleted);
        let message = body.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("  Message: {}", message);
    } else {
        println!("  ✗ Flush failed: {}", flush_result.get_status_code());
        println!("  Error: {}", flush_result.get_body());
        return Ok(());
    }
    
    println!();
    
    // Step 5: Re-check collection count after flush
    println!("Step 5: Checking collection count after flush...");
    let collections_after = client.list_collections(0, 1000).await?;
    let mut count_after = -1;
    if collections_after.is_success() {
        let body: serde_json::Value = serde_json::from_str(&collections_after.get_body().to_string())?;
        if let Some(collections) = body.get("collections").and_then(|c| c.as_array()) {
            count_after = collections.len() as i32;
            println!("  Collections after flush: {}", count_after);
            
            if count_after == 0 {
                println!("  ✓ SUCCESS: All collections have been flushed");
            } else {
                println!("  ⚠ Warning: Expected 0 collections, but found {}", count_after);
            }
        }
    } else {
        println!("  ✗ Failed to list collections: {}", collections_after.get_status_code());
    }
    
    println!();
    println!("{}", "=".repeat(70));
    println!("FLUSH EXAMPLE COMPLETED");
    println!("{}", "=".repeat(70));
    println!("Summary:");
    println!("  Collections before flush: {}", count_before);
    println!("  Collections after flush: {}", count_after);
    println!();
    
    Ok(())
}
