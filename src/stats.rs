use std::sync::atomic::{AtomicU64, AtomicUsize};

pub struct Stats {

    pub order_added: AtomicUsize,
    pub order_removed: AtomicUsize,
    pub order_processed: AtomicUsize,
    pub quantity_executed: AtomicU64,
    pub quality_executed: AtomicU64,
    pub last_executed_time: AtomicU64,
    pub first_execution_time: AtomicU64,
    pub sum_waiting_time: AtomicU64,
}

