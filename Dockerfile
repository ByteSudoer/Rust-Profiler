FROM rust:1.75 AS builder

WORKDIR /usr/src/app

COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir .cargo
RUN cargo vendor > .cargo/config


COPY ./src src
COPY ./tests tests
COPY ./benchmarks benchmarks
RUN cargo build --release
RUN cargo install --path . --verbose

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/rust-profiler /app

COPY .env .

COPY ./src ./src

CMD ["./rust-profiler"]

