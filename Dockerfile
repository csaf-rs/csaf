FROM rust:1.95-alpine3.21 AS builder

RUN apk add --no-cache musl-dev curl

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY csaf-rs/ csaf-rs/
COPY csaf-service/ csaf-service/
COPY csaf-validator/ csaf-validator/
COPY csaf-converter/ csaf-converter/
COPY type-generator/ type-generator/

# Drop csaf-ffi from workspace members (not needed for the service)
RUN sed -i 's/, "csaf-ffi"//' Cargo.toml

# Build the service in release mode (musl is the native target on Alpine)
RUN cargo build --release -p csaf-service

# Runtime stage
FROM alpine:3.21

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/csaf-service /usr/local/bin/csaf-service

ENV CSAF_SERVICE_PORT=3000
# Listen on all interfaces for container compatibility
ENV CSAF_SERVICE_HOST=0.0.0.0
EXPOSE 3000

ENTRYPOINT ["csaf-service"]
