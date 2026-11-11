mod handlers;
mod routes;
mod settings;
mod test_helpers;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use std::future::Future;
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
        version = "0.7.1",
        description = "REST API for validating CSAF (Common Security Advisory Framework) documents against the OASIS CSAF standard.
        Uses csaf-rs under the hood. Currently conforms to the format defined by Secvisogram CSAF Validator Service (see https://github.com/secvisogram/csaf-validator-service)."
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
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Returns a future that resolves when the process is asked to terminate
/// (Ctrl+C/SIGINT or SIGTERM). Signal handlers are installed eagerly, before
/// the future is first polled: as PID 1 in a container the kernel drops
/// signals with a default-terminate disposition, so a SIGTERM arriving before
/// the first poll would otherwise be lost (or kill us, if unhandled) and
/// `docker stop` would end in SIGKILL.
fn shutdown_signal() -> impl Future<Output = ()> + Send {
    #[cfg(unix)]
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
        .expect("failed to install SIGINT handler");
    #[cfg(unix)]
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler");

    async move {
        #[cfg(unix)]
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
        #[cfg(not(unix))]
        tokio::signal::ctrl_c().await.expect("failed to install SIGINT handler");

        tracing::info!("Shutdown signal received, stopping");
    }
}

#[cfg(all(test, unix))]
mod shutdown_tests {
    use super::shutdown_signal;
    use std::time::Duration;

    /// Raising SIGTERM against our own process must resolve `shutdown_signal`.
    /// The handler is installed when the future is created, so it is safe to
    /// send the signal afterwards.
    /// Raising SIGTERM against our own process must resolve `shutdown_signal`.
    /// Safe for the whole test binary because the handlers are installed
    /// eagerly when the future is created, before we send the signal.
    #[tokio::test]
    async fn sigterm_resolves_shutdown_signal() {
        let fut = shutdown_signal();
        std::process::Command::new("kill")
            .args(["-TERM", &std::process::id().to_string()])
            .status()
            .expect("`kill` must be available on unix");
        tokio::time::timeout(Duration::from_secs(5), fut)
            .await
            .expect("shutdown_signal did not resolve on SIGTERM");
    }
}
