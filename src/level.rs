use std::sync::atomic::{AtomicU64, AtomicUsize};
use crate::order_queue::OrderQueue;

pub struct PriceLevel {

    price: u64,

    order_count: AtomicUsize,

    quantity: AtomicU64,

    orders: OrderQueue
}

