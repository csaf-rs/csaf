use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON schema error: {0}")]
    Schema(#[from] typify::Error),
    #[error("Rust syntax error: {0}")]
    Syntax(#[from] syn::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Path escape error: {0}")]
    PathEscape(String),
    #[error("Schema patch error: {0}")]
    SchemaPatch(String),
}
