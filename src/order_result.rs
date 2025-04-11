pub enum OrderResult {
    Acknowledgement { user: u64, user_order_id: u64 },
    Rejection { user: u64, user_order_id: u64 },
    TopOfBookChange { side: char, price: String, total_quantity: String },
    Trade { user_buy: u64, user_order_id_buy: u64, user_sell: u64, user_order_id_sell: u64, price: u64, qty: u64 }
}

impl ToString for OrderResult {
    fn to_string(&self) -> String {
        match self {
            Self::Acknowledgement { user, user_order_id } => format!("A, {}, {}", user, user_order_id),
            Self::Rejection { user, user_order_id } => format!("R, {}, {}", user, user_order_id),
            Self::TopOfBookChange { side, price, total_quantity} => format!("B, {}, {}, {}", side, price, total_quantity),
            Self::Trade { user_buy, user_order_id_buy, user_sell, user_order_id_sell, price, qty } =>
                format!("T, {}, {}, {}, {}, {}, {}", user_buy, user_order_id_buy, user_sell, user_order_id_sell, price, qty)
        }
    }
}