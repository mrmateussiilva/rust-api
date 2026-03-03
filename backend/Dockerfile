FROM rust:1.75 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists

WORKDIR /app

COPY --from=builder /app/target/release/rust-api .

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./rust-api"]
