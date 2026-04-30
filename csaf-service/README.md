# How to

If you want to use the `csaf-rs` library behind a web API, you can use this `csaf-service` to expose it directly or via a Dockerfile.

Run
```bash
cargo run -p csaf-service --release
```

You can define the port by setting an environment variable `CSAF_SERVICE_PORT`, which will be `3000` by default.
```bash
CSAF_SERVICE_PORT=5000 cargo run -p csaf-service --release
```

## CORS

By default, the service uses a restrictive CORS policy. For local development you can opt in to permissive CORS by setting:
```bash
CSAF_SERVICE_PERMISSIVE_CORS=true cargo run -p csaf-service --release
```

## Request Body Limit

The maximum request body size defaults to 150 MB (the CSAF document size limit). You can lower it via:
```bash
CSAF_SERVICE_BODY_LIMIT=52428800 cargo run -p csaf-service --release  # 50 MB
```

Values exceeding 150 MB are clamped to the maximum.
