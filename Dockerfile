# Build stage
FROM rust:1.80-slim AS builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Create dummy source and build dependencies for caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy real source code
COPY src/ ./src/

# Rebuild real application
RUN touch src/main.rs && cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/huseyin-portfolio /app/huseyin-portfolio

# Copy static assets
COPY static/ /app/static/

EXPOSE 80

CMD ["/app/huseyin-portfolio"]
