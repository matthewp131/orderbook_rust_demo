use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::sync::{mpsc, Arc};
use std::sync::Mutex;
use chrono::{DateTime, Utc};

struct NewOrder {
    user: u64,
    symbol: String,
    price: u64,
    qty: u64,
    side: char,
    user_order_id: u64,
    time_received: DateTime<Utc>
}

impl NewOrder {
    fn new(user: u64, symbol: String, price: u64, qty: u64, side: char, user_order_id: u64, time_received: DateTime<Utc>) -> NewOrder {
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
    user: u64,
    qty: u64,
    user_order_id: u64,
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
    user: u64,
    user_order_id: u64
}

impl CancelOrder {
    fn new(user: u64, user_order_id: u64) -> CancelOrder {
        CancelOrder {
            user,
            user_order_id
        }
    }
}

enum OrderResult {
    Acknowledgement { user: u64, user_order_id: u64 },

}

impl ToString for OrderResult {
    fn to_string(&self) -> String {
        match self {
            Self::Acknowledgement { user, user_order_id } => format!("A, {}, {}", user, user_order_id)
        }
    }
}

#[derive(Debug)]
struct OrderBook {
    symbol: String,
    // key is price; Vec<ExistingOrder> is sorted by time_received
    buy_orders: HashMap<u64, Vec<ExistingOrder>>,
    sell_orders: HashMap<u64, Vec<ExistingOrder>>
}

impl OrderBook {
    fn new(symbol: &str) -> OrderBook {
        OrderBook {
            symbol: symbol.to_string(),
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new()
        }
    }

    fn add_buy_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        let mut order_results = vec![];
        let order_info = OrderResult::Acknowledgement{user: new_order.user, user_order_id: new_order.user_order_id};
        order_results.push(order_info);
        if let Some(v) = self.buy_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap());
        } else {
            self.buy_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }
        order_results   
    }

    fn add_sell_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        let mut order_results = vec![];
        let order_info = OrderResult::Acknowledgement{user: new_order.user, user_order_id: new_order.user_order_id};
        order_results.push(order_info);
        if let Some(v) = self.sell_orders.get_mut(&new_order.price) {
            v.push(ExistingOrder::new(new_order));
            v.sort_by(|a, b| a.time_received.partial_cmp(&b.time_received).unwrap().reverse());
        } else {
            self.sell_orders.insert(new_order.price, vec![ExistingOrder::new(new_order)]);
        }    
        order_results  
    }

    fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        if new_order.side == 'B' {
            self.add_buy_order(new_order)
        } else if new_order.side == 'S' {
            self.add_sell_order(new_order)
        } else {
            vec![]
        }
    }
}

struct OrderBooks {
    all_orders: HashMap<String, OrderBook>
}

impl OrderBooks {
    fn new() -> OrderBooks {
        OrderBooks {
            all_orders: HashMap::new()
        }
    }

    fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        if let Some(v) = self.all_orders.get_mut(&new_order.symbol) {
            v.add_order(new_order)
        } else {
            let new_symbol = new_order.symbol.clone();
            let mut new_order_book = OrderBook::new(&new_symbol);
            let order_results = new_order_book.add_order(new_order);
            self.all_orders.insert(new_symbol, new_order_book);
            order_results
        }
    }
}

fn main() {
    let mut order_books = OrderBooks::new();
    
    let (tx, rx) = mpsc::channel();
    
    let reader_thread = thread::Builder::new().name("reader_thread".to_string()).spawn(move || {
        let tx = tx;
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
                                    "N" => {
                                        assert_eq!(row.len(), 7, "Invalid New Order: \"{}\"", row.as_slice().to_string());
                                        let new_order = NewOrder::new(
                                            row.get(1).unwrap().trim().parse::<u64>().unwrap(),
                                            row.get(2).unwrap().trim().to_string(),
                                            row.get(3).unwrap().trim().parse::<u64>().unwrap(),
                                            row.get(4).unwrap().trim().parse::<u64>().unwrap(),
                                            row.get(5).unwrap().trim().chars().nth(0).unwrap(),
                                            row.get(6).unwrap().trim().parse::<u64>().unwrap(),
                                            Utc::now()
                                        );
                                        let order_results = order_books.add_order(new_order);
                                        for order_result in order_results {
                                            tx.send(order_result.to_string()).unwrap();
                                        }
                                    },
                                    "C" => tx.send("C".to_string()).unwrap(),
                                    "F" => tx.send("F".to_string()).unwrap(),
                                    _ => ()
                                }
                            }
                        }
                    },
                    Err(e) => println!("{e}")
                }
            }
        }        
    }).unwrap();

    let writer_thread = thread::Builder::new().name("writer_thread".to_string()).spawn(move || {
        for x in rx {
            println!("{x}");
        }
    }).unwrap();

    reader_thread.join().unwrap();
    writer_thread.join().unwrap();
}