use std::collections::BTreeMap;

use crate::{order_result::OrderResult, order::{ExistingOrder, NewOrder, CancelOrder}};

#[derive(PartialEq)]
struct TopOfBook {
    price: Option<u64>,
    total_quantity: Option<u64>
}

impl TopOfBook {
    fn new(price: Option<u64>, total_quantity: Option<u64>) -> TopOfBook {
        TopOfBook { 
            price, 
            total_quantity 
        }
    }

    fn to_order_result(&self, side: char) -> OrderResult {
        let price_string = match self.price {
            Some(price) => price.to_string(),
            None => "-".to_string()
        };
        let total_quantity_string = match self.total_quantity {
            Some(total_quantity) => total_quantity.to_string(),
            None => "-".to_string()
        };
        OrderResult::TopOfBookChange { side, price: price_string, total_quantity: total_quantity_string }
    }
}

struct OrderBookLocation {
    side: char,
    price: u64,
    index: usize
}

impl OrderBookLocation {
    fn new(side: char, price: u64, index: usize) -> OrderBookLocation {
        OrderBookLocation { side, price, index }
    }
}

pub struct OrderBook {
    _symbol: String,
    // key is price; Vec<ExistingOrder> is sorted by time_received
    buy_orders: BTreeMap<u64, Vec<ExistingOrder>>,
    sell_orders: BTreeMap<u64, Vec<ExistingOrder>>,
    trading_enabled: bool
}

impl OrderBook {
    pub fn new(symbol: &str, trading_enabled: bool) -> OrderBook {
        OrderBook {
            _symbol: symbol.to_string(),
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            trading_enabled
        }
    }

    fn crosses_book(&self, new_order: &NewOrder) -> bool {
        if new_order.side == 'B' {
            if self.is_above_lowest_sell_price(new_order.price) {
                true
            } else {
                false
            }
        } else if new_order.side == 'S' {
            if self.is_below_highest_buy_price(new_order.price) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_above_lowest_sell_price(&self, buy_price: u64) -> bool {
        if self.sell_orders.len() == 0 {
            false
        } else {
            let lowest_sell_price = self.sell_orders.keys().nth(0).unwrap();
            buy_price >= *lowest_sell_price
        }
    }

    fn is_below_highest_buy_price(&self, sell_price: u64) -> bool {
        if self.buy_orders.len() == 0 {
            false
        } else {
            let highest_buy_price = self.buy_orders.keys().rev().nth(0).unwrap();
            sell_price <= *highest_buy_price
        }
    }

    fn get_top_of_buy_book(&self) -> TopOfBook {
        if self.buy_orders.is_empty() {
            TopOfBook::new(None, None)
        } else {
            let top = self.buy_orders.iter().rev().nth(0).unwrap();
            let price = *top.0;
            let total_quantity = top.1.iter().map(|existing_order| existing_order.qty).sum::<u64>();
            TopOfBook::new(Some(price), Some(total_quantity))
        }
    }

    fn add_buy_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        let current_top = self.get_top_of_buy_book();
        
        let mut order_results = vec![];
        order_results.push(OrderResult::Acknowledgement{user: new_order.user, user_order_id: new_order.user_order_id});
        
        if let Some(v) = self.buy_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap());
        } else {
            self.buy_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }

        let new_top = self.get_top_of_buy_book();
        if new_top != current_top {
            order_results.push(new_top.to_order_result('B'));
        }

