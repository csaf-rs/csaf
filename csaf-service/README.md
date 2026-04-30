# How to

If you want to use the `csaf-rs` library behind a web api, you can use this `csaf-service` to expose it directly or via a docker file.

Run
```bash
cargo run -p csaf-service --release
```

You can define the port by setting an environment variable `CSAF_SERVICE_PORT`, which will be `3000` by default.
```bash
CSAF_SERVICE_PORT=5000 cargo run -p csaf-service --release
```

### CORS

By default, the service uses a restrictive CORS policy. For local development you can opt in to permissive CORS by setting:
```bash
CSAF_SERVICE_PERMISSIVE_CORS=true cargo run -p csaf-service --release
```
