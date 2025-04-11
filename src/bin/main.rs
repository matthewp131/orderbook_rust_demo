//! The main file, which handles program start-up and command line arguments

use std::thread;
use std::sync::{mpsc, mpsc::Sender};
use orderbook::order_books::OrderBooks;
use std::env;
use csv::StringRecord;
use orderbook::order::{CancelOrder, NewOrder};

/// Holds options passed as command line arguments
struct RuntimeConfig {
    input_file: String,
    trading_enabled: bool
}

impl RuntimeConfig {
    fn new(input_file: String, trading_enabled: bool) -> RuntimeConfig {
        RuntimeConfig { input_file, trading_enabled }
    }
}

/// Handles command line arguments
fn parse_args(args: Vec<String>) -> RuntimeConfig {
    let mut trading_enabled = false;
    let mut input_file = String::new();
    for arg in args {        
        if arg == "-t" || arg == "--trading-enabled" {
            trading_enabled = true;
        } else if arg.ends_with(".csv") {
            input_file = arg;
        }
    }
    RuntimeConfig::new(input_file, trading_enabled)
}

/// Takes each row from the input CSV, outputs name or descr directly, and otherwise
/// parses transaction input messages.
fn handle_row(row: StringRecord, tx: &Sender<String>, order_books: &mut OrderBooks) {
    if let Some(value) = row.get(0) {                          
        if value.starts_with("#name: ") {
            tx.send(row.as_slice().to_string()).unwrap();
        } else if value.starts_with("#descr:") {
            let mut s = row.get(0).unwrap().to_string();
            if let Some(row1) = row.get(1) {
                s.push_str(",");
                s.push_str(row1);
            }
            tx.send(s).unwrap();
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
                        row.get(6).unwrap().trim().parse::<u64>().unwrap()
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
}

/// The main function takes in command line arguments, starts a reader thread which handles
/// the input csv row-by-row, outputting the results over a Sender to the writer thread.
/// The writer thread receives results and writes them to stdout. The program waits for both threads 
/// to finish before exiting.
fn main() {
    let runtime_config = parse_args(env::args().collect());

    let mut order_books = OrderBooks::new(runtime_config.trading_enabled);
    
    let (tx, rx) = mpsc::channel();
    
    let reader_thread = thread::Builder::new().name("reader_thread".to_string()).spawn(move || {
        let tx = tx;
        if let Ok(mut reader) = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(runtime_config.input_file) {
            for line in reader.records() {
                match line {
                    Ok(row) => {
                        handle_row(row, &tx, &mut order_books);
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