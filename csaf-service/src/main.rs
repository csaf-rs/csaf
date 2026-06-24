mod errors;
mod handlers;
mod routes;
mod test_helpers;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::errors::ErrorResponse;
use crate::handlers::get_preset_tests::*;
use crate::handlers::get_presets::*;
use crate::handlers::get_tests::*;
use crate::handlers::health::*;
use crate::handlers::legacy::*;
use crate::handlers::validate::*;

fn permissive_cors_enabled() -> bool {
    std::env::var("CSAF_SERVICE_PERMISSIVE_CORS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

const MAX_BODY_SIZE: usize = 150 * 1024 * 1024; // 150 MB

fn body_limit() -> usize {
    std::env::var("CSAF_SERVICE_BODY_LIMIT")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(MAX_BODY_SIZE)
        .min(MAX_BODY_SIZE)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        //handlers::get_presets::get_presets,
        //handlers::get_preset_tests::get_preset_tests,
        //handlers::get_tests::get_tests,
        //handlers::validate::validate,
        handlers::health::health,
        handlers::legacy::get_tests_legacy,
        handlers::legacy::validate_legacy,
    ),
    components(schemas(
        //PresetsResponse,
        //PresetTestsResponse,
        //TestsResponse,
        ErrorResponse,
        //LegacyTestInPreset,
        //LegacyValidateBody,
        TestOrPreset,
        //LegacyValidateResponse,
        //LegacyTestResult,
        //LegacyFinding
    )),
    tags(
        (name = "meta", description = "CSAF preset and test listing and details"),
        (name = "validation", description = "CSAF document validation"),
        (name = "health", description = "Service health checks"),
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

    let port = std::env::var("CSAF_SERVICE_PORT").unwrap_or_else(|_| "8082".to_string());
    let host = std::env::var("CSAF_SERVICE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let addr = format!("{host}:{port}");

    let cors_layer = if permissive_cors_enabled() {
        tracing::warn!("Permissive CORS is enabled — do not use in production");
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
    };
    // ToDo: Allow configuring CORS more granularly (e.g. allowed origins) via environment variables
    // See https://docs.rs/tower-http/latest/tower_http/cors/struct.CorsLayer.html for details

    let app = Router::new()
        //.route(routes::PRESETS, get(get_presets))
        //.route(routes::TESTS, get(get_tests))
        //.route(routes::PRESET_TESTS, get(get_preset_tests))
        //.route(routes::VALIDATE, post(validate))
        .route(routes::HEALTH, get(health))
        .route(routes::TESTS_LEGACY, get(get_tests_legacy))
        .route(routes::VALIDATE_LEGACY, post(validate_legacy))
        .merge(SwaggerUi::new("/openapi").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(DefaultBodyLimit::max(body_limit()))
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    tracing::info!("Starting CSAF Validation API on {addr}");
    tracing::info!("Swagger UI available at http://{addr}/openapi/");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
