use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::match_result::MatchResult;
use crate::order_error::OrderError;
use crate::order_queue::OrderQueue;
use crate::order_type::{OrderId, OrderType};
use crate::stats::Stats;
use crate::transaction::Transaction;

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
        self.stats.record_order_added();
    }

    pub fn get_order(&self) -> Vec<Arc<OrderType>> {
        let mut data = Vec::new();
        loop {
            if self.orders.is_empty() {
                break;
            }
            if let Some(order) = self.orders.pop() {
                data.push(order);
            }
        }

        data
    }

    pub fn match_order(&mut self, taker_oid: OrderId, quantity: u64) -> MatchResult {
        let mut result = MatchResult::new(taker_oid, quantity);
        let mut remaining = quantity;
        while remaining > 0 {
            if let Some(order) = self.orders.pop() {
                let (executed_qty, matched_order, remaining_qty) = order.match_order(remaining);
                if executed_qty > 0 {
                    let transaction = Transaction {
                        transaction_id: Uuid::new_v4(),
                        taker_order_id: taker_oid,
                        maker_order_id: order.id(),
                        price: order.price(),
                        quantity: executed_qty,
                        taker_side: order.side(),
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    };
                    result.add_transaction(transaction);
                    self.stats.order_executed.fetch_add(1, Ordering::SeqCst);
                    self.stats.quantity_executed.fetch_add(executed_qty, Ordering::SeqCst);
                    if matched_order.is_none() {
                        result.filled_order_ids.push(order.id());
                    }
                    if let Some(order) = matched_order {
                        self.orders.push(Arc::new(order));
                    } else {
                        self.order_count.fetch_sub(1, Ordering::SeqCst);
                    }





                    remaining = remaining_qty;
                    if remaining == 0 {
                        break;
                    }
                }

            }
        }
        result
    }
}

