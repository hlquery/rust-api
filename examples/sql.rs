/**
 * SQL Examples
 *
 * Demonstrates top-level and collection-bound SQL usage with the hlquery Rust client
 */
use hlquery_rust_client::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200", None)?;
    let sql_api = client.sql_api();

    // Top-level SQL through /sql
    let rows = sql_api.query("SHOW COLLECTIONS;", None).await?;
    println!("SHOW COLLECTIONS: {}", rows.get_body());

    // Top-level SQL statement execution through POST /sql
    let exec_result = sql_api
        .exec("INSERT INTO logs_archive (id, title) VALUES ('row-1', 'warm cache');")
        .await?;
    println!("Exec result: {}", exec_result.get_body());

    // Collection-bound SQL SELECT through /collections/{name}/documents/search
    let mut search_params = HashMap::new();
    search_params.insert("highlight".to_string(), "false".to_string());

    let products = sql_api
        .search(
            "products",
            "SELECT id, title, price FROM products WHERE price > 100 ORDER BY price DESC LIMIT 3;",
            Some(search_params),
        )
        .await?;
    println!("Products SQL results: {}", products.get_body());

    Ok(())
}
