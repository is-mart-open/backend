FROM lukemathwalker/cargo-chef:latest-rust-1.56.0-alpine3.14 AS chef
WORKDIR /app
RUN apk add --no-cache openssl-dev

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:3.14 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/is-mart-open-api /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/is-mart-open-api" ]