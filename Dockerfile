# Build stage
FROM rust:1.80-slim AS builder

WORKDIR /app

# Install target and download pre-compiled wasm-bindgen
RUN rustup target add wasm32-unknown-unknown && \
    apt-get update && apt-get install -y curl && \
    curl -L https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.125/wasm-bindgen-0.2.125-x86_64-unknown-linux-musl.tar.gz | tar -xz --strip-components=1 -C /usr/local/bin wasm-bindgen-0.2.125-x86_64-unknown-linux-musl/wasm-bindgen

# Copy workspace configuration
COPY Cargo.toml ./
COPY frontend/Cargo.toml ./frontend/

# Create dummy directories for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    mkdir -p frontend/src && echo "" > frontend/src/lib.rs && \
    cargo build --release && \
    cargo build --manifest-path frontend/Cargo.toml --target wasm32-unknown-unknown --release && \
    rm -rf src frontend/src

# Copy real source files
COPY src/ ./src/
COPY frontend/ ./frontend/
COPY static/ ./static/

# Build real WASM library
RUN touch frontend/src/lib.rs && \
    cargo build --manifest-path frontend/Cargo.toml --target wasm32-unknown-unknown --release && \
    mkdir -p static/js/wasm && \
    wasm-bindgen --target web --out-dir static/js/wasm/ target/wasm32-unknown-unknown/release/portfolio_frontend.wasm

# Build real backend binary
RUN touch src/main.rs && cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/huseyin-portfolio /app/huseyin-portfolio

# Copy static assets (including the newly generated WASM files)
COPY --from=builder /app/static/ /app/static/

EXPOSE 80

CMD ["/app/huseyin-portfolio"]
