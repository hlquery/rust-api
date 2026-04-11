/**
 * hlquery Rust Client - Main Entry Point
 * 
 * Copyright (C) 2021-2026, Carlos F. Ferry <carlos.ferry@gmail.com>
 * 
 * This file is part of hlquery, released under the BSD License version 3.
 */

pub mod client;
pub mod request;
pub mod response;
pub mod collections;
pub mod documents;
pub mod search;
pub mod error;
pub mod utils;
pub mod ranker;

// Re-export main types
pub use client::Client;
pub use request::Request;
pub use response::Response;
pub use collections::Collections;
pub use documents::Documents;
pub use search::Search;
pub use error::{HlqueryError, Result};

// Re-export utilities
pub use utils::config::Config;
pub use utils::validator::Validator;
pub use utils::auth::Auth;
pub use ranker::{attach_rank_sort, compute_rank_signal};
