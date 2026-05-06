/**
 * hlquery Rust Client - SAM API
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::error::Result;
use crate::request::Request;
use crate::response::Response;
use crate::utils::validator::Validator;
use std::collections::HashMap;
use std::sync::Arc;

/// SAM API handler
pub struct Sam
{
    request: Arc<Request>,
}

impl Sam
{
    /// Create a new SAM handler
    pub fn new(request: Arc<Request>) -> Self
    {
        Sam { request }
    }

    /// Execute a SAM search
    pub async fn search(
        &self,
        collection_name: &str,
        query: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response>
    {
        Validator::validate_collection_name(collection_name)?;

        let mut query_params = params.unwrap_or_default();
        query_params.insert("collection".to_string(), collection_name.to_string());
        query_params.insert("q".to_string(), query.to_string());

        self.request
            .execute("GET", "/sam/search", None, Some(query_params))
            .await
    }

    /// Get SAM background status
    pub async fn status(
        &self,
        collection_name: Option<&str>,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response>
    {
        let mut query_params = params.unwrap_or_default();

        if let Some(name) = collection_name
        {
            Validator::validate_collection_name(name)?;
            query_params.insert("collection".to_string(), name.to_string());
        }

        let maybe_query = if query_params.is_empty()
        {
            None
        }
        else
        {
            Some(query_params)
        };

        self.request
            .execute("GET", "/sam/status", None, maybe_query)
            .await
    }

    /// Get SAM search history
    pub async fn history(
        &self,
        collection_name: Option<&str>,
        limit: usize,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response>
    {
        let mut query_params = params.unwrap_or_default();
        query_params.insert("limit".to_string(), limit.to_string());

        if let Some(name) = collection_name
        {
            Validator::validate_collection_name(name)?;
            query_params.insert("collection".to_string(), name.to_string());
        }

        self.request
            .execute("GET", "/sam/history", None, Some(query_params))
            .await
    }
}
