use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::order_queue::OrderQueue;
use crate::order_type::OrderType;
use crate::stats::Stats;

pub struct PriceLevel {

    price: u64,

    order_count: AtomicUsize,

    quantity: AtomicU64,

    orders: OrderQueue,

    stats: Stats
}

impl PriceLevel {

    pub fn new(price: u64) -> Self {
        PriceLevel {
            price,
            order_count: AtomicUsize::new(0),
            quantity: AtomicU64::new(0),
            orders: OrderQueue::new(),
            stats: Stats::new()
        }

    }

    pub fn add_order(&self, order: OrderType) {
        self.order_count.fetch_add(1, Ordering::SeqCst);
        self.quantity.fetch_add(order.quantity(), Ordering::SeqCst);
        self.orders.push(Arc::new(order));
    }
}

