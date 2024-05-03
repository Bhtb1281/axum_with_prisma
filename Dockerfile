# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /axum_server

FROM rust AS prisma
RUN apt update && apt install -y libssl-dev pkg-config
WORKDIR /prisma_cli
COPY prisma_cli prisma ./
COPY .env .
RUN mkdir .cargo && \
    echo '[alias]\nprisma = "run --"' > .cargo/config.toml && \
    cargo prisma db push

FROM chef AS planner
COPY axum_server/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /axum_server/recipe.json recipe.json
RUN cargo chef cook --release --target=x86_64-unknown-linux-musl --recipe-path recipe.json
COPY axum_server .env ./
COPY prisma/schema.prisma .env /prisma_cli/
COPY --from=prisma /axum_server/src/prisma/prisma.rs src/prisma/prisma.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
WORKDIR /app
COPY --from=builder /axum_server/target/x86_64-unknown-linux-musl/release/axum_server ./
COPY --from=prisma /prisma_cli/dev.db /prisma_cli/.env ./
RUN chown -R myuser:myuser ./
USER myuser
ENTRYPOINT ["/app/axum_server"]
