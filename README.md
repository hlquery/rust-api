<div align="center">
  <img src="https://docs.hlquery.com/img/hlquery/2.png" alt="hlquery logo" width="200">
</div>

<div align="center">

**A modern, async Rust client library for hlquery.**

[![Follow hlquery](https://img.shields.io/badge/Follow-%40hlquery-blue?logo=x&logoColor=white)](https://x.com/hlquery)
[![Commit Activity](https://img.shields.io/github/commit-activity/m/hlquery/rust-api)](https://github.com/hlquery/rust-api/pulse)
[![rust-api](https://img.shields.io/badge/GitHub-rust--api-181717?logo=github&logoColor=white)](https://github.com/hlquery/rust-api/stargazers)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

</div>

# hlquery Rust API Client


## Features

-  **Async/Await Support**: Built on Tokio for modern async Rust
-  **Type-safe**: Strong typing throughout with serde for JSON handling
-  **Intuitive API**: Familiar and easy-to-use structure
-  **Authentication Support**: Bearer token and X-API-Key authentication
-  **Comprehensive Validation**: Input validation for all operations
-  **Error Handling**: Rich error types with thiserror
-  **No Runtime Dependencies**: Minimal dependencies, maximum performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hlquery-rust-client = { path = "../etc/api/rust" }
tokio = { version = "1", features = ["full"] }
```

Or if published to crates.io:

```toml
[dependencies]
hlquery-rust-client = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use hlquery_rust_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new("http://localhost:9200", None)?;
    
    // Health check
    let health = client.health().await?;
    println!("Status: {}", health.get_status_code());
    
    // List collections
    let collections = client.list_collections(0, 10).await?;
    if collections.is_success() {
        let body = collections.get_body();
        if let Some(collections_array) = body.get("collections").and_then(|c| c.as_array()) {
            println!("Found {} collections", collections_array.len());
        }
    }
    
    Ok(())
}
```

### With Authentication

```rust
use hlquery_rust_client::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Set token in constructor
    let mut options = HashMap::new();
    options.insert("token".to_string(), "your_token_here".to_string());
    options.insert("auth_method".to_string(), "bearer".to_string());
    let client = Client::new("http://localhost:9200", Some(options))?;
    
    // Method 2: Set token dynamically
    let client = Client::new("http://localhost:9200", None)?;
    client.set_auth_token("your_token_here".to_string(), "bearer".to_string());
    
    // Method 3: Use X-API-Key
    client.set_auth_token("your_key_here".to_string(), "api-key".to_string());
    
    Ok(())
}
```

### Reduce Text Example

You can use the raw request helper to call custom module routes directly:

```rust
use hlquery_rust_client::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;

    let mut query = HashMap::new();
    query.insert("q".to_string(), "example query".to_string());

    let module_response = client
        .execute_request("GET", "/modules/<name>/<route>", None, Some(query))
        .await?;

    println!("{}", module_response.get_body());
    Ok(())
}
```

## API Reference

### Client Initialization

```rust
let client = Client::new(base_url, options)?;
```

**Parameters**:
- `base_url` (string, required): Base URL of hlquery server (e.g., `"http://localhost:9200"`)
- `options` (Option<HashMap<String, String>>, optional): Client options
  - `"base_url"`: Legacy fallback URL (used only when `base_url` argument is blank)
  - `"token"`: Authentication token
  - `"auth_method"`: Authentication method (`"bearer"` or `"api-key"`)
  - `"timeout"`: Request timeout in milliseconds (default: 30000)

**URL precedence**:
- `base_url` argument is canonical and overrides `options["base_url"]`
- `options["base_url"]` is only used as a backward-compatible fallback when `base_url` is blank

### System APIs

```rust
// Health check
let health = client.health().await?;

// Server statistics
let stats = client.stats().await?;

// Server information
let info = client.info().await?;
```

### Collections API

```rust
// List collections
let collections = client.list_collections(0, 10).await?;

// Get collection details
let collection = client.get_collection("my_collection").await?;

// Get collection fields (formatted)
let fields = client.get_collection_fields("my_collection").await?;

// Using the collections API handler
let collections_api = client.collections();

// Create collection
let schema = serde_json::json!({
    "fields": [
        { "name": "title", "type": "string" },
        { "name": "content", "type": "string" }
    ]
});
let result = collections_api.create("new_collection", schema).await?;

// Update collection schema
let result = collections_api.update("collection_name", schema).await?;

// Delete collection
let result = collections_api.delete("collection_name").await?;
```

### Documents API

```rust
// List documents
use std::collections::HashMap;
let mut params = HashMap::new();
params.insert("offset".to_string(), "0".to_string());
params.insert("limit".to_string(), "10".to_string());
let docs = client.list_documents("collection", Some(params)).await?;

// Get document by ID
let doc = client.get_document("collection", "doc_id").await?;

// Using the documents API handler
let documents_api = client.documents();

// Add document
let new_doc = serde_json::json!({
    "id": "doc_1",
    "title": "New Document",
    "content": "Document content"
});
let result = documents_api.add("collection", new_doc).await?;

// Update document
let updated_doc = serde_json::json!({
    "title": "Updated Document"
});
let result = documents_api.update("collection", "doc_id", updated_doc).await?;

// Delete document
let result = documents_api.delete("collection", "doc_id").await?;

// Bulk import
let bulk_docs = vec![
    serde_json::json!({ "id": "doc1", "title": "Doc 1" }),
    serde_json::json!({ "id": "doc2", "title": "Doc 2" })
];
let result = documents_api.import("collection", bulk_docs).await?;

// Delete by filter
let result = documents_api.delete_by_filter("collection", "title:Bulk*").await?;
```

### Search API

```rust
use std::collections::HashMap;

// Simple search
let mut params = HashMap::new();
params.insert("q".to_string(), "search query".to_string());
params.insert("query_by".to_string(), "title,content".to_string());
params.insert("limit".to_string(), "10".to_string());
let results = client.search("collection", params).await?;

// Supported query syntax
// Field-specific: params.insert("q".to_string(), "title:laptop".to_string());
// Boolean OR: params.insert("q".to_string(), "title:laptop OR title:notebook".to_string());
// Boolean NOT: params.insert("q".to_string(), "title:laptop NOT title:refurbished".to_string());
// Phrase: params.insert("q".to_string(), "\"wireless keyboard\"".to_string());
// Wildcard: params.insert("q".to_string(), "laptop*".to_string());

// Search with filters
let mut params = HashMap::new();
params.insert("q".to_string(), "query".to_string());
params.insert("query_by".to_string(), "title".to_string());
params.insert("filter_by".to_string(), "category:electronics".to_string());
params.insert("sort_by".to_string(), "price:asc".to_string());
params.insert("limit".to_string(), "20".to_string());
let results = client.search("collection", params).await?;

// Vector search
let mut params = HashMap::new();
params.insert("vector_query".to_string(), "[0.1,0.2,0.3,0.4,0.5]".to_string());
params.insert("limit".to_string(), "10".to_string());
params.insert("threshold".to_string(), "0.5".to_string());
let results = client.vector_search("collection", params).await?;

// Advanced vector search (POST body with query params / distance controls)
let body = serde_json::json!({
    "vector": [0.1, 0.2, 0.3, 0.4, 0.5],
    "field_name": "embedding",
    "topk": 10,
    "include_distance": true,
    "query_params": { "ef": 64, "nprobe": 4, "is_linear": true },
    "radius": 1.0
});
let results = client.execute_request(
    "POST",
    "/collections/collection/vector_search",
    Some(body),
    None
).await?;

// Multi-search
let searches = vec![
    serde_json::json!({
        "collection": "col1",
        "q": "query1",
        "query_by": "title"
    }),
    serde_json::json!({
        "collection": "col2",
        "q": "query2",
        "query_by": "content"
    })
];
let results = client.search_api().multi_search(searches).await?;
```

### Ranking helpers

`hlquery::compute_rank_signal` and `hlquery::attach_rank_sort` are available from the root crate, so you can derive a `rank_signal` from popularity/hit inputs and sort by that field everywhere.

```rust
use hlquery::{compute_rank_signal, attach_rank_sort};
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("q".to_string(), "guide".to_string());
let signal = compute_rank_signal(popularity as f64, hit_log as f64, None);
params.insert("rank_signal".to_string(), signal.to_string());
attach_rank_sort(&mut params, "rank_signal", "desc");
let results = client.search("collection", params).await?;
```

### Search Parameters

The `search()` method accepts flexible parameters:

#### Query String (`q`)
The `q` parameter supports the current lexical query syntax:
- **FIELD**: `"q" => "title:laptop"`
- **NOT**: `"q" => "title:laptop NOT title:refurbished"` or `"q" => "NOT apple"`
- **Boolean**: `"q" => "title:laptop OR title:notebook"`
- **WILDCARD**: `"q" => "laptop*"`, `"q" => "*laptop"`, `"q" => "lap*top"`
- **Phrase**: `"q" => "\"exact phrase\""`

Use `filter_by` for field filters and numeric comparisons, for example:
- `"filter_by" => "price:>100&&category:electronics"`
- `"filter_by" => "category:food||category:nature"`

#### Other Parameters
- `query_by` - Fields to search in (comma-separated string)
- `filter_by` - Filter conditions (string)
- `sort_by` - Sort fields (string)
- `limit` - Number of results (string)
- `offset` - Starting offset (string)

## Response Handling

All API methods return a `Result<Response>`:

```rust
let response = client.health().await?;

// Check success
if response.is_success() {
    let body = response.get_body();
    // Process response...
}

// Check status code
let status_code = response.get_status_code();

// Get error message
if response.is_error() {
    if let Some(error) = response.get_error() {
        println!("Error: {}", error);
    }
}
```

## Error Handling

The client uses Rust's `Result` type for error handling:

```rust
use hlquery_rust_client::{Client, HlqueryError};

match client.search("collection", params).await {
    Ok(response) => {
        // Handle success
    }
    Err(HlqueryError::RequestError(e)) => {
        println!("Request failed: {}", e);
    }
    Err(HlqueryError::ValidationError(msg)) => {
        println!("Validation failed: {}", msg);
    }
    Err(HlqueryError::AuthenticationError(msg)) => {
        println!("Auth failed: {}", msg);
    }
    Err(e) => {
        println!("Error: {}", e);
    }
}
```

## Examples

### Run Examples

```bash
# Comprehensive example
cargo run --example example

# Basic usage
cargo run --example basic_usage

# Collections examples
cargo run --example collections

# Documents examples
cargo run --example documents

# Search examples
cargo run --example search
```

### With Authentication Token

```bash
cargo run --example example your_token_here
```

## Requirements

- Rust 1.70+ (Edition 2021)
- Tokio runtime (for async support)

## Dependencies

- `reqwest`: HTTP client
- `serde` / `serde_json`: JSON serialization
- `url`: URL parsing
- `thiserror`: Error handling
- `md5`: Token generation
- `urlencoding`: URL encoding
- `tokio`: Async runtime

## Architecture

The client follows a modular architecture:

- **Client**: Main entry point, coordinates all operations
- **Request**: Handles HTTP requests and authentication
- **Response**: Wraps HTTP responses with helper methods
- **Collections**: Collection management operations
- **Documents**: Document CRUD operations
- **Search**: Search operations
- **Utils**: Configuration, validation, and authentication utilities

## Support

- **API Documentation**: See main [hlquery docs](../../../docs/)
- **Examples**: Check the `examples/` directory
- **Issues**: Report issues in the main hlquery repository

## Contributing

Contributions are welcome! The client follows Rust best practices:

- Use `async/await` for all I/O operations
- Return `Result<T>` for error handling
- Use `serde` for JSON serialization
- Follow Rust naming conventions (snake_case)

---

**Happy coding! 🚀**
