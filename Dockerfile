FROM alpine AS builder
WORKDIR /app
ARG TARGETPLATFORM

COPY ./target/x86_64-unknown-linux-musl/release/ ./target/x86_64-unknown-linux-musl/release/
COPY ./target/aarch64-unknown-linux-musl/release/ ./target/aarch64-unknown-linux-musl/release/

RUN mkdir /app/release

RUN case ${TARGETPLATFORM} in \
    linux/amd64) echo "amd64" && mv target/x86_64-unknown-linux-musl/release/is-mart-open-api ./release ;; \
    linux/arm64) echo "arm64" && mv target/aarch64-unknown-linux-musl/release/is-mart-open-api ./release ;; \
    esac

FROM alpine AS runtime
WORKDIR /app

COPY --from=builder /app/release/is-mart-open-api /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/is-mart-open-api" ]