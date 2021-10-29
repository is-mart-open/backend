FROM rust AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/is-mart-open-api /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/is-mart-open-api" ]