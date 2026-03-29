FROM rust:1.84-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migration ./migration

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies (like OpenSSL and ca-certificates)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/app ./app
COPY config.toml ./config.toml

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["./app"]