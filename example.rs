/**
 * hlquery Rust API Comprehensive Example
 * 
 * This example demonstrates the main features of the hlquery Rust client:
 * - Health checks
 * - Authentication (with and without token)
 * - Listing collections
 * - Getting collection fields
 * - Listing documents with pagination
 * - Multiple search methods
 * - Dynamic authentication
 * 
 * Usage: cargo run --example example [command] [token]
 *   Commands:
 *     cols   - Run collections API examples
 *     docs   - Run documents API examples
 *     open   - List and open collections (interactive)
 *     status - Show server health and status information
 *     help   - Show this help message
 *     all    - Run all examples (default)
 */

use hlquery_rust_client::Client;
use serde_json::json;
use std::collections::HashMap;
use std::env;

// Helper function to print results
fn print_result(title: &str, response: &hlquery_rust_client::Response, print_body: bool) {
    println!("{}", "=".repeat(70));
    println!("TEST: {}", title);
    println!("{}", "-".repeat(70));
    
    let status = response.get_status_code();
    let body = response.get_body();
    
    println!("Status Code: {}", status);
    
    if print_body {
        println!("Response Body:");
        println!("{}", serde_json::to_string_pretty(body).unwrap_or_default());
    }
    
    if status >= 200 && status < 300 {
        println!("✓ SUCCESS");
    } else {
        println!("✗ FAILED");
    }
    println!();
}

