use std::sync::Arc;
use crossbeam::queue::SegQueue;
use crate::order_type::OrderType;

pub struct OrderQueue {

    orders: SegQueue<Arc<OrderType>>
}

impl OrderQueue {

    pub fn new() -> Self {
        Self {
            orders: SegQueue::new(),
        }
    }

    pub fn push(&self, order: Arc<OrderType>) {
        self.orders.push(order)
    }

    pub fn pop(&self) -> Option<Arc<OrderType>> {
        self.orders.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

}