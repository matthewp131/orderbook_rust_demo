use std::cell::RefCell;
use std::collections::HashMap;
use std::thread;
use std::sync::{mpsc, Arc};
use std::sync::Mutex;
use chrono::{DateTime, Utc};

struct NewOrder {
    user: i64,
    symbol: String,
    price: i64,
    qty: i64,
    side: char,
    user_order_id: i64,
    time_received: DateTime<Utc>
}

impl NewOrder {
    fn new(user: i64, symbol: String, price: i64, qty: i64, side: char, user_order_id: i64, time_received: DateTime<Utc>) -> NewOrder {
        NewOrder {
            user,
            symbol,
            price,
            qty,
            side,
            user_order_id,
            time_received
        }
    }
}

#[derive(Debug)]
struct ExistingOrder {
    user: i64,
    qty: i64,
    user_order_id: i64,
    time_received: DateTime<Utc>
}

impl ExistingOrder {
    fn new(new_order: NewOrder) -> ExistingOrder {
        ExistingOrder {
            user: new_order.user,
            qty: new_order.qty,
            user_order_id: new_order.user_order_id,
            time_received: new_order.time_received
        }
    }
}

struct CancelOrder {
    user: i64,
    user_order_id: i64
}

impl CancelOrder {
    fn new(user: i64, user_order_id: i64) -> CancelOrder {
        CancelOrder {
            user,
            user_order_id
        }
    }
}

#[derive(Debug)]
struct OrderBook {
    symbol: String,
     // key is price; Vec<ExistingOrder> is sorted by time_received
    buy_orders: HashMap<i64, Vec<ExistingOrder>>,
    sell_orders: HashMap<i64, Vec<ExistingOrder>>
}

impl OrderBook {
    fn new(symbol: String) -> OrderBook {
        OrderBook {
            symbol,
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new()
        }
    }

    fn add_buy_order(&mut self, new_order: NewOrder) {
        if let Some(v) = self.buy_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap());
        } else {
            self.buy_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }        
    }

    fn add_sell_order(&mut self, new_order: NewOrder) {
        if let Some(v) = self.sell_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap().reverse());
        } else {
            self.sell_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }        
    }
}

fn main() {
    let mut all_orders: HashMap<String, OrderBook> = HashMap::new();
    
    let (tx, rx) = mpsc::channel();
    
    let input_thread = thread::spawn(move || {
        if let Ok(mut reader) = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path("input_file.csv") {
            for line in reader.records() {
                match line {
                    Ok(row) => {
                        if let Some(value) = row.get(0) {  
                            // let value = value.to_string();                          
                            if value.starts_with("#name: ") {
                                tx.send(value.to_string()).unwrap();
                            } else if value.starts_with("#descr:") {
                                tx.send(value.to_string()).unwrap();
                                tx.send("\n".to_string()).unwrap();
                            } else {
                                match value {
                                    "N" => tx.send("N".to_string()).unwrap(),
                                    "C" => tx.send("C".to_string()).unwrap(),
                                    "F" => tx.send("F".to_string()).unwrap(),
                                    _ => tx.send("error".to_string()).unwrap()
                                }
                            }
                        }
                    },
                    Err(e) => println!("{e}")
                }
            }
        }        
    });

    let output_thread = thread::spawn(move || {
        for x in rx {

            println!("{x}");
        }
    });

    input_thread.join().unwrap();
    output_thread.join().unwrap();
}