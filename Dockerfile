FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bot ./bot
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/reaction-role ./

EXPOSE 8080

CMD ["./reaction-role"]
