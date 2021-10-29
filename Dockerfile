FROM rust AS builder
WORKDIR /app
RUN apt install -y build-essential
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/is-mart-open-api /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/is-mart-open-api" ]