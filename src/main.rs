use std::collections::{HashMap, BTreeMap};
use std::thread;
use std::sync::{mpsc};
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
    Rejection { user: u64, user_order_id: u64 },
    TopOfBookChange { side: char, price: String, total_quantity: String }
}

impl ToString for OrderResult {
    fn to_string(&self) -> String {
        match self {
            Self::Acknowledgement { user, user_order_id } => format!("A, {}, {}", user, user_order_id),
            Self::Rejection { user, user_order_id } => format!("R, {}, {}", user, user_order_id),
            Self::TopOfBookChange { side, price, total_quantity} => format!("B, {}, {}, {}", side, price, total_quantity)
        }
    }
}

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

struct OrderBook {
    symbol: String,
    // key is price; Vec<ExistingOrder> is sorted by time_received
    buy_orders: BTreeMap<u64, Vec<ExistingOrder>>,
    sell_orders: BTreeMap<u64, Vec<ExistingOrder>>
}

impl OrderBook {
    fn new(symbol: &str) -> OrderBook {
        OrderBook {
            symbol: symbol.to_string(),
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new()
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

    fn add_order(&mut self, new_order: NewOrder) -> Vec<OrderResult> {
        assert!(new_order.side == 'B' || new_order.side == 'S', "Invalid New Order. New order must be B or S.");
        if self.crosses_book(&new_order) {
            vec![OrderResult::Rejection { user: new_order.user, user_order_id: new_order.user_order_id }]
        } else if new_order.side == 'B' {
            self.add_buy_order(new_order)
        } else if new_order.side == 'S' {
            self.add_sell_order(new_order)
        } else {
            vec![]
        }
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

    fn cancel_order(&mut self, cancel_order: &CancelOrder) -> Vec<OrderResult> {
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

    fn cancel_order(&mut self, cancel_order: CancelOrder) -> Vec<OrderResult> {
        let mut order_results = vec![OrderResult::Acknowledgement { user: cancel_order.user, user_order_id: cancel_order.user_order_id }];
        
        for order_book in self.all_orders.values_mut() {
            order_results.append(&mut order_book.cancel_order(&cancel_order));
        }

        order_results
    }

    fn flush(&mut self) {
        self.all_orders.clear()
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
                            if value.starts_with("#name: ") {
                                tx.send("".to_string()).unwrap();
                                tx.send(value.to_string()).unwrap();
                            } else if value.starts_with("#descr:") {
                                tx.send(value.to_string()).unwrap();
                                tx.send("".to_string()).unwrap();
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
                                    "C" => {
                                        assert_eq!(row.len(), 3, "Invalid Cancel Order: \"{}\"", row.as_slice().to_string());
                                        let cancel_order = CancelOrder::new(
                                            row.get(1).unwrap().trim().parse::<u64>().unwrap(),
                                            row.get(2).unwrap().trim().parse::<u64>().unwrap()
                                        );
                                        let order_results = order_books.cancel_order(cancel_order);
                                        for order_result in order_results {
                                            tx.send(order_result.to_string()).unwrap();
                                        }
                                    },
                                    "F" => order_books.flush(),
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