# Orderbook Demo
This crate simulates an orderbook accepting limit orders for buying and selling, with the option to support matching trades with the `-t` flag. 

## Build and run

### Install Rust and Cargo
See https://www.rust-lang.org/tools/install for the latest Rust and Cargo tools for your OS.

### Generate Documentation
To see documentation, run `cargo doc --no-deps --open`

### Run without trading enabled
`cargo run input_file.csv`
### Run with trading enabled
`cargo run input_file.csv -t`

## Run with Docker
Note: the dockerfile has trading mode enabled by default. Remove '-t' from Dockerfile line 8 to run without trading enabled.
```
docker build -t orderbook .
docker run -it --rm orderbook
```

## Run with Docker Compose
`docker-compose up --build`

## Unit tests
Unit tests are provided covering all of the scenarios shown in output_file.csv. Run them with `cargo test --release -- --nocapture --test-threads 1` for accurate timing.
See the optimize-order-cancellation branch for my attempt to improve order cancellation efficiency by storing order metadata in a separate data structure. While this eliminated some additional for-loops, the overall runtime performance was actually not noticably different. It's possible that the hash operations needed caused as much impact as the additional for loops, and that it would require many more orders present in the order book for this optimization to prove beneficial.

## Input
The input CSV file may contain the following:
1. An introductory block, which will also be copied into output, such as 
    ```
    #name: scenario 1
    #descr: balanced book, my first scenario
    ```
1. A new order command: N, user(int), symbol(string), price(int), qty(int), side('B' or 'S'), userOrderId(int). For example: 
    ```
    N, 1, IBM, 10, 100, B, 1
    ```
1. A cancel order command: C, user(int), userOrderId(int). For example:
    ```
    C, 1, 1
    ```
1. A flush orderbooks command: F. For example:
    ```
    F
    ```

## Output
The output will be sent to stdout, but may be piped to a CSV file by running `cargo run input_file.csv > output_file.csv`.

The output may contain the following:
1. An acknowledgement of new order placement or order cancellation: A, userId(int), userOrderId(int). For example:
    ```
    A, 1, 1
    ```
1. A change at the top of the book for the Buy or Sell side: B, side('B' or 'S'), price(int), totalQuantity(int). For example:
    ```
    B, B, 10, 100
    ```
1. When trade matching is disabled, a rejection for orders that would cross the book: R, userId(int), userOrderId(int). For example:
    ```
    R, 1, 1
    ```
1. When trade matching is enabled, a matched order acknowledgement: T, userIdBuy(int), userOrderIdBuy(int), userIdSell(int), userOrderIdSell(int), price(int), quantity(int). For example:
    ```
    T, 1, 1, 2, 101, 10, 100
    ````
