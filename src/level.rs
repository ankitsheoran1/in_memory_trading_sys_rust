use std::sync::atomic::{AtomicU64, AtomicUsize};

pub struct PriceLevel {

    price: u64,

    order_count: AtomicUsize,

    quantity: AtomicU64,

    orders: SegQueue<Arc<OrderType>>
}