// Helper to get first collection name
async fn get_first_collection(client: &Client) -> Option<String> {
    if let Ok(collections) = client.list_collections(0, 1).await {
        if collections.get_status_code() == 200 {
            if let Some(collections_array) = collections.get_body().get("collections").and_then(|c| c.as_array()) {
                if let Some(first) = collections_array.get(0) {
                    if let Some(name) = first.get("name").and_then(|n| n.as_str()) {
                        return Some(name.to_string());
                    } else if let Some(name) = first.as_str() {
                        return Some(name.to_string());
                    }
                }
            }
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("HLQ_BASE_URL")
        .or_else(|_| env::var("HLQUERY_BASE_URL"))
        .unwrap_or_else(|_| "http://localhost:9200".to_string());
    
    // Parse command line arguments
    let mut args: Vec<String> = env::args().skip(1).collect();
    let (command, test_token, offset, limit, collection_name) = if args.is_empty() {
        ("all".to_string(), None, 0, 1000, None)
    } else {
        let first = args[0].clone();
        if ["cols", "docs", "open", "status", "help", "all"].contains(&first.as_str()) {
            let mut offset = 0;
            let mut limit = 1000;
            let mut token = None;
            let mut coll_name = None;
            
            // Parse pagination for cols command: cols [offset] [limit] [token]
            if first == "cols" && args.len() >= 2 {
                if let Ok(off) = args[1].parse::<i32>() {
                    offset = off;
                    if args.len() >= 3 {
                        if let Ok(lim) = args[2].parse::<i32>() {
                            limit = lim;
                            token = args.get(3).cloned();
                        } else {
                            token = args.get(2).cloned();
                        }
                    }
                } else {
                    token = args.get(1).cloned();
                }
            } else if first == "docs" && args.len() >= 2 {
                // For docs command: docs [collection_name] [token]
                coll_name = args.get(1).cloned();
                token = args.get(2).cloned();
            } else {
                // Look for token in remaining args (skip numeric args)
                for arg in args.iter().skip(1) {
                    if arg.parse::<i32>().is_err() {
                        token = Some(arg.clone());
                        break;
                    }
                }
            }
            (first, token, offset, limit, coll_name)
        } else {
            ("all".to_string(), Some(first), 0, 1000, None)
        }
    };
    
    // Show help if requested
    if command == "help" {
        println!("=== hlquery Rust API Example ===\n");
        println!("Usage: cargo run --example example [command] [args...] [token]\n");
        println!("Commands:");
        println!("  cols   - List collections (with pagination)");
        println!("          Usage: cols [offset] [limit] [token]");
        println!("          Example: cols 0 200 (list first 200 collections)");
        println!("  docs   - Run documents API examples");
        println!("          Usage: docs [collection_name] [token]");
        println!("          Example: docs my_collection");
        println!("  open   - List and open collections (interactive)");
        println!("  status - Show server health and status information");
        println!("  help   - Show this help message");
        println!("  all    - Run all examples (default)\n");
        println!("Authentication:");
        println!("  Token is optional. Only provide if server requires authentication.");
        println!("  Example: cargo run --example example cols 0 200 my_token");
        println!("  Example: cargo run --example example docs my_collection my_token\n");
        println!("Examples:");
        println!("  cargo run --example example");
        println!("  cargo run --example example cols");
        println!("  cargo run --example example cols 0 200");
        println!("  cargo run --example example docs my_collection");
        println!("  cargo run --example example status");
        return Ok(());
    }
    
    println!("=== hlquery Rust API Example ===");
    println!("Command: {}\n", command);
    
    // Create client
    // Optional: Set authentication token if provided
    // Uncomment and set token if your server requires authentication:
    // let auth_token = "your_token_here";
    
    let mut options = HashMap::new();
    if let Some(ref token) = test_token {
        options.insert("token".to_string(), token.clone());
        options.insert("auth_method".to_string(), "bearer".to_string());
        println!("Using authentication token: {}...\n", &token[..token.len().min(8)]);
    }
    
    let client = Client::new(base_url, if options.is_empty() { None } else { Some(options) })?;
    
    // ----------------------------------------------------------------====================================
    // STATUS COMMAND
    // ----------------------------------------------------------------====================================
    if command == "status" {
        println!("\n{}", "#".repeat(70));
        println!("# SERVER STATUS");
        println!("{}\n", "#".repeat(70));
        
        // GET /health
        print_result("GET /health", &client.health().await?, true);
        
        // GET /stats
        print_result("GET /stats", &client.stats().await?, true);
        
        // GET /status
        let status = client.execute_request("GET", "/status", None, None).await?;
        print_result("GET /status", &status, true);
        
        // GET / (Root Info) - show concise version
        let info = client.info().await?;
        println!("{}", "=".repeat(70));
        println!("TEST: GET / (Root Info)");
        println!("{}", "-".repeat(70));
        let status_code = info.get_status_code();
        let body = info.get_body();
        println!("Status Code: {}", status_code);
        if status_code >= 200 && status_code < 300 {
            if let Some(name) = body.get("name").and_then(|n| n.as_str()) {
                println!("Name: {}", name);
            } else {
                println!("Name: N/A");
            }
            if let Some(version) = body.get("version").and_then(|v| v.as_str()) {
                println!("Version: {}", version);
            } else {
                println!("Version: N/A");
            }
            if let Some(description) = body.get("description").and_then(|d| d.as_str()) {
                println!("Description: {}", description);
            } else {
                println!("Description: N/A");
            }
            println!("✓ SUCCESS");
        } else {
            println!("Response Body:");
            println!("{}", serde_json::to_string_pretty(body).unwrap_or_default());
            println!("✓ SUCCESS");
        }
        println!();
        
        return Ok(());
    }
    
    // ----------------------------------------------------------------====================================
    // System APIs (only for 'all' command)
    // ----------------------------------------------------------------====================================
    if command == "all" {
        println!("\n{}", "#".repeat(70));
        println!("# System APIs");
        println!("{}\n", "#".repeat(70));
        
        // GET /health
        print_result("GET /health", &client.health().await?, true);
        
        // GET /stats
        print_result("GET /stats", &client.stats().await?, true);
        
        // GET /metrics (Prometheus-compatible)
        let metrics = client.execute_request("GET", "/metrics", None, None).await?;
        print_result("GET /metrics", &metrics, true);
        
        // GET /status
        let status = client.execute_request("GET", "/status", None, None).await?;
        print_result("GET /status", &status, true);
        
        // GET /
        print_result("GET / (Root)", &client.info().await?, true);
    }
    
    // ----------------------------------------------------------------====================================
    // COLLECTIONS API
    // ----------------------------------------------------------------====================================
    if command == "all" || command == "cols" || command == "open" {
        println!("\n{}", "#".repeat(70));
        println!("# COLLECTIONS API");
        println!("{}\n", "#".repeat(70));
        
        // GET /collections with pagination
        let mut doc_params = HashMap::new();
        doc_params.insert("offset".to_string(), offset.to_string());
        doc_params.insert("limit".to_string(), limit.to_string());
        let collections = client.list_collections(offset, limit).await?;
        
        if command == "cols" {
            // Simple list display for cols command
            if collections.get_status_code() == 200 {
                if let Some(collections_array) = collections.get_body().get("collections").and_then(|c| c.as_array()) {
                    let total = collections_array.len();
                    println!("Collections (showing {}, offset: {}, limit: {}):\n", total, offset, limit);
                    for col in collections_array {
                        let name = col.get("name")
                            .and_then(|n| n.as_str())
                            .or_else(|| col.as_str())
                            .unwrap_or("unknown");
                        println!("  {}", name);
                    }
                    println!();
                } else {
                    println!("No collections found.\n");
                }
            } else {
                println!("Error: {}\n", collections.get_status_code());
                if let Some(msg) = collections.get_body().get("message").and_then(|m| m.as_str()) {
                    println!("Message: {}\n", msg);
                }
            }
        } else {
            // Full display for 'all' and 'open' commands
            print_result("GET /collections (List)", &collections, true);
            
            // Get first collection for other tests
            let first_collection = get_first_collection(&client).await;
            
            if let Some(ref collection_name) = first_collection {
        println!("Using collection: {}\n", collection_name);
        
        // GET /collections/{name}
        print_result(
            &format!("GET /collections/{}", collection_name),
            &client.get_collection(collection_name).await?,
            true,
        );
        
        // GET /collections/{name}/fields (formatted)
        print_result(
            &format!("GET /collections/{}/fields (formatted)", collection_name),
            &client.get_collection_fields(collection_name).await?,
            true,
        );
        
        // Test creating a temporary collection
        let test_collection_name = format!("test_collection_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        let test_schema = json!({
            "fields": [
                { "name": "title", "type": "string" },
                { "name": "content", "type": "string" },
                { "name": "embedding", "type": "float[]" }
            ]
        });
        
        // POST /collections
        let create_result = client.collections().create(&test_collection_name, test_schema.clone()).await?;
        print_result("POST /collections (Create)", &create_result, true);
        
        if create_result.get_status_code() == 200 || create_result.get_status_code() == 201 {
            // POST /collections/{name}/update
            let update_schema = json!({
                "fields": [
                    { "name": "title", "type": "string" },
                    { "name": "content", "type": "string" },
                    { "name": "embedding", "type": "float[]" },
                    { "name": "tags", "type": "string[]" }
                ]
            });
            let update_result = client.collections().update(&test_collection_name, update_schema).await?;
            print_result("POST /collections/{name}/update", &update_result, true);
            
            // DELETE /collections/{name} (cleanup)
            let delete_result = client.collections().delete(&test_collection_name).await?;
            print_result("DELETE /collections/{name}", &delete_result, true);
        }
        
        // For 'open' command, list all collections
        if command == "open" {
            let collections = client.list_collections(0, 100).await?;
            if collections.get_status_code() == 200 {
                if let Some(collections_array) = collections.get_body().get("collections").and_then(|c| c.as_array()) {
                    println!("\nAvailable Collections:");
                    println!("{}", "-".repeat(70));
                    for col in collections_array {
                        let name = col.get("name")
                            .and_then(|n| n.as_str())
                            .or_else(|| col.as_str())
                            .unwrap_or("unknown");
                        println!("  - {}", name);
                    }
                    println!();
                }
            }
        }
    } else {
        println!("No collections found - skipping collection-specific tests\n");
    }
    
    // ----------------------------------------------------------------====================================
    // DOCUMENTS API
    // ----------------------------------------------------------------====================================
    if command == "all" || command == "docs" || command == "open" {
        println!("\n{}", "#".repeat(70));
        println!("# DOCUMENTS API");
        println!("{}\n", "#".repeat(70));
        
        // Use provided collection name or get first collection
        let test_collection = collection_name.clone().or_else(|| {
            // This will be handled in the async block
            None
        });
        let test_collection = if let Some(ref name) = collection_name {
            Some(name.clone())
        } else {
            get_first_collection(&client).await
        };
        
        if test_collection.is_none() {
            if command == "docs" && collection_name.is_none() {
                println!("Error: No collection name provided and no collections found.");
                println!("Usage: cargo run --example example docs [collection_name] [token]\n");
            } else {
                println!("No collections available - skipping document tests\n");
            }
        } else {
            let collection_name_str = test_collection.unwrap();
            if command == "docs" && collection_name.is_some() {
                println!("Using collection: {}\n", collection_name_str);
            }
            
            // GET /collections/{name}/documents
            let mut doc_params = HashMap::new();
            doc_params.insert("offset".to_string(), "0".to_string());
            doc_params.insert("limit".to_string(), limit.to_string());
            let documents = client.list_documents(&collection_name_str, Some(doc_params)).await?;
            
            if command == "docs" {
                // Simple list display for docs command
                if documents.get_status_code() == 200 {
                    if let Some(docs_array) = documents.get_body().get("documents").and_then(|d| d.as_array()) {
                        let total = docs_array.len();
                        println!("Documents in '{}' (showing {}, limit: {}):\n", collection_name_str, total, limit);
                        for doc in docs_array {
                            let doc_id = doc.get("id")
                                .and_then(|id| id.as_str())
                                .or_else(|| doc.as_str())
                                .unwrap_or("unknown");
                            println!("  {}", doc_id);
                        }
                        println!();
                    } else {
                        println!("No documents found.\n");
                    }
                } else {
                    println!("Error: {}\n", documents.get_status_code());
                    if let Some(msg) = documents.get_body().get("message").and_then(|m| m.as_str()) {
                        println!("Message: {}\n", msg);
                    }
                }
            } else {
                // Full display for 'all' command
                print_result(
                    "GET /collections/{name}/documents (List)",
                    &documents,
                    true,
                );
            }
            
            // Continue with rest of document operations only for 'all' command
            if command == "all" {
                // POST /collections/{name}/documents (Add)
                let new_doc = json!({
                "id": format!("test_doc_{}", std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()),
                "title": "Test Document",
                "content": "This is a test document for API testing",
                "embedding": [0.1, 0.2, 0.3, 0.4, 0.5]
            });
            let add_result = client.documents().add(&collection_name_str, new_doc.clone()).await?;
            print_result("POST /collections/{name}/documents (Add)", &add_result, true);
            
            if add_result.get_status_code() == 200 || add_result.get_status_code() == 201 {
                let added_doc_id = new_doc.get("id").and_then(|id| id.as_str()).unwrap_or("unknown");
                
                // PUT /collections/{name}/documents/{id}
                let updated_doc = json!({
                    "title": "Updated Test Document",
                    "content": "This document has been updated"
                });
                let update_result = client.documents().update(&collection_name_str, added_doc_id, updated_doc).await?;
                print_result("PUT /collections/{name}/documents/{id} (Update)", &update_result, true);
                
                // POST /collections/{name}/documents/import
                let bulk_docs = vec![
                    json!({ "id": "bulk_1", "title": "Bulk Doc 1", "content": "Content 1" }),
                    json!({ "id": "bulk_2", "title": "Bulk Doc 2", "content": "Content 2" }),
                    json!({ "id": "bulk_3", "title": "Bulk Doc 3", "content": "Content 3" })
                ];
                let import_result = client.documents().import(&collection_name_str, bulk_docs).await?;
                print_result("POST /collections/{name}/documents/import (Bulk Import)", &import_result, true);
                
                // DELETE /collections/{name}/documents/{id}
                let delete_result = client.documents().delete(&collection_name_str, added_doc_id).await?;
                print_result("DELETE /collections/{name}/documents/{id}", &delete_result, true);
                
                // DELETE /collections/{name}/documents (by filter)
                let delete_by_filter_result = client.documents().delete_by_filter(&collection_name_str, "title:Bulk*").await?;
                print_result("DELETE /collections/{name}/documents (by filter)", &delete_by_filter_result, true);
            }
        }
    }
    
    // ----------------------------------------------------------------====================================
    // SEARCH API (only for 'all' command)
    // ----------------------------------------------------------------====================================
    if command == "all" {
        println!("\n{}", "#".repeat(70));
        println!("# SEARCH API");
        println!("{}\n", "#".repeat(70));
        
        let search_collection = get_first_collection(&client).await;
        if search_collection.is_none() {
            println!("No collections available - skipping search tests\n");
        } else {
            let collection_name = search_collection.unwrap();
        
        // GET/POST /collections/{name}/documents/search (Regular search)
        let mut search_params = HashMap::new();
        search_params.insert("q".to_string(), "test".to_string());
        search_params.insert("query_by".to_string(), "title,content".to_string());
        search_params.insert("limit".to_string(), "5".to_string());
        print_result(
            "GET /collections/{name}/documents/search (Regular Search)",
            &client.search(&collection_name, search_params).await?,
            true,
        );
        
        // GET /collections/{name}/vector_search (Vector search query params)
        let mut vector_params = HashMap::new();
        vector_params.insert("vector_query".to_string(), "[0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0]".to_string());
        vector_params.insert("limit".to_string(), "5".to_string());
        vector_params.insert("threshold".to_string(), "0.0".to_string());
        vector_params.insert("normalize".to_string(), "true".to_string());
        print_result(
            "GET /collections/{name}/vector_search (Vector Search)",
            &client.vector_search(&collection_name, vector_params).await?,
            true,
        );
        
        // POST /multi_search
        let multi_search_params = vec![
            json!({
                "collection": collection_name,
                "q": "test",
                "query_by": "title"
            }),
            json!({
                "collection": collection_name,
                "q": "document",
                "query_by": "content"
            })
        ];
        print_result(
            "POST /multi_search",
            &client.search_api().multi_search(multi_search_params).await?,
            true,
        );
        }
    }
    
    // ----------------------------------------------------------------====================================
    // SUMMARY (only show for 'all' command)
    // ----------------------------------------------------------------====================================
    if command == "all" {
        println!("\n{}", "#".repeat(70));
        println!("# TESTING COMPLETE");
        println!("{}\n", "#".repeat(70));
        
        println!("Routes have been tested. Check the output above for results.");
        println!("Note: Some tests may fail if:");
        println!("  - Authentication is required but no token was provided");
        println!("  - Collections don't exist");
        println!("  - Required data is missing");
        println!();
        println!("Usage: cargo run --example example [command] [args...] [token]");
        println!("  Commands:");
        println!("    cols   - List collections (with pagination)");
        println!("             Usage: cols [offset] [limit] [token]");
        println!("             Example: cols 0 200");
        println!("    docs   - Run documents API examples");
        println!("             Usage: docs [collection_name] [token]");
        println!("             Example: docs my_collection");
        println!("    open   - List and open collections");
        println!("    status - Show server health and status information");
        println!("    help   - Show help message");
        println!("    all    - Run all examples (default)");
        println!();
    }
    
    Ok(())
}
