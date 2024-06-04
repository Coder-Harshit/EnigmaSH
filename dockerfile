FROM alpine:latest AS builder

# Set working directory for build context
WORKDIR /app

COPY src/*.rs .

# Install Rust toolchain (multi-stage build for smaller image)
RUN apk add --no-cache --virtual .rust-toolchain \
    rustc \
    cargo \
    gcc

# Build the Enigmash application in release mode
WORKDIR /app
RUN cargo build --release

# Final image (smaller, no build tools)
FROM alpine:latest

# Copy the compiled binary
COPY target/release/enigmash .

# Set the entrypoint to run your shell program
ENTRYPOINT ["./enigmash"]
