use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct Stats {

    pub order_added: AtomicUsize,
    pub order_removed: AtomicUsize,
    pub order_executed: AtomicUsize,
    pub quantity_executed: AtomicU64,
    pub last_execution_time: AtomicU64,
    pub first_arrival_time: AtomicU64,
    pub sum_waiting_time: AtomicU64,
    pub value_executed: AtomicU64,

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
            value_executed: AtomicU64::new(0),
            quantity_executed: AtomicU64::new(0),
            last_execution_time: AtomicU64::new(0),
            first_arrival_time: AtomicU64::new(current_time),
            sum_waiting_time: AtomicU64::new(0),
        }
    }

    pub fn record_order_added(&self) {
        self.order_added.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_execution(&self, quantity: u64, price: u64, order_timestamp: u64) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        self.order_executed.fetch_add(1, Ordering::Relaxed);
        self.quantity_executed
            .fetch_add(quantity, Ordering::Relaxed);
        self.value_executed
            .fetch_add(quantity * price, Ordering::Relaxed);
        self.last_execution_time
            .store(current_time, Ordering::Relaxed);

        // Calculate waiting time for this order
        if order_timestamp > 0 {
            let waiting_time = current_time.saturating_sub(order_timestamp);
            self.sum_waiting_time
                .fetch_add(waiting_time, Ordering::Relaxed);
        }
    }
}


