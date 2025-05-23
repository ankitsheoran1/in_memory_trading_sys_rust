use std::cmp::Ordering;
use std::sync::atomic::{AtomicU64, AtomicUsize};

pub struct Stats {

    pub order_added: AtomicUsize,
    pub order_removed: AtomicUsize,
    pub order_executed: AtomicUsize,
    pub quantity_executed: AtomicU64,
    pub last_execution_time: AtomicU64,
    pub first_arrival_time: AtomicU64,
    pub sum_waiting_time: AtomicU64,

}

impl Stats {
    pub fn new() -> Self {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Stats {
            order_added: AtomicUsize::new(0),
            order_removed: AtomicUsize::new(0),
            order_executed: AtomicUsize::new(0),
            quantity_executed: AtomicU64::new(0),
            last_execution_time: AtomicU64::new(0),
            first_arrival_time: AtomicU64::new(current_time),
            sum_waiting_time: AtomicU64::new(0),
        }
    }

    pub fn record_order_added(&self) {
        self.order_added.fetch_add(1, Ordering::Relaxed);
    }



}


