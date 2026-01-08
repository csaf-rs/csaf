use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("JSON schema error")]
    Schema(#[from] typify::Error),
    #[error("Rust syntax error")]
    Syntax(#[from] syn::Error),
    #[error("JSON parsing error")]
    Json(#[from] serde_json::Error),
}
