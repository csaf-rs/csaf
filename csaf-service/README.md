# How to

If you want to use the `csaf-rs` library behind a web API, you can use this `csaf-service` to expose it directly or via a Dockerfile.

Run
```bash
cargo run -p csaf-service --release
```

You can change the following settings via environment variables:
| Setting | Description | Default Value |
| --- | --- | --- |
| `CSAF_SERVICE_HOST` | The host the service listens on. Change to `0.0.0.0` to listen on all devices. | `127.0.0.1` |
| `CSAF_SERVICE_PORT` | The port the service listens on. | `8082` |
| `CSAF_SERVICE_PERMISSIVE_CORS` | Whether to use a permissive CORS policy (allow all origins). Allows `1` or `true` to activate. | `false` |
| `CSAF_SERVICE_BODY_LIMIT` | The maximum request body size in bytes. Values exceeding this limit are clamped to the maximum. | `157286400` (150 MB) |
