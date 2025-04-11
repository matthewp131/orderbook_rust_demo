//! Unit tests for `OrderBooks`

#[cfg(test)]
mod orderbooks_tests {
    use crate::order_books::OrderBooks;
    use crate::order::*;
    use crate::order_result::*;
    use std::any::Any;
    use std::time::Instant;

    #[test]
    fn scenario1() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 11, 100, 'B', 3)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 10, 100, 'S', 103)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 4)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 104))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Rejection { user: 1, user_order_id: 3 });
        assert_eq!(order_results[8], OrderResult::Rejection { user: 2, user_order_id: 103 });
        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 1, user_order_id: 4 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "200".to_string() });
        assert_eq!(order_results[11], OrderResult::Acknowledgement { user: 2, user_order_id: 104 });
        assert_eq!(order_results[12], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });
    }

    #[test]
    fn scenario2() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "AAPL".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "AAPL".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "AAPL".to_string(), 10, 100, 'S', 103)),
            Box::new(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'B', 3))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[5], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[6], OrderResult::Rejection { user: 2, user_order_id: 103 });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 1, user_order_id: 3 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "200".to_string() });
    }

    #[test]
    fn scenario3() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "VAL".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "VAL".to_string(), 11, 100, 'B', 2)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 11, 100, 'S', 103)),
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[3], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[4], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[5], OrderResult::Rejection { user: 1, user_order_id: 2 });
        assert_eq!(order_results[6], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[7], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });
    }

    #[test]
    fn scenario14_scenario3_trading_enabled() {
        let mut order_books = OrderBooks::new(true);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "VAL".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "VAL".to_string(), 11, 100, 'B', 2)),
            Box::new(NewOrder::new(2, "VAL".to_string(), 11, 100, 'S', 103)),
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[3], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[4], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[6], OrderResult::Trade { user_buy: 1, user_order_id_buy: 2, user_sell: 2, user_order_id_sell: 102, price: 11, qty: 100 });
        assert_eq!(order_results[7], OrderResult::TopOfBookChange { side: 'S', price: "-".to_string(), total_quantity: "-".to_string() });
        assert_eq!(order_results[8], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[9], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
    }
    
    #[test]
    fn scenario4() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'S', 103))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Rejection { user: 2, user_order_id: 103 });
    }

    #[test]
    fn scenario5() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'B', 103)),
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Rejection { user: 1, user_order_id: 103 });
    }

    #[test]
    fn scenario13_scenario5_trading_enabled() {
        let mut order_books = OrderBooks::new(true);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'B', 103)),
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 1, user_order_id: 103 });
        assert_eq!(order_results[8], OrderResult::Trade { user_buy: 1, user_order_id_buy: 103, user_sell: 2, user_order_id_sell: 102, price: 11, qty: 100 });
        assert_eq!(order_results[9], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
    }

    #[test]
    fn scenario6() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 16, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 15, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'B', 103)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 14, 100, 'S', 3))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "16".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "15".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'B', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 1, user_order_id: 3 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'S', price: "14".to_string(), total_quantity: "100".to_string() });
    }

    #[test]
    fn scenario7() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 10, 20, 'S', 103))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Rejection { user: 2, user_order_id: 103 });
    }

    #[test]
    fn scenario8() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 11, 20, 'B', 3))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Rejection { user: 1, user_order_id: 3 });
    }

    #[test]
    fn scenario9() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(CancelOrder::new(1, 1)),
            Box::new(CancelOrder::new(2, 102))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        let start = Instant::now();
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed in scenario9() is: {}ns", duration.as_nanos());

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'B', price: "9".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
    }

    #[test]
    fn scenario10() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(CancelOrder::new(1, 2)),
            Box::new(CancelOrder::new(2, 101))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        let start = Instant::now();
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed in scenario10() is: {}ns", duration.as_nanos());

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[8], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
    }

    #[test]
    fn scenario11() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(CancelOrder::new(1, 1)),
            Box::new(CancelOrder::new(2, 101))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        let start = Instant::now();
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed in scenario11() is: {}ns", duration.as_nanos());

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'B', price: "9".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'B', price: "-".to_string(), total_quantity: "-".to_string() });
    }

    #[test]
    fn scenario12() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 103)),
            Box::new(CancelOrder::new(2, 103)),
            Box::new(CancelOrder::new(2, 102)),
            Box::new(CancelOrder::new(1, 2))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        let start = Instant::now();
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed in scenario12() is: {}ns", duration.as_nanos());

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });
        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[11], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[12], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[13], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[14], OrderResult::TopOfBookChange { side: 'S', price: "-".to_string(), total_quantity: "-".to_string() });
    }

    #[test]
    fn scenario15_mutiple_symbols() {
        let mut order_books = OrderBooks::new(false);

        let orders: Vec<Box<dyn Any>> = vec![
            Box::new(NewOrder::new(1, "IBM".to_string(), 10, 100, 'B', 1)),
            Box::new(NewOrder::new(1, "IBM".to_string(), 12, 100, 'S', 2)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 9, 100, 'B', 101)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 102)),
            Box::new(NewOrder::new(2, "IBM".to_string(), 11, 100, 'S', 103)),
            Box::new(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'B', 3)),
            Box::new(NewOrder::new(1, "AAPL".to_string(), 12, 100, 'S', 4)),
            Box::new(NewOrder::new(2, "AAPL".to_string(), 9, 100, 'B', 104)),
            Box::new(NewOrder::new(2, "AAPL".to_string(), 11, 100, 'S', 105)),
            Box::new(NewOrder::new(2, "AAPL".to_string(), 11, 100, 'S', 106)),
            Box::new(NewOrder::new(1, "MSFT".to_string(), 10, 100, 'B', 5)),
            Box::new(NewOrder::new(1, "MSFT".to_string(), 12, 100, 'S', 6)),
            Box::new(NewOrder::new(2, "MSFT".to_string(), 9, 100, 'B', 107)),
            Box::new(NewOrder::new(2, "MSFT".to_string(), 11, 100, 'S', 108)),
            Box::new(NewOrder::new(2, "MSFT".to_string(), 11, 100, 'S', 109)),
            Box::new(CancelOrder::new(2, 108)),
            Box::new(CancelOrder::new(2, 104)),
            Box::new(CancelOrder::new(1, 5)),
            Box::new(CancelOrder::new(2, 103)),
            Box::new(CancelOrder::new(2, 102)),
            Box::new(CancelOrder::new(1, 6)),
            Box::new(CancelOrder::new(2, 106)),
            Box::new(CancelOrder::new(2, 107)),
            Box::new(CancelOrder::new(1, 3))
        ];

        let mut order_results: Vec<OrderResult> = vec![];
        let start = Instant::now();
        for order in orders {
            if let Some(new_order) = order.downcast_ref::<NewOrder>() {
                order_results.append(&mut order_books.add_order(new_order.clone()))
            } else if let Some(cancel_order) = order.downcast_ref::<CancelOrder>() {
                order_results.append(&mut order_books.cancel_order(cancel_order.clone()))
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed in scenario15() is: {}ns", duration.as_nanos());

        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 2 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[4], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[5], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[6], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[7], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[8], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });

        assert_eq!(order_results[9], OrderResult::Acknowledgement { user: 1, user_order_id: 3 });
        assert_eq!(order_results[10], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[11], OrderResult::Acknowledgement { user: 1, user_order_id: 4 });
        assert_eq!(order_results[12], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[13], OrderResult::Acknowledgement { user: 2, user_order_id: 104 });
        assert_eq!(order_results[14], OrderResult::Acknowledgement { user: 2, user_order_id: 105 });
        assert_eq!(order_results[15], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[16], OrderResult::Acknowledgement { user: 2, user_order_id: 106 });
        assert_eq!(order_results[17], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });

        assert_eq!(order_results[18], OrderResult::Acknowledgement { user: 1, user_order_id: 5 });
        assert_eq!(order_results[19], OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[20], OrderResult::Acknowledgement { user: 1, user_order_id: 6 });
        assert_eq!(order_results[21], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[22], OrderResult::Acknowledgement { user: 2, user_order_id: 107 });
        assert_eq!(order_results[23], OrderResult::Acknowledgement { user: 2, user_order_id: 108 });
        assert_eq!(order_results[24], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[25], OrderResult::Acknowledgement { user: 2, user_order_id: 109 });
        assert_eq!(order_results[26], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "200".to_string() });

        assert_eq!(order_results[27], OrderResult::Acknowledgement { user: 2, user_order_id: 108 });
        assert_eq!(order_results[28], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[29], OrderResult::Acknowledgement { user: 2, user_order_id: 104 });
        assert_eq!(order_results[30], OrderResult::Acknowledgement { user: 1, user_order_id: 5 });
        assert_eq!(order_results[31], OrderResult::TopOfBookChange { side: 'B', price: "9".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[32], OrderResult::Acknowledgement { user: 2, user_order_id: 103 });
        assert_eq!(order_results[33], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[34], OrderResult::Acknowledgement { user: 2, user_order_id: 102 });
        assert_eq!(order_results[35], OrderResult::TopOfBookChange { side: 'S', price: "12".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[36], OrderResult::Acknowledgement { user: 1, user_order_id: 6 });
        assert_eq!(order_results[37], OrderResult::Acknowledgement { user: 2, user_order_id: 106 });
        assert_eq!(order_results[38], OrderResult::TopOfBookChange { side: 'S', price: "11".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[39], OrderResult::Acknowledgement { user: 2, user_order_id: 107 });
        assert_eq!(order_results[40], OrderResult::TopOfBookChange { side: 'B', price: "-".to_string(), total_quantity: "-".to_string() });
        assert_eq!(order_results[41], OrderResult::Acknowledgement { user: 1, user_order_id: 3 });
        assert_eq!(order_results[42], OrderResult::TopOfBookChange { side: 'B', price: "-".to_string(), total_quantity: "-".to_string() });
    }
}