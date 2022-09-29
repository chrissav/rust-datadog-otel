FROM rust:1.63.0

COPY . .

RUN cargo build --release

CMD ["./target/release/datadog"]
