# Orderbook Demo
This crate simulates an orderbook accepting limit orders for buying and selling, with the option to support matching trades with the `-t` flag. Please see [Generate Documentation](#generate-documentation) for source code documentation. Please see [Runtime Complexity](#runtime-complexity) for info on time and space complexity and potential optimizations. Please see [Run with Docker](#run-with-docker) for information on starting with either Docker or Docker Compose.

## Build and run locally

### Install Rust and Cargo
See https://www.rust-lang.org/tools/install for the latest Rust and Cargo tools for your OS.

### Generate Documentation
To see documentation, run `cargo doc --no-deps --open`

### Run without trading enabled
`cargo run input_file.csv`
### Run with trading enabled
`cargo run input_file.csv -t` or `cargo run input_file.csv --trading-enabled`
### Run and pipe output to CSV
`cargo run input_file.csv > output_file.csv`

## Run with Docker
Note: the dockerfile has trading mode enabled by default. Remove '-t' from Dockerfile line 8 to run without trading enabled.
```
docker build -t orderbook .
docker run -it --rm orderbook
```

## Run with Docker Compose
`docker-compose up --build`

## Unit tests
Unit tests are provided covering all of the scenarios shown in output_file.csv. Run them with `cargo test`, or to see a printout of time elapsed for orders involving cancellations, use `cargo test --release -- --nocapture --test-threads 1` for accurate timing.
See the optimize-order-cancellation branch for my attempt to improve order cancellation efficiency by storing order metadata in a separate data structure. While this eliminated some additional for-loops, the overall runtime performance was actually not noticably different. It's possible that the hash operations needed caused as much impact as the additional for loops, and that it would require many more orders present in the order book for this optimization to prove beneficial. Further detail is given in [Runtime Complexity](#runtime-complexity).

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
    In the future it would be helpful to provide distinct messages for new order accepted and existing order cancelled.

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

## Runtime Complexity
### New Orders
#### Time Complexity
First, the `OrderBooks` struct must locate the proper `OrderBook` for the new order in its `all_orders: HashMap<String, OrderBook>` or make a new `OrderBook` for the new `symbol: String`. This can be done with O(1) complexity except in case of hashing collisions, where '1' is the theoretically constant time it takes to hash the key.

Next, the `NewOrder` must be added to the proper `Orderbook`. Here, it must first be evaluated for crossing the book, which is O(1). If trading is enabled, matching the trade is an additional O(n*m) operation. This is of high complexity because, as per the output provided for scenario 13 (scenario 5 with trading), a buy order which crosses the book does not necessarily match to a sell order at the price of the buy order, but instead matches first to any sell order of equal quantity which is at a lesser price than the buy order. Likewise, a sell order crossing the book may match to a buy order for a price higher than the sell order. If this limitation were removed, and only exact matches of price and quantity were allowed, then matching the trade would become an O(n) operation. 
#### Space Complexity
Under the current implementation, orders are only stored in one data structure each, and no additional associating structures are used to decrease lookup time during order matching and/or cancellation. As discussed in [Unit tests](#unit-tests) above, an initial attempt to add association between (user, user_order_id) and an orderbook location did not improve performance. However, with a much larger set of orders, the additional time used to create the order metadata might yeild better performance when performing matching and cancellation.

### Cancelling Orders
#### Time Complexity
At the `OrderBooks` level, because the symbol associated with the cancellation (user, user_order_id) is not known (See discussion in [Unit tests](#unit-tests) above regarding potential optimization), the cancellation must be attempted against each symbol, an O(n) operation. Within each `OrderBook`, the attempt at cancellation is O(n*m), searching through every order at every price level. 

This appears terribly inefficient, which is why I added order metadata in the branch "optimize-order-cancellation", specifically commit 3c7d53248317bc717e31f384f5c5d88a3057542a, which reduces the time complexity to O(1) at the `OrderBooks` level and O(n) at the `OrderBook` level. However, I did not see shorter run times in unit tests on my machine. That could be an artifact of poor system timing on Windows, and I really expect that for large volumes of trades, the optimizations on the "optimize-order-cancellation" branch would be beneficial. This would require further investigation and experimentation.
#### Space Complexity
Cancelling an order removes its `ExistingOrder` from the proper `OrderBook`, and will remove the entire `Vec<ExistingOrder>` at that price level if it was the only entry.