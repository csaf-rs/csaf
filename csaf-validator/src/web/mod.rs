//! Web server module for the CSAF validator
//!
//! Provides an embedded web server that serves a demo UI for validating CSAF documents.
//! All validation is performed client-side using WASM.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;

/// Embedded static files (HTML, CSS, JS, WASM)
#[derive(RustEmbed)]
#[folder = "src/web/static/"]
struct StaticAssets;

/// Start the web server
pub async fn start_server(host: &str, port: u16) -> anyhow::Result<()> {
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // Build the router - serve all static files with fallback to index.html
    let app = Router::new()
        .route("/", get(static_handler))
        .route("/*path", get(static_handler));

    println!("\n🚀 CSAF Validator Web UI starting...");
    println!("📝 Open your browser and navigate to: http://{}:{}", host, port);
    println!("💡 All validation runs in your browser using WebAssembly");
    println!("Press Ctrl+C to stop the server\n");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Handler for all static files (HTML, JS, CSS, WASM, etc.)
/// Defaults to index.html for root path
async fn static_handler(path: Option<axum::extract::Path<String>>) -> Response {
    let path = path
        .map(|p| p.0)
        .unwrap_or_else(|| "index.html".to_string());
    
    let path = path.trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    
    match StaticAssets::get(path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, mime_type.as_ref())],
                content.data.to_vec(),
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", path),
        )
            .into_response(),
    }
}
