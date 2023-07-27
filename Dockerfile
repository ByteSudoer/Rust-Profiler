FROM rust:1.71

WORKDIR /app

COPY . .
RUN cargo build --release

CMD ["./target/release/rust-profiler"]
