use std::sync::Arc;
use crate::order_type::OrderType;

pub struct Snapshot {

    pub price: u64,
    pub quantity: u64,
    pub order_count: usize,
    pub orders: Vec<Arc<OrderType>>,
}



