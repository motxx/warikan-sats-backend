FROM rust:latest as builder

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y protobuf-compiler

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src

RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/warikan-sats-backend /usr/local/bin/warikan-sats-backend

CMD ["warikan-sats-backend"]
