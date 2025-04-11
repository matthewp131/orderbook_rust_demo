//! All order books, managed by symbol

use std::collections::HashMap;

use crate::{order_book::OrderBook, order::{NewOrder, CancelOrder}, order_result::OrderResult};

struct OrderBooksLocation {
    symbol: String,
    side: char,
    price: u64
}

impl OrderBooksLocation {
    fn new(symbol: &str, side: char, price: u64) -> OrderBooksLocation {
        OrderBooksLocation { symbol: symbol.to_string(), side, price }
    }
}

pub struct OrderBooks {
    all_orders: HashMap<String, OrderBook>,
    trading_enabled: bool,
    order_metadata: HashMap<(u64, u64), OrderBooksLocation>
}

impl OrderBooks {
    pub fn new(trading_enabled: bool) -> OrderBooks {
        OrderBooks {
            all_orders: HashMap::new(),
            trading_enabled,
            order_metadata: HashMap::new()
        }
    }

    pub fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        self.order_metadata.insert((new_order.user, new_order.user_order_id), OrderBooksLocation::new(&new_order.symbol, new_order.side, new_order.price));
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

    pub fn cancel_order(&mut self, cancel_order: CancelOrder) -> Vec<OrderResult> {
        let mut order_results = vec![OrderResult::Acknowledgement { user: cancel_order.user, user_order_id: cancel_order.user_order_id }];
        
        let order_books_location = self.order_metadata.get(&(cancel_order.user, cancel_order.user_order_id)).unwrap();
        let order_book = self.all_orders.get_mut(&order_books_location.symbol).unwrap();
        order_results.append(&mut order_book.cancel_order(&cancel_order, order_books_location.side, order_books_location.price));

        order_results
    }

    /// Flush all `OrderBook`s
    pub fn flush(&mut self) {
        self.all_orders.clear()
    }
}