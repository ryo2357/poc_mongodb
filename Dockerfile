# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/template-rust-devcontainer /app/template-rust-devcontainer
CMD ["/app/template-rust-devcontainer"]