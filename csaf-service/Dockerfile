FROM rust:1.95-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

# Wrapper to strip -m64 which musl-gcc does not support
RUN printf '#!/bin/sh\nwhile [ $# -gt 0 ]; do\n  case "$1" in\n    -m64) shift ;;\n    *) set -- "$@" "$1"; shift ;;\n  esac\ndone\nexec musl-gcc "$@"\n' \
    > /usr/local/bin/musl-gcc-wrapper && chmod +x /usr/local/bin/musl-gcc-wrapper
ENV CC_x86_64_unknown_linux_musl=/usr/local/bin/musl-gcc-wrapper
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=/usr/local/bin/musl-gcc-wrapper

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY csaf-rs/ csaf-rs/
COPY csaf-service/ csaf-service/
COPY csaf-validator/ csaf-validator/
COPY csaf-converter/ csaf-converter/
COPY type-generator/ type-generator/

# Build the service in release mode targeting musl for Alpine compatibility
RUN cargo build --release -p csaf-service --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:3

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/csaf-service /usr/local/bin/csaf-service

ENV PORT=3000
EXPOSE 3000

ENTRYPOINT ["csaf-service"]
