/**
 * hlquery Rust Client - Documents API
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

/// Documents API handler
pub struct Documents {
    request: Arc<Request>,
}

impl Documents {
    /// Create a new Documents handler
    pub fn new(request: Arc<Request>) -> Self {
        Documents { request }
    }
    
    /// List documents
    pub async fn list(&self, collection_name: &str, params: Option<HashMap<String, String>>) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        let path = format!("/collections/{}/documents", urlencoding::encode(collection_name));
        self.request.execute("GET", &path, None, params).await
    }
    
    /// Get document by ID
    pub async fn get(&self, collection_name: &str, document_id: &str) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        Validator::validate_document_id(document_id)?;
        
        let path = format!(
            "/collections/{}/documents/{}",
            urlencoding::encode(collection_name),
            urlencoding::encode(document_id)
        );
        self.request.execute("GET", &path, None, None).await
    }
    
    /// Add document
    pub async fn add(&self, collection_name: &str, document: Value) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        let path = format!("/collections/{}/documents", urlencoding::encode(collection_name));
        self.request.execute("POST", &path, Some(document), None).await
    }
    
    /// Update document
    pub async fn update(&self, collection_name: &str, document_id: &str, document: Value) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        Validator::validate_document_id(document_id)?;
        
        let path = format!(
            "/collections/{}/documents/{}",
            urlencoding::encode(collection_name),
            urlencoding::encode(document_id)
        );
        self.request.execute("PUT", &path, Some(document), None).await
    }
    
    /// Delete document
    pub async fn delete(&self, collection_name: &str, document_id: &str) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        Validator::validate_document_id(document_id)?;
        
        let path = format!(
            "/collections/{}/documents/{}",
            urlencoding::encode(collection_name),
            urlencoding::encode(document_id)
        );
        self.request.execute("DELETE", &path, None, None).await
    }
    
    /// Import documents (bulk)
    pub async fn import(&self, collection_name: &str, documents: Vec<Value>) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        let path = format!("/collections/{}/documents/import", urlencoding::encode(collection_name));
        let body = serde_json::json!({ "documents": documents });
        self.request.execute("POST", &path, Some(body), None).await
    }
    
    /// Delete documents by filter
    pub async fn delete_by_filter(&self, collection_name: &str, filter: &str) -> Result<Response> {
        Validator::validate_collection_name(collection_name)?;
        
        let path = format!("/collections/{}/documents", urlencoding::encode(collection_name));
        let mut query_params = HashMap::new();
        query_params.insert("filter_by".to_string(), filter.to_string());
        self.request.execute("DELETE", &path, None, Some(query_params)).await
    }
}
