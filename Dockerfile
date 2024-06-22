FROM lukemathwalker/cargo-chef:latest as chef

# use cargo-chef, easier docker caching

WORKDIR /app


FROM chef AS planner

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./currency-api ./currency-api
RUN cargo chef prepare


FROM chef AS builder

ENV BIN=currency-converter-v2

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./currency-api ./currency-api
RUN cargo build --release
RUN mv ./target/release/$BIN ./app


FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]