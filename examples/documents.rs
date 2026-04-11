/**
 * Documents Examples
 * 
 * Demonstrates document CRUD operations
 */

use hlquery_rust_client::Client;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;
    
    // List documents
    let mut params = HashMap::new();
    params.insert("offset".to_string(), "0".to_string());
    params.insert("limit".to_string(), "10".to_string());
    let docs = client.documents().list("collection", Some(params)).await?;
    println!("Documents: {}", docs.get_body());
    
    // Get document
    let doc = client.documents().get("collection", "doc_id").await?;
    println!("Document: {}", doc.get_body());
    
    // Add document
    let new_doc = json!({
        "id": "doc_1",
        "title": "New Document",
        "content": "Document content"
    });
    let add_result = client.documents().add("collection", new_doc).await?;
    println!("Add result: {}", add_result.get_body());
    
    // Update document
    let updated_doc = json!({
        "title": "Updated Document",
        "content": "Updated content"
    });
    let update_result = client.documents().update("collection", "doc_id", updated_doc).await?;
    println!("Update result: {}", update_result.get_body());
    
    // Delete document
    let delete_result = client.documents().delete("collection", "doc_id").await?;
    println!("Delete result: {}", delete_result.get_body());
    
    // Bulk import
    let bulk_docs = vec![
        json!({ "id": "doc1", "title": "Doc 1" }),
        json!({ "id": "doc2", "title": "Doc 2" }),
        json!({ "id": "doc3", "title": "Doc 3" })
    ];
    let import_result = client.documents().import("collection", bulk_docs).await?;
    println!("Import result: {}", import_result.get_body());
    
    Ok(())
}
