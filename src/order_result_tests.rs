//! Unit tests for `OrderBook`

#[cfg(test)]
mod order_result_tests {
    use crate::order_result::*;

    #[test]
    fn acknowledgement() {
        let acknowledgement = OrderResult::Acknowledgement { user: 1, user_order_id: 1 };
        assert_eq!(acknowledgement.to_string(), "A, 1, 1");
    }

    #[test]
    fn rejection() {
        let rejection = OrderResult::Rejection { user: 1, user_order_id: 1 };
        assert_eq!(rejection.to_string(), "R, 1, 1");
    }

    #[test]
    fn top_of_book_change() {
        let top_of_book_change = OrderResult::TopOfBookChange { side: 'B', price: 10.to_string(), total_quantity: 100.to_string() };
        assert_eq!(top_of_book_change.to_string(), "B, B, 10, 100");
    }

    #[test]
    fn trade() {
        let trade = OrderResult::Trade { user_buy: 1, user_order_id_buy: 1, user_sell: 2, user_order_id_sell: 101, price: 10, qty: 100 };
        assert_eq!(trade.to_string(), "T, 1, 1, 2, 101, 10, 100");
    }
}