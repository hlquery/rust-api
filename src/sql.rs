/**
 * hlquery Rust Client - SQL API
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::error::Result;
use crate::request::Request;
use crate::response::Response;
use crate::search::Search;
use crate::utils::validator::Validator;
use std::collections::HashMap;
use std::sync::Arc;

/// SQL API handler
pub struct Sql {
    request: Arc<Request>,
    search: Arc<Search>,
}

impl Sql {
    /// Create a new SQL handler
    pub fn new(request: Arc<Request>, search: Arc<Search>) -> Self {
        Sql { request, search }
    }

    /// Execute a top-level SQL query through GET /sql
    pub async fn query(
        &self,
        sql: &str,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        Validator::validate_sql(sql)?;

        let mut params = query_params.unwrap_or_default();
        params.insert("sql".to_string(), sql.to_string());

        self.request
            .execute("GET", "/sql", None, Some(params))
            .await
    }

    /// Execute a top-level SQL statement through POST /sql
    pub async fn exec(&self, sql: &str) -> Result<Response> {
        Validator::validate_sql(sql)?;

        let body = serde_json::json!({ "exec": sql });
        self.request.execute("POST", "/sql", Some(body), None).await
    }

    /// Execute a collection-bound SQL SELECT through the search endpoint
    pub async fn search(
        &self,
        collection_name: &str,
        sql: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.search.sql(collection_name, sql, params).await
    }
}
