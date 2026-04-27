mod handlers;
mod models;

use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::get_preset_tests::*;
use crate::handlers::get_presets::*;
use crate::handlers::health::*;
use crate::handlers::validate::*;
use crate::models::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_presets::get_presets,
        handlers::get_preset_tests::get_preset_tests,
        handlers::validate::validate,
        handlers::validate::validate_file,
        handlers::health::health,
    ),
    components(schemas(
        PresetsResponse,
        PresetTestsResponse,
        ErrorResponse,
    )),
    tags(
        (name = "presets", description = "CSAF preset listing and details"),
        (name = "validation", description = "CSAF document validation"),
        (name = "health", description = "Service health checks")
    ),
    info(
        title = "CSAF Validation API",
        version = "0.4.1",
        description = "REST API for validating CSAF (Common Security Advisory Framework) documents against the OASIS CSAF standard. Uses csaf-rs under the hood."
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let port = std::env::var("CSAF_SERVICE_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let app = Router::new()
        .route("/api/v1/csaf/{version}/presets", get(get_presets))
        .route("/api/v1/csaf/{version}/presets/{preset}/tests", get(get_preset_tests))
        .route("/api/v1/csaf/{version}/validate", post(validate))
        .route("/api/v1/csaf/{version}/validate/file", post(validate_file))
        .route("/api/v1/health", get(health))
        .merge(SwaggerUi::new("/swagger-ui").url("/api/v1/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing::info!("Starting CSAF Validation API on {addr}");
    tracing::info!("Swagger UI available at http://{addr}/swagger-ui/");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
