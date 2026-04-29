/**
 * Search Examples
 *
 * Demonstrates various search patterns with the hlquery Rust client
 */
use hlquery_rust_client::Client;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;

    // Simple search
    let mut search_params = HashMap::new();
    search_params.insert("q".to_string(), "search query".to_string());
    search_params.insert("query_by".to_string(), "title,content".to_string());
    search_params.insert("limit".to_string(), "10".to_string());
    let results = client.search("collection_name", search_params).await?;
    println!("Search results: {}", results.get_body());

    // Search with filters
    let mut filtered_params = HashMap::new();
    filtered_params.insert("q".to_string(), "query".to_string());
    filtered_params.insert("query_by".to_string(), "title".to_string());
    filtered_params.insert("filter_by".to_string(), "category:electronics".to_string());
    filtered_params.insert("sort_by".to_string(), "price:asc".to_string());
    filtered_params.insert("limit".to_string(), "20".to_string());
    let filtered_results = client.search("collection_name", filtered_params).await?;
    println!("Filtered results: {}", filtered_results.get_body());

    // Supported query semantics
    // Field-specific search
    let mut field_params = HashMap::new();
    field_params.insert("q".to_string(), "title:laptop".to_string());
    field_params.insert("query_by".to_string(), "title,content".to_string());
    field_params.insert("limit".to_string(), "10".to_string());
    let field_results = client.search("collection_name", field_params).await?;
    println!("Field search: {}", field_results.get_body());

    // Boolean OR query
    let mut or_params = HashMap::new();
    or_params.insert(
        "q".to_string(),
        "title:laptop OR title:notebook".to_string(),
    );
    or_params.insert("query_by".to_string(), "title,content".to_string());
    or_params.insert("limit".to_string(), "10".to_string());
    let or_results = client.search("collection_name", or_params).await?;
    println!("Boolean OR search: {}", or_results.get_body());

    // Boolean NOT query
    let mut not_params = HashMap::new();
    not_params.insert(
        "q".to_string(),
        "title:laptop NOT title:refurbished".to_string(),
    );
    not_params.insert("query_by".to_string(), "title,content".to_string());
    not_params.insert("limit".to_string(), "10".to_string());
    let not_results = client.search("collection_name", not_params).await?;
    println!("Boolean NOT search: {}", not_results.get_body());

    // Phrase search
    let mut phrase_params = HashMap::new();
    phrase_params.insert("q".to_string(), "\"wireless keyboard\"".to_string());
    phrase_params.insert("query_by".to_string(), "title".to_string());
    phrase_params.insert("limit".to_string(), "10".to_string());
    let phrase_results = client.search("collection_name", phrase_params).await?;
    println!("Phrase search: {}", phrase_results.get_body());

    // Wildcard search
    let mut wildcard_params = HashMap::new();
    wildcard_params.insert("q".to_string(), "laptop*".to_string());
    wildcard_params.insert("query_by".to_string(), "title,content".to_string());
    wildcard_params.insert("limit".to_string(), "10".to_string());
    let wildcard_results = client.search("collection_name", wildcard_params).await?;
    println!("Wildcard search: {}", wildcard_results.get_body());

    // Filter operators belong in filter_by
    let mut combined_params = HashMap::new();
    combined_params.insert("q".to_string(), "*".to_string());
    combined_params.insert("query_by".to_string(), "title,content".to_string());
    combined_params.insert(
        "filter_by".to_string(),
        "price:>100&&category:electronics".to_string(),
    );
    combined_params.insert("limit".to_string(), "10".to_string());
    let combined_results = client.search("collection_name", combined_params).await?;
    println!("Filtered search: {}", combined_results.get_body());

    // Vector search
    let mut vector_params = HashMap::new();
    vector_params.insert(
        "vector_query".to_string(),
        "[0.1,0.2,0.3,0.4,0.5]".to_string(),
    );
    vector_params.insert("limit".to_string(), "10".to_string());
    vector_params.insert("threshold".to_string(), "0.5".to_string());
    let vector_results = client
        .vector_search("collection_name", vector_params)
        .await?;
    println!("Vector results: {}", vector_results.get_body());

    // Multi-search
    let searches = vec![
        json!({
            "collection": "col1",
            "q": "query1",
            "query_by": "title"
        }),
        json!({
            "collection": "col2",
            "q": "query2",
            "query_by": "content"
        }),
    ];
    let multi_results = client.search_api().multi_search(searches).await?;
    println!("Multi-search results: {}", multi_results.get_body());

    Ok(())
}
