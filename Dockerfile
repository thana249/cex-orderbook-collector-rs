# Build Stage
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache the dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/cex_orderbook_collector_rs*

# Now copy your actual source code
COPY ./src ./src

# Build for release
RUN cargo build --release


# Runtime Stage
FROM debian:bookworm-slim
ARG APP=/usr/src/app

# Create app directory
RUN mkdir -p ${APP}

WORKDIR ${APP}

# Install OpenSSL and CA certificates
RUN apt-get update && apt-get install -y \
    openssl \
    libssl-dev \
    ca-certificates

# Ensure CA certificates are up to date
RUN update-ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/cex-orderbook-collector-rs ${APP}/app

# Copy the config.json file
COPY ./config.json ${APP}/config.json

# Run the binary
CMD ["./app"]
