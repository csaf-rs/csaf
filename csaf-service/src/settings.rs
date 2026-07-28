use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use tracing::Level;

/// Defaults are compiled into the binary so the service runs correctly even
/// when no config directory is present at runtime (e.g. a bare binary or a
/// minimal container image). `config/default.toml` remains the single
/// source of truth for these values and is kept in sync via `include_str!`.
const DEFAULT_CONFIG: &str = include_str!("../config/default.toml");

/// Hard safety ceiling for request body size, as required by the CSAF
/// standard. `server.body_limit_mb` is clamped to this value regardless of
/// what is configured.
pub const MAX_BODY_LIMIT_BYTES: usize = 150 * 1024 * 1024; // 150 MB

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub body_limit_mb: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    pub permissive: bool,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub allowed_methods: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    /// Default log filter, used when the `RUST_LOG` environment variable is
    /// not set. Accepts anything `tracing_subscriber::EnvFilter` understands
    /// (e.g. `"info"` or `"info,tower_http=debug"`).
    pub level: String,
    /// Level at which incoming/outgoing HTTP requests are logged by
    /// `tower_http`'s `TraceLayer`. Must be one of `trace`, `debug`, `info`,
    /// `warn`, `error`.
    pub request_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub cors: CorsSettings,
    pub logging: LoggingSettings,
}

impl Settings {
    /// Loads settings from, in increasing order of precedence:
    /// 1. Built-in defaults, compiled from `config/default.toml`.
    /// 2. `csaf-service/config/local.toml` (optional, gitignored,
    ///    per-developer/per-deployment overrides). It is looked up both
    ///    relative to the crate directory and relative to the workspace
    ///    root, so it is found whether the service is run from the
    ///    workspace root (`cargo run -p csaf-service`) or from within
    ///    `csaf-service/` directly.
    /// 3. Environment variables prefixed with `CSAF_SERVICE__` (note the
    ///    double underscore, which is also used as the nesting separator),
    ///    e.g. `CSAF_SERVICE__SERVER__PORT` or
    ///    `CSAF_SERVICE__CORS__PERMISSIVE`.
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
            .add_source(File::with_name("config/local").required(false))
            .add_source(File::with_name("csaf-service/config/local").required(false))
            .add_source(
                Environment::with_prefix("CSAF_SERVICE")
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("cors.allowed_origins")
                    .with_list_parse_key("cors.allowed_methods")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }

    pub fn body_limit_bytes(&self) -> usize {
        self.server
            .body_limit_mb
            .saturating_mul(1024 * 1024)
            .min(MAX_BODY_LIMIT_BYTES)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Parses `logging.request_level`, falling back to `INFO` (and logging a
    /// warning) if it isn't a valid tracing level.
    pub fn request_log_level(&self) -> Level {
        self.logging.request_level.parse().unwrap_or_else(|_| {
            tracing::warn!(
                "Invalid logging.request_level '{}', falling back to INFO",
                self.logging.request_level
            );
            Level::INFO
        })
    }
}
