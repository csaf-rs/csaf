# How to

If you want to use the `csaf-rs` library behind a web API, you can use this `csaf-service` to expose it directly or via a Dockerfile.

Run
```bash
cargo run -p csaf-service --release
```

## Configuration

Settings are loaded, in increasing order of precedence, from:
1. Built-in defaults, compiled from [`config/default.toml`](config/default.toml).
2. `config/local.toml` — optional, gitignored, for per-developer or per-deployment
   overrides. Copy [`config/local.example.toml`](config/local.example.toml) to
   `config/local.toml` and adjust as needed (e.g. to relax CORS locally). Only
   include the keys you want to change.
3. Environment variables prefixed with `CSAF_SERVICE__` (double underscore,
   also used as the nesting separator).

| Setting | Config key | Environment variable | Description | Default Value |
| --- | --- | --- | --- | --- |
| Host | `server.host` | `CSAF_SERVICE__SERVER__HOST` | The host the service listens on. Change to `0.0.0.0` to listen on all devices. | `localhost` |
| Port | `server.port` | `CSAF_SERVICE__SERVER__PORT` | The port the service listens on. | `8082` |
| Body limit | `server.body_limit_mb` | `CSAF_SERVICE__SERVER__BODY_LIMIT_MB` | The maximum request body size in MB. Hard-capped at 150 MB as required by the standard, regardless of this setting. | `150` |
| Permissive CORS | `cors.permissive` | `CSAF_SERVICE__CORS__PERMISSIVE` | Whether to use a permissive CORS policy (allow all origins/methods/headers). Do not use in production. | `false` |
| Allowed origins | `cors.allowed_origins` | `CSAF_SERVICE__CORS__ALLOWED_ORIGINS` | Allow-list of origins for CORS, used when `permissive` is `false`. | `[]` |
| Allowed methods | `cors.allowed_methods` | `CSAF_SERVICE__CORS__ALLOWED_METHODS` | Allow-list of HTTP methods for CORS, used when `permissive` is `false`. | `["GET", "POST"]` |
