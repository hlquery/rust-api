/**
 * hlquery Rust Client - Authentication Utilities
 * 
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 * 
 * This file is part of hlquery, released under the BSD License version 3.
 */

use md5;
use crate::error::AuthenticationException;

/// Authentication utilities
pub struct Auth;

impl Auth {
    /// Generate MD5 hash for a token (utility function for token generation)
    /// Note: Authentication only requires a token - no username/password needed
    pub fn generate_token(token: &str) -> String {
        let digest = md5::compute(token.as_bytes());
        format!("{:x}", digest)
    }
    
    /// Validate token format
    pub fn validate_token(token: &str) -> Result<(), AuthenticationException> {
        if token.trim().is_empty() {
            return Err(AuthenticationException(
                "Token must be a non-empty string".to_string(),
            ));
        }
        Ok(())
    }
}
