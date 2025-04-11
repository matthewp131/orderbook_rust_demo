FROM rust:1.60

WORKDIR /usr/src/orderbook
COPY . .

RUN cargo install --path .

CMD ["./target/release/main", "input_file.csv", "-t"]
