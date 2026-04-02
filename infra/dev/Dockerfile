FROM rust:1.84-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Build dependencies first to cache them
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml
# Create dummy src files to trick Cargo into building dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && echo "fn main() {}" > migration/src/lib.rs
RUN cargo build --release
# Remove the dummy files
RUN rm -rf src migration/src

# Now copy the actual source code
COPY src ./src
COPY migration ./migration

# Touch the main file so cargo rebuilds the application and install it to a known directory
RUN touch src/main.rs && cargo install --path . --root /usr/src/app/out

FROM debian:bookworm-slim

# Install runtime dependencies (like OpenSSL and ca-certificates)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage using the generic bin directory
COPY --from=builder /usr/src/app/out/bin/* ./app
COPY config.toml ./config.toml

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["./app"]