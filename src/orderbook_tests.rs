//! Unit tests for `OrderBook`

#[cfg(test)]
mod top_of_book_tests {
    use crate::order_book::TopOfBook;
    use crate::order_result::*;

    #[test]
    fn top_of_book() {
        let tb1 = TopOfBook::new('B', Some(10), Some(100));
        let tb2 = TopOfBook::new('S', Some(10), Some(100));
        let tb3 = TopOfBook::new('B', None, None);
        let tb4 = TopOfBook::new('S', None, None);

        assert_eq!(tb1.to_order_result(), OrderResult::TopOfBookChange { side: 'B', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(tb2.to_order_result(), OrderResult::TopOfBookChange { side: 'S', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(tb3.to_order_result(), OrderResult::TopOfBookChange { side: 'B', price: "-".to_string(), total_quantity: "-".to_string() });
        assert_eq!(tb4.to_order_result(), OrderResult::TopOfBookChange { side: 'S', price: "-".to_string(), total_quantity: "-".to_string() });
    }
}

#[cfg(test)]
mod orderbook_tests {
    use crate::order_book::OrderBook;
    use crate::order::*;
    use crate::order_result::*;

    #[test]
    fn order_book_empty_cancel() {
        let mut order_book = OrderBook::new("AAPL", false);

        let order_results = order_book.cancel_order(&CancelOrder::new(1, 1));

        assert_eq!(order_results.len(), 0);
    }

    #[test]
    fn order_book_add() {
        let mut order_book = OrderBook::new("AAPL", false);

        let order_results = order_book.add_order(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'S', 1));
        
        assert_eq!(order_results.len(), 2);
        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'S', price: "10".to_string(), total_quantity: "100".to_string() });
    }

    #[test]
    fn order_book_cancel() {
        let mut order_book = OrderBook::new("AAPL", false);

        let mut order_results = order_book.add_order(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'S', 1));
        order_results.append(&mut order_book.cancel_order(&CancelOrder::new(1, 1)));

        assert_eq!(order_results.len(), 4);
        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'S', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[3], OrderResult::TopOfBookChange { side: 'S', price: "-".to_string(), total_quantity: "-".to_string() });
    }

    #[test]
    fn order_book_cross() {
        let mut order_book = OrderBook::new("AAPL", false);

        let mut order_results = order_book.add_order(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'S', 1));
        order_results.append(&mut order_book.add_order(NewOrder::new(2, "AAPL".to_string(), 11, 100, 'B', 101)));

        assert_eq!(order_results.len(), 3);
        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'S', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Rejection { user: 2, user_order_id: 101 });
    }

    #[test]
    fn order_book_trade() {
        let mut order_book = OrderBook::new("AAPL", true);

        let mut order_results = order_book.add_order(NewOrder::new(1, "AAPL".to_string(), 10, 100, 'S', 1));
        order_results.append(&mut order_book.add_order(NewOrder::new(2, "AAPL".to_string(), 10, 100, 'B', 101)));

        assert_eq!(order_results.len(), 5);
        assert_eq!(order_results[0], OrderResult::Acknowledgement { user: 1, user_order_id: 1 });
        assert_eq!(order_results[1], OrderResult::TopOfBookChange { side: 'S', price: "10".to_string(), total_quantity: "100".to_string() });
        assert_eq!(order_results[2], OrderResult::Acknowledgement { user: 2, user_order_id: 101 });
        assert_eq!(order_results[3], OrderResult::Trade { user_buy: 2, user_order_id_buy: 101, user_sell: 1, user_order_id_sell: 1, price: 10, qty: 100 });
        assert_eq!(order_results[4], OrderResult::TopOfBookChange { side: 'S', price: "-".to_string(), total_quantity: "-".to_string() });
    }
}