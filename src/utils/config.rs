/**
 * hlquery Rust Client - Configuration Utilities
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use std::collections::HashMap;
use std::env;
use url::Url;

/// Configuration options for the client
#[derive(Debug, Clone)]
pub struct ConfigOptions {
    pub base_url: String,
    pub timeout: u64,
    pub token: Option<String>,
    pub auth_method: String,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        ConfigOptions {
            base_url: env::var("HLQ_BASE_URL")
                .or_else(|_| env::var("HLQUERY_BASE_URL"))
                .unwrap_or_else(|_| "http://localhost:9200".to_string()),
            timeout: 30000,
            token: None,
            auth_method: "bearer".to_string(),
        }
    }
}

/// Configuration management utilities
pub struct Config;

impl Config {
    /// Merge options with defaults
    pub fn merge_defaults(options: Option<HashMap<String, String>>) -> ConfigOptions {
        let mut config = ConfigOptions::default();

        if let Some(opts) = options {
            if let Some(url) = opts.get("base_url") {
                config.base_url = url.clone();
            }
            if let Some(timeout) = opts.get("timeout") {
                if let Ok(t) = timeout.parse::<u64>() {
                    config.timeout = t;
                }
            }
            if let Some(token) = opts.get("token") {
                config.token = Some(token.clone());
            }
            if let Some(method) = opts.get("auth_method") {
                config.auth_method = method.clone();
            }
        }

        config
    }

    /// Normalize URL
    pub fn normalize_url(url: &str) -> String {
        let url = url.trim();
        let url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("http://{}", url)
        } else {
            url.to_string()
        };

        url.trim_end_matches('/').to_string()
    }

    /// Validate URL
    pub fn is_valid_url(url: &str) -> bool {
        Url::parse(url).is_ok()
    }
}
