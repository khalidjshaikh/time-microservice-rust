# ---- Build stage ----
FROM rust:1.99 AS builder

WORKDIR /src

ARG VERSION=dev
ARG COMMIT=none
ARG BUILD_DATE=unknown
ENV VERSION=${VERSION}
ENV COMMIT=${COMMIT}
ENV BUILD_DATE=${BUILD_DATE}

COPY Cargo.toml build.rs ./
COPY src ./src

RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim

COPY --from=builder /src/target/release/time-microservice-rust /time-microservice-rust

EXPOSE 8080

ENV PORT=8080
ENV TZ=UTC

ENTRYPOINT ["/time-microservice-rust"]