        order_results
    }

    fn get_top_of_sell_book(&self) -> TopOfBook {
        if self.sell_orders.is_empty() {
            TopOfBook::new(None, None)
        } else {
            let top = self.sell_orders.iter().nth(0).unwrap();
            let price = *top.0;
            let total_quantity = top.1.iter().map(|existing_order| existing_order.qty).sum::<u64>();
            TopOfBook::new(Some(price), Some(total_quantity))
        }
    }

    fn add_sell_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        let current_top = self.get_top_of_sell_book();
        
        let mut order_results = vec![];
        order_results.push(OrderResult::Acknowledgement{user: new_order.user, user_order_id: new_order.user_order_id});

        if let Some(v) = self.sell_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap().reverse());
        } else {
            self.sell_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }    

        let new_top = self.get_top_of_sell_book();
        if new_top != current_top {
            order_results.push(new_top.to_order_result('S'));
        }

        order_results  
    }

    pub fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        assert!(new_order.side == 'B' || new_order.side == 'S', "Invalid New Order. New order must be B or S.");
        if self.crosses_book(&new_order) {
            if self.trading_enabled {
                self.attempt_order_match(new_order)
            } else {
                vec![OrderResult::Rejection { user: new_order.user, user_order_id: new_order.user_order_id }]
            }
        } else if new_order.side == 'B' {
            self.add_buy_order(new_order)
        } else if new_order.side == 'S' {
            self.add_sell_order(new_order)
        } else {
            vec![]
        }
    }

    fn attempt_order_match(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        let mut order_results = vec![];

        order_results.push(OrderResult::Acknowledgement { user: new_order.user, user_order_id: new_order.user_order_id });
        
        if new_order.side == 'B' {
            let current_top = self.get_top_of_sell_book();
            if let Some(order_book_location) = self.match_order(&new_order, 'S') {
                let existing_order = self.remove_order(order_book_location);
                order_results.push(OrderResult::Trade { 
                    user_buy: new_order.user, 
                    user_order_id_buy: new_order.user_order_id, 
                    user_sell: existing_order.user, 
                    user_order_id_sell: existing_order.user_order_id, 
                    price: existing_order.price, 
                    qty: existing_order.qty });
                let new_top = self.get_top_of_sell_book();
                if new_top != current_top {
                    order_results.push(new_top.to_order_result('S'));
                }
            }
        } else {
            let current_top = self.get_top_of_buy_book();
            if let Some(order_book_location) = self.match_order(&new_order, 'B') {
                let existing_order = self.remove_order(order_book_location);
                order_results.push(OrderResult::Trade { 
                    user_buy: existing_order.user, 
                    user_order_id_buy: existing_order.user_order_id, 
                    user_sell: new_order.user, 
                    user_order_id_sell: new_order.user_order_id, 
                    price: existing_order.price, 
                    qty: existing_order.qty });
                let new_top = self.get_top_of_buy_book();
                if new_top != current_top {
                    order_results.push(new_top.to_order_result('B'));
                }
            }
        }

        order_results
    }

    fn remove_order(&mut self, order_book_location: OrderBookLocation) -> ExistingOrder {
        if order_book_location.side == 'B' {
            let vec = self.buy_orders.get_mut(&order_book_location.price).unwrap();
            let existing_order = vec.remove(order_book_location.index);
            if vec.len() == 0 {
                self.buy_orders.remove(&order_book_location.price);
            }
            existing_order
        } else {
            let vec = self.sell_orders.get_mut(&order_book_location.price).unwrap();
            let existing_order = vec.remove(order_book_location.index);
            if vec.len() == 0 {
                self.sell_orders.remove(&order_book_location.price);
            }
            existing_order
        }
    }

    fn match_order(&self, new_order: &NewOrder, side: char) -> Option<OrderBookLocation> {
        if side == 'S' {
            for (price, existing_orders) in self.sell_orders.iter() {
                if *price <= new_order.price {
                    for (index, existing_order) in existing_orders.iter().enumerate() {
                        if existing_order.qty == new_order.qty {
                            return Some(OrderBookLocation::new(side, *price, index));
                        }
                    }
                }
            }
        } else if side == 'B' {
            for (price, existing_orders) in self.buy_orders.iter() {
                if *price >= new_order.price {
                    for (index, existing_order) in existing_orders.iter().enumerate() {
                        if existing_order.qty == new_order.qty {
                            return Some(OrderBookLocation::new(side, *price, index));
                        }
                    }
                }
            }
        }
        None
    }

    fn clean_buy_orders(&mut self) {
        let prices = self.buy_orders.iter()
            .filter(|(_, existing_orders)| existing_orders.len() == 0)
            .map(|(price, _)| *price).collect::<Vec<u64>>();
        for price in prices {
            self.buy_orders.remove(&price);
        }
    }

    fn clean_sell_orders(&mut self) {
        let prices = self.sell_orders.iter()
            .filter(|(_, existing_orders)| existing_orders.len() == 0)
            .map(|(price, _)| *price).collect::<Vec<u64>>();
        for price in prices {
            self.sell_orders.remove(&price);
        }
    }

    pub fn cancel_order(&mut self, cancel_order: &CancelOrder) -> Vec<OrderResult> {
        let mut order_results = vec![];
        
        let current_top = self.get_top_of_buy_book();
        for existing_orders in self.buy_orders.values_mut() {
            existing_orders.retain(|existing_order| !(existing_order.user == cancel_order.user && existing_order.user_order_id == cancel_order.user_order_id));
        }
        self.clean_buy_orders();
        let new_top = self.get_top_of_buy_book();
        if new_top != current_top {
            order_results.push(new_top.to_order_result('B'));
        }
        
        let current_top = self.get_top_of_sell_book();
        for existing_orders in self.sell_orders.values_mut() {
            existing_orders.retain(|existing_order| !(existing_order.user == cancel_order.user && existing_order.user_order_id == cancel_order.user_order_id));
        }
        self.clean_sell_orders();
        let new_top = self.get_top_of_sell_book();
        if new_top != current_top {
            order_results.push(new_top.to_order_result('S'));
        }

        order_results
    }
}