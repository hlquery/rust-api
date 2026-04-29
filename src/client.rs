/**
 * hlquery Rust Client - Main Client Class
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::collections::Collections;
use crate::documents::Documents;
use crate::error::{HlqueryError, Result};
use crate::request::Request;
use crate::response::Response;
use crate::search::Search;
use crate::sql::Sql;
use crate::utils::config::Config;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Main client class for hlquery
pub struct Client {
    request: Arc<Request>,
    collections: Arc<Collections>,
    documents: Arc<Documents>,
    search: Arc<Search>,
    sql: Arc<Sql>,
}

impl Client {
    /// Create a new client
    pub fn new(base_url: &str, options: Option<HashMap<String, String>>) -> Result<Self> {
        let mut config = Config::merge_defaults(options);
        config.base_url = if base_url.trim().is_empty() {
            config.base_url
        } else {
            base_url.to_string()
        };

        let url = Config::normalize_url(&config.base_url);

        if !Config::is_valid_url(&url) {
            return Err(HlqueryError::InvalidUrl(url));
        }

        let request = Arc::new(Request::new(
            url,
            config.timeout,
            config.token,
            config.auth_method,
        )?);

        let collections = Arc::new(Collections::new(Arc::clone(&request)));
        let documents = Arc::new(Documents::new(Arc::clone(&request)));
        let search = Arc::new(Search::new(Arc::clone(&request)));
        let sql = Arc::new(Sql::new(Arc::clone(&request), Arc::clone(&search)));

        Ok(Client {
            request,
            collections,
            documents,
            search,
            sql,
        })
    }

    /// Set authentication token
    pub fn set_auth_token(&self, token: String, method: String) {
        self.request.set_auth_token(token, method);
    }

    /// Clear authentication
    pub fn clear_auth(&self) {
        self.request.clear_auth();
    }

    // System APIs

    /// Health check
    pub async fn health(&self) -> Result<Response> {
        self.request.execute("GET", "/health", None, None).await
    }

    /// Get server statistics
    pub async fn stats(&self) -> Result<Response> {
        self.request.execute("GET", "/stats", None, None).await
    }

    /// Get protocol codes for API communication
    /// Returns HTTP status codes and protocol information
    pub async fn etc(&self) -> Result<Response> {
        self.request.execute("GET", "/etc", None, None).await
    }

    /// Get server information
    pub async fn info(&self) -> Result<Response> {
        self.request.execute("GET", "/", None, None).await
    }

    /// Flush all data to disk
    pub async fn flush(&self) -> Result<Response> {
        self.request.execute("POST", "/flush", None, None).await
    }

    /// List configured cluster links
    pub async fn links(&self) -> Result<Response> {
        self.request.execute("GET", "/links", None, None).await
    }

    /// Ping configured cluster links
    pub async fn links_ping(&self) -> Result<Response> {
        self.request.execute("GET", "/links/ping", None, None).await
    }

    /// Execute a top-level SQL query through GET /sql
    pub async fn sql(
        &self,
        sql: &str,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.sql.query(sql, query_params).await
    }

    /// Execute a top-level SQL statement through POST /sql
    pub async fn exec_sql(&self, sql: &str) -> Result<Response> {
        self.sql.exec(sql).await
    }

    /// Add a cluster link (in-memory only)
    pub async fn links_connect(
        &self,
        endpoint_or_host: &str,
        port: Option<u16>,
    ) -> Result<Response> {
        let body = if let Some(port_val) = port {
            serde_json::json!({"host": endpoint_or_host, "port": port_val})
        } else {
            serde_json::json!({"endpoint": endpoint_or_host})
        };
        self.request
            .execute("POST", "/links/connect", Some(body), None)
            .await
    }

    /// Remove a cluster link (in-memory only)
    pub async fn links_disconnect(
        &self,
        endpoint_or_host: &str,
        port: Option<u16>,
    ) -> Result<Response> {
        let body = if let Some(port_val) = port {
            serde_json::json!({"host": endpoint_or_host, "port": port_val})
        } else {
            serde_json::json!({"endpoint": endpoint_or_host})
        };
        self.request
            .execute("POST", "/links/disconnect", Some(body), None)
            .await
    }

    // Collections API

    /// Get collections API handler
    pub fn collections(&self) -> Arc<Collections> {
        Arc::clone(&self.collections)
    }

    /// List collections
    pub async fn list_collections(&self, offset: usize, limit: usize) -> Result<Response> {
        self.collections.list(offset, limit).await
    }

    /// List collections across all configured nodes
    pub async fn list_collections_distributed(&self) -> Result<Response> {
        self.request
            .execute("GET", "/collections/distributed", None, None)
            .await
    }

    /// Get collection details
    pub async fn get_collection(&self, name: &str) -> Result<Response> {
        self.collections.get(name).await
    }

    /// Get collection fields
    pub async fn get_collection_fields(&self, name: &str) -> Result<Response> {
        self.collections.get_fields(name).await
    }

    // Documents API

    /// Get documents API handler
    pub fn documents(&self) -> Arc<Documents> {
        Arc::clone(&self.documents)
    }

    /// List documents
    pub async fn list_documents(
        &self,
        collection_name: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.documents.list(collection_name, params).await
    }

    /// Get document by ID
    pub async fn get_document(&self, collection_name: &str, document_id: &str) -> Result<Response> {
        self.documents.get(collection_name, document_id).await
    }

    // Search API

    /// Get search API handler
    pub fn search_api(&self) -> Arc<Search> {
        Arc::clone(&self.search)
    }

    /// Get SQL API handler
    pub fn sql_api(&self) -> Arc<Sql> {
        Arc::clone(&self.sql)
    }

    /// Perform search
    pub async fn search(
        &self,
        collection_name: &str,
        params: HashMap<String, String>,
    ) -> Result<Response> {
        self.search.search(collection_name, params).await
    }

    /// Perform vector search
    pub async fn vector_search(
        &self,
        collection_name: &str,
        params: HashMap<String, String>,
    ) -> Result<Response> {
        self.search.vector_search(collection_name, params).await
    }

    /// Execute a collection-bound SQL SELECT through the search endpoint
    pub async fn sql_search(
        &self,
        collection_name: &str,
        sql: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.search.sql(collection_name, sql, params).await
    }

    /// Execute arbitrary request
    pub async fn execute_request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request.execute(method, path, body, query_params).await
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use std::collections::HashMap;

    #[test]
    fn uses_base_url_argument_over_default_localhost() {
        let client = Client::new("http://127.0.0.1:9400/", None).expect("client should be created");
        assert_eq!(client.request.base_url(), "http://127.0.0.1:9400");
    }

    #[test]
    fn base_url_argument_overrides_options_base_url() {
        let mut options = HashMap::new();
        options.insert("base_url".to_string(), "http://localhost:9200".to_string());

        let client = Client::new("https://api.example.com:9443", Some(options))
            .expect("client should be created");
        assert_eq!(client.request.base_url(), "https://api.example.com:9443");
    }

    #[test]
    fn falls_back_to_options_base_url_when_argument_is_blank() {
        let mut options = HashMap::new();
        options.insert("base_url".to_string(), "http://127.0.0.1:9500".to_string());

        let client = Client::new("   ", Some(options)).expect("client should be created");
        assert_eq!(client.request.base_url(), "http://127.0.0.1:9500");
    }
}
