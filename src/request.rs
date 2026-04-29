/**
 * hlquery Rust Client - HTTP Request Handler
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::error::{HlqueryError, Result};
use crate::response::Response;
use reqwest::Client as ReqwestClient;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use url::Url;

/// HTTP request handler
pub struct Request {
    base_url: String,
    client: ReqwestClient,
    auth: Arc<Mutex<AuthState>>,
}

struct AuthState {
    token: Option<String>,
    auth_method: String,
}

impl Request {
    /// Create a new request handler
    pub fn new(
        base_url: String,
        timeout: u64,
        token: Option<String>,
        auth_method: String,
    ) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_millis(timeout))
            .build()
            .map_err(|e| HlqueryError::Unknown(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Request {
            base_url,
            client,
            auth: Arc::new(Mutex::new(AuthState { token, auth_method })),
        })
    }

    /// Set authentication token
    pub fn set_auth_token(&self, token: String, method: String) {
        let mut auth = self.auth.lock().unwrap();
        auth.token = Some(token);
        auth.auth_method = method;
    }

    /// Clear authentication
    pub fn clear_auth(&self) {
        let mut auth = self.auth.lock().unwrap();
        auth.token = None;
    }

    /// Execute HTTP request
    pub async fn execute(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let url = self.build_url(path)?;

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            _ => {
                return Err(HlqueryError::Unknown(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        // Add authentication
        {
            let auth = self.auth.lock().unwrap();
            if let Some(ref token) = auth.token {
                match auth.auth_method.as_str() {
                    "bearer" => {
                        request = request.bearer_auth(token);
                    }
                    "api-key" => {
                        request = request.header("X-API-Key", token);
                    }
                    _ => {
                        request = request.bearer_auth(token);
                    }
                }
            }
        }

        // Add query parameters
        if let Some(params) = query_params {
            request = request.query(&params);
        }

        // Add body for POST/PUT/PATCH
        if let Some(body_value) = body {
            request = request.json(&body_value);
        }

        // Execute request
        let response = request
            .send()
            .await
            .map_err(|e| HlqueryError::RequestError(e))?;

        let status_code = response.status().as_u16();

        // Parse headers first
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        // Parse body
        let body_text = response
            .text()
            .await
            .map_err(|e| HlqueryError::RequestError(e))?;

        // Check for errors
        if status_code == 403 {
            if let Ok(body_json) = serde_json::from_str::<Value>(&body_text) {
                let error_msg = body_json
                    .get("error")
                    .and_then(|e| e.as_str())
                    .or_else(|| body_json.get("message").and_then(|m| m.as_str()))
                    .unwrap_or("");

                if error_msg.contains("Demo mode is enabled")
                    || error_msg.contains("Operation not allowed")
                    || error_msg.contains("Write operations are not allowed")
                {
                    let message = body_json
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or(error_msg);
                    return Err(HlqueryError::DemoModeError(format!(
                        "Demo mode is enabled on the server. Write operations are blocked. Only search and read operations are permitted. Server message: {}", message
                    )));
                }
            }
        }

        let body: Value = if body_text.is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&body_text).unwrap_or_else(|_| {
                // If JSON parsing fails, return as string
                serde_json::json!({ "raw": body_text })
            })
        };

        Ok(Response::new(status_code, body, headers))
    }

    /// Build full URL from path
    fn build_url(&self, path: &str) -> Result<String> {
        let base = Url::parse(&self.base_url)
            .map_err(|_| HlqueryError::InvalidUrl(self.base_url.clone()))?;

        let url = base
            .join(path)
            .map_err(|_| HlqueryError::InvalidUrl(format!("{}{}", self.base_url, path)))?;

        Ok(url.to_string())
    }

    #[cfg(test)]
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }
}
