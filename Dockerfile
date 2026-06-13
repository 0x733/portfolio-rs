# Build stage
FROM rust:1.80-slim AS builder

WORKDIR /app

# Copy dependency configuration
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs for caching dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source files
COPY src/ ./src/
COPY static/ ./static/

# Build real backend binary
RUN touch src/main.rs && cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/huseyin-portfolio /app/huseyin-portfolio

# Copy static assets
COPY --from=builder /app/static/ /app/static/

EXPOSE 80

CMD ["/app/huseyin-portfolio"]
