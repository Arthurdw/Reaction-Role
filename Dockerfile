FROM rust:1.98 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bot ./bot
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/reaction-role ./

RUN useradd --system --uid 10001 --no-create-home reaction-role \
    && mkdir -p db config \
    && chown -R reaction-role /app

USER reaction-role

EXPOSE 8080

CMD ["./reaction-role"]
