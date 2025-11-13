# Stage 1: build
FROM rust:1.81 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: runtime
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-login-backend /app/
EXPOSE 3000
CMD ["./rust-login-backend"]
