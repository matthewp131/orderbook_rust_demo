use std::collections::HashMap;

use crate::{order_book::OrderBook, order::{NewOrder, CancelOrder}, order_result::OrderResult};

pub struct OrderBooks {
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

    pub fn cancel_order(&mut self, cancel_order: CancelOrder) -> Vec<OrderResult> {
        let mut order_results = vec![OrderResult::Acknowledgement { user: cancel_order.user, user_order_id: cancel_order.user_order_id }];
        
        for order_book in self.all_orders.values_mut() {
            order_results.append(&mut order_book.cancel_order(&cancel_order));
        }

        order_results
    }

    pub fn flush(&mut self) {
        self.all_orders.clear()
    }
}