#[cfg(test)]
mod tests {
    use crate::order_books::OrderBooks;
    use crate::order::*;
    use crate::order_result::*;
    use std::any::Any;

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
}