/**
 * hlquery Rust Client - Collections API
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::error::Result;
use crate::request::Request;
use crate::response::Response;
use crate::utils::validator::Validator;
use serde_json::Value;
use std::sync::Arc;

/// Collections API handler
pub struct Collections {
    request: Arc<Request>,
}

impl Collections {
    /// Create a new Collections handler
    pub fn new(request: Arc<Request>) -> Self {
        Collections { request }
    }

    /// List collections
    pub async fn list(&self, offset: usize, limit: usize) -> Result<Response> {
        Validator::validate_pagination(Some(offset), Some(limit))?;

        let mut query_params = std::collections::HashMap::new();
        query_params.insert("offset".to_string(), offset.to_string());
        query_params.insert("limit".to_string(), limit.to_string());

        self.request
            .execute("GET", "/collections", None, Some(query_params))
            .await
    }

    /// Get collection details
    pub async fn get(&self, name: &str) -> Result<Response> {
        Validator::validate_collection_name(name)?;

        let path = format!("/collections/{}", urlencoding::encode(name));
        self.request.execute("GET", &path, None, None).await
    }

    /// Get collection fields (formatted)
    pub async fn get_fields(&self, name: &str) -> Result<Response> {
        self.get(name).await
    }

    /// Get detected collection language information
    pub async fn language(&self, name: &str) -> Result<Response> {
        Validator::validate_collection_name(name)?;
        let path = format!("/collections/{}/lang", urlencoding::encode(name));
        self.request.execute("GET", &path, None, None).await
    }

    /// Create collection
    pub async fn create(&self, name: &str, schema: Value) -> Result<Response> {
        Validator::validate_collection_name(name)?;

        let mut body = match schema {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        };
        body.insert("name".to_string(), Value::String(name.to_string()));

        self.request
            .execute("POST", "/collections", Some(Value::Object(body)), None)
            .await
    }

    /// Update collection schema
    pub async fn update(&self, name: &str, schema: Value) -> Result<Response> {
        Validator::validate_collection_name(name)?;

        let path = format!("/collections/{}/update", urlencoding::encode(name));
        self.request
            .execute("POST", &path, Some(schema), None)
            .await
    }

    /// Delete collection
    pub async fn delete(&self, name: &str) -> Result<Response> {
        Validator::validate_collection_name(name)?;

        let path = format!("/collections/{}", urlencoding::encode(name));
        self.request.execute("DELETE", &path, None, None).await
    }
}
