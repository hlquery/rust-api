/**
 * hlquery Rust Client - Error Types
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use thiserror::Error;

/// Result type alias for hlquery operations
pub type Result<T> = std::result::Result<T, HlqueryError>;

/// Main error type for hlquery client
#[derive(Error, Debug)]
pub enum HlqueryError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP error: status {0}, message: {1}")]
    HttpError(u16, String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Demo mode error: {0}")]
    DemoModeError(String),
}

/// Request exception
#[derive(Error, Debug)]
#[error("Request exception: {0}")]
pub struct RequestException(pub String);

/// Authentication exception
#[derive(Error, Debug)]
#[error("Authentication exception: {0}")]
pub struct AuthenticationException(pub String);

/// Validation exception
#[derive(Error, Debug)]
#[error("Validation exception: {0}")]
pub struct ValidationException(pub String);

impl From<ValidationException> for HlqueryError {
    fn from(err: ValidationException) -> Self {
        HlqueryError::ValidationError(err.0)
    }
}

impl From<AuthenticationException> for HlqueryError {
    fn from(err: AuthenticationException) -> Self {
        HlqueryError::AuthenticationError(err.0)
    }
}

impl From<RequestException> for HlqueryError {
    fn from(err: RequestException) -> Self {
        HlqueryError::Unknown(err.0)
    }
}
