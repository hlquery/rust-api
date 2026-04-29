/**
 * hlquery Rust Client - Response Wrapper
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use serde_json::Value;
use std::collections::HashMap;

/// Response wrapper with helper methods
#[derive(Debug, Clone)]
pub struct Response {
    status_code: u16,
    body: Value,
    headers: HashMap<String, String>,
}

impl Response {
    /// Create a new response
    pub fn new(status_code: u16, body: Value, headers: HashMap<String, String>) -> Self {
        Response {
            status_code,
            body,
            headers,
        }
    }

    /// Get HTTP status code
    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    /// Get response body
    pub fn get_body(&self) -> &Value {
        &self.body
    }

    /// Get response body as mutable reference
    pub fn get_body_mut(&mut self) -> &mut Value {
        &mut self.body
    }

    /// Get response headers
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Check if response is successful (2xx status code)
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Check if response is an error (4xx or 5xx status code)
    pub fn is_error(&self) -> bool {
        self.status_code >= 400
    }

    /// Get error message from response body
    pub fn get_error(&self) -> Option<String> {
        if self.is_error() {
            if let Some(error) = self.body.get("error") {
                if let Some(msg) = error.as_str() {
                    return Some(msg.to_string());
                }
                if let Some(msg) = error.get("message") {
                    if let Some(msg_str) = msg.as_str() {
                        return Some(msg_str.to_string());
                    }
                }
            }
            if let Some(msg) = self.body.get("message") {
                if let Some(msg_str) = msg.as_str() {
                    return Some(msg_str.to_string());
                }
            }
        }
        None
    }
}
