//! All order books, managed by symbol

use std::collections::HashMap;

use crate::{order_book::OrderBook, order::{NewOrder, CancelOrder}, order_result::OrderResult};

/// Hold a colection of orderbooks in a hashmap and track whether trading mode is enabled
pub struct OrderBooks {
    /// A hashmap where the key is a stock symbol (Ex. AAPL) and the value is an `OrderBook`
    all_orders: HashMap<String, OrderBook>,
    trading_enabled: bool
}

impl OrderBooks {
    pub fn new(trading_enabled: bool) -> OrderBooks {
        OrderBooks {
            all_orders: HashMap::new(),
            trading_enabled
        }
    }

    /// Locate the proper `OrderBook` for the new order or create if not already existing for that 
    /// symbol. 
    pub fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        if let Some(v) = self.all_orders.get_mut(&new_order.symbol) {
            v.add_order(new_order)
        } else {
            let new_symbol = new_order.symbol.clone();
            let mut new_order_book = OrderBook::new(&new_symbol, self.trading_enabled);
            let order_results = new_order_book.add_order(new_order);
            self.all_orders.insert(new_symbol, new_order_book);
            order_results
        }
    }

    /// Search through each `OrderBook` attempting to cancel an existing order
    pub fn cancel_order(&mut self, cancel_order: CancelOrder) -> Vec<OrderResult> {
        let mut order_results: Vec<OrderResult> = vec![];
        
        for order_book in self.all_orders.values_mut() {
            order_results.append(&mut order_book.cancel_order(&cancel_order));
        }

        order_results
    }

    /// Flush all `OrderBook`s
    pub fn flush(&mut self) {
        self.all_orders.clear()
    }
}