/**
 * hlquery Rust Client - Search API
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
use std::collections::HashMap;
use std::sync::Arc;

/// Search API handler
pub struct Search {
    request: Arc<Request>,
}

impl Search {
    /// Create a new Search handler
    pub fn new(request: Arc<Request>) -> Self {
        Search { request }
    }
    
    /// Perform search
    pub async fn search(&self, collection_name: &str, params: HashMap<String, String>) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        // Convert params to JSON for validation
        let params_json: Value = serde_json::to_value(&params)
            .map_err(|e| crate::error::HlqueryError::JsonError(e))?;
        Validator::validate_search_params(&params_json)?;
        
        let path = format!("/collections/{}/documents/search", urlencoding::encode(collection_name));
        self.request.execute("GET", &path, None, Some(params)).await
    }
    
    /// Perform vector search
    pub async fn vector_search(&self, collection_name: &str, params: HashMap<String, String>) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        let path = format!("/collections/{}/vector_search", urlencoding::encode(collection_name));
        self.request.execute("GET", &path, None, Some(params)).await
    }
    
    /// Perform multi-search
    pub async fn multi_search(&self, searches: Vec<Value>) -> Result<Response> {
        let body = serde_json::json!({ "searches": searches });
        self.request.execute("POST", "/multi_search", Some(body), None).await
    }
}
