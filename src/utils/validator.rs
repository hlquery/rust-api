/**
 * hlquery Rust Client - Input Validation
 *
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 *
 * This file is part of hlquery, released under the BSD License version 3.
 */
use crate::error::ValidationException;

/// Input validation utilities
pub struct Validator;

impl Validator {
    /// Validate collection name
    pub fn validate_collection_name(name: &str) -> Result<(), ValidationException> {
        if name.trim().is_empty() {
            return Err(ValidationException(
                "Collection name must be a non-empty string".to_string(),
            ));
        }

        // Check name length (matches server validation: 1-64 characters)
        if name.len() > 64 {
            return Err(ValidationException(
                "Collection name must be between 1 and 64 characters".to_string(),
            ));
        }

        // Collection names should be URL-safe
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(ValidationException(
                "Collection name contains invalid characters. Use only letters, numbers, underscores, and hyphens".to_string(),
            ));
        }

        // Check if name starts with letter or underscore (matches server validation)
        let first_char = name.chars().next().unwrap_or(' ');
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(ValidationException(
                "Collection name must start with a letter or underscore".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate document ID
    pub fn validate_document_id(id: &str) -> Result<(), ValidationException> {
        if id.trim().is_empty() {
            return Err(ValidationException(
                "Document ID must be a non-empty string".to_string(),
            ));
        }

        // Check name length (matches server validation: 1-64 characters)
        if id.len() > 64 {
            return Err(ValidationException(
                "Document ID must be between 1 and 64 characters".to_string(),
            ));
        }

        // Document IDs should be URL-safe: alphanumeric, underscores, and hyphens only
        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(ValidationException(
                "Document ID contains invalid characters. Use only letters, numbers, underscores, and hyphens".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate pagination parameters
    pub fn validate_pagination(
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<(), ValidationException> {
        // Offset can be 0 or positive, so we just check it's not negative
        // (usize is already unsigned, so this is implicit)
        let _ = offset;
        if let Some(lim) = limit {
            if lim < 1 {
                return Err(ValidationException(
                    "Limit must be a positive number".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Validate search parameters
    pub fn validate_search_params(params: &serde_json::Value) -> Result<(), ValidationException> {
        if let Some(offset) = params.get("offset") {
            if let Some(off) = offset.as_u64() {
                // Offset can be 0, so we just check it's a valid number
                let _ = off;
            } else {
                return Err(ValidationException(
                    "Offset must be a non-negative number".to_string(),
                ));
            }
        }
        if let Some(limit) = params.get("limit") {
            if let Some(lim) = limit.as_u64() {
                if lim < 1 {
                    return Err(ValidationException(
                        "Limit must be a positive number".to_string(),
                    ));
                }
            } else {
                return Err(ValidationException(
                    "Limit must be a positive number".to_string(),
                ));
            }
        }
        if let Some(from) = params.get("from") {
            if let Some(f) = from.as_u64() {
                let _ = f;
            } else {
                return Err(ValidationException(
                    "From must be a non-negative number".to_string(),
                ));
            }
        }
        if let Some(size) = params.get("size") {
            if let Some(s) = size.as_u64() {
                if s < 1 {
                    return Err(ValidationException(
                        "Size must be a positive number".to_string(),
                    ));
                }
            } else {
                return Err(ValidationException(
                    "Size must be a positive number".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Validate SQL query text
    pub fn validate_sql(sql: &str) -> Result<(), ValidationException> {
        if sql.trim().is_empty() {
            return Err(ValidationException(
                "SQL query must be a non-empty string".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Validator;

    #[test]
    fn rejects_blank_sql() {
        let error = Validator::validate_sql("   ").expect_err("blank SQL should fail");
        assert_eq!(error.0, "SQL query must be a non-empty string");
    }

    #[test]
    fn accepts_non_blank_sql() {
        Validator::validate_sql("SHOW COLLECTIONS;").expect("SQL should be accepted");
    }
}
