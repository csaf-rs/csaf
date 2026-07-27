mod handlers;
mod routes;
mod settings;
mod test_helpers;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::health::*;
use crate::handlers::v1::errors::*;
use crate::handlers::v1::get_tests::*;
use crate::handlers::v1::validate::*;
use crate::settings::{CorsSettings, Settings};

fn build_cors_layer(cors: &CorsSettings) -> CorsLayer {
    if cors.permissive {
        tracing::warn!("Permissive CORS is enabled — do not use in production");
        return CorsLayer::permissive();
    }

    let origins: Vec<HeaderValue> = cors
        .allowed_origins
        .iter()
        .filter_map(|origin| {
            origin
                .parse()
                .inspect_err(|_| tracing::warn!("Ignoring invalid CORS origin: {origin}"))
                .ok()
        })
        .collect();

    let methods: Vec<Method> = cors
        .allowed_methods
        .iter()
        .filter_map(|method| {
            method
                .parse()
                .inspect_err(|_| tracing::warn!("Ignoring invalid CORS method: {method}"))
                .ok()
        })
        .collect();

    let mut layer = CorsLayer::new();
    if !origins.is_empty() {
        layer = layer.allow_origin(origins);
    }
    if !methods.is_empty() {
        layer = layer.allow_methods(methods);
    }
    layer
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health::health,
        handlers::v1::get_tests::get_tests,
        handlers::v1::validate::validate,
    ),
    components(schemas(
        ErrorResponse,
        TestInPreset,
        ValidateBody,
        TestOrPreset,
        ValidateResponse,
        TestResult,
        Finding
    )),
    tags(
        (name = "meta", description = "CSAF preset and test listing and details"),
        (name = "validation", description = "CSAF document validation"),
        (name = "health", description = "Service health checks"),
    ),
    info(
        title = "CSAF Validation API",
        version = "0.5.1",
        description = "REST API for validating CSAF (Common Security Advisory Framework) documents against the OASIS CSAF standard. Uses csaf-rs under the hood."
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let settings = Settings::load().expect("Failed to load configuration");

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| settings.logging.level.clone().into()),
        )
        .init();

    let addr = settings.addr();
    let cors_layer = build_cors_layer(&settings.cors);
    let request_level = settings.request_log_level();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(request_level))
        .on_request(DefaultOnRequest::new().level(request_level))
        .on_response(DefaultOnResponse::new().level(request_level));

    let app = Router::new()
        .route(routes::HEALTH, get(health))
        .route(routes::V1_TESTS, get(get_tests))
        .route(routes::V1_VALIDATE, post(validate))
        .merge(SwaggerUi::new("/openapi").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(DefaultBodyLimit::max(settings.body_limit_bytes()))
        .layer(cors_layer)
        .layer(trace_layer);

    tracing::info!("Starting CSAF Validation API on {addr}");
    tracing::info!("Swagger UI available at http://{addr}/openapi/");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
