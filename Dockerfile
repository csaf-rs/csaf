# Dependencies are compiled in a separate layer (cargo-chef), so a change in the
# workspace sources only recompiles the workspace crates, not all dependencies.

# Base for the build stages: Rust toolchain + cargo-chef (official image, pinned by
# multi-arch index digest; the tag is kept for readability and Dependabot updates)
FROM lukemathwalker/cargo-chef:0.1.78-rust-1.100.0-alpine3.24@sha256:3dc65f92f830cadaa78d29ddcc8e23a22c5d6785f7758afdf50eba4e88238966 AS chef

# curl: utoipa-swagger-ui's build script downloads the Swagger UI bundle with it
RUN apk add --no-cache musl-dev curl

WORKDIR /app

# Planner: reduce the workspace to a recipe of its dependencies
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY csaf-rs/ csaf-rs/
COPY csaf-service/ csaf-service/
COPY csaf-validator/ csaf-validator/
COPY csaf-converter/ csaf-converter/
COPY type-generator/ type-generator/
COPY csaf-result-json/ csaf-result-json/
COPY csaf/ csaf/

# Drop csaf-ffi from workspace members (not needed for the service)
RUN sed -i 's/, "csaf-ffi"//' Cargo.toml

RUN cargo chef prepare --recipe-path recipe.json

# Builder: compile the dependencies first, then the workspace
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# This layer is only rebuilt when the dependencies (recipe.json) change
RUN cargo chef cook --release -p csaf-service --recipe-path recipe.json

COPY --from=planner /app/ ./

# Build the service in release mode (musl is the native target on Alpine)
RUN cargo build --release -p csaf-service

# Runtime stage — distroless static (no shell, no package manager; ships ca-certificates + tzdata)
FROM gcr.io/distroless/static-debian12:nonroot@sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab

COPY --from=builder /app/target/release/csaf-service /usr/local/bin/csaf-service

ENV CSAF_SERVICE__SERVER__PORT=8082
# Listen on all interfaces for container compatibility
ENV CSAF_SERVICE__SERVER__HOST=0.0.0.0
EXPOSE 8082

ENTRYPOINT ["/usr/local/bin/csaf-service"]
