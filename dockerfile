# Stage 1: Build the Enigmash application
FROM rust:alpine AS builder

# Set working directory for build context
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock if available
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN cargo build --release || true

# Now copy the rest of the source code
COPY src ./src

# Build the Enigmash application in release mode
RUN cargo build --release

# Final image (smaller, no build tools)
FROM alpine:latest

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/enigmash .

# # Ensure the binary has execution permissions
# RUN chmod +x /app/enigmash

# Set the entrypoint to run your binary
ENTRYPOINT ["./enigmash"]
