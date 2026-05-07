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

    // Top-level SQL through the client convenience method
    let rows = client.sql("SHOW COLLECTIONS;", None).await?;
    println!("SHOW COLLECTIONS: {}", rows.get_body());

    // Top-level SQL statement execution through POST /sql
    let exec_result = client
        .exec_sql("INSERT INTO logs_archive (id, title) VALUES ('row-1', 'warm cache');")
        .await?;
    println!("Exec result: {}", exec_result.get_body());

    // Collection-bound SQL SELECT through /collections/{name}/documents/search
    let mut search_params = HashMap::new();
    search_params.insert("highlight".to_string(), "false".to_string());

    let products = client
        .sql_search(
            "products",
            "SELECT id, title, price FROM products WHERE price > 100 ORDER BY price DESC LIMIT 3;",
            Some(search_params),
        )
        .await?;
    println!("Products SQL results: {}", products.get_body());

    // The dedicated SQL handler remains available for the same endpoints.
    let sql_api = client.sql_api();
    let same_rows = sql_api.query("SHOW COLLECTIONS;", None).await?;
    println!("SHOW COLLECTIONS via sql_api(): {}", same_rows.get_body());

    let same_exec_result = sql_api
        .exec("INSERT INTO logs_archive (id, title) VALUES ('row-1', 'warm cache');")
        .await?;
    println!("Exec result via sql_api(): {}", same_exec_result.get_body());

    Ok(())
}
