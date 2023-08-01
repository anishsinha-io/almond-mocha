FROM rust:1.71-bullseye
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/mocha"]
