<div align="center">
  <img src="https://docs.hlquery.com/img/hlquery/2.png" alt="hlquery logo" width="200">
</div>

<div align="center">

**A modern async Rust client library for hlquery, designed with a familiar and intuitive API structure.**

[![Follow hlquery](https://img.shields.io/badge/Follow-%40hlquery-blue?logo=x&logoColor=white&labelColor=000000)](https://x.com/hlquery)
[![Rust build](https://img.shields.io/badge/Rust%20build-passing-brightgreen?logo=rust&logoColor=white&labelColor=000000)](https://github.com/hlquery/rust-api/actions/workflows/rust-api.yml)
[![GitHub](https://img.shields.io/badge/GitHub-rust--api-purple?logo=github&logoColor=white&labelColor=000000)](https://github.com/hlquery/rust-api/)
[![hlquery](https://img.shields.io/badge/GitHub-hlquery-blue?logo=github&logoColor=white&labelColor=000000)](https://github.com/hlquery/hlquery/)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-a35a0f?logo=open-source-initiative&logoColor=white&labelColor=000000)](https://opensource.org/licenses/BSD-3-Clause)

</div>

### What is the hlquery Rust API?

The hlquery Rust API is the official Rust client for [hlquery](https://github.com/hlquery/hlquery). It wraps the server's HTTP interface in an async client with helpers for collections, documents, search, and SQL.

It is intended for async services, tools, and applications that want strong typing and a small high-level integration layer over hlquery.

### Why use it?

- Async-first design built for Tokio.
- Strong typing and structured error handling.
- Modular API objects for collections, search, and SQL.
- Raw request helper for custom routes.

### Why choose it over raw HTTP?

Choose the Rust client over raw HTTP when you want less repetitive request building and JSON parsing, cleaner auth and timeout handling, and application-level search code that stays compact.

### Install

Add to `Cargo.toml`:

```toml
[dependencies]
hlquery-rust-client = { path = "./rust" }
tokio = { version = "1", features = ["full"] }
```

If published:

```toml
[dependencies]
hlquery-rust-client = "1.0"
tokio = { version = "1", features = ["full"] }
```

### Quick Start

```rust
use hlquery_rust_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("HLQ_BASE_URL")
        .or_else(|_| std::env::var("HLQUERY_BASE_URL"))
        .unwrap_or_else(|_| "http://localhost:9200".to_string());

    let client = Client::new(&base_url, None)?;

    let health = client.health().await?;
    println!("Status: {}", health.get_status_code());

    let collections = client.list_collections(0, 10).await?;
    println!("{}", collections.get_body());

    Ok(())
}
```

### Auth

```rust
use hlquery_rust_client::Client;
use std::collections::HashMap;

let mut options = HashMap::new();
options.insert("token".to_string(), "your_token_here".to_string());
options.insert("auth_method".to_string(), "bearer".to_string());

let client = Client::new("http://localhost:9200", Some(options))?;
client.set_auth_token("your_api_key_here".to_string(), "api-key".to_string());
```

### SQL

```rust
let client = Client::new("http://localhost:9200", None)?;

let rows = client.sql("SHOW COLLECTIONS;", None).await?;
let exec_result = client
    .exec_sql("INSERT INTO logs_archive (id, title) VALUES ('row-1', 'warm cache');")
    .await?;
let products = client
    .sql_search(
        "products",
        "SELECT id, title, price FROM products WHERE price > 100 ORDER BY price DESC LIMIT 3;",
        None,
    )
    .await?;

let sql_api = client.sql_api();
let same_rows = sql_api.query("SHOW COLLECTIONS;", None).await?;
```

### Contributing

We welcome contributions from the community! All contributions must be released under the BSD 3-Clause license.

### How to Contribute

- Check existing [Rust API issues](https://github.com/hlquery/rust-api/issues) or create new ones
- Contribute Rust client changes to [hlquery/rust-api](https://github.com/hlquery/rust-api)
- Contribute shared server/API changes to [hlquery/hlquery](https://github.com/hlquery/hlquery)
- Test and report bugs against the Rust client
- Improve Rust-specific documentation and examples

### Search all collections

```rust
let result = client.search_api().search_all("GET", None, Some(params)).await?;
```

Use `POST` with a JSON body to pass a collection array. `global_search` remains available as an equivalent name. Results are globally merged and each hit includes `document._collection`.

### Community

- 📖 [Documentation](https://docs.hlquery.com)
- 🐦 [X (Twitter)](https://x.com/hlquery)
- 🛡️ [Rust API GitHub](https://github.com/hlquery/rust-api)
- 📦 [hlquery GitHub](https://github.com/hlquery/hlquery)

### License

The hlquery Rust API is licensed under the [BSD 3-Clause License](https://opensource.org/licenses/BSD-3-Clause).
