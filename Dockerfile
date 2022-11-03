FROM rust:1.63.0-slim-bullseye AS builder

RUN apt-get update \
    && apt-get install -y curl ca-certificates pkg-config libssl-dev \
    && update-ca-certificates

WORKDIR /app

COPY . .

RUN cargo build --release

## Production

FROM debian:11-slim

RUN apt-get update \
    && apt-get install -y curl ca-certificates \
    && update-ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/liq_rs /app

ENTRYPOINT [ "/app/liq_rs" ]