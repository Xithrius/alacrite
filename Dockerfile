FROM rust:1-slim-bookworm AS builder
WORKDIR /code

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY . .

RUN cargo build --release

FROM bitnami/minideb:bookworm

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /code/target/release/alacrite-api /code/alacrite-api
COPY --from=builder /code/migrations/* /code/migrations/
WORKDIR /code

USER 1001
EXPOSE 8001

CMD [ "/code/alacrite-api" ]
