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

    // Build the router - serves all static files
    let app = Router::new()
        .route("/", get(index_handler))
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

/// Handler for the index page
async fn index_handler() -> impl IntoResponse {
    static_handler(axum::extract::Path("index.html".to_string())).await
}

/// Handler for all static files (HTML, JS, CSS, WASM, etc.)
async fn static_handler(axum::extract::Path(path): axum::extract::Path<String>) -> Response {
    // Remove leading slash if present
    let path = path.trim_start_matches('/');
    
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
