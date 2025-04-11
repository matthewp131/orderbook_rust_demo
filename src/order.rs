//! Order structs corresponding to New Order and Cancel Order transaction requests, as well as the ExistingOrder struct
//! for storing orders in memory

use chrono::{DateTime, Utc};

/// A request to place a new order into OrderBooks
#[derive(Clone)]
pub struct NewOrder {
    pub user: u64,
    pub symbol: String,
    pub price: u64,
    pub qty: u64,
    pub side: char,
    pub user_order_id: u64,
    pub time_received: DateTime<Utc>
}

impl NewOrder {
    pub fn new(user: u64, symbol: String, price: u64, qty: u64, side: char, user_order_id: u64) -> NewOrder {
        NewOrder { user, symbol, price, qty, side, user_order_id, time_received: Utc::now() }
    }
}

/// The format of an order inside of a single Orderbook
#[derive(Debug)]
pub struct ExistingOrder {
    pub user: u64,
    pub price: u64,
    pub qty: u64,
    pub user_order_id: u64,
    pub time_received: DateTime<Utc>
}

impl ExistingOrder {
    pub fn new(new_order: NewOrder) -> ExistingOrder {
        ExistingOrder {
            user: new_order.user,
            price: new_order.price,
            qty: new_order.qty,
            user_order_id: new_order.user_order_id,
            time_received: new_order.time_received
        }
    }
}

/// The format of an order cancellation request
#[derive(Clone)]
pub struct CancelOrder {
    pub user: u64,
    pub user_order_id: u64
}

impl CancelOrder {
    pub fn new(user: u64, user_order_id: u64) -> CancelOrder {
        CancelOrder { user, user_order_id }
    }
